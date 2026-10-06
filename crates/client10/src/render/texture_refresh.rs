//! RenderEngine.refreshTextures: retain the device and all geometry, replace texture storage.
use super::{make_slot, Gpu, Image};
use wgpu::util::DeviceExt;

impl Gpu {
    pub fn update_item_fx(&self, fx: &crate::render::item_fx::ItemFx) {
        for pipeline in [&self.entities, &self.hand, &self.overlays, &self.targeting, &self.lit_effects] {
            pipeline.update_items(&self.queue, fx);
        }
        // Item particles bind their own copy of the atlas.
        crate::gui::render::write_tile(&self.queue, &self.items_tex, crate::render::item_fx::ICONS[0], &fx.images[0]);
        crate::gui::render::write_tile(&self.queue, &self.items_tex, crate::render::item_fx::ICONS[1], &fx.images[1]);
    }

    pub fn refresh_textures(&mut self, assets: &crate::assets::Assets) {
        let upload = |image: &Image| {
            self.device.create_texture_with_data(
                &self.queue,
                &wgpu::TextureDescriptor {
                    label: Some("refresh texture"),
                    size: wgpu::Extent3d { width: image.width, height: image.height, depth_or_array_layers: 1 },
                    mip_level_count: 1,
                    sample_count: 1,
                    dimension: wgpu::TextureDimension::D2,
                    format: wgpu::TextureFormat::Rgba8Unorm,
                    usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                    view_formats: &[],
                },
                wgpu::util::TextureDataOrder::LayerMajor,
                &image.rgba,
            )
        };
        let terrain = upload(&assets.terrain);
        let slot = |image: &Image| {
            make_slot(
                &self.device,
                &self.texture_layout,
                &self.sampler,
                &upload(image).create_view(&Default::default()),
            )
        };
        // Both slots must sample the storage overwritten by the procedural animations.
        self.terrain_slot =
            make_slot(&self.device, &self.texture_layout, &self.sampler, &terrain.create_view(&Default::default()));
        self.water_slot =
            make_slot(&self.device, &self.texture_layout, &self.sampler, &terrain.create_view(&Default::default()));
        self.sun_slot = slot(&assets.sun);
        self.moon_slot = slot(&assets.moon);
        self.end_slots = std::array::from_fn(|_| slot(&assets.mobs["misc/tunnel.png"]));
        self.cloud_slot = slot(&assets.clouds);
        self.weather_slots[0] = slot(&assets.snow);
        self.weather_slots[1] = slot(&assets.rain);
        self.weather_slots[2] = slot(&assets.particles);
        self.weather_slots[4] =
            make_slot(&self.device, &self.texture_layout, &self.sampler, &terrain.create_view(&Default::default()));
        self.items_tex = upload(&assets.mobs["gui/items.png"]);
        self.weather_slots[5] = make_slot(
            &self.device,
            &self.texture_layout,
            &self.sampler,
            &self.items_tex.create_view(&Default::default()),
        );
        self.terrain_tex = terrain;
        for pipeline in
            [&mut self.entities, &mut self.hand, &mut self.overlays, &mut self.targeting, &mut self.lit_effects]
        {
            for (name, image) in assets
                .mobs
                .iter()
                .map(|(n, i)| (n.as_str(), i))
                .chain([("terrain.png", &assets.terrain), ("misc/footprint.png", &assets.footprint)])
            {
                pipeline.replace_texture(&self.device, &self.queue, name, image);
            }
        }
    }
}
