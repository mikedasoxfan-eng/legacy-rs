//! Read-only rendering snapshot. All ray queries mutate only this private mirror,
//! never the simulation's singleton block bounds, chunk lists or random streams.
use super::camera;
use std::collections::BTreeMap;
use worldgen::{
    entity::{AxisAlignedBB, Entity},
    world::{chunk::Chunk, Dimension},
    World,
};

pub struct Snapshot {
    spawn: (i32,i32,i32),
    chunks: Vec<Chunk>,
    entities: BTreeMap<i32, Entity>,
    bounds: [AxisAlignedBB; 256],
    sky_sub: i32,
    dimension: Dimension,
    leaves_opaque: bool,
    map: Option<super::map::Data>,
}
impl Snapshot {
    pub fn capture(w: &World, e: &Entity) -> Self {
        let cx = worldgen::math::floor_double(e.pos.x) >> 4;
        let cz = worldgen::math::floor_double(e.pos.z) >> 4;
        let mut chunks = Vec::new();
        let mut entities = BTreeMap::new();
        for x in cx - 1..=cx + 1 {
            for z in cz - 1..=cz + 1 {
                if let Some(c) = w.chunks.get(&(x, z)) {
                    let mut copy = Chunk::new(x, z, c.blocks.clone());
                    copy.data = c.data.clone();
                    copy.sky_light = c.sky_light.clone();
                    copy.block_light = c.block_light.clone();
                    copy.height_map = c.height_map;
                    copy.entity_lists = c.entity_lists.clone();
                    for id in c.entity_lists.iter().flatten() {
                        if let Some(e) = w.entities.get(id) {
                            entities.insert(*id, e.clone());
                        }
                    }
                    chunks.push(copy);
                }
            }
        }
        Self {
            spawn: w.spawn,
            chunks,
            entities,
            bounds: w.block_bounds,
            sky_sub: w.skylight_subtracted,
            dimension: w.dimension,
            leaves_opaque: w.leaves_opaque,
            map: super::map::saved(w, e),
        }
    }
    pub fn apply(self, w: &mut World) -> Option<super::map::Data> {
        w.spawn = self.spawn;
        w.chunks.clear();
        for c in self.chunks {
            w.chunks.insert((c.x, c.z), Box::new(c));
        }
        w.entities = self.entities;
        w.block_bounds = self.bounds;
        w.skylight_subtracted = self.sky_sub;
        w.dimension = self.dimension;
        w.leaves_opaque = self.leaves_opaque;
        w.light_brightness_table = self.dimension.light_brightness_table();
        self.map
    }
}
pub struct Frame {
    pub ops: Vec<super::entity::fixed::Op>,
    pub settings: camera::Settings,
    pub medium: camera::Medium,
    pub fov: f32,
    pub nether: bool,
    pub hand: super::entity::fixed::Fixed,
    pub overlays: super::entity::fixed::Fixed,
    pub player: super::entity::fixed::Fixed,
    pub outline: super::entity::fixed::Fixed,
    pub damage: super::entity::fixed::Fixed,
    pub map_image: Option<super::gpu::Image>,
    /// b1.8.1: `World.func_35451_b` at the player's block, for the held item and hand.
    pub hand_light: u32,
    /// b1.8.1 void-fog factor (`camera::void_factor`) of the view entity.
    pub void_fog: f64,
}
pub struct View {
    pub entity: Entity,
    pub hud: worldgen::entity::player::PlayerHud,
    pub world: World,
    pub model: super::entity::player::Renderer,
    pub equip: super::first_person::Equip,
    maps: BTreeMap<String, super::map::Data>,
    /// `[field_35813_N, field_35812_M]`: the sprint/bow FOV zoom, eased each tick.
    fov_multiplier: [f32; 2],
    /// The FOV option (`field_35379_L`).
    pub fov_option: f32,
}
impl View {
    pub fn new(
        entity: Entity,
        hud: worldgen::entity::player::PlayerHud,
        snapshot: Snapshot,
    ) -> Self {
        let mut world = World::new(0, 0);
        let map = snapshot.apply(&mut world);
        let maps = map.into_iter().map(|m| (m.label.clone(), m)).collect();
        let mut equip = super::first_person::Equip::default();
        equip.update(hud.held_item, hud.inventory.current_item);
        Self {
            entity,
            hud,
            world,
            equip,
            maps,
            model: Default::default(),
            fov_multiplier: [1.0, 1.0],
            fov_option: 0.0,
        }
    }
    pub fn update(
        &mut self,
        entity: Entity,
        hud: worldgen::entity::player::PlayerHud,
        snapshot: Snapshot,
    ) {
        // EntityRenderer.func_35809_c: previous = current; current eases half way to the target.
        self.fov_multiplier[0] = self.fov_multiplier[1];
        self.fov_multiplier[1] += (hud.fov_multiplier - self.fov_multiplier[1]) * 0.5;
        self.equip.update(hud.held_item, hud.inventory.current_item);
        self.entity = entity;
        self.hud = hud;
        // Keep the old map while its equip animation lowers it out of view.
        if let Some(map) = snapshot.apply(&mut self.world) {
            self.maps.insert(map.label.clone(), map);
        }
    }
    pub fn bed(&mut self) -> Option<i32> {
        let e = &self.entity;
        let (x, y, z) = (
            worldgen::math::floor_double(e.pos.x),
            worldgen::math::floor_double(e.pos.y),
            worldgen::math::floor_double(e.pos.z),
        );
        (self.world.get_block_id(x, y, z) == 26).then(|| self.world.get_block_metadata(x, y, z) & 3)
    }
    pub fn camera(
        &mut self,
        t: f32,
        look: [f32; 4],
        s: camera::Settings,
    ) -> (Vec<super::entity::fixed::Op>, f32, camera::Medium) {
        let mut e = self.entity.clone();
        e.yaw = look[0];
        e.pitch = look[1];
        e.prev_yaw = look[2];
        e.prev_pitch = look[3];
        let medium = camera::medium(&e, &mut self.world);
        let d = if s.third_person && !s.debug_camera {
            camera::third_distance(&e, t, &mut self.world)
        } else {
            4.0
        };
        let bed = self.bed();
        (
            camera::transform(&e, t, s, d, bed),
            camera::fov(
                &e,
                medium == camera::Medium::Water,
                t,
                &camera::Settings { fov_option: self.fov_option, fov_zoom: Some(self.fov_multiplier), ..s },
                true,
            ),
            medium,
        )
    }
    pub fn frame(
        &mut self,
        t: f32,
        look: [f32; 4],
        settings: camera::Settings,
        hide: bool,
        camera: [f64; 3],
        col: &super::blocks::Colorizers,
        fancy: bool,
    ) -> Frame {
        let (ops, fov, medium) = self.camera(t, look, settings);
        let mut e = self.entity.clone();
        e.yaw = look[0];
        e.pitch = look[1];
        e.prev_yaw = look[2];
        e.prev_pitch = look[3];
        let light = self.world.get_block_light_value(
            worldgen::math::floor_double(e.pos.x),
            worldgen::math::floor_double(e.pos.y),
            worldgen::math::floor_double(e.pos.z),
        );
        let brightness = self.world.light_brightness_table[light.clamp(0, 15) as usize];
        let hand_light = self.world.packed_light181(
            worldgen::math::floor_double(e.pos.x),
            worldgen::math::floor_double(e.pos.y),
            worldgen::math::floor_double(e.pos.z),
            0,
        ) as u32;
        let void_fog = camera::void_factor(
            e.packed_brightness(&mut self.world, t) as u32,
            e.last_tick_pos.y + (e.pos.y - e.last_tick_pos.y) * t as f64,
        );
        let fallback = self
            .equip
            .item
            .filter(|i| i.id == 358)
            .map(|i| super::map::Data {
                label: format!("map_{}", i.damage),
                ..Default::default()
            });
        let map = fallback
            .as_ref()
            .map(|m| self.maps.get(&m.label).unwrap_or(m));
        let hand = super::first_person::render_with_map(
            &e,
            t,
            &self.equip,
            brightness,
            self.hud.brightness,
            settings,
            hide,
            map,
        );
        let map_image = map.map(|m| super::map::image(m, settings.anaglyph));
        let opaque = super::first_person::opaque_textures(&e, &mut self.world);
        let overlays = super::first_person::overlays(
            &e,
            medium == camera::Medium::Water,
            &opaque,
            self.hud.brightness,
            settings,
        );
        let (outline, damage) = if hide {
            (
                super::entity::fixed::Fixed::new(1.0),
                super::entity::fixed::Fixed::new(1.0),
            )
        } else {
            super::targeting::frame(&e, &mut self.world, t, camera, col, fancy)
        };
        let bed = e
            .player_state()
            .and_then(|p| p.bed)
            .map_or(0, |(x, y, z)| self.world.get_block_metadata(x, y, z) & 3);
        let player = if settings.third_person || e.player_state().is_some_and(|p| p.sleeping) {
            self.model.render(&e, t, camera, self.hud.brightness, bed)
        } else {
            super::entity::fixed::Fixed::new(1.0)
        };
        Frame {
            ops,
            settings,
            medium,
            fov,
            nether: self.world.dimension.is_hell_world(),
            hand,
            overlays,
            player,
            outline,
            damage,
            map_image,
            hand_light,
            void_fog,
        }
    }
}
