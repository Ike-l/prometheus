use anyhow::{
	Context, Error, Result
};

use crate::prelude::{
	label_plugin::prelude::LabelComponent, promethius_std::prelude::Position
};

use super::{
    raw_render_component::RawRenderComponent, FloatPrecision
};

use anyhow::Ok;
use cgmath::{
    Array, Deg, EuclideanSpace, InnerSpace, Matrix4, One, Quaternion, Rotation3, Transform, Vector3, Vector4
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
		if vertex.w == 0.0 { warn!("Vertex is a direction, translations won't apply") }
		self.model_matrix() * vertex
	}

	pub fn normalised_width_dir(&self) -> Vector3<FloatPrecision> {
		self.model_vertex(Vector4::unit_x()).truncate()
	}

	pub fn normalised_height_dir(&self) -> Vector3<FloatPrecision> {
		self.model_vertex(Vector4::unit_y()).truncate()
	}

	pub fn normalised_depth_dir(&self) -> Vector3<FloatPrecision> {
		self.model_vertex(Vector4::unit_z()).truncate()
	}

	pub fn model_position(&self, position: &Position) -> Position {
		let position = Vector4::new(position.x as f32, position.y as f32, position.z as f32, 1.0);

		let position = self.model_vertex(position);

		Position::new(position.x as f64, position.y as f64, position.z as f64)
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

	pub fn set_dimensions(&mut self, current_dimensions: &Position, target_dimensions: &Position) -> Result<()> {
		self.set_width(current_dimensions.x as f32, target_dimensions.x as f32)?;
		self.set_height(current_dimensions.y as f32, target_dimensions.y as f32)?;
		self.set_depth(current_dimensions.z as f32, target_dimensions.z as f32)?;

		Ok(())
	}

	pub fn set_min(
		&mut self,  
		mesh_min: &Position, 
		target_min: &Position
	) -> Result<(), Error> {
		// spent a week on this please dont ask me how it works jk... it is quite intuitive if u abstract it.

		// doing this means you can override the set_min and adjust it *relative* to the *target_min* by just changing the world translation
		// to undo this change `inv_out` to self.outer_model_matrix() . inverse yada yada
		let outer_model_matrix_excluding_translation = Matrix4::from(self.world_rotation) *
        Matrix4::from_nonuniform_scale(self.world_scale.x, self.world_scale.y, self.world_scale.z);

		let inv_out = outer_model_matrix_excluding_translation.inverse_transform().context("failed to inverse the world transformations")?;

		// doing this means i dont have to set the model translation to 0
		// to undo this change it to self.inner_model_matrix() and set the model translation to 0.0
		let inner_model_matrix_excluding_translation = Matrix4::from(self.model_rotation) *
        Matrix4::from_nonuniform_scale(self.model_scale.x, self.model_scale.y, self.model_scale.z);

		self.model_translation = (
			(
				inv_out * 
				target_min.position
					.to_vec()
					.cast()
					.unwrap()
					.extend(1.0)
			) - 
			(
				inner_model_matrix_excluding_translation * 
				mesh_min.position
					.to_vec()
					.cast()
					.unwrap()
					.extend(1.0)
				)
		).truncate();
		
	    Ok(())
	}

	pub fn set_min_max(
		&mut self,
		mesh_min: &Position,
		mesh_max: &Position,
		target_min: &Position,
		target_max: &Position,
	) -> Result<(), Error> {
		// see tests for why
		assert_eq!(self.model_rotation, Quaternion::one(), "having rotations and `set_min_max` is undefined");
		assert_eq!(self.world_rotation, Quaternion::one(), "having rotations and `set_min_max` is undefined"); 
		assert_eq!(self.world_scale, Vector3::from_value(1.0), "having a world scale and `set_min_max` is undefined");

		let current_dimensions = mesh_max - mesh_min;
		let target_dimensions = target_max - target_min;

		self.set_dimensions(&current_dimensions, &target_dimensions)?;
		self.set_min(&mesh_min, target_min)?;

		Ok(())
	}
}

