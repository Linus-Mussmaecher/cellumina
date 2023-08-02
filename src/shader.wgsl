// Vertex shader

// struct ShaderInfo {
//     time: f32,
//     w: u32,
//     h: u32,
// }

//@group(0) @binding(0)
//var<uniform> shader_info: ShaderInfo;

@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;

@group(0) @binding(1)
var s_diffuse: sampler;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) vert_pos: vec2<f32>,
    //@location(1) time: f32,
    @location(1) tex_coords: vec2<f32>,
};


@vertex
fn vs_main(
    vertex: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = vec4<f32>(vertex.position.xyz, 1.0);
    out.vert_pos = out.clip_position.xy;
    out.tex_coords = vertex.tex_coords;
    //out.vert_pos.x *= f32(shader_info.w) / f32(shader_info.h);
    //out.time = shader_info.time;
    return out;
}


@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(t_diffuse, s_diffuse, in.tex_coords);
}