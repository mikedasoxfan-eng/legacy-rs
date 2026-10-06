//! RenderPlayer's SP model, armor passes, sleeping transform and held item.
use super::{
    fixed::Fixed,
    inventory,
    models::{Model, ModelKind},
    special,
};
use crate::render::{camera::lerp, first_person};
use worldgen::entity::Entity;
pub const ARMOR: [&str; 10] = [
    "armor/cloth_1.png",
    "armor/cloth_2.png",
    "armor/chain_1.png",
    "armor/chain_2.png",
    "armor/iron_1.png",
    "armor/iron_2.png",
    "armor/diamond_1.png",
    "armor/diamond_2.png",
    "armor/gold_1.png",
    "armor/gold_2.png",
];
pub struct Renderer {
    main: Model,
    chest: Model,
    armor: Model,
    pass: Option<bool>,
}
impl Default for Renderer {
    fn default() -> Self {
        Self {
            main: Model::new(ModelKind::Biped),
            chest: Model::with_options(ModelKind::Biped, 1.0, 0.0, 12),
            armor: Model::with_options(ModelKind::Biped, 0.5, 0.0, 12),
            pass: None,
        }
    }
}
impl Renderer {
    /// `setArmorModel`: -1 without armor, 1 for armor, 15 for enchanted armor (the glint pass).
    fn armor_pass(&mut self, g: &mut Fixed, e: &Entity, pass: usize, pose: [f32; 6]) -> i32 {
        let Some(item) = e.player_state().and_then(|p| p.inventory.armor[3 - pass]) else {
            return -1;
        };
        if !(298..=317).contains(&item.id) {
            return -1;
        }
        let material = (item.id - 298) / 4;
        g.texture = (20 + material * 2 + if pass == 2 { 1 } else { 0 }) as u32;
        let m = if pass == 2 {
            &mut self.armor
        } else {
            &mut self.chest
        };
        m["bipedHead"].show_model = pass == 0;
        m["bipedHeadwear"].show_model = pass == 0;
        m["bipedBody"].show_model = pass == 1 || pass == 2;
        m["bipedRightArm"].show_model = pass == 1;
        m["bipedLeftArm"].show_model = pass == 1;
        m["bipedRightLeg"].show_model = pass == 2 || pass == 3;
        m["bipedLeftLeg"].show_model = pass == 2 || pass == 3;
        self.pass = Some(pass == 2);
        m.set_rotation_angles(pose);
        special::model(g, m);
        if item.is_enchanted() {
            15
        } else {
            1
        }
    }
    /// RenderLiving's pass-15 enchantment glint over the armor model just drawn.
    fn armor_glint(&mut self, g: &mut Fixed, age: f32) {
        use super::fixed::Op;
        let m = if self.pass == Some(true) { &self.armor } else { &self.chest };
        g.texture = 76;
        g.enable(16, true);
        g.material = [0.5, 0.5, 0.5, 1.0];
        g.depth = 514;
        g.enable(32, false);
        for i in 0..2 {
            g.enable(1, false);
            g.material = [0.5 * 0.76, 0.25 * 0.76, 0.8 * 0.76, 1.0];
            g.src = 768;
            g.dst = 1;
            let offset = age * (0.001 + i as f32 * 0.003) * 20.0;
            let third = 1.0f32 / 3.0;
            g.texture_ops = vec![
                Op { code: 1, args: [third, third, third, 0.0] },
                Op { code: 2, args: [30.0 - i as f32 * 60.0, 0.0, 0.0, 1.0] },
                Op { code: 0, args: [0.0, offset, 0.0, 0.0] },
            ];
            special::model(g, m);
        }
        g.material = [1.0; 4];
        g.enable(32, true);
        g.texture_ops.clear();
        g.enable(1, true);
        g.enable(16, false);
        g.depth = 515;
    }
    pub fn render(
        &mut self,
        e: &Entity,
        t: f32,
        camera: [f64; 3],
        brightness: f32,
        bed: i32,
    ) -> Fixed {
        self.render_lit(e, t, camera, brightness, brightness, bed, true)
    }
    /// RenderManager's white primary color and RenderLiving's scalar hurt tint
    /// are distinct inputs in b1.8.1.
    pub fn render_lit(
        &mut self, e: &Entity, t: f32, camera: [f64; 3], brightness: f32,
        overlay_brightness: f32, bed: i32, local_player: bool,
    ) -> Fixed {
        let mut g = Fixed::new(brightness);
        let p = e.player_state().expect("player");
        let held = p.inventory.held();
        let sneaking = e.sneaking && (!local_player || !p.sleeping);
        // 1.0 renderPlayer: held pose 1, 3 while blocking; aimed bow while drawing.
        use worldgen::entity::player::registry::{use_action, ItemUseAction};
        let action = held.filter(|_| p.item_use_ticks > 0).map(|h| use_action(h.id));
        let held_pose = match (held, action) {
            (None, _) => 0,
            (Some(_), Some(ItemUseAction::Block)) => 3,
            _ => 1,
        };
        let aimed_bow = action == Some(ItemUseAction::Bow);
        for m in [&mut self.main, &mut self.chest, &mut self.armor] {
            m.held_right = false;
            m.held_right_pose = held_pose;
            m.aimed_bow = aimed_bow;
            m.is_sneak = sneaking;
        }
        self.main.on_ground = e.swing_progress(t);
        self.main.is_riding = e.riding.is_some();
        if let Some(armor) = self.pass {
            let m = if armor {
                &mut self.armor
            } else {
                &mut self.chest
            };
            m.on_ground = self.main.on_ground;
            m.is_riding = self.main.is_riding;
        }
        let l = &e.living;
        let body = lerp(l.prev_render_yaw_offset, l.render_yaw_offset, t);
        let yaw = lerp(e.prev_yaw, e.yaw, t);
        let pitch = lerp(e.prev_pitch, e.pitch, t);
        let pose = [
            l.limb_swing - l.limb_amount * (1.0 - t),
            lerp(l.prev_limb_amount, l.limb_amount, t).min(1.0),
            e.ticks_existed as f32 + t,
            yaw - body,
            pitch,
            0.0625,
        ];
        self.main.set_rotation_angles(pose);
        let pos = [
            e.last_tick_pos.x + (e.pos.x - e.last_tick_pos.x) * t as f64,
            e.last_tick_pos.y + (e.pos.y - e.last_tick_pos.y) * t as f64,
            e.last_tick_pos.z + (e.pos.z - e.last_tick_pos.z) * t as f64,
        ];
        let sleeping = l.health > 0 && p.sleeping;
        let off = if sleeping {
            p.bed_offset
        } else {
            Default::default()
        };
        g.push();
        g.translate(
            (pos[0] - camera[0] + off.x) as f32,
            (pos[1] - camera[1] - e.y_offset as f64 + off.y - if sneaking && !local_player {0.125} else {0.0}) as f32,
            (pos[2] - camera[2] + off.z) as f32,
        );
        if sleeping {
            g.rotate(if bed < 0 {0.0} else {[90.0, 0.0, 270.0, 180.0][bed as usize & 3]}, 0.0, 1.0, 0.0);
            g.rotate(90.0, 0.0, 0.0, 1.0);
            g.rotate(270.0, 0.0, 1.0, 0.0);
        } else {
            g.rotate(180.0 - body, 0.0, 1.0, 0.0);
            if l.death_time > 0 {
                let f = ((l.death_time as f32 + t - 1.0) / 20.0 * 1.6) as f64;
                g.rotate((f.sqrt() as f32).min(1.0) * 90.0, 0.0, 0.0, 1.0);
            }
        }
        g.enable(2, true);
        g.scale(-1.0, -1.0, 1.0);
        g.scale(0.9375, 0.9375, 0.9375);
        g.translate(0.0, -24.0 * 0.0625 - 0.0078125, 0.0);
        g.texture = 9;
        special::model(&mut g, &self.main);
        for pass in 0..4 {
            if self.armor_pass(&mut g, e, pass, pose) == 15 {
                self.armor_glint(&mut g, e.ticks_existed as f32 + t);
            }
        }
        if let Some(helmet) = p.inventory.armor[3].filter(|s| s.id < 256) {
            g.push();
            special::part(&mut g, &self.main["bipedHead"], true);
            if inventory::in_3d(helmet.id as i16) {
                g.translate(0.0, -0.25, 0.0);
                g.rotate(180.0, 0.0, 1.0, 0.0);
                g.scale(0.625, -0.625, 0.625);
            }
            first_person::item(&mut g, helmet, first_person::Holder::player(e));
            g.pop();
        }
        if let Some(mut item) = held {
            g.push();
            special::part(&mut g, &self.main["bipedRightArm"], true);
            g.translate(-0.0625, 0.4375, 0.0625);
            if p.fish_entity.is_some() {item=worldgen::entity::player::ItemStack::new(280,1,0);}
            let action = (p.item_use_ticks > 0).then(|| use_action(item.id));
            if inventory::in_3d(item.id as i16) {
                let scale = 0.5f32 * 0.75;
                g.translate(0.0, 0.1875, -0.3125);
                g.rotate(20.0, 1.0, 0.0, 0.0);
                g.rotate(45.0, 0.0, 1.0, 0.0);
                g.scale(scale, -scale, scale);
            } else if item.id == 261 {
                g.translate(0.0, 0.125, 0.3125);
                g.rotate(-20.0, 0.0, 1.0, 0.0);
                g.scale(0.625, -0.625, 0.625);
                g.rotate(-100.0, 1.0, 0.0, 0.0);
                g.rotate(45.0, 0.0, 1.0, 0.0);
            } else if full_3d(item.id) {
                if item.id == 346 {
                    g.rotate(180.0, 0.0, 0.0, 1.0);
                    g.translate(0.0, -0.125, 0.0);
                }
                if action == Some(ItemUseAction::Block) {
                    g.translate(0.05, 0.0, -0.1);
                    g.rotate(-50.0, 0.0, 1.0, 0.0);
                    g.rotate(-10.0, 1.0, 0.0, 0.0);
                    g.rotate(-60.0, 0.0, 0.0, 1.0);
                }
                g.translate(0.0, 0.1875, 0.0);
                g.scale(0.625, -0.625, 0.625);
                g.rotate(-100.0, 1.0, 0.0, 0.0);
                g.rotate(45.0, 0.0, 1.0, 0.0);
            } else {
                g.translate(0.25, 0.1875, -0.1875);
                g.scale(0.375, 0.375, 0.375);
                g.rotate(60.0, 0.0, 0.0, 1.0);
                g.rotate(-90.0, 1.0, 0.0, 0.0);
                g.rotate(20.0, 0.0, 0.0, 1.0);
            }
            let holder = first_person::Holder::player(e);
            let times = first_person::current_glint_times();
            if item.id == 373 {
                let c = super::item_table::properties(373, item.damage as i16).1;
                g.material = [(c >> 16 & 255) as f32 / 255.0, (c >> 8 & 255) as f32 / 255.0, (c & 255) as f32 / 255.0, 1.0];
                first_person::item_pass(&mut g, item, holder, 0, times);
                g.material = [1.0; 4];
                first_person::item_pass(&mut g, item, holder, 1, times);
            } else {
                first_person::item_pass(&mut g, item, holder, 0, times);
            }
            g.pop();
        }
        if l.hurt_time > 0 || l.death_time > 0 {
            g.enable(8, false);
            g.enable(16, true);
            g.src = 770;
            g.dst = 771;
            g.depth = 514;
            g.material = [overlay_brightness, 0.0, 0.0, 0.4];
            special::model(&mut g, &self.main);
            for pass in 0..4 {
                g.material = [overlay_brightness, 0.0, 0.0, 0.4];
                self.armor_pass(&mut g, e, pass, pose);
            }
            g.depth = 515;
            g.enable(16, false);
            g.enable(8, true);
            g.enable(1, true);
        }
        g.enable(2, false);
        g.pop();
        for m in [&mut self.main, &mut self.chest, &mut self.armor] {
            m.aimed_bow = false;
            m.is_sneak = false;
            m.held_right = false;
            m.held_right_pose = 0;
        }
        g
    }
}
pub fn full_3d(id: i32) -> bool {
    matches!(id,256..=258|267..=280|283..=286|290..=294|346|352|353)
}
