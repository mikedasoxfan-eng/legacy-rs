//! Original-bytecode EffectRenderer dispatch, including virtual callback order.
use super::{
    particles::{EffectRenderer, Kind},
    weather::{Column, Snapshot},
};

pub fn check(path: &str) -> bool {
    let mut terrain = Snapshot::default();
    let mut column = Column {
        x: 0,
        z: 0,
        top: 0,
        rain: false,
        snow: false,
        temperature: 0.8,
        cache_temperature: 0.8,
        blocks: [0; 128],
        meta: [0; 128],
        brightness: [0.0; 129],
        packed_light: [0; 129],
        biome: worldgen::Biome::Plains,
        particle_rain: false,
    };
    for (id, light) in column.packed_light.iter_mut().enumerate() {
        *light = (((id & 15) * 16) << 16 | ((15 - (id & 15)) * 16)) as i32;
    }
    terrain.columns.push(column);
    fn fill(r: &mut EffectRenderer, mask: i32, count: i32) {
        for j in 0..count {
            for layer in (0..4).rev() {
                if mask & (1 << layer) == 0 {
                    continue;
                }
                let id = layer * 8 + j;
                let kind = [
                    Kind::Base,
                    Kind::Digging {
                        block: 1,
                        metadata: 0,
                    },
                    Kind::Snowball,
                    Kind::Footstep,
                ][layer as usize];
                let mut p = r.create(kind, [0.1, id as f64 + 0.2, 0.1], [0.0; 3]);
                p.texture = id;
                // Probe overrides its brightness callback; give the runtime
                // snapshot lookup the same external input without changing FX.
                p.y_offset = 0.0;
                p.bounds[4] = p.bounds[1];
                r.add(p);
            }
        }
    }
    let mut actual = Vec::new();
    fn record(
        out: &mut Vec<u8>,
        layer: usize,
        id: i32,
        partial: f32,
        origin: [f64; 3],
        rot: [f32; 5],
        light: i32,
        color: [f32; 4],
    ) {
        for v in [0, id] {
            out.extend(v.to_le_bytes());
        }
        out.extend(partial.to_bits().to_le_bytes());
        for v in [1, id, layer as i32] {
            out.extend(v.to_le_bytes());
        }
        for v in [partial].into_iter().chain(rot) {
            out.extend(v.to_bits().to_le_bytes());
        }
        for v in origin {
            out.extend(v.to_bits().to_le_bytes());
        }
        out.extend(light.to_le_bytes());
        for component in color {
            out.extend(component.to_bits().to_le_bytes());
        }
    }
    let mut cases = 0;
    for mask in [0, 1, 8, 10, 15] {
        for count in [1, 6] {
            for yaw in [0.0f32, 37.25, -280.125, 1.0e10] {
                for pitch in [0.0f32, -21.5, 90.0] {
                    for partial in [-0.25f32, 0.0, 0.375, 1.0, 1.25] {
                        let mut r = EffectRenderer::new(0);
                        fill(&mut r, mask, count);
                        r.interp_pos = [19.125, -3.25, 7.75];
                        let mut color = [0.25, 0.5, 0.75, 0.125];
                        let last = [-1e8, 85.1, -3.75];
                        let now = [0.125, 80.7, 7.1];
                        let origin = |t: f32| {
                            std::array::from_fn(|i| last[i] + (now[i] - last[i]) * t as f64)
                        };
                        r.render_lit_particles(
                            &terrain,
                            yaw,
                            pitch,
                            partial,
                            |p, cam, rot, light| {
                                record(&mut actual, 3, p.texture, partial, cam, rot, light, color);
                                if p.texture % 8 == 0 {
                                    color = [0.1, 0.2, 0.3, 0.4];
                                }
                            },
                        );
                        r.render_particles(
                            &terrain,
                            origin(partial),
                            super::particles::active_render_rotation(yaw, pitch, count == 6),
                            partial,
                            |layer, p, cam, rot, light, material| {
                                if let Some(material) = material {
                                    color = material;
                                }
                                record(
                                    &mut actual,
                                    layer,
                                    p.texture,
                                    partial,
                                    cam,
                                    rot,
                                    light,
                                    color,
                                );
                                if p.texture % 8 == 0 {
                                    color = [0.1, 0.2, 0.3, 0.4];
                                }
                            },
                        );
                        r.render_lit_particles(
                            &terrain,
                            yaw,
                            pitch,
                            partial,
                            |p, cam, rot, light| {
                                record(&mut actual, 3, p.texture, partial, cam, rot, light, color);
                                if p.texture % 8 == 0 {
                                    color = [0.1, 0.2, 0.3, 0.4];
                                }
                            },
                        );
                        r.clear();
                        r.render_lit_particles(
                            &terrain,
                            yaw,
                            pitch,
                            partial,
                            |p, cam, rot, light| {
                                record(&mut actual, 3, p.texture, partial, cam, rot, light, color);
                                if p.texture % 8 == 0 {
                                    color = [0.1, 0.2, 0.3, 0.4];
                                }
                            },
                        );
                        fill(&mut r, mask, count);
                        r.render_lit_particles(
                            &terrain,
                            yaw,
                            pitch,
                            partial,
                            |p, cam, rot, light| {
                                record(&mut actual, 3, p.texture, partial, cam, rot, light, color);
                                if p.texture % 8 == 0 {
                                    color = [0.1, 0.2, 0.3, 0.4];
                                }
                            },
                        );
                        let partial = 1.0 - partial;
                        r.render_particles(
                            &terrain,
                            origin(partial),
                            super::particles::active_render_rotation(-yaw, -pitch, count == 6),
                            partial,
                            |layer, p, cam, rot, light, material| {
                                if let Some(material) = material {
                                    color = material;
                                }
                                record(
                                    &mut actual,
                                    layer,
                                    p.texture,
                                    partial,
                                    cam,
                                    rot,
                                    light,
                                    color,
                                );
                                if p.texture % 8 == 0 {
                                    color = [0.1, 0.2, 0.3, 0.4];
                                }
                            },
                        );
                        r.render_lit_particles(
                            &terrain,
                            -yaw,
                            -pitch,
                            partial,
                            |p, cam, rot, light| {
                                record(&mut actual, 3, p.texture, partial, cam, rot, light, color);
                                if p.texture % 8 == 0 {
                                    color = [0.1, 0.2, 0.3, 0.4];
                                }
                            },
                        );
                        cases += 1;
                    }
                }
            }
        }
    }
    let expected = std::fs::read(path).unwrap();
    if actual != expected {
        let at = actual
            .iter()
            .zip(&expected)
            .position(|(a, b)| a != b)
            .unwrap_or(actual.len().min(expected.len()));
        eprintln!(
            "EffectRenderer dispatch DIFFER byte {at}, Rust {} bytes, Java {} bytes",
            actual.len(),
            expected.len()
        );
        std::fs::write(
            std::path::Path::new(path).with_extension("rust.bin"),
            actual,
        )
        .unwrap();
        return false;
    }
    println!(
        "EXACT: EffectRenderer dispatch, {cases} pass sequences, {} bytes",
        actual.len()
    );
    true
}
