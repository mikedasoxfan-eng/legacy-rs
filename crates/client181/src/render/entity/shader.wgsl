// RenderLiving: fixed-function primary color * GL_MODULATE texture, then the
// same eye-radial linear fog as terrain. Primary color is computed per face on
// the CPU, preserving GL_FLAT lighting and the original packed BYTE normals.
struct Uniforms {
    view_proj: mat4x4<f32>,
    fog_view: mat4x4<f32>,
    fog_color: vec4<f32>,
    fog: vec4<f32>,
};
@group(0) @binding(0) var<uniform> u: Uniforms;
@group(1) @binding(0) var tex: texture_2d<f32>;
@group(1) @binding(1) var samp: sampler;

struct In {
    @location(0) pos: vec3<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) color: vec4<f32>,
};
struct Out {
    @builtin(position) clip: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) color: vec4<f32>,
    @location(2) dist: f32,
};
@vertex fn vs_main(v: In) -> Out {
    var o: Out;
    o.clip = u.view_proj * vec4<f32>(v.pos,1.0);
    o.uv = v.uv;
    o.color = v.color;
    o.dist = length((u.fog_view * vec4<f32>(v.pos,1.0)).xyz);
    return o;
}
fn fog(c: vec4<f32>, dist: f32) -> vec4<f32> {
    if u.fog.z < 0.5 { return c; }
    var f = clamp((u.fog.y-dist)/(u.fog.y-u.fog.x),0.0,1.0);
    if u.fog.z > 1.5 { f = clamp(exp(-u.fog.x*dist),0.0,1.0); }
    return vec4<f32>(mix(u.fog_color.rgb,c.rgb,f),c.a);
}
@fragment fn fs_test(i: Out) -> @location(0) vec4<f32> {
    let c = textureSample(tex,samp,i.uv)*i.color;
    if c.a <= 0.1 { discard; }
    return fog(c,i.dist);
}
@fragment fn fs_main(i: Out) -> @location(0) vec4<f32> {
    return fog(textureSample(tex,samp,i.uv)*i.color,i.dist);
}
