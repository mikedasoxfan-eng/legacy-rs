//! Shared color attachment with literal GL color masks and independent eye depth clears.
use super::super::color_mask::Pipelines;
use wgpu::util::DeviceExt;
pub struct Stereo {
    layout: wgpu::BindGroupLayout,
    present: wgpu::RenderPipeline,
    clear_layout: wgpu::BindGroupLayout,
    clear: Pipelines,
    target: Option<(wgpu::Texture, u32, u32)>,
}
impl Stereo {
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("stereo present"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: false },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            }],
        });
        let clear_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("masked clear"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });
        let source = r#"
@vertex fn vs(@builtin(vertex_index) i: u32) -> @builtin(position) vec4<f32> {
    let p = array<vec2<f32>, 3>(vec2(-1.0,-1.0), vec2(3.0,-1.0), vec2(-1.0,3.0));
    return vec4(p[i],0.0,1.0);
}
"#;
        let build = |layout: &wgpu::BindGroupLayout, fragment: &str, mask| {
            let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("glColorMask"),
                source: wgpu::ShaderSource::Wgsl(format!("{source}{fragment}").into()),
            });
            let pl = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("stereo"),
                bind_group_layouts: &[Some(layout)],
                immediate_size: 0,
            });
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("stereo"),
                layout: Some(&pl),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: Some("vs"),
                    compilation_options: Default::default(),
                    buffers: &[],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: Some("fs"),
                    compilation_options: Default::default(),
                    targets: &[Some(wgpu::ColorTargetState { format, blend: None, write_mask: mask })],
                }),
                primitive: Default::default(),
                depth_stencil: None,
                multisample: Default::default(),
                multiview_mask: None,
                cache: None,
            })
        };
        let present = build(
            &layout,
            r#"
@group(0) @binding(0) var image: texture_2d<f32>;
@fragment fn fs(@builtin(position) p: vec4<f32>) -> @location(0) vec4<f32> { return textureLoad(image, vec2<i32>(p.xy), 0); }
"#,
            wgpu::ColorWrites::ALL,
        );
        let clear = Pipelines::new(|mask| {
            build(
                &clear_layout,
                r#"
@group(0) @binding(0) var<uniform> color: vec4<f32>;
@fragment fn fs() -> @location(0) vec4<f32> { return color; }
"#,
                mask,
            )
        });
        Self { layout, present, clear_layout, clear, target: None }
    }
    pub fn target(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        w: u32,
        h: u32,
        format: wgpu::TextureFormat,
    ) -> wgpu::TextureView {
        if self.target.as_ref().is_none_or(|(_, old_w, old_h)| (*old_w, *old_h) != (w, h)) {
            let tex = device.create_texture(&wgpu::TextureDescriptor {
                label: Some("anaglyph shared framebuffer"),
                size: wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
                view_formats: &[],
            });
            let view = tex.create_view(&Default::default());
            // The game's default LWJGL pixel format has no alpha plane: destination alpha is one.
            self.clear(device, queue, &view, 0, [0.0; 3]);
            self.target = Some((tex, w, h));
        }
        self.target.as_ref().unwrap().0.create_view(&Default::default())
    }
    pub fn clear(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        target: &wgpu::TextureView,
        mask: usize,
        rgb: [f32; 3],
    ) {
        let uniform = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("masked fog clear"),
            contents: bytemuck::cast_slice(&[rgb[0], rgb[1], rgb[2], 1.0]),
            usage: wgpu::BufferUsages::UNIFORM,
        });
        let bind = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &self.clear_layout,
            entries: &[wgpu::BindGroupEntry { binding: 0, resource: uniform.as_entire_binding() }],
        });
        self.draw(device, queue, target, self.clear.get(mask), &bind);
    }
    pub fn present(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        target: &wgpu::TextureView,
        shared: &wgpu::TextureView,
    ) {
        let bind = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &self.layout,
            entries: &[wgpu::BindGroupEntry { binding: 0, resource: wgpu::BindingResource::TextureView(shared) }],
        });
        self.draw(device, queue, target, &self.present, &bind);
    }
    fn draw(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        target: &wgpu::TextureView,
        pipeline: &wgpu::RenderPipeline,
        bind: &wgpu::BindGroup,
    ) {
        let mut enc = device.create_command_encoder(&Default::default());
        {
            let mut pass = enc.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("stereo framebuffer"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: target,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations { load: wgpu::LoadOp::Load, store: wgpu::StoreOp::Store },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });
            pass.set_pipeline(pipeline);
            pass.set_bind_group(0, bind, &[]);
            pass.draw(0..3, 0..1);
        }
        queue.submit([enc.finish()]);
    }
}
