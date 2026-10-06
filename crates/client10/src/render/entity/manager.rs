//! RenderManager's registry, camera cache and ordered virtual dispatch.
//! Resource ownership stays with the backend; `T` is its resource handle type.
use std::collections::HashMap;
use worldgen::entity::{Entity, EntityKind};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    NullPointer,
    Callback,
}

#[derive(Clone, Copy, Debug)]
pub struct Renderer {
    pub entity_class: &'static str,
    pub renderer_class: &'static str,
    pub shadow_size: f32,
    pub shadow_opacity: f32,
}

// Each entry owns a distinct renderer, including the two RenderSpider objects.
pub const RENDERERS: &[Renderer] = &[
    renderer("EntitySpider", "RenderSpider", 1.0),
    renderer("EntityCaveSpider", "RenderSpider", 1.0),
    renderer("EntityPig", "RenderPig", 0.7),
    renderer("EntitySheep", "RenderSheep", 0.7),
    renderer("EntityCow", "RenderCow", 0.7),
    renderer("EntityWolf", "RenderWolf", 0.5),
    renderer("EntityChicken", "RenderChicken", 0.3),
    renderer("EntitySilverfish", "RenderSilverfish", 0.3),
    renderer("EntityCreeper", "RenderCreeper", 0.5),
    renderer("EntityEnderman", "RenderEnderman", 0.5),
    renderer("EntitySkeleton", "RenderBiped", 0.5),
    renderer("EntityZombie", "RenderBiped", 0.5),
    renderer("EntitySlime", "RenderSlime", 0.25),
    renderer("EntityPlayer", "RenderPlayer", 0.5),
    renderer("EntityGiantZombie", "RenderGiantZombie", 3.0),
    renderer("EntityGhast", "RenderGhast", 0.5),
    renderer("EntitySquid", "RenderSquid", 0.7),
    renderer("EntityLiving", "RenderLiving", 0.5),
    renderer("Entity", "RenderEntity", 0.0),
    renderer("EntityPainting", "RenderPainting", 0.0),
    renderer("EntityArrow", "RenderArrow", 0.0),
    renderer("EntitySnowball", "RenderSnowball", 0.0),
    renderer("EntityEgg", "RenderSnowball", 0.0),
    renderer("EntityFireball", "RenderFireball", 0.0),
    Renderer {
        entity_class: "EntityItem",
        renderer_class: "RenderItem",
        shadow_size: 0.15,
        shadow_opacity: 12.0 / 16.0,
    },
    Renderer {
        entity_class: "EntityXPOrb",
        renderer_class: "RenderXPOrb",
        shadow_size: 0.15,
        shadow_opacity: 12.0 / 16.0,
    },
    renderer("EntityTNTPrimed", "RenderTNTPrimed", 0.5),
    renderer("EntityFallingSand", "RenderFallingSand", 0.5),
    renderer("EntityMinecart", "RenderMinecart", 0.5),
    renderer("EntityBoat", "RenderBoat", 0.5),
    renderer("EntityFishHook", "RenderFish", 0.0),
    renderer("EntityLightningBolt", "RenderLightningBolt", 0.0),
    renderer("EntityMooshroom", "RenderMooshroom", 0.7),
    renderer("EntityVillager", "RenderVillager", 0.5),
    renderer("EntitySnowman", "RenderSnowMan", 0.5),
    renderer("EntityBlaze", "RenderBlaze", 0.5),
    renderer("EntityMagmaCube", "RenderMagmaCube", 0.25),
    renderer("EntityDragon", "RenderDragon", 0.5),
    renderer("EntityEnderCrystal", "RenderEnderCrystal", 0.5),
    renderer("EntityEnderPearl", "RenderSnowball", 0.0),
    renderer("EntityEnderEye", "RenderSnowball", 0.0),
    renderer("EntityPotion", "RenderSnowball", 0.0),
    renderer("EntitySmallFireball", "RenderFireball", 0.0),
];
const fn renderer(
    entity_class: &'static str,
    renderer_class: &'static str,
    shadow_size: f32,
) -> Renderer {
    Renderer {
        entity_class,
        renderer_class,
        shadow_size,
        shadow_opacity: 1.0,
    }
}

pub fn class(e: &Entity) -> &'static str {
    match &e.kind {
        EntityKind::DragonPart {..} => "Entity",
        EntityKind::Snowman => "EntitySnowman",
        EntityKind::Giant => "EntityGiantZombie",
        EntityKind::Tnt { .. } => "EntityTNTPrimed",
        EntityKind::Egg(_) => "EntityEgg",
        EntityKind::Fish(_) => "EntityFishHook",
        EntityKind::PigZombie { .. } => "EntityPigZombie",
        // The simulation names the mooshroom by its save id; its class is EntityMooshroom.
        EntityKind::Mooshroom => "EntityMooshroom",
        _ => {
            RENDERERS
                .iter()
                .find(|r| r.entity_class.strip_prefix("Entity") == Some(e.kind.name()))
                .expect("registered simulation entity renderer")
                .entity_class
        }
    }
}
pub fn renderer_for(e: &Entity) -> &'static Renderer {
    let name = class(e);
    // PigZombie inherits EntityZombie's renderer; every other simulation kind
    // has an exact-class constructor entry.
    RENDERERS
        .iter()
        .find(|r| r.entity_class == name)
        .unwrap_or(&RENDERERS[11])
}

#[derive(Clone, Debug, Default)]
pub struct Context<T> {
    pub world: Option<T>,
    pub engine: Option<T>,
    pub font: Option<T>,
    pub player: Option<T>,
    pub options: Option<T>,
}
#[derive(Clone, Copy, Debug)]
pub struct CameraState {
    pub last: [f64; 3],
    pub position: [f64; 3],
    pub previous_rotation: [f32; 2],
    pub rotation: [f32; 2],
    pub sleeping: bool,
}
impl From<&Entity> for CameraState {
    fn from(e: &Entity) -> Self {
        Self {
            last: [e.last_tick_pos.x, e.last_tick_pos.y, e.last_tick_pos.z],
            position: [e.pos.x, e.pos.y, e.pos.z],
            previous_rotation: [e.prev_yaw, e.prev_pitch],
            rotation: [e.yaw, e.pitch],
            sleeping: e.player_state().is_some_and(|p| p.sleeping),
        }
    }
}
pub fn position(last: [f64; 3], now: [f64; 3], partial: f32) -> [f64; 3] {
    std::array::from_fn(|i| last[i] + (now[i] - last[i]) * partial as f64)
}
pub fn rotation(previous: [f32; 2], now: [f32; 2], partial: f32) -> [f32; 2] {
    std::array::from_fn(|i| previous[i] + (now[i] - previous[i]) * partial)
}
pub fn lightmap_coordinates(light: i32) -> [f32; 2] {
    // Java's signed remainder/division, including arbitrary virtual overrides.
    [(light % 65536) as f32 / 1.0, (light / 65536) as f32 / 1.0]
}
#[derive(Clone, Copy, Debug)]
pub struct Arguments {
    pub position: [f64; 3],
    pub yaw: f32,
    pub partial: f32,
}
#[derive(Clone, Copy, Debug)]
pub enum Event {
    Lightmap([f32; 2]),
    White,
    Body(usize, Arguments),
    ShadowAndFire(usize, Arguments),
}

pub struct RenderManager<T = ()> {
    pub context: Context<T>,
    pub view: [f32; 2],
    pub camera: [f64; 3],
    pub render_origin: [f64; 3],
    pub classes: HashMap<String, Option<usize>>,
    parents: HashMap<String, Option<String>>,
}
impl<T> Default for RenderManager<T> {
    fn default() -> Self {
        let mut manager = Self {
            context: Context {
                world: None,
                engine: None,
                font: None,
                player: None,
                options: None,
            },
            view: [0.0; 2],
            camera: [0.0; 3],
            render_origin: [0.0; 3],
            classes: RENDERERS
                .iter()
                .enumerate()
                .map(|(i, r)| (r.entity_class.into(), Some(i)))
                .collect(),
            parents: HashMap::new(),
        };
        for (name, parent) in [
            ("EntityMooshroom", "EntityCow"), ("EntityVillager", "EntityLiving"),
            ("EntitySnowman", "EntityLiving"), ("EntityBlaze", "EntityMob"),
            ("EntityMagmaCube", "EntitySlime"), ("EntityDragon", "EntityLiving"),
            ("EntityEnderCrystal", "Entity"), ("EntityEnderPearl", "Entity"),
            ("EntityEnderEye", "Entity"), ("EntityPotion", "Entity"), ("EntitySmallFireball", "Entity"),
            ("EntitySpider", "EntityMob"),
            ("EntityCaveSpider", "EntitySpider"),
            ("EntityPig", "EntityAnimal"),
            ("EntitySheep", "EntityAnimal"),
            ("EntityCow", "EntityAnimal"),
            ("EntityWolf", "EntityAnimal"),
            ("EntityChicken", "EntityAnimal"),
            ("EntitySilverfish", "EntityMob"),
            ("EntityCreeper", "EntityMob"),
            ("EntityEnderman", "EntityMob"),
            ("EntitySkeleton", "EntityMob"),
            ("EntityZombie", "EntityMob"),
            ("EntitySlime", "EntityLiving"),
            ("EntityPlayer", "EntityLiving"),
            ("EntityGiantZombie", "EntityMob"),
            ("EntityGhast", "EntityFlying"),
            ("EntitySquid", "EntityWaterMob"),
            ("EntityLiving", "Entity"),
            ("EntityPainting", "Entity"),
            ("EntityArrow", "Entity"),
            ("EntitySnowball", "Entity"),
            ("EntityEgg", "Entity"),
            ("EntityFireball", "Entity"),
            ("EntityItem", "Entity"),
            ("EntityXPOrb", "Entity"),
            ("EntityTNTPrimed", "Entity"),
            ("EntityFallingSand", "Entity"),
            ("EntityMinecart", "Entity"),
            ("EntityBoat", "Entity"),
            ("EntityFishHook", "Entity"),
            ("EntityLightningBolt", "EntityWeatherEffect"),
            ("Entity", "Object"),
            ("Object", ""),
            ("EntityCreature", "EntityLiving"),
            ("EntityMob", "EntityCreature"),
            ("EntityAnimal", "EntityCreature"),
            ("EntityWaterMob", "EntityLiving"),
            ("EntityFlying", "EntityLiving"),
            ("EntityWeatherEffect", "Entity"),
            ("EntityPigZombie", "EntityZombie"),
            ("EntityPlayerSP", "EntityPlayer"),
            ("EntityPlayerMP", "EntityPlayer"),
            ("EntityOtherPlayerMP", "EntityPlayer"),
            ("EntityFX", "Entity"),
            ("EntityPickupFX", "EntityFX"),
        ] {
            manager.declare_class(name, (!parent.is_empty()).then_some(parent));
        }
        manager
    }
}
impl<T> RenderManager<T> {
    pub fn declare_class(&mut self, name: &str, parent: Option<&str>) {
        self.parents.insert(name.into(), parent.map(str::to_owned));
    }
    pub fn get_entity_class_render_object(
        &mut self,
        class: Option<&str>,
    ) -> Result<Option<usize>, Error> {
        let class = class.ok_or(Error::NullPointer)?;
        let mut renderer = self.classes.get(class).copied().flatten();
        if renderer.is_none() && class != "Entity" {
            let parent = self.parents.get(class).cloned().flatten();
            renderer = self.get_entity_class_render_object(parent.as_deref())?;
            // Even null fallback results are inserted, but are searched again.
            self.classes.insert(class.into(), renderer);
        }
        Ok(renderer)
    }
    pub fn get_entity_render_object(
        &mut self,
        entity: Option<&Entity>,
    ) -> Result<Option<usize>, Error> {
        self.get_entity_class_render_object(Some(class(entity.ok_or(Error::NullPointer)?)))
    }
    pub fn cache_active_render_info(
        &mut self,
        context: Context<T>,
        player: Option<CameraState>,
        partial: f32,
        front_view: bool,
        block: impl FnOnce([i32; 3]) -> Result<(i32, i32), Error>,
    ) -> Result<(), Error> {
        // Assign all five references before invoking isPlayerSleeping.
        self.context = context;
        let player = player.ok_or(Error::NullPointer)?;
        if player.sleeping {
            if self.context.world.is_none() {
                return Err(Error::NullPointer);
            }
            let (id, metadata) = block(player.position.map(worldgen::math::floor_double))?;
            if id == 26 {
                self.view = [((metadata & 3) * 90 + 180) as f32, 0.0];
            }
            // A missing bed preserves both previous angles.
        } else {
            self.view = rotation(player.previous_rotation, player.rotation, partial);
        }
        // 1.0: options.thirdPersonView == 2 turns billboards to face the front camera.
        if self.context.options.is_none() {
            return Err(Error::NullPointer);
        }
        if front_view {
            self.view[0] += 180.0;
        }
        self.camera = position(player.last, player.position, partial);
        Ok(())
    }
    pub fn set(&mut self, world: Option<T>) {
        self.context.world = world;
    }
    pub fn get_font_renderer(&self) -> Option<&T> {
        self.context.font.as_ref()
    }
    pub fn get_distance_to_camera(&self, point: [f64; 3]) -> f64 {
        let d = std::array::from_fn::<_, 3, _>(|i| point[i] - self.camera[i]);
        d[0] * d[0] + d[1] * d[1] + d[2] * d[2]
    }
    pub fn render_entity(
        &mut self,
        class: Option<&str>,
        entity: Option<CameraState>,
        partial: f32,
        brightness: impl FnOnce(f32) -> Result<i32, Error>,
        mut callback: impl FnMut(Event) -> Result<(), Error>,
    ) -> Result<(), Error> {
        let e = entity.ok_or(Error::NullPointer)?;
        let p = position(e.last, e.position, partial);
        let yaw = e.previous_rotation[0] + (e.rotation[0] - e.previous_rotation[0]) * partial;
        let light = brightness(partial)?;
        callback(Event::Lightmap(lightmap_coordinates(light)))?;
        callback(Event::White)?;
        self.render_entity_with_pos_yaw(
            class,
            Arguments {
                position: std::array::from_fn(|i| p[i] - self.render_origin[i]),
                yaw,
                partial,
            },
            callback,
        )
    }
    pub fn render_entity_with_pos_yaw(
        &mut self,
        class: Option<&str>,
        args: Arguments,
        mut callback: impl FnMut(Event) -> Result<(), Error>,
    ) -> Result<(), Error> {
        if let Some(renderer) = self.get_entity_class_render_object(class)? {
            callback(Event::Body(renderer, args))?;
            callback(Event::ShadowAndFire(renderer, args))?;
        }
        Ok(())
    }
}
