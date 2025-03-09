use bytemuck::{Pod, Zeroable};
use cgmath::{Deg, InnerSpace, Vector3};

use crate::{prelude::SupportedFloat, prom_plugins::dependent_plugins::interfaces::render_plugin::{DirectionLightInterface, PointLightInterface, SpotLightInterface}};

#[derive(Debug, Clone, Copy)]
pub enum LightTypeKind {
    Spot = 1,
    Direction = 1 << 1,
    Point = 1 << 2,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct Light {
    pub position: [SupportedFloat; 3],
    pub light_type: SupportedFloat,
    pub colour: [SupportedFloat; 3],
    pub inner_cone: SupportedFloat,
    pub intensity: [SupportedFloat; 3],
    pub outer_cone: SupportedFloat,
    pub direction: [SupportedFloat; 3],
    pub padding1: SupportedFloat,
    pub attenuation: [SupportedFloat; 3],
    pub padding2: SupportedFloat,
    pub projection: [[SupportedFloat; 4]; 4],
    pub view_projection: [[SupportedFloat; 4]; 4], // Shadow pass
}

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.5,
    0.0, 0.0, 0.0, 1.0,
);


impl From<&Box<dyn SpotLightInterface>> for Light {
    fn from(light: &Box<dyn SpotLightInterface>) -> Self {
        // from testing 90 is better than 45
        let projection = OPENGL_TO_WGPU_MATRIX * cgmath::perspective(Deg(90.0), 1.0, 0.1, 1000.0);
        let view = cgmath::Matrix4::look_to_rh(
            light.position(),
            light.direction().normalize(),
            cgmath::Vector3::unit_y(),
        );
        let light_view_projection = projection * view;
        Self {
            view_projection: light_view_projection.into(),
            position: light.position().into(),
            colour: light.colour().into(),
            intensity: light.intensity().into(),
            projection: projection.into(),
            direction: light.direction().normalize().into(),
            padding1: 0.0,
            attenuation: light.attenutation().into(),
            padding2: 0.0,
            light_type: LightTypeKind::Spot as u32 as SupportedFloat,
            inner_cone: light.inner_cone().0,
            outer_cone: light.outer_cone().0,
        }
    }
}

impl From<&Box<dyn DirectionLightInterface>> for Light {
    fn from(light: &Box<dyn DirectionLightInterface>) -> Self {
        // corners decided by the size of the representation of the light i.e if a face 10x10 then ortho 10x10, make direction of light player pos - light pos
        let h_l = light.face_length() / 2.0;
        let projection = OPENGL_TO_WGPU_MATRIX * cgmath::ortho(-h_l, h_l, -h_l, h_l, 0.1, 1000.0);
        let view = cgmath::Matrix4::look_to_rh(
            light.position(),
            light.direction().normalize(),
            cgmath::Vector3::unit_y(),
        );
        let light_view_projection = projection * view;
        Self {
            view_projection: light_view_projection.into(),
            position: light.position().into(),
            colour: light.colour().into(),
            intensity: light.intensity().into(),
            projection: projection.into(),
            direction: light.direction().normalize().into(),
            padding1: 0.0,
            attenuation: light.attenutation().into(),
            padding2: 0.0,
            light_type: LightTypeKind::Direction as u32 as SupportedFloat,
            inner_cone: 0.0,
            outer_cone: 0.0,
        }      
    }
}

impl Light {
    pub fn from_split_point(light: &Box<dyn PointLightInterface>, direction: Vector3<SupportedFloat>) -> Self {
        // near plane defines half length of side of cube that represents the point light
        let up = if direction == Vector3::unit_y() {
            -Vector3::unit_x()
        } else if direction == -Vector3::unit_y() {
            Vector3::unit_x()
        } else {
            Vector3::unit_y()
        };
        
        let h_l = light.face_length() / 2.0;

        let position = light.position() + direction / h_l;
        let projection = OPENGL_TO_WGPU_MATRIX * cgmath::perspective(Deg(90.0), 1.0, 0.1, 1000.0);
        let view = cgmath::Matrix4::look_to_rh(
            position,
            direction,
            up,
        );

        let light_view_projection = projection * view;

        Self {
            view_projection: light_view_projection.into(),
            position: position.into(),
            colour: light.colour().into(),
            intensity: light.intensity().into(),
            projection: projection.into(),
            direction: direction.into(),
            padding1: 0.0,
            attenuation: light.attenutation().into(),
            padding2: 0.0,
            light_type: LightTypeKind::Point as u32 as SupportedFloat,
            inner_cone: 0.0,
            outer_cone: 0.0,
        }
    }
    pub fn from_point_light(light: &Box<dyn PointLightInterface>) -> [Self; 6] {
        let lights = vec![
            Vector3::unit_x(),
            Vector3::unit_y(),
            Vector3::unit_z(),
            -Vector3::unit_x(),
            -Vector3::unit_y(),
            -Vector3::unit_z(),
        ].into_iter().map(|direction| Self::from_split_point(light, direction)).collect::<Vec<_>>();

        [
            lights[0],
            lights[1],
            lights[2],
            lights[3],
            lights[4],
            lights[5],
        ]
    }
}

/*
struct Light {
    view_projection: mat4x4<f32>, 
    position: vec4<f32>,          
    colour: vec3<f32>,                        
    inner_cone: f32,             
    intensity: vec3<f32>,         
    outer_cone: f32,             
    direction: vec3<f32>,                    
    light_type: f32,                          
    attenuation: vec3<f32>,                 
}; */

/*
pub trait LightCameraInterface {
    fn view_projection(&self) -> Matrix4<SupportedFloat>;
    fn projection(&self) -> Matrix4<SupportedFloat>;

    fn mut_view_projection(&mut self) -> Option<&mut Matrix4<SupportedFloat>> { None }
    fn mut_projection(&mut self) -> Option<&mut Matrix4<SupportedFloat>> { None }
}

pub trait LightGenericsInterface {
    fn position(&self) -> Vector3<SupportedFloat>;
    fn colour(&self) -> Vector3<SupportedFloat>;
    fn intensity(&self) -> Vector3<SupportedFloat>;
    fn attenutation(&self) -> Vector3<SupportedFloat>;
    
    fn mut_position(&mut self) -> Option<&mut Vector3<SupportedFloat>> { None }
    fn mut_colour(&mut self) -> Option<&mut Vector3<SupportedFloat>> { None }
    fn mut_intensity(&mut self) -> Option<&mut Vector3<SupportedFloat>> { None }
    fn mut_attenutation(&mut self) -> Option<&mut Vector3<SupportedFloat>> { None }
}


pub trait SpotLightInterface: LightGenericsInterface + LightCameraInterface + Send + Sync {
    fn direction(&self) -> Vector3<SupportedFloat>;
    fn inner_cone(&self) -> Rad<SupportedFloat>;
    fn outer_cone(&self) -> Rad<SupportedFloat>;
    
    fn mut_direction(&mut self) -> Option<&mut Vector3<SupportedFloat>> { None }
    fn mut_inner_cone(&mut self) -> Option<&mut Rad<SupportedFloat>> { None }
    fn mut_outer_cone(&mut self) -> Option<&mut Rad<SupportedFloat>> { None }
}

pub trait DirectionLightInterface: LightGenericsInterface + LightCameraInterface + Send + Sync {
    fn direction(&self) -> Vector3<SupportedFloat>;
    fn camera(&self) -> Option<&impl LightCameraInterface>;

    fn mut_direction(&mut self) -> Option<&mut Vector3<SupportedFloat>> { None }
    fn mut_camera(&mut self) -> Option<&mut impl LightCameraInterface> { None::<&mut DummyLightCameraInterface> }
}

pub trait PointLightInterface: LightGenericsInterface + Send + Sync {} */
impl Light {
    pub fn cast_slice(&self) -> Vec<u8> {
        bytemuck::cast_slice(&[*self]).to_vec()
    }
}
