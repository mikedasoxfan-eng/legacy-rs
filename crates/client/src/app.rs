//! The game window: `Minecraft`'s main loop as far as this port goes. The title screen and
//! menus run without a world; choosing one starts the world thread, shows the loading
//! screen, then the fly-camera view with the game menu on Esc.

use crate::render::blocks::Colorizers;
use crate::render::gpu::{FrameParams, Gpu};
use crate::render::sky;
use crate::worker::{self, FromWorker, ToWorker, WorldOptions};
use crate::{assets, gui, render, Options};
use glam::DVec3;
use std::collections::HashSet;
use std::sync::Arc;
use std::time::{Duration, Instant};
use winit::application::ApplicationHandler;
use winit::event::{DeviceEvent, ElementState, MouseButton, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{CursorGrabMode, Window, WindowId};
use worldgen::BiomeSource;

struct Camera {
    pos: DVec3,
    yaw: f32,
    pitch: f32,
}

/// LWJGL `Keyboard` code for a physical key (DirectInput scancodes).
fn lwjgl_key(k: KeyCode) -> i32 {
    use KeyCode::*;
    match k {
        Escape => 1,
        Digit1 => 2,
        Digit2 => 3,
        Digit3 => 4,
        Digit4 => 5,
        Digit5 => 6,
        Digit6 => 7,
        Digit7 => 8,
        Digit8 => 9,
        Digit9 => 10,
        Digit0 => 11,
        Minus => 12,
        Equal => 13,
        Backspace => 14,
        Tab => 15,
        KeyQ => 16,
        KeyW => 17,
        KeyE => 18,
        KeyR => 19,
        KeyT => 20,
        KeyY => 21,
        KeyU => 22,
        KeyI => 23,
        KeyO => 24,
        KeyP => 25,
        BracketLeft => 26,
        BracketRight => 27,
        Enter => 28,
        ControlLeft => 29,
        KeyA => 30,
        KeyS => 31,
        KeyD => 32,
        KeyF => 33,
        KeyG => 34,
        KeyH => 35,
        KeyJ => 36,
        KeyK => 37,
        KeyL => 38,
        Semicolon => 39,
        Quote => 40,
        Backquote => 41,
        ShiftLeft => 42,
        Backslash => 43,
        KeyZ => 44,
        KeyX => 45,
        KeyC => 46,
        KeyV => 47,
        KeyB => 48,
        KeyN => 49,
        KeyM => 50,
        Comma => 51,
        Period => 52,
        Slash => 53,
        ShiftRight => 54,
        NumpadMultiply => 55,
        AltLeft => 56,
        Space => 57,
        CapsLock => 58,
        F1 => 59,
        F2 => 60,
        F3 => 61,
        F4 => 62,
        F5 => 63,
        F6 => 64,
        F7 => 65,
        F8 => 66,
        F9 => 67,
        F10 => 68,
        NumLock => 69,
        ScrollLock => 70,
        Numpad7 => 71,
        Numpad8 => 72,
        Numpad9 => 73,
        NumpadSubtract => 74,
        Numpad4 => 75,
        Numpad5 => 76,
        Numpad6 => 77,
        NumpadAdd => 78,
        Numpad1 => 79,
        Numpad2 => 80,
        Numpad3 => 81,
        Numpad0 => 82,
        NumpadDecimal => 83,
        F11 => 87,
        F12 => 88,
        NumpadEnter => 156,
        ControlRight => 157,
        NumpadDivide => 181,
        AltRight => 184,
        Pause => 197,
        Home => 199,
        ArrowUp => 200,
        PageUp => 201,
        ArrowLeft => 203,
        ArrowRight => 205,
        End => 207,
        ArrowDown => 208,
        PageDown => 209,
        Insert => 210,
        Delete => 211,
        SuperLeft => 219,
        SuperRight => 220,
        ContextMenu => 221,
        _ => 0,
    }
}

/// `--flowtest <dir>`: click through title -> new world -> game menu -> quit, saving a
/// screenshot at each step (an end-to-end check of the menu flow).
struct Flow {
    dir: std::path::PathBuf,
    step: u32,
    frames: u32,
    capture: Option<String>,
}

pub struct App {
    opts: Options,
    flow: Option<Flow>,
    gpu: Option<Gpu>,
    gui: Option<gui::system::GuiSystem>,
    col: Option<Colorizers>,
    /// The loading screen while a world opens or saves (`LoadingScreenRenderer`).
    loading: Option<gui::loading::LoadingScreen>,
    /// After the world thread has saved: quit the program (true) or go to the title (false).
    exit_after_save: bool,
    tx: Option<std::sync::mpsc::Sender<ToWorker>>,
    rx: Option<std::sync::mpsc::Receiver<FromWorker>>,
    worker: Option<std::thread::JoinHandle<()>>,
    cam: Camera,
    ready: bool,
    idle: bool,
    keys: HashSet<KeyCode>,
    grabbed: bool,
    /// Cursor position in window pixels (for the GUI).
    cursor: (f64, f64),
    paused: bool,
    /// `GuiIngame` and the player state it draws.
    ingame: gui::ingame::GuiIngame,
    hud: gui::ingame::HudPlayer,
    chunks_loaded: usize,
    /// World ticks seen since the world became ready.
    ticks_ready: i64,
    /// Entities near the camera from the last tick, with their brightness.
    entities: Vec<(worldgen::entity::Entity, f32)>,
    /// Chunk meshes uploaded this second (`WorldRenderer.chunksUpdated`) and the last count.
    chunk_updates: u32,
    debug_line: String,
    started: Instant,
    gui_ticks: Instant,
    world_time: i64,
    /// When the last tick arrived from the world thread (for partial ticks).
    last_tick: Instant,
    last_frame: Instant,
    last_cam_sent: DVec3,
    temps: BiomeSource,
    seed: i64,
    world_name: String,
    fps_frames: u32,
    fps_time: Instant,
    fps: u32,
    fx: Vec<render::fx::TextureFx>,
    fx_rng: render::fx::Rng,
    cloud_ticks: i64,
    weather: render::weather::Weather,
    weather_terrain: render::weather::Snapshot,
    weather_rng: render::weather::ClientRandom,
    rain_sound_counter: i32,
    fog_fade: [f32; 2],
    particles: render::weather::Particles,
    bolts: Vec<(render::weather::Lightning, i32)>,
}

impl App {
    pub fn new(opts: Options) -> App {
        let seed = opts.world.seed;
        let flow = opts.flowtest.clone().map(|dir| Flow { dir, step: 0, frames: 0, capture: None });
        App {
            opts,
            flow,
            gpu: None,
            gui: None,
            col: None,
            loading: None,
            exit_after_save: false,
            tx: None,
            rx: None,
            worker: None,
            cam: Camera { pos: DVec3::new(0.5, 80.0, 0.5), yaw: 0.0, pitch: 0.0 },
            ready: false,
            idle: false,
            keys: HashSet::new(),
            grabbed: false,
            cursor: (0.0, 0.0),
            paused: false,
            ingame: gui::ingame::GuiIngame::new(),
            hud: gui::ingame::HudPlayer::default(),
            chunks_loaded: 0,
            ticks_ready: 0,
            entities: Vec::new(),
            chunk_updates: 0,
            debug_line: String::new(),
            started: Instant::now(),
            gui_ticks: Instant::now(),
            world_time: 0,
            last_tick: Instant::now(),
            last_frame: Instant::now(),
            last_cam_sent: DVec3::splat(f64::MAX),
            temps: BiomeSource::new(seed),
            seed,
            world_name: String::new(),
            fps_frames: 0,
            fps_time: Instant::now(),
            fps: 0,
            fx: render::fx::TextureFx::terrain_set(),
            fx_rng: render::fx::Rng::new(0x9E3779B97F4A7C15),
            cloud_ticks: 0,
            weather: Default::default(),
            weather_terrain: Default::default(),
            weather_rng: render::weather::ClientRandom::new(seed),
            rain_sound_counter: 0,
            fog_fade: [0.0; 2],
            particles: render::weather::Particles::new(seed),
            bolts: Vec::new(),
        }
    }

    fn set_grab(&mut self, on: bool) {
        let Some(gpu) = &self.gpu else { return };
        let w = &gpu.window;
        if on {
            let ok = w.set_cursor_grab(CursorGrabMode::Locked).or_else(|_| w.set_cursor_grab(CursorGrabMode::Confined));
            if ok.is_ok() {
                w.set_cursor_visible(false);
                self.grabbed = true;
            }
        } else {
            let _ = w.set_cursor_grab(CursorGrabMode::None);
            w.set_cursor_visible(true);
            self.grabbed = false;
        }
    }

    /// `Minecraft.startWorld`: open a world on the world thread (the loading screen shows
    /// its progress until it is ready).
    fn start_world(&mut self, w: WorldOptions) {
        let seed = w.seed;
        self.ready = false;
        self.idle = false;
        self.world_time = 0;
        self.last_cam_sent = DVec3::splat(f64::MAX);
        self.temps = BiomeSource::new(seed);
        self.seed = seed;
        self.fx = render::fx::TextureFx::terrain_set();
        self.cloud_ticks = 0;
        self.weather = Default::default();
        self.weather_terrain = Default::default();
        self.weather_rng = render::weather::ClientRandom::new(seed);
        self.rain_sound_counter = 0;
        self.fog_fade = [0.0; 2];
        self.particles = render::weather::Particles::new(seed);
        self.bolts.clear();
        self.paused = false;
        if let Some((x, y, z, yaw, pitch)) = self.opts.at {
            self.cam = Camera { pos: DVec3::new(x, y, z), yaw, pitch };
        }
        self.loading = Some(gui::loading::LoadingScreen { title: String::new(), text: String::new(), progress: -1 });
        let col = self.col.clone().expect("colorizers");
        let (tx, rx, h) = worker::spawn(w, col);
        self.tx = Some(tx);
        self.rx = Some(rx);
        self.worker = Some(h);
    }

    /// Leave the world: the world thread saves (shown as "Saving chunks"), then the title
    /// screen (or the program exits).
    fn quit_world(&mut self, exit: bool) {
        self.exit_after_save = exit;
        if let Some(tx) = &self.tx {
            let _ = tx.send(ToWorker::Quit);
            self.loading = Some(gui::loading::LoadingScreen { title: String::new(), text: String::new(), progress: -1 });
        }
        if let Some(g) = &mut self.gui {
            g.screen = None;
        }
        self.set_grab(false);
    }

    fn world_closed(&mut self, el: &ActiveEventLoop) {
        self.tx = None;
        self.rx = None;
        if let Some(h) = self.worker.take() {
            let _ = h.join();
        }
        self.ready = false;
        self.loading = None;
        self.entities.clear();
        if let Some(g) = &mut self.gpu {
            g.clear_chunks();
            g.entities.states.clear();
        }
        if self.exit_after_save {
            el.exit();
            return;
        }
        if let Some(gui) = &mut self.gui {
            gui.ui.world_open = false;
            let mut acts = Vec::new();
            let menu = gui.main_menu();
            gui.display_gui_screen(Some(menu), &mut acts);
        }
    }

    /// Close the window: save the world first if one is open.
    fn shutdown(&mut self, el: &ActiveEventLoop) {
        if self.tx.is_some() {
            if let Some(g) = &self.gpu {
                g.window.set_title("Minecraft Beta 1.7.3 (Rust) — saving world...");
            }
            self.quit_world(true);
            // Wait here: the window is closing.
            while let Some(rx) = &self.rx {
                match rx.recv() {
                    Ok(FromWorker::Saved) | Err(_) => break,
                    Ok(_) => {}
                }
            }
            self.world_closed(el);
        }
        el.exit();
    }

    fn apply_gui_actions(&mut self, el: &ActiveEventLoop) {
        let Some(gui) = &mut self.gui else { return };
        let acts = gui.process_actions();
        for a in acts {
            match a {
                gui::system::GameAction::StartWorld { folder, name, seed } => {
                    let saves = self.gui.as_ref().unwrap().ui.saves_dir.clone();
                    let seed = seed.unwrap_or(0);
                    let w = WorldOptions {
                        dir: Some(saves.join(&folder)),
                        seed,
                        // World.rand is unseeded in vanilla.
                        world_rand: seed ^ 0x5DEECE66D ^ self.started.elapsed().as_nanos() as i64,
                        server: self.opts.world.server,
                        view_distance: self.opts.world.view_distance,
                        weather: None,
                        time: None,
                        name: Some(name),
                    };
                    self.start_world(w);
                }
                gui::system::GameAction::QuitWorld => self.quit_world(false),
                gui::system::GameAction::Shutdown => {
                    self.shutdown(el);
                    return;
                }
                gui::system::GameAction::SettingsChanged => {}
                gui::system::GameAction::IngameFocus => {
                    if self.ready {
                        self.set_grab(true);
                    }
                }
                gui::system::GameAction::IngameUnfocus => self.set_grab(false),
            }
        }
    }

    /// Press a button of the current screen like the mouse would.
    fn click_button(&mut self, id: i32) {
        let Some(g) = &mut self.gui else { return };
        let Some(s) = &mut g.screen else { return };
        let Some(b) = s.base().buttons.iter().find(|b| b.id == id).cloned() else { return };
        let (x, y) = (b.x + b.width / 2, b.y + b.height / 2);
        let sr = g.ui.scaled();
        self.cursor = (
            (x as f64 + 0.5) * g.ui.display_width as f64 / sr.width as f64,
            (y as f64 + 0.5) * g.ui.display_height as f64 / sr.height as f64,
        );
        g.mouse_event(x, y, 0, true);
        g.mouse_event(x, y, 0, false);
    }

    fn has_button(&mut self, id: i32) -> bool {
        self.gui.as_mut().and_then(|g| g.screen.as_mut()).is_some_and(|s| s.base().buttons.iter().any(|b| b.id == id))
    }

    /// Request a flow screenshot and move to the next step.
    fn flow_next(&mut self, shot: &str) {
        let f = self.flow.as_mut().unwrap();
        f.capture = Some(shot.to_string());
        f.step += 1;
        f.frames = 0;
    }

    /// One step of `--flowtest` per frame.
    fn flow_step(&mut self, el: &ActiveEventLoop) {
        let Some(f) = &mut self.flow else { return };
        f.frames += 1;
        let (step, frames) = (f.step, f.frames);
        match step {
            // Title screen (Singleplayer = 1).
            0 if frames > 10 && !self.ready && self.has_button(1) => self.flow_next("1_title"),
            1 if frames > 2 => {
                self.click_button(1);
                self.flow_next("")
            }
            // Select world (Create New World = 3).
            2 if frames > 10 && self.has_button(3) => self.flow_next("2_select"),
            3 if frames > 2 => {
                self.click_button(3);
                self.flow_next("")
            }
            // Create world (Create = 0).
            4 if frames > 10 && self.has_button(0) => self.flow_next("3_create"),
            5 if frames > 2 => {
                self.click_button(0);
                self.flow_next("")
            }
            6 if self.loading.as_ref().is_some_and(|l| l.progress >= 40) => self.flow_next("4_loading"),
            // Idle, or meshed enough (flowing water can keep chunks changing).
            7 if self.ready && self.loading.is_none() && (self.idle || frames > 400) => self.flow_next("5_ingame"),
            8 if frames > 2 => {
                self.display_ingame_menu();
                self.flow_next("")
            }
            // Game menu (Save and quit = 1), after its "Saving level.." note has gone.
            9 if frames > 40 => self.flow_next("6_game_menu"),
            10 if frames > 2 => {
                self.click_button(1);
                self.flow_next("")
            }
            11 if frames > 10 && !self.ready && self.loading.is_none() && self.has_button(1) => self.flow_next("7_title_again"),
            12 if frames > 2 => {
                self.click_button(1);
                self.flow_next("")
            }
            13 if frames > 10 && self.has_button(3) => self.flow_next("8_select_with_world"),
            14 if frames > 2 => {
                eprintln!("flowtest: done");
                self.shutdown(el);
                return;
            }
            _ if frames > 3000 => {
                eprintln!(
                    "flowtest: stuck at step {step} (ready {}, idle {}, loading {:?}, screen {})",
                    self.ready,
                    self.idle,
                    self.loading.as_ref().map(|l| (&l.title, &l.text, l.progress)),
                    self.gui.as_ref().is_some_and(|g| g.screen.is_some())
                );
                self.shutdown(el);
                return;
            }
            _ => {}
        }
        self.apply_gui_actions(el);
    }

    fn flow_capture(&mut self) -> Option<(std::path::PathBuf, String)> {
        let f = self.flow.as_mut()?;
        let name = f.capture.take().filter(|n| !n.is_empty())?;
        Some((f.dir.clone(), name))
    }

    /// `Minecraft.displayInGameMenu`
    fn display_ingame_menu(&mut self) {
        if let Some(gui) = &mut self.gui {
            if gui.screen.is_none() {
                gui.ui.display(Some(Box::new(gui::screens::ingame_menu::GuiIngameMenu::new())));
            }
        }
    }

    fn frame(&mut self, el: &ActiveEventLoop) {
        self.flow_step(el);
        let flow_shot = self.flow_capture();
        let now = Instant::now();
        let dt = (now - self.last_frame).as_secs_f64().min(0.25);
        self.last_frame = now;

        // GuiScreen.updateScreen runs every game tick (20 per second).
        let mut gui_ticks = 0;
        while now >= self.gui_ticks && gui_ticks < 20 {
            self.ingame.update_tick();
            if let Some(g) = &mut self.gui {
                g.update_screen();
            }
            self.gui_ticks += Duration::from_millis(50);
            gui_ticks += 1;
        }
        if gui_ticks == 20 {
            self.gui_ticks = now;
        }
        if let Some(g) = &mut self.gui {
            g.ui.millis = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_millis() as i64);
        }
        self.apply_gui_actions(el);

        // World messages: upload a bounded number of meshes per frame.
        let mut uploads = 0;
        let mut new_ticks = 0;
        let mut closed = false;
        if let (Some(rx), Some(gpu)) = (&self.rx, &mut self.gpu) {
            while uploads < 64 {
                match rx.try_recv() {
                    Ok(FromWorker::Ready { spawn, seed, name }) => {
                        self.ready = true;
                        self.seed = seed;
                        self.temps = BiomeSource::new(seed);
                        self.world_name = name;
                        if self.opts.at.is_none() {
                            self.cam.pos = DVec3::new(spawn.0, spawn.1, spawn.2);
                        }
                        self.loading = None;
                        if let Some(gui) = &mut self.gui {
                            gui.ui.world_open = true;
                            // startWorld ends with displayGuiScreen(null): into the game.
                            gui.ui.display(None);
                        }
                    }
                    Ok(FromWorker::Loading(l)) => {
                        let ls = self.loading.get_or_insert_with(Default::default);
                        match l {
                            worker::Loading::Title(t) => ls.title = t,
                            worker::Loading::Text(t) => {
                                ls.text = t;
                                ls.progress = -1;
                            }
                            worker::Loading::Progress(p) => ls.progress = p,
                        }
                    }
                    Ok(FromWorker::Entities(list)) => self.entities = list,
                    Ok(FromWorker::QuickSaved(done)) => {
                        if let Some(gui) = &mut self.gui {
                            gui.ui.world_saved = done;
                        }
                    }
                    Ok(FromWorker::Mesh { cx, cz, opaque, translucent }) => {
                        gpu.upload_chunk(cx, cz, &opaque, &translucent);
                        uploads += 1;
                        self.chunk_updates += 1;
                        self.idle = false;
                    }
                    Ok(FromWorker::Drop { cx, cz }) => gpu.remove_chunk(cx, cz),
                    Ok(FromWorker::Time { time, brightness, chunks, weather, terrain, bolts }) => {
                        new_ticks += 1;
                        if self.ready {
                            self.ticks_ready += 1;
                        }
                        self.hud.brightness = brightness;
                        self.chunks_loaded = chunks;
                        self.cloud_ticks += 1;
                        self.fog_fade[0] = self.fog_fade[1];
                        // EntityRenderer.updateRenderer: on Far, (3-renderDistance)/3=1.
                        self.fog_fade[1] += (1.0f32 - self.fog_fade[1]) * 0.1f32;
                        self.world_time = time;
                        self.weather = weather;
                        self.weather_terrain = terrain;
                        self.last_tick = now;
                        for (_, life) in &mut self.bolts {
                            *life -= 1;
                        }
                        self.bolts.retain(|(_, life)| *life > 0);
                        self.bolts.extend(bolts.into_iter().map(|b| (b, 3)));
                        let (splashes, _) = render::weather::add_rain_particles(
                            &self.weather_terrain,
                            self.cam.pos.to_array(),
                            self.cloud_ticks as i32,
                            weather.rain_at(1.0),
                            &mut self.weather_rng,
                            &mut self.rain_sound_counter,
                        );
                        self.particles.spawn(splashes);
                        self.particles.tick(&self.weather_terrain);
                    }
                    Ok(FromWorker::Idle) => self.idle = true,
                    Ok(FromWorker::Saved) => {
                        closed = true;
                        break;
                    }
                    Ok(FromWorker::Error(e)) => {
                        eprintln!("{e}");
                        closed = true;
                        break;
                    }
                    Err(_) => break,
                }
            }
        }
        if closed {
            self.world_closed(el);
            return;
        }
        self.apply_gui_actions(el);

        // The pause menu saves a few chunks per frame; screens that pause stop the ticks.
        if let (Some(gui), Some(tx)) = (&mut self.gui, &self.tx) {
            for n in gui.ui.save_requests.drain(..) {
                let _ = tx.send(ToWorker::QuickSave(n));
            }
            let pause = gui.screen.as_ref().is_some_and(|s| s.does_gui_pause_game());
            if pause != self.paused {
                self.paused = pause;
                let _ = tx.send(ToWorker::Pause(pause));
            }
        }
        let in_game = self.ready && self.gui.as_ref().is_some_and(|g| g.screen.is_none());

        // Fly camera (yaw 0 faces +Z, like the original).
        if in_game {
            let mut speed = 10.0;
            if self.keys.contains(&KeyCode::ControlLeft) {
                speed = 40.0;
            }
            let (sy, cy) = (self.cam.yaw.to_radians() as f64).sin_cos();
            let fwd = DVec3::new(-sy, 0.0, cy);
            let right = DVec3::new(-cy, 0.0, -sy);
            let mut mv = DVec3::ZERO;
            if self.keys.contains(&KeyCode::KeyW) {
                mv += fwd;
            }
            if self.keys.contains(&KeyCode::KeyS) {
                mv -= fwd;
            }
            if self.keys.contains(&KeyCode::KeyD) {
                mv += right;
            }
            if self.keys.contains(&KeyCode::KeyA) {
                mv -= right;
            }
            if self.keys.contains(&KeyCode::Space) {
                mv.y += 1.0;
            }
            if self.keys.contains(&KeyCode::ShiftLeft) {
                mv.y -= 1.0;
            }
            if mv.length_squared() > 0.0 {
                self.cam.pos += mv.normalize() * speed * dt;
            }
        }
        if self.ready && self.cam.pos.distance_squared(self.last_cam_sent) > 1e-4 {
            self.last_cam_sent = self.cam.pos;
            if let Some(tx) = &self.tx {
                let p = self.cam.pos;
                let _ = tx.send(ToWorker::Camera { x: p.x, y: p.y, z: p.z });
            }
        }

        // Per tick: animated textures and cloud drift.
        for _ in 0..new_ticks {
            for f in &mut self.fx {
                f.tick(&mut self.fx_rng);
            }
        }
        if new_ticks > 0 {
            if let Some(g) = &self.gpu {
                g.update_fx(&self.fx);
            }
        }
        let partial = if self.paused { 0.0 } else { ((now - self.last_tick).as_secs_f32() * 20.0).min(1.0) };
        let gui_partial = ((now + Duration::from_millis(50)).saturating_duration_since(self.gui_ticks).as_secs_f32() * 20.0).clamp(0.0, 1.0);

        // Menus or the loading screen, no world drawn.
        if !self.ready || self.loading.is_some() {
            let (Some(gpu), Some(gui)) = (&mut self.gpu, &mut self.gui) else { return };
            let loading = self.loading.clone();
            let cursor = self.cursor;
            let img = gpu.render_overlay_only(flow_shot.is_some(), &mut |device, queue, view, size| {
                gui.ui.gl.begin_frame();
                if let Some(l) = &loading {
                    l.draw(&mut gui.ui);
                } else {
                    let (mx, my) = gui.gui_mouse(cursor.0, cursor.1);
                    gui.draw_screen(mx, my, gui_partial);
                }
                gui.renderer.render(device, queue, view, size, &gui.ui.gl, Some([0.0, 0.0, 0.0, 0.0]));
            });
            if let (Some(img), Some((dir, name))) = (img, &flow_shot) {
                let _ = assets::save_png(&img, &dir.join(format!("{name}.png")));
            }
            self.apply_gui_actions(el);
            return;
        }

        let angle = sky::celestial_angle(self.world_time, partial);
        let temp = self.temps.raw_temperature(self.cam.pos.x.floor() as i32, self.cam.pos.z.floor() as i32) as f32;
        let rain = self.weather.rain_at(partial);
        let thunder = self.weather.thunder_at(partial);
        let precipitation = render::weather::precipitation(
            &self.weather_terrain,
            self.cam.pos.to_array(),
            self.cloud_ticks as i32,
            partial,
            rain,
            &mut self.weather_rng,
        );
        let lightning = self
            .bolts
            .iter()
            .map(|(b, _)| *b)
            .chain(self.opts.bolt)
            .flat_map(|b| render::weather::lightning(b, self.cam.pos.to_array()))
            .collect();
        let particles = self.particles.vertices(
            &self.weather_terrain,
            self.cam.pos.to_array(),
            self.cam.yaw,
            self.cam.pitch,
            partial,
        );
        let params = FrameParams {
            cam_pos: self.cam.pos,
            yaw: self.cam.yaw,
            pitch: self.cam.pitch,
            fov_y_deg: 70.0,
            celestial: angle,
            sky_color: sky::sky_color(angle, temp, rain, thunder, self.weather.flash, partial),
            fog_color: sky::fog_color(angle, temp, 0, rain, thunder, self.weather.flash, partial)
                .map(|c| c * (self.fog_fade[0] + (self.fog_fade[1] - self.fog_fade[0]) * partial)),
            far: 256.0,
            cloud_offset: self.cloud_ticks as f32 + partial,
            rain,
            thunder,
            precipitation,
            lightning,
            particles,
        };

        if let Some(gpu) = &mut self.gpu {
            gpu.entities.states = self
                .entities
                .iter()
                .filter_map(|(e, b)| render::entity::bridge::render_state(e, partial, *b))
                .collect();
        }
        let want_shot =
            self.opts.shot.is_some()
                && self.ready
                && uploads == 0
                // Ticking keeps changing blocks somewhere, so a timed shot doesn't wait for idle.
                && if self.opts.shot_ticks > 0 { self.ticks_ready >= self.opts.shot_ticks } else { self.idle };
        let capture = want_shot || self.keys.remove(&KeyCode::F2) || flow_shot.is_some();
        let (Some(gpu), Some(gui)) = (&mut self.gpu, &mut self.gui) else { return };
        let cursor = self.cursor;
        let ingame = &mut self.ingame;
        let mut hud = self.hud.clone();
        hud.pos = [self.cam.pos.x, self.cam.pos.y, self.cam.pos.z];
        hud.yaw = self.cam.yaw;
        let debug = gui::ingame::DebugInfo {
            fps_line: self.debug_line.clone(),
            renders: format!("C: {}/{}. F: 0, O: 0, E: 0", gpu.chunk_count(), gpu.chunk_count()),
            entities: "E: 0/0. B: 0, I: 0".into(),
            particles: format!("P: {}. T: All: 0", self.particles.len()),
            chunks: format!("ServerChunkCache: {} Drop: 0", self.chunks_loaded),
            ..memory_info()
        };
        let shot = gpu.render_with_overlay(&params, capture, &mut |device, queue, view, size| {
            let (mx, my) = gui.gui_mouse(cursor.0, cursor.1);
            let screen_open = gui.screen.is_some();
            // EntityRenderer: the game overlay (unless F1), then the screen on a cleared depth.
            if !gui.ui.settings.hide_gui || screen_open {
                gui.ui.gl.begin_frame();
                let gl = &mut gui.ui.gl;
                gl.texture_2d = true;
                gl.alpha_test = true;
                gl.depth_test = true;
                gl.color4f(1.0, 1.0, 1.0, 1.0);
                ingame.render_game_overlay(&mut gui.ui, gui_partial, false, &hud, &debug);
                gui.renderer.render(device, queue, view, size, &gui.ui.gl, None);
            }
            if screen_open {
                gui.ui.gl.begin_frame();
                gui.draw_screen(mx, my, gui_partial);
                gui.renderer.render(device, queue, view, size, &gui.ui.gl, None);
            }
        });
        if let (Some(img), Some((dir, name))) = (&shot, &flow_shot) {
            let _ = assets::save_png(img, &dir.join(format!("{name}.png")));
        }
        if let Some(img) = shot.filter(|_| flow_shot.is_none()) {
            let path = self.opts.shot.clone().unwrap_or_else(|| {
                let t = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
                format!("screenshot-{t}.png").into()
            });
            match assets::save_png(&img, &path) {
                Ok(()) => eprintln!("saved {}", path.display()),
                Err(e) => eprintln!("screenshot failed: {e}"),
            }
            if want_shot {
                self.shutdown(el);
                return;
            }
        }
        self.apply_gui_actions(el);

        self.fps_frames += 1;
        if self.fps_time.elapsed().as_secs_f64() >= 1.0 {
            self.fps = self.fps_frames;
            self.fps_frames = 0;
            self.debug_line = format!("{} fps, {} chunk updates", self.fps, self.chunk_updates);
            self.chunk_updates = 0;
            let Some(gpu) = &self.gpu else { return };
            if std::env::var_os("BETA_FPS").is_some() {
                eprintln!("fps {} chunks {}", self.fps, gpu.chunk_count());
            }
            self.fps_time = Instant::now();
            let p = self.cam.pos;
            gpu.window.set_title(&format!(
                "Minecraft Beta 1.7.3 (Rust) — {} (seed {}) — {} fps — x {:.1} y {:.1} z {:.1} — {} chunks{}",
                self.world_name,
                self.seed,
                self.fps,
                p.x,
                p.y,
                p.z,
                gpu.chunk_count(),
                if self.opts.world.server { " — server mode" } else { "" },
            ));
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, el: &ActiveEventLoop) {
        if self.gpu.is_some() {
            return;
        }
        let assets = match assets::load() {
            Ok(a) => a,
            Err(e) => {
                eprintln!("{e}");
                el.exit();
                return;
            }
        };
        let attrs = Window::default_attributes()
            .with_title("Minecraft Beta 1.7.3 (Rust)")
            .with_inner_size(winit::dpi::LogicalSize::new(854.0, 480.0));
        let window = Arc::new(el.create_window(attrs).expect("create window"));
        let mut gpu = Gpu::new(
            window,
            &assets.terrain,
            &assets.sun,
            &assets.moon,
            &assets.clouds,
            [&assets.snow, &assets.rain, &assets.particles],
            &assets.mobs,
        );
        let (images, font_txt, lang, splashes) = match assets::Jar::open().and_then(|mut j| gui::load_resources(&mut j)) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("{e}");
                el.exit();
                return;
            }
        };
        let (w, h) = gpu.size();
        let data_dir = match &self.flow {
            Some(f) => {
                let d = f.dir.join("data");
                let _ = std::fs::create_dir_all(&d);
                d
            }
            None => std::env::current_dir().unwrap_or_default(),
        };
        let settings = gui::settings::GameSettings::load(&data_dir);
        let splash_seed = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_nanos() as i64);
        let mut gui = gui::system::GuiSystem::new(
            gpu.device(),
            gpu.queue(),
            gpu.surface_format(),
            gui::system::Resources { images, font_txt, lang, splashes },
            settings,
            (w as i32, h as i32),
            data_dir.join("saves"),
            splash_seed,
            1,
        );
        // Minecraft.loadScreen: the Mojang logo while starting up.
        gpu.render_overlay_only(false, &mut |device, queue, view, size| {
            gui.ui.gl.begin_frame();
            gui.draw_mojang_logo();
            gui.renderer.render(device, queue, view, size, &gui.ui.gl, Some([0.0, 0.0, 0.0, 0.0]));
        });
        self.col = Some(Colorizers { grass: assets.grass_color, foliage: assets.foliage_color });
        self.gpu = Some(gpu);
        if self.opts.direct {
            // A world named on the command line: straight in, like the old viewer.
            self.gui = Some(gui);
            let w = self.opts.world.clone();
            self.start_world(w);
        } else {
            let mut acts = Vec::new();
            let menu = gui.main_menu();
            gui.display_gui_screen(Some(menu), &mut acts);
            self.gui = Some(gui);
        }
    }

    fn window_event(&mut self, el: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => self.shutdown(el),
            WindowEvent::Resized(s) => {
                if let Some(g) = &mut self.gpu {
                    g.resize(s.width, s.height);
                }
                if let Some(g) = &mut self.gui {
                    g.resize(s.width as i32, s.height as i32);
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.cursor = (position.x, position.y);
                if let Some(g) = &mut self.gui {
                    if g.screen.is_some() {
                        // LWJGL reports moves as button -1 events (mouseMovedOrUp).
                        let (x, y) = g.gui_mouse(position.x, position.y);
                        g.mouse_event(x, y, -1, false);
                    }
                }
            }
            WindowEvent::MouseInput { state, button, .. } => {
                let b = match button {
                    MouseButton::Left => 0,
                    MouseButton::Right => 1,
                    MouseButton::Middle => 2,
                    _ => return,
                };
                let pressed = state == ElementState::Pressed;
                let screen_open = self.gui.as_ref().is_some_and(|g| g.screen.is_some());
                if screen_open && self.loading.is_none() {
                    let cursor = self.cursor;
                    if let Some(g) = &mut self.gui {
                        if b == 0 {
                            g.ui.mouse_down = pressed;
                        }
                        let (x, y) = g.gui_mouse(cursor.0, cursor.1);
                        g.mouse_event(x, y, b, pressed);
                    }
                    self.apply_gui_actions(el);
                } else if pressed && self.ready && !self.grabbed {
                    // Clicking into the game window takes the mouse (setIngameFocus).
                    self.set_grab(true);
                }
            }
            WindowEvent::Focused(false) => {
                if self.ready && self.loading.is_none() {
                    // EntityRenderer: losing focus opens the pause menu.
                    self.display_ingame_menu();
                    self.apply_gui_actions(el);
                }
                self.set_grab(false);
            }
            WindowEvent::KeyboardInput { event, .. } => {
                let PhysicalKey::Code(code) = event.physical_key else { return };
                let screen_open = self.gui.as_ref().is_some_and(|g| g.screen.is_some());
                match event.state {
                    ElementState::Pressed => {
                        if let Some(g) = &mut self.gui {
                            g.ui.shift_down = self.keys.contains(&KeyCode::ShiftLeft) || self.keys.contains(&KeyCode::ShiftRight);
                        }
                        if screen_open && self.loading.is_none() {
                            let c = event.text.as_ref().and_then(|t| t.chars().next()).unwrap_or('\0');
                            let key = lwjgl_key(code);
                            if let Some(g) = &mut self.gui {
                                g.key_event(c, key);
                            }
                            self.apply_gui_actions(el);
                        } else if self.ready && self.loading.is_none() {
                            match code {
                                KeyCode::Escape => {
                                    self.display_ingame_menu();
                                    self.apply_gui_actions(el);
                                }
                                KeyCode::F1 => {
                                    if let Some(g) = &mut self.gui {
                                        g.ui.settings.hide_gui = !g.ui.settings.hide_gui;
                                    }
                                }
                                KeyCode::F3 => {
                                    if let Some(g) = &mut self.gui {
                                        g.ui.settings.show_debug_info = !g.ui.settings.show_debug_info;
                                    }
                                }
                                KeyCode::KeyT if !event.repeat => {
                                    if let Some(tx) = &self.tx {
                                        let _ = tx.send(ToWorker::AddTime(1000));
                                    }
                                }
                                _ => {}
                            }
                        }
                        self.keys.insert(code);
                    }
                    ElementState::Released => {
                        if code != KeyCode::F2 {
                            self.keys.remove(&code);
                        }
                    }
                }
            }
            WindowEvent::RedrawRequested => {
                self.frame(el);
                if let Some(g) = &self.gpu {
                    g.window.request_redraw();
                }
            }
            _ => {}
        }
    }

    fn device_event(&mut self, _el: &ActiveEventLoop, _id: winit::event::DeviceId, event: DeviceEvent) {
        if let DeviceEvent::MouseMotion { delta } = event {
            if self.grabbed {
                // Default sensitivity: 0.15 degrees per mouse count, as in EntityPlayerSP.setAngles.
                self.cam.yaw += delta.0 as f32 * 0.15;
                self.cam.pitch = (self.cam.pitch + delta.1 as f32 * 0.15).clamp(-90.0, 90.0);
            }
        }
    }
}

/// The F3 memory lines' numbers. The original shows the Java heap; here: this process's
/// working set out of a quarter of physical memory (the JVM's default maximum heap).
fn memory_info() -> gui::ingame::DebugInfo {
    use windows_sys::Win32::System::ProcessStatus::{GetProcessMemoryInfo, PROCESS_MEMORY_COUNTERS};
    use windows_sys::Win32::System::SystemInformation::{GlobalMemoryStatusEx, MEMORYSTATUSEX};
    use windows_sys::Win32::System::Threading::GetCurrentProcess;
    let mut pmc: PROCESS_MEMORY_COUNTERS = unsafe { std::mem::zeroed() };
    pmc.cb = std::mem::size_of::<PROCESS_MEMORY_COUNTERS>() as u32;
    let mut ms: MEMORYSTATUSEX = unsafe { std::mem::zeroed() };
    ms.dwLength = std::mem::size_of::<MEMORYSTATUSEX>() as u32;
    // SAFETY: both structs are sized and owned here; the handle is the current process.
    let ok = unsafe { GetProcessMemoryInfo(GetCurrentProcess(), &mut pmc, pmc.cb) != 0 && GlobalMemoryStatusEx(&mut ms) != 0 };
    if !ok {
        return Default::default();
    }
    let total = pmc.PagefileUsage as i64;
    let used = pmc.WorkingSetSize as i64;
    gui::ingame::DebugInfo { max_memory: (ms.ullTotalPhys / 4) as i64, total_memory: total.max(used), free_memory: (total - used).max(0), ..Default::default() }
}
