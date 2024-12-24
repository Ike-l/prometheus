use super::{
    camera_plugin::prelude::{
        CameraProjectionComponent, CameraRenderComponent
    }, 
    acceleration_structures_plugin::AccelerationStructure, 
    entity::prelude::InstanceRenderComponent, 
    label_plugin::prelude::LabeledEntities, 
    object_plugin::prelude::ObjectRegistry, 
    render_plugin::WindowDimensions, 
    ui_acceleration_structure::UIAccelerationStructure, 
    CursorPosition, DeviceEventBus, EventReader, MutWorld, Res, ResMut, WindowEventBus
};

#[derive(Debug, Clone)]
pub enum Event {
    Window(WindowEventBus),
    Device(DeviceEventBus)
}

#[derive(Debug, Default)]
pub struct UIComponent {
    pub event_buffer: Vec<Event>,
}

pub fn input(
    window_events: EventReader<WindowEventBus>, 
    device_events: EventReader<DeviceEventBus>, 
    mut world: MutWorld,
    acc_struct: Res<UIAccelerationStructure>,
    object_registry: Res<ObjectRegistry>,
    mut cursor_position: ResMut<CursorPosition>,
    labels: Res<LabeledEntities>,
    window_dimensions: Res<WindowDimensions>
) {
    for event in window_events.read() {
        match event.0 {
            winit::event::WindowEvent::CursorMoved { position, .. } => {
                cursor_position.x = position.x;
                cursor_position.y = position.y;
            }
            _ => {
                log::info!("Window Event captured in UI: {event:?}");                
                handle_event(
                    &mut world,
                    &acc_struct,
                    &object_registry,
                    &cursor_position,
                    &labels,
                    &window_dimensions, 
                    &Event::Window(event.clone()),
                );
            }
        }
    }

    for event in device_events.read() {
        log::info!("Device Event captured in UI: {event:?}");
        handle_event(
            &mut world,
            &acc_struct, 
            &object_registry,
            &cursor_position,
            &labels, 
            &window_dimensions,
            &Event::Device(event.clone()),
        );
    }
}

fn handle_event(
    world: &mut MutWorld,
    acc_struct: &UIAccelerationStructure, 
    object_registry: &ObjectRegistry,
    cursor_position: &CursorPosition,
    labels: &LabeledEntities, 
    window_dimensions: &WindowDimensions,
    event: &Event,
) {
    let mut entity_coords = vec![];

    for (entity, render) in &mut world.query::<&InstanceRenderComponent>().with::<&UIComponent>() {
        let object = object_registry.objects.get(&render.object_label).expect("render component object label does not have a corresponding object in `ObjectRegistry`");
        let mut camera_query = world
            .query_one::<&CameraRenderComponent>(
                labels.get_entity(&object.camera_label)
                    .expect("Failed to find an entity corresponding to the camera label in `LabeledEntities`. Ensure the label is correct.")
                    .to_owned())
            .expect("Failed to query the `CameraRenderComponent` for the given entity. Ensure the entity has a `CameraRenderComponent` attached.");

        let camera = camera_query
            .get()
            .expect("Failed to retrieve the `CameraRenderComponent`. Ensure the component exists and is accessible.");

        let world_space = match &camera.projection {
            CameraProjectionComponent::Ortho(ortho_projection) => {
                ortho_projection.to_world_space(cursor_position, window_dimensions)
            },
            CameraProjectionComponent::Persp(_) => unreachable!("doesnt really make sense to have a perspective camera on a UIComponent found in a QuadTree"),
        };

        entity_coords.push((entity, world_space));
    }

    for (entity, coords) in entity_coords {
        let collisions = acc_struct.query(&coords);
        if collisions
            .iter()
            .any(|collider| labels.get_label(&entity) == Some(&collider.entity_label)) {
                match world.query_one_mut::<&mut UIComponent>(entity) {
                    Ok(component) => {
                        component.event_buffer.push(event.clone());
                    },
                    Err(_) => {
                        unreachable!("Accleration Structure has an entity without a `UIComponent`")
                    }
                } 
        }
    }
}




