#![cfg(test)]

use cgmath::{
    Deg, Point3, Transform, Vector3, Vector4
};
use crate::prelude::{
    acceleration_structures_plugin::prelude::AABB, 
    promethius_std::prelude::Position
};

use super::{
    prelude::InstanceRenderComponent, 
    FloatPrecision
};

const EPSILON: f32 = 0.000001;
fn approx_equal(got: Vector3<FloatPrecision>, expected: Vector3<FloatPrecision>) {
	assert!(
		(got.x - expected.x).abs() < EPSILON &&
		(got.y - expected.y).abs() < EPSILON &&
		(got.z - expected.z).abs() < EPSILON,
		"{}", &format!("Expected: {:?}, Got: {:?}", expected, got)
	)
}

#[test]
fn rotating() {
	let mut r = InstanceRenderComponent::default();
	r.world_rotate(&Deg(90.0), &Vector3::unit_y());
	let v = r.model_matrix() * Vector4::unit_x();
	approx_equal(v.truncate(), -Vector3::unit_z());

	r.model_rotate(&Deg(45.0), &Vector3::unit_y());
	let v = r.model_matrix() * Vector4::unit_x();
	approx_equal(v.truncate(), Vector3::new(-0.7071068, 0.0, -0.7071068));
}

#[test]
fn model_rotating() {
	let mut r = InstanceRenderComponent::default();
	r.model_rotate(&Deg(90.0), &Vector3::unit_y());
	let v = r.model_matrix() * Vector4::unit_x();
	approx_equal(v.truncate(), -Vector3::unit_z());
}

#[test]
fn scaling() {
	let mut r = InstanceRenderComponent::default();
	r.world_scale = Vector3::new(2.0, 2.0, 2.0);
	let v = r.model_matrix() * Vector4::unit_x();
	approx_equal(v.truncate(), Vector3::new(2.0, 0.0, 0.0));
	r.model_scale *= 2.0;
	let v = r.model_matrix() * Vector4::new(1.0, 1.0, 1.0, 0.0);
	approx_equal(v.truncate(), Vector3::new(4.0, 4.0, 4.0));
}

#[test]
fn translating() {
	let mut r = InstanceRenderComponent::default();
	r.world_translation = Vector3::unit_x();
	// w = 1.0 allows translations
	let v = r.model_matrix() * Vector4::new(1.0, 0.0, 0.0, 1.0);
	approx_equal(v.truncate(), Vector3::new(2.0, 0.0, 0.0));
	r.model_translation = Vector3::unit_y();
	let v = r.model_matrix() * Vector4::new(0.0, 0.0, 0.0, 1.0);
	approx_equal(v.truncate(), Vector3::new(1.0, 1.0, 0.0));
}

#[test]
fn all() {
	let mut r = InstanceRenderComponent::default();
	r.model_scale = Vector3::new(2.0, 0.5, 1.0);
	r.model_rotate(&Deg(90.0), &Vector3::unit_z());
	r.model_translation = Vector3::new(3.0, 2.0, 0.0);
	r.world_scale = Vector3::new(1.0, 1.0, 3.0);
	r.world_rotate(&Deg(45.0), &Vector3::unit_x());
	r.world_translation = Vector3::new(0.0, 0.0, 1.0);

	// w = 0.0 -> direction -> translation doesn't apply
	// w = 1.0 -> position -> translation doesn't apply
	let v = r.model_vertex(Vector4::new(1.0, 0.0, 0.0, 1.0));
	approx_equal(v.truncate(), Vector3::new(3.0, 2.8284268, 3.8284273));
}

fn get_aabb() -> AABB {
	let old_min = Position::new(-1.0, -1.0, -1.0);
	let old_max = Position::new(1.0, 1.0, 1.0);

	AABB::new(old_min, old_max)
}

fn get_target_aabb() -> AABB {
	let old_min = Position::new(-5.0, -5.0, -5.0);
	let old_max = Position::new(5.0, 5.0, 5.0);

	AABB::new(old_min, old_max)
}

fn get_aabb_as_cgmath() -> (Point3<FloatPrecision>, Point3<FloatPrecision>) {
	(position_to_point(get_aabb().min), position_to_point(get_aabb().max))
}

fn position_to_point(a: Position) -> Point3<FloatPrecision> {
	Point3::new(a.x as FloatPrecision, a.y as FloatPrecision, a.z as FloatPrecision)
}

fn point_to_vector(a: Point3<FloatPrecision>) -> Vector3<FloatPrecision> {
	Vector3::new(a.x, a.y, a.z)
}

#[test]
fn set_width() {
	let mut instance = InstanceRenderComponent::default();
	instance.set_width(get_aabb().width() as FloatPrecision, 15.5).unwrap();

	let new = get_aabb_as_cgmath();

	let new_min = instance.model_matrix().transform_point(new.0);
	let new_max = instance.model_matrix().transform_point(new.1);

	assert_eq!(new_max.x - new_min.x, 15.5);
}

#[test]
fn set_height() {
	let mut instance = InstanceRenderComponent::default();
	instance.set_height(get_aabb().height() as FloatPrecision, 17.2).unwrap();

	let new = get_aabb_as_cgmath();

	let new_min = instance.model_matrix().transform_point(new.0);
	let new_max = instance.model_matrix().transform_point(new.1);

	assert_eq!(new_max.y - new_min.y, 17.2);
}

#[test]
fn set_depth() {
	let mut instance = InstanceRenderComponent::default();
	instance.set_depth(get_aabb().depth() as FloatPrecision, 0.001).unwrap();

	let new = get_aabb_as_cgmath();

	let new_min = instance.model_matrix().transform_point(new.0);
	let new_max = instance.model_matrix().transform_point(new.1);

	assert_eq!(new_max.z - new_min.z, 0.001);
}

#[test]
fn set_all() {
	let mut instance = InstanceRenderComponent::default();
	instance.set_width(get_aabb().width() as FloatPrecision, 40.0).unwrap();
	instance.set_height(get_aabb().height() as FloatPrecision, 24.123).unwrap();
	instance.set_depth(get_aabb().depth() as FloatPrecision, -0.001321).unwrap();

	let new = get_aabb_as_cgmath();

	let new_min = instance.model_matrix().transform_point(new.0);
	let new_max = instance.model_matrix().transform_point(new.1);

	assert_eq!(new_max.x - new_min.x, 40.0);
	assert_eq!(new_max.y - new_min.y, 24.123);
	assert_eq!(new_max.z - new_min.z, -0.001321);
}

fn test_min(
	mut instance: InstanceRenderComponent,
	self_min: Position,
	target_min: Position,
) {
	instance.set_min(
		&self_min, 
		&target_min
	).unwrap();


	let new_min = instance.model_position(&self_min);

	approx_equal(
		point_to_vector(
			position_to_point(new_min)
		), 
		point_to_vector(
			position_to_point(target_min)
		)
	);
}

#[test]
fn set_min() {
	test_min(
		InstanceRenderComponent::default(), 
		get_aabb().min,
		Position::new(3.0, 6.0, 8.0)
	);
}

#[test]
fn set_min_dims() {
	let mut i = InstanceRenderComponent::default();
	let current_aabb = get_aabb();
	let target_aabb = get_target_aabb();

	i.set_width(current_aabb.width() as f32, target_aabb.width() as f32).unwrap();
	i.set_height(current_aabb.height() as f32, target_aabb.height() as f32).unwrap();
	i.set_depth(current_aabb.depth() as f32, target_aabb.depth() as f32).unwrap();

	test_min(
		i,
		current_aabb.min, 
		Position::new(3.0, 6.0, 8.0)
	);
}

#[test]
fn set_min_model_rotations() {
	let mut i = InstanceRenderComponent::default();
	let current_aabb = get_aabb();

	i.model_rotate(&Deg(123.0), &Vector3::unit_y());
	i.world_rotate(&Deg(321.0), &Vector3::unit_z());

	test_min(
		i,
		current_aabb.min, 
		Position::new(3.0, 6.0, 8.0)
	);
}

// set min but instance has:
/*
	0. None
	1. a custom width and height
	2. a custom model rotation
	3. a custom world rotation
	4. a custom world translation

	5. 1. + 2.
	6. 1. + 3.
	7. 1. + 4.

	8. 2. + 3.
	9. 2. + 4.

	10. 3. + 4.

	11. 1. + 2. + 3.
	12. 1. + 2. + 4.

	13. 1. + 3. + 4.
	
	14. 2. + 3. + 4.
	
	15. 1. + 2. + 3. + 4.
*/