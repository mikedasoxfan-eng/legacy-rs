//! Texture-pack refresh and original dynamic-texture registration/tick order.
use super::{assets, render, App, Colorizers, ToWorker, VideoSettings};
impl App {
    pub(super) fn refresh_textures(&mut self) {
        let (Some(gui), Some(gpu)) = (&mut self.gui, &mut self.gpu) else { return };
        match assets::load_with(gui.ui.packs.path(), gui.ui.settings.anaglyph) {
            Ok(assets) => {
                gpu.refresh_textures(&assets);
                if let Err(e) = gui.refresh_textures(gpu.device(), gpu.queue()) {
                    eprintln!("refreshTextures: {e}");
                }
                let col = Colorizers { grass: assets.grass_color, foliage: assets.foliage_color };
                self.col = Some(col.clone());
                if let Some(tx) = &self.tx {
                    let _ = tx.send(ToWorker::RefreshTextures(col));
                    self.idle = false;
                }
            }
            Err(e) => eprintln!("refreshTextures: {e}"),
        }
    }

    pub(super) fn tick_texture_animations(&mut self, gui_ticks: i32, video: VideoSettings) {
        // Minecraft.runTick updates dynamic textures even with no world.
        let fx_ticks = if self.paused { 0 } else { gui_ticks };
        for _ in 0..fx_ticks {
            for f in &mut self.fx[..3] {
                f.tick_anaglyph(&mut self.fx_rng, video.anaglyph);
            }
            let angle = self.celestial_angle(1.0);
            if let Some(fx) = &mut self.item_fx {
                let mut target = self.player.as_ref().map_or([0.0; 2], |p| {
                    render::item_fx::ItemFx::targets(
                        [p.entity.pos.x, p.entity.pos.y, p.entity.pos.z],
                        p.entity.yaw,
                        p.world.spawn,
                        angle,
                    )
                });
                // TextureCompassFX / TextureWatchFX: `worldProvider.isNether` (the Nether and the End) spins them.
                if self.nether || self.end {
                    target = [
                        self.fx_rng.random() * std::f64::consts::PI as f32 as f64 * 2.0,
                        self.fx_rng.random() * std::f64::consts::PI as f32 as f64 * 2.0,
                    ];
                }
                fx.tick(target, video.anaglyph);
            }
            for f in &mut self.fx[3..] {
                f.tick_anaglyph(&mut self.fx_rng, video.anaglyph);
            }
            if let Some(gpu) = &self.gpu {
                gpu.update_fx(&self.fx);
                if let Some(gui) = &self.gui {
                    for f in &self.fx {
                        for (x,y) in crate::gui::render_engine::dynamic_tiles(f.icon as i32,f.tile_size as i32) {
                            gui.renderer.update_atlas_tile(gpu.queue(),gui.ui.texture("terrain.png"),(x/16+y/16*16) as u32,&f.image);
                        }
                    }
                    if let Some(fx) = &self.item_fx {
                        gpu.update_item_fx(fx);
                        for (i, icon) in render::item_fx::ICONS.into_iter().enumerate() {
                            gui.renderer.update_atlas_tile(
                                gpu.queue(),
                                gui.ui.texture("gui/items.png"),
                                icon,
                                &fx.images[i],
                            );
                        }
                    }
                }
            }
        }
    }
}
