//! ItemRenderer.renderItemInFirstPerson, updateEquippedItem and renderOverlays.
use super::{
    camera,
    entity::{
        fixed::Fixed,
        inventory, item_table,
        models::{Model, ModelKind},
        special,
    },
};
use worldgen::{
    entity::player::ItemStack,
    entity::Entity,
    math::{self, PI_F32},
};

pub struct Equip {
    pub item: Option<ItemStack>,
    pub progress: f32,
    pub previous: f32,
    pub slot: i32,
}
impl Default for Equip {
    fn default() -> Self {
        Self {
            item: None,
            progress: 0.0,
            previous: 0.0,
            slot: -1,
        }
    }
}
impl Equip {
    pub fn update(&mut self, item: Option<ItemStack>, slot: i32) {
        // Player snapshots store value types. A stack in the same slot retains
        // its identity while durability/count changes in place.
        let identity = slot == self.slot
            && match (self.item, item) {
                (Some(a), Some(b)) => a.id == b.id,
                (None, None) => true,
                _ => false,
            };
        self.update_identity(item, slot, identity);
    }
    pub fn update_identity(&mut self, item: Option<ItemStack>, slot: i32, identity: bool) {
        self.previous = self.progress;
        let same = match (self.item, item) {
            (None, None) => true,
            (Some(_), Some(_)) if identity => {
                self.item = item;
                slot == self.slot
            }
            (Some(a), Some(b)) if a.id == b.id && a.damage == b.damage => {
                self.item = item;
                true
            }
            _ => false,
        };
        let delta = (if same { 1.0 } else { 0.0 }) - self.progress;
        self.progress += delta.clamp(-0.4, 0.4);
        if self.progress < 0.1 {
            self.item = item;
            self.slot = slot;
        }
    }
    pub fn at(&self, t: f32) -> f32 {
        camera::lerp(self.previous, self.progress, t)
    }
}
pub fn millis() -> u64 {std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_millis() as u64}
/// The entity passed to `ItemRenderer.renderItem`: `EntityPlayer.getItemIcon` swaps the cast
/// fishing rod, the potion liquid (pass 0) and the bow pull frames; other mobs use the icon.
#[derive(Clone, Copy, Debug, Default)]
pub struct Holder {
    pub player: bool,
    pub fishing: bool,
    /// `getMaxItemUseDuration() - itemInUseCount` while `itemInUse` is set.
    pub bow_use: Option<i32>,
}
impl Holder {
    pub const MOB: Holder = Holder { player: false, fishing: false, bow_use: None };
    pub fn player(e: &Entity) -> Holder {
        let p = e.player_state();
        Holder {
            player: true,
            fishing: p.is_some_and(|p| p.fish_entity.is_some()),
            bow_use: p.and_then(|p| p.item_in_use.map(|i| worldgen::entity::player::registry::use_duration(i.id) - p.item_use_ticks)),
        }
    }
    /// `getItemIcon(stack, pass)`.
    pub fn icon(&self, item: ItemStack, pass: i32) -> i32 {
        let base = item_table::properties(item.id as i16, item.damage as i16).0;
        if !self.player {
            return base;
        }
        if item.id == 346 && self.fishing {
            return base + 16;
        }
        if item.id == 373 {
            return if pass == 1 { base } else { 141 };
        }
        if let (Some(d), 261) = (self.bow_use, item.id) {
            if d >= 18 {
                return 133;
            }
            if d > 13 {
                return 117;
            }
            if d > 0 {
                return 101;
            }
        }
        base
    }
}
pub fn item(g: &mut Fixed, item: ItemStack, holder: Holder) {
    item_pass(g, item, holder, 0, current_glint_times());
}
thread_local! {
    /// Checks replay the original's `System.currentTimeMillis` glint offsets.
    static GLINT_OVERRIDE: std::cell::Cell<Option<[f32; 2]>> = const { std::cell::Cell::new(None) };
}
pub fn override_glint_times(times: Option<[f32; 2]>) {
    GLINT_OVERRIDE.with(|c| c.set(times));
}
/// The glint offsets for this frame (or the replayed ones in a check).
pub fn current_glint_times() -> [f32; 2] {
    GLINT_OVERRIDE.with(|c| c.get()).unwrap_or_else(|| glint_times(millis()))
}
/// The two glint texture offsets: `currentTimeMillis % 3000 / 3000 * 8` and `% 4873 / 4873 * 8`.
pub fn glint_times(time: u64) -> [f32; 2] {
    [(time % 3000) as f32 / 3000.0 * 8.0, (time % 4873) as f32 / 4873.0 * 8.0]
}
/// `ItemRenderer.renderItem(entity, stack, pass)` (1.0).
pub fn item_pass(g: &mut Fixed, item: ItemStack, holder: Holder, pass: i32, times: [f32; 2]) {
    if inventory::in_3d(item.id as i16) {
        g.push();
        g.texture = 1;
        inventory::inventory(g, item.id as u8, item.damage, 1.0, true);
        g.pop();
    } else {
        let icon = holder.icon(item, pass);
        let start = g.draws.len();
        special::sprite(g, icon, item.id < 256);
        if pass == 0 && worldgen::item10::glint(item) {
            // func_40686_a runs again inside the sprite's transform, still with RESCALE_NORMAL.
            g.push();
            g.ops = g.draws[start].ops.clone();
            g.enable(2, true);
            g.depth = 514;
            g.enable(1, false);
            g.texture = 76;
            g.enable(16, true);
            g.src = 768;
            g.dst = 1;
            g.material = [0.5 * 0.76, 0.25 * 0.76, 0.8 * 0.76, 1.0];
            for i in 0..2 {
                use super::entity::fixed::Op;
                g.texture_ops = vec![
                    Op { code: 1, args: [0.125, 0.125, 0.125, 0.0] },
                    Op { code: 0, args: [if i == 0 { times[i] } else { -times[i] }, 0.0, 0.0, 0.0] },
                    Op { code: 2, args: [if i == 0 { -50.0 } else { 10.0 }, 0.0, 0.0, 1.0] },
                ];
                special::sprite_geometry(g, [0.0, 0.0, 1.0, 1.0]);
            }
            g.texture_ops.clear();
            g.enable(16, false);
            g.enable(1, true);
            g.depth = 515;
            g.enable(2, false);
            g.pop();
        }
    }
}
pub fn hand(g: &mut Fixed) {
    let mut m = Model::new(ModelKind::Biped);
    m.on_ground = 0.0;
    m.set_rotation_angles([0.0, 0.0, 0.0, 0.0, 0.0, 0.0625]);
    special::part(g, &m["bipedRightArm"], false);
}
pub fn render(
    e: &Entity,
    t: f32,
    equip: &Equip,
    brightness: f32,
    block_brightness: f32,
    s: camera::Settings,
    hide: bool,
) -> Fixed {
    render_with_map(e, t, equip, brightness, block_brightness, s, hide, None)
}
pub fn render_with_map(
    e: &Entity,
    t: f32,
    equip: &Equip,
    brightness: f32,
    block_brightness: f32,
    s: camera::Settings,
    hide: bool,
    map: Option<&super::map::Data>,
) -> Fixed {
    // b1.8.1 ItemRenderer: getLightBrightness is read but overwritten with 1; the light comes
    // from the lightmap (func_35451_b at the player), and items render at brightness 1.
    let _ = (brightness, block_brightness);
    let brightness = 1.0f32;
    let mut g = Fixed::new(brightness);
    if s.anaglyph {
        g.translate((s.eye * 2 - 1) as f32 * 0.1, 0.0, 0.0);
    }
    if s.third_person || hide || e.player_state().is_some_and(|p| p.sleeping) {
        return g;
    }
    camera::hurt(&mut g, e, t);
    if s.bobbing {
        camera::bob(&mut g, e, t);
    }
    let progress = equip.at(t);
    let swing = e.swing_progress(t);
    let a = math::sin(swing * PI_F32);
    let b = math::sin((swing as f64).sqrt() as f32 * PI_F32);
    let c = math::sin((swing as f64).sqrt() as f32 * PI_F32 * 2.0);
    let sq = math::sin(swing * swing * PI_F32);
    let pitch = camera::lerp(e.prev_pitch, e.pitch, t);
    let yaw = camera::lerp(e.prev_yaw, e.yaw, t);
    let mut light = g.ops.clone();
    light.push(super::entity::fixed::Op {
        code: 2,
        args: [pitch, 1.0, 0.0, 0.0],
    });
    light.push(super::entity::fixed::Op {
        code: 2,
        args: [yaw, 0.0, 1.0, 0.0],
    });
    g.light_transform = Some(camera::matrix(&light));
    // EntityPlayerSP arm sway (field_35225_ar..field_35222_e), against the un-interpolated view.
    if let Some(p) = e.player_state() {
        let arm_pitch = p.arm_pitch[0] + (p.arm_pitch[1] - p.arm_pitch[0]) * t;
        let arm_yaw = p.arm_yaw[0] + (p.arm_yaw[1] - p.arm_yaw[0]) * t;
        g.rotate((e.pitch - arm_pitch) * 0.1, 1.0, 0.0, 0.0);
        g.rotate((e.yaw - arm_yaw) * 0.1, 0.0, 1.0, 0.0);
    }
    // Item in use: (ticks left, max duration, action) as func_35205_Y / func_35866_m / func_35865_n.
    let in_use = e.player_state().and_then(|p| {
        let i = p.item_in_use?;
        (p.item_use_ticks > 0).then(|| {
            let reg = worldgen::entity::player::registry::use_action(i.id as i32);
            (p.item_use_ticks, worldgen::entity::player::registry::use_duration(i.id as i32), reg)
        })
    });
    if let Some(held) = equip.item {
        let color = item_table::properties(held.id as i16, held.damage as i16).1;
        g.material = [
            brightness * ((color >> 16 & 255) as f32 / 255.0),
            brightness * ((color >> 8 & 255) as f32 / 255.0),
            brightness * ((color & 255) as f32 / 255.0),
            1.0,
        ];
        if held.id == 358 {
            g.push();
            g.translate(-b * 0.4, c * 0.2, -a * 0.2);
            let f = (1.0 - pitch / 45.0 + 0.1).clamp(0.0, 1.0);
            let f = -math::cos(f * PI_F32) * 0.5 + 0.5;
            g.translate(
                0.0,
                0.0 * 0.8 - (1.0 - progress) * 1.2 - f * 0.5 + 0.04,
                -0.9 * 0.8,
            );
            g.rotate(90.0, 0.0, 1.0, 0.0);
            g.rotate(f * -85.0, 0.0, 0.0, 1.0);
            g.enable(2, true);
            g.texture = 9;
            for i in 0..2 {
                let sign = (i * 2 - 1) as f32;
                g.push();
                g.translate(-0.0, -0.6, 1.1 * sign);
                g.rotate(-45.0 * sign, 1.0, 0.0, 0.0);
                g.rotate(-90.0, 0.0, 0.0, 1.0);
                g.rotate(59.0, 0.0, 0.0, 1.0);
                g.rotate(-65.0 * sign, 0.0, 1.0, 0.0);
                g.scale(1.0, 1.0, 1.0);
                hand(&mut g);
                g.pop();
            }
            g.rotate(-sq * 20.0, 0.0, 1.0, 0.0);
            g.rotate(-b * 20.0, 0.0, 0.0, 1.0);
            g.rotate(-b * 80.0, 1.0, 0.0, 0.0);
            g.scale(0.38, 0.38, 0.38);
            g.rotate(90.0, 0.0, 1.0, 0.0);
            g.rotate(180.0, 0.0, 0.0, 1.0);
            g.translate(-1.0, -1.0, 0.0);
            g.scale(0.015625, 0.015625, 0.015625);
            g.texture = 11;
            g.normal = [0.0, 0.0, -1.0];
            g.begin();
            g.quad(
                [
                    [-7.0, 135.0, 0.0],
                    [135.0, 135.0, 0.0],
                    [135.0, -7.0, 0.0],
                    [-7.0, -7.0, 0.0],
                ],
                [[0.0, 1.0], [1.0, 1.0], [1.0, 0.0], [0.0, 0.0]],
            );
            g.end();
            let default = super::map::Data {
                label: format!("map_{}", held.damage),
                ..Default::default()
            };
            super::map::render(&mut g, map.unwrap_or(&default));
            g.pop();
        } else {
            use worldgen::entity::player::registry::ItemUseAction;
            g.push();
            if let Some((ticks, max, action)) = in_use {
                if matches!(action, ItemUseAction::Eat | ItemUseAction::Drink) {
                    let f18 = ticks as f32 - t + 1.0f32;
                    let f10 = 1.0f32 - f18 / max as f32;
                    let mut f12 = 1.0f32 - f10;
                    f12 = f12 * f12 * f12;
                    f12 = f12 * f12 * f12;
                    f12 = f12 * f12 * f12;
                    let f13 = 1.0f32 - f12;
                    let bob = (math::cos(f18 / 4.0f32 * PI_F32) * 0.1f32).abs();
                    g.translate(0.0, bob * if f10 as f64 > 0.2 { 1.0 } else { 0.0 }, 0.0);
                    g.translate(f13 * 0.6f32, -f13 * 0.5f32, 0.0);
                    g.rotate(f13 * 90.0f32, 0.0, 1.0, 0.0);
                    g.rotate(f13 * 10.0f32, 1.0, 0.0, 0.0);
                    g.rotate(f13 * 30.0f32, 0.0, 0.0, 1.0);
                }
            } else {
                g.translate(-b * 0.4, c * 0.2, -a * 0.2);
            }
            g.translate(0.7 * 0.8, -0.65 * 0.8 - (1.0 - progress) * 0.6, -0.9 * 0.8);
            g.rotate(45.0, 0.0, 1.0, 0.0);
            g.enable(2, true);
            g.rotate(-sq * 20.0, 0.0, 1.0, 0.0);
            g.rotate(-b * 20.0, 0.0, 0.0, 1.0);
            g.rotate(-b * 80.0, 1.0, 0.0, 0.0);
            g.scale(0.4, 0.4, 0.4);
            if let Some((ticks, max, action)) = in_use {
                if action == ItemUseAction::Block {
                    g.translate(-0.5, 0.2, 0.0);
                    g.rotate(30.0, 0.0, 1.0, 0.0);
                    g.rotate(-80.0, 1.0, 0.0, 0.0);
                    g.rotate(60.0, 0.0, 1.0, 0.0);
                } else if action == ItemUseAction::Bow {
                    g.rotate(-18.0, 0.0, 0.0, 1.0);
                    g.rotate(-12.0, 0.0, 1.0, 0.0);
                    g.rotate(-8.0, 1.0, 0.0, 0.0);
                    g.translate(-0.9, 0.2, 0.0);
                    let f10 = max as f32 - (ticks as f32 - t + 1.0f32);
                    let mut f11 = f10 / 20.0f32;
                    f11 = (f11 * f11 + f11 * 2.0f32) / 3.0f32;
                    if f11 > 1.0 {
                        f11 = 1.0;
                    }
                    if f11 > 0.1 {
                        g.translate(0.0, math::sin((f10 - 0.1f32) * 1.3f32) * 0.01f32 * (f11 - 0.1f32), 0.0);
                    }
                    g.translate(0.0, 0.0, f11 * 0.1f32);
                    g.rotate(-335.0, 0.0, 0.0, 1.0);
                    g.rotate(-50.0, 0.0, 1.0, 0.0);
                    g.translate(0.0, 0.5, 0.0);
                    g.scale(1.0, 1.0, 1.0f32 + f11 * 0.2f32);
                    g.translate(0.0, -0.5, 0.0);
                    g.rotate(50.0, 0.0, 1.0, 0.0);
                    g.rotate(335.0, 0.0, 0.0, 1.0);
                }
            }
            if held.id == 346 {
                g.rotate(180.0, 0.0, 1.0, 0.0);
            }
            let holder = Holder::player(e);
            let times = current_glint_times();
            item_pass(&mut g, held, holder, 0, times);
            if held.id == 373 {
                g.material = [brightness, brightness, brightness, 1.0];
                item_pass(&mut g, held, holder, 1, times);
            }
            g.pop();
        }
    } else {
        g.push();
        g.translate(-b * 0.3, c * 0.4, -a * 0.4);
        g.translate(0.8 * 0.8, -0.75 * 0.8 - (1.0 - progress) * 0.6, -0.9 * 0.8);
        g.rotate(45.0, 0.0, 1.0, 0.0);
        g.enable(2, true);
        g.rotate(b * 70.0, 0.0, 1.0, 0.0);
        g.rotate(-sq * 20.0, 0.0, 0.0, 1.0);
        g.texture = 9;
        g.translate(-1.0, 3.6, 3.5);
        g.rotate(120.0, 0.0, 0.0, 1.0);
        g.rotate(200.0, 1.0, 0.0, 0.0);
        g.rotate(-135.0, 0.0, 1.0, 0.0);
        g.scale(1.0, 1.0, 1.0);
        g.translate(5.6, 0.0, 0.0);
        g.scale(1.0, 1.0, 1.0);
        hand(&mut g);
        g.pop();
    }
    g.enable(2, false);
    g.enable(1, false);
    g
}
pub fn inside(g: &mut Fixed, texture: i32) {
    g.material = [0.1, 0.1, 0.1, 0.5];
    g.texture = 1;
    g.push();
    // Vanilla uses texture % 16 (no *16) here. Keep the bug.
    let u0 = (texture % 16) as f32 / 256.0 - 0.0078125;
    let u1 = ((texture % 16) as f32 + 15.99) / 256.0 + 0.0078125;
    let v0 = (texture / 16) as f32 / 256.0 - 0.0078125;
    let v1 = ((texture / 16) as f32 + 15.99) / 256.0 + 0.0078125;
    g.begin();
    g.quad(
        [
            [-1.0, -1.0, -0.5],
            [1.0, -1.0, -0.5],
            [1.0, 1.0, -0.5],
            [-1.0, 1.0, -0.5],
        ],
        [[u1, v1], [u0, v1], [u0, v0], [u1, v0]],
    );
    g.end();
    g.pop();
    g.material = [1.0; 4];
}
pub fn overlays(
    e: &Entity,
    water: bool,
    opaque: &[i32],
    brightness: f32,
    s: camera::Settings,
) -> Fixed {
    let mut g = Fixed::new(1.0);
    g.enable(1, false);
    if s.anaglyph {
        g.translate((s.eye * 2 - 1) as f32 * 0.1, 0.0, 0.0);
    }
    if s.third_person || e.player_state().is_some_and(|p| p.sleeping) {
        return g;
    }
    if e.fire > 0 {
        g.material = [1.0, 1.0, 1.0, 0.9];
        g.enable(16, true);
        g.src = 770;
        g.dst = 771;
        g.texture = 1;
        for i in 0..2 {
            g.push();
            let texture = 31 + i * 16;
            let x = ((texture & 15) << 4) as f32;
            let y = (texture & 240) as f32;
            let (u0, u1, v0, v1) = (
                x / 256.0,
                (x + 15.99) / 256.0,
                y / 256.0,
                (y + 15.99) / 256.0,
            );
            g.translate(-(i * 2 - 1) as f32 * 0.24, -0.3, 0.0);
            g.rotate((i * 2 - 1) as f32 * 10.0, 0.0, 1.0, 0.0);
            g.begin();
            g.quad(
                [
                    [-0.5, -0.5, -0.5],
                    [0.5, -0.5, -0.5],
                    [0.5, 0.5, -0.5],
                    [-0.5, 0.5, -0.5],
                ],
                [[u1, v1], [u0, v1], [u0, v0], [u1, v0]],
            );
            g.end();
            g.pop();
        }
        g.material = [1.0; 4];
        g.enable(16, false);
    }
    for &tex in opaque {
        inside(&mut g, tex);
    }
    if water {
        g.texture = 10;
        g.material = [brightness, brightness, brightness, 0.5];
        g.enable(16, true);
        g.src = 770;
        g.dst = 771;
        g.push();
        let u = -e.yaw / 64.0;
        let v = e.pitch / 64.0;
        g.begin();
        g.quad(
            [
                [-1.0, -1.0, -0.5],
                [1.0, -1.0, -0.5],
                [1.0, 1.0, -0.5],
                [-1.0, 1.0, -0.5],
            ],
            [
                [4.0 + u, 4.0 + v],
                [0.0 + u, 4.0 + v],
                [0.0 + u, 0.0 + v],
                [4.0 + u, 0.0 + v],
            ],
        );
        g.end();
        g.pop();
        g.material = [1.0; 4];
        g.enable(16, false);
    }
    g
}
pub fn opaque_textures(e: &Entity, w: &mut worldgen::World) -> Vec<i32> {
    if e.player_state().is_some_and(|p| p.sleeping) {
        return vec![];
    }
    let mut inside = false;
    for i in 0..8 {
        // 1.0 Entity.isEntityInsideOpaqueBlock narrowed the probe from 0.9 to 0.8 widths.
        let x = ((i % 2) as f32 - 0.5) * e.width * 0.8;
        let y = ((i / 2 % 2) as f32 - 0.5) * 0.1;
        let z = ((i / 4 % 2) as f32 - 0.5) * e.width * 0.8;
        if w.is_block_normal_cube(
            math::floor_double(e.pos.x + x as f64),
            math::floor_double(e.pos.y + e.eye_height() as f64 + y as f64),
            math::floor_double(e.pos.z + z as f64),
        ) {
            inside = true;
            break;
        }
    }
    if !inside {
        return vec![];
    }
    let (x, y, z) = (
        math::floor_double(e.pos.x),
        math::floor_double(e.pos.y),
        math::floor_double(e.pos.z),
    );
    let mut id = w.get_block_id(x, y, z);
    let mut textures = vec![];
    if w.is_block_normal_cube(x, y, z) {
        textures.push(super::rprops(id).textures[0][2] as i32);
    } else {
        for i in 0..8 {
            let dx = ((i % 2) as f32 - 0.5) * e.width * 0.9;
            let dy = ((i / 2 % 2) as f32 - 0.5) * e.height * 0.2;
            let dz = ((i / 4 % 2) as f32 - 0.5) * e.width * 0.9;
            let (bx, by, bz) = (
                math::floor_float(x as f32 + dx),
                math::floor_float(y as f32 + dy),
                math::floor_float(z as f32 + dz),
            );
            if w.is_block_normal_cube(bx, by, bz) {
                id = w.get_block_id(bx, by, bz);
            }
        }
    }
    if id!=0 && super::rprops(id).exists {
        textures.push(super::rprops(id).textures[0][2] as i32);
    }
    textures
}
