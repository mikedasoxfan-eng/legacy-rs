//! TileEntityRenderer's backend-independent cache and dispatch. Live tile
//! drawing and world simulation stay in the existing backend callbacks.
use super::{
    manager::{self, CameraState, Error},
    tile_special::Context,
};
use worldgen::world::java_hash::JavaHashSet;

pub const RENDERERS: &[(&str, &str)] = &[
    ("TileEntitySign", "TileEntitySignRenderer"),
    ("TileEntityMobSpawner", "TileEntityMobSpawnerRenderer"),
    ("TileEntityPiston", "TileEntityRendererPiston"),
    ("TileEntityChest", "TileEntityChestRenderer"),
];
struct Class {
    name: String,
    parent: Option<String>,
    hash: i32,
    renderer: Option<usize>,
}
#[derive(Clone, Copy)]
pub struct Tile<'a> {
    pub class: &'a str,
    pub position: [i32; 3],
}
impl Tile<'_> {
    pub fn distance_from(&self, camera: [f64; 3]) -> f64 {
        let delta = std::array::from_fn::<_, 3, _>(|i| self.position[i] as f64 + 0.5 - camera[i]);
        delta[0] * delta[0] + delta[1] * delta[1] + delta[2] * delta[2]
    }
}
#[derive(Clone, Copy)]
pub struct Arguments {
    pub position: [f64; 3],
    pub partial: f32,
}
#[derive(Clone, Copy)]
pub enum Event {
    Lightmap([f32; 2]),
    White,
    Render(usize, Arguments),
}

pub struct TileEntityRenderer<W = (), E = (), F = (), P = ()> {
    pub world: Option<W>,
    pub resources: Context<E, F>,
    pub player: Option<P>,
    pub view: [f32; 2],
    pub camera: [f64; 3],
    pub render_origin: [f64; 3],
    classes: Vec<Class>,
    order: JavaHashSet<usize>,
}
impl<W: PartialEq, E, F, P> TileEntityRenderer<W, E, F, P> {
    /// Class identity hashes are external VM/backend inputs, just as for the
    /// existing SpawnerAnimals identity-aware JavaHashSet implementation.
    pub fn new(mut class_hash: impl FnMut(&str) -> i32) -> Self {
        let mut m = Self {
            world: None,
            resources: Context {
                render_engine: None,
                font_renderer: None,
            },
            player: None,
            view: [0.0; 2],
            camera: [0.0; 3],
            render_origin: [0.0; 3],
            classes: Vec::new(),
            order: JavaHashSet::default(),
        };
        m.declare_class("TileEntity", Some("Object"), class_hash("TileEntity"));
        m.declare_class("Object", None, class_hash("Object"));
        for (slot, &(name, _)) in RENDERERS.iter().enumerate() {
            m.declare_class(name, Some("TileEntity"), class_hash(name));
            let key = m.key(name).unwrap();
            m.put(key, Some(slot));
        }
        m
    }
    pub fn declare_class(&mut self, name: &str, parent: Option<&str>, hash: i32) {
        if let Some(key) = self.key(name) {
            self.classes[key].parent = parent.map(str::to_owned);
            self.classes[key].hash = hash;
        } else {
            self.classes.push(Class {
                name: name.into(),
                parent: parent.map(str::to_owned),
                hash,
                renderer: None,
            });
        }
    }
    fn key(&self, name: &str) -> Option<usize> {
        self.classes.iter().position(|c| c.name == name)
    }
    fn put(&mut self, key: usize, renderer: Option<usize>) {
        self.order
            .insert_with_identity(self.classes[key].hash, self.classes[key].hash, key);
        self.classes[key].renderer = renderer;
    }
    pub fn cached_count(&self) -> usize {
        self.order.len()
    }
    pub fn set_class_renderer(&mut self, name: &str, renderer: Option<usize>) {
        let key = self.key(name).expect("declared renderer class");
        self.put(key, renderer);
    }
    pub fn is_cached(&self, name: &str) -> bool {
        self.key(name)
            .is_some_and(|key| self.order.iter().any(|k| k == key))
    }
    pub fn get_special_renderer_for_class(
        &mut self,
        class: Option<&str>,
    ) -> Result<Option<usize>, Error> {
        let name = class.ok_or(Error::NullPointer)?;
        let key = self.key(name).ok_or(Error::NullPointer)?;
        let mut renderer = self.classes[key].renderer;
        if renderer.is_none() && name != "TileEntity" {
            let parent = self.classes[key].parent.clone();
            renderer = self.get_special_renderer_for_class(parent.as_deref())?;
            self.put(key, renderer);
        }
        Ok(renderer)
    }
    pub fn get_special_renderer_for_entity(
        &mut self,
        tile: Option<Tile<'_>>,
    ) -> Result<Option<usize>, Error> {
        match tile {
            Some(tile) => self.get_special_renderer_for_class(Some(tile.class)),
            None => Ok(None),
        }
    }
    pub fn has_special_renderer(&mut self, tile: Option<Tile<'_>>) -> Result<bool, Error> {
        Ok(self.get_special_renderer_for_entity(tile)?.is_some())
    }
    pub fn set_world(
        &mut self,
        world: Option<W>,
        mut notify: impl FnMut(usize, Option<&W>) -> Result<(), Error>,
    ) -> Result<(), Error> {
        self.world = world;
        for key in self.order.iter() {
            if let Some(renderer) = self.classes[key].renderer {
                notify(renderer, self.world.as_ref())?;
            }
        }
        Ok(())
    }
    pub fn cache_active_render_info(
        &mut self,
        world: Option<W>,
        engine: Option<E>,
        font: Option<F>,
        player: Option<P>,
        camera: Option<CameraState>,
        partial: f32,
        notify: impl FnMut(usize, Option<&W>) -> Result<(), Error>,
    ) -> Result<(), Error> {
        if self.world != world {
            self.set_world(world, notify)?;
        }
        self.resources.render_engine = engine;
        self.player = player;
        self.resources.font_renderer = font;
        let camera = camera.ok_or(Error::NullPointer)?;
        self.view = manager::rotation(camera.previous_rotation, camera.rotation, partial);
        self.camera = manager::position(camera.last, camera.position, partial);
        Ok(())
    }
    pub fn get_font_renderer(&self) -> Option<&F> {
        self.resources.font_renderer.as_ref()
    }
    pub fn render_tile_entity(
        &mut self,
        tile: Option<Tile<'_>>,
        partial: f32,
        distance: impl FnOnce([f64; 3]) -> Result<f64, Error>,
        brightness: impl FnOnce(Option<&W>, [i32; 3], i32) -> Result<i32, Error>,
        mut callback: impl FnMut(Event) -> Result<(), Error>,
    ) -> Result<(), Error> {
        let tile = tile.ok_or(Error::NullPointer)?;
        if distance(self.camera)? < 4096.0 {
            if self.world.is_none() {
                return Err(Error::NullPointer);
            }
            let light = brightness(self.world.as_ref(), tile.position, 0)?;
            callback(Event::Lightmap(manager::lightmap_coordinates(light)))?;
            callback(Event::White)?;
            self.render_tile_entity_at(
                Some(tile),
                Arguments {
                    position: std::array::from_fn(|i| {
                        tile.position[i] as f64 - self.render_origin[i]
                    }),
                    partial,
                },
                callback,
            )?;
        }
        Ok(())
    }
    pub fn render_tile_entity_at(
        &mut self,
        tile: Option<Tile<'_>>,
        args: Arguments,
        mut callback: impl FnMut(Event) -> Result<(), Error>,
    ) -> Result<(), Error> {
        if let Some(renderer) = self.get_special_renderer_for_entity(tile)? {
            callback(Event::Render(renderer, args))?;
        }
        Ok(())
    }
}
