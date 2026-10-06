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
// b1.8.1 lightmap (texture unit 1), sampled at the draw's brightness coordinates.
@group(0) @binding(1) var lightmap: texture_2d<f32>;
@group(0) @binding(2) var lightmap_samp: sampler;
@group(1) @binding(0) var tex: texture_2d<f32>;
@group(1) @binding(1) var samp: sampler;

struct In {
    @location(0) pos: vec3<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) color: vec4<f32>,
    @location(3) light: u32,
    @location(4) cutoff: f32,
};
struct Out {
    @builtin(position) clip: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) color: vec4<f32>,
    @location(2) dist: f32,
    @location(3) lm: vec2<f32>,
    @location(4) @interpolate(flat) lit: u32,
    @location(5) @interpolate(flat) cutoff: f32,
};
@vertex fn vs_main(v: In) -> Out {
    var o: Out;
    o.clip = u.view_proj * vec4<f32>(v.pos,1.0);
    o.uv = v.uv;
    o.cutoff = v.cutoff;
    o.color = v.color;
    o.dist = length((u.fog_view * vec4<f32>(v.pos,1.0)).xyz);
    o.lit = select(1u, 0u, v.light == 0xffffffffu);
    o.lm = (vec2<f32>(f32(v.light & 0xffffu), f32(v.light >> 16u)) + 8.0) / 256.0;
    return o;
}
fn light(i: Out) -> vec4<f32> {
    let l = textureSample(lightmap, lightmap_samp, i.lm);
    return select(vec4<f32>(1.0), l, i.lit == 1u);
}
fn fog(c: vec4<f32>, dist: f32) -> vec4<f32> {
    if u.fog.z < 0.5 { return c; }
    var f = clamp((u.fog.y-dist)/(u.fog.y-u.fog.x),0.0,1.0);
    if u.fog.z > 1.5 { f = clamp(exp(-u.fog.x*dist),0.0,1.0); }
    return vec4<f32>(mix(u.fog_color.rgb,c.rgb,f),c.a);
}
@fragment fn fs_test(i: Out) -> @location(0) vec4<f32> {
    let c = textureSample(tex,samp,i.uv)*i.color*light(i);
    if c.a <= i.cutoff { discard; }
    return fog(c,i.dist);
}
@fragment fn fs_main(i: Out) -> @location(0) vec4<f32> {
    return fog(textureSample(tex,samp,i.uv)*i.color*light(i),i.dist);
}
