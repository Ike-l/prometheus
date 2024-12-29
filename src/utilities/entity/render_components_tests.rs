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

fn get_aabb_as_cgmath() -> (Point3<FloatPrecision>, Point3<FloatPrecision>) {
	(position_to_point(get_aabb().min), position_to_point(get_aabb().max))
}

fn position_to_point(a: Position) -> Point3<FloatPrecision> {
	Point3::new(a.x as FloatPrecision, a.y as FloatPrecision, a.z as FloatPrecision)
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

#[test]
fn set_min_basic() {
	let mut instance = InstanceRenderComponent::default();
	let other_render = &InstanceRenderComponent::default();

	instance.set_min(other_render, &get_aabb().min, &Position::new(3.0, 6.0, 8.0)).unwrap();

	let new = get_aabb_as_cgmath();

	let new_min = instance.model_matrix().transform_point(new.0);

	assert_eq!(new_min.x, 3.0);
	assert_eq!(new_min.y, 6.0);
	assert_eq!(new_min.z, 8.0);
}

#[test]
fn set_min_basic2() {
	let mut instance = InstanceRenderComponent::default();
	let other_render = &InstanceRenderComponent::default();

	instance.set_min(
		other_render, 
		&Position::new(-1.0, -1.0, -1.0), 
		&Position::new(10.0, 20.0, 30.0)
	).unwrap();

	let new = get_aabb_as_cgmath();

	let new_min = instance.model_matrix().transform_point(new.0);

	assert_eq!(new_min.x, 10.0);
	assert_eq!(new_min.y, 20.0);
	assert_eq!(new_min.z, 30.0);
}

#[test]
fn set_min_rotation() {
	let mut instance = InstanceRenderComponent::default();
	let mut other_render = InstanceRenderComponent::default();
	
	other_render.world_rotate(&Deg(150.0), &Vector3::unit_x());

	instance.set_min(&other_render, &get_aabb().min, &Position::new(3.0, 4.0, 5.0)).unwrap();

	let new = get_aabb_as_cgmath();

	let new_min = instance.model_matrix().transform_point(new.0);

	assert!((new_min.x - 3.0).abs() < EPSILON);
	assert!((new_min.y - 4.0).abs() < EPSILON);
	assert!((new_min.z - 5.0).abs() < EPSILON);
}

#[test]
fn set_min_rotation_scale() {
	let mut instance = InstanceRenderComponent::default();
	let mut other_render = InstanceRenderComponent::default();
	
	other_render.world_rotate(&Deg(150.0), &Vector3::unit_x());
	other_render.set_width(get_aabb().width() as f32, 5.0).unwrap();
	other_render.set_height(get_aabb().height() as f32, 15.0).unwrap();

	instance.set_min(&other_render, &get_aabb().min, &Position::new(3.0, 4.0, 5.0)).unwrap();

	let new = get_aabb_as_cgmath();

	let new_min = instance.model_matrix().transform_point(new.0);

	assert!((new_min.x - 3.0).abs() < EPSILON);
	assert!((new_min.y - 4.0).abs() < EPSILON);
	assert!((new_min.z - 5.0).abs() < EPSILON);
}

#[test]
fn set_min_playground() {
	let mut instance = InstanceRenderComponent::default();
	let world_translation = Vector3::unit_z();
	instance.world_translation = world_translation;

	let mut other_render = InstanceRenderComponent::default();
	
	other_render.world_rotate(&Deg(150.0), &Vector3::unit_x());
	other_render.set_width(get_aabb().width() as f32, 5.0).unwrap();
	other_render.set_height(get_aabb().height() as f32, 15.0).unwrap();
	other_render.world_translation = Vector3::unit_y();

	let min = other_render.model_matrix().transform_point(Point3::new(3.0, 4.0, 5.0));

	let target_min = Position::new(min.x as f64, min.y as f64, min.z as f64);

	instance.set_min(&other_render, &get_aabb().min, &target_min).unwrap();

	let new = get_aabb_as_cgmath();

	let new_min = instance.model_matrix().transform_point(new.0);

	assert!(((new_min.x - world_translation.x) - (target_min.x as f32)).abs() < EPSILON);
	assert!(((new_min.y - world_translation.y) - (target_min.y as f32)).abs() < EPSILON);
	assert!(((new_min.z - world_translation.z) - (target_min.z as f32)).abs() < EPSILON);
}

/*#[test]
fn set_min_max_basic() {
	let mut instance = InstanceRenderComponent::default();

	let mut other_render = InstanceRenderComponent::default();
	//other_render.model_rotate(&Deg(45.0), &Vector3::unit_y());
	//other_render.model_translation.y += 1.0;
	//other_render.set_width(get_aabb().width() as f32, get_aabb().width() as f32 * 1.75).unwrap();
	//other_render.set_height(get_aabb().height() as f32, get_aabb().height() as f32 * 1.75 * 2.0).unwrap();
	//other_render.set_depth(get_aabb().depth() as f32, get_aabb().depth() as f32 * 1.75 * 3.0).unwrap();
	//other_render.world_rotate(&Deg(90.0), &Vector3::unit_x());
	//other_render.world_translation.z += 1.0;
	//other_render.world_scale.x *= 2.0

	let (target_min_x, target_min_y, target_min_z) = (2.0, 4.0, 8.0);
	let (target_width, target_height, target_depth) = (1.0, 3.0, 9.0);

	let target_min = Position::new(
		target_min_x, 
		target_min_y, 
		target_min_z,
	);

	let target_max = Position::new(
		target_min_x + target_width, 
		target_min_y + target_height, 
		target_min_z + target_depth,
	);


	instance.set_min_max(&other_render, &get_aabb(), &target_min, &target_max).unwrap();

	let (mut new_min, mut new_max) = get_aabb_as_cgmath();

	println!("new_min: {new_min:?}, new_max: {new_max:?}");

	new_min = instance.model_matrix().transform_point(new_min);
	new_max = instance.model_matrix().transform_point(new_max);

	println!("new_min: {new_min:?}, new_max: {new_max:?}");

	approx_equal(
		Vector3::new(new_min.x, new_min.y, new_min.z), 
		Vector3::new(target_min.x as f32, target_min.y as f32, target_min.z as f32)
	);

	approx_equal(
		Vector3::new(new_max.x, new_max.y, new_max.z), 
		Vector3::new(target_max.x as f32, target_max.y as f32, target_max.z as f32)
	);
}*/

#[test]
fn set_min_max_basic() {
	let mut instance = InstanceRenderComponent::default();
	let other_render = InstanceRenderComponent::default();

	let target_min = Position::new(10.0, 20.0, 30.0);
	let target_max = Position::new(20.0, 40.0, 80.0);

	instance.set_min_max(&other_render, &get_aabb(), &target_min, &target_max).unwrap();

	let should_be_target_min = instance.model_matrix().transform_point(get_aabb_as_cgmath().0);
	let should_be_target_max = instance.model_matrix().transform_point(get_aabb_as_cgmath().1);

	approx_equal(point_to_vector(should_be_target_max), point_to_vector(target_max.position.cast::<FloatPrecision>().unwrap()));
	approx_equal(point_to_vector(should_be_target_min), point_to_vector(target_min.position.cast::<FloatPrecision>().unwrap()));
}

#[test]
fn set_min_max_with_dimensions() {
	let mut instance = InstanceRenderComponent::default();
	let mut other_render = InstanceRenderComponent::default();
	
	other_render.set_width(get_aabb().width() as f32, 45.0).unwrap();
	other_render.set_height(get_aabb().height() as f32, 0.032).unwrap();
	other_render.set_depth(get_aabb().depth() as f32, 3.0).unwrap();

	let target_min = Position::new(10.0, 20.0, 30.0);
	let target_max = Position::new(72.1, 40.0321, 0.23);

	instance.set_min_max(&other_render, &get_aabb(), &target_min, &target_max).unwrap();

	let expected_target_min = instance.model_matrix().transform_point(get_aabb_as_cgmath().0);
	let expected_target_max = instance.model_matrix().transform_point(get_aabb_as_cgmath().1);

	approx_equal(point_to_vector(expected_target_max), point_to_vector(target_max.position.cast::<FloatPrecision>().unwrap()));
	approx_equal(point_to_vector(expected_target_min), point_to_vector(target_min.position.cast::<FloatPrecision>().unwrap()));
}

fn point_to_vector(a: Point3<FloatPrecision>) -> Vector3<FloatPrecision> {
	Vector3::new(a.x, a.y, a.z)
}