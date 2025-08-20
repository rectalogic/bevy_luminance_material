@group(1) @binding(1) var color_texture: texture_2d<f32>;
@group(1) @binding(2) var color_sampler: sampler;

struct VertexOutput {
    @builtin(position) pos: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let color = textureSample(color_texture, color_sampler, in.uv);
    let luminance = dot(color.rgb, vec3<f32>(0.299, 0.587, 0.114));
    return vec4<f32>(luminance, luminance, luminance, 1.0);
}
