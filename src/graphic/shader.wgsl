// Vertex shader

// struct ShaderInfo {
//     w: u32,
//     h: u32,
//     cells_w: u32,
//     cells_h: u32,
// }


// @group(1) @binding(0)
// var<uniform> shader_info: ShaderInfo;

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
    out.tex_coords = vertex.tex_coords;
    out.vert_pos = vertex.position.xy;

    // let ratio_tex = f32(shader_info.cells_w) / f32(shader_info.cells_h);
    // let ratio_window = f32(shader_info.w) / f32(shader_info.h);

    // if ratio_tex > ratio_window{
    //     out.tex_coords.y *= ratio_tex / ratio_window;
    //     out.tex_coords.y -= (ratio_tex / ratio_window - 1.) / 2.;
    // }else{
    //     out.tex_coords.x *= ratio_window / ratio_tex;
    //     out.tex_coords.x -= (ratio_window / ratio_tex - 1.) / 2.;
    // }


    return out;
}


@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(t_diffuse, s_diffuse, in.tex_coords);
}