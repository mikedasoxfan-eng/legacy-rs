//! `beta --guishot <script> <outdir>`: drive the GUI from a scene script and save each `shot`
//! as a PNG, rendered off-screen at 854x480. `tools/oracle/guishot.sh` runs the same script
//! through the original client, so the two sets of images can be diffed pixel for pixel.
//!
//! Script lines (whitespace separated, `#` comments):
//!   saves <dir>                   world list folder
//!   millis <n>                    System.currentTimeMillis() for animations
//!   splash <text...>              title-screen splash text for the next `open title`
//!   open title|options|video|controls|select|create|texturepacks
//!   open rename <folder>          GuiRenameWorld for a save folder
//!   mojang                        draw the startup Mojang logo instead of a screen
//!   loading <progress> <title>|<text>
//!   mouse <x> <y>                 GUI-scaled mouse position
//!   down / up                     left button state (Mouse.isButtonDown)
//!   click / release               mouseClicked / mouseMovedOrUp with button 0
//!   key <lwjgl code> <char code>  keyTyped
//!   update [n]                    updateScreen n times
//!   shot <name>                   draw a frame and capture it

use super::gl::Gl;
use super::loading::LoadingScreen;
use super::screens;
use super::system::{GuiSystem, Resources};
use crate::render::gpu::Image;

pub const SIZE: (u32, u32) = (854, 480);

pub fn run(script: &str, out_dir: &std::path::Path) -> Result<(), String> {
    let text = std::fs::read_to_string(script).map_err(|e| format!("{script}: {e}"))?;
    std::fs::create_dir_all(out_dir).map_err(|e| e.to_string())?;
    let mut jar = crate::assets::Jar::open()?;
    let (images, font_txt, lang, splashes) = super::load_resources(&mut jar)?;
    let instance =
        wgpu::Instance::new(wgpu::InstanceDescriptor::new_without_display_handle_from_env());
    let adapter =
        pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions::default()))
            .map_err(|e| format!("no GPU adapter: {e}"))?;
    let (device, queue) = pollster::block_on(adapter.request_device(&Default::default()))
        .map_err(|e| e.to_string())?;
    eprintln!(
        "guishot: {:?} on {}",
        adapter.get_info().backend,
        adapter.get_info().name
    );
    let format = wgpu::TextureFormat::Rgba8Unorm;
    let target = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("guishot"),
        size: wgpu::Extent3d {
            width: SIZE.0,
            height: SIZE.1,
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
    let mut sys = GuiSystem::new(
        &device,
        &queue,
        format,
        Resources {
            images,
            font_txt,
            lang,
            splashes,
        },
        Default::default(),
        (SIZE.0 as i32, SIZE.1 as i32),
        "saves".into(),
        0,
        std::env::var("GUI_MSAA")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(1),
    );
    sys.ui.fixed_date = true;
    sys.renderer.gl_orientation = true;
    let mut splash = String::from("missingno");
    let (mut mx, mut my) = (-1, -1);
    let mut loading: Option<LoadingScreen> = None;
    let mut mojang = false;
    let mut hud: Option<super::ingame::HudPlayer> = None;
    let mut ingame = super::ingame::GuiIngame::new();
    let mut clear_level = 0.0f64;
    let mut container: Option<(
        worldgen::container::Container,
        worldgen::container::InventoryPlayer,
    )> = None;
    let mut preview = worldgen::container::PlayerPreview::default();
    for line in text.lines() {
        let line = line.split('#').next().unwrap().trim();
        if line.is_empty() {
            continue;
        }
        let (cmd, rest) = line.split_once(' ').unwrap_or((line, ""));
        let arg = |i: usize| rest.split_whitespace().nth(i).unwrap_or("").to_string();
        match cmd {
            "saves" => {
                // Relative to the script's folder.
                let base = std::path::Path::new(script)
                    .parent()
                    .unwrap_or(std::path::Path::new("."));
                sys.ui.saves_dir = base.join(rest);
            }
            "millis" => sys.ui.millis = rest.parse().map_err(|_| "millis")?,
            "splash" => splash = rest.to_string(),
            "mojang" => {
                mojang = true;
                loading = None;
            }
            "loading" => {
                let progress: i32 = arg(0).parse().map_err(|_| "loading progress")?;
                let (t, s) = rest
                    .split_once(' ')
                    .map(|(_, r)| r)
                    .unwrap_or("")
                    .split_once('|')
                    .unwrap_or(("", ""));
                loading = Some(LoadingScreen {
                    title: t.into(),
                    text: s.into(),
                    progress,
                });
                mojang = false;
            }
            "open" => {
                mojang = false;
                loading = None;
                hud = None;
                let title: Box<dyn super::Screen> =
                    Box::new(screens::main_menu::GuiMainMenu::with_splash(&splash));
                let s: Box<dyn super::Screen> = match arg(0).as_str() {
                    "inventory" | "crafting" | "chest" | "largechest" | "furnace" | "dispenser" => {
                        preview = Default::default();
                        use worldgen::container::*;
                        let kind = match arg(0).as_str() {
                            "inventory" => ContainerKind::Player,
                            "crafting" => ContainerKind::Workbench,
                            "chest" => ContainerKind::Chest { rows: 3 },
                            "largechest" => ContainerKind::Chest { rows: 6 },
                            "furnace" => ContainerKind::Furnace,
                            _ => ContainerKind::Dispenser,
                        };
                        let c = Container::new(kind, 0);
                        let p = InventoryPlayer::default();
                        let snapshot = c.snapshot(&p);
                        let screen = Box::new(screens::container::GuiContainer::new(&snapshot));
                        sys.ui.container_snapshot = Some(snapshot);
                        sys.ui.world_open = true;
                        container = Some((c, p));
                        screen
                    }
                    "title" => title,
                    "options" => Box::new(screens::options::GuiOptions::new(title)),
                    "video" => Box::new(screens::options::GuiVideoSettings::new(title)),
                    "controls" => Box::new(screens::options::GuiControls::new(title)),
                    "select" => Box::new(screens::select_world::GuiSelectWorld::new(title)),
                    "create" => Box::new(screens::select_world::GuiCreateWorld::new(title)),
                    "rename" => Box::new(screens::select_world::GuiRenameWorld::new(title, arg(1))),
                    "texturepacks" => Box::new(screens::texture_packs::GuiTexturePacks::new(title)),
                    "achievements" => {
                        sys.ui.world_open = true;
                        Box::new(screens::achievements::GuiAchievements::new())
                    }
                    other => return Err(format!("unknown screen {other}")),
                };
                let mut acts = Vec::new();
                sys.display_gui_screen(Some(s), &mut acts);
            }
            "world" => sys.ui.world_open = true,
            "stack" => {
                let (c, p) = container.as_mut().ok_or("stack requires container")?;
                let id: i32 = arg(1).parse().map_err(|_| "stack id")?;
                let stack = if id < 0 {
                    None
                } else {
                    Some(worldgen::container::ItemStack::new(
                        id,
                        arg(2).parse().map_err(|_| "stack count")?,
                        arg(3).parse().map_err(|_| "stack damage")?,
                    ))
                };
                c.put_stack(p, arg(0).parse().map_err(|_| "stack slot")?, stack);
                sys.ui.container_snapshot = Some(c.snapshot(p));
            }
            "cursor" => {
                let (c, p) = container.as_mut().ok_or("cursor requires container")?;
                let id: i32 = arg(0).parse().map_err(|_| "cursor id")?;
                p.cursor = if id < 0 {
                    None
                } else {
                    Some(worldgen::container::ItemStack::new(
                        id,
                        arg(1).parse().map_err(|_| "cursor count")?,
                        arg(2).parse().map_err(|_| "cursor damage")?,
                    ))
                };
                sys.ui.container_snapshot = Some(c.snapshot(p));
            }
            "furnace" => {
                let (c, p) = container.as_mut().ok_or("furnace requires container")?;
                for i in 0..3 {
                    c.progress[i] = arg(i).parse().map_err(|_| "furnace progress")?;
                }
                sys.ui.container_snapshot = Some(c.snapshot(p));
            }
            "pose" => {
                preview = worldgen::container::PlayerPreview {
                    age: arg(0).parse().map_err(|_| "pose age")?,
                    limb_swing: arg(1).parse().map_err(|_| "pose swing")?,
                    limb_amount: arg(2).parse().map_err(|_| "pose amount")?,
                    attack: arg(3).parse().map_err(|_| "pose attack")?,
                    riding: arg(4) == "on",
                    sneak: arg(5) == "on",
                    hurt_time: if arg(6).is_empty() {
                        0
                    } else {
                        arg(6).parse().map_err(|_| "pose hurt")?
                    },
                };
            }
            "hud" => {
                mojang = false;
                loading = None;
                let mut p = super::ingame::HudPlayer::default();
                for kv in rest.split_whitespace() {
                    let (k, v) = kv.split_once('=').ok_or("hud key=value")?;
                    let i = || v.parse::<i32>().map_err(|_| format!("hud {k}"));
                    match k {
                        "health" => p.health = i()?,
                        "prev" => p.prev_health = i()?,
                        "hearts_life" => p.hearts_life = i()?,
                        "item" => p.current_item = i()?,
                        "brightness" => p.brightness = v.parse().map_err(|_| "hud brightness")?,
                        "pos" => {}
                        _ => return Err(format!("unknown hud field {k}")),
                    }
                }
                hud = Some(p);
            }
            "clear" => clear_level = rest.parse::<f64>().map_err(|_| "clear")? / 255.0,
            "white" => {
                let (w, h) = if rest == "mob/char.png" {
                    (64, 32)
                } else {
                    (256, 256)
                };
                sys.renderer.update_texture(
                    &queue,
                    sys.ui.texture(rest),
                    &Image {
                        width: w,
                        height: h,
                        rgba: vec![255; (w * h * 4) as usize],
                    },
                );
            }
            "counter" => ingame.set_update_counter(rest.parse().map_err(|_| "counter")?),
            "mouse" => {
                mx = arg(0).parse().map_err(|_| "mouse x")?;
                my = arg(1).parse().map_err(|_| "mouse y")?;
            }
            "down" => sys.ui.mouse_down = true,
            "up" => sys.ui.mouse_down = false,
            "click" => {
                sys.mouse_event(
                    mx,
                    my,
                    if rest.is_empty() {
                        0
                    } else {
                        rest.parse().map_err(|_| "click button")?
                    },
                    true,
                );
                sys.process_actions();
            }
            "shift" => sys.ui.shift_down = rest == "on",
            "release" => {
                sys.mouse_event(mx, my, 0, false);
                sys.process_actions();
            }
            "key" => {
                let code: i32 = arg(0).parse().map_err(|_| "key code")?;
                let ch: u32 = arg(1).parse().map_err(|_| "key char")?;
                sys.key_event(char::from_u32(ch).unwrap_or('\0'), code);
                sys.process_actions();
            }
            "update" => {
                let n: usize = if rest.is_empty() {
                    1
                } else {
                    rest.parse().map_err(|_| "update n")?
                };
                for _ in 0..n {
                    sys.update_screen();
                }
            }
            "shot" => {
                sys.ui.gl.begin_frame();
                let clear;
                if mojang {
                    sys.draw_mojang_logo();
                    clear = Some([clear_level, clear_level, clear_level, 0.0]);
                } else if let Some(p) = &hud {
                    ingame = super::ingame::GuiIngame::new_with_counter(&ingame);
                    ingame.render_game_overlay(&mut sys.ui, 0.0, false, p, &Default::default());
                    clear = Some([clear_level, clear_level, clear_level, 0.0]);
                } else if let Some(l) = &loading {
                    l.draw(&mut sys.ui);
                    clear = Some([clear_level, clear_level, clear_level, 0.0]);
                } else {
                    sys.draw_screen(mx, my, 0.0);
                    sys.process_actions();
                    clear = Some([clear_level, clear_level, clear_level, 0.0]);
                }
                sys.renderer
                    .render(&device, &queue, &view, SIZE, &sys.ui.gl, clear);
                let img = read_back(&device, &queue, &target);
                let path = out_dir.join(format!("{}.png", rest.trim()));
                crate::assets::save_png(&img, &path).map_err(|e| e.to_string())?;
                println!("rust: {}", path.display());
            }
            other => return Err(format!("unknown script command {other}")),
        }
        for command in sys.take_container_commands() {
            if let Some((c, p)) = container.as_mut() {
                match command {
                    worldgen::container::ContainerCommand::Click {
                        window,
                        slot,
                        button,
                        shift,
                    } if window == c.window => {
                        c.slot_click(p, slot, button, shift);
                        sys.update_container(c.snapshot(p));
                    }
                    worldgen::container::ContainerCommand::Close { window }
                        if window == c.window =>
                    {
                        c.close(p, false);
                    }
                    _ => {}
                }
            }
        }
        if let Some(s) = sys.ui.container_snapshot.as_mut() {
            s.preview = preview;
        }
    }
    let _ = Gl::new;
    Ok(())
}

fn read_back(device: &wgpu::Device, queue: &wgpu::Queue, tex: &wgpu::Texture) -> Image {
    let (w, h) = SIZE;
    let row = (w * 4).div_ceil(256) * 256;
    let buf = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("guishot readback"),
        size: (row * h) as u64,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });
    let mut enc = device.create_command_encoder(&Default::default());
    enc.copy_texture_to_buffer(
        wgpu::TexelCopyTextureInfo {
            texture: tex,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        wgpu::TexelCopyBufferInfo {
            buffer: &buf,
            layout: wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(row),
                rows_per_image: Some(h),
            },
        },
        wgpu::Extent3d {
            width: w,
            height: h,
            depth_or_array_layers: 1,
        },
    );
    queue.submit([enc.finish()]);
    let slice = buf.slice(..);
    slice.map_async(wgpu::MapMode::Read, |_| {});
    let _ = device.poll(wgpu::PollType::wait_indefinitely());
    let data = slice.get_mapped_range().expect("mapped");
    let mut rgba = Vec::with_capacity((w * h * 4) as usize);
    for y in 0..h {
        let r = &data[(y * row) as usize..(y * row + w * 4) as usize];
        rgba.extend_from_slice(r);
    }
    {
        // Rendered in OpenGL orientation (see GuiRenderer::gl_orientation): flip back.
        let row = (w * 4) as usize;
        let mut flipped = Vec::with_capacity(rgba.len());
        for y in (0..h as usize).rev() {
            flipped.extend_from_slice(&rgba[y * row..(y + 1) * row]);
        }
        rgba = flipped;
    }
    // Opaque output like the GL color buffer read as RGB.
    for px in rgba.chunks_mut(4) {
        px[3] = 255;
    }
    Image {
        width: w,
        height: h,
        rgba,
    }
}

/// `beta --guidiff <java dir> <rust dir> [diff dir]`: compare same-named PNGs pixel by pixel.
/// Differences are drawn red over a dimmed copy of the Java image in the diff dir.
pub fn diff(
    java_dir: &std::path::Path,
    rust_dir: &std::path::Path,
    diff_dir: Option<&std::path::Path>,
) -> bool {
    let list = |d: &std::path::Path| -> Vec<String> {
        std::fs::read_dir(d)
            .map(|rd| {
                rd.filter_map(|e| e.ok())
                    .map(|e| e.file_name().to_string_lossy().into_owned())
                    .filter(|n| n.ends_with(".png"))
                    .collect()
            })
            .unwrap_or_default()
    };
    let mut names = list(java_dir);
    names.extend(list(rust_dir));
    names.sort();
    names.dedup();
    let mut all = true;
    let mut near: Vec<String> = Vec::new();
    for n in names {
        let load = |p: std::path::PathBuf| -> Option<Image> {
            let bytes = std::fs::read(p).ok()?;
            Some(crate::assets::decode_png_pub(&bytes))
        };
        let (Some(j), Some(r)) = (load(java_dir.join(&n)), load(rust_dir.join(&n))) else {
            println!("{n}: missing on one side");
            all = false;
            continue;
        };
        if (j.width, j.height) != (r.width, r.height) {
            println!(
                "{n}: size differs ({}x{} vs {}x{})",
                j.width, j.height, r.width, r.height
            );
            all = false;
            continue;
        }
        let mut bad = 0usize;
        let mut max = 0u8;
        let mut first = None;
        let mut out = Image {
            width: j.width,
            height: j.height,
            rgba: j.rgba.clone(),
        };
        for i in 0..(j.width * j.height) as usize {
            let a = &j.rgba[i * 4..i * 4 + 3];
            let b = &r.rgba[i * 4..i * 4 + 3];
            let d = a.iter().zip(b).map(|(x, y)| x.abs_diff(*y)).max().unwrap();
            if d > 0 {
                bad += 1;
                max = max.max(d);
                if first.is_none() {
                    first = Some((
                        i as u32 % j.width,
                        i as u32 / j.width,
                        [a[0], a[1], a[2]],
                        [b[0], b[1], b[2]],
                    ));
                }
                out.rgba[i * 4..i * 4 + 4].copy_from_slice(&[255, 0, 0, 255]);
            } else {
                for c in &mut out.rgba[i * 4..i * 4 + 3] {
                    *c /= 3;
                }
            }
        }
        if bad == 0 {
            println!("{n}: EXACT");
        } else if max <= 1 && bad <= 200 {
            // GPU lighting arithmetic on lit 3D item icons can round a few pixels by one level.
            near.push(n.clone());
            println!("{n}: within one level ({bad} pixels)");
        } else {
            all = false;
            let (x, y, a, b) = first.unwrap();
            println!(
                "{n}: {bad} pixels differ (max channel diff {max}); first at ({x},{y}) java {a:?} rust {b:?}"
            );
            if let Some(d) = diff_dir {
                let _ = std::fs::create_dir_all(d);
                let _ = crate::assets::save_png(&out, &d.join(&n));
            }
        }
    }
    if all && !near.is_empty() {
        println!(
            "(exact except {} scene(s) within one color level)",
            near.len()
        );
    }
    all
}
