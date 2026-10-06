//! Time of day, sky and fog colors, and the sky geometry (`World`, `WorldProvider`,
//! `RenderGlobal` constructor and `renderSky`).

use worldgen::math;
use worldgen::JavaRandom;

const PI_F: f32 = std::f64::consts::PI as f32;

/// `WorldProvider.calculateCelestialAngle`
pub fn celestial_angle(world_time: i64, partial: f32) -> f32 {
    let t = (world_time % 24000) as i32;
    let mut a = (t as f32 + partial) / 24000.0f32 - 0.25f32;
    if a < 0.0 {
        a += 1.0;
    }
    if a > 1.0 {
        a -= 1.0;
    }
    let raw = a;
    a = 1.0f32 - ((((a as f64) * std::f64::consts::PI).cos() + 1.0) / 2.0) as f32;
    raw + (a - raw) / 3.0f32
}

fn daylight(angle: f32) -> f32 {
    (math::cos(angle * PI_F * 2.0f32) * 2.0f32 + 0.5f32).clamp(0.0, 1.0)
}

/// `World.calculateSkylightSubtracted` (interpolated rain and effective thunder).
pub fn skylight_subtracted(angle: f32, rain: f32, thunder: f32) -> i32 {
    let mut v = 1.0f32 - (math::cos(angle * PI_F * 2.0f32) * 2.0f32 + 0.5f32);
    v = v.clamp(0.0, 1.0);
    v = 1.0f32 - v;
    v = (v as f64 * (1.0 - (rain * 5.0f32) as f64 / 16.0)) as f32;
    v = (v as f64 * (1.0 - (thunder * 5.0f32) as f64 / 16.0)) as f32;
    v = 1.0f32 - v;
    (v * 11.0f32) as i32
}

/// `java.awt.Color.HSBtoRGB`
pub fn hsb_to_rgb(hue: f32, sat: f32, bri: f32) -> u32 {
    let (r, g, b);
    if sat == 0.0 {
        let v = (bri * 255.0f32 + 0.5f32) as i32;
        r = v;
        g = v;
        b = v;
    } else {
        let h = (hue - hue.floor()) * 6.0f32;
        let f = h - h.floor();
        let p = bri * (1.0f32 - sat);
        let q = bri * (1.0f32 - sat * f);
        let t = bri * (1.0f32 - sat * (1.0f32 - f));
        let c = |v: f32| (v * 255.0f32 + 0.5f32) as i32;
        (r, g, b) = match h as i32 {
            0 => (c(bri), c(t), c(p)),
            1 => (c(q), c(bri), c(p)),
            2 => (c(p), c(bri), c(t)),
            3 => (c(p), c(q), c(bri)),
            4 => (c(t), c(p), c(bri)),
            5 => (c(bri), c(p), c(q)),
            _ => (0, 0, 0),
        };
    }
    0xff000000 | ((r as u32) << 16) | ((g as u32) << 8) | b as u32
}

/// `BiomeGenBase.getSkyColorByTemp`
pub fn sky_color_by_temp(temp: f32) -> u32 {
    let t = (temp / 3.0f32).clamp(-1.0, 1.0);
    hsb_to_rgb(224.0f32 / 360.0f32 - t * 0.05f32, 0.5f32 + t * 0.1f32, 1.0f32)
}

/// `World.func_4079_a`: sky color at the camera. `temp` is `WorldChunkManager.getTemperature`.
pub fn sky_color(angle: f32, temp: f32, rain: f32, thunder: f32, flash: i32, partial: f32) -> [f32; 3] {
    let d = daylight(angle);
    let c = sky_color_by_temp(temp);
    let mut rgb = [(c >> 16 & 255) as f32 / 255.0 * d, (c >> 8 & 255) as f32 / 255.0 * d, (c & 255) as f32 / 255.0 * d];
    desaturate(&mut rgb, rain, 0.6, 12.0f32 / 16.0f32);
    desaturate(&mut rgb, thunder, 0.2, 12.0f32 / 16.0f32);
    if flash > 0 {
        let a = (flash as f32 - partial).min(1.0) * 0.45f32;
        for (c, target) in rgb.iter_mut().zip([0.8f32, 0.8, 1.0]) {
            *c = *c * (1.0f32 - a) + target * a;
        }
    }
    rgb
}

fn desaturate(rgb: &mut [f32; 3], strength: f32, gray_scale: f32, amount: f32) {
    if strength > 0.0 {
        let gray = (rgb[0] * 0.3f32 + rgb[1] * 0.59f32 + rgb[2] * 0.11f32) * gray_scale;
        let k = 1.0f32 - strength * amount;
        for c in rgb {
            *c = *c * k + gray * (1.0f32 - k);
        }
    }
}

/// `WorldProvider.func_4096_a`
pub fn fog_color_base(angle: f32) -> [f32; 3] {
    let d = daylight(angle);
    [
        192.0f32 / 255.0f32 * (d * 0.94f32 + 0.06f32),
        216.0f32 / 255.0f32 * (d * 0.94f32 + 0.06f32),
        1.0f32 * (d * 0.91f32 + 0.09f32),
    ]
}

/// `EntityLiving.getLook(partial)` for a camera with these (interpolated) angles.
pub fn look(yaw: f32, pitch: f32, prev_yaw: f32, prev_pitch: f32, partial: f32) -> [f64; 3] {
    let r = std::f32::consts::PI / 180.0f32;
    let pi = std::f32::consts::PI;
    if partial == 1.0 {
        let c = math::cos(-yaw * r - pi);
        let s = math::sin(-yaw * r - pi);
        let cp = -math::cos(-pitch * r);
        let sp = math::sin(-pitch * r);
        [(s * cp) as f64, sp as f64, (c * cp) as f64]
    } else {
        let p = prev_pitch + (pitch - prev_pitch) * partial;
        let y = prev_yaw + (yaw - prev_yaw) * partial;
        let c = math::cos(-y * r - pi);
        let s = math::sin(-y * r - pi);
        let cp = -math::cos(-p * r);
        let sp = math::sin(-p * r);
        [(s * cp) as f64, sp as f64, (c * cp) as f64]
    }
}

/// `EntityRenderer.updateFogColor` for render distance Far, out of water. `look` is the view
/// entity's `getLook(partial)`: from b1.8.1, at Far and Normal the fog takes on the sunrise
/// colour when looking toward the sun.
#[allow(clippy::too_many_arguments)]
pub fn fog_color(
    angle: f32,
    temp: f32,
    render_distance: i32,
    rain: f32,
    thunder: f32,
    flash: i32,
    partial: f32,
    look: [f64; 3],
) -> [f32; 3] {
    let mut k = 1.0f32 / (4 - render_distance) as f32;
    k = 1.0f32 - (k as f64).powf(0.25) as f32;
    let sky = sky_color(angle, temp, rain, thunder, flash, partial);
    let mut f = sunrise_fog(fog_color_base(angle), angle, render_distance, look);
    for i in 0..3 {
        f[i] += (sky[i] - f[i]) * k;
    }
    if rain > 0.0 {
        f[0] *= 1.0f32 - rain * 0.5f32;
        f[1] *= 1.0f32 - rain * 0.5f32;
        f[2] *= 1.0f32 - rain * 0.4f32;
    }
    if thunder > 0.0 {
        for c in &mut f {
            *c *= 1.0f32 - thunder * 0.5f32;
        }
    }
    f
}

/// b1.8.1 `updateFogColor`: at Far and Normal, blend the fog toward the sunrise colour by how
/// directly `look` faces the sun (`World.func_35456_d` picks the east or west horizon).
pub fn sunrise_fog(mut f: [f32; 3], angle: f32, render_distance: i32, look: [f64; 3]) -> [f32; 3] {
    if render_distance < 2 {
        let sun_z = if math::sin(angle * std::f32::consts::PI * 2.0f32) > 0.0 { 1.0f64 } else { -1.0 };
        let mut d = (look[0] * 0.0 + look[1] * 0.0 + look[2] * sun_z) as f32;
        if d < 0.0 {
            d = 0.0;
        }
        if d > 0.0 {
            if let Some(c) = sunrise_colors(angle) {
                d *= c[3];
                f[0] = f[0] * (1.0f32 - d) + c[0] * d;
                f[1] = f[1] * (1.0f32 - d) + c[1] * d;
                f[2] = f[2] * (1.0f32 - d) + c[2] * d;
            }
        }
    }
    f
}

/// `EntityRenderer.updateFogColor` in the Nether: `WorldProviderHell`'s constant fog color
/// (0.2, 0.03, 0.03) pulled toward the sky color by render distance, then rain and thunder
/// darkening (their strengths carry over from the Overworld's level info).
pub fn nether_fog_color(sky: [f32; 3], render_distance: i32, rain: f32, thunder: f32) -> [f32; 3] {
    let mut k = 1.0f32 / (4 - render_distance) as f32;
    k = 1.0f32 - (k as f64).powf(0.25) as f32;
    let mut f = [0.2f32, 0.03f32, 0.03f32];
    for i in 0..3 {
        f[i] += (sky[i] - f[i]) * k;
    }
    if rain > 0.0 {
        f[0] *= 1.0f32 - rain * 0.5f32;
        f[1] *= 1.0f32 - rain * 0.5f32;
        f[2] *= 1.0f32 - rain * 0.4f32;
    }
    if thunder > 0.0 {
        for c in &mut f {
            *c *= 1.0f32 - thunder * 0.5f32;
        }
    }
    f
}

/// `WorldProvider.calcSunriseSunsetColors`
pub fn sunrise_colors(angle: f32) -> Option<[f32; 4]> {
    let span = 0.4f32;
    let c = math::cos(angle * PI_F * 2.0f32) - 0.0f32;
    let mid = -0.0f32;
    if c >= mid - span && c <= mid + span {
        let f = (c - mid) / span * 0.5f32 + 0.5f32;
        let mut a = 1.0f32 - (1.0f32 - math::sin(f * PI_F)) * 0.99f32;
        a *= a;
        Some([f * 0.3f32 + 0.7f32, f * f * 0.7f32 + 0.2f32, f * f * 0.0f32 + 0.2f32, a])
    } else {
        None
    }
}

/// `World.getStarBrightness`
pub fn star_brightness(angle: f32) -> f32 {
    let v = (1.0f32 - (math::cos(angle * PI_F * 2.0f32) * 2.0f32 + 0.75f32)).clamp(0.0, 1.0);
    v * v * 0.5f32
}

/// The sky plane (`glSkyList`, y = +16) or the void plane (`glSkyList2`, y = -16), as triangles.
pub fn sky_plane(y: f32, upper: bool) -> Vec<[f32; 3]> {
    let s = 64i32;
    let n = 256 / s + 2;
    let mut out = Vec::new();
    let mut x = -s * n;
    while x <= s * n {
        let mut z = -s * n;
        while z <= s * n {
            let (x0, x1, z0, z1) = (x as f32, (x + s) as f32, z as f32, (z + s) as f32);
            let q = if upper {
                [[x0, y, z0], [x1, y, z0], [x1, y, z1], [x0, y, z1]]
            } else {
                [[x1, y, z0], [x0, y, z0], [x0, y, z1], [x1, y, z1]]
            };
            out.extend_from_slice(&[q[0], q[1], q[2], q[0], q[2], q[3]]);
            z += s;
        }
        x += s;
    }
    out
}

/// `RenderGlobal.renderStars`, as triangles.
pub fn stars() -> Vec<[f32; 3]> {
    let mut r = JavaRandom::new(10842);
    let mut out = Vec::new();
    for _ in 0..1500 {
        let mut x = (r.next_float() * 2.0f32 - 1.0f32) as f64;
        let mut y = (r.next_float() * 2.0f32 - 1.0f32) as f64;
        let mut z = (r.next_float() * 2.0f32 - 1.0f32) as f64;
        let size = (0.25f32 + r.next_float() * 0.25f32) as f64;
        let mut d = x * x + y * y + z * z;
        if d < 1.0 && d > 0.01 {
            d = 1.0 / d.sqrt();
            x *= d;
            y *= d;
            z *= d;
            let (px, py, pz) = (x * 100.0, y * 100.0, z * 100.0);
            let a = x.atan2(z);
            let (sa, ca) = (a.sin(), a.cos());
            let b = (x * x + z * z).sqrt().atan2(y);
            let (sb, cb) = (b.sin(), b.cos());
            let c = r.next_double() * std::f64::consts::PI * 2.0;
            let (sc, cc) = (c.sin(), c.cos());
            let mut q = [[0.0f32; 3]; 4];
            for (i, v) in q.iter_mut().enumerate() {
                let i = i as i32;
                let e = 0.0;
                let u = ((i & 2) - 1) as f64 * size;
                let w = ((i + 1 & 2) - 1) as f64 * size;
                let rx = u * cc - w * sc;
                let rz = w * cc + u * sc;
                let ry = rx * sb + e * cb;
                let t = e * sb - rx * cb;
                let fx = t * sa - rz * ca;
                let fz = rz * sa + t * ca;
                *v = [(px + fx) as f32, (py + ry) as f32, (pz + fz) as f32];
            }
            out.extend_from_slice(&[q[0], q[1], q[2], q[0], q[2], q[3]]);
        }
    }
    out
}

/// `World.func_628_d`: cloud color, including rain and thunder.
pub fn cloud_color(angle: f32, rain: f32, thunder: f32) -> [f32; 3] {
    let d = daylight(angle);
    let mut rgb = [1.0f32; 3];
    desaturate(&mut rgb, rain, 0.6, 0.95);
    rgb[0] *= d * 0.9f32 + 0.1f32;
    rgb[1] *= d * 0.9f32 + 0.1f32;
    rgb[2] *= d * 0.85f32 + 0.15f32;
    desaturate(&mut rgb, thunder, 0.2, 0.95);
    rgb
}

/// `RenderGlobal.renderClouds` Fast branch. Camera-relative flat quads, no scale.
pub fn flat_clouds(cam_x: f64, cam_y: f64, cam_z: f64, offset: f32, color: [f32; 3]) -> Vec<super::Vertex> {
    let step = 0.5f32 / 1024.0f32;
    // b1.8.1: (double)(cloudOffsetX + partial) * (double)0.03F (plus World.field_35467_J's time
    // warp, which nothing ever sets), no longer a float product.
    let mut px = cam_x + offset as f64 * 0.03f32 as f64;
    let mut pz = cam_z;
    px -= (math::floor_double(px / 2048.0) * 2048) as f64;
    pz -= (math::floor_double(pz / 2048.0) * 2048) as f64;
    let y = 128.0f32 - cam_y as f32 + 0.33f32; // b1.8.1 getCloudHeight: 128
    let u = (px * step as f64) as f32;
    let v = (pz * step as f64) as f32;
    let ch = |c: f32| ((c * 255.0f32) as i32).clamp(0, 255) as u8;
    let color = [ch(color[0]), ch(color[1]), ch(color[2]), ch(0.8)];
    let mut out = Vec::with_capacity(1024);
    for x in (-256..256).step_by(32) {
        for z in (-256..256).step_by(32) {
            for (xx, zz) in [(x, z + 32), (x + 32, z + 32), (x + 32, z), (x, z)] {
                out.push(super::Vertex {
                    pos: [xx as f32, y, zz as f32],
                    uv: [xx as f32 * step + u, zz as f32 * step + v],
                    color,
                    light: 0,
                });
            }
        }
    }
    out
}

/// `RenderGlobal.renderCloudsFancy` geometry, in cloud space (draw with scale 12,1,12)
/// relative to the camera. `offset` is `cloudOffsetX + partialTicks`. Quads of 4 vertices.
pub fn fancy_clouds(cam_x: f64, cam_y: f64, cam_z: f64, offset: f32, color: [f32; 3]) -> Vec<super::Vertex> {
    let scale = 12.0f32;
    let thick = 4.0f32;
    let mut px = (cam_x + offset as f64 * 0.03f32 as f64) / scale as f64;
    let mut pz = cam_z / scale as f64 + 0.33f32 as f64;
    let rel_y = 128.0f32 - cam_y as f32 + 0.33f32; // b1.8.1 getCloudHeight: 128
    let wx = math::floor_double(px / 2048.0);
    let wz = math::floor_double(pz / 2048.0);
    px -= (wx * 2048) as f64;
    pz -= (wz * 2048) as f64;
    let (r, g, b) = (color[0], color[1], color[2]);
    let step = 0.00390625f32;
    let u_base = math::floor_double(px) as f32 * step;
    let v_base = math::floor_double(pz) as f32 * step;
    let fx = (px - math::floor_double(px) as f64) as f32;
    let fz = (pz - math::floor_double(pz) as f64) as f32;
    let cell = 8i32;
    let radius = 4i32; // b1.8.1 draws one more ring of cloud cells
    let eps = 1.0f32 / 1024.0f32;
    let mut out = Vec::new();
    let mut color_now = [0u8; 4];
    let c = |r: f32, g: f32, b: f32, a: f32| {
        let ch = |v: f32| ((v * 255.0f32) as i32).clamp(0, 255) as u8;
        [ch(r), ch(g), ch(b), ch(a)]
    };
    let v = |out: &mut Vec<super::Vertex>, col: [u8; 4], x: f32, y: f32, z: f32, u: f32, vv: f32| {
        out.push(super::Vertex { pos: [x, y, z], uv: [u, vv], color: col, light: 0 });
    };
    for cx in -radius + 1..=radius {
        for cz in -radius + 1..=radius {
            let x0 = (cx * cell) as f32;
            let z0 = (cz * cell) as f32;
            let ox = x0 - fx;
            let oz = z0 - fz;
            let s = cell as f32;
            let uu = |a: f32| a * step + u_base;
            let vvv = |a: f32| a * step + v_base;
            if rel_y > -thick - 1.0 {
                color_now = c(r * 0.7, g * 0.7, b * 0.7, 0.8);
                let y = rel_y + 0.0;
                v(&mut out, color_now, ox + 0.0, y, oz + s, uu(x0 + 0.0), vvv(z0 + s));
                v(&mut out, color_now, ox + s, y, oz + s, uu(x0 + s), vvv(z0 + s));
                v(&mut out, color_now, ox + s, y, oz + 0.0, uu(x0 + s), vvv(z0 + 0.0));
                v(&mut out, color_now, ox + 0.0, y, oz + 0.0, uu(x0 + 0.0), vvv(z0 + 0.0));
            }
            if rel_y <= thick + 1.0 {
                color_now = c(r, g, b, 0.8);
                let y = rel_y + thick - eps;
                v(&mut out, color_now, ox + 0.0, y, oz + s, uu(x0 + 0.0), vvv(z0 + s));
                v(&mut out, color_now, ox + s, y, oz + s, uu(x0 + s), vvv(z0 + s));
                v(&mut out, color_now, ox + s, y, oz + 0.0, uu(x0 + s), vvv(z0 + 0.0));
                v(&mut out, color_now, ox + 0.0, y, oz + 0.0, uu(x0 + 0.0), vvv(z0 + 0.0));
            }
            color_now = c(r * 0.9, g * 0.9, b * 0.9, 0.8);
            if cx > -1 {
                for i in 0..cell {
                    let fi = i as f32;
                    let xx = ox + fi + 0.0;
                    let u = uu(x0 + fi + 0.5);
                    v(&mut out, color_now, xx, rel_y + 0.0, oz + s, u, vvv(z0 + s));
                    v(&mut out, color_now, xx, rel_y + thick, oz + s, u, vvv(z0 + s));
                    v(&mut out, color_now, xx, rel_y + thick, oz + 0.0, u, vvv(z0 + 0.0));
                    v(&mut out, color_now, xx, rel_y + 0.0, oz + 0.0, u, vvv(z0 + 0.0));
                }
            }
            if cx <= 1 {
                for i in 0..cell {
                    let fi = i as f32;
                    let xx = ox + fi + 1.0 - eps;
                    let u = uu(x0 + fi + 0.5);
                    v(&mut out, color_now, xx, rel_y + 0.0, oz + s, u, vvv(z0 + s));
                    v(&mut out, color_now, xx, rel_y + thick, oz + s, u, vvv(z0 + s));
                    v(&mut out, color_now, xx, rel_y + thick, oz + 0.0, u, vvv(z0 + 0.0));
                    v(&mut out, color_now, xx, rel_y + 0.0, oz + 0.0, u, vvv(z0 + 0.0));
                }
            }
            color_now = c(r * 0.8, g * 0.8, b * 0.8, 0.8);
            if cz > -1 {
                for i in 0..cell {
                    let fi = i as f32;
                    let zz = oz + fi + 0.0;
                    let vv2 = vvv(z0 + fi + 0.5);
                    v(&mut out, color_now, ox + 0.0, rel_y + thick, zz, uu(x0 + 0.0), vv2);
                    v(&mut out, color_now, ox + s, rel_y + thick, zz, uu(x0 + s), vv2);
                    v(&mut out, color_now, ox + s, rel_y + 0.0, zz, uu(x0 + s), vv2);
                    v(&mut out, color_now, ox + 0.0, rel_y + 0.0, zz, uu(x0 + 0.0), vv2);
                }
            }
            if cz <= 1 {
                for i in 0..cell {
                    let fi = i as f32;
                    let zz = oz + fi + 1.0 - eps;
                    let vv2 = vvv(z0 + fi + 0.5);
                    v(&mut out, color_now, ox + 0.0, rel_y + thick, zz, uu(x0 + 0.0), vv2);
                    v(&mut out, color_now, ox + s, rel_y + thick, zz, uu(x0 + s), vv2);
                    v(&mut out, color_now, ox + s, rel_y + 0.0, zz, uu(x0 + s), vv2);
                    v(&mut out, color_now, ox + 0.0, rel_y + 0.0, zz, uu(x0 + 0.0), vv2);
                }
            }
        }
    }
    let _ = color_now;
    out
}

/// RenderGlobal.renderSky: convert the center only; the transparent rim keeps raw RGB.
pub fn sunrise_fan(angle: f32, anaglyph: bool) -> Option<Vec<super::Vertex>> {
    let c = sunrise_colors(angle)?;
    let rgb = if anaglyph { super::stereo_color([c[0], c[1], c[2]]) } else { [c[0], c[1], c[2]] };
    let rgba = |rgb: [f32; 3], alpha: f32| [rgb[0],rgb[1],rgb[2],alpha].map(|v| ((v*255.0f32) as i32).clamp(0,255) as u8);
    let center = super::Vertex { pos: [0.0,100.0,0.0], uv: [0.0;2], color: rgba(rgb,c[3]), light: 0 };
    let mut rim=Vec::new();
    for i in 0..=16 {
        let a=i as f32 * PI_F * 2.0 / 16.0;
        let (s,co)=(math::sin(a),math::cos(a));
        rim.push(super::Vertex { pos: [s*120.0,co*120.0,-co*40.0*c[3]],uv:[0.0;2],color:rgba([c[0],c[1],c[2]],0.0), light: 0 });
    }
    let mut fan=Vec::new();
    for i in 0..16 { fan.extend_from_slice(&[center,rim[i],rim[i+1]]); }
    // Tessellator.addVertex adds its double translation before narrowing to float.
    for v in &mut fan { v.pos = v.pos.map(|c| (c as f64 + 0.0) as f32); }
    Some(fan)
}
