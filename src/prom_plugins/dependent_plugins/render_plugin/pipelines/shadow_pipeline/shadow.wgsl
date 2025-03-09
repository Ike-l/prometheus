struct Camera {
    view_projection: mat4x4<f32>,
};

@group(0)
@binding(0)
var<uniform> camera: Camera;

struct Object {
    @location(0) position: vec3<f32>,
    @location(1) normals: vec3<f32>,
    @location(2) colour: vec4<f32>,
    @location(3) texture_coords: vec2<f32>,
};

struct Instance {
    @location(4) model_matrix_0: vec4<f32>,
    @location(5) model_matrix_1: vec4<f32>,
    @location(6) model_matrix_2: vec4<f32>,
    @location(7) model_matrix_3: vec4<f32>,

    @location(8) normal_matrix_0: vec4<f32>,
    @location(9) normal_matrix_1: vec4<f32>,
    @location(10) normal_matrix_2: vec4<f32>,
    @location(11) normal_matrix_3: vec4<f32>,
};

@vertex
fn main(
    object: Object,
    instance: Instance,
) -> @builtin(position) vec4<f32> {
    let model_matrix = mat4x4<f32>(
        instance.model_matrix_0,
        instance.model_matrix_1,
        instance.model_matrix_2,
        instance.model_matrix_3
    );

    let position = camera.view_projection * model_matrix * vec4<f32>(object.position, 1.0);
    
    return position;
}