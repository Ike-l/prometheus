// LIGHT
struct Light {
    position: vec3<f32>,          
    light_type: f32,                          
    colour: vec3<f32>,                        
    inner_cone: f32,             
    intensity: vec3<f32>,         
    outer_cone: f32,             
    direction: vec3<f32>,                    
    attenuation: vec3<f32>,                 
    projection: mat4x4<f32>,
    view_projection: mat4x4<f32>, 
};

@group(0)
@binding(0)
var<storage, read> lights: array<Light>;

// SHADOW
@group(1)
@binding(0)
var t_shadow: texture_depth_2d_array;

@group(1)
@binding(1)
var sampler_shadow: sampler_comparison;

// CAMERA
struct Camera {
    view_projection: mat4x4<f32>,
    inverse_projection: mat4x4<f32>,
    position: vec3<f32>,
};

@group(2)
@binding(0)
var<uniform> camera: Camera;

// MATERIAL
struct Material {
    ambience: vec4<f32>,
    specularity: vec4<f32>,
    diffusivity: vec4<f32>,
};

@group(3)
@binding(0)
var<uniform> material: Material;

// TEXTURE
@group(4)
@binding(0)
var s_texture: texture_2d<f32>;

@group(4)
@binding(1)
var sampler_texture: sampler;

@group(5)
@binding(0)
var<uniform> bind_group_flags: u32;

//
struct Object {
    @location(0) position: vec3<f32>,
    @location(1) normals: vec3<f32>,
    @location(2) colour: vec3<f32>,
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

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) colour: vec4<f32>,
    @location(1) world_position: vec4<f32>,
};

@vertex
fn main_vs(
    object: Object,
    instance: Instance,
) -> VertexOutput {
    let model_matrix = mat4x4<f32>(
        instance.model_matrix_0,
        instance.model_matrix_1,
        instance.model_matrix_2,
        instance.model_matrix_3
    );
    let world_position = model_matrix * vec4<f32>(object.position, 1.0);
    let clip_position = camera.view_projection * world_position;
    
    var out: VertexOutput;
    out.clip_position = clip_position;
    out.colour = vec4<f32>(object.colour, 1.0);
    out.world_position = world_position;

    return out;
}

@fragment 
fn main_fs(
    in: VertexOutput
) -> @location(0) vec4<f32> {
    let lights_f = (bind_group_flags >> 0) & 1;
    let shadow_f = (bind_group_flags >> 1) & 1;
    let camera_f = (bind_group_flags >> 2) & 1;
    let material_f = (bind_group_flags >> 3) & 1;
    let texture_f = (bind_group_flags >> 4) & 1;

    //return vec4<f32>(material, 0.0, 0.0, 1.0);
    // var o = 1.0;
    // if material_f {
    //     o = material.ambience[3];
    // }
    // return vec4<f32>(o, o, o, 1.0);
    ///let f = lights[0].light_type * 1.0;
    //let s = lights[0].outer_cone * 0.0;
    //let t = lights[0].outer_cone * 0.0;
    //return vec4<f32>(1.0, 1.0, 1.0, 1.0);
    //return vec4<f32>(material_f, material_f, material_f, 1.0);

    let light1 = lights[0];
    let shadow1 = fetch_shadow(0, light1.view_projection * in.world_position);

    let light2 = lights[1];
    let shadow2 = fetch_shadow(0, light2.view_projection * in.world_position);

    let light3 = lights[2];
    let shadow3 = fetch_shadow(0, light3.view_projection * in.world_position);

    let light4 = lights[3];
    let shadow4 = fetch_shadow(0, light4.view_projection * in.world_position);

    let light5 = lights[4];
    let shadow5 = fetch_shadow(0, light5.view_projection * in.world_position);

    let light6 = lights[5];
    let shadow6 = fetch_shadow(0, light6.view_projection * in.world_position);

    let s_r = shadow1 * shadow2 * shadow3 * shadow4 * shadow5 * shadow6;

//     let hom = light.view_projection * in.world_position;

//    let test_pos = vec4<f32>(0.0, 100.0, 0.0, 1.0);  // A known position
// let test_hom = light.view_projection * test_pos;

    return vec4<f32>(s_r, 0.0, 0.0, 0.5);


}

// fn fetch_shadow(light_space_position: vec4<f32>) -> f32 {
//     // Normalize light space coordinates (typically in [-1,1])
//     let light_coords = light_space_position.xy / light_space_position.w * 0.5 + 0.5;  // NDC to [0, 1] space
    
//     // Fetch depth value from shadow map at light coordinates using textureSampleCompare
//     let depth = textureSampleCompare(t_shadow, sampler_shadow, light_coords, light_space_position.z);
    
//     // Return 0.0 (shadow) or 1.0 (no shadow)
//     return depth;
// }

// chatgpt
// fn fetch_shadow(light_id: u32, homogeneous_coords: vec4<f32>) -> f32 {
//     if (homogeneous_coords.w <= 0.0) {
//         return 1.0;
//     }

//     // compensate for the Y-flip difference between the NDC and texture coordinates
//     let flip_correction = vec2<f32>(0.5, -0.5);

//     // compute texture coordinates for shadow lookup
//     let proj_correction = 1.0 / homogeneous_coords.w;
//     let light_local = homogeneous_coords.xy * flip_correction * proj_correction + vec2<f32>(0.5, 0.5);

//     // Add a small bias to the depth value to reduce acne
//     let depth_bias = 0.0001; // Experiment with this value
//     let biased_depth = homogeneous_coords.z * proj_correction + depth_bias;

//     // Size of the filter (e.g., 2 for a 5x5 grid)
//     let filter_size = 2; 
//     let texel_size = vec2<f32>(1.0 / 2048.0, 1.0 / 2048.0); // Assuming the shadow map is 1024x1024, adjust accordingly

//     var shadow_value: f32 = 0.0;

//     var dx: i32;
//     var dy: i32;
//     for (dx = -filter_size; dx <= filter_size; dx = dx + 1) { // Loop from -filter_size to filter_size
//         for (dy = -filter_size; dy <= filter_size; dy = dy + 1) {
//             let offset = vec2<f32>(f32(dx), f32(dy)) * texel_size;
//             shadow_value += textureSampleCompareLevel(t_shadow, sampler_shadow, light_local + offset, i32(light_id), biased_depth);
//         }
//     }

//     // Average result to get smooth shadow
//     return shadow_value / f32((2 * filter_size + 1) * (2 * filter_size + 1));
// }

fn fetch_shadow(light_id: u32, homogeneous_coords: vec4<f32>) -> f32 {
    // if behind light direction
    if (homogeneous_coords.w <= 0.0) {
        return 1.0;
    }
    
    let flip_correction = vec2<f32>(0.5, -0.5);
    
    let proj_correction = 1.0 / homogeneous_coords.w;
    let light_local = homogeneous_coords.xy * flip_correction * proj_correction + vec2<f32>(0.5, 0.5);
    
    return textureSampleCompareLevel(t_shadow, sampler_shadow, light_local, i32(light_id), homogeneous_coords.z * proj_correction);
}

/*
const c_ambient: vec3<f32> = vec3<f32>(0.05, 0.05, 0.05);
const c_max_lights: u32 = 10u;

@fragment
fn fs_main(vertex: VertexOutput) -> @location(0) vec4<f32> {
    let normal = normalize(vertex.world_normal);
    // accumulate color
    var color: vec3<f32> = c_ambient;
    for(var i = 0u; i < min(u_globals.num_lights.x, c_max_lights); i += 1u) {
        let light = s_lights[i];
        // project into the light space
        let shadow = fetch_shadow(i, light.proj * vertex.world_position);
        // compute Lambertian diffuse term
        let light_dir = normalize(light.pos.xyz - vertex.world_position.xyz);
        let diffuse = max(0.0, dot(normal, light_dir));
        // add light contribution
        color += shadow * diffuse * light.color.xyz;
    }
    // multiply the light by material color
    return vec4<f32>(color, 1.0) * u_entity.color;
}
*/