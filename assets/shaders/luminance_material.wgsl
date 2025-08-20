@group(2) @binding(1) var color_texture: texture_2d<f32>;
@group(2) @binding(2) var color_sampler: sampler;

struct VertexOutput {
    @builtin(position) pos: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@vertex
fn vertex(@builtin(vertex_index) idx: u32) -> VertexOutput {
    var pos = array<vec2<f32>, 3>(
        vec2<f32>(-1.0, -3.0),
        vec2<f32>( 3.0,  1.0),
        vec2<f32>(-1.0,  1.0),
    );
    var uv = (pos[idx] + vec2<f32>(1.0)) * 0.5;
    uv.y = 1.0 - uv.y;
    return VertexOutput(vec4<f32>(pos[idx], 0.0, 1.0), uv);
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let color = textureSample(color_texture, color_sampler, in.uv);
    let luminance = 1.0 - dot(color.rgb, vec3<f32>(0.299, 0.587, 0.114));
    return vec4<f32>(luminance, luminance, luminance, 1.0);
}
