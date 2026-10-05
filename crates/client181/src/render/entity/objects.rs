//! Literal RenderItem, RenderArrow, RenderFireball and RenderTNTPrimed.
use super::{fixed::Fixed, inventory};
use worldgen::world::tile::ItemStack;
use worldgen::{math, JavaRandom};
#[derive(Clone, Debug)]
pub enum ObjectKind {
    XPOrb {
        value: i32,
        ticks: i32,
    },
    Vehicle(super::vehicles::VehicleState),
    Item {
        item: ItemStack,
        age: i32,
        hover: f32,
    },
    Falling {
        block: u8,
        light: [f32; 7],
    },
    Tnt {
        fuse: i32,
    },
    Arrow {
        yaw: f32,
        pitch: f32,
        shake: i32,
        visible: bool,
    },
    Sprite {
        icon: i32,
        scale: f32,
    },
}
#[derive(Clone, Debug)]
pub struct ObjectState {
    pub kind: ObjectKind,
    pub position: [f64; 3],
    pub partial: f32,
    pub brightness: f32,
}
pub fn render(s: &ObjectState, camera: [f64; 3], view: [f32; 2]) -> Fixed {
    let mut g = Fixed::new(s.brightness);
    let [x, y, z] = std::array::from_fn(|i| (s.position[i] - camera[i]) as f32);
    let partial = s.partial;
    match s.kind {
        ObjectKind::XPOrb { value, ticks } => {
            g.push();
            g.translate(x, y, z);
            g.texture = 30;
            let index = worldgen::entity::xp::texture_index(value);
            let u0 = (index % 4 * 16) as f32 / 64.0;
            let u1 = (index % 4 * 16 + 16) as f32 / 64.0;
            let v0 = (index / 4 * 16) as f32 / 64.0;
            let v1 = (index / 4 * 16 + 16) as f32 / 64.0;
            let bright = s.brightness * 255.0;
            let phase = (ticks as f32 + partial) / 2.0;
            let r = ((math::sin(phase) + 1.0) * 0.5 * bright) as i32;
            let green = bright as i32;
            let b = ((math::sin(phase + math::PI_F32 * 4.0 / 3.0) + 1.0) * 0.1 * bright) as i32;
            g.rotate(180.0 - view[0], 0.0, 1.0, 0.0);
            g.rotate(-view[1], 1.0, 0.0, 0.0);
            g.scale(0.3, 0.3, 0.3);
            g.begin();
            g.vertex_color = Some(
                (r as u8 as u32)
                    | ((green as u8 as u32) << 8)
                    | ((b as u8 as u32) << 16)
                    | (128 << 24),
            );
            g.set_normal(0.0, 1.0, 0.0);
            g.quad(
                [
                    [-0.5, -0.25, 0.0],
                    [0.5, -0.25, 0.0],
                    [0.5, 0.75, 0.0],
                    [-0.5, 0.75, 0.0],
                ],
                [[u0, v1], [u1, v1], [u1, v0], [u0, v0]],
            );
            g.end();
            g.enable(16, false);
            g.enable(2, false);
            g.pop();
        }

        ObjectKind::Vehicle(v) => super::vehicles::render(
            &mut g,
            v,
            std::array::from_fn(|i| s.position[i] - camera[i]),
            partial,
            s.brightness,
        ),
        ObjectKind::Item { item, age, hover } => {
            let mut rng = JavaRandom::new(187);
            g.push();
            let bob = math::sin((age as f32 + partial) / 10.0 + hover) * 0.1 + 0.1;
            let angle = ((age as f32 + partial) / 20.0 + hover) * (180.0 / math::PI_F32);
            let copies = if item.count > 20 {
                4
            } else if item.count > 5 {
                3
            } else if item.count > 1 {
                2
            } else {
                1
            };
            g.translate(x, y + bob, z);
            g.enable(2, true);
            if inventory::in_3d(item.id) {
                g.rotate(angle, 0.0, 1.0, 0.0);
                g.texture = 1;
                let id = item.id as u8;
                let scale = if !crate::render::rprops(id).normal
                    && id != 44
                    && crate::render::rprops(id).render_type != 16
                {
                    0.5
                } else {
                    0.25
                };
                g.scale(scale, scale, scale);
                for i in 0..copies {
                    g.push();
                    if i > 0 {
                        g.translate(
                            (rng.next_float() * 2.0 - 1.0) * 0.2 / scale,
                            (rng.next_float() * 2.0 - 1.0) * 0.2 / scale,
                            (rng.next_float() * 2.0 - 1.0) * 0.2 / scale,
                        );
                    }
                    inventory::inventory(&mut g, id, item.damage as i32, s.brightness, true);
                    g.pop();
                }
            } else {
                g.scale(0.5, 0.5, 0.5);
                g.texture = if item.id < 256 { 1 } else { 2 };
                let (icon, color, _) = super::item_table::properties(item.id, item.damage);
                g.material = [
                    (color >> 16 & 255) as f32 / 255.0 * s.brightness,
                    (color >> 8 & 255) as f32 / 255.0 * s.brightness,
                    (color & 255) as f32 / 255.0 * s.brightness,
                    1.0,
                ];
                for i in 0..copies {
                    g.push();
                    if i > 0 {
                        g.translate(
                            (rng.next_float() * 2.0 - 1.0) * 0.3,
                            (rng.next_float() * 2.0 - 1.0) * 0.3,
                            (rng.next_float() * 2.0 - 1.0) * 0.3,
                        );
                    }
                    g.rotate(180.0 - view[0], 0.0, 1.0, 0.0);
                    sprite_quad(&mut g, icon);
                    g.pop();
                }
            }
            g.enable(2, false);
            g.pop();
        }
        ObjectKind::Falling { block, light } => {
            g.push();
            g.translate(x, y, z);
            g.texture = 1;
            g.enable(1, false);
            inventory::falling(&mut g, block, light);
            g.enable(1, true);
            g.pop();
        }
        ObjectKind::Tnt { fuse } => {
            g.push();
            g.translate(x, y, z);
            if fuse as f32 - partial + 1.0 < 10.0 {
                let mut f = (1.0 - (fuse as f32 - partial + 1.0) / 10.0).clamp(0.0, 1.0);
                f *= f;
                f *= f;
                let scale = 1.0 + f * 0.3;
                g.scale(scale, scale, scale);
            }
            let alpha = (1.0 - (fuse as f32 - partial + 1.0) / 100.0) * 0.8;
            g.texture = 1;
            inventory::inventory(&mut g, 46, 0, s.brightness, true);
            if fuse / 5 % 2 == 0 {
                g.enable(8, false);
                g.enable(1, false);
                g.enable(16, true);
                g.src = 770;
                g.dst = 772;
                g.material = [1.0, 1.0, 1.0, alpha];
                // Vanilla renderBlockOnInventory overwrites this alpha with 1.0.
                inventory::inventory(&mut g, 46, 0, 1.0, true);
                g.material = [1.0; 4];
                g.enable(16, false);
                g.enable(1, true);
                g.enable(8, true);
            }
            g.pop();
        }
        ObjectKind::Sprite { icon, scale } => {
            g.push();
            g.translate(x, y, z);
            g.enable(2, true);
            g.scale(scale, scale, scale);
            g.texture = 2;
            g.rotate(180.0 - view[0], 0.0, 1.0, 0.0);
            g.rotate(-view[1], 1.0, 0.0, 0.0);
            sprite_quad(&mut g, icon);
            g.enable(2, false);
            g.pop();
        }
        ObjectKind::Arrow {
            yaw,
            pitch,
            shake,
            visible: _,
        } => {
            { // Beta 1.8.1 renders arrows even before their first nonzero rotation.
                g.texture = 3;
                g.push();
                g.translate(x, y, z);
                g.rotate(yaw - 90.0, 0.0, 1.0, 0.0);
                g.rotate(pitch, 0.0, 0.0, 1.0);
                g.enable(2, true);
                let f = shake as f32 - partial;
                if f > 0.0 {
                    g.rotate(-math::sin(f * 3.0) * f, 0.0, 0.0, 1.0);
                }
                g.rotate(45.0, 1.0, 0.0, 0.0);
                g.scale(0.05625, 0.05625, 0.05625);
                g.translate(-4.0, 0.0, 0.0);
                g.normal = [0.05625, 0.0, 0.0];
                g.begin();
                g.quad(
                    [
                        [-7.0, -2.0, -2.0],
                        [-7.0, -2.0, 2.0],
                        [-7.0, 2.0, 2.0],
                        [-7.0, 2.0, -2.0],
                    ],
                    [
                        [0.0, 0.15625],
                        [0.15625, 0.15625],
                        [0.15625, 0.3125],
                        [0.0, 0.3125],
                    ],
                );
                g.end();
                g.normal = [-0.05625, 0.0, 0.0];
                g.begin();
                g.quad(
                    [
                        [-7.0, 2.0, -2.0],
                        [-7.0, 2.0, 2.0],
                        [-7.0, -2.0, 2.0],
                        [-7.0, -2.0, -2.0],
                    ],
                    [
                        [0.0, 0.15625],
                        [0.15625, 0.15625],
                        [0.15625, 0.3125],
                        [0.0, 0.3125],
                    ],
                );
                g.end();
                for _ in 0..4 {
                    g.rotate(90.0, 1.0, 0.0, 0.0);
                    g.normal = [0.0, 0.0, 0.05625];
                    g.begin();
                    g.quad(
                        [
                            [-8.0, -2.0, 0.0],
                            [8.0, -2.0, 0.0],
                            [8.0, 2.0, 0.0],
                            [-8.0, 2.0, 0.0],
                        ],
                        [[0.0, 0.0], [0.5, 0.0], [0.5, 0.15625], [0.0, 0.15625]],
                    );
                    g.end();
                }
                g.enable(2, false);
                g.pop();
            }
        }
    }
    g
}
fn sprite_quad(g: &mut Fixed, icon: i32) {
    let u0 = (icon % 16 * 16) as f32 / 256.0;
    let u1 = (icon % 16 * 16 + 16) as f32 / 256.0;
    let v0 = (icon / 16 * 16) as f32 / 256.0;
    let v1 = (icon / 16 * 16 + 16) as f32 / 256.0;
    g.begin();
    g.set_normal(0.0, 1.0, 0.0);
    g.quad(
        [
            [-0.5, -0.25, 0.0],
            [0.5, -0.25, 0.0],
            [0.5, 0.75, 0.0],
            [-0.5, 0.75, 0.0],
        ],
        [[u0, v1], [u1, v1], [u1, v0], [u0, v0]],
    );
    g.end();
}
