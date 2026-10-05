//! TileEntitySpecialRenderer. Texture/font ownership and tile drawing remain
//! with the existing backend; this is the base class's dependency interface.
use super::{fixed::Fixed, manager::Error};

pub trait TextureEngine {
    fn get_texture(&mut self, name: Option<&str>) -> Result<i32, Error>;
    fn bind_texture(&mut self, texture: i32) -> Result<(), Error>;
}
pub struct Context<E, F> {
    pub render_engine: Option<E>,
    pub font_renderer: Option<F>,
}
pub struct TileEntitySpecialRenderer<'a, E, F> {
    pub tile_entity_renderer: Option<&'a mut Context<E, F>>,
}
impl<'a, E: TextureEngine, F> TileEntitySpecialRenderer<'a, E, F> {
    pub fn new() -> Self {
        Self {
            tile_entity_renderer: None,
        }
    }
    pub fn set_tile_entity_renderer(&mut self, renderer: Option<&'a mut Context<E, F>>) {
        self.tile_entity_renderer = renderer;
    }
    pub fn bind_texture_by_name(&mut self, name: Option<&str>) -> Result<(), Error> {
        let renderer = self
            .tile_entity_renderer
            .as_mut()
            .ok_or(Error::NullPointer)?;
        if let Some(engine) = renderer.render_engine.as_mut() {
            let texture = engine.get_texture(name)?;
            engine.bind_texture(texture)?;
        }
        Ok(())
    }
    pub fn get_font_renderer(&self) -> Result<Option<&F>, Error> {
        Ok(self
            .tile_entity_renderer
            .as_ref()
            .ok_or(Error::NullPointer)?
            .font_renderer
            .as_ref())
    }
    /// The original world-change callback is empty, even without a renderer.
    pub fn world_changed<W>(&mut self, _world: Option<W>) {}
}

/// Uses the existing Fixed texture binding. The caller supplies its resource
/// lookup, so this helper does not create a second texture cache or loader.
pub struct FixedBinding<'a, R> {
    pub target: &'a mut Fixed,
    pub resolve: R,
}
impl<R: FnMut(Option<&str>) -> Result<i32, Error>> TextureEngine for FixedBinding<'_, R> {
    fn get_texture(&mut self, name: Option<&str>) -> Result<i32, Error> {
        (self.resolve)(name)
    }
    fn bind_texture(&mut self, texture: i32) -> Result<(), Error> {
        // RenderEngine.bindTexture's nonnegative guard belongs to the engine.
        if texture >= 0 {
            self.target.texture = texture as u32;
        }
        Ok(())
    }
}
