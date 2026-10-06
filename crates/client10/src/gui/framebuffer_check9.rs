//! Use the production GUI renderer for the original GL masked-clear fixture.
use super::{
    gl::{Factor, Gl},
    render::GuiRenderer,
    shot::read_back,
};

pub fn check(path: &str) -> bool {
    let expected = std::fs::read(path).unwrap();
    let tail = &expected[expected.len() - 324..];
    assert_eq!(i32::from_be_bytes(tail[..4].try_into().unwrap()), 80);
    let instance =
        wgpu::Instance::new(wgpu::InstanceDescriptor::new_without_display_handle_from_env());
    let adapter = pollster::block_on(instance.request_adapter(&Default::default())).unwrap();
    let (device, queue) = pollster::block_on(adapter.request_device(&Default::default())).unwrap();
    // Also exercise framebuffer copies and BGRA conversion with the same renderer.
    for format in [
        wgpu::TextureFormat::Rgba8Unorm,
        wgpu::TextureFormat::Bgra8Unorm,
    ] {
        for (size, orientation) in [((256, 256), true), ((160, 200), true), ((160, 200), false)] {
            let target = device.create_texture(&wgpu::TextureDescriptor {
                label: Some("GUI mask fixture"),
                size: wgpu::Extent3d {
                    width: size.0,
                    height: size.1,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
                view_formats: &[],
            });
            let view = target.create_view(&Default::default());
            let mut renderer = GuiRenderer::new(&device, &queue, format, 1);
            renderer.gl_orientation = orientation;
            let copied = renderer.add_texture(
                &device,
                &queue,
                &crate::render::gpu::Image {
                    width: 256,
                    height: 256,
                    rgba: vec![0; 256 * 256 * 4],
                },
            );
            let mut out = Vec::new();
            for c in 0..80 {
                let mask = c % 5;
                let (alpha, source_alpha) = (
                    ((c / 5) % 2) as f64,
                    if (c / 10) % 2 == 0 { 0.5 } else { 1.0 },
                );
                let (r, g) = (((c / 20) % 2) as f64, ((c / 40) % 2) as f64);
                renderer.render(
                    &device,
                    &queue,
                    &view,
                    size,
                    &Gl::new(),
                    Some([0.0, 1.0, 1.0, alpha]),
                );
                let mut gl = Gl::new();
                gl.color_mask = match mask {
                    0 => 15,
                    1 => 6,
                    2 => 1,
                    3 => 14,
                    _ => 9,
                };
                gl.begin_frame();
                gl.depth_test = false;
                gl.texture_2d = false;
                gl.alpha_test = false;
                gl.blend = true;
                gl.blend_func = (Factor::SrcAlpha, Factor::DstAlpha);
                gl.color4f(1.0, 0.0, 1.0, source_alpha);
                gl.start_drawing_quads();
                for (x, y) in [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)] {
                    gl.add_vertex(x, y, 0.0);
                }
                gl.draw();
                renderer.render(
                    &device,
                    &queue,
                    &view,
                    size,
                    &gl,
                    Some([r, g, 1.0 - r, 1.0]),
                );
                let image = read_back(&device, &queue, &target);
                let pixel = &image.rgba[(128 * size.0 as usize + 128) * 4..][..4];
                let rgba = if format == wgpu::TextureFormat::Bgra8Unorm {
                    [pixel[2], pixel[1], pixel[0], pixel[3]]
                } else {
                    pixel.try_into().unwrap()
                };
                out.extend(rgba);
                renderer.copy_framebuffer(&device, &queue, &view, size, copied);
                let copied_image =
                    read_back(&device, &queue, renderer.texture_view(copied).texture());
                assert_eq!(
                    &copied_image.rgba[(128 * 256 + 128) * 4..][..4],
                    &rgba,
                    "framebuffer copy {format:?} case {c}"
                );
            }
            if out != tail[4..] {
                eprintln!("GUI FRAMEBUFFER DIFFERS ({format:?})");
                return false;
            }
            let face = renderer.add_texture(
                &device,
                &queue,
                &crate::render::gpu::Image {
                    width: 1,
                    height: 1,
                    rgba: vec![255; 4],
                },
            );
            renderer.check_panorama_resize(&device, &queue, &view, size, face, copied);
            let _ = read_back(&device, &queue, &target);
        }
    }
    println!("GUI FRAMEBUFFER EXACT: 80 original RGBA pixels, masked clears, destination-alpha blends and RGBA/BGRA copies");
    println!("PANORAMA RESIZE SMOKE PASSED: 256x256 and 160x200, both orientations and RGBA/BGRA; no small-window original pixel credit");
    true
}
