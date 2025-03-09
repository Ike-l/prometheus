use cgmath::{Matrix4, Point3, Rad, Transform, Vector2, Vector3, Vector4};
use wgpu::{Sampler, TextureView};

use crate::{prelude::SupportedFloat, prom_plugins::dependent_plugins::render_plugin::lib_types::{camera::Camera, material::Material}};

use super::Interface;

pub trait LightGenericsInterface: Interface {
    fn position(&self) -> Point3<SupportedFloat>;
    fn colour(&self) -> Vector3<SupportedFloat>;
    fn intensity(&self) -> Vector3<SupportedFloat>;
    fn attenutation(&self) -> Vector3<SupportedFloat>;
    
    fn mut_position(&mut self) -> Option<&mut Point3<SupportedFloat>> { None }
    fn mut_colour(&mut self) -> Option<&mut Vector3<SupportedFloat>> { None }
    fn mut_intensity(&mut self) -> Option<&mut Vector3<SupportedFloat>> { None }
    fn mut_attenutation(&mut self) -> Option<&mut Vector3<SupportedFloat>> { None }
}


pub trait SpotLightInterface: LightGenericsInterface + Send + Sync {
    fn direction(&self) -> Vector3<SupportedFloat>;
    fn inner_cone(&self) -> Rad<SupportedFloat>;
    fn outer_cone(&self) -> Rad<SupportedFloat>;
    
    fn mut_direction(&mut self) -> Option<&mut Vector3<SupportedFloat>> { None }
    fn mut_inner_cone(&mut self) -> Option<&mut Rad<SupportedFloat>> { None }
    fn mut_outer_cone(&mut self) -> Option<&mut Rad<SupportedFloat>> { None }
}

pub trait DirectionLightInterface: LightGenericsInterface + Send + Sync {
    fn direction(&self) -> Vector3<SupportedFloat>;
    fn face_length(&self) -> SupportedFloat;
    
    fn mut_direction(&mut self) -> Option<&mut Vector3<SupportedFloat>> { None }
    fn mut_face_length(&mut self) -> Option<&mut SupportedFloat> { None }
}

pub trait PointLightInterface: LightGenericsInterface + Send + Sync {
    fn face_length(&self) -> SupportedFloat {
        // "near" plane * 2
        0.2
    }

    fn mut_face_length(&mut self) -> Option<&mut SupportedFloat> { None }
}

pub trait CameraInterface: Send + Sync + Interface {
    fn view(&self) -> Matrix4<SupportedFloat>;
    fn projection(&self) -> Matrix4<SupportedFloat>;
    fn position(&self) -> Point3<SupportedFloat>;
    #[allow(private_interfaces)]
    fn camera(&self) -> Camera {
        Camera {
            position: self.position().into(),
            view_projection: ( self.projection() * self.view() ).into(),
            inverse_projection: self.projection().inverse_transform().unwrap().into(),
            ..Default::default()
        }
    }
}

pub trait MaterialInterface: Send + Sync + Interface {
    fn ambience(&self) -> Vector3<SupportedFloat>;
    fn opacity(&self) -> SupportedFloat;
    fn specularity(&self) -> Vector3<SupportedFloat>;
    fn shininess(&self) -> SupportedFloat;
    fn diffusivity(&self) -> Vector3<SupportedFloat>;
    fn material(&self) -> Material {
        Material {
            ambience: self.ambience().extend(self.opacity()).into(),
            specularity: self.specularity().extend(self.shininess()).into(),
            diffusivity: self.diffusivity().extend(1.0).into()
        }
    }

    fn mut_ambience(&mut self) -> Option<&mut Vector3<SupportedFloat>> { None }
    fn mut_opacity(&mut self) -> Option<&mut SupportedFloat> { None }
    fn mut_specularity(&mut self) -> Option<&mut Vector3<SupportedFloat>> { None }
    fn mut_shininess(&mut self) -> Option<&mut SupportedFloat> { None }
    fn mut_diffusivity(&mut self) -> Option<&mut Vector3<SupportedFloat>> { None }
}

pub trait InstanceInterface: Send + Sync + Interface {
    fn model(&self) -> Matrix4<SupportedFloat>;
    fn normal(&self) -> Option<Matrix4<SupportedFloat>>;
    fn object_id(&self) -> Option<&String>;
}

pub trait MeshInterface: Send + Sync + Interface {
    fn positions(&self) -> &Vec<Point3<SupportedFloat>>;
    fn indices(&self) -> &Vec<u32>;
    fn normals(&self) -> Option<&Vec<Vector3<SupportedFloat>>>;
    fn colours(&self) -> Option<&Vec<Vector3<SupportedFloat>>>;
    fn texture_coords(&self) -> Option<&Vec<Vector2<SupportedFloat>>>;
    fn object_id(&self) -> Option<&String>;
    fn texture_id(&self) -> Option<&String>;
    fn material_id(&self) -> Option<&String>;

    fn mut_positions(&mut self) -> Option<&mut Vec<Point3<SupportedFloat>>> { None }
    fn mut_indices(&mut self) -> Option<&mut Vec<u32>> { None }
    fn mut_normals(&mut self) -> Option<&mut Vec<Vector3<SupportedFloat>>> { None }
    fn mut_colours(&mut self) -> Option<&mut Vec<Vector4<SupportedFloat>>> { None }
    fn mut_texture_coords(&self) -> Option<&mut Vec<Vector2<SupportedFloat>>> { None }
    fn mut_object_id(&mut self) -> Option<&mut String> { None }
    fn mut_texture_id(&mut self) -> Option<&mut String> { None }
    fn mut_material_id(&mut self) -> Option<&mut String> { None }
}

pub trait ObjectInterface: Send + Sync + Interface {
    fn camera_id(&self) -> Option<&String>;

    fn mut_camera_id(&mut self) -> Option<&mut String> { None }
}

pub trait TextureInterface: Send + Sync + Interface {
    fn sampler(&self) -> &Sampler;
    fn view(&self) -> &TextureView;

    fn mut_sampler(&mut self) -> Option<&mut Sampler> { None }
    fn mut_view(&mut self) -> Option<&mut TextureView> { None }
}

