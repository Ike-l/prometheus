use anyhow::{
	Context, Error, Result
};

use crate::prelude::{
	acceleration_structures_plugin::prelude::AABB, label_plugin::prelude::LabelComponent, promethius_std::prelude::Position
};

use super::{
    raw_render_component::RawRenderComponent, FloatPrecision
};

use anyhow::Ok;
use cgmath::{
    Array, Deg, InnerSpace, Matrix4, One, Quaternion, Rotation3, Transform, Vector3, Vector4
};

use log::warn;

#[derive(Debug)]
pub struct InstanceRenderComponent {
    pub visible: bool,
	pub object_label: LabelComponent,

    pub model_translation: Vector3<FloatPrecision>,
    pub world_translation: Vector3<FloatPrecision>,

    pub model_rotation: Quaternion<FloatPrecision>,
    pub world_rotation: Quaternion<FloatPrecision>,

    pub model_scale: Vector3<FloatPrecision>,
    pub world_scale: Vector3<FloatPrecision>,

	/// model.color * instance.tint + instance.highlight;
    pub tint: Vector4<FloatPrecision>,
	/// model.color * instance.tint + instance.highlight;
    pub highlight: Vector4<FloatPrecision>,
}

impl Default for InstanceRenderComponent {
    fn default() -> Self {
        Self {
            visible: true,
			object_label: LabelComponent::default(),

            model_translation: Vector3::from_value(0.0),
            world_translation: Vector3::from_value(0.0),

            model_rotation: Quaternion::one(),
            world_rotation: Quaternion::one(),

            model_scale: Vector3::from_value(1.0),
            world_scale: Vector3::from_value(1.0),

            tint: Vector4::from_value(1.0), 
            highlight: Vector4::from_value(0.0), 
        }
    }
}

impl InstanceRenderComponent {
    pub fn to_raw(&self) -> RawRenderComponent {
        let model = self.model_matrix();

        RawRenderComponent::new(model.into(), self.tint.into(), self.highlight.into())
    }

    pub fn model_matrix(&self) -> Matrix4<FloatPrecision> {
        Matrix4::from_translation(self.world_translation) *
        Matrix4::from(self.world_rotation) *
        Matrix4::from_nonuniform_scale(self.world_scale.x, self.world_scale.y, self.world_scale.z) *
        Matrix4::from_translation(self.model_translation) *
        Matrix4::from(self.model_rotation) *
        Matrix4::from_nonuniform_scale(self.model_scale.x, self.model_scale.y, self.model_scale.z)
    }

	pub fn inner_model_matrix(&self) -> Matrix4<FloatPrecision> {
		Matrix4::from_translation(self.model_translation) *
        Matrix4::from(self.model_rotation) *
        Matrix4::from_nonuniform_scale(self.model_scale.x, self.model_scale.y, self.model_scale.z)
	}

	pub fn outer_model_matrix(&self) -> Matrix4<FloatPrecision> {
		Matrix4::from_translation(self.world_translation) *
        Matrix4::from(self.world_rotation) *
        Matrix4::from_nonuniform_scale(self.world_scale.x, self.world_scale.y, self.world_scale.z)
	}

    pub fn model_rotate(&mut self, angle: &Deg<FloatPrecision>, axis: &Vector3<FloatPrecision>) {
        let rotation_quat = Quaternion::from_axis_angle(axis.normalize(), *angle);
        self.model_rotation = rotation_quat * self.model_rotation;
    }

    pub fn world_rotate(&mut self, angle: &Deg<FloatPrecision>, axis: &Vector3<FloatPrecision>) {
        let rotation_quat = Quaternion::from_axis_angle(axis.normalize(), *angle);
        self.world_rotation = rotation_quat * self.world_rotation;
    }

	pub fn model_vertex(&self, vertex: Vector4<FloatPrecision>) -> Vector4<FloatPrecision> {
		if vertex.w != 1.0 { warn!("Vertex taken as direction, translations won't apply") }
		self.model_matrix() * vertex
	}

	pub fn set_width(&mut self, current_width: FloatPrecision, target_width: FloatPrecision) -> Result<()> {
		if current_width == 0.0 {
			anyhow::bail!("Division by 0 `current_width`");
		} 

		self.model_scale.x = target_width / current_width;
		Ok(())
	}	

	pub fn set_height(&mut self, current_height: FloatPrecision, target_height: FloatPrecision) -> Result<()> {
		if current_height == 0.0 {
			anyhow::bail!("Division by 0 `current_height`");
		}

		self.model_scale.y = target_height / current_height;
		Ok(())
	}

	pub fn set_depth(&mut self, current_depth: FloatPrecision, target_depth: FloatPrecision) -> Result<()> {
		if current_depth == 0.0 {
			anyhow::bail!("Division by 0 `current_depth`");
		}

		self.model_scale.z = target_depth / current_depth;
		Ok(())
	}

	pub fn set_min(
		&mut self, 
		other_render: &Self, 
		self_min: &Position, 
		target_min: &Position
	) -> Result<(), Error> {
		self.model_rotation = other_render.model_rotation;
		self.world_rotation = other_render.world_rotation;
		self.model_translation = Vector3::from_value(0.0);

		let target_min = Vector3::new(
			target_min.x as FloatPrecision, 
			target_min.y as FloatPrecision, 
			target_min.z as FloatPrecision
		);

		let self_min = Vector3::new(
			self_min.x as FloatPrecision,
			self_min.y as FloatPrecision,
			self_min.z as FloatPrecision
		);

		self.model_translation = self
			.inner_model_matrix()
			.inverse_transform_vector(
				other_render
					.outer_model_matrix()
					.inverse_transform_vector(target_min)
					.context("Cannot inverse-transform `target_min` with `other outer model`")?
				)
				.context("Cannot inverse-transform (previous step) with `self inner model`")?
			- self_min;

		Ok(())
	}

	pub fn set_min_max(
		&mut self,
		other_render: &Self,
		current_aabb: &AABB,
		target_min: &Position,
		target_max: &Position,
	) -> Result<(), Error> {
		self.model_rotation = other_render.model_rotation;
		self.world_rotation = other_render.world_rotation;

		let target_dimensions = Position::new(
			target_max.x - target_min.x, 
			target_max.y - target_min.y, 
			target_max.z - target_min.z,
		);

		let current_min = Position::new(
			current_aabb.min.x * target_dimensions.x / current_aabb.width() , 
			current_aabb.min.y * target_dimensions.y / current_aabb.height(), 
			current_aabb.min.z * target_dimensions.z / current_aabb.depth()
		);

		self.set_min(other_render, &current_min, &target_min).unwrap();

		self.set_width(current_aabb.width() as f32, target_dimensions.x as f32).unwrap();
		self.set_height(current_aabb.height() as f32, target_dimensions.y as f32).unwrap();
		self.set_depth(current_aabb.depth() as f32, target_dimensions.z as f32).unwrap();

		Ok(())
	}
}

