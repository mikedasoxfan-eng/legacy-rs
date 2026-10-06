//! Presentation of ordered World/IWorldAccess callbacks, isolated from simulation RNG.
use crate::{
    render::{
        blocks::Colorizers,
        particles::{EffectRenderer, Kind},
        weather::{Snapshot, Splash, WeatherVertex},
    },
    sound::SoundManager,
};
use worldgen::{world::events::WorldEvent, BiomeSource};

pub struct Effects {
    pub renderer: EffectRenderer,
    pub terrain: Snapshot,
    pub colors: Option<Colorizers>,
    pub fancy: bool,
    biomes: BiomeSource,
    pub collector: Option<worldgen::entity::Entity>,
}

#[cfg(test)]
mod release_tests {
    use super::*;

    fn event(code: i32, data: i32) -> WorldEvent {
        WorldEvent::Auxiliary { code, data, position: (0,64,0), random: worldgen::JavaRandom::new(42) }
    }
    fn effects() -> Effects {
        let mut effects=Effects::new(42);
        effects.renderer=EffectRenderer::new(42);
        effects
    }
    #[test]
    fn splash_potion_emits_eight_bottle_fragments_and_one_hundred_tinted_spells() {
        let mut fx=effects();
        fx.receive(vec![event(2002,5)],Snapshot::default(),[0.0,64.0,0.0],None);
        assert_eq!(fx.renderer.layers[2].len(),8);
        assert!(fx.renderer.layers[2].iter().all(|p|p.texture==140));
        assert_eq!(fx.renderer.layers[0].len(),100);
        assert!(fx.renderer.layers[0].iter().all(|p|p.kind==Kind::InstantSpell));
        let mut reduced=effects();
        reduced.renderer.particle_setting=2;
        reduced.receive(vec![event(2002,5)],Snapshot::default(),[0.0,64.0,0.0],None);
        assert_eq!(reduced.len(),0);
    }
    #[test]
    fn eye_ring_and_spawner_burst_preserve_ordered_particle_positions() {
        let mut fx=effects();
        fx.receive(vec![event(2003,0)],Snapshot::default(),[0.0,64.0,0.0],None);
        assert_eq!(fx.renderer.layers[2].len(),8);
        assert!(fx.renderer.layers[2].iter().all(|p|p.texture==155));
        let portal=&fx.renderer.layers[0];
        assert_eq!(portal.len(),80);
        assert_eq!(portal[0].pos,[5.5,63.6,0.5]);
        assert_eq!(portal[0].pos,portal[1].pos);
        assert_eq!(portal[0].motion,[-5.0,0.0,-0.0]);
        assert_eq!(portal[1].motion,[-7.0,0.0,-0.0]);
        let mut spawner=effects();
        spawner.receive(vec![event(2004,0)],Snapshot::default(),[0.0,64.0,0.0],None);
        assert_eq!(spawner.renderer.layers[0].len(),40);
        for pair in spawner.renderer.layers[0].chunks_exact(2) {
            assert_eq!(pair[0].kind,Kind::Smoke);
            assert_eq!(pair[1].kind,Kind::Flame);
            assert_eq!(pair[0].pos,pair[1].pos);
        }
    }
}
impl Effects {
    pub fn object_draws(
        state: &crate::render::entity::objects::ObjectState,
        camera: [f64; 3],
        view: [f32; 2],
        fancy: bool,
    ) -> crate::render::entity::fixed::Fixed {
        let mut draws = crate::render::entity::objects::render(state, camera, view);
        if !fancy
            && matches!(state.kind,crate::render::entity::objects::ObjectKind::Item {item,..} if item.id==18)
        {
            // Leaf tiles have U columns 4/5 in both oak and pine rows; both are in
            // the same float exponent range, so the one-tile translation is exact.
            for draw in &mut draws.draws {
                for vertex in &mut draw.vertices {
                    vertex.uv[0] += 1.0 / 16.0;
                }
            }
        }
        draws
    }
    pub fn record_name(event: &WorldEvent) -> Option<&str> {
        match event {
            WorldEvent::Record { name, .. } => name.as_deref(),
            WorldEvent::Auxiliary {
                code: 1005,
                data: 2256,
                ..
            } => Some("13"),
            WorldEvent::Auxiliary {
                code: 1005,
                data: 2257,
                ..
            } => Some("cat"),
            _ => None,
        }
    }
    pub fn new(seed: i64) -> Self {
        Self {
            renderer: EffectRenderer::new_runtime(seed),
            terrain: Snapshot::default(),
            colors: None,
            fancy: true,
            collector: None,
            biomes: BiomeSource::new(seed),
        }
    }
    pub fn change_world(&mut self, seed: i64) {
        self.renderer.clear_world(Snapshot::default());
        self.terrain = Snapshot::default();
        self.collector = None;
        self.biomes = BiomeSource::new(seed);
    }
    pub fn len(&self) -> usize {
        self.renderer.len()
    }
    pub fn spawn(&mut self, splashes: Vec<Splash>) {
        for s in splashes {
            let p = self.renderer.create(
                if s.smoke { Kind::Smoke } else { Kind::Rain },
                s.pos,
                [0.0; 3],
            );
            self.renderer.add(p);
        }
    }
    pub fn tick(&mut self, view: [f64; 3]) {
        let view = self
            .collector
            .as_ref()
            .map_or(view, |e| [e.pos.x, e.pos.y, e.pos.z]);
        self.renderer.cloud_player = self
            .collector
            .as_ref()
            .map(|p| [p.pos.x, p.pos.y, p.pos.z, p.bounding_box.min_y, p.motion.y]);
        self.renderer.tick(&self.terrain, view);
    }
    pub fn vertices(
        &mut self,
        cam: [f64; 3],
        yaw: f32,
        pitch: f32,
        partial: f32,
        lightmap: &[u32; 256],
    ) -> [Vec<WeatherVertex>; 3] {
        // b1.8.1 EntityFX: Tessellator brightness from func_35115_a, drawn through the lightmap.
        self.renderer
            .lightmapped_vertices(&self.terrain, cam, yaw, pitch, partial)
            .map(|layer| {
                layer
                    .into_iter()
                    .map(|(mut v, light)| {
                        let l = crate::render::lightmap::sample(lightmap, light as u32);
                        for i in 0..3 {
                            v.color[i] *= l[i];
                        }
                        v
                    })
                    .collect()
            })
    }
    pub fn pickup(&mut self, source: Box<worldgen::entity::Entity>, target: i32) {
        let pos = [source.pos.x, source.pos.y, source.pos.z];
        let velocity = [source.motion.x, source.motion.y, source.motion.z];
        let mut p = self.renderer.create(Kind::Pickup, pos, velocity);
        p.pickup = Some(source);
        p.target = target;
        self.renderer.add(p);
    }
    /// EntityPickupFX's squared three-tick interpolation, before RenderManager's float cast.
    pub fn pickup_position(
        source: [f64; 3],
        last: [f64; 3],
        now: [f64; 3],
        age: i32,
        partial: f32,
    ) -> [f64; 3] {
        let mut t = (age as f32 + partial) / 3.0;
        t *= t;
        std::array::from_fn(|i| {
            let target = last[i]
                + (now[i] - last[i]) * partial as f64
                + if i == 1 { -0.5f32 as f64 } else { 0.0 };
            source[i] + (target - source[i]) * t as f64
        })
    }
    pub fn lit_batches(
        &mut self,
        cam: [f64; 3],
        view: [f32; 2],
        partial: f32,
        fancy: bool,
    ) -> Vec<crate::render::entity::living::Batch> {
        use crate::render::entity::{
            bridge,
            living::{Batch, EntityVertex, Pass},
        };
        let mut batches = Vec::new();
        // EntityRenderer calls the lit layer before renderParticles updates interpPos.
        self.renderer.render_lit_particles(&self.terrain, view[0], view[1], partial, |p, origin, rotation, _light| {
            if matches!(p.kind, Kind::Footstep | Kind::LargeExplode) {
                let vs = p.vertices(&self.terrain, origin, rotation, partial);
                if vs.is_empty() { return; }
                let vertices = [0, 1, 2, 0, 2, 3]
                    .map(|i| EntityVertex {
                        pos: vs[i].pos,
                        uv: vs[i].uv,
                        color: vs[i].color,
                    })
                    .to_vec();
                batches.push(Batch {
                    texture: Some(if p.kind == Kind::LargeExplode { "misc/explosion.png" } else { "misc/footprint.png" }),
                    pass: Pass::Alpha,
                    vertices,
                });
            } else if let (Some(source), Some(target)) = (&p.pickup, &self.collector) {
                let worldpos = Self::pickup_position(
                    [source.pos.x, source.pos.y, source.pos.z],
                    [
                        target.last_tick_pos.x,
                        target.last_tick_pos.y,
                        target.last_tick_pos.z,
                    ],
                    [target.pos.x, target.pos.y, target.pos.z],
                    p.timer,
                    partial,
                );
                let relative = std::array::from_fn(|i| (worldpos[i] - origin[i]) as f32 as f64);
                let color = 1.0;
                if let Some(mut state) = bridge::object_state(source, partial, color, [color; 7]) {
                    state.position = relative;
                    let g = Self::object_draws(&state, [0.0; 3], view, fancy);
                    batches.extend(g.draws.iter().map(|d| d.batch()));
                }
                use crate::render::entity::effects;
                if fancy {
                    let size = effects::radius(source) as f64;
                    let pos = bridge::position(source, partial);
                    let y = pos[1] + (source.height / 2.0) as f64;
                    let fl = worldgen::math::floor_double;
                    let mut surfaces = Vec::new();
                    let f = 1.0f32 - 3.0 / 15.0;
                    let min_light = (1.0 - f) / (f * 3.0 + 1.0) * 0.95 + 0.05;
                    for x in fl(pos[0] - size)..=fl(pos[0] + size) {
                        for yy in fl(y - size)..=fl(y) {
                            for z in fl(pos[2] - size)..=fl(pos[2] + size) {
                                if let Some(c) = self.terrain.column(x, z) {
                                    if (1..128).contains(&yy) {
                                        let block = c.blocks[(yy - 1) as usize];
                                        let brightness = c.brightness[yy as usize];
                                        if block > 0 && brightness > min_light {
                                            surfaces.push(effects::Surface {
                                                xyz: [x, yy, z],
                                                block,
                                                brightness,
                                            });
                                        }
                                    }
                                }
                            }
                        }
                    }
                    let g = effects::shadow_at(source, &surfaces, partial, cam, relative);
                    batches.extend(g.draws.iter().map(|d| d.batch()));
                }
                let g = effects::fire_at(source, relative, view[0]);
                batches.extend(g.draws.iter().map(|d| d.batch()));
            }
        });
        batches
    }
    pub(crate) fn tint(&mut self, block: u8, meta: i32, p: [i32; 3]) -> u32 {
        let Some(c) = &self.colors else {
            return 0xffffff;
        };
        if block == 18 && meta & 1 != 0 {
            return 6396257;
        }
        if block == 18 && meta & 2 != 0 {
            return 8431445;
        }
        let mut x = p[0];
        let mut z = p[2];
        if block == 31 && meta != 0 {
            let mut v = x
                .wrapping_mul(3129871)
                .wrapping_add(z.wrapping_mul(6129781))
                .wrapping_add(p[1]) as i64;
            v = v
                .wrapping_mul(v)
                .wrapping_mul(42317861)
                .wrapping_add(v.wrapping_mul(11));
            x = (x as i64 + ((v >> 14 & 31) - 16)) as i32;
            z = (z as i64 + ((v >> 24 & 31) - 16)) as i32;
        }
        let (t, h) = (self.biomes.raw_temperature(x, z), self.biomes.raw_rainfall(x, z));
        match block {
            18 => c.foliage(t, h),
            31 if meta != 0 => c.grass(t, h),
            _ => 0xffffff,
        }
    }
    pub(crate) fn fragments(
        &mut self,
        block: u8,
        meta: i32,
        p: [i32; 3],
        hit: Option<(i32, [f64; 6])>,
    ) {
        let first = self.renderer.layers[1].len();
        if let Some((side, bounds)) = hit {
            self.renderer.hit(p, side, block, meta, bounds);
        } else {
            self.renderer.destroy(p, block, meta);
        }
        let tint = self.tint(block, meta, p);
        // Grass's top texture is deliberately untinted in EntityDiggingFX.func_4041_a.
        let count = if hit.is_some() { 1 } else { 64 };
        let start = first.min(self.renderer.layers[1].len().saturating_sub(count));
        for f in &mut self.renderer.layers[1][start..] {
            if block == 18 && !self.fancy {
                f.texture += 1;
            }
            for i in 0..3 {
                f.color[i] *= ((tint >> ((2 - i) * 8) & 255) as f32) / 255.0;
            }
        }
    }
    pub fn receive(
        &mut self,
        events: Vec<WorldEvent>,
        terrain: Snapshot,
        view: [f64; 3],
        mut snd: Option<&mut SoundManager>,
    ) {
        self.terrain = terrain;
        self.renderer.creation_terrain = self.terrain.clone();
        let view = self
            .collector
            .as_ref()
            .map_or(view, |e| [e.pos.x, e.pos.y, e.pos.z]);
        for e in events {
            match e {
                WorldEvent::Sound {
                    name: Some(n),
                    position,
                    volume,
                    pitch,
                } => {
                    if let Some(s) = snd.as_deref_mut() {
                        crate::app::play_world_sound(
                            s,
                            glam::DVec3::from_array(view),
                            &n,
                            [position.x, position.y, position.z],
                            volume,
                            pitch,
                        );
                    }
                }
                WorldEvent::Sound { name: None, .. } => {}
                WorldEvent::DirectSound {
                    name,
                    position,
                    volume,
                    pitch,
                } => {
                    if let Some(s) = snd.as_deref_mut() {
                        if let Some(p) = position {
                            s.play_sound(
                                &name,
                                [p.x as f32, p.y as f32, p.z as f32],
                                volume,
                                pitch,
                            );
                        } else {
                            s.play_sound_fx(&name, volume, pitch);
                        }
                    }
                }
                WorldEvent::Particle {
                    name,
                    position,
                    velocity,
                } => self.renderer.spawn(
                    &name,
                    [position.x, position.y, position.z],
                    [velocity.x, velocity.y, velocity.z],
                    view,
                ),
                WorldEvent::Record {
                    name,
                    position: (x, y, z),
                } => {
                    if let Some(s) = snd.as_deref_mut() {
                        s.play_streaming(name.as_deref(), [x as f32, y as f32, z as f32], 1.0, 1.0);
                    }
                }
                WorldEvent::BlockHit {
                    position: (x, y, z),
                    side,
                    block,
                    metadata,
                    bounds,
                } => self.fragments(block, metadata, [x, y, z], Some((side, bounds))),
                WorldEvent::Auxiliary {
                    code,
                    position: (x, y, z),
                    data,
                    random,
                } => {
                    // RenderGlobal reads World.rand. Replay a non-consuming snapshot so the
                    // original simulation fingerprints remain independent of presentation.
                    let mut r = random;
                    let p = [x as f64, y as f64, z as f64];
                    let center = p.map(|v| v + 0.5);
                    let sound = match code {
                        1000 => Some(("random.click", p, 1.0, 1.0)),
                        1001 => Some(("random.click", p, 1.0, 1.2)),
                        1002 => Some(("random.bow", p, 1.0, 1.2)),
                        1003 => Some((
                            if self.renderer.math_random.next_double() < 0.5 {
                                "random.door_open"
                            } else {
                                "random.door_close"
                            },
                            center,
                            1.0,
                            r.next_float() * 0.1 + 0.9,
                        )),
                        1004 => Some((
                            "random.fizz",
                            [
                                (x as f32 + 0.5) as f64,
                                (y as f32 + 0.5) as f64,
                                (z as f32 + 0.5) as f64,
                            ],
                            0.5,
                            2.6 + (r.next_float() - r.next_float()) * 0.8,
                        )),
                        1007..=1009 => Some((
                            if code == 1007 { "mob.ghast.charge" } else { "mob.ghast.fireball" },
                            center, if code == 1009 {1.0} else {10.0},
                            (r.next_float()-r.next_float())*0.2+1.0,
                        )),
                        _ => None,
                    };
                    if let (Some(s), Some((n, p, v, f))) = (snd.as_deref_mut(), sound) {
                        crate::app::play_world_sound(s, glam::DVec3::from_array(view), n, p, v, f);
                    }
                    if code == 1005 {
                        if let Some(s) = snd.as_deref_mut() {
                            s.play_streaming(
                                match data {
                                    2256 => Some("13"),
                                    2257 => Some("cat"),
                                    _ => None,
                                },
                                [x as f32, y as f32, z as f32],
                                1.0,
                                1.0,
                            );
                        }
                    }
                    if code == 2000 {
                        let dx = (data % 3 - 1) as f64;
                        let dz = (data / 3 % 3 - 1) as f64;
                        let origin = [p[0] + dx * 0.6 + 0.5, p[1] + 0.5, p[2] + dz * 0.6 + 0.5];
                        for _ in 0..10 {
                            let speed = r.next_double() * 0.2 + 0.01;
                            let pos = [
                                origin[0] + dx * 0.01 + (r.next_double() - 0.5) * dz * 0.5,
                                origin[1] + (r.next_double() - 0.5) * 0.5,
                                origin[2] + dz * 0.01 + (r.next_double() - 0.5) * dx * 0.5,
                            ];
                            let velocity = [
                                dx * speed + r.next_gaussian() * 0.01,
                                -0.03 + r.next_gaussian() * 0.01,
                                dz * speed + r.next_gaussian() * 0.01,
                            ];
                            self.renderer.try_spawn_return("smoke", pos, velocity, view, &mut r).expect("smoke factory");
                        }
                    }
                    if code == 2002 || code == 2003 {
                        let origin = if code == 2003 { [p[0]+0.5,p[1],p[2]+0.5] } else {p};
                        let icon = if code == 2002 {"iconcrack_373"} else {"iconcrack_381"};
                        for _ in 0..8 {
                            let velocity = [r.next_gaussian()*0.15,r.next_double()*0.2,r.next_gaussian()*0.15];
                            self.renderer.try_spawn_return(icon,origin,velocity,view,&mut r).expect("item particle");
                        }
                        if code == 2002 {
                            let (color,instant)=crate::render::potion_color::properties(data);
                            let rgb=[(color>>16&255) as f32/255.0,(color>>8&255) as f32/255.0,(color&255) as f32/255.0];
                            let name=if instant {"instantSpell"} else {"spell"};
                            for _ in 0..100 {
                                let radius=r.next_double()*4.0;
                                let angle=r.next_double()*std::f64::consts::PI*2.0;
                                let dx=angle.cos()*radius;
                                let dy=0.01+r.next_double()*0.5;
                                let dz=angle.sin()*radius;
                                if let Some((layer,index))=self.renderer.try_spawn_return(name,
                                    [p[0]+dx*0.1,p[1]+0.3,p[2]+dz*0.1],[dx,dy,dz],view,&mut r).expect("spell factory") {
                                    let tint=12.0f32/16.0+r.next_float()*0.25;
                                    let particle=&mut self.renderer.layers[layer][index];
                                    particle.color=rgb.map(|v|v*tint);
                                    particle.multiply_velocity(radius as f32);
                                }
                            }
                            let pitch=r.next_float()*0.1+0.9;
                            if let Some(s)=snd.as_deref_mut() {
                                crate::app::play_world_sound(s,glam::DVec3::from_array(view),"random.glass",center,1.0,pitch);
                            }
                        } else {
                            let mut angle=0.0f64;
                            while angle<std::f64::consts::PI*2.0 {
                                let pos=[origin[0]+angle.cos()*5.0,origin[1]-0.4,origin[2]+angle.sin()*5.0];
                                for speed in [-5.0,-7.0] {
                                    self.renderer.try_spawn_return("portal",pos,[angle.cos()*speed,0.0,angle.sin()*speed],view,&mut r).expect("portal factory");
                                }
                                angle+=std::f64::consts::PI*0.05;
                            }
                        }
                    }
                    if code == 2004 {
                        for _ in 0..20 {
                            let pos=std::array::from_fn(|i|p[i]+0.5+(r.next_float() as f64-0.5)*2.0);
                            self.renderer.try_spawn_return("smoke",pos,[0.0;3],view,&mut r).expect("smoke factory");
                            self.renderer.try_spawn_return("flame",pos,[0.0;3],view,&mut r).expect("flame factory");
                        }
                    }
                    if code == 2001 {
                        let block = (data & 255) as u8;
                        if block > 0 {
                            let (n, v, f) = worldgen::world::events::break_sound(block);
                            if let Some(s) = snd.as_deref_mut() {
                                s.play_sound(
                                    n,
                                    [x as f32 + 0.5, y as f32 + 0.5, z as f32 + 0.5],
                                    (v + 1.0) / 2.0,
                                    f * 0.8,
                                );
                            }
                        }
                        self.fragments(block, data >> 8 & 255, [x, y, z], None);
                    }
                }
            }
        }
    }
}
