//! `ModelBase` and the mob model constructors/`setRotationAngles`.
//! Java part names are retained as keys, so oracle diagnostics identify the exact field.

use super::geometry::{ModelRenderer, TextureOffset};
use std::collections::BTreeMap;
use std::ops::{Index, IndexMut};
use worldgen::math::{self, PI_F32};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ModelKind {
    Biped,
    Zombie,
    Skeleton,
    Quadruped,
    Pig,
    Cow,
    SheepFur,
    Sheep,
    Chicken,
    Spider,
    Creeper,
    SlimeOuter,
    SlimeInner,
    Squid,
    Wolf,
    Ghast,
    Enderman,
    Silverfish,
    Chest,
    LargeChest,
    Minecart,
    Boat,
    Sign,
    Dragon,
    Villager,
    SnowMan,
    Blaze,
    MagmaCube,
    EnderCrystal,
    Book,
}

#[derive(Clone, Debug)]
pub struct Model {
    pub kind: ModelKind,
    pub parts: BTreeMap<String, ModelRenderer>,
    pub registered: Vec<String>,
    retired: BTreeMap<usize, ModelRenderer>,
    /// `ModelBase.onGround`: interpolated attack swing progress.
    pub on_ground: f32,
    pub is_riding: bool,
    pub texture_size: [i32; 2],
    pub texture_offsets: BTreeMap<String, TextureOffset>,
    pub child_links: Vec<(String, String)>,
    pub is_child: bool,
    pub child_head_y: f32,
    pub child_head_z: f32,
    pub held_left_pose: i32,
    pub held_right_pose: i32,
    pub aimed_bow: bool,
    pub held_left: bool,
    pub held_right: bool,
    pub is_sneak: bool,
    pub carrying: bool,
    pub screaming: bool,
}

impl Index<&str> for Model {
    type Output = ModelRenderer;
    fn index(&self, name: &str) -> &Self::Output {
        &self.parts[name]
    }
}
impl IndexMut<&str> for Model {
    fn index_mut(&mut self, name: &str) -> &mut Self::Output {
        self.parts.get_mut(name).expect("model part")
    }
}

impl Model {
    pub(super) fn insert_part(&mut self, name: String, part: ModelRenderer) {
        if let Some(old) = self.parts.remove(&name) {
            let index = self.registered.iter().rposition(|n| n == &name).unwrap();
            self.retired.insert(index, old);
        }
        self.registered.push(name.clone());
        self.parts.insert(name, part);
    }
    pub fn registered_part(&self, index: usize) -> &ModelRenderer {
        self.retired.get(&index).unwrap_or_else(|| &self.parts[&self.registered[index]])
    }
    pub fn random_part(&self, random: &mut worldgen::JavaRandom) -> &ModelRenderer {
        self.registered_part(random.next_int(self.registered.len() as i32) as usize)
    }

    pub fn new(kind: ModelKind) -> Self {
        Self::with_options(kind, 0.0, 0.0, 12)
    }
    /// Constructor parameters: box inflation, biped vertical offset, quadruped leg height.
    pub fn with_options(kind: ModelKind, inflate: f32, offset: f32, legs: i32) -> Self {
        let mut m = Self {
            kind,
            parts: BTreeMap::new(),
            registered: Vec::new(),
            retired: BTreeMap::new(),
            texture_size: [64, 32],
            texture_offsets: BTreeMap::new(),
            child_links: Vec::new(),
            is_child: true,
            child_head_y: 8.0,
            child_head_z: 4.0,
            held_left_pose: 0,
            held_right_pose: 0,
            aimed_bow: false,
            on_ground: 0.0,
            is_riding: false,
            held_left: false,
            held_right: false,
            is_sneak: false,
            carrying: false,
            screaming: false,
        };
        match kind {
            ModelKind::Dragon | ModelKind::Villager | ModelKind::SnowMan | ModelKind::Blaze | ModelKind::MagmaCube | ModelKind::EnderCrystal | ModelKind::Book => m.init_release(inflate, offset),
            ModelKind::Biped => m.init_biped(inflate, offset),
            ModelKind::Zombie => m.init_biped(0.0, 0.0),
            ModelKind::Skeleton => {
                m.init_biped(0.0, 0.0);
                m.init_skeleton();
            }
            ModelKind::Quadruped => m.init_quadruped(legs, inflate),
            ModelKind::Pig => {
                m.init_quadruped(6, inflate);
                m["head"].set_texture_offset(16, 16).add_box(-2.0, 0.0, -9.0, 4, 3, 1, inflate);
                m.child_head_y = 4.0;
            }
            ModelKind::Cow => m.init_cow(),
            ModelKind::SheepFur => m.init_sheep1(),
            ModelKind::Sheep => m.init_sheep2(),
            ModelKind::Chicken => m.init_chicken(),
            ModelKind::Spider => m.init_spider(),
            ModelKind::Creeper => m.init_creeper(inflate),
            ModelKind::SlimeOuter => m.init_slime(0),
            ModelKind::SlimeInner => m.init_slime(16),
            ModelKind::Squid => m.init_squid(),
            ModelKind::Wolf => m.init_wolf(),
            ModelKind::Ghast => m.init_ghast(),
            ModelKind::Enderman => {
                m.init_biped(0.0, -14.0);
                m.init_enderman();
            }
            ModelKind::Silverfish => m.init_silverfish(),
            ModelKind::Chest => m.init_chest(false),
            ModelKind::LargeChest => { m.init_chest(false); m.init_chest(true); },
            ModelKind::Minecart | ModelKind::Boat => {
                for (i, mut part) in super::vehicles::model_parts(kind == ModelKind::Minecart).into_iter().enumerate() {
                    if i == 5 { part.rotation_point_y = 4.0; }
                    m.insert_part(format!("side[{i}]"), part);
                }
            }
            ModelKind::Sign => {
                let mut board = ModelRenderer::new(0, 0);
                board.add_box(-12.0, -14.0, -1.0, 24, 12, 2, 0.0);
                m.insert_part("signBoard".into(), board);
                let mut stick = ModelRenderer::new(0, 14);
                stick.add_box(-1.0, -2.0, -1.0, 2, 14, 2, 0.0);
                m.insert_part("signStick".into(), stick);
            }
        }
        m
    }

    /// `setRotationAngles(limbSwing, limbAmount, age, headYaw, headPitch, scale)`.
    pub fn set_rotation_angles(&mut self, p: [f32; 6]) {
        match self.kind {
            ModelKind::Dragon | ModelKind::Villager | ModelKind::SnowMan | ModelKind::Blaze | ModelKind::MagmaCube | ModelKind::EnderCrystal | ModelKind::Book => self.pose_release(p),
            ModelKind::Enderman => {
                self.pose_biped(p);
                self.pose_enderman();
            }
            ModelKind::Silverfish => self.pose_silverfish(p[2]),
            ModelKind::Chest | ModelKind::LargeChest | ModelKind::Boat | ModelKind::Sign => {}
            ModelKind::Minecart => self.parts.get_mut("side[5]").unwrap().rotation_point_y = 4.0 - p[2],
            ModelKind::Biped => self.pose_biped(p),
            ModelKind::Zombie | ModelKind::Skeleton => {
                if self.kind == ModelKind::Skeleton { self.aimed_bow = true; }
                self.pose_biped(p);
                self.pose_zombie(p);
            }
            ModelKind::Quadruped | ModelKind::Pig | ModelKind::SheepFur | ModelKind::Sheep => {
                self.pose_quadruped(p);

            }
            ModelKind::Cow => {
                self.pose_quadruped(p);

            }
            ModelKind::Chicken => self.pose_chicken(p),
            ModelKind::Spider => self.pose_spider(p),
            ModelKind::Creeper => self.pose_creeper(p),
            ModelKind::SlimeOuter | ModelKind::SlimeInner => {}
            ModelKind::Squid => {
                for i in 0..8 {
                    self.parts
                        .get_mut(&format!("squidTentacles[{i}]"))
                        .unwrap()
                        .rotate_angle_x = p[2];
                }
            }
            ModelKind::Wolf => self.pose_wolf(p),
            ModelKind::Ghast => {
                for i in 0..9 {
                    self.parts
                        .get_mut(&format!("tentacles[{i}]"))
                        .unwrap()
                        .rotate_angle_x = 0.2 * math::sin(p[2] * 0.3 + i as f32) + 0.4;
                }
            }
        }
    }
    fn init_chest(&mut self, large: bool) {
        let width = if large { 30 } else { 14 };
        let tex = if large { 128.0 } else { 64.0 };
        for (name, uv, pos, dim, point) in [
            (
                "field_35405_a",
                [0, 0],
                [0.0, -5.0, -14.0],
                [width, 5, 14],
                [1.0, 7.0, 15.0],
            ),
            (
                "field_35404_c",
                [0, 0],
                [-1.0, -2.0, -15.0],
                [2, 4, 1],
                [if large { 16.0 } else { 8.0 }, 7.0, 15.0],
            ),
            (
                "field_35403_b",
                [0, 19],
                [0.0, 0.0, 0.0],
                [width, 10, 14],
                [1.0, 6.0, 1.0],
            ),
        ] {
            let mut r = ModelRenderer::new(uv[0], uv[1]);
            r.texture_size = [tex, 64.0];
            r.add_box(pos[0], pos[1], pos[2], dim[0], dim[1], dim[2], 0.0);
            r.set_rotation_point(point[0], point[1], point[2]);
            self.insert_part(name.into(), r);
        }
    }
    fn init_enderman(&mut self) {
        for (name, uv, boxpos, dims, point, inflate, mirror) in [
            (
                "bipedHeadwear",
                [0, 16],
                [-4.0, -8.0, -4.0],
                [8, 8, 8],
                [0.0, -14.0, 0.0],
                -0.5,
                false,
            ),
            (
                "bipedBody",
                [32, 16],
                [-4.0, 0.0, -2.0],
                [8, 12, 4],
                [0.0, -14.0, 0.0],
                0.0,
                false,
            ),
            (
                "bipedRightArm",
                [56, 0],
                [-1.0, -2.0, -1.0],
                [2, 30, 2],
                [-3.0, -12.0, 0.0],
                0.0,
                false,
            ),
            (
                "bipedLeftArm",
                [56, 0],
                [-1.0, -2.0, -1.0],
                [2, 30, 2],
                [5.0, -12.0, 0.0],
                0.0,
                true,
            ),
            (
                "bipedRightLeg",
                [56, 0],
                [-1.0, 0.0, -1.0],
                [2, 30, 2],
                [-2.0, -2.0, 0.0],
                0.0,
                false,
            ),
            (
                "bipedLeftLeg",
                [56, 0],
                [-1.0, 0.0, -1.0],
                [2, 30, 2],
                [2.0, -2.0, 0.0],
                0.0,
                true,
            ),
        ] {
            let mut r = ModelRenderer::new(uv[0], uv[1]);
            r.mirror = mirror;
            r.add_box(
                boxpos[0], boxpos[1], boxpos[2], dims[0], dims[1], dims[2], inflate,
            );
            r.set_rotation_point(point[0], point[1], point[2]);
            self.insert_part(name.into(), r);
        }
    }
    fn pose_enderman(&mut self) {
        self["bipedHead"].show_model = true;
        let var7: f32 = -14.0f32;
        self["bipedBody"].rotate_angle_x = 0.0f32;
        self["bipedBody"].rotation_point_y = var7;
        self["bipedBody"].rotation_point_z = -0.0f32;
        self["bipedRightLeg"].rotate_angle_x -= 0.0f32;
        self["bipedLeftLeg"].rotate_angle_x -= 0.0f32;
        self["bipedRightArm"].rotate_angle_x =
            (self["bipedRightArm"].rotate_angle_x as f64 * 0.5f64) as f32;
        self["bipedLeftArm"].rotate_angle_x =
            (self["bipedLeftArm"].rotate_angle_x as f64 * 0.5f64) as f32;
        self["bipedRightLeg"].rotate_angle_x =
            (self["bipedRightLeg"].rotate_angle_x as f64 * 0.5f64) as f32;
        self["bipedLeftLeg"].rotate_angle_x =
            (self["bipedLeftLeg"].rotate_angle_x as f64 * 0.5f64) as f32;
        let var8: f32 = 0.4f32;
        if self["bipedRightArm"].rotate_angle_x > var8 {
            self["bipedRightArm"].rotate_angle_x = var8;
        }

        if self["bipedLeftArm"].rotate_angle_x > var8 {
            self["bipedLeftArm"].rotate_angle_x = var8;
        }

        if self["bipedRightArm"].rotate_angle_x < -var8 {
            self["bipedRightArm"].rotate_angle_x = -var8;
        }

        if self["bipedLeftArm"].rotate_angle_x < -var8 {
            self["bipedLeftArm"].rotate_angle_x = -var8;
        }

        if self["bipedRightLeg"].rotate_angle_x > var8 {
            self["bipedRightLeg"].rotate_angle_x = var8;
        }

        if self["bipedLeftLeg"].rotate_angle_x > var8 {
            self["bipedLeftLeg"].rotate_angle_x = var8;
        }

        if self["bipedRightLeg"].rotate_angle_x < -var8 {
            self["bipedRightLeg"].rotate_angle_x = -var8;
        }

        if self["bipedLeftLeg"].rotate_angle_x < -var8 {
            self["bipedLeftLeg"].rotate_angle_x = -var8;
        }

        if self.carrying {
            self["bipedRightArm"].rotate_angle_x = -0.5f32;
            self["bipedLeftArm"].rotate_angle_x = -0.5f32;
            self["bipedRightArm"].rotate_angle_z = 0.05f32;
            self["bipedLeftArm"].rotate_angle_z = -0.05f32;
        }

        self["bipedRightArm"].rotation_point_z = 0.0f32;
        self["bipedLeftArm"].rotation_point_z = 0.0f32;
        self["bipedRightLeg"].rotation_point_z = 0.0f32;
        self["bipedLeftLeg"].rotation_point_z = 0.0f32;
        self["bipedRightLeg"].rotation_point_y = 9.0f32 + var7;
        self["bipedLeftLeg"].rotation_point_y = 9.0f32 + var7;
        self["bipedHead"].rotation_point_z = -0.0f32;
        self["bipedHead"].rotation_point_y = var7 + 1.0f32;
        self["bipedHeadwear"].rotation_point_x = self["bipedHead"].rotation_point_x;
        self["bipedHeadwear"].rotation_point_y = self["bipedHead"].rotation_point_y;
        self["bipedHeadwear"].rotation_point_z = self["bipedHead"].rotation_point_z;
        self["bipedHeadwear"].rotate_angle_x = self["bipedHead"].rotate_angle_x;
        self["bipedHeadwear"].rotate_angle_y = self["bipedHead"].rotate_angle_y;
        self["bipedHeadwear"].rotate_angle_z = self["bipedHead"].rotate_angle_z;
        if self.screaming {
            let var9: f32 = 1.0f32;
            self["bipedHead"].rotation_point_y -= var9 * 5.0f32;
        }
    }
    fn init_silverfish(&mut self) {
        let sizes = [
            [3, 2, 2],
            [4, 3, 2],
            [6, 4, 3],
            [3, 3, 3],
            [2, 2, 3],
            [2, 1, 2],
            [1, 1, 2],
        ];
        let uv = [[0, 0], [0, 4], [0, 9], [0, 16], [0, 22], [11, 0], [13, 4]];
        let mut z = -3.5f32;
        let mut zs = [0.0; 7];
        for i in 0..7 {
            let [x, y, d] = sizes[i];
            let mut r = ModelRenderer::new(uv[i][0], uv[i][1]);
            r.add_box(x as f32 * -0.5, 0.0, d as f32 * -0.5, x, y, d, 0.0);
            r.set_rotation_point(0.0, (24 - y) as f32, z);
            self.insert_part(format!("field_35400_a[{i}]"), r);
            zs[i] = z;
            if i < 6 {
                z += (d + sizes[i + 1][2]) as f32 * 0.5;
            }
        }
        for (i, uv, x, y, d, z) in [
            (0, [20, 0], 10, 8, 3, zs[2]),
            (1, [20, 11], 6, 4, 3, zs[4]),
            (2, [20, 18], 6, 5, 2, zs[1]),
        ] {
            let mut r = ModelRenderer::new(uv[0], uv[1]);
            r.add_box(x as f32 * -0.5, 0.0, -1.5, x, y, d, 0.0);
            r.set_rotation_point(0.0, (24 - y) as f32, z);
            self.insert_part(format!("field_35398_b[{i}]"), r);
        }
    }
    fn pose_silverfish(&mut self, age: f32) {
        for i in 0i32..7 {
            let r = self.parts.get_mut(&format!("field_35400_a[{i}]")).unwrap();
            r.rotate_angle_y = math::cos(age * 0.9 + i as f32 * 0.15 * PI_F32)
                * PI_F32
                * 0.05
                * (1 + (i - 2).abs()) as f32;
            r.rotation_point_x = math::sin(age * 0.9 + i as f32 * 0.15 * PI_F32)
                * PI_F32
                * 0.2
                * (i - 2).abs() as f32;
        }
        for (i, src) in [(0, 2), (1, 4), (2, 1)] {
            let r = self.parts[&format!("field_35400_a[{src}]")].clone();
            let wing = self.parts.get_mut(&format!("field_35398_b[{i}]")).unwrap();
            wing.rotate_angle_y = r.rotate_angle_y;
            if i > 0 {
                wing.rotation_point_x = r.rotation_point_x;
            }
        }
    }
    fn pose_skeleton(&mut self, age: f32) {
        self["bipedRightArm"].rotate_angle_z = 0.0;
        self["bipedLeftArm"].rotate_angle_z = 0.0;
        self["bipedRightArm"].rotate_angle_y = -0.1 + self["bipedHead"].rotate_angle_y;
        self["bipedLeftArm"].rotate_angle_y = 0.1 + self["bipedHead"].rotate_angle_y + 0.4;
        self["bipedRightArm"].rotate_angle_x = PI_F32 * -0.5 + self["bipedHead"].rotate_angle_x;
        self["bipedLeftArm"].rotate_angle_x = PI_F32 * -0.5 + self["bipedHead"].rotate_angle_x;
        self["bipedRightArm"].rotate_angle_z += math::cos(age * 0.09) * 0.05 + 0.05;
        self["bipedLeftArm"].rotate_angle_z -= math::cos(age * 0.09) * 0.05 + 0.05;
        self["bipedRightArm"].rotate_angle_x += math::sin(age * 0.067) * 0.05;
        self["bipedLeftArm"].rotate_angle_x -= math::sin(age * 0.067) * 0.05;
    }

    fn init_ghast(&mut self) {
        let mut body = ModelRenderer::new(0, 0);
        body.add_box(-8.0, -8.0, -8.0, 16, 16, 16, 0.0);
        body.rotation_point_y += 8.0;
        self.insert_part("body".into(), body);
        let mut random = worldgen::JavaRandom::new(1660);
        for i in 0..9 {
            let mut t = ModelRenderer::new(0, 0);
            let x = ((i % 3) as f32 - (i / 3 % 2) as f32 * 0.5 + 0.25) / 2.0 * 2.0 - 1.0;
            let z = (i / 3) as f32 / 2.0 * 2.0 - 1.0;
            t.add_box(-1.0, 0.0, -1.0, 2, random.next_int(7) + 8, 2, 0.0);
            t.set_rotation_point(x * 5.0, 15.0, z * 5.0);
            self.insert_part(format!("tentacles[{i}]"), t);
        }
    }

    /// `ModelBiped` constructor.
    #[allow(unused_mut)]
    fn init_biped(&mut self, var1: f32, var2: f32) {
        self.held_left = false;
        self.held_right = false;
        self.is_sneak = false;
        self.insert_part("bipedCloak".into(), ModelRenderer::new(0, 0));
        self["bipedCloak"].add_box(-5.0f32, 0.0f32, -1.0f32, 10, 16, 1, var1);
        self.insert_part("bipedEars".into(), ModelRenderer::new(24, 0));
        self["bipedEars"].add_box(-3.0f32, -6.0f32, -1.0f32, 6, 6, 1, var1);
        self.insert_part("bipedHead".into(), ModelRenderer::new(0, 0));
        self["bipedHead"].add_box(-4.0f32, -8.0f32, -4.0f32, 8, 8, 8, var1);
        self["bipedHead"].set_rotation_point(0.0f32, 0.0f32 + var2, 0.0f32);
        self.insert_part("bipedHeadwear".into(), ModelRenderer::new(32, 0));
        self["bipedHeadwear"].add_box(-4.0f32, -8.0f32, -4.0f32, 8, 8, 8, var1 + 0.5f32);
        self["bipedHeadwear"].set_rotation_point(0.0f32, 0.0f32 + var2, 0.0f32);
        self.insert_part("bipedBody".into(), ModelRenderer::new(16, 16));
        self["bipedBody"].add_box(-4.0f32, 0.0f32, -2.0f32, 8, 12, 4, var1);
        self["bipedBody"].set_rotation_point(0.0f32, 0.0f32 + var2, 0.0f32);
        self.insert_part("bipedRightArm".into(), ModelRenderer::new(40, 16));
        self["bipedRightArm"].add_box(-3.0f32, -2.0f32, -2.0f32, 4, 12, 4, var1);
        self["bipedRightArm"].set_rotation_point(-5.0f32, 2.0f32 + var2, 0.0f32);
        self.insert_part("bipedLeftArm".into(), ModelRenderer::new(40, 16));
        self["bipedLeftArm"].mirror = true;
        self["bipedLeftArm"].add_box(-1.0f32, -2.0f32, -2.0f32, 4, 12, 4, var1);
        self["bipedLeftArm"].set_rotation_point(5.0f32, 2.0f32 + var2, 0.0f32);
        self.insert_part("bipedRightLeg".into(), ModelRenderer::new(0, 16));
        self["bipedRightLeg"].add_box(-2.0f32, 0.0f32, -2.0f32, 4, 12, 4, var1);
        self["bipedRightLeg"].set_rotation_point(-2.0f32, 12.0f32 + var2, 0.0f32);
        self.insert_part("bipedLeftLeg".into(), ModelRenderer::new(0, 16));
        self["bipedLeftLeg"].mirror = true;
        self["bipedLeftLeg"].add_box(-2.0f32, 0.0f32, -2.0f32, 4, 12, 4, var1);
        self["bipedLeftLeg"].set_rotation_point(2.0f32, 12.0f32 + var2, 0.0f32);
    }

    /// `ModelSkeleton` constructor.
    #[allow(unused_mut)]
    fn init_skeleton(&mut self) {
        let mut var1: f32 = 0.0f32;
        self.insert_part("bipedRightArm".into(), ModelRenderer::new(40, 16));
        self["bipedRightArm"].add_box(-1.0f32, -2.0f32, -1.0f32, 2, 12, 2, var1);
        self["bipedRightArm"].set_rotation_point(-5.0f32, 2.0f32, 0.0f32);
        self.insert_part("bipedLeftArm".into(), ModelRenderer::new(40, 16));
        self["bipedLeftArm"].mirror = true;
        self["bipedLeftArm"].add_box(-1.0f32, -2.0f32, -1.0f32, 2, 12, 2, var1);
        self["bipedLeftArm"].set_rotation_point(5.0f32, 2.0f32, 0.0f32);
        self.insert_part("bipedRightLeg".into(), ModelRenderer::new(0, 16));
        self["bipedRightLeg"].add_box(-1.0f32, 0.0f32, -1.0f32, 2, 12, 2, var1);
        self["bipedRightLeg"].set_rotation_point(-2.0f32, 12.0f32, 0.0f32);
        self.insert_part("bipedLeftLeg".into(), ModelRenderer::new(0, 16));
        self["bipedLeftLeg"].mirror = true;
        self["bipedLeftLeg"].add_box(-1.0f32, 0.0f32, -1.0f32, 2, 12, 2, var1);
        self["bipedLeftLeg"].set_rotation_point(2.0f32, 12.0f32, 0.0f32);
    }

    /// `ModelQuadruped` constructor.
    #[allow(unused_mut)]
    fn init_quadruped(&mut self, var1: i32, var2: f32) {
        self.insert_part("head".into(), ModelRenderer::new(0, 0));

        self["head"].add_box(-4.0f32, -4.0f32, -8.0f32, 8, 8, 8, var2);
        self["head"].set_rotation_point(0.0f32, (18 - var1) as f32, -6.0f32);
        self.insert_part("body".into(), ModelRenderer::new(28, 8));
        self["body"].add_box(-5.0f32, -10.0f32, -7.0f32, 10, 16, 8, var2);
        self["body"].set_rotation_point(0.0f32, (17 - var1) as f32, 2.0f32);
        self.insert_part("leg1".into(), ModelRenderer::new(0, 16));
        self["leg1"].add_box(-2.0f32, 0.0f32, -2.0f32, 4, var1, 4, var2);
        self["leg1"].set_rotation_point(-3.0f32, (24 - var1) as f32, 7.0f32);
        self.insert_part("leg2".into(), ModelRenderer::new(0, 16));
        self["leg2"].add_box(-2.0f32, 0.0f32, -2.0f32, 4, var1, 4, var2);
        self["leg2"].set_rotation_point(3.0f32, (24 - var1) as f32, 7.0f32);
        self.insert_part("leg3".into(), ModelRenderer::new(0, 16));
        self["leg3"].add_box(-2.0f32, 0.0f32, -2.0f32, 4, var1, 4, var2);
        self["leg3"].set_rotation_point(-3.0f32, (24 - var1) as f32, -5.0f32);
        self.insert_part("leg4".into(), ModelRenderer::new(0, 16));
        self["leg4"].add_box(-2.0f32, 0.0f32, -2.0f32, 4, var1, 4, var2);
        self["leg4"].set_rotation_point(3.0f32, (24 - var1) as f32, -5.0f32);
    }

    /// `ModelCow` constructor.
    #[allow(unused_mut)]
    fn init_cow(&mut self) {
        self.init_quadruped(12, 0.0f32);
        self.insert_part("head".into(), ModelRenderer::new(0, 0));
        self["head"].add_box(-4.0f32, -4.0f32, -6.0f32, 8, 8, 6, 0.0f32);
        self["head"].set_rotation_point(0.0f32, 4.0f32, -8.0f32);
        self["head"].set_texture_offset(22, 0).add_box(-5.0, -5.0, -4.0, 1, 3, 1, 0.0);
        self["head"].set_texture_offset(22, 0).add_box(4.0, -5.0, -4.0, 1, 3, 1, 0.0);
        self.insert_part("body".into(), ModelRenderer::new(18, 4));
        self["body"].add_box(-6.0f32, -10.0f32, -7.0f32, 12, 18, 10, 0.0f32);
        self["body"].set_rotation_point(0.0f32, 5.0f32, 2.0f32);
        self["body"].set_texture_offset(52, 0).add_box(-2.0, 2.0, -8.0, 4, 6, 1, 0.0);
        self.child_head_z += 2.0;
        self["leg1"].rotation_point_x -= 1.0;
        self["leg2"].rotation_point_x += 1.0;
        self["leg1"].rotation_point_z += 0.0f32;
        self["leg2"].rotation_point_z += 0.0f32;
        self["leg3"].rotation_point_x -= 1.0;
        self["leg4"].rotation_point_x += 1.0;
        self["leg3"].rotation_point_z -= 1.0;
        self["leg4"].rotation_point_z -= 1.0;
    }

    /// `ModelSheep1` constructor.
    #[allow(unused_mut)]
    fn init_sheep1(&mut self) {
        self.init_quadruped(12, 0.0f32);
        self.insert_part("head".into(), ModelRenderer::new(0, 0));
        self["head"].add_box(-3.0f32, -4.0f32, -4.0f32, 6, 6, 6, 0.6f32);
        self["head"].set_rotation_point(0.0f32, 6.0f32, -8.0f32);
        self.insert_part("body".into(), ModelRenderer::new(28, 8));
        self["body"].add_box(-4.0f32, -10.0f32, -7.0f32, 8, 16, 6, 1.75f32);
        self["body"].set_rotation_point(0.0f32, 5.0f32, 2.0f32);
        let mut var1: f32 = 0.5f32;
        self.insert_part("leg1".into(), ModelRenderer::new(0, 16));
        self["leg1"].add_box(-2.0f32, 0.0f32, -2.0f32, 4, 6, 4, var1);
        self["leg1"].set_rotation_point(-3.0f32, 12.0f32, 7.0f32);
        self.insert_part("leg2".into(), ModelRenderer::new(0, 16));
        self["leg2"].add_box(-2.0f32, 0.0f32, -2.0f32, 4, 6, 4, var1);
        self["leg2"].set_rotation_point(3.0f32, 12.0f32, 7.0f32);
        self.insert_part("leg3".into(), ModelRenderer::new(0, 16));
        self["leg3"].add_box(-2.0f32, 0.0f32, -2.0f32, 4, 6, 4, var1);
        self["leg3"].set_rotation_point(-3.0f32, 12.0f32, -5.0f32);
        self.insert_part("leg4".into(), ModelRenderer::new(0, 16));
        self["leg4"].add_box(-2.0f32, 0.0f32, -2.0f32, 4, 6, 4, var1);
        self["leg4"].set_rotation_point(3.0f32, 12.0f32, -5.0f32);
    }

    /// `ModelSheep2` constructor.
    #[allow(unused_mut)]
    fn init_sheep2(&mut self) {
        self.init_quadruped(12, 0.0f32);
        self.insert_part("head".into(), ModelRenderer::new(0, 0));
        self["head"].add_box(-3.0f32, -4.0f32, -6.0f32, 6, 6, 8, 0.0f32);
        self["head"].set_rotation_point(0.0f32, 6.0f32, -8.0f32);
        self.insert_part("body".into(), ModelRenderer::new(28, 8));
        self["body"].add_box(-4.0f32, -10.0f32, -7.0f32, 8, 16, 6, 0.0f32);
        self["body"].set_rotation_point(0.0f32, 5.0f32, 2.0f32);
    }

    /// `ModelChicken` constructor.
    #[allow(unused_mut)]
    fn init_chicken(&mut self) {
        let var1: i32 = 16;
        self.insert_part("head".into(), ModelRenderer::new(0, 0));
        self["head"].add_box(-2.0f32, -6.0f32, -2.0f32, 4, 6, 3, 0.0f32);
        self["head"].set_rotation_point(0.0f32, (-1 + var1) as f32, -4.0f32);
        self.insert_part("bill".into(), ModelRenderer::new(14, 0));
        self["bill"].add_box(-2.0f32, -4.0f32, -4.0f32, 4, 2, 2, 0.0f32);
        self["bill"].set_rotation_point(0.0f32, (-1 + var1) as f32, -4.0f32);
        self.insert_part("chin".into(), ModelRenderer::new(14, 4));
        self["chin"].add_box(-1.0f32, -2.0f32, -3.0f32, 2, 2, 2, 0.0f32);
        self["chin"].set_rotation_point(0.0f32, (-1 + var1) as f32, -4.0f32);
        self.insert_part("body".into(), ModelRenderer::new(0, 9));
        self["body"].add_box(-3.0f32, -4.0f32, -3.0f32, 6, 8, 6, 0.0f32);
        self["body"].set_rotation_point(0.0f32, (0 + var1) as f32, 0.0f32);
        self.insert_part("rightLeg".into(), ModelRenderer::new(26, 0));
        self["rightLeg"].add_box(-1.0f32, 0.0f32, -3.0f32, 3, 5, 3, 0.0);
        self["rightLeg"].set_rotation_point(-2.0f32, (3 + var1) as f32, 1.0f32);
        self.insert_part("leftLeg".into(), ModelRenderer::new(26, 0));
        self["leftLeg"].add_box(-1.0f32, 0.0f32, -3.0f32, 3, 5, 3, 0.0);
        self["leftLeg"].set_rotation_point(1.0f32, (3 + var1) as f32, 1.0f32);
        self.insert_part("rightWing".into(), ModelRenderer::new(24, 13));
        self["rightWing"].add_box(0.0f32, 0.0f32, -3.0f32, 1, 4, 6, 0.0);
        self["rightWing"].set_rotation_point(-4.0f32, (-3 + var1) as f32, 0.0f32);
        self.insert_part("leftWing".into(), ModelRenderer::new(24, 13));
        self["leftWing"].add_box(-1.0f32, 0.0f32, -3.0f32, 1, 4, 6, 0.0);
        self["leftWing"].set_rotation_point(4.0f32, (-3 + var1) as f32, 0.0f32);
    }

    /// `ModelSpider` constructor.
    #[allow(unused_mut)]
    fn init_spider(&mut self) {
        let mut var1: f32 = 0.0f32;
        let var2: i32 = 15;
        self.insert_part("spiderHead".into(), ModelRenderer::new(32, 4));
        self["spiderHead"].add_box(-4.0f32, -4.0f32, -8.0f32, 8, 8, 8, var1);
        self["spiderHead"].set_rotation_point(0.0f32, (0 + var2) as f32, -3.0f32);
        self.insert_part("spiderNeck".into(), ModelRenderer::new(0, 0));
        self["spiderNeck"].add_box(-3.0f32, -3.0f32, -3.0f32, 6, 6, 6, var1);
        self["spiderNeck"].set_rotation_point(0.0f32, var2 as f32, 0.0f32);
        self.insert_part("spiderBody".into(), ModelRenderer::new(0, 12));
        self["spiderBody"].add_box(-5.0f32, -4.0f32, -6.0f32, 10, 8, 12, var1);
        self["spiderBody"].set_rotation_point(0.0f32, (0 + var2) as f32, 9.0f32);
        self.insert_part("spiderLeg1".into(), ModelRenderer::new(18, 0));
        self["spiderLeg1"].add_box(-15.0f32, -1.0f32, -1.0f32, 16, 2, 2, var1);
        self["spiderLeg1"].set_rotation_point(-4.0f32, (0 + var2) as f32, 2.0f32);
        self.insert_part("spiderLeg2".into(), ModelRenderer::new(18, 0));
        self["spiderLeg2"].add_box(-1.0f32, -1.0f32, -1.0f32, 16, 2, 2, var1);
        self["spiderLeg2"].set_rotation_point(4.0f32, (0 + var2) as f32, 2.0f32);
        self.insert_part("spiderLeg3".into(), ModelRenderer::new(18, 0));
        self["spiderLeg3"].add_box(-15.0f32, -1.0f32, -1.0f32, 16, 2, 2, var1);
        self["spiderLeg3"].set_rotation_point(-4.0f32, (0 + var2) as f32, 1.0f32);
        self.insert_part("spiderLeg4".into(), ModelRenderer::new(18, 0));
        self["spiderLeg4"].add_box(-1.0f32, -1.0f32, -1.0f32, 16, 2, 2, var1);
        self["spiderLeg4"].set_rotation_point(4.0f32, (0 + var2) as f32, 1.0f32);
        self.insert_part("spiderLeg5".into(), ModelRenderer::new(18, 0));
        self["spiderLeg5"].add_box(-15.0f32, -1.0f32, -1.0f32, 16, 2, 2, var1);
        self["spiderLeg5"].set_rotation_point(-4.0f32, (0 + var2) as f32, 0.0f32);
        self.insert_part("spiderLeg6".into(), ModelRenderer::new(18, 0));
        self["spiderLeg6"].add_box(-1.0f32, -1.0f32, -1.0f32, 16, 2, 2, var1);
        self["spiderLeg6"].set_rotation_point(4.0f32, (0 + var2) as f32, 0.0f32);
        self.insert_part("spiderLeg7".into(), ModelRenderer::new(18, 0));
        self["spiderLeg7"].add_box(-15.0f32, -1.0f32, -1.0f32, 16, 2, 2, var1);
        self["spiderLeg7"].set_rotation_point(-4.0f32, (0 + var2) as f32, -1.0f32);
        self.insert_part("spiderLeg8".into(), ModelRenderer::new(18, 0));
        self["spiderLeg8"].add_box(-1.0f32, -1.0f32, -1.0f32, 16, 2, 2, var1);
        self["spiderLeg8"].set_rotation_point(4.0f32, (0 + var2) as f32, -1.0f32);
    }

    /// `ModelCreeper` constructor.
    #[allow(unused_mut)]
    fn init_creeper(&mut self, var1: f32) {
        let var2: i32 = 4;
        self.insert_part("head".into(), ModelRenderer::new(0, 0));
        self["head"].add_box(-4.0f32, -8.0f32, -4.0f32, 8, 8, 8, var1);
        self["head"].set_rotation_point(0.0f32, var2 as f32, 0.0f32);
        self.insert_part("unusedCreeperHeadwear".into(), ModelRenderer::new(32, 0));
        self["unusedCreeperHeadwear"].add_box(-4.0f32, -8.0f32, -4.0f32, 8, 8, 8, var1 + 0.5f32);
        self["unusedCreeperHeadwear"].set_rotation_point(0.0f32, var2 as f32, 0.0f32);
        self.insert_part("body".into(), ModelRenderer::new(16, 16));
        self["body"].add_box(-4.0f32, 0.0f32, -2.0f32, 8, 12, 4, var1);
        self["body"].set_rotation_point(0.0f32, var2 as f32, 0.0f32);
        self.insert_part("leg1".into(), ModelRenderer::new(0, 16));
        self["leg1"].add_box(-2.0f32, 0.0f32, -2.0f32, 4, 6, 4, var1);
        self["leg1"].set_rotation_point(-2.0f32, (12 + var2) as f32, 4.0f32);
        self.insert_part("leg2".into(), ModelRenderer::new(0, 16));
        self["leg2"].add_box(-2.0f32, 0.0f32, -2.0f32, 4, 6, 4, var1);
        self["leg2"].set_rotation_point(2.0f32, (12 + var2) as f32, 4.0f32);
        self.insert_part("leg3".into(), ModelRenderer::new(0, 16));
        self["leg3"].add_box(-2.0f32, 0.0f32, -2.0f32, 4, 6, 4, var1);
        self["leg3"].set_rotation_point(-2.0f32, (12 + var2) as f32, -4.0f32);
        self.insert_part("leg4".into(), ModelRenderer::new(0, 16));
        self["leg4"].add_box(-2.0f32, 0.0f32, -2.0f32, 4, 6, 4, var1);
        self["leg4"].set_rotation_point(2.0f32, (12 + var2) as f32, -4.0f32);
    }

    /// `ModelSlime` constructor.
    #[allow(unused_mut)]
    fn init_slime(&mut self, var1: i32) {
        self.insert_part("slimeBodies".into(), ModelRenderer::new(0, var1));
        self["slimeBodies"].add_box(-4.0f32, 16.0f32, -4.0f32, 8, 8, 8, 0.0);
        if var1 > 0 {
            self.insert_part("slimeBodies".into(), ModelRenderer::new(0, var1));
            self["slimeBodies"].add_box(-3.0f32, 17.0f32, -3.0f32, 6, 6, 6, 0.0);
            self.insert_part("slimeRightEye".into(), ModelRenderer::new(32, 0));
            self["slimeRightEye"].add_box(-3.25f32, 18.0f32, -3.5f32, 2, 2, 2, 0.0);
            self.insert_part("slimeLeftEye".into(), ModelRenderer::new(32, 4));
            self["slimeLeftEye"].add_box(1.25f32, 18.0f32, -3.5f32, 2, 2, 2, 0.0);
            self.insert_part("slimeMouth".into(), ModelRenderer::new(32, 8));
            self["slimeMouth"].add_box(0.0f32, 21.0f32, -3.5f32, 1, 1, 1, 0.0);
        }
    }

    /// `ModelWolf` constructor.
    #[allow(unused_mut)]
    fn init_wolf(&mut self) {
        let mut var1: f32 = 0.0f32;
        let mut var2: f32 = 13.5f32;
        self.insert_part("wolfHeadMain".into(), ModelRenderer::new(0, 0));
        self["wolfHeadMain"].add_box(-3.0f32, -3.0f32, -2.0f32, 6, 6, 4, var1);
        self["wolfHeadMain"].set_rotation_point(-1.0f32, var2, -7.0f32);
        self.insert_part("wolfBody".into(), ModelRenderer::new(18, 14));
        self["wolfBody"].add_box(-4.0f32, -2.0f32, -3.0f32, 6, 9, 6, var1);
        self["wolfBody"].set_rotation_point(0.0f32, 14.0f32, 2.0f32);
        self.insert_part("wolfMane".into(), ModelRenderer::new(21, 0));
        self["wolfMane"].add_box(-4.0f32, -3.0f32, -3.0f32, 8, 6, 7, var1);
        self["wolfMane"].set_rotation_point(-1.0f32, 14.0f32, 2.0f32);
        self.insert_part("wolfLeg1".into(), ModelRenderer::new(0, 18));
        self["wolfLeg1"].add_box(-1.0f32, 0.0f32, -1.0f32, 2, 8, 2, var1);
        self["wolfLeg1"].set_rotation_point(-2.5f32, 16.0f32, 7.0f32);
        self.insert_part("wolfLeg2".into(), ModelRenderer::new(0, 18));
        self["wolfLeg2"].add_box(-1.0f32, 0.0f32, -1.0f32, 2, 8, 2, var1);
        self["wolfLeg2"].set_rotation_point(0.5f32, 16.0f32, 7.0f32);
        self.insert_part("wolfLeg3".into(), ModelRenderer::new(0, 18));
        self["wolfLeg3"].add_box(-1.0f32, 0.0f32, -1.0f32, 2, 8, 2, var1);
        self["wolfLeg3"].set_rotation_point(-2.5f32, 16.0f32, -4.0f32);
        self.insert_part("wolfLeg4".into(), ModelRenderer::new(0, 18));
        self["wolfLeg4"].add_box(-1.0f32, 0.0f32, -1.0f32, 2, 8, 2, var1);
        self["wolfLeg4"].set_rotation_point(0.5f32, 16.0f32, -4.0f32);
        self.insert_part("wolfTail".into(), ModelRenderer::new(9, 18));
        self["wolfTail"].add_box(-1.0f32, 0.0f32, -1.0f32, 2, 8, 2, var1);
        self["wolfTail"].set_rotation_point(-1.0f32, 12.0f32, 8.0f32);
        self["wolfHeadMain"].set_texture_offset(16, 14).add_box(-3.0, -5.0, 0.0, 2, 2, 1, var1);
        self["wolfHeadMain"].set_texture_offset(16, 14).add_box(1.0, -5.0, 0.0, 2, 2, 1, var1);
        self["wolfHeadMain"].set_texture_offset(0, 10).add_box(-1.5, 0.0, -5.0, 3, 3, 4, var1);
    }

    /// `ModelBiped.setRotationAngles`.
    #[allow(unused_variables, unused_mut)]
    fn pose_biped(&mut self, p: [f32; 6]) {
        let [var1, var2, var3, var4, var5, var6] = p;

        self["bipedHead"].rotate_angle_y = var4 / (180.0f32 / PI_F32);
        self["bipedHead"].rotate_angle_x = var5 / (180.0f32 / PI_F32);
        self["bipedHeadwear"].rotate_angle_y = self["bipedHead"].rotate_angle_y;
        self["bipedHeadwear"].rotate_angle_x = self["bipedHead"].rotate_angle_x;
        self["bipedRightArm"].rotate_angle_x =
            math::cos(var1 * 0.6662f32 + PI_F32) * 2.0f32 * var2 * 0.5f32;
        self["bipedLeftArm"].rotate_angle_x = math::cos(var1 * 0.6662f32) * 2.0f32 * var2 * 0.5f32;
        self["bipedRightArm"].rotate_angle_z = 0.0f32;
        self["bipedLeftArm"].rotate_angle_z = 0.0f32;
        self["bipedRightLeg"].rotate_angle_x = math::cos(var1 * 0.6662f32) * 1.4f32 * var2;
        self["bipedLeftLeg"].rotate_angle_x = math::cos(var1 * 0.6662f32 + PI_F32) * 1.4f32 * var2;
        self["bipedRightLeg"].rotate_angle_y = 0.0f32;
        self["bipedLeftLeg"].rotate_angle_y = 0.0f32;
        if self.is_riding {
            self["bipedRightArm"].rotate_angle_x += PI_F32 * -0.2f32;
            self["bipedLeftArm"].rotate_angle_x += PI_F32 * -0.2f32;
            self["bipedRightLeg"].rotate_angle_x = PI_F32 * -0.4f32;
            self["bipedLeftLeg"].rotate_angle_x = PI_F32 * -0.4f32;
            self["bipedRightLeg"].rotate_angle_y = PI_F32 * 0.1f32;
            self["bipedLeftLeg"].rotate_angle_y = PI_F32 * -0.1f32;
        }

        if self.held_left || self.held_left_pose != 0 {
            self["bipedLeftArm"].rotate_angle_x =
                self["bipedLeftArm"].rotate_angle_x * 0.5f32 - PI_F32 * 0.1f32 * if self.held_left_pose != 0 { self.held_left_pose as f32 } else { 1.0 };
        }

        if self.held_right || self.held_right_pose != 0 {
            self["bipedRightArm"].rotate_angle_x =
                self["bipedRightArm"].rotate_angle_x * 0.5f32 - PI_F32 * 0.1f32 * if self.held_right_pose != 0 { self.held_right_pose as f32 } else { 1.0 };
        }

        self["bipedRightArm"].rotate_angle_y = 0.0f32;
        self["bipedLeftArm"].rotate_angle_y = 0.0f32;
        if self.on_ground > -9990.0f32 {
            let mut var7: f32 = self.on_ground;
            self["bipedBody"].rotate_angle_y =
                math::sin(((var7 as f64).sqrt() as f32) * PI_F32 * 2.0f32) * 0.2f32;
            self["bipedRightArm"].rotation_point_z =
                math::sin(self["bipedBody"].rotate_angle_y) * 5.0f32;
            self["bipedRightArm"].rotation_point_x =
                -math::cos(self["bipedBody"].rotate_angle_y) * 5.0f32;
            self["bipedLeftArm"].rotation_point_z =
                -math::sin(self["bipedBody"].rotate_angle_y) * 5.0f32;
            self["bipedLeftArm"].rotation_point_x =
                math::cos(self["bipedBody"].rotate_angle_y) * 5.0f32;
            self["bipedRightArm"].rotate_angle_y += self["bipedBody"].rotate_angle_y;
            self["bipedLeftArm"].rotate_angle_y += self["bipedBody"].rotate_angle_y;
            self["bipedLeftArm"].rotate_angle_x += self["bipedBody"].rotate_angle_y;
            var7 = 1.0f32 - self.on_ground;
            var7 *= var7;
            var7 *= var7;
            var7 = 1.0f32 - var7;
            let mut var8: f32 = math::sin(var7 * PI_F32);
            let mut var9: f32 = math::sin(self.on_ground * PI_F32)
                * -(self["bipedHead"].rotate_angle_x - 0.7f32)
                * (12.0f32 / 16.0f32);
            self["bipedRightArm"].rotate_angle_x = (self["bipedRightArm"].rotate_angle_x as f64
                - (var8 as f64 * 1.2f64 + var9 as f64))
                as f32;
            self["bipedRightArm"].rotate_angle_y += self["bipedBody"].rotate_angle_y * 2.0f32;
            self["bipedRightArm"].rotate_angle_z = math::sin(self.on_ground * PI_F32) * -0.4f32;
        }

        if self.is_sneak {
            self["bipedBody"].rotate_angle_x = 0.5f32;
            self["bipedRightLeg"].rotate_angle_x -= 0.0f32;
            self["bipedLeftLeg"].rotate_angle_x -= 0.0f32;
            self["bipedRightArm"].rotate_angle_x += 0.4f32;
            self["bipedLeftArm"].rotate_angle_x += 0.4f32;
            self["bipedRightLeg"].rotation_point_z = 4.0f32;
            self["bipedLeftLeg"].rotation_point_z = 4.0f32;
            self["bipedRightLeg"].rotation_point_y = 9.0f32;
            self["bipedLeftLeg"].rotation_point_y = 9.0f32;
            self["bipedHead"].rotation_point_y = 1.0f32;
        } else {
            self["bipedBody"].rotate_angle_x = 0.0f32;
            self["bipedRightLeg"].rotation_point_z = 0.0f32;
            self["bipedLeftLeg"].rotation_point_z = 0.0f32;
            self["bipedRightLeg"].rotation_point_y = 12.0f32;
            self["bipedLeftLeg"].rotation_point_y = 12.0f32;
            self["bipedHead"].rotation_point_y = 0.0f32;
        }

        self["bipedRightArm"].rotate_angle_z += math::cos(var3 * 0.09f32) * 0.05f32 + 0.05f32;
        self["bipedLeftArm"].rotate_angle_z -= math::cos(var3 * 0.09f32) * 0.05f32 + 0.05f32;
        self["bipedRightArm"].rotate_angle_x += math::sin(var3 * 0.067f32) * 0.05f32;
        self["bipedLeftArm"].rotate_angle_x -= math::sin(var3 * 0.067f32) * 0.05f32;
        if self.aimed_bow { self.pose_skeleton(var3); }
    }

    /// `ModelZombie.setRotationAngles`.
    #[allow(unused_variables, unused_mut)]
    fn pose_zombie(&mut self, p: [f32; 6]) {
        let [var1, var2, var3, var4, var5, var6] = p;

        let mut var7: f32 = math::sin(self.on_ground * PI_F32);
        let mut var8: f32 =
            math::sin((1.0f32 - (1.0f32 - self.on_ground) * (1.0f32 - self.on_ground)) * PI_F32);
        self["bipedRightArm"].rotate_angle_z = 0.0f32;
        self["bipedLeftArm"].rotate_angle_z = 0.0f32;
        self["bipedRightArm"].rotate_angle_y = -(0.1f32 - var7 * 0.6f32);
        self["bipedLeftArm"].rotate_angle_y = 0.1f32 - var7 * 0.6f32;
        self["bipedRightArm"].rotate_angle_x = PI_F32 * -0.5f32;
        self["bipedLeftArm"].rotate_angle_x = PI_F32 * -0.5f32;
        self["bipedRightArm"].rotate_angle_x -= var7 * 1.2f32 - var8 * 0.4f32;
        self["bipedLeftArm"].rotate_angle_x -= var7 * 1.2f32 - var8 * 0.4f32;
        self["bipedRightArm"].rotate_angle_z += math::cos(var3 * 0.09f32) * 0.05f32 + 0.05f32;
        self["bipedLeftArm"].rotate_angle_z -= math::cos(var3 * 0.09f32) * 0.05f32 + 0.05f32;
        self["bipedRightArm"].rotate_angle_x += math::sin(var3 * 0.067f32) * 0.05f32;
        self["bipedLeftArm"].rotate_angle_x -= math::sin(var3 * 0.067f32) * 0.05f32;
    }

    /// `ModelQuadruped.setRotationAngles`.
    #[allow(unused_variables, unused_mut)]
    fn pose_quadruped(&mut self, p: [f32; 6]) {
        let [var1, var2, var3, var4, var5, var6] = p;

        self["head"].rotate_angle_x = var5 / (180.0f32 / PI_F32);
        self["head"].rotate_angle_y = var4 / (180.0f32 / PI_F32);
        self["body"].rotate_angle_x = PI_F32 * 0.5f32;
        self["leg1"].rotate_angle_x = math::cos(var1 * 0.6662f32) * 1.4f32 * var2;
        self["leg2"].rotate_angle_x = math::cos(var1 * 0.6662f32 + PI_F32) * 1.4f32 * var2;
        self["leg3"].rotate_angle_x = math::cos(var1 * 0.6662f32 + PI_F32) * 1.4f32 * var2;
        self["leg4"].rotate_angle_x = math::cos(var1 * 0.6662f32) * 1.4f32 * var2;
    }

    /// `ModelCow.setRotationAngles`.
    #[allow(unused_variables, unused_mut)]


    /// `ModelChicken.setRotationAngles`.
    #[allow(unused_variables, unused_mut)]
    fn pose_chicken(&mut self, p: [f32; 6]) {
        let [var1, var2, var3, var4, var5, var6] = p;

        self["head"].rotate_angle_x = -(var5 / (180.0f32 / PI_F32));
        self["head"].rotate_angle_y = var4 / (180.0f32 / PI_F32);
        self["bill"].rotate_angle_x = self["head"].rotate_angle_x;
        self["bill"].rotate_angle_y = self["head"].rotate_angle_y;
        self["chin"].rotate_angle_x = self["head"].rotate_angle_x;
        self["chin"].rotate_angle_y = self["head"].rotate_angle_y;
        self["body"].rotate_angle_x = PI_F32 * 0.5f32;
        self["rightLeg"].rotate_angle_x = math::cos(var1 * 0.6662f32) * 1.4f32 * var2;
        self["leftLeg"].rotate_angle_x = math::cos(var1 * 0.6662f32 + PI_F32) * 1.4f32 * var2;
        self["rightWing"].rotate_angle_z = var3;
        self["leftWing"].rotate_angle_z = -var3;
    }

    /// `ModelSpider.setRotationAngles`.
    #[allow(unused_variables, unused_mut)]
    fn pose_spider(&mut self, p: [f32; 6]) {
        let [var1, var2, var3, var4, var5, var6] = p;

        self["spiderHead"].rotate_angle_y = var4 / (180.0f32 / PI_F32);
        self["spiderHead"].rotate_angle_x = var5 / (180.0f32 / PI_F32);
        let mut var7: f32 = PI_F32 * 0.25f32;
        self["spiderLeg1"].rotate_angle_z = -var7;
        self["spiderLeg2"].rotate_angle_z = var7;
        self["spiderLeg3"].rotate_angle_z = -var7 * 0.74f32;
        self["spiderLeg4"].rotate_angle_z = var7 * 0.74f32;
        self["spiderLeg5"].rotate_angle_z = -var7 * 0.74f32;
        self["spiderLeg6"].rotate_angle_z = var7 * 0.74f32;
        self["spiderLeg7"].rotate_angle_z = -var7;
        self["spiderLeg8"].rotate_angle_z = var7;
        let mut var8: f32 = -0.0f32;
        let mut var9: f32 = PI_F32 * 0.125f32;
        self["spiderLeg1"].rotate_angle_y = var9 * 2.0f32 + var8;
        self["spiderLeg2"].rotate_angle_y = -var9 * 2.0f32 - var8;
        self["spiderLeg3"].rotate_angle_y = var9 * 1.0f32 + var8;
        self["spiderLeg4"].rotate_angle_y = -var9 * 1.0f32 - var8;
        self["spiderLeg5"].rotate_angle_y = -var9 * 1.0f32 + var8;
        self["spiderLeg6"].rotate_angle_y = var9 * 1.0f32 - var8;
        self["spiderLeg7"].rotate_angle_y = -var9 * 2.0f32 + var8;
        self["spiderLeg8"].rotate_angle_y = var9 * 2.0f32 - var8;
        let mut var10: f32 = -(math::cos(var1 * 0.6662f32 * 2.0f32 + 0.0f32) * 0.4f32) * var2;
        let mut var11: f32 = -(math::cos(var1 * 0.6662f32 * 2.0f32 + PI_F32) * 0.4f32) * var2;
        let mut var12: f32 =
            -(math::cos(var1 * 0.6662f32 * 2.0f32 + PI_F32 * 0.5f32) * 0.4f32) * var2;
        let mut var13: f32 =
            -(math::cos(var1 * 0.6662f32 * 2.0f32 + PI_F32 * 3.0f32 / 2.0f32) * 0.4f32) * var2;
        let mut var14: f32 = f32::abs(math::sin(var1 * 0.6662f32 + 0.0f32) * 0.4f32) * var2;
        let mut var15: f32 = f32::abs(math::sin(var1 * 0.6662f32 + PI_F32) * 0.4f32) * var2;
        let mut var16: f32 =
            f32::abs(math::sin(var1 * 0.6662f32 + PI_F32 * 0.5f32) * 0.4f32) * var2;
        let mut var17: f32 =
            f32::abs(math::sin(var1 * 0.6662f32 + PI_F32 * 3.0f32 / 2.0f32) * 0.4f32) * var2;
        self["spiderLeg1"].rotate_angle_y += var10;
        self["spiderLeg2"].rotate_angle_y += -var10;
        self["spiderLeg3"].rotate_angle_y += var11;
        self["spiderLeg4"].rotate_angle_y += -var11;
        self["spiderLeg5"].rotate_angle_y += var12;
        self["spiderLeg6"].rotate_angle_y += -var12;
        self["spiderLeg7"].rotate_angle_y += var13;
        self["spiderLeg8"].rotate_angle_y += -var13;
        self["spiderLeg1"].rotate_angle_z += var14;
        self["spiderLeg2"].rotate_angle_z += -var14;
        self["spiderLeg3"].rotate_angle_z += var15;
        self["spiderLeg4"].rotate_angle_z += -var15;
        self["spiderLeg5"].rotate_angle_z += var16;
        self["spiderLeg6"].rotate_angle_z += -var16;
        self["spiderLeg7"].rotate_angle_z += var17;
        self["spiderLeg8"].rotate_angle_z += -var17;
    }

    /// `ModelCreeper.setRotationAngles`.
    #[allow(unused_variables, unused_mut)]
    fn pose_creeper(&mut self, p: [f32; 6]) {
        let [var1, var2, var3, var4, var5, var6] = p;

        self["head"].rotate_angle_y = var4 / (180.0f32 / PI_F32);
        self["head"].rotate_angle_x = var5 / (180.0f32 / PI_F32);
        self["leg1"].rotate_angle_x = math::cos(var1 * 0.6662f32) * 1.4f32 * var2;
        self["leg2"].rotate_angle_x = math::cos(var1 * 0.6662f32 + PI_F32) * 1.4f32 * var2;
        self["leg3"].rotate_angle_x = math::cos(var1 * 0.6662f32 + PI_F32) * 1.4f32 * var2;
        self["leg4"].rotate_angle_x = math::cos(var1 * 0.6662f32) * 1.4f32 * var2;
    }

    /// `ModelWolf.setRotationAngles`.
    #[allow(unused_variables, unused_mut)]
    fn pose_wolf(&mut self, p: [f32; 6]) {
        let [var1, var2, var3, var4, var5, var6] = p;

        self["wolfHeadMain"].rotate_angle_x = var5 / (180.0f32 / PI_F32);
        self["wolfHeadMain"].rotate_angle_y = var4 / (180.0f32 / PI_F32);
        self["wolfTail"].rotate_angle_x = var3;
    }

    /// `ModelSquid` constructor: Math.sin/cos are evaluated as doubles.
    fn init_squid(&mut self) {
        let mut body = ModelRenderer::new(0, 0);
        body.add_box(-6.0, -8.0, -6.0, 12, 16, 12, 0.0);
        body.rotation_point_y += 8.0;
        self.insert_part("squidBody".into(), body);
        for i in 0..8 {
            let a = i as f64 * std::f64::consts::PI * 2.0 / 8.0;
            let mut part = ModelRenderer::new(48, 0);
            part.add_box(-1.0, 0.0, -1.0, 2, 18, 2, 0.0);
            part.rotation_point_x = a.cos() as f32 * 5.0;
            part.rotation_point_z = a.sin() as f32 * 5.0;
            part.rotation_point_y = 15.0;
            part.rotate_angle_y =
                (i as f64 * std::f64::consts::PI * -2.0 / 8.0 + std::f64::consts::PI * 0.5) as f32;
            self.insert_part(format!("squidTentacles[{i}]"), part);
        }
    }
    /// `ModelWolf.setLivingAnimations`: standing/sitting, tail wag, begging and wet shake.
    pub fn set_living_animations(&mut self, var2: f32, var3: f32, var4: f32, wolf: &WolfAnimation) {
        if self.kind != ModelKind::Wolf {
            return;
        }

        if wolf.angry {
            self["wolfTail"].rotate_angle_y = 0.0f32;
        } else {
            self["wolfTail"].rotate_angle_y = math::cos(var2 * 0.6662f32) * 1.4f32 * var3;
        }

        if wolf.sitting {
            self["wolfMane"].set_rotation_point(-1.0f32, 16.0f32, -3.0f32);
            self["wolfMane"].rotate_angle_x = PI_F32 * 0.4f32;
            self["wolfMane"].rotate_angle_y = 0.0f32;
            self["wolfBody"].set_rotation_point(0.0f32, 18.0f32, 0.0f32);
            self["wolfBody"].rotate_angle_x = PI_F32 * 0.25f32;
            self["wolfTail"].set_rotation_point(-1.0f32, 21.0f32, 6.0f32);
            self["wolfLeg1"].set_rotation_point(-2.5f32, 22.0f32, 2.0f32);
            self["wolfLeg1"].rotate_angle_x = PI_F32 * 3.0f32 / 2.0f32;
            self["wolfLeg2"].set_rotation_point(0.5f32, 22.0f32, 2.0f32);
            self["wolfLeg2"].rotate_angle_x = PI_F32 * 3.0f32 / 2.0f32;
            self["wolfLeg3"].rotate_angle_x = PI_F32 * 1.85f32;
            self["wolfLeg3"].set_rotation_point(-2.49f32, 17.0f32, -4.0f32);
            self["wolfLeg4"].rotate_angle_x = PI_F32 * 1.85f32;
            self["wolfLeg4"].set_rotation_point(0.51f32, 17.0f32, -4.0f32);
        } else {
            self["wolfBody"].set_rotation_point(0.0f32, 14.0f32, 2.0f32);
            self["wolfBody"].rotate_angle_x = PI_F32 * 0.5f32;
            self["wolfMane"].set_rotation_point(-1.0f32, 14.0f32, -3.0f32);
            self["wolfMane"].rotate_angle_x = self["wolfBody"].rotate_angle_x;
            self["wolfTail"].set_rotation_point(-1.0f32, 12.0f32, 8.0f32);
            self["wolfLeg1"].set_rotation_point(-2.5f32, 16.0f32, 7.0f32);
            self["wolfLeg2"].set_rotation_point(0.5f32, 16.0f32, 7.0f32);
            self["wolfLeg3"].set_rotation_point(-2.5f32, 16.0f32, -4.0f32);
            self["wolfLeg4"].set_rotation_point(0.5f32, 16.0f32, -4.0f32);
            self["wolfLeg1"].rotate_angle_x = math::cos(var2 * 0.6662f32) * 1.4f32 * var3;
            self["wolfLeg2"].rotate_angle_x = math::cos(var2 * 0.6662f32 + PI_F32) * 1.4f32 * var3;
            self["wolfLeg3"].rotate_angle_x = math::cos(var2 * 0.6662f32 + PI_F32) * 1.4f32 * var3;
            self["wolfLeg4"].rotate_angle_x = math::cos(var2 * 0.6662f32) * 1.4f32 * var3;
        }

        let var6: f32 = wolf.interested_angle(var4) + wolf.shake_angle(var4, 0.0f32);
        self["wolfHeadMain"].rotate_angle_z = var6;
        self["wolfMane"].rotate_angle_z = wolf.shake_angle(var4, -0.08f32);
        self["wolfBody"].rotate_angle_z = wolf.shake_angle(var4, -0.16f32);
        self["wolfTail"].rotate_angle_z = wolf.shake_angle(var4, -0.2f32);
    }
    /// Original model draw order (ears/cloak and creeper headwear are not body parts).
    pub fn render_parts(&self) -> Vec<(&ModelRenderer, bool)> {
        if matches!(self.kind, ModelKind::Minecart | ModelKind::Boat | ModelKind::Sign) {
            return self.registered.iter().map(|name| (&self.parts[name], false)).collect();
        }
        if matches!(self.kind, ModelKind::Dragon | ModelKind::Villager | ModelKind::SnowMan | ModelKind::Blaze | ModelKind::MagmaCube | ModelKind::EnderCrystal | ModelKind::Book) { return self.release_parts(); }
        let names: &[(&str, bool)] = match self.kind {
            ModelKind::Biped | ModelKind::Zombie | ModelKind::Skeleton | ModelKind::Enderman => &[
                ("bipedHead", false),
                ("bipedBody", false),
                ("bipedRightArm", false),
                ("bipedLeftArm", false),
                ("bipedRightLeg", false),
                ("bipedLeftLeg", false),
                ("bipedHeadwear", false),
            ],
            ModelKind::Quadruped
            | ModelKind::Pig
            | ModelKind::Sheep
            | ModelKind::SheepFur
            | ModelKind::Cow => &[
                ("head", false),
                ("body", false),
                ("leg1", false),
                ("leg2", false),
                ("leg3", false),
                ("leg4", false),
            ],
            ModelKind::Chicken => &[
                ("head", false),
                ("bill", false),
                ("chin", false),
                ("body", false),
                ("rightLeg", false),
                ("leftLeg", false),
                ("rightWing", false),
                ("leftWing", false),
            ],
            ModelKind::Spider => &[
                ("spiderHead", false),
                ("spiderNeck", false),
                ("spiderBody", false),
                ("spiderLeg1", false),
                ("spiderLeg2", false),
                ("spiderLeg3", false),
                ("spiderLeg4", false),
                ("spiderLeg5", false),
                ("spiderLeg6", false),
                ("spiderLeg7", false),
                ("spiderLeg8", false),
            ],
            ModelKind::Creeper => &[
                ("head", false),
                ("body", false),
                ("leg1", false),
                ("leg2", false),
                ("leg3", false),
                ("leg4", false),
            ],
            ModelKind::SlimeOuter | ModelKind::SlimeInner => &[
                ("slimeBodies", false),
                ("slimeRightEye", false),
                ("slimeLeftEye", false),
                ("slimeMouth", false),
            ],
            ModelKind::Wolf => &[
                ("wolfHeadMain", true),
                ("wolfBody", false),
                ("wolfLeg1", false),
                ("wolfLeg2", false),
                ("wolfLeg3", false),
                ("wolfLeg4", false),
                ("wolfTail", true),
                ("wolfMane", false),
            ],
            ModelKind::Chest | ModelKind::LargeChest => &[
                ("field_35405_a", false),
                ("field_35404_c", false),
                ("field_35403_b", false),
            ],
            ModelKind::Squid | ModelKind::Ghast | ModelKind::Silverfish
                | ModelKind::Minecart | ModelKind::Boat | ModelKind::Sign | ModelKind::Dragon | ModelKind::Villager | ModelKind::SnowMan | ModelKind::Blaze | ModelKind::MagmaCube | ModelKind::EnderCrystal | ModelKind::Book => &[],
        };
        let mut parts: Vec<_> = names
            .iter()
            .filter_map(|(n, r)| self.parts.get(*n).map(|p| (p, *r)))
            .collect();
        if self.kind == ModelKind::Squid {
            parts.push((&self["squidBody"], false));
            for i in 0..8 {
                parts.push((&self.parts[&format!("squidTentacles[{i}]")], false));
            }
        }
        if self.kind == ModelKind::Ghast {
            parts.push((&self["body"], false));
            for i in 0..9 {
                parts.push((&self.parts[&format!("tentacles[{i}]")], false));
            }
        }
        if self.kind == ModelKind::Silverfish {
            for i in 0..7 {
                parts.push((&self.parts[&format!("field_35400_a[{i}]")], false));
            }
            for i in 0..3 {
                parts.push((&self.parts[&format!("field_35398_b[{i}]")], false));
            }
        }
        parts
    }
}

/// Inputs read by `EntityWolf.getInterestedAngle`/`getShakeAngle`.
/// These retain previous/current values because their float operation order matters.
#[derive(Clone, Copy, Debug, Default)]
pub struct WolfAnimation {
    pub angry: bool,
    pub sitting: bool,
    pub interested: [f32; 2],
    pub shake: [f32; 2],
}
impl WolfAnimation {
    pub fn interested_angle(&self, partial: f32) -> f32 {
        (self.interested[0] + (self.interested[1] - self.interested[0]) * partial) * 0.15 * PI_F32
    }
    pub fn shake_angle(&self, partial: f32, offset: f32) -> f32 {
        let mut v = (self.shake[0] + (self.shake[1] - self.shake[0]) * partial + offset) / 1.8;
        if v < 0.0 {
            v = 0.0;
        } else if v > 1.0 {
            v = 1.0;
        }
        math::sin(v * PI_F32) * math::sin(v * PI_F32 * 11.0) * 0.15 * PI_F32
    }
}
