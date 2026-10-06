//! Release 1.0 models. Numbered fields retain the original source names.
use super::{
    fixed::Fixed,
    geometry::{ModelRenderer, TextureOffset},
    models::{Model, ModelKind},
};
use worldgen::math::{self, PI_F32};

#[allow(unused_variables)]
impl Model {
    fn init_dragon(&mut self, var1: f32, var2: f32) {
        self.texture_size[0] = 256;
        self.texture_size[1] = 256;
        self.set_texture_offset("body.body", 0, 0);
        self.set_texture_offset("wing.skin", -56, 88);
        self.set_texture_offset("wingtip.skin", -56, 144);
        self.set_texture_offset("rearleg.main", 0, 0);
        self.set_texture_offset("rearfoot.main", 112, 0);
        self.set_texture_offset("rearlegtip.main", 196, 0);
        self.set_texture_offset("head.upperhead", 112, 30);
        self.set_texture_offset("wing.bone", 112, 88);
        self.set_texture_offset("head.upperlip", 176, 44);
        self.set_texture_offset("jaw.jaw", 176, 65);
        self.set_texture_offset("frontleg.main", 112, 104);
        self.set_texture_offset("wingtip.bone", 112, 136);
        self.set_texture_offset("frontfoot.main", 144, 104);
        self.set_texture_offset("neck.box", 192, 104);
        self.set_texture_offset("frontlegtip.main", 226, 138);
        self.set_texture_offset("body.scale", 220, 53);
        self.set_texture_offset("head.scale", 0, 0);
        self.set_texture_offset("neck.scale", 48, 0);
        self.set_texture_offset("head.nostril", 112, 0);
        let var2: f32 = -16.0f32;
        self.insert_part(
            "field_40314_a".into(),
            ModelRenderer::named("head", self.texture_size),
        );
        self.named_box(
            "field_40314_a",
            "upperlip",
            -6.0f32,
            -1.0f32,
            -8.0f32 + var2,
            12,
            5,
            16,
        );
        self.named_box(
            "field_40314_a",
            "upperhead",
            -8.0f32,
            -8.0f32,
            6.0f32 + var2,
            16,
            16,
            16,
        );
        self["field_40314_a"].mirror = true;
        self.named_box(
            "field_40314_a",
            "scale",
            -5.0f32,
            -12.0f32,
            12.0f32 + var2,
            2,
            4,
            6,
        );
        self.named_box(
            "field_40314_a",
            "nostril",
            -5.0f32,
            -3.0f32,
            -6.0f32 + var2,
            2,
            2,
            4,
        );
        self["field_40314_a"].mirror = false;
        self.named_box(
            "field_40314_a",
            "scale",
            3.0f32,
            -12.0f32,
            12.0f32 + var2,
            2,
            4,
            6,
        );
        self.named_box(
            "field_40314_a",
            "nostril",
            3.0f32,
            -3.0f32,
            -6.0f32 + var2,
            2,
            2,
            4,
        );
        self.insert_part(
            "field_40313_c".into(),
            ModelRenderer::named("jaw", self.texture_size),
        );
        self["field_40313_c"].set_rotation_point(0.0f32, 4.0f32, 8.0f32 + var2);
        self.named_box("field_40313_c", "jaw", -6.0f32, 0.0f32, -16.0f32, 12, 4, 16);
        self.attach("field_40314_a", "field_40313_c");
        self.insert_part(
            "field_40312_b".into(),
            ModelRenderer::named("neck", self.texture_size),
        );
        self.named_box(
            "field_40312_b",
            "box",
            -5.0f32,
            -5.0f32,
            -5.0f32,
            10,
            10,
            10,
        );
        self.named_box("field_40312_b", "scale", -1.0f32, -9.0f32, -3.0f32, 2, 4, 6);
        self.insert_part(
            "field_40310_d".into(),
            ModelRenderer::named("body", self.texture_size),
        );
        self["field_40310_d"].set_rotation_point(0.0f32, 4.0f32, 8.0f32);
        self.named_box(
            "field_40310_d",
            "body",
            -12.0f32,
            0.0f32,
            -16.0f32,
            24,
            24,
            64,
        );
        self.named_box(
            "field_40310_d",
            "scale",
            -1.0f32,
            -6.0f32,
            -10.0f32,
            2,
            6,
            12,
        );
        self.named_box(
            "field_40310_d",
            "scale",
            -1.0f32,
            -6.0f32,
            10.0f32,
            2,
            6,
            12,
        );
        self.named_box(
            "field_40310_d",
            "scale",
            -1.0f32,
            -6.0f32,
            30.0f32,
            2,
            6,
            12,
        );
        self.insert_part(
            "field_40319_q".into(),
            ModelRenderer::named("wing", self.texture_size),
        );
        self["field_40319_q"].set_rotation_point(-12.0f32, 5.0f32, 2.0f32);
        self.named_box(
            "field_40319_q",
            "bone",
            -56.0f32,
            -4.0f32,
            -4.0f32,
            56,
            8,
            8,
        );
        self.named_box("field_40319_q", "skin", -56.0f32, 0.0f32, 2.0f32, 56, 0, 56);
        self.insert_part(
            "field_40318_r".into(),
            ModelRenderer::named("wingtip", self.texture_size),
        );
        self["field_40318_r"].set_rotation_point(-56.0f32, 0.0f32, 0.0f32);
        self.named_box(
            "field_40318_r",
            "bone",
            -56.0f32,
            -2.0f32,
            -2.0f32,
            56,
            4,
            4,
        );
        self.named_box("field_40318_r", "skin", -56.0f32, 0.0f32, 2.0f32, 56, 0, 56);
        self.attach("field_40319_q", "field_40318_r");
        self.insert_part(
            "field_40308_f".into(),
            ModelRenderer::named("frontleg", self.texture_size),
        );
        self["field_40308_f"].set_rotation_point(-12.0f32, 20.0f32, 2.0f32);
        self.named_box("field_40308_f", "main", -4.0f32, -4.0f32, -4.0f32, 8, 24, 8);
        self.insert_part(
            "field_40315_n".into(),
            ModelRenderer::named("frontlegtip", self.texture_size),
        );
        self["field_40315_n"].set_rotation_point(0.0f32, 20.0f32, -1.0f32);
        self.named_box("field_40315_n", "main", -3.0f32, -1.0f32, -3.0f32, 6, 24, 6);
        self.attach("field_40308_f", "field_40315_n");
        self.insert_part(
            "field_40320_p".into(),
            ModelRenderer::named("frontfoot", self.texture_size),
        );
        self["field_40320_p"].set_rotation_point(0.0f32, 23.0f32, 0.0f32);
        self.named_box("field_40320_p", "main", -4.0f32, 0.0f32, -12.0f32, 8, 4, 16);
        self.attach("field_40315_n", "field_40320_p");
        self.insert_part(
            "field_40311_e".into(),
            ModelRenderer::named("rearleg", self.texture_size),
        );
        self["field_40311_e"].set_rotation_point(-16.0f32, 16.0f32, 42.0f32);
        self.named_box(
            "field_40311_e",
            "main",
            -8.0f32,
            -4.0f32,
            -8.0f32,
            16,
            32,
            16,
        );
        self.insert_part(
            "field_40309_g".into(),
            ModelRenderer::named("rearlegtip", self.texture_size),
        );
        self["field_40309_g"].set_rotation_point(0.0f32, 32.0f32, -4.0f32);
        self.named_box(
            "field_40309_g",
            "main",
            -6.0f32,
            -2.0f32,
            0.0f32,
            12,
            32,
            12,
        );
        self.attach("field_40311_e", "field_40309_g");
        self.insert_part(
            "field_40316_o".into(),
            ModelRenderer::named("rearfoot", self.texture_size),
        );
        self["field_40316_o"].set_rotation_point(0.0f32, 31.0f32, 4.0f32);
        self.named_box(
            "field_40316_o",
            "main",
            -9.0f32,
            0.0f32,
            -20.0f32,
            18,
            6,
            24,
        );
        self.attach("field_40309_g", "field_40316_o");
    }
    fn init_villager(&mut self, var1: f32, var2: f32) {
        let var3: i32 = 64;
        let var4: i32 = 64;
        self.insert_part("field_40340_a".into(), ModelRenderer::new(0, 0));
        self["field_40340_a"].set_texture_size(var3, var4);
        self["field_40340_a"].set_rotation_point(0.0f32, 0.0f32 + var2, 0.0f32);
        self["field_40340_a"]
            .set_texture_offset(0, 0)
            .add_box(-4.0f32, -10.0f32, -4.0f32, 8, 10, 8, var1);
        self["field_40340_a"]
            .set_texture_offset(24, 0)
            .add_box(-1.0f32, -3.0f32, -6.0f32, 2, 4, 2, var1);
        self.insert_part("field_40338_b".into(), ModelRenderer::new(0, 0));
        self["field_40338_b"].set_texture_size(var3, var4);
        self["field_40338_b"].set_rotation_point(0.0f32, 0.0f32 + var2, 0.0f32);
        self["field_40338_b"]
            .set_texture_offset(16, 20)
            .add_box(-4.0f32, 0.0f32, -3.0f32, 8, 12, 6, var1);
        self["field_40338_b"].set_texture_offset(0, 38).add_box(
            -4.0f32,
            0.0f32,
            -3.0f32,
            8,
            18,
            6,
            var1 + 0.5f32,
        );
        self.insert_part("field_40339_c".into(), ModelRenderer::new(0, 0));
        self["field_40339_c"].set_texture_size(var3, var4);
        self["field_40339_c"].set_rotation_point(0.0f32, 0.0f32 + var2 + 2.0f32, 0.0f32);
        self["field_40339_c"]
            .set_texture_offset(44, 22)
            .add_box(-8.0f32, -2.0f32, -2.0f32, 4, 8, 4, var1);
        self["field_40339_c"]
            .set_texture_offset(44, 22)
            .add_box(4.0f32, -2.0f32, -2.0f32, 4, 8, 4, var1);
        self["field_40339_c"]
            .set_texture_offset(40, 38)
            .add_box(-4.0f32, 2.0f32, -2.0f32, 8, 4, 4, var1);
        self.insert_part("field_40336_d".into(), ModelRenderer::new(0, 22));
        self["field_40336_d"].set_texture_size(var3, var4);
        self["field_40336_d"].set_rotation_point(-2.0f32, 12.0f32 + var2, 0.0f32);
        self["field_40336_d"].add_box(-2.0f32, 0.0f32, -2.0f32, 4, 12, 4, var1);
        self.insert_part("field_40337_e".into(), ModelRenderer::new(0, 22));
        self["field_40337_e"].set_texture_size(var3, var4);
        self["field_40337_e"].mirror = true;
        self["field_40337_e"].set_rotation_point(2.0f32, 12.0f32 + var2, 0.0f32);
        self["field_40337_e"].add_box(-2.0f32, 0.0f32, -2.0f32, 4, 12, 4, var1);
    }
    fn pose_villager(&mut self, p: [f32; 6]) {
        let [var1, var2, var3, var4, var5, var6] = p;
        self["field_40340_a"].rotate_angle_y = var4 / (180.0f32 / PI_F32);
        self["field_40340_a"].rotate_angle_x = var5 / (180.0f32 / PI_F32);
        self["field_40339_c"].rotation_point_y = 3.0f32;
        self["field_40339_c"].rotation_point_z = -1.0f32;
        self["field_40339_c"].rotate_angle_x = -(12.0f32 / 16.0f32);
        self["field_40336_d"].rotate_angle_x = math::cos(var1 * 0.6662f32) * 1.4f32 * var2 * 0.5f32;
        self["field_40337_e"].rotate_angle_x =
            math::cos(var1 * 0.6662f32 + PI_F32) * 1.4f32 * var2 * 0.5f32;
        self["field_40336_d"].rotate_angle_y = 0.0f32;
        self["field_40337_e"].rotate_angle_y = 0.0f32;
    }
    fn init_snow_man(&mut self, var1: f32, var2: f32) {
        let var1: f32 = 4.0f32;
        let var2: f32 = 0.0f32;
        self.insert_part("field_40305_c".into(), ModelRenderer::new(0, 0));
        self["field_40305_c"].set_texture_size(64, 64);
        self["field_40305_c"].add_box(-4.0f32, -8.0f32, -4.0f32, 8, 8, 8, var2 - 0.5f32);
        self["field_40305_c"].set_rotation_point(0.0f32, 0.0f32 + var1, 0.0f32);
        self.insert_part("field_40302_d".into(), ModelRenderer::new(32, 0));
        self["field_40302_d"].set_texture_size(64, 64);
        self["field_40302_d"].add_box(-1.0f32, 0.0f32, -1.0f32, 12, 2, 2, var2 - 0.5f32);
        self["field_40302_d"].set_rotation_point(0.0f32, 0.0f32 + var1 + 9.0f32 - 7.0f32, 0.0f32);
        self.insert_part("field_40303_e".into(), ModelRenderer::new(32, 0));
        self["field_40303_e"].set_texture_size(64, 64);
        self["field_40303_e"].add_box(-1.0f32, 0.0f32, -1.0f32, 12, 2, 2, var2 - 0.5f32);
        self["field_40303_e"].set_rotation_point(0.0f32, 0.0f32 + var1 + 9.0f32 - 7.0f32, 0.0f32);
        self.insert_part("field_40306_a".into(), ModelRenderer::new(0, 16));
        self["field_40306_a"].set_texture_size(64, 64);
        self["field_40306_a"].add_box(-5.0f32, -10.0f32, -5.0f32, 10, 10, 10, var2 - 0.5f32);
        self["field_40306_a"].set_rotation_point(0.0f32, 0.0f32 + var1 + 9.0f32, 0.0f32);
        self.insert_part("field_40304_b".into(), ModelRenderer::new(0, 36));
        self["field_40304_b"].set_texture_size(64, 64);
        self["field_40304_b"].add_box(-6.0f32, -12.0f32, -6.0f32, 12, 12, 12, var2 - 0.5f32);
        self["field_40304_b"].set_rotation_point(0.0f32, 0.0f32 + var1 + 20.0f32, 0.0f32);
    }
    fn pose_snow_man(&mut self, p: [f32; 6]) {
        let [var1, var2, var3, var4, var5, var6] = p;
        self["field_40305_c"].rotate_angle_y = var4 / (180.0f32 / PI_F32);
        self["field_40305_c"].rotate_angle_x = var5 / (180.0f32 / PI_F32);
        self["field_40306_a"].rotate_angle_y = var4 / (180.0f32 / PI_F32) * 0.25f32;
        let var7: f32 = math::sin(self["field_40306_a"].rotate_angle_y);
        let var8: f32 = math::cos(self["field_40306_a"].rotate_angle_y);
        self["field_40302_d"].rotate_angle_z = 1.0f32;
        self["field_40303_e"].rotate_angle_z = -1.0f32;
        self["field_40302_d"].rotate_angle_y = 0.0f32 + self["field_40306_a"].rotate_angle_y;
        self["field_40303_e"].rotate_angle_y = PI_F32 + self["field_40306_a"].rotate_angle_y;
        self["field_40302_d"].rotation_point_x = var8 * 5.0f32;
        self["field_40302_d"].rotation_point_z = -var7 * 5.0f32;
        self["field_40303_e"].rotation_point_x = -var8 * 5.0f32;
        self["field_40303_e"].rotation_point_z = var7 * 5.0f32;
    }
    fn init_book(&mut self, var1: f32, var2: f32) {
        self.insert_part("field_40330_a".into(), ModelRenderer::new(0, 0));
        self["field_40330_a"]
            .set_texture_offset(0, 0)
            .add_box(-6.0f32, -5.0f32, 0.0f32, 6, 10, 0, 0.0);
        self.insert_part("field_40328_b".into(), ModelRenderer::new(0, 0));
        self["field_40328_b"]
            .set_texture_offset(16, 0)
            .add_box(0.0f32, -5.0f32, 0.0f32, 6, 10, 0, 0.0);
        self.insert_part("field_40325_g".into(), ModelRenderer::new(0, 0));
        self["field_40325_g"]
            .set_texture_offset(12, 0)
            .add_box(-1.0f32, -5.0f32, 0.0f32, 2, 10, 0, 0.0);
        self.insert_part("field_40329_c".into(), ModelRenderer::new(0, 0));
        self["field_40329_c"]
            .set_texture_offset(0, 10)
            .add_box(0.0f32, -4.0f32, -0.99f32, 5, 8, 1, 0.0);
        self.insert_part("field_40326_d".into(), ModelRenderer::new(0, 0));
        self["field_40326_d"]
            .set_texture_offset(12, 10)
            .add_box(0.0f32, -4.0f32, -0.01f32, 5, 8, 1, 0.0);
        self.insert_part("field_40327_e".into(), ModelRenderer::new(0, 0));
        self["field_40327_e"]
            .set_texture_offset(24, 10)
            .add_box(0.0f32, -4.0f32, 0.0f32, 5, 8, 0, 0.0);
        self.insert_part("field_40324_f".into(), ModelRenderer::new(0, 0));
        self["field_40324_f"]
            .set_texture_offset(24, 10)
            .add_box(0.0f32, -4.0f32, 0.0f32, 5, 8, 0, 0.0);
        self["field_40330_a"].set_rotation_point(0.0f32, 0.0f32, -1.0f32);
        self["field_40328_b"].set_rotation_point(0.0f32, 0.0f32, 1.0f32);
        self["field_40325_g"].rotate_angle_y = PI_F32 * 0.5f32;
    }
    fn pose_book(&mut self, p: [f32; 6]) {
        let [var1, var2, var3, var4, var5, var6] = p;
        let var7: f32 = (math::sin(var1 * 0.02f32) * 0.1f32 + 1.25f32) * var4;
        self["field_40330_a"].rotate_angle_y = PI_F32 + var7;
        self["field_40328_b"].rotate_angle_y = -var7;
        self["field_40329_c"].rotate_angle_y = var7;
        self["field_40326_d"].rotate_angle_y = -var7;
        self["field_40327_e"].rotate_angle_y = var7 - var7 * 2.0f32 * var2;
        self["field_40324_f"].rotate_angle_y = var7 - var7 * 2.0f32 * var3;
        self["field_40329_c"].rotation_point_x = math::sin(var7);
        self["field_40326_d"].rotation_point_x = math::sin(var7);
        self["field_40327_e"].rotation_point_x = math::sin(var7);
        self["field_40324_f"].rotation_point_x = math::sin(var7);
    }
    fn init_ender_crystal(&mut self, var1: f32, var2: f32) {
        self.insert_part(
            "field_41058_h".into(),
            ModelRenderer::named("glass", self.texture_size),
        );
        self["field_41058_h"]
            .set_texture_offset(0, 0)
            .add_box(-4.0f32, -4.0f32, -4.0f32, 8, 8, 8, 0.0);
        self.insert_part(
            "field_41057_g".into(),
            ModelRenderer::named("cube", self.texture_size),
        );
        self["field_41057_g"]
            .set_texture_offset(32, 0)
            .add_box(-4.0f32, -4.0f32, -4.0f32, 8, 8, 8, 0.0);
        self.insert_part(
            "field_41059_i".into(),
            ModelRenderer::named("base", self.texture_size),
        );
        self["field_41059_i"]
            .set_texture_offset(0, 16)
            .add_box(-6.0f32, 0.0f32, -6.0f32, 12, 4, 12, 0.0);
    }
}

#[allow(unused_variables)]
impl Model {
    pub fn set_texture_offset(&mut self, name: &str, u: i32, v: i32) {
        self.texture_offsets
            .insert(name.into(), TextureOffset::new(u, v));
    }
    pub fn get_texture_offset(&self, name: &str) -> Option<TextureOffset> {
        self.texture_offsets.get(name).copied()
    }
    #[allow(clippy::too_many_arguments)]
    fn named_box(
        &mut self,
        part: &str,
        name: &str,
        x: f32,
        y: f32,
        z: f32,
        w: i32,
        h: i32,
        d: i32,
    ) {
        let key = format!(
            "{}.{}",
            self[part].box_name.as_deref().unwrap_or("null"),
            name
        );
        let uv = self
            .get_texture_offset(&key)
            .expect("named box texture offset");
        self[part].add_named_box(name, uv, x, y, z, w, h, d);
    }
    fn attach(&mut self, parent: &str, child: &str) {
        self.child_links.push((parent.into(), child.into()));
    }
    pub(super) fn init_release(&mut self, inflate: f32, offset: f32) {
        match self.kind {
            ModelKind::Dragon => self.init_dragon(inflate, offset),
            ModelKind::Villager => self.init_villager(inflate, offset),
            ModelKind::SnowMan => self.init_snow_man(inflate, offset),
            ModelKind::Book => self.init_book(inflate, offset),
            ModelKind::EnderCrystal => self.init_ender_crystal(inflate, offset),
            ModelKind::Blaze => {
                for i in 0..12 {
                    let mut p = ModelRenderer::new(0, 16);
                    p.add_box(0.0, 0.0, 0.0, 2, 8, 2, 0.0);
                    self.insert_part(format!("field_40323_a[{i}]"), p);
                }
                let mut p = ModelRenderer::new(0, 0);
                p.add_box(-4.0, -4.0, -4.0, 8, 8, 8, 0.0);
                self.insert_part("field_40322_b".into(), p);
            }
            ModelKind::MagmaCube => {
                for i in 0..8 {
                    let uv = match i {
                        2 => [24, 10],
                        3 => [24, 19],
                        _ => [0, i],
                    };
                    let mut p = ModelRenderer::new(uv[0], uv[1]);
                    p.add_box(-4.0, (16 + i) as f32, -4.0, 8, 1, 8, 0.0);
                    self.insert_part(format!("field_40345_a[{i}]"), p);
                }
                let mut p = ModelRenderer::new(0, 16);
                p.add_box(-2.0, 18.0, -2.0, 4, 4, 4, 0.0);
                self.insert_part("field_40344_b".into(), p);
            }
            _ => unreachable!(),
        }
    }
    pub(super) fn pose_release(&mut self, p: [f32; 6]) {
        match self.kind {
            ModelKind::Villager => self.pose_villager(p),
            ModelKind::SnowMan => self.pose_snow_man(p),
            ModelKind::Book => self.pose_book(p),
            ModelKind::Blaze => {
                let mut angle = p[2] * PI_F32 * -0.1;
                for i in 0..12 {
                    if i == 4 {
                        angle = PI_F32 * 0.25 + p[2] * PI_F32 * 0.03;
                    }
                    if i == 8 {
                        angle = PI_F32 * 0.15 + p[2] * PI_F32 * -0.05;
                    }
                    let (y, radius) = if i < 4 {
                        (-2.0 + math::cos(((i * 2) as f32 + p[2]) * 0.25), 9.0)
                    } else if i < 8 {
                        (2.0 + math::cos(((i * 2) as f32 + p[2]) * 0.25), 7.0)
                    } else {
                        (11.0 + math::cos((i as f32 * 1.5 + p[2]) * 0.5), 5.0)
                    };
                    let r = self.parts.get_mut(&format!("field_40323_a[{i}]")).unwrap();
                    r.rotation_point_y = y;
                    r.rotation_point_x = math::cos(angle) * radius;
                    r.rotation_point_z = math::sin(angle) * radius;
                    angle += PI_F32 * 0.5;
                }
                self["field_40322_b"].rotate_angle_y = p[3] / (180.0 / PI_F32);
                self["field_40322_b"].rotate_angle_x = p[4] / (180.0 / PI_F32);
            }
            _ => {}
        }
    }
    pub fn magma_animation(&mut self, prev: f32, current: f32, partial: f32) {
        let mut squish = prev + (current - prev) * partial;
        if squish < 0.0 {
            squish = 0.0;
        }
        for i in 0..8 {
            self.parts
                .get_mut(&format!("field_40345_a[{i}]"))
                .unwrap()
                .rotation_point_y = (-(4 - i)) as f32 * squish * 1.7;
        }
    }
    pub fn model_version(&self) -> i32 {
        match self.kind {
            ModelKind::Blaze => 8,
            ModelKind::MagmaCube => 5,
            _ => 0,
        }
    }
    pub(super) fn release_parts(&self) -> Vec<(&ModelRenderer, bool)> {
        let names: &[&str] = match self.kind {
            ModelKind::Villager => &[
                "field_40340_a",
                "field_40338_b",
                "field_40336_d",
                "field_40337_e",
                "field_40339_c",
            ],
            ModelKind::SnowMan => &[
                "field_40306_a",
                "field_40304_b",
                "field_40305_c",
                "field_40302_d",
                "field_40303_e",
            ],
            ModelKind::Book => &[
                "field_40330_a",
                "field_40328_b",
                "field_40325_g",
                "field_40329_c",
                "field_40326_d",
                "field_40327_e",
                "field_40324_f",
            ],
            ModelKind::Blaze => &["field_40322_b"],
            ModelKind::MagmaCube => &["field_40344_b"],
            ModelKind::Dragon | ModelKind::EnderCrystal => &[],
            _ => unreachable!(),
        };
        let mut parts: Vec<_> = names.iter().map(|n| (&self[*n], false)).collect();
        let array = match self.kind {
            ModelKind::Blaze => Some(("field_40323_a", 12)),
            ModelKind::MagmaCube => Some(("field_40345_a", 8)),
            _ => None,
        };
        if let Some((name, count)) = array {
            for i in 0..count {
                parts.push((&self.parts[&format!("{name}[{i}]")], false));
            }
        }
        parts
    }
    /// Child head/body transforms, after RenderLiving's transform and before the part.
    pub fn child_ops(&self, part: &ModelRenderer, scale: f32) -> Vec<super::fixed::Op> {
        if !self.is_child {
            return vec![];
        }
        let head = self
            .parts
            .iter()
            .find(|(_, p)| std::ptr::eq(*p, part))
            .map(|(n, _)| n.as_str())
            .unwrap_or("");
        let (head_names, y, z): (&[&str], f32, f32) = match self.kind {
            ModelKind::Quadruped
            | ModelKind::Cow
            | ModelKind::Pig
            | ModelKind::Sheep
            | ModelKind::SheepFur => (&["head"], self.child_head_y, self.child_head_z),
            ModelKind::Chicken => (&["head", "bill", "chin"], 5.0, 2.0),
            ModelKind::Wolf => (&["wolfHeadMain"], 5.0, 2.0),
            _ => return vec![],
        };
        let mut g = Fixed::new(1.0);
        if head_names.contains(&head) {
            g.translate(0.0, y * scale, z * scale);
        } else {
            g.scale(0.5, 0.5, 0.5);
            g.translate(0.0, 24.0 * scale, 0.0);
        }
        g.ops
    }
    /// Resolve model children by identity, preserving live animation fields.
    pub fn linked_part(&self, g: &mut Fixed, name: &str, scale: f32) {
        let p = &self[name];
        if p.hidden || !p.show_model {
            return;
        }
        p.render_fixed_children(g, scale, 0, |g| {
            for (parent, child) in &self.child_links {
                if parent == name {
                    self.linked_part(g, child, scale);
                }
            }
        });
    }

    pub fn crystal_render(&self, g: &mut Fixed, spin: f32, bob: f32, scale: f32) {
        g.push();
        g.scale(2.0, 2.0, 2.0);
        g.translate(0.0, -0.5, 0.0);
        self["field_41059_i"].render_fixed(g, scale, 0);
        g.rotate(spin, 0.0, 1.0, 0.0);
        g.translate(0.0, 0.8 + bob, 0.0);
        g.rotate(60.0, 0.7071, 0.0, 0.7071);
        self["field_41058_h"].render_fixed(g, scale, 0);
        for name in ["field_41058_h", "field_41057_g"] {
            g.scale(0.875, 0.875, 0.875);
            g.rotate(60.0, 0.7071, 0.0, 0.7071);
            g.rotate(spin, 0.0, 1.0, 0.0);
            self[name].render_fixed(g, scale, 0);
        }
        g.pop();
    }
}

/// ModelDragon reads interpolated movement history from the simulation; no AI is copied.
impl Model {
    pub fn dragon_render(
        &mut self,
        g: &mut Fixed,
        animation: f32,
        history: &[[f64; 3]; 24],
        scale: f32,
    ) {
        use worldgen::jmath::{java_cos as cos, java_sin as sin};
        fn wrap(mut a: f64) -> f32 {
            while a >= 180.0 {
                a -= 360.0;
            }
            while a < -180.0 {
                a += 360.0;
            }
            a as f32
        }
        let phase = animation * PI_F32 * 2.0;
        g.push();
        self["field_40313_c"].rotate_angle_x = (sin(phase as f64) + 1.0) as f32 * 0.2;
        let mut bob = (sin((phase - 1.0) as f64) + 1.0) as f32;
        bob = (bob * bob * 1.0 + bob * 2.0) * 0.05;
        g.translate(0.0, bob - 2.0, -3.0);
        g.rotate(bob * 2.0, 1.0, 0.0, 0.0);
        let base = history[6];
        let turn = wrap(history[5][0] - history[10][0]);
        let middle = wrap(history[5][0] + (turn / 2.0) as f64);
        let (mut y, mut z, mut x) = (20.0f32, -12.0f32, 0.0f32);
        for i in 0..5 {
            let sample = history[5 - i];
            let wobble = cos((i as f32 * 0.45 + phase) as f64) as f32 * 0.15;
            let p = &mut self["field_40312_b"];
            p.rotate_angle_y = wrap(sample[0] - base[0]) * PI_F32 / 180.0 * 1.5;
            p.rotate_angle_x = wobble + (sample[1] - base[1]) as f32 * PI_F32 / 180.0 * 1.5 * 5.0;
            p.rotate_angle_z = -wrap(sample[0] - middle as f64) * PI_F32 / 180.0 * 1.5;
            p.set_rotation_point(x, y, z);
            y = (y as f64 + sin(p.rotate_angle_x as f64) * 10.0) as f32;
            z = (z as f64 - cos(p.rotate_angle_y as f64) * cos(p.rotate_angle_x as f64) * 10.0)
                as f32;
            x = (x as f64 - sin(p.rotate_angle_y as f64) * cos(p.rotate_angle_x as f64) * 10.0)
                as f32;
            p.render_fixed(g, scale, 0);
        }
        self["field_40314_a"].set_rotation_point(x, y, z);
        self["field_40314_a"].rotate_angle_y = wrap(history[0][0] - base[0]) * PI_F32 / 180.0 * 1.0;
        self["field_40314_a"].rotate_angle_z =
            -wrap(history[0][0] - middle as f64) * PI_F32 / 180.0 * 1.0;
        self.linked_part(g, "field_40314_a", scale);
        g.push();
        g.translate(0.0, 1.0, 0.0);
        g.rotate(-turn * 1.5 * 1.0, 0.0, 0.0, 1.0);
        g.translate(0.0, -1.0, 0.0);
        self["field_40310_d"].rotate_angle_z = 0.0;
        self["field_40310_d"].render_fixed(g, scale, 0);
        for side in 0..2 {
            g.culling = true;
            self["field_40319_q"].rotate_angle_x = 0.125 - cos(phase as f64) as f32 * 0.2;
            self["field_40319_q"].rotate_angle_y = 0.25;
            self["field_40319_q"].rotate_angle_z = (sin(phase as f64) + 0.125) as f32 * 0.8;
            self["field_40318_r"].rotate_angle_z = -(sin((phase + 2.0) as f64) + 0.5) as f32 * 0.75;
            self["field_40311_e"].rotate_angle_x = 1.0 + bob * 0.1;
            self["field_40309_g"].rotate_angle_x = 0.5 + bob * 0.1;
            self["field_40316_o"].rotate_angle_x = 0.75 + bob * 0.1;
            self["field_40308_f"].rotate_angle_x = 1.3 + bob * 0.1;
            self["field_40315_n"].rotate_angle_x = -0.5 - bob * 0.1;
            self["field_40320_p"].rotate_angle_x = 0.75 + bob * 0.1;
            for name in ["field_40319_q", "field_40308_f", "field_40311_e"] {
                self.linked_part(g, name, scale);
            }
            g.scale(-1.0, 1.0, 1.0);
            if side == 0 {
                g.cull_face = 1;
            }
        }
        g.pop();
        g.cull_face = 2;
        g.culling = false;
        let mut wobble = -(sin(phase as f64) as f32) * 0.0;
        y = 10.0;
        z = 60.0;
        x = 0.0;
        let base = history[11];
        for i in 0..12 {
            let sample = history[12 + i];
            wobble =
                (wobble as f64 + sin((i as f32 * 0.45 + phase) as f64) * 0.05f32 as f64) as f32;
            let p = &mut self["field_40312_b"];
            p.rotate_angle_y = (wrap(sample[0] - base[0]) * 1.5 + 180.0) * PI_F32 / 180.0;
            p.rotate_angle_x = wobble + (sample[1] - base[1]) as f32 * PI_F32 / 180.0 * 1.5 * 5.0;
            p.rotate_angle_z = wrap(sample[0] - middle as f64) * PI_F32 / 180.0 * 1.5;
            p.set_rotation_point(x, y, z);
            y = (y as f64 + sin(p.rotate_angle_x as f64) * 10.0) as f32;
            z = (z as f64 - cos(p.rotate_angle_y as f64) * cos(p.rotate_angle_x as f64) * 10.0)
                as f32;
            x = (x as f64 - sin(p.rotate_angle_y as f64) * cos(p.rotate_angle_x as f64) * 10.0)
                as f32;
            p.render_fixed(g, scale, 0);
        }
        g.pop();
    }
}
