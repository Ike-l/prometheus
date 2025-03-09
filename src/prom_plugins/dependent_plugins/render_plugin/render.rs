use std::collections::HashMap;

use wgpu::{util::{BufferInitDescriptor, DeviceExt}, BufferUsages, CommandEncoderDescriptor, LoadOp, Operations, RenderPassColorAttachment, RenderPassDepthStencilAttachment, RenderPassDescriptor, StoreOp, TextureView, TextureViewDescriptor};

use crate::{prelude::{Res, ResMut, SupportedFloat, WriteWorld}, prom_plugins::dependent_plugins::interfaces::render_plugin::{CameraInterface, DirectionLightInterface, InstanceInterface, ObjectInterface, PointLightInterface, SpotLightInterface, TextureInterface}};

use super::{bind_groups::{bind_group_flags::BindGroupFlags, bind_group_registry::{update_bind_group_registries, BindGroupRegistries}, default_bind_groups::DefaultBindGroups, storage_bind_group::StorageBindGroup, uniform_bind_group::UniformBindGroup}, lib_types::{instance::Instance, lights::Light}, meshes::mesh_registry::update_mesh_registry, window::{background_colour::BackgroundColour, state::WindowState}, MeshRegistry};

pub fn render_scene(
    mut world: WriteWorld, 
    mut bind_group_registries: ResMut<BindGroupRegistries>,
    mut mesh_registry: ResMut<MeshRegistry>,
    state: Res<WindowState>, 
    background_colour: Res<BackgroundColour>,
    default_bind_groups: Res<DefaultBindGroups>,
) {
    let mut new_buffers  = vec![];

    update_mesh_registry(&mut mesh_registry, &mut world, &state);
    update_bind_group_registries(&mut world, &mut bind_group_registries, &state);

    let world_registry = world.world_registry.borrow();
    let world = world.get_world();

    let background_colour = background_colour.value.into();

    let mut object_meshes = HashMap::new();
    for mesh in mesh_registry.meshes.values() {
        if let Some(object_id) = &mesh.object_id {
            object_meshes.entry(object_id).or_insert_with(Vec::new).push(mesh);
        }
    }

    let mut object_instances = HashMap::new();
    let mut q = world.query::<&Box<dyn InstanceInterface>>();
    for (_, instance) in q.into_iter() {
        object_instances.entry(instance.object_id()).or_insert_with(Vec::new).push(instance);
    }

    let mut object_instance_buffers = HashMap::new();
    for (object, instances) in object_instances {
        let instances_len = instances.len();

        let objects_data = instances
            .into_iter()
            .map(<&Box<dyn InstanceInterface> as Into<Instance>>::into)
            .collect::<Vec<_>>();

        let instance_buffer = state.device().create_buffer_init(&BufferInitDescriptor {
            label: Some("Instance buffer"),
            contents: bytemuck::cast_slice(&objects_data),
            usage: BufferUsages::VERTEX
        });

        object_instance_buffers.insert(object.unwrap().to_string(), (instance_buffer, instances_len));
    }

    let mut lights: Vec<Light> = vec![]; 
    for (_, light) in world.query::<&Box<dyn SpotLightInterface>>().iter() {
        lights.push(light.into())
    }

    for (_, light) in world.query::<&Box<dyn DirectionLightInterface>>().iter() {
        lights.push(light.into())
    }

    for (_, light) in world.query::<&Box<dyn PointLightInterface>>().iter() {
        lights.append(&mut Light::from_point_light(light).to_vec());
    }
    //println!("lights: {lights:?}");

//    let (shadow_bind_group, shadow_texture) = default_bind_groups.shadow();

    let (shadow_view, shadow_sampler, shadow_texture) = super::bind_groups::default_bind_groups::create_shadow(state.device(), lights.len() as u32);
    let shadow = super::bind_groups::texture_bind_group::TextureBindGroup::new(state.device(), state.bind_group_layouts().shadow(), &shadow_view, &shadow_sampler, Some("Shadow Bind Group"));
    let shadow_bind_group = shadow.bind_group();

    let light_cameras: Vec<(TextureView, [[SupportedFloat; 4]; 4])> = lights
        .iter()
        .enumerate()
        .map(
            |(i, light)| {
            (
                shadow_texture.create_view(&wgpu::TextureViewDescriptor {
                    label: Some("shadow"),
                    format: None,
                    dimension: Some(wgpu::TextureViewDimension::D2),
                    usage: None,
                    aspect: wgpu::TextureAspect::All,
                    base_mip_level: 0,
                    mip_level_count: None,
                    base_array_layer: i as u32,
                    array_layer_count: Some(1),
                }),
                light.view_projection.clone()
            )
        }).collect();

    // let shadow_map_bind_group = TextureBindGroup::new(
    //     state.device(), 
    //     state.bind_group_layouts().shadow(), 
    //     &shadow_view, 
    //     &shadow_sampler, 
    //     "Shadow".into(),
    // );

    // let shadow_bind_group = shadow_map_bind_group.bind_group();

    let output = state.surface().get_current_texture().unwrap();
    let mut encoder = state.device().create_command_encoder(&CommandEncoderDescriptor::default());
    {
        // create light_camera bind group
        let light_camera_bind_group = UniformBindGroup::new(state.device(), state.bind_group_layouts().light_camera(), &bytemuck::cast_slice(&[[0.0 as f32; 4]; 4]), Some("LightCamera"));
        
        for (view, proj) in light_cameras {
            light_camera_bind_group.write(state.queue(), &bytemuck::cast_slice(&proj));

            let mut shadow_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: None,
                color_attachments: &[],
                depth_stencil_attachment: Some(RenderPassDepthStencilAttachment {
                    view: &view,
                    depth_ops: Some(Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            shadow_pass.set_pipeline(state.shadow_pipeline());
            
            let mut object_query = world.query::<&Box<dyn ObjectInterface>>();
            for (object_entity, _) in object_query.into_iter() {         
                let object_label = world_registry.entity_to_label().get(&object_entity).unwrap();
                let (buffer, instances_len) = if let Some(instance_info) = object_instance_buffers.get(object_label) {
                    instance_info
                } else {
                    continue;
                };

                let meshes = if let Some(meshes) = object_meshes.get(object_label) {
                    meshes
                } else {
                    continue
                };

                shadow_pass.set_vertex_buffer(1, buffer.slice(..));
                

                for mesh in meshes {
                    shadow_pass.set_vertex_buffer(0, mesh.vertex_buffer().slice(..));
                    shadow_pass.set_index_buffer(mesh.index_buffer().slice(..), wgpu::IndexFormat::Uint32);

                    shadow_pass.set_bind_group(0, light_camera_bind_group.bind_group(), &[]);

                    shadow_pass.draw_indexed(0..*mesh.indices(), 0, 0..*instances_len as u32);
                }
            }
        }

        let view = output.texture.create_view(&TextureViewDescriptor::default());
        let mut forward_pass = encoder.begin_render_pass(&RenderPassDescriptor {
            label: Some("Main Render Pass"),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Clear(background_colour),
                    store: StoreOp::Store,
                }
            })],
            depth_stencil_attachment: Some(RenderPassDepthStencilAttachment {
                view: &state.depth_texture().view(),
                depth_ops: Some(Operations {
                    load: LoadOp::Clear(1.0),
                    store: StoreOp::Store
                }),
                stencil_ops: None
            }),
            timestamp_writes: None,
            occlusion_query_set: None,
        });
        
        forward_pass.set_pipeline(state.main_pipeline());
        let mut pass_flags = 0 | BindGroupFlags::Shadow as u8;
                                         
        let light_data = if lights.len() == 0 {
            vec![0 as u8]            
        } else {
            pass_flags |= BindGroupFlags::Lights as u8;
            bytemuck::cast_slice(&lights).to_vec()
        };

        let lights_storage_bind_group = StorageBindGroup::new(
            state.device(), 
            state.bind_group_layouts().lights(), 
            &light_data, 
            "Lights".into()
        );
        
        let lights_bind_group = lights_storage_bind_group.bind_group();
            
        let mut object_query = world.query::<&Box<dyn ObjectInterface>>();
        for (object_entity, object) in object_query.into_iter() {  
            let mut object_flags = pass_flags;          
            let camera_bind_group = if let Some(camera) = object.camera_id() {
                let camera_entity = world_registry.label_to_entity().get(camera).unwrap();
                object_flags |= BindGroupFlags::Camera as u8;

                let camera_bind_group = bind_group_registries.cameras.get(camera_entity).unwrap();
                if let Ok(mut camera) = world.query_one::<&Box<dyn CameraInterface>>(*camera_entity) {
                    let camera = camera.get().unwrap();
                    camera_bind_group.write(state.queue(), &camera.camera().cast_slice());
                }

                camera_bind_group.bind_group()
            } else {
                default_bind_groups.camera()
            };
            
            let object_label = world_registry.entity_to_label().get(&object_entity).unwrap();
            let (buffer, instances_len) = if let Some(instance_info) = object_instance_buffers.get(object_label) {
                instance_info
            } else {
                continue;
            };

            forward_pass.set_vertex_buffer(1, buffer.slice(..));

            let meshes = if let Some(meshes) = object_meshes.get(object_label) {
                meshes
            } else {
                continue
            };

            for mesh in meshes {
                let mut mesh_flags = object_flags;

                let material_bind_group = if let Some(material_id) = &mesh.material_id {
                    mesh_flags |= BindGroupFlags::Material as u8;
                    let entity = world_registry.label_to_entity().get(material_id).unwrap();
                    bind_group_registries.materials.get(entity).unwrap().bind_group()
                } else {
                    default_bind_groups.material()
                };

                let texture_bind_group = if let Some(texture_id) = &mesh.texture_id {
                    mesh_flags |= BindGroupFlags::Texture as u8;
                    let entity = world_registry.label_to_entity().get(texture_id).unwrap();
                    bind_group_registries.textures.get(entity).unwrap().bind_group()
                } else {
                    default_bind_groups.texture()
                };

                // create not in loop and just write to it
                let bind_group_flags_bind_group = UniformBindGroup::new(
                    state.device(), 
                    state.bind_group_layouts()
                        .bind_group_flags(), 
                    &vec![mesh_flags], 
                    Some("BindGroupFlags")
                );

                forward_pass.set_vertex_buffer(0, mesh.vertex_buffer().slice(..));
                forward_pass.set_index_buffer(mesh.index_buffer().slice(..), wgpu::IndexFormat::Uint32);

                forward_pass.set_bind_group(0, lights_bind_group, &[]);
                forward_pass.set_bind_group(1, shadow_bind_group, &[]);
                forward_pass.set_bind_group(2, camera_bind_group, &[]);
                forward_pass.set_bind_group(3, material_bind_group, &[]);
                forward_pass.set_bind_group(4, texture_bind_group, &[]);
                forward_pass.set_bind_group(5, bind_group_flags_bind_group.bind_group(), &[]);

                forward_pass.draw_indexed(0..*mesh.indices(), 0, 0..*instances_len as u32);

                new_buffers.push(bind_group_flags_bind_group.take_buffer());
            }
        }

        new_buffers.push(lights_storage_bind_group.take_buffer());
    }

    
    state.queue().submit(std::iter::once(encoder.finish()));
    output.present();

    // encoder.clear_buffer(buffer, offset, size);
    // encoder.clear_texture(texture, subresource_range);
    
    object_instance_buffers.drain().into_iter().for_each(
        |(_, (buffer, _))| {
            buffer.destroy();    
            std::mem::drop(buffer);
        });

    // new_buffers.iter().for_each(|buffer| {
    //     //buffer.unmap();
    //     buffer.destroy();
    // });

    // state.device().poll(wgpu::Maintain::Wait);
    
}