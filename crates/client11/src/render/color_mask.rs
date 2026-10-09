//! GL color-write masks used by EntityRenderer and RenderGlobal.
pub const MASKS: [wgpu::ColorWrites; 5] = [
    wgpu::ColorWrites::ALL,
    wgpu::ColorWrites::GREEN.union(wgpu::ColorWrites::BLUE),
    wgpu::ColorWrites::RED,
    wgpu::ColorWrites::GREEN.union(wgpu::ColorWrites::BLUE).union(wgpu::ColorWrites::ALPHA),
    wgpu::ColorWrites::RED.union(wgpu::ColorWrites::ALPHA),
];
pub struct Pipelines([wgpu::RenderPipeline; 5]);
impl Pipelines {
    pub fn new(mut build: impl FnMut(wgpu::ColorWrites) -> wgpu::RenderPipeline) -> Self {
        Self(MASKS.map(|mask| build(mask)))
    }
    pub fn get(&self, mask: usize) -> &wgpu::RenderPipeline {
        &self.0[mask]
    }
    pub fn get_bind_group_layout(&self, index: u32) -> wgpu::BindGroupLayout {
        self.0[0].get_bind_group_layout(index)
    }
}
pub fn eye_mask(anaglyph: bool, eye: i32, alpha: bool) -> usize {
    if anaglyph {
        1 + eye as usize + if alpha { 2 } else { 0 }
    } else {
        0
    }
}

/// Pixel-check the world pipeline masks against actual original-client OpenGL readback.
pub fn check(file: &mut impl std::io::Read) -> Result<(), String> {
    use super::entity::{
        living::{Batch, EntityVertex, Pass},
        pipeline::EntityPipeline,
    };
    let mut count = [0; 4];
    file.read_exact(&mut count).map_err(|e| e.to_string())?;
    let count = i32::from_be_bytes(count);
    assert_eq!(count, 80);
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::new_without_display_handle_from_env());
    let adapter = pollster::block_on(instance.request_adapter(&Default::default())).map_err(|e| e.to_string())?;
    let (device, queue) = pollster::block_on(adapter.request_device(&Default::default())).map_err(|e| e.to_string())?;
    let format = wgpu::TextureFormat::Rgba8Unorm;
    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("mask oracle"),
        size: wgpu::Extent3d { width: 1, height: 1, depth_or_array_layers: 1 },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
        view_formats: &[],
    });
    let view = texture.create_view(&Default::default());
    let depth = device
        .create_texture(&wgpu::TextureDescriptor {
            label: None,
            size: wgpu::Extent3d { width: 1, height: 1, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        })
        .create_view(&Default::default());
    let stereo = super::gpu::stereo::Stereo::new(&device, format);
    let mut pipeline = EntityPipeline::new(&device, &queue, format, std::iter::empty());
    let readback = device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        size: 256,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });
    for c in 0..count {
        let mask = (c % 5) as usize;
        let alpha = ((c / 5) % 2) as f64;
        let source = if (c / 10) % 2 == 0 { 0.5 } else { 1.0 };
        let r = ((c / 20) % 2) as f32;
        let g = ((c / 40) % 2) as f32;
        let mut enc = device.create_command_encoder(&Default::default());
        {
            let _pass = enc.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color { r: 0.0, g: 1.0, b: 1.0, a: alpha }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });
        }
        queue.submit([enc.finish()]);
        stereo.clear(&device, &queue, &view, mask, [r, g, 1.0 - r]);
        let vertices = [
            [-1.0, -1.0, 0.0],
            [1.0, -1.0, 0.0],
            [1.0, 1.0, 0.0],
            [-1.0, -1.0, 0.0],
            [1.0, 1.0, 0.0],
            [-1.0, 1.0, 0.0],
        ]
        .map(|pos| EntityVertex { pos, uv: [0.0; 2], color: [1.0, 0.0, 1.0, source] })
        .to_vec();
        pipeline.upload(
            &device,
            &queue,
            vec![Batch { alpha_cutoff: 0.1, cull: 0, texture: None, pass: Pass::TntFlash, vertices }],
            glam::Mat4::IDENTITY,
            [0.0; 3],
            0.0,
        );
        let mut enc = device.create_command_encoder(&Default::default());
        {
            let mut pass = enc.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations { load: wgpu::LoadOp::Load, store: wgpu::StoreOp::Store },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &depth,
                    depth_ops: Some(wgpu::Operations { load: wgpu::LoadOp::Clear(1.0), store: wgpu::StoreOp::Store }),
                    stencil_ops: None,
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });
            pipeline.draw_masked(&mut pass, mask);
        }
        enc.copy_texture_to_buffer(
            wgpu::TexelCopyTextureInfo {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::TexelCopyBufferInfo {
                buffer: &readback,
                layout: wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(256), rows_per_image: Some(1) },
            },
            wgpu::Extent3d { width: 1, height: 1, depth_or_array_layers: 1 },
        );
        queue.submit([enc.finish()]);
        let slice = readback.slice(..);
        slice.map_async(wgpu::MapMode::Read, |_| {});
        let _ = device.poll(wgpu::PollType::wait_indefinitely());
        let mapped = slice.get_mapped_range().map_err(|e| e.to_string())?;
        let actual: [u8; 4] = mapped[..4].try_into().unwrap();
        drop(mapped);
        readback.unmap();
        let mut expected = [0; 4];
        file.read_exact(&mut expected).map_err(|e| e.to_string())?;
        if actual != expected {
            return Err(format!("GL color mask case {c} differs: GPU {actual:?}, original {expected:?}"));
        }
    }
    println!("COLOR MASKS EXACT: 80 GPU pixels, masked clears and destination-alpha blending");
    Ok(())
}
