//! The game window: `Minecraft`'s main loop as far as this port goes. The title screen and
//! menus run without a world; choosing one starts the world thread, shows the loading
//! screen, then the fly-camera view with the game menu on Esc.

#[path = "app_block_tiles.rs"]
mod block_tiles;
#[path = "client_textures.rs"]
mod client_textures;

use crate::render::blocks::Colorizers;
use crate::render::gpu::{FrameParams, Gpu};
use crate::render::sky;
use crate::render::video::VideoSettings;
use crate::worker::{self, FromWorker, ToWorker, WorldOptions};
use crate::{assets, gui, render, Options};
use glam::DVec3;
use std::collections::HashSet;
use std::sync::Arc;
use std::time::{Duration, Instant};
use winit::application::ApplicationHandler;
use winit::event::{DeviceEvent, ElementState, MouseButton, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{CursorGrabMode, Window, WindowId};
use worldgen::BiomeSource;

type PlayerView = render::player_view::View;

/// `StatList.startGameStat`, `createWorldStat`, `loadWorldStat`
const STAT_START_GAME: i32 = 1000;
const STAT_CREATE_WORLD: i32 = 1001;
const STAT_LOAD_WORLD: i32 = 1002;


/// `Entity.setAngles` on (yaw, pitch, prevYaw, prevPitch).
fn set_angles(look: &mut [f32; 4], yaw: f32, pitch: f32) {
    let (old_yaw, old_pitch) = (look[0], look[1]);
    look[0] = (look[0] as f64 + yaw as f64 * 0.15) as f32;
    look[1] = ((look[1] as f64 - pitch as f64 * 0.15) as f32).clamp(-90.0, 90.0);
    look[3] += look[1] - old_pitch;
    look[2] += look[0] - old_yaw;
}

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

struct VideoFlow {
    dir: std::path::PathBuf,
    stage: usize,
    frames: u32,
    waiting: bool,
    reset: bool,
}

const VIDEO_CASES: [(i32, bool, bool, i32); 5] =
    [(0, true, true, 1), (3, false, false, 2), (1, true, true, 1), (2, false, true, 0), (0, true, false, 1)];

pub struct App {
    opts: Options,
    flow: Option<Flow>,
    video_flow: Option<VideoFlow>,
    gpu: Option<Gpu>,
    gui: Option<gui::system::GuiSystem>,
    col: Option<Colorizers>,
    /// The loading screen while a world opens or saves (`LoadingScreenRenderer`).
    loading: Option<gui::loading::LoadingScreen>,
    /// After the world thread has saved: quit the program (true) or go to the title (false).
    exit_after_save: bool,
    world_dir: Option<std::path::PathBuf>,
    delete_hardcore_after_close: bool,
    /// The world thread dropped the world over a session-lock conflict.
    session_conflict: bool,
    /// `mcDataDir`: options.txt, saves/, stats/, screenshots/.
    data_dir: std::path::PathBuf,
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
    loaded_entities: usize,
    /// World ticks seen since the world became ready.
    ticks_ready: i64,
    sound: Option<crate::sound::SoundManager>,
    /// Entities near the camera from the last tick, with their brightness.
    entities: Vec<render::entity::effects::Snapshot>,
    pistons: Vec<render::pistons::Snapshot>,
    /// Chunk meshes uploaded this second (`WorldRenderer.chunksUpdated`) and the last count.
    chunk_updates: u32,
    debug_line: String,
    started: Instant,
    gui_ticks: Instant,
    world_time: i64,
    /// When the last tick arrived from the world thread (for partial ticks).
    last_tick: Instant,
    /// This tick's partial-tick clock was already restarted by the player update.
    tick_started_by_player: bool,
    last_frame: Instant,
    frame_deadline: Option<Instant>,
    last_cam_sent: DVec3,
    temps: BiomeSource,
    seed: i64,
    world_name: String,
    fps_frames: u32,
    fps_time: Instant,
    fps: u32,
    fx: Vec<render::fx::TextureFx>,
    item_fx: Option<render::item_fx::ItemFx>,
    fx_rng: render::fx::Rng,
    cloud_ticks: i64,
    /// EntityRenderer torch flicker (b1.8.1 lightmap), and its Math.random source.
    flicker: render::lightmap::Flicker,
    flicker_rng: worldgen::random::JavaRandom,
    weather: render::weather::Weather,
    weather_terrain: render::weather::Snapshot,
    weather_rng: render::weather::ClientRandom,
    rain_sound_counter: i32,
    fog_fade: [f32; 2],
    particles: crate::world_effects::Effects,
    /// Singleplayer: the player the world thread simulates.
    player: Option<PlayerView>,
    /// Button and key presses since the last input frame went to the world thread.
    input: worker::InputFrame,
    minecraft_events: crate::minecraft_input::Events,
    minecraft_system_time: Option<Instant>,
    /// `MouseHelper` counts this frame (y up, as LWJGL reports).
    mouse_delta: (f64, f64),
    mouse_held: [bool; 2],
    /// `MovementInputFromOptions.movementKeyStates`: set by key events, cleared when a screen
    /// takes the focus (`resetPlayerKeyState`).
    move_keys: [bool; 6],
    /// `EntityRenderer`'s smooth (cinematic) camera state.
    smooth_camera: render::camera::SmoothCamera,
    input_seq: u64,
    /// Look deltas the world thread may not have applied yet when its last message left.
    pending_look: Vec<(u64, f32, f32)>,
    /// The player's rotation as this frame sees it: yaw, pitch, prevYaw, prevPitch.
    look: [f32; 4],
    /// Respawns asked for; older player messages are ignored.
    life: u32,
    script: Option<crate::playscript::Script>,
    /// 20 Hz steps since the world became ready, paused or not (the script's clock).
    script_ticks: i64,
    /// The player is in the Nether (`worldProvider.isNether`).
    nether: bool,
}

impl App {
    pub fn new(opts: Options) -> App {
        let seed = opts.world.seed;
        let flow = opts.flowtest.clone().map(|dir| Flow { dir, step: 0, frames: 0, capture: None });
        let video_flow =
            opts.video_flowtest.clone().map(|dir| VideoFlow { dir, stage: 0, frames: 0, waiting: false, reset: false });
        App {
            opts,
            flow,
            video_flow,
            gpu: None,
            gui: None,
            col: None,
            loading: None,
            exit_after_save: false,
            world_dir: None,
            delete_hardcore_after_close: false,
            session_conflict: false,
            data_dir: std::path::PathBuf::new(),
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
            loaded_entities: 0,
            ticks_ready: 0,
            entities: Vec::new(),
            pistons: Vec::new(),
            sound: None,
            chunk_updates: 0,
            debug_line: String::new(),
            started: Instant::now(),
            gui_ticks: Instant::now(),
            world_time: 0,
            last_tick: Instant::now(),
            tick_started_by_player: false,
            last_frame: Instant::now(),
            frame_deadline: None,
            last_cam_sent: DVec3::splat(f64::MAX),
            temps: BiomeSource::new(seed),
            seed,
            world_name: String::new(),
            player: None,
            input: Default::default(),
            minecraft_events: Default::default(),
            minecraft_system_time: None,
            mouse_delta: (0.0, 0.0),
            mouse_held: [false; 2],
            move_keys: [false; 6],
            smooth_camera: Default::default(),
            input_seq: 0,
            pending_look: Vec::new(),
            look: [0.0; 4],
            life: 0,
            script: None,
            script_ticks: 0,
            nether: false,
            fps_frames: 0,
            fps_time: Instant::now(),
            fps: 0,
            fx: render::fx::TextureFx::terrain_set(),
            item_fx: None,
            fx_rng: render::fx::Rng::new(0x9E3779B97F4A7C15),
            cloud_ticks: 0,
            flicker: Default::default(),
            flicker_rng: worldgen::random::JavaRandom::new(std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_nanos() as i64)),
            weather: Default::default(),
            weather_terrain: Default::default(),
            weather_rng: render::weather::ClientRandom::new(seed),
            rain_sound_counter: 0,
            fog_fade: [0.0; 2],
            particles: crate::world_effects::Effects::new(seed),
        }
    }

    fn video_settings(&self) -> VideoSettings {
        self.gui.as_ref().map_or_else(VideoSettings::default, |g| VideoSettings::from(&g.ui.settings))
    }

    fn settings_changed(&mut self) {
        let video = self.video_settings();
        if let (Some(tx),Some(g))=(&self.tx,&self.gui) {
            let _=tx.send(ToWorker::MinecraftSettings(g.ui.settings.clone()));
        }
        if self.gui.as_ref().is_some_and(|g| g.texture_anaglyph != video.anaglyph) { self.refresh_textures(); }
        self.frame_deadline = None;
        if let Some(gpu) = &mut self.gpu {
            gpu.apply_video(video);
        }
        if let Some(tx) = &self.tx {
            let _ = tx.send(ToWorker::Settings(video));
            self.idle = false;
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
        if let Some(tx)=&self.tx {
            let _=tx.send(ToWorker::MinecraftContext {
                screen:self.gui.as_ref().is_some_and(|g|g.screen.is_some()),focused:self.grabbed,
            });
        }
    }

    /// `Minecraft.startWorld`: open a world on the world thread (the loading screen shows
    /// its progress until it is ready).
    fn start_world(&mut self, w: WorldOptions) {
        self.world_dir = w.dir.clone();
        self.delete_hardcore_after_close = false;
        let seed = w.seed;
        self.ready = false;
        self.minecraft_events=Default::default();
        self.minecraft_system_time=None;
        self.idle = false;
        self.world_time = 0;
        self.last_cam_sent = DVec3::splat(f64::MAX);
        self.temps = BiomeSource::new(seed);
        self.seed = seed;
        self.cloud_ticks = 0;
        self.player = None;
        self.pending_look.clear();
        self.life = 0;
        self.hud = gui::ingame::HudPlayer::default();
        if let Some(g) = &mut self.gui {
            g.ui.player_dead = false;
        }
        self.weather = Default::default();
        self.weather_terrain = Default::default();
        self.weather_rng = render::weather::ClientRandom::new(seed);
        self.rain_sound_counter = 0;
        self.fog_fade = [0.0; 2];
        self.particles.change_world(seed);
        self.paused = false;
        if let Some((x, y, z, yaw, pitch)) = self.opts.at {
            self.cam = Camera { pos: DVec3::new(x, y, z), yaw, pitch };
        }
        self.loading = Some(gui::loading::LoadingScreen { title: String::new(), text: String::new(), progress: -1 });
        // Minecraft.startWorld: count the start, then changeWorld syncs the stats file.
        if let Some(g) = &mut self.gui {
            let is_new = w.dir.as_ref().is_none_or(|d| !d.join("level.dat").exists());
            g.ui.stats.add(if is_new { STAT_CREATE_WORLD } else { STAT_LOAD_WORLD }, 1);
            g.ui.stats.add(STAT_START_GAME, 1);
            g.ui.stats.sync();
        }
        let col = self.col.clone().expect("colorizers");
        let (tx, rx, h) = worker::spawn(w, col, self.video_settings());
        self.tx = Some(tx);
        self.rx = Some(rx);
        self.worker = Some(h);
    }

    /// Leave the world: the world thread saves (shown as "Saving chunks"), then the title
    /// screen (or the program exits).
    fn quit_world(&mut self, exit: bool) {
        self.exit_after_save = exit;
        // changeWorld(null) syncs the stats file.
        if let Some(g) = &mut self.gui {
            g.ui.stats.sync();
        }
        if let Some(tx) = &self.tx {
            let _ = tx.send(ToWorker::Quit);
            self.loading =
                Some(gui::loading::LoadingScreen { title: String::new(), text: String::new(), progress: -1 });
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
        if std::mem::take(&mut self.delete_hardcore_after_close) {
            if let Some(directory) = self.world_dir.take() {
                if let (Some(parent), Some(folder)) = (directory.parent(), directory.file_name()) {
                    gui::saves::delete(parent, &folder.to_string_lossy());
                }
            }
        }
        self.ready = false;
        self.loading = None;
        self.entities.clear();
        self.pistons.clear();
        if let Some(snd) = &mut self.sound {
            snd.stop_streaming();
        }
        if let Some(g) = &mut self.gpu {
            g.clear_chunks();
            g.entities.states.clear();
            g.entities.pistons.clear();
        }
        if self.exit_after_save {
            el.exit();
            return;
        }
        if let Some(gui) = &mut self.gui {
            gui.ui.world_open = false;
            let mut acts = Vec::new();
            let screen: Box<dyn gui::Screen> = if std::mem::take(&mut self.session_conflict) {
                Box::new(gui::screens::conflict::GuiConflictWarning::new())
            } else {
                gui.main_menu()
            };
            gui.display_gui_screen(Some(screen), &mut acts);
        }
    }

    /// Close the window: save the world first if one is open.
    fn shutdown(&mut self, el: &ActiveEventLoop) {
        if self.tx.is_some() {
            if let Some(g) = &self.gpu {
                g.window.set_title("Minecraft 1.0.0 (Rust) — saving world...");
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
        // shutdownMinecraftApplet
        if let Some(g) = &mut self.gui {
            g.ui.stats.sync();
        }
        el.exit();
    }

    fn apply_gui_actions(&mut self, el: &ActiveEventLoop) {
        let Some(gui) = &mut self.gui else { return };
        if let Some(snd) = &mut self.sound {
            for (name, v, p) in gui.ui.sounds.drain(..) {
                snd.play_sound_fx(name, v, p);
            }
            let (s, m) = (gui.ui.settings.sound_volume, gui.ui.settings.music_volume);
            if s != snd.sound_volume || m != snd.music_volume {
                snd.options_changed(s, m);
            }
        } else {
            gui.ui.sounds.clear();
        }
        let acts = gui.process_actions();
        // Container clicks and closes go to the world thread, in order.
        let commands = gui.take_container_commands();
        let signs=std::mem::take(&mut gui.ui.sign_commands);
        if let Some(tx) = &self.tx {
            for (position,text) in signs {let _=tx.send(ToWorker::EditSign{position,text});}
            for c in commands {
                let _ = tx.send(ToWorker::Container(c));
            }
        }
        for a in acts {
            match a {
                gui::system::GameAction::StartWorld { folder, name, seed, settings } => {
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
                        settings,
                    };
                    self.start_world(w);
                }
                gui::system::GameAction::QuitWorld => self.quit_world(false),
                gui::system::GameAction::DeleteHardcoreWorld => {
                    // Close and join the world worker before deleting its save.
                    self.delete_hardcore_after_close = true;
                    self.quit_world(false);
                }
                gui::system::GameAction::Shutdown => {
                    self.shutdown(el);
                    return;
                }
                gui::system::GameAction::RefreshTextures => self.refresh_textures(),
                gui::system::GameAction::SettingsChanged => {
                    self.settings_changed();
                }
                gui::system::GameAction::IngameFocus => {
                    if self.ready {
                        self.set_grab(true);
                    }
                }
                gui::system::GameAction::IngameUnfocus => {
                    // setIngameNotInFocus -> KeyBinding.unPressAllKeys()
                    self.move_keys = [false; 6];
                    if self.grabbed {if let Some(g)=&mut self.gui {g.ui.settings.release_keys();}}
                    self.set_grab(false);
                }
                action @ (gui::system::GameAction::Respawn | gui::system::GameAction::EndReturn) => {
                    // Minecraft.respawn: a new player object replaces the dead one.
                    self.life += 1;
                    self.pending_look.clear();
                    if let Some(tx) = &self.tx {
                        let _ = tx.send(if matches!(action, gui::system::GameAction::EndReturn) { ToWorker::EndReturn } else { ToWorker::Respawn });
                    }
                }
            }
        }
    }

    /// Press a button of the current screen like the mouse would.
    fn click_button(&mut self, id: i32) {
        let Some(g) = &mut self.gui else { return };
        let Some(s) = &mut g.screen else { return };
        let Some(b) = s.base().buttons.iter().find(|b| b.id == id).cloned() else {
            return;
        };
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
            11 if frames > 10 && !self.ready && self.loading.is_none() && self.has_button(1) => {
                self.flow_next("7_title_again")
            }
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

    /// Exercise the GUI action -> worker reset -> GPU upload path in one world.
    fn video_test_step(&mut self, el: &ActiveEventLoop) -> Option<(std::path::PathBuf, String)> {
        let mut flow = self.video_flow.take()?;
        flow.frames += 1;
        if flow.frames > 30000 {
            eprintln!("video-flowtest: timed out at stage {}", flow.stage);
            self.shutdown(el);
            std::process::exit(1);
        }
        if !self.ready || self.loading.is_some() || self.ticks_ready < 100 {
            self.video_flow = Some(flow);
            return None;
        }
        if flow.stage == VIDEO_CASES.len() {
            eprintln!("VIDEO FLOW EXACT: five live settings combinations, renderer reset and window counts");
            self.shutdown(el);
            return None;
        }
        let (distance, fancy, ao, limit) = VIDEO_CASES[flow.stage];
        if !flow.waiting {
            self.display_ingame_menu();
            self.apply_gui_actions(el);
            self.click_button(0);
            self.apply_gui_actions(el);
            self.click_button(101);
            self.apply_gui_actions(el);
            use gui::settings::EnumOptions;
            for _ in 0..4 {
                if self.video_settings().distance != distance {
                    self.click_button(EnumOptions::RenderDistance.ordinal());
                    self.apply_gui_actions(el);
                }
            }
            if self.video_settings().fancy != fancy {
                self.click_button(EnumOptions::Graphics.ordinal());
                self.apply_gui_actions(el);
            }
            if self.video_settings().ao != ao {
                self.click_button(EnumOptions::AmbientOcclusion.ordinal());
                self.apply_gui_actions(el);
            }
            for _ in 0..3 {
                if self.video_settings().limit != limit {
                    self.click_button(EnumOptions::FramerateLimit.ordinal());
                    self.apply_gui_actions(el);
                }
            }
            let v = self.video_settings();
            assert_eq!((v.distance, v.fancy, v.ao, v.limit), (distance, fancy, ao, limit));
            self.click_button(200);
            self.apply_gui_actions(el);
            self.click_button(200);
            self.apply_gui_actions(el);
            // Stay in the pause menu while meshes rebuild, as when a user edits Video
            // Settings. Resume for the captured world frame below.
            flow.waiting = true;
            flow.frames = 0;
            flow.reset = flow.stage == 0;
            eprintln!(
                "video-flowtest stage {}: distance {distance}, fancy {fancy}, AO {ao}, limit {limit}",
                flow.stage
            );
        } else if flow.reset
            && flow.frames > 120
            && self.gpu.as_ref().unwrap().chunk_count() == self.video_settings().chunks_wide().pow(2) as usize
        {
            let expected = self.video_settings().chunks_wide().pow(2) as usize;
            assert_eq!(self.gpu.as_ref().unwrap().chunk_count(), expected, "live renderer window count");
            self.click_button(4);
            self.apply_gui_actions(el);
            eprintln!("video-flowtest stage {}: {expected} chunk meshes, {} fps", flow.stage, self.fps);
            std::fs::create_dir_all(&flow.dir).expect("video screenshots directory");
            let shot = (
                flow.dir.clone(),
                format!("{}_d{distance}_{}_ao{ao}", flow.stage, if fancy { "fancy" } else { "fast" }),
            );
            flow.stage += 1;
            flow.waiting = false;
            self.video_flow = Some(flow);
            return Some(shot);
        }
        if flow.frames > 0 && flow.frames % 600 == 0 {
            eprintln!(
                "video-flowtest waiting: stage {}, frames {}, chunks {}, ticks {}, reset {}",
                flow.stage,
                flow.frames,
                self.gpu.as_ref().unwrap().chunk_count(),
                self.ticks_ready,
                flow.reset
            );
        }
        self.video_flow = Some(flow);
        None
    }

    /// A mouse button: to the screen if one is open, else queued for runTick.
    fn mouse_button(&mut self, el: &ActiveEventLoop, b: i32, pressed: bool) {
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
        } else if self.ready && self.player.is_some() {
            self.minecraft_events.mouse.push(crate::minecraft_input::MouseEvent {button:b,pressed,wheel:0,fresh:true});
        }
        if b < 2 {
            self.mouse_held[b as usize] = pressed;
        }
    }

    /// A key going down or up (`c` is the typed character for screens).
    fn key_input(&mut self, el: &ActiveEventLoop, code: KeyCode, c: char, pressed: bool, repeat: bool) {
        let repeats = self.gui.as_ref().and_then(|g| g.screen.as_ref()).is_some_and(|s| s.repeats_keys());
        if repeat && !repeats { return; }
        let screen_open = self.gui.as_ref().is_some_and(|g| g.screen.is_some());
        let in_game_key = !screen_open && self.ready && self.loading.is_none() && !repeat;
        if pressed && code == KeyCode::F11 && !in_game_key {
            self.toggle_fullscreen();
            return;
        }
        if in_game_key && self.player.is_some() {
            self.minecraft_events.keys.push(crate::minecraft_input::KeyEvent {code:lwjgl_key(code),pressed});
        }
        // Keyboard.isKeyDown(LSHIFT/RSHIFT) as screens see it, including this event.
        if matches!(code, KeyCode::ShiftLeft | KeyCode::ShiftRight) {
            if pressed {
                self.keys.insert(code);
            } else {
                self.keys.remove(&code);
            }
        }
        if let Some(g) = &mut self.gui {
            g.ui.shift_down = self.keys.contains(&KeyCode::ShiftLeft) || self.keys.contains(&KeyCode::ShiftRight);
        }
        if pressed {
            if screen_open && self.loading.is_none() {
                let key = lwjgl_key(code);
                if let Some(g) = &mut self.gui {
                    g.key_event(c, key);
                }
                self.apply_gui_actions(el);
            }
            if code != KeyCode::F2 || !repeat { self.keys.insert(code); }
        } else {
            if code != KeyCode::F2 {
                self.keys.remove(&code);
            }
        }
    }

    fn toggle_fullscreen(&self) {
        if let Some(gpu)=&self.gpu {
            let fullscreen=gpu.window.fullscreen().is_none().then(||winit::window::Fullscreen::Borderless(gpu.window.current_monitor()));
            gpu.window.set_fullscreen(fullscreen);
        }
    }

    /// Run the `--play-script` commands that are due; a screenshot request comes back.
    fn script_step(&mut self, el: &ActiveEventLoop) -> Option<(std::path::PathBuf, String)> {
        if self.script.is_none() {
            if let Some(path) = self.opts.play_script.clone() {
                match crate::playscript::Script::load(&path) {
                    Ok(s) => self.script = Some(s),
                    Err(e) => {
                        eprintln!("{e}");
                        self.opts.play_script = None;
                    }
                }
            }
        }
        // Scripts also drive the title screen after a directly opened world is closed.
        // Keep waiting during world loading, when there is no player to drive yet.
        let menus = !self.ready && self.loading.is_none();
        if !menus && (!self.ready || self.loading.is_some() || self.player.is_none()) {
            return None;
        }
        // The script plays the role of the focused window.
        self.grabbed = self.gui.as_ref().is_some_and(|g| g.screen.is_none());
        let script = self.script.as_mut()?;
        let dir = script.dir.clone();
        let mut shot = None;
        for c in script.due(self.script_ticks) {
            use crate::playscript::Command;
            match c {
                Command::Key(k, down) => self.key_input(el, k, '\0', down, false),
                Command::Mouse(b, down) => self.mouse_button(el, b, down),
                Command::Look(x, y) => {
                    self.mouse_delta.0 += x;
                    self.mouse_delta.1 -= y;
                }
                // Routed like WindowEvent::MouseWheel: an open screen scrolls, otherwise the hotbar.
                Command::Scroll(n) => {
                    let screen_open = self.gui.as_ref().is_some_and(|g| g.screen.is_some());
                    if screen_open && n != 0 {if let Some(g)=self.gui.as_mut(){if let Some(s)=g.screen.as_mut(){s.mouse_scrolled(&mut g.ui,n.signum());}}}
                    else {self.minecraft_events.mouse.push(crate::minecraft_input::MouseEvent {button:-1,pressed:false,wheel:n,fresh:true});}
                }
                Command::Move(x, y) => {
                    self.cursor = (x, y);
                    if let Some(g) = &mut self.gui {
                        if g.screen.is_some() {
                            let (gx, gy) = g.gui_mouse(x, y);
                            g.mouse_event(gx, gy, -1, false);
                        }
                    }
                }
                Command::Shot(name) => {
                    if let Some(p) = &self.player {
                        eprintln!(
                            "script {name}: pos {:.2} {:.2} {:.2} portal {} prev {} nether {}",
                            p.entity.pos.x, p.entity.pos.y, p.entity.pos.z, self.hud.time_in_portal, self.hud.prev_time_in_portal, self.nether
                        );
                    }
                    shot = Some((dir.clone(), name))
                }
                Command::Button(id) => {
                    self.click_button(id);
                    self.apply_gui_actions(el);
                }
                Command::Quit => {
                    self.shutdown(el);
                    return None;
                }
            }
        }
        shot
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
        let script_shot = self.script_step(el);
        let flow_shot = self.flow_capture().or(script_shot);
        let now = Instant::now();
        let dt = (now - self.last_frame).as_secs_f64().min(0.25);
        self.last_frame = now;

        // GuiScreen.updateScreen runs every game tick (20 per second).
        let mut gui_ticks = 0;
        let tick_start = Instant::now();
        while now >= self.gui_ticks && gui_ticks < 20 {
            // runTick: statFileWriter.func_27178_d, then the HUD.
            if let Some(g) = &mut self.gui {
                g.ui.stats.tick();
            }
            self.ingame.update_tick();
            if let Some(g) = &mut self.gui {
                g.update_screen();
            }
            if self.ready && self.player.is_some() {
                let mut events=std::mem::take(&mut self.minecraft_events);
                let fresh=self.minecraft_system_time.is_some_and(|time|now.duration_since(time)<=Duration::from_millis(200));
                for event in &mut events.mouse {event.fresh=fresh;}
                self.minecraft_system_time=Some(now);
                events.down=self.keys.iter().map(|&k|lwjgl_key(k)).collect();
                if let (Some(g),Some(tx))=(&self.gui,&self.tx) {
                    let _=tx.send(ToWorker::MinecraftEvents {settings:g.ui.settings.clone(),events,
                        screen:g.screen.is_some(),focused:self.grabbed});
                }
            }
            self.gui_ticks += Duration::from_millis(50);
            gui_ticks += 1;
        }
        if let Some(g) = &mut self.gui {
            g.tick_nanos = tick_start.elapsed().as_nanos() as i64;
        }
        if gui_ticks == 20 {
            self.gui_ticks = now;
        }
        if (self.ready && self.player.is_some()) || (!self.ready && self.loading.is_none()) {
            self.script_ticks += gui_ticks as i64;
        }
        if let Some(g) = &mut self.gui {
            g.ui.millis =
                std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_millis() as i64);
        }
        self.apply_gui_actions(el);

        let video = self.video_settings();
        // World messages: upload a bounded number of meshes per frame.
        let mut uploads = 0;
        let mut closed = false;
        let mut ungrab = false;
        let mut minecraft_actions=Vec::new();
        if let (Some(rx), Some(gpu)) = (&self.rx, &mut self.gpu) {
            while uploads < 64 {
                match rx.try_recv() {
                    Ok(FromWorker::Ready { spawn, seed, name }) => {
                        // changeWorld / respawn: thePlayer.movementInput = new MovementInputFromOptions
                        self.move_keys = [false; 6];
                    if let Some(g)=&mut self.gui {g.ui.settings.release_keys();}
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
                    Ok(FromWorker::ResetMeshes) => {
                        gpu.clear_chunks();
                        self.idle = false;
                        if let Some(flow) = &mut self.video_flow {
                            flow.reset = true;
                        }
                    }
                    Ok(FromWorker::Entities(list)) => {
                        // EntityCrit2FX follows its live target entity.
                        for e in &list { self.particles.renderer.update_critical_target(&e.entity); }
                        self.entities = list;
                    }
                    Ok(FromWorker::BlockTiles {pistons,events}) => {
                        self.pistons=pistons;
                        // Note and piston sounds arrive with the world effects (BlockNote/BlockPistonBase.playBlock).
                        let _ = events;
                    }
                    Ok(FromWorker::ContainerOpened(s)) => {
                        if let Some(g) = &mut self.gui {
                            for a in g.open_container(s) {
                                match a {
                                    // setIngameNotInFocus -> resetPlayerKeyState
                                    gui::system::GameAction::IngameUnfocus => self.move_keys = [false; 6],
                                    gui::system::GameAction::IngameFocus => {}
                                    _ => {}
                                }
                            }
                        }
                        ungrab = true;
                    }
                    Ok(FromWorker::ContainerUpdate(s)) => {
                        if let Some(g) = &mut self.gui {
                            g.update_container(s);
                        }
                    }
                    Ok(FromWorker::Chat(key)) => {
                        if let Some(g) = &self.gui {
                            let text = g.ui.lang.translate_key(&key);
                            self.ingame.add_chat_message(&g.ui, text);
                        }
                    }
                    // Presentation callbacks stay on the audio/render thread.
                    Ok(FromWorker::Effects { events, terrain }) => {
                        for event in &events {
                            if let Some(name) = crate::world_effects::Effects::record_name(event) {
                                self.ingame.set_record_playing(&format!("C418 - {name}"));
                            }
                        }
                        self.particles.renderer.particle_setting = self.gui.as_ref().map_or(0, |g| g.ui.settings.particles);
                        self.particles.fancy = self.gui.as_ref().is_none_or(|gui| gui.ui.settings.fancy_graphics);
                        self.particles.colors = self.col.clone();
                        self.particles.receive(events, terrain, self.cam.pos.to_array(), self.sound.as_mut());
                    }
                    Ok(FromWorker::Player {
                        entity,
                        hud,
                        view,
                        events,
                        in_water,
                        input_seq,
                        score,
                        hardcore,
                        life,
                    }) => {
                        if life < self.life {
                            continue;
                        }
                        // The partial tick restarts when the new position is applied: the Time
                        // message that also restarts it comes a few milliseconds later (after the
                        // tick's effects), and drawing the new position with the old partial tick
                        // made the camera overshoot and snap back several times a second.
                        self.last_tick = now;
                        self.tick_started_by_player = true;
                        // The world thread's rotation plus the frames it hasn't seen yet.
                        self.pending_look.retain(|&(s, ..)| s > input_seq);
                        let mut look = [entity.yaw, entity.pitch, entity.prev_yaw, entity.prev_pitch];
                        for &(_, y, p) in &self.pending_look {
                            set_angles(&mut look, y, p);
                        }
                        self.look = look;
                        let h = &mut self.hud;
                        h.health = hud.health;
                        h.food_level = hud.food.level;
                        h.food_saturation = hud.food.saturation;
                        h.xp_points = hud.experience.points;
                        h.xp_level = hud.experience.level;
                        h.survival = !hud.capabilities.creative;
                        h.seed = self.seed;
                        h.prev_health = hud.prev_health;
                        h.hearts_life = hud.hearts_life;
                        h.armor = hud.armor;
                        h.helmet = hud.inventory.armor[3].map(|s| s.id);
                        h.current_item = hud.inventory.current_item;
                        h.air = hud.air;
                        h.head_in_water = in_water;
                        h.brightness = hud.brightness;
                        h.prev_time_in_portal = hud.prev_time_in_portal;
                        h.time_in_portal = hud.time_in_portal;
                        h.sleep_timer = hud.sleep_timer;
                        for i in 0..9 {
                            h.hotbar[i] = hud.inventory.main[i].map(|s| (s.into(), s.animations));
                        }
                        for ev in events {
                            match ev {
                                worldgen::entity::player::PlayerEvent::EditSign { position, block, metadata, sign } => {
                                    if let Some(g)=&mut self.gui {g.ui.display(Some(Box::new(gui::screens::edit_sign::GuiEditSign::new(position,block,metadata,sign))));}
                                }
                                worldgen::entity::player::PlayerEvent::Pickup { source, .. } => self.particles.pickup(source, entity.entity_id),
                                worldgen::entity::player::PlayerEvent::CriticalHit { target } => self.particles.renderer.critical_hit(&target, self.cam.pos.to_array()),
                                // EntityPlayerSP.func_40183_c: EntityCrit2FX(world, target, "magicCrit").
                                worldgen::entity::player::PlayerEvent::MagicCriticalHit { target } => self.particles.renderer.critical_hit_named(&target, self.cam.pos.to_array(), "magicCrit"),
                                // The world thread switches dimension itself (usePortal).
                                worldgen::entity::player::PlayerEvent::ChangeDimension => {}
                                worldgen::entity::player::PlayerEvent::WinGame => {
                                    if let Some(g) = &mut self.gui {
                                        let username = entity.player_state().map_or("Player", |s| s.username.as_str());
                                        g.ui.display(Some(Box::new(gui::screens::win_game::GuiWinGame::new(username))));
                                        ungrab = true;
                                    }
                                }
                                worldgen::entity::player::PlayerEvent::Chat(key) => {
                                    if let Some(g) = &self.gui {
                                        let text = g.ui.lang.translate_key(key);
                                        self.ingame.add_chat_message(&g.ui, text);
                                    }
                                }
                                worldgen::entity::player::PlayerEvent::Stat { id, amount } => {
                                    if let Some(g) = &mut self.gui {
                                        g.add_stat(id, amount);
                                    }
                                }
                                worldgen::entity::player::PlayerEvent::Container(c) => {
                                    use worldgen::container::ContainerEvent;
                                    match c {
                                        ContainerEvent::Closed { window } => {
                                            if let Some(g) = &mut self.gui {
                                                g.container_closed(window);
                                            }
                                        }
                                        ContainerEvent::Achievement(key) => {
                                            if let Some(g) = &mut self.gui {
                                                let list = &gui::stats::ACHIEVEMENTS;
                                                if let Some(a) = list.iter().find(|a| a.key == key) {
                                                    g.add_stat(a.id, 1);
                                                }
                                            }
                                        }
                                        // ItemStack.onCrafting
                                        ContainerEvent::Crafted { item, count } => {
                                            if let Some(g) = &mut self.gui {
                                                let id = worldgen::entity::player::stat::crafted(item);
                                                if id >= 0 {
                                                    g.add_stat(id, count);
                                                }
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                                _ => {}
                            }
                        }
                        if let Some(g) = &mut self.gui {
                            // EntityPlayerSP.onLivingUpdate: the inventory hint until it is taken.
                            if !g.ui.stats.unlocked(0) {
                                g.toast.queue_information(&g.ui, 0);
                            }
                            g.ui.player_dead = hud.health <= 0;
                            g.ui.player_score = score;
                            g.ui.release.hardcore = hardcore;
                            g.ui.release.player_level = hud.experience.level;
                            g.ui.release.effects = entity.living.effects.clone();
                            let capacity = entity.living.effect_capacity;
                            g.ui.release.effects.sort_by_key(|e| ((e.id as u32 ^ (e.id as u32 >> 16)) as usize) & (capacity - 1));
                            // Minecraft.runTick: no screen and a dead player shows GuiGameOver.
                            if g.ui.player_dead && g.screen.is_none() {
                                g.ui.display(None);
                            }
                        }
                        self.particles.collector = Some((*entity).clone());
                        let nether = entity.player_state().is_some_and(|s| s.dimension == -1);
                        if nether != self.nether {
                            self.nether = nether;
                            self.temps = if nether { BiomeSource::hell() } else { BiomeSource::new(self.seed) };
                        }
                        if let Some(p) = &mut self.player {
                            p.update(*entity, *hud, view);
                        } else {
                            self.player = Some(PlayerView::new(*entity, *hud, view));
                        }
                    }
                    Ok(FromWorker::QuickSaved(done)) => {
                        if let Some(gui) = &mut self.gui {
                            gui.ui.world_saved = done;
                        }
                    }
                    Ok(FromWorker::ReleaseTiles(books,portals)) => {gpu.entities.books = books;gpu.entities.portals=portals;}
                    Ok(FromWorker::DragonCrystals(crystals)) => {gpu.entities.dragon_crystals = crystals.into_iter().map(|(id, position, rotation)| (id, (position, rotation))).collect();}
                    Ok(FromWorker::Signs(signs,chests,spawners)) => {gpu.entities.signs=signs;gpu.entities.chests=chests;gpu.entities.spawners=spawners;}
                    Ok(FromWorker::Mesh { cx, cz, opaque, translucent }) => {
                        gpu.upload_chunk(cx, cz, &opaque, &translucent);
                        uploads += 1;
                        self.chunk_updates += 1;
                        self.idle = false;
                    }
                    Ok(FromWorker::Drop { cx, cz }) => gpu.remove_chunk(cx, cz),
                    Ok(FromWorker::Time { time, brightness, fog_brightness, chunks, loaded_entities, weather, terrain }) => {
                        if self.ready {
                            self.ticks_ready += 1;
                        }
                        if self.player.is_none() {
                            self.hud.brightness = brightness;
                        }
                        self.chunks_loaded = chunks;
                        self.loaded_entities=loaded_entities;
                        self.cloud_ticks += 1;
                        // EntityRenderer.updateRenderer: the smooth camera's filters advance per tick.
                        if let Some(g) = self.gui.as_ref().filter(|g| g.ui.settings.smooth_camera) {
                            self.smooth_camera.tick(g.ui.settings.mouse_sensitivity);
                        }
                        let rng = &mut self.flicker_rng;
                        self.flicker.tick(|| rng.next_double());
                        self.fog_fade[0] = self.fog_fade[1];
                        // EntityRenderer.updateRenderer: distance-dependent fade toward eye brightness.
                        self.fog_fade[1] += (video.fog_target(fog_brightness) - self.fog_fade[1]) * 0.1f32;
                        self.world_time = time;
                        self.weather = weather;
                        self.weather_terrain = terrain;
                        if !std::mem::take(&mut self.tick_started_by_player) {
                            self.last_tick = now;
                        }
                        if let Some(snd) = &mut self.sound {
                            // PlayerControllerSP.updateController, every unpaused tick.
                            let creative = self.player.as_ref()
                                .and_then(|p| p.entity.player_state())
                                .is_some_and(|s| s.capabilities.creative);
                            if worldgen::entity::player::controller::PlayerControllerSP::updates_each_tick(creative) {
                                snd.play_random_music_if_ready();
                            }
                        }
                        let (splashes, rain_sound) = render::weather::add_rain_particles(
                            &self.weather_terrain,
                            self.cam.pos.to_array(),
                            self.cloud_ticks as i32,
                            weather.rain_at(1.0),
                            video.fancy,
                            self.gui.as_ref().map_or(0, |g| g.ui.settings.particles),
                            &mut self.weather_rng,
                            &mut self.rain_sound_counter,
                        );
                        if let (Some(snd), Some(r)) = (&mut self.sound, rain_sound) {
                            play_world_sound(snd, self.cam.pos, "ambient.weather.rain", r.pos, r.volume, r.pitch);
                        }
                        self.particles.spawn(splashes);
                        self.particles.tick(self.cam.pos.to_array());
                    }
                    Ok(FromWorker::MinecraftActions {actions,settings}) => {
                        if let Some(g)=&mut self.gui {
                            g.ui.settings.hide_gui=settings.hide_gui;
                            g.ui.settings.show_debug_info=settings.show_debug_info;
                            g.ui.settings.third_person_view=settings.third_person_view;
                            g.ui.settings.render_distance=settings.render_distance;
                            g.ui.settings.smooth_camera_zoom_amount=settings.smooth_camera_zoom_amount;
                        }
                        minecraft_actions.extend(actions);
                    }
                    Ok(FromWorker::Idle) => self.idle = true,
                    Ok(FromWorker::Saved) => {
                        closed = true;
                        break;
                    }
                    Ok(FromWorker::SessionConflict) => {
                        self.session_conflict = true;
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
        for action in minecraft_actions {
            use crate::minecraft_input::Action;
            match action {
                Action::Focus=>self.set_grab(true),
                Action::Menu=>{self.display_ingame_menu();self.apply_gui_actions(el);},
                Action::Inventory=>{if let Some(tx)=&self.tx {let _=tx.send(ToWorker::Container(gui::container_api::inventory_key_request()));}},
                Action::Chat=>{if let Some(g)=&mut self.gui {g.ui.display(Some(Box::new(gui::screens::chat::GuiChat::default())));}self.apply_gui_actions(el);},
                Action::Fullscreen=>self.toggle_fullscreen(),
                Action::Container=>{}, // Existing PlayerEvent opens the actual screen.
                Action::Settings=>self.settings_changed(),
                Action::Reload=>{
                    let seed=self.started.elapsed().as_nanos() as i64;
                    let mut sound=crate::sound::SoundManager::new(&crate::sound::resources_dir(),seed);
                    if let Some(g)=&self.gui {sound.options_changed(g.ui.settings.sound_volume,g.ui.settings.music_volume);}
                    self.sound=Some(sound);
                },
            }
        }
        if ungrab {
            self.set_grab(false);
        }
        if closed {
            self.world_closed(el);
            return;
        }
        self.apply_gui_actions(el);

        let flow_shot = self.video_test_step(el).or(flow_shot);

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

        // The player's controls, one frame at a time (Minecraft.runTick reads the queued
        // events; EntityRenderer turns the player every frame).
        if self.player.is_some() {
            let mut f = std::mem::take(&mut self.input);
            let (dx, dy) = std::mem::take(&mut self.mouse_delta);
            if in_game && self.grabbed {
                if let Some(g) = &self.gui {
                    let s = &g.ui.settings;
                    let k = s.mouse_sensitivity * 0.6 + 0.2;
                    let k = k * k * k * 8.0;
                    let invert = if s.invert_mouse { -1.0 } else { 1.0 };
                    let mut x = dx as i32 as f32 * k;
                    let mut y = dy as i32 as f32 * k;
                    if s.smooth_camera {
                        let partial = if self.paused { 0.0 } else { ((now - self.last_tick).as_secs_f32() * 20.0).min(1.0) };
                        (x, y) = self.smooth_camera.frame(x, y, partial);
                    }
                    f.yaw_delta = x;
                    f.pitch_delta = y * invert;
                }
                // Mouse.isButtonDown with the screen gone and the game focused.
                if let Some(g)=&self.gui {
                    f.left_held = g.ui.settings.binding_pressed("key.attack");
                    f.right_held = g.ui.settings.binding_pressed("key.use");
                }
            }
            let m = self.gui.as_ref().map_or([false;6],|g| {
                ["key.forward","key.back","key.left","key.right","key.jump","key.sneak"].map(|name|g.ui.settings.binding_pressed(name))
            });
            (f.forward, f.back, f.left, f.right, f.jump, f.sneak) = (m[0], m[1], m[2], m[3], m[4], m[5]);
            self.input_seq += 1;
            f.seq = self.input_seq;
            if f.yaw_delta != 0.0 || f.pitch_delta != 0.0 {
                set_angles(&mut self.look, f.yaw_delta, f.pitch_delta);
                self.pending_look.push((f.seq, f.yaw_delta, f.pitch_delta));
            }
            if let Some(tx) = &self.tx {
                let _ = tx.send(ToWorker::Input(f));
            }
        }

        // Fly camera (yaw 0 faces +Z, like the original) when there is no player.
        if in_game && self.player.is_none() {
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

        self.tick_texture_animations(gui_ticks, video);
        let partial = if self.paused { 0.0 } else { ((now - self.last_tick).as_secs_f32() * 20.0).min(1.0) };
        // EntityRenderer.orientCamera: the player's eyes between the last two ticks.
        if let Some(p) = &self.player {
            let e = &p.entity;
            let t = partial as f64;
            self.cam.pos = DVec3::new(
                e.last_tick_pos.x + (e.pos.x - e.last_tick_pos.x) * t,
                e.last_tick_pos.y + (e.pos.y - e.last_tick_pos.y) * t,
                e.last_tick_pos.z + (e.pos.z - e.last_tick_pos.z) * t,
            );
            let l = self.look;
            self.cam.yaw = l[2] + (l[0] - l[2]) * partial;
            self.cam.pitch = l[3] + (l[1] - l[3]) * partial;
        }
        let gui_partial = ((now + Duration::from_millis(50)).saturating_duration_since(self.gui_ticks).as_secs_f32()
            * 20.0)
            .clamp(0.0, 1.0);

        // Menus or the loading screen, no world drawn.
        if !self.ready || self.loading.is_some() {
            let (Some(gpu), Some(gui)) = (&mut self.gpu, &mut self.gui) else {
                return;
            };
            let loading = self.loading.clone();
            let cursor = self.cursor;
            let screenshot = self.keys.remove(&KeyCode::F2);
            let img = gpu.render_overlay_only(flow_shot.is_some() || screenshot, &mut |device, queue, view, size| {
                gui.ui.gl.begin_frame();
                if let Some(l) = &loading {
                    l.draw(&mut gui.ui);
                } else {
                    gui.sync_pack_icons(device, queue);
            let (mx, my) = gui.gui_mouse(cursor.0, cursor.1);
                    gui.draw_screen(mx, my, gui_partial);
                }
                gui.draw_after_frame();
                gui.renderer.render(device, queue, view, size, &gui.ui.gl, Some([0.0, 0.0, 0.0, 0.0]));
            });
            if screenshot {
                if let Some(img) = &img {
                    let message = crate::screenshot::save(gui.ui.saves_dir.parent().unwrap_or(std::path::Path::new(".")), img);
                    self.ingame.add_chat_message(&gui.ui, message);
                }
            }
            if let (Some(img), Some((dir, name))) = (img, &flow_shot) {
                let _ = assets::save_png(&img, &dir.join(format!("{name}.png")));
            }
            self.apply_gui_actions(el);
            return;
        }

        if let Some(snd) = &mut self.sound {
            snd.set_listener(self.cam.pos.to_array(), self.cam.yaw);
        }
        // WorldProviderHell.calculateCelestialAngle is always 0.5.
        let angle = if self.nether { 0.5 } else { sky::celestial_angle(self.world_time, partial) };
        let temp = self.temps.raw_temperature(self.cam.pos.x.floor() as i32, self.cam.pos.z.floor() as i32) as f32;
        let rain = self.weather.rain_at(partial);
        let thunder = self.weather.thunder_at(partial);
        let precipitation = {
            let dim = if self.nether { worldgen::world::Dimension::Nether } else { worldgen::world::Dimension::Overworld };
            let angle1 = if self.nether { 0.5 } else { sky::celestial_angle(self.world_time, 1.0) };
            let sky_factor = render::lightmap::sky_factor(angle1, self.weather.rain_at(1.0), self.weather.thunder_at(1.0));
            let gamma = self.gui.as_ref().map_or(0.0, |g| g.ui.settings.gamma);
            let texels = render::lightmap::compute_dimension(&dim.light_brightness_table(), sky_factor, self.flicker.x, self.weather.flash > 0, gamma, self.player.as_ref().and_then(|p| p.entity.player_state()).is_some_and(|s| s.dimension == 1));
            let mut out: [Vec<render::weather::WeatherVertex>; 2] = [Vec::new(), Vec::new()];
            for (tex, mut v, light) in render::weather::precipitation10(
                &self.weather_terrain,
                self.cam.pos.to_array(),
                self.cloud_ticks as i32,
                partial,
                rain,
                video.fancy,
                &mut self.weather_rng,
            ) {
                // The rain quads sample the lightmap (texture unit 1) at their brightness.
                let l = render::lightmap::sample(&texels, light);
                for i in 0..3 {
                    v.color[i] *= l[i];
                }
                out[tex as usize].push(v);
            }
            out
        };
        let lightning = self
            .entities
            .iter()
            .filter_map(|s| render::entity::bridge::lightning(&s.entity))
            .chain(self.opts.bolt)
            .flat_map(|b| render::weather::lightning(b, self.cam.pos.to_array()))
            .collect();
        let lit_effect_batches = self.particles.lit_batches(
            self.cam.pos.to_array(), [self.cam.yaw, self.cam.pitch], partial, video.fancy,
        );
        // EntityRenderer.func_35808_e: the lightmap, with the world read at partial tick 1.
        let lightmap = {
            let dim = if self.nether { worldgen::world::Dimension::Nether } else { worldgen::world::Dimension::Overworld };
            let angle1 = if self.nether { 0.5 } else { sky::celestial_angle(self.world_time, 1.0) };
            let sky_factor = render::lightmap::sky_factor(angle1, self.weather.rain_at(1.0), self.weather.thunder_at(1.0));
            // The Brightness option (GameSettings.field_35380_M).
            let gamma = self.gui.as_ref().map_or(0.0, |g| g.ui.settings.gamma);
            render::lightmap::compute_dimension(&dim.light_brightness_table(), sky_factor, self.flicker.x, self.weather.flash > 0, gamma, self.player.as_ref().and_then(|p| p.entity.player_state()).is_some_and(|s| s.dimension == 1))
        };
        let particles = self.particles.vertices(
            self.cam.pos.to_array(),
            render::particles::active_render_rotation(self.cam.yaw, self.cam.pitch,
                self.gui.as_ref().is_some_and(|g| g.ui.settings.third_person_view == 2)),
            partial,
            &lightmap,
        );
        // First-person camera, player model and targeting use a private render snapshot.
        let settings = self.gui.as_ref().map(|g| &g.ui.settings);
        let camera_settings = render::camera::Settings {
            bobbing: settings.is_none_or(|s| s.view_bobbing),
            third_person: settings.is_some_and(|s| s.third_person_view != 0),
            front_view: settings.is_some_and(|s| s.third_person_view == 2),
            anaglyph: video.anaglyph,
            distance: video.distance,
            ticks: self.cloud_ticks as i32,
            zoom: 1.0,
            ..Default::default()
        };
        if let (Some(p), Some(gpu)) = (self.player.as_mut(), self.gpu.as_ref()) {
            let (w, h) = gpu.size();
            p.viewport = [0, 0, w as i32, h as i32];
            p.far = video.far();
        }
        let player_frame = self.player.as_mut().map(|p| {
            p.fov_option=settings.map_or(0.0,|s|s.fov);
            p.frame(
                partial,
                self.look,
                camera_settings,
                settings.is_some_and(|s| s.hide_gui),
                self.cam.pos.to_array(),
                self.col.as_ref().expect("world colorizers"),
                video.fancy,
            )
        });
        let player_right = if video.anaglyph {
            self.player.as_mut().map(|p| p.frame(partial, self.look, render::camera::Settings { eye: 1, ..camera_settings },
                settings.is_some_and(|s| s.hide_gui), self.cam.pos.to_array(), self.col.as_ref().expect("world colorizers"), video.fancy))
        } else { None };
        let params = FrameParams {
            cam_pos: self.cam.pos,
            yaw: self.cam.yaw,
            pitch: self.cam.pitch,
            fov_y_deg: 70.0,
            celestial: angle,
            moon_phase: render::sky_release::moon_phase(self.world_time),
            end: self.player.as_ref().and_then(|p| p.entity.player_state()).is_some_and(|s| s.dimension == 1),
            sky_color: sky::sky_color(angle, temp, rain, thunder, self.weather.flash, partial),
            fog_color: render::camera::fog_color_release(
                if self.nether {
                    let sky = sky::sky_color(angle, temp, rain, thunder, self.weather.flash, partial);
                    sky::nether_fog_color(sky, video.distance, rain, thunder)
                } else {
                    sky::fog_color(angle, temp, video.distance, rain, thunder, self.weather.flash, partial, sky::look(self.cam.yaw, self.cam.pitch, self.cam.yaw, self.cam.pitch, partial))
                },
                &player_frame.as_ref().map_or_else(Default::default, |v| v.fog),
                self.fog_fade[0] + (self.fog_fade[1] - self.fog_fade[0]) * partial,
                video.anaglyph,
                self.cam.pos.y,
            ),
            far: video.far(),
            video,
            cloud_offset: self.cloud_ticks as f32 + partial,
            rain,
            thunder,
            precipitation,
            lightning,
            particles,
            nether: self.nether,
            player: player_frame,
            player_right,
        };

        if let Some(gpu) = &mut self.gpu {
            gpu.set_lightmap(&lightmap);
            gpu.lit_effect_batches = lit_effect_batches;
            gpu.entities.snapshots = self.entities.clone();
            gpu.entities.pistons = self.pistons.clone();
            gpu.entities.partial = partial;
            gpu.entities.fancy = self.gui.as_ref().is_none_or(|gui| gui.ui.settings.fancy_graphics);
            if let Some(p) = &mut self.player {
                let mut camera = render::entity::manager::CameraState::from(&p.entity);
                camera.rotation = [self.look[0], self.look[1]];
                camera.previous_rotation = [self.look[2], self.look[3]];
                let bed = if camera.sleeping {p.bed()} else {None};
                gpu.entities.manager.cache_active_render_info(
                    render::entity::manager::Context {
                        world: Some(()), engine: Some(()), font: Some(()),
                        player: Some(()), options: Some(()),
                    }, Some(camera), partial, camera_settings.front_view,
                    |_| Ok(bed.map_or((0, 0), |metadata| (26, metadata))),
                ).expect("active singleplayer render context");
                gpu.entities.view = gpu.entities.manager.view;
            } else {
                gpu.entities.manager.camera = self.cam.pos.to_array();
                gpu.entities.view = [self.cam.yaw, self.cam.pitch];
            }
            gpu.entities.angler=self.player.as_ref().map(|p|p.entity.clone());
            gpu.entities.third_person=camera_settings.third_person;
        }
        let want_shot = self.opts.shot.is_some()
                && self.ready
                && uploads == 0
                // Ticking keeps changing blocks somewhere, so a timed shot doesn't wait for idle.
                && if self.opts.shot_ticks > 0 { self.ticks_ready >= self.opts.shot_ticks } else { self.idle };
        let screenshot = self.keys.remove(&KeyCode::F2);
        let capture = want_shot || screenshot || flow_shot.is_some();
        let (Some(gpu), Some(gui)) = (&mut self.gpu, &mut self.gui) else {
            return;
        };
        let cursor = self.cursor;
        let ingame = &mut self.ingame;
        let mut hud = self.hud.clone();
        hud.pos = [self.cam.pos.x, self.cam.pos.y, self.cam.pos.z];
        hud.yaw = self.cam.yaw;
        let debug = gui::ingame::DebugInfo {
            fps_line: self.debug_line.clone(),
            renders: format!("C: {}/{}. F: 0, O: 0, E: 0", gpu.chunk_count(), gpu.chunk_count()),
            entities: "E: 0/0. B: 0, I: 0".into(),
            particles: crate::minecraft_input::debug_entities(self.particles.len(),self.loaded_entities),
            chunks: format!("ServerChunkCache: {} Drop: 0", self.chunks_loaded),
            ..memory_info()
        };
        let shot = gpu.render_with_overlay(&params, capture, &mut |device, queue, view, size| {
            // renderWorld's final func_35810_a disables unit 1, retaining its
            // binding and last first-person glMultiTexCoord for GUI model draws.
            gui.ui.gl.lightmap_enabled = false;
            gui.ui.gl.lightmap_texels = Some(Box::new(lightmap));
            gui.ui.gl.lightmap_coordinates = self.weather_terrain.packed_brightness(
                self.player.as_ref().map_or(self.cam.pos.to_array(), |p| {
                    [p.entity.pos.x, p.entity.pos.y, p.entity.pos.z]
                })
            ) as u32;
            gui.sync_pack_icons(device, queue);
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
                ingame.render_game_overlay(&mut gui.ui, gui_partial, gui.screen.as_ref().is_some_and(|s| s.is_chat()), &hud, &debug);
                gui.renderer.render(device, queue, view, size, &gui.ui.gl, None);
            }
            if screen_open {
                gui.ui.gl.begin_frame();
                gui.draw_screen(mx, my, gui_partial);
                gui.renderer.render(device, queue, view, size, &gui.ui.gl, None);
            }
            // Minecraft.run: guiAchievement.updateAchievementWindow() after the frame.
            gui.ui.gl.begin_frame();
            gui.draw_after_frame();
            gui.renderer.render(device, queue, view, size, &gui.ui.gl, None);
        });
        if let (Some(img), Some((dir, name))) = (&shot, &flow_shot) {
            let _ = assets::save_png(img, &dir.join(format!("{name}.png")));
        }
        if screenshot {
            if let Some(img) = &shot {
                let message = crate::screenshot::save(gui.ui.saves_dir.parent().unwrap_or(std::path::Path::new(".")), img);
                self.ingame.add_chat_message(&gui.ui, message);
            }
        }
        if let Some(img) = shot.filter(|_| want_shot) {
            let path = self.opts.shot.clone().expect("requested screenshot path");
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
                "Minecraft 1.0.0 (Rust) — {} (seed {}) — {} fps — x {:.1} y {:.1} z {:.1} — {} chunks{}",
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
            .with_title("Minecraft 1.0.0 (Rust)")
            .with_inner_size(winit::dpi::LogicalSize::new(854.0, 480.0));
        let window = Arc::new(el.create_window(attrs).expect("create window"));
        let mut gpu = Gpu::new(
            window,
            &assets.terrain,
            &assets.sun,
            &assets.moon,
            &assets.clouds,
            [&assets.snow, &assets.rain, &assets.particles, &assets.mobs["gui/items.png"], &assets.footprint],
            &assets.mobs,
        );
        let (images, font_txt, lang, splashes) = match assets::Jar::open().and_then(|mut j| gui::load_resources(&mut j))
        {
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
            None => self.opts.options_dir.clone().unwrap_or_else(|| std::env::current_dir().unwrap_or_default()),
        };
        let settings = gui::settings::GameSettings::load(&data_dir);
        self.data_dir = data_dir.clone();
        let stat_map = assets::Jar::open().and_then(|mut j| j.text("achievement/map.txt")).unwrap_or_default();
        gpu.apply_video(VideoSettings::from(&settings));
        let splash_seed =
            std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_nanos() as i64);
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
        for f in &mut self.fx { f.tick(&mut self.fx_rng); }
        let assets = assets::load_with(gui.ui.packs.path(), gui.ui.settings.anaglyph).unwrap_or(assets);
        gpu.refresh_textures(&assets);
        if let Err(e) = gui.refresh_textures(gpu.device(), gpu.queue()) { eprintln!("refreshTextures: {e}"); }
        gui.ui.stats = gui::stats::Stats::open(&data_dir, "Player", &stat_map);
        // Minecraft.loadScreen: the Mojang logo while starting up.
        gpu.render_overlay_only(false, &mut |device, queue, view, size| {
            gui.ui.gl.begin_frame();
            gui.draw_mojang_logo();
            gui.renderer.render(device, queue, view, size, &gui.ui.gl, Some([0.0, 0.0, 0.0, 0.0]));
        });
        if let Ok(mut jar) = assets::Jar::open() {
            if let (Ok(items), Ok(dial)) = (jar.image("gui/items.png"), jar.image("misc/dial.png")) {
                self.item_fx = Some(render::item_fx::ItemFx::new(&items, &dial));
            }
        }
        self.col = Some(Colorizers { grass: assets.grass_color, foliage: assets.foliage_color });
        let mut snd = crate::sound::SoundManager::new(&crate::sound::resources_dir(), splash_seed ^ 0x534f554e44);
        snd.options_changed(gui.ui.settings.sound_volume, gui.ui.settings.music_volume);
        self.sound = Some(snd);
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
                self.mouse_button(el, b, state == ElementState::Pressed);
            }
            WindowEvent::MouseWheel { delta, .. } => {
                // Mouse.getEventDWheel: up is positive; changeCurrentItem uses the sign.
                let y = match delta {
                    winit::event::MouseScrollDelta::LineDelta(_, y) => y as f64,
                    winit::event::MouseScrollDelta::PixelDelta(p) => p.y,
                };
                let screen_open = self.gui.as_ref().is_some_and(|g| g.screen.is_some());
                if screen_open && y!=0.0 {if let Some(g)=self.gui.as_mut(){if let Some(s)=g.screen.as_mut(){s.mouse_scrolled(&mut g.ui,y.signum() as i32);}}}
                if self.ready && !screen_open && y != 0.0 {
                    self.minecraft_events.mouse.push(crate::minecraft_input::MouseEvent {button:-1,pressed:false,wheel:y.signum() as i32,fresh:true});
                }
            }
            WindowEvent::Focused(false) => {
                if self.gpu.as_ref().is_some_and(|g|g.window.fullscreen().is_some()) {self.toggle_fullscreen();}
                if self.ready && self.loading.is_none() && self.script.is_none() {
                    // EntityRenderer: losing focus opens the pause menu.
                    self.display_ingame_menu();
                    self.apply_gui_actions(el);
                }
                self.set_grab(false);
            }
            WindowEvent::KeyboardInput { event, .. } => {
                let PhysicalKey::Code(code) = event.physical_key else {
                    return;
                };
                let c = event.text.as_ref().and_then(|t| t.chars().next()).unwrap_or('\0');
                self.key_input(el, code, c, event.state == ElementState::Pressed, event.repeat);
            }
            WindowEvent::RedrawRequested => {
                let started = Instant::now();
                if self.frame_deadline.is_some_and(|deadline| started < deadline) {
                    return;
                }
                self.frame(el);
                let video = self.video_settings();
                self.frame_deadline = video.frame_nanos().map(|nanos| {
                    let now = Instant::now();
                    if video.limit == 2 {
                        let remaining =
                            nanos as i64 - now.duration_since(started).as_nanos().min(i64::MAX as u128) as i64;
                        // Wait without blocking input; retain vanilla's whole-millisecond sleep.
                        now + Duration::from_millis(VideoSettings::power_sleep_millis(remaining, self.ready))
                    } else {
                        started + Duration::from_nanos(nanos)
                    }
                });
            }
            _ => {}
        }
    }

    fn device_event(&mut self, _el: &ActiveEventLoop, _id: winit::event::DeviceId, event: DeviceEvent) {
        if let DeviceEvent::MouseMotion { delta } = event {
            if self.grabbed && self.player.is_some() {
                // LWJGL counts y upward.
                self.mouse_delta.0 += delta.0;
                self.mouse_delta.1 -= delta.1;
            } else if self.grabbed {
                // Default sensitivity: 0.15 degrees per mouse count, as in EntityPlayerSP.setAngles.
                self.cam.yaw += delta.0 as f32 * 0.15;
                self.cam.pitch = (self.cam.pitch + delta.1 as f32 * 0.15).clamp(-90.0, 90.0);
            }
        }
    }

    fn about_to_wait(&mut self, el: &ActiveEventLoop) {
        if let Some(deadline) = self.frame_deadline.filter(|&d| d > Instant::now()) {
            el.set_control_flow(ControlFlow::WaitUntil(deadline));
        } else {
            el.set_control_flow(ControlFlow::Poll);
            if let Some(gpu) = &self.gpu {
                gpu.window.request_redraw();
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
    let ok = unsafe {
        GetProcessMemoryInfo(GetCurrentProcess(), &mut pmc, pmc.cb) != 0 && GlobalMemoryStatusEx(&mut ms) != 0
    };
    if !ok {
        return Default::default();
    }
    let total = pmc.PagefileUsage as i64;
    let used = pmc.WorkingSetSize as i64;
    gui::ingame::DebugInfo {
        max_memory: (ms.ullTotalPhys / 4) as i64,
        total_memory: total.max(used),
        free_memory: (total - used).max(0),
        ..Default::default()
    }
}

/// `World.playSoundEffect` -> `RenderGlobal.playSound`: only within 16 blocks (16 x volume
/// when louder than 1) of the view entity.
pub(crate) fn play_world_sound(
    snd: &mut crate::sound::SoundManager,
    view: DVec3,
    name: &str,
    pos: [f64; 3],
    volume: f32,
    pitch: f32,
) {
    let mut range = 16.0f32;
    if volume > 1.0 {
        range *= volume;
    }
    let d = DVec3::from_array(pos) - view;
    if d.length_squared() < (range * range) as f64 {
        snd.play_sound(name, [pos[0] as f32, pos[1] as f32, pos[2] as f32], volume, pitch);
    }
}
