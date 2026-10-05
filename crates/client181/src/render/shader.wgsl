// Fixed-function OpenGL 1.x as b1.7.3 used it: texture * vertex color (GL_MODULATE),
// alpha test (GL_GREATER ref), then linear fog on eye-radial distance.

struct Uniforms {
    view_proj: mat4x4<f32>,
    fog_view: mat4x4<f32>,
    model: mat4x4<f32>,
    color: vec4<f32>,
    fog_color: vec4<f32>,
    // x = fog start, y = fog end, z = fog enabled (0/1), w = alpha-test reference (< 0: off)
    fog: vec4<f32>,
};

@group(0) @binding(0) var<uniform> u: Uniforms;
@group(0) @binding(1) var tex: texture_2d<f32>;
@group(0) @binding(2) var samp: sampler;
@group(1) @binding(0) var<storage, read> origins: array<vec4<f32>>;
// b1.8.1 lightmap: texture unit 1, texture matrix scale(1/256) translate(8) on the brightness.
@group(1) @binding(1) var lightmap: texture_2d<f32>;
@group(1) @binding(2) var lightmap_samp: sampler;

struct VIn {
    @location(0) pos: vec3<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) color: vec4<f32>,
    @location(3) light: u32,
};

struct VInSky {
    @location(0) pos: vec3<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) color: vec4<f32>,
};

struct VOut {
    @builtin(position) clip: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) color: vec4<f32>,
    @location(2) dist: f32,
    @location(3) lm: vec2<f32>,
};

@vertex
fn vs_terrain(v: VIn, @builtin(instance_index) inst: u32) -> VOut {
    let rel = v.pos + origins[inst].xyz;
    var o: VOut;
    o.clip = u.view_proj * vec4<f32>(rel, 1.0);
    o.uv = v.uv;
    o.color = v.color;
    o.dist = length((u.fog_view * vec4<f32>(rel,1.0)).xyz);
    o.lm = (vec2<f32>(f32(v.light & 0xffffu), f32(v.light >> 16u)) + 8.0) / 256.0;
    return o;
}

@vertex
fn vs_sky(v: VInSky) -> VOut {
    let p = u.model * vec4<f32>(v.pos, 1.0);
    var o: VOut;
    o.clip = u.view_proj * p;
    o.uv = v.uv;
    o.color = v.color;
    o.dist = length((u.fog_view * p).xyz);
    o.lm = vec2<f32>(0.0, 0.0);
    return o;
}

@fragment
fn fs_main(i: VOut) -> @location(0) vec4<f32> {
    return finish(textureSample(tex, samp, i.uv) * i.color * u.color, i);
}

@fragment
fn fs_terrain(i: VOut) -> @location(0) vec4<f32> {
    let lit = textureSample(lightmap, lightmap_samp, i.lm);
    return finish(textureSample(tex, samp, i.uv) * i.color * u.color * lit, i);
}

fn finish(c0: vec4<f32>, i: VOut) -> vec4<f32> {
    var c = c0;
    if (u.fog.w >= 0.0 && c.a <= u.fog.w) {
        discard;
    }
    if (u.fog.z > 0.5) {
        var f = clamp((u.fog.y - i.dist) / (u.fog.y - u.fog.x), 0.0, 1.0);
        if (u.fog.z > 1.5) { f = clamp(exp(-u.fog.x*i.dist),0.0,1.0); }
        c = vec4<f32>(mix(u.fog_color.rgb, c.rgb, f), c.a);
    }
    return c;
}
