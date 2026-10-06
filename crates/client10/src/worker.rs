//! The world thread: owns the `World`, runs it at 20 ticks per second, loads chunks the
//! way singleplayer or the multiplayer server does, and builds meshes for the renderer
//! window (`RenderGlobal`).

use crate::render::access::Access;
use crate::render::blocks::{Colorizers, RenderBlocks};
use crate::render::video::VideoSettings;
use crate::render::{rprops, Tessellator, Vertex};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::sync::mpsc::{self, Receiver, Sender};
use std::time::{Duration, Instant};
use worldgen::world::server::PlayerManager;
use worldgen::{BiomeSource, World};

pub enum ToWorker {
    EditSign { position:(i32,i32,i32), text:[String;4] },
    Camera {
        x: f64,
        y: f64,
        z: f64,
    },
    Settings(VideoSettings),
    RefreshTextures(Colorizers),
    /// `isGamePaused`: a screen that pauses the game is open (no ticks, no catch-up).
    Pause(bool),
    /// `World.func_650_a(n)`: the pause menu saves a few chunks every frame.
    QuickSave(i32),
    /// The player's keys and mouse since the last message (merged until the next tick).
    Input(InputFrame),
    MinecraftEvents { settings:crate::gui::settings::GameSettings, events:crate::minecraft_input::Events, screen:bool, focused:bool },
    /// A completed window-side screen/focus callback acknowledges the logical
    /// state already changed by runTick on the worker.
    MinecraftContext { screen:bool, focused:bool },
    MinecraftSettings(crate::gui::settings::GameSettings),
    /// `GuiGameOver` "Respawn".
    Respawn,
    /// A container screen request: open the inventory, click, close (see gui/CONTAINERS.md).
    Container(worldgen::container::ContainerCommand),
    Quit,
}

/// Player input gathered by the window thread. Movement keys and held buttons are states;
/// look deltas and scroll add up; presses latch until a tick consumes them.
#[derive(Clone, Copy, Debug, Default)]
pub struct InputFrame {
    pub forward: bool,
    pub back: bool,
    pub left: bool,
    pub right: bool,
    pub jump: bool,
    pub sneak: bool,
    pub yaw_delta: f32,
    pub pitch_delta: f32,
    pub left_pressed: bool,
    pub attack_presses: i32,
    pub left_held: bool,
    pub right_pressed: bool,
    pub use_presses: i32,
    pub right_held: bool,
    pub drop_pressed: bool,
    pub drop_presses: i32,
    /// `clickMiddleMouseButton`
    pub middle_pressed: bool,
    pub pick_presses: i32,
    pub hotbar_slot: Option<u8>,
    pub scroll: i32,
    /// Numbers the frames, so the window can tell which look deltas a tick has used.
    pub seq: u64,
}

impl InputFrame {
    /// Fold a newer frame into this pending one.
    pub fn merge(&mut self, n: InputFrame) {
        self.forward = n.forward;
        self.back = n.back;
        self.left = n.left;
        self.right = n.right;
        self.jump = n.jump;
        self.sneak = n.sneak;
        self.yaw_delta += n.yaw_delta;
        self.pitch_delta += n.pitch_delta;
        self.left_pressed |= n.left_pressed;
        self.attack_presses += n.attack_presses;
        self.left_held = n.left_held;
        self.right_pressed |= n.right_pressed;
        self.use_presses += n.use_presses;
        self.right_held = n.right_held;
        self.drop_pressed |= n.drop_pressed;
        self.drop_presses += n.drop_presses;
        self.middle_pressed |= n.middle_pressed;
        self.pick_presses += n.pick_presses;
        if n.hotbar_slot.is_some() {
            self.hotbar_slot = n.hotbar_slot;
        }
        self.scroll += n.scroll;
        self.seq = self.seq.max(n.seq);
    }

    fn take_tick_input(&mut self) -> worldgen::entity::player::PlayerTickInput {
        use worldgen::entity::player::{PlayerInput, PlayerTickInput};
        let t = PlayerTickInput {
            movement: PlayerInput {
                forward: self.forward,
                back: self.back,
                left: self.left,
                right: self.right,
                jump: self.jump,
                sneak: self.sneak,
                yaw_delta: self.yaw_delta,
                pitch_delta: self.pitch_delta,
            },
            left_pressed: self.left_pressed,
            attack_presses: self.attack_presses,
            left_held: self.left_held,
            right_pressed: self.right_pressed,
            use_presses: self.use_presses,
            pick_presses: self.pick_presses + self.middle_pressed as i32,
            right_held: self.right_held,
            drop_pressed: self.drop_pressed,
            drop_presses: self.drop_presses,
            hotbar_slot: self.hotbar_slot,
            scroll: self.scroll,
            ..Default::default()
        };
        self.yaw_delta = 0.0;
        self.pitch_delta = 0.0;
        self.left_pressed = false;
        self.attack_presses = 0;
        self.right_pressed = false;
        self.use_presses = 0;
        self.drop_pressed = false;
        self.drop_presses = 0;
        self.middle_pressed = false;
        self.pick_presses = 0;
        self.hotbar_slot = None;
        self.scroll = 0;
        t
    }
}

/// The actual player/controller adapter shared by the worker and runTick replay.
pub(crate) struct MinecraftHost<'a> {
    pub world:&'a mut World,
    pub player:worldgen::entity::Entity,
    pub controller:worldgen::entity::player::controller::PlayerControllerSP,
    pub target:Option<worldgen::entity::MovingObjectPosition>,
    pub screen:bool,
    pub focused:bool,
    pub multiplayer:bool,
    pub allow_input:bool,
    pub actions:Vec<crate::minecraft_input::Action>,
    pub drops:i32,
    pub clicks:[i32;6],
}
impl crate::minecraft_input::Host for MinecraftHost<'_> {
    fn screen(&self)->bool {self.screen}
    fn allow_input(&self)->bool {self.allow_input}
    fn focused(&self)->bool {self.focused}
    fn multiplayer(&self)->bool {self.multiplayer}
    fn left_counter(&mut self)->&mut i32 {&mut self.player.player_state_mut().unwrap().left_click_counter}
    fn right_counter(&mut self)->&mut i32 {&mut self.player.player_state_mut().unwrap().right_click_delay}
    fn action(&mut self, action:crate::minecraft_input::Action) {
        use crate::minecraft_input::Action;
        match action {
            Action::Focus=>{self.focused=true;self.screen=false;*self.left_counter()=10000;},
            Action::Menu|Action::Inventory|Action::Chat=>{
                self.screen=true;
                if self.focused {crate::gui::settings::KeyBinding::release_all();self.focused=false;}
            },
            _=>{},
        }
        self.actions.push(action);
    }
    fn scroll(&mut self, wheel:i32) {self.player.player_state_mut().unwrap().inventory.change_current_item(wheel);}
    fn hotbar(&mut self, slot:i32) {self.player.player_state_mut().unwrap().inventory.current_item=slot;}
    fn drop_item(&mut self) {
        self.drops+=1;
        let s=self.player.player_state_mut().unwrap();
        if let Some(stack)=s.inventory.decrement(s.inventory.current_item as usize,1) {self.player.drop_player_item(self.world,stack,false);}
    }
    fn using_item(&self)->bool {self.player.player_state().unwrap().item_in_use.is_some()}
    fn stop_using(&mut self) {self.player.stop_item_use(self.world);}
    fn click(&mut self, right:bool) {
        let before=self.player.player_state().unwrap().events.len();
        let callbacks=self.player.click_player(self.world,&mut self.controller,self.target,right);
        for bit in 0..6 {if callbacks&(1<<bit)!=0 {self.clicks[bit]+=1;}}
        // Original block/entity callbacks can display a screen inside clickMouse.
        // Its key release affects the remaining press-count loops immediately.
        if self.player.player_state().unwrap().events[before..].iter().any(|e|matches!(e,
            worldgen::entity::player::PlayerEvent::OpenContainer{..}
            |worldgen::entity::player::PlayerEvent::OpenVehicleInventory{..}
            |worldgen::entity::player::PlayerEvent::EditSign{..})) {
            self.screen=true;
            if self.focused {crate::gui::settings::KeyBinding::release_all();self.focused=false;}
            self.actions.push(crate::minecraft_input::Action::Container);
        }
    }
    fn pick(&mut self) {self.player.pick_block(self.world,self.target);}
    fn mine(&mut self, held:bool) {
        if held && self.player.player_state().unwrap().left_click_counter<=0 && matches!(self.target,Some(worldgen::entity::MovingObjectPosition::Block{..})) {
            if let Some(worldgen::entity::MovingObjectPosition::Block{x,y,z,..})=self.target {if self.player.can_edit_block181(x,y,z) {self.clicks[5]+=1;}}
        }
        self.player.send_click_block(self.world,&mut self.controller,self.target,held);
    }
}

fn minecraft_events(world:&mut World,id:i32,settings:&mut crate::gui::settings::GameSettings,
    events:crate::minecraft_input::Events,screen:bool,focused:bool,paused:bool)->(Vec<crate::minecraft_input::Action>,bool,bool) {
    let Some(mut player)=world.entities.remove(&id) else {return (Vec::new(),screen,focused);};
    let mut controller=std::mem::take(&mut player.player_state_mut().unwrap().controller);
    if !paused {controller.update_for_mode(player.player_state().unwrap().capabilities.creative);}
    let target=player.mouse_over(world,1.0);
    let multiplayer=world.multiplayer_world;
    let mut host=MinecraftHost {world,player,controller,target,screen,focused,multiplayer,
        allow_input:false,actions:Vec::new(),drops:0,clicks:[0;6]};
    crate::minecraft_input::run_tick(settings,&mut host,events);
    let states=["key.forward","key.back","key.left","key.right","key.jump","key.sneak"].map(|name|settings.binding_pressed(name));
    host.player.set_player_input(worldgen::entity::player::PlayerInput {forward:states[0],back:states[1],left:states[2],right:states[3],jump:states[4],sneak:states[5],..Default::default()});
    host.player.player_state_mut().unwrap().controller=host.controller;
    host.world.entities.insert(id,host.player);
    (host.actions,host.screen,host.focused)
}

/// `LoadingScreenRenderer` calls made by the world thread.
pub enum Loading {
    Title(String),
    Text(String),
    Progress(i32),
}

pub enum FromWorker {
    MinecraftActions { actions:Vec<crate::minecraft_input::Action>, settings:crate::gui::settings::GameSettings },
    Signs(Vec<crate::render::sign::Snapshot>, Vec<crate::render::entity::chest::Tile>, Vec<crate::render::spawner::Snapshot>),
    /// Ordered after old meshes, before meshes using the new settings.
    ResetMeshes,
    Ready {
        spawn: (f64, f64, f64),
        seed: i64,
        name: String,
    },
    Mesh {
        cx: i32,
        cz: i32,
        opaque: Vec<Vertex>,
        translucent: Vec<Vertex>,
    },
    Drop {
        cx: i32,
        cz: i32,
    },
    Time {
        time: i64,
        /// `EntityPlayer.getEntityBrightness` at the camera (vignette), loaded chunk count.
        brightness: f32,
        fog_brightness: f32,
        chunks: usize,
        loaded_entities: usize,
        weather: crate::render::weather::Weather,
        terrain: crate::render::weather::Snapshot,
    },
    /// Everything in the renderer window is loaded and meshed.
    Idle,
    Loading(Loading),
    /// Answer to `QuickSave`: true once nothing was left to save.
    QuickSaved(bool),
    /// Entities near the camera after a tick, with `getEntityBrightness`.
    Entities(Vec<crate::render::entity::effects::Snapshot>),
    /// Moving tile geometry and block audio events, separate from entity snapshots.
    BlockTiles { pistons:Vec<crate::render::pistons::Snapshot>,events:Vec<worldgen::world::notes::BlockEvent> },
    /// `addChatMessageTranslate(key)` (e.g. "tile.bed.notValid" on respawn).
    Chat(String),
    /// A container window opened (inventory key or a block activation).
    ContainerOpened(worldgen::container::ContainerSnapshot),
    /// The open container's contents after a click or a tick.
    ContainerUpdate(worldgen::container::ContainerSnapshot),
    /// Ordered presentation callbacks and collision/light data for EntityFX.
    Effects { events: Vec<worldgen::world::events::WorldEvent>, terrain: crate::render::weather::Snapshot },
    /// The player after a tick: entity (positions, rotation), HUD state, events.
    Player {
        entity: Box<worldgen::entity::Entity>,
        hud: Box<worldgen::entity::player::PlayerHud>,
        view: crate::render::player_view::Snapshot,
        events: Vec<worldgen::entity::player::PlayerEvent>,
        /// `isInsideOfMaterial(Material.water)` (the air bubbles).
        in_water: bool,
        /// The last input frame this tick consumed.
        input_seq: u64,
        /// `getScore()` for the game over screen.
        score: i32,
        /// Counts respawns, so messages about the previous (dead) player can be told apart.
        life: u32,
    },
    Saved,
    /// `MinecraftException` from `checkSessionLock`: the world was dropped unsaved.
    SessionConflict,
    Error(String),
}

#[derive(Clone)]
pub struct WorldOptions {
    pub settings: Option<worldgen::entity::player::food::WorldSettings>,
    pub dir: Option<PathBuf>,
    pub seed: i64,
    pub world_rand: i64,
    /// Multiplayer-server chunk loading/unloading instead of singleplayer's.
    pub server: bool,
    pub view_distance: i32,
    pub weather: Option<crate::render::weather::Weather>,
    pub time: Option<i64>,
    /// Name for a newly created world (`GuiCreateWorld`'s text box).
    pub name: Option<String>,
}

pub fn spawn(
    opts: WorldOptions,
    col: Colorizers,
    video: VideoSettings,
) -> (Sender<ToWorker>, Receiver<FromWorker>, std::thread::JoinHandle<()>) {
    let (to_tx, to_rx) = mpsc::channel();
    let (from_tx, from_rx) = mpsc::channel();
    let h = std::thread::Builder::new()
        .name("world".into())
        .spawn(move || run(opts, col, video, to_rx, from_tx))
        .expect("spawn world thread");
    (to_tx, from_rx, h)
}

/// `RenderGlobal.markRenderersForNewPosition` along one axis: the chunk coordinates the
/// renderer window covers around block position `p`.
pub(crate) fn window_axis(p: i32, wide: i32) -> Vec<i32> {
    let p = p - 8;
    let w = wide * 16;
    let half = w / 2;
    (0..wide)
        .map(|i| {
            let mut x = i * 16;
            let mut k = x + half - p;
            if k < 0 {
                k -= w - 1;
            }
            k /= w;
            x -= k * w;
            x >> 4
        })
        .collect()
}

fn run(opts: WorldOptions, mut col: Colorizers, mut video: VideoSettings, rx: Receiver<ToWorker>, tx: Sender<FromWorker>) {
    let t = Instant::now();
    let mut last_progress = -2;
    let mut progress = |step: worldgen::scenario::LoadStep| {
        use worldgen::scenario::LoadStep;
        let msg = match step {
            LoadStep::Title(t) => Loading::Title(t.to_string()),
            LoadStep::Text(t) => {
                last_progress = -2;
                Loading::Text(t.to_string())
            }
            LoadStep::Progress(p) => {
                if p == last_progress {
                    return;
                }
                last_progress = p;
                Loading::Progress(p)
            }
        };
        let _ = tx.send(FromWorker::Loading(msg));
    };
    // Minecraft.startWorld: a world from before McRegion is converted first.
    let mut open_seed=opts.seed;
    let mut open_settings=opts.settings;
    if let Some(dir) = opts.dir.as_deref() {
        if worldgen::convert::is_old_map_format(dir) {
            worldgen::convert::convert_map_format(dir, &mut progress);
            // convertMapFormat restarts through startWorld with WorldSettings(0,0,true).
            open_seed=0;
            open_settings=Some(worldgen::entity::player::food::WorldSettings {seed:0,game_mode:0,map_features:true});
        }
    }
    progress(worldgen::scenario::LoadStep::Title("Switching level"));
    progress(worldgen::scenario::LoadStep::Text(""));
    let opened = World::open_with_settings(
        opts.dir.as_deref(),
        open_seed,
        opts.world_rand,
        opts.server,
        opts.name.as_deref(),
        open_settings,
        &mut progress,
    );
    let (mut world, is_new) = match opened {
        Ok(w) => w,
        Err(e) => {
            let _ = tx.send(FromWorker::Error(format!("could not open world: {e}")));
            return;
        }
    };
    if let Some(w) = opts.weather {
        world.info.raining = w.rain > 0.0;
        world.info.thundering = w.thunder > 0.0;
        world.info.rain_time = 1000000;
        world.info.thunder_time = 1000000;
        world.raining_strength = w.rain;
        world.prev_raining_strength = w.prev_rain;
        world.thundering_strength = w.thunder;
        world.prev_thundering_strength = w.prev_thunder;
        world.lightning_flash = w.flash;
        world.lightning_sky_flash = w.flash;
    }
    if let Some(time) = opts.time {
        world.world_time = time;
    }
    // RenderGlobal.loadRenderers runs after the first-world preload.
    world.leaves_opaque = !video.fancy;
    world.skylight_subtracted = world.calculate_skylight_subtracted(1.0);
    let (sx, _, sz) = world.spawn;
    // The saved player, if the real game left one; otherwise stand on the spawn column.
    let mut start = None;
    if let Some(p) = &world.info.player {
        if let [worldgen::nbt::Tag::Double(x), worldgen::nbt::Tag::Double(y), worldgen::nbt::Tag::Double(z)] =
            p.list("Pos")
        {
            start = Some((*x, *y + 1.62, *z));
        }
    }
    let start = start.unwrap_or_else(|| {
        let mut top = 127;
        while top > 0 && world.get_block_id(sx, top, sz) == 0 {
            top -= 1;
        }
        (sx as f64 + 0.5, top as f64 + 1.0 + 1.62, sz as f64 + 0.5)
    });
    eprintln!(
        "world: {} '{}', seed {}, spawn {:?}, {} chunks ready in {:.2?}{}",
        if is_new { "created" } else { "loaded" },
        world.info.name,
        world.seed,
        world.spawn,
        world.chunks.len(),
        t.elapsed(),
        if opts.server { " (server chunk handling)" } else { "" }
    );
    // Singleplayer: the real player (Minecraft.changeWorld creates thePlayer).
    let player = if opts.server { None } else { Some(world.spawn_player("Player")) };
    if let Some(id) = player {
        if let Some(e) = world.entities.get_mut(&id) { e.initialize_player_mode(worldgen::entity::player::controller::PlayerControllerSP::saved_world_is_creative(world.info.game_type)); }
    }
    let mut input = InputFrame::default();
    let mut minecraft_input_active = false;
    let mut minecraft_context=None;
    let mut minecraft_settings=None;
    let mut life = 0u32;
    let start = match player.and_then(|id| world.entities.get(&id)) {
        Some(e) => (e.pos.x, e.pos.y, e.pos.z),
        None => start,
    };
    let _ = tx.send(FromWorker::Ready { spawn: start, seed: world.seed, name: world.info.name.clone() });

    let mut players = if opts.server {
        let mut pm = PlayerManager::new(opts.view_distance.clamp(3, 15));
        pm.add_player(&mut world, start.0, start.2);
        Some(pm)
    } else {
        None
    };

    let biomes = RefCell::new(biome_source(&world));
    let mut cam = start;
    let mut sort_pos = (f64::MAX, f64::MAX, f64::MAX);
    let mut window: Vec<(i32, i32)> = Vec::new();
    let mut window_set: HashSet<(i32, i32)>;
    let mut meshed: HashMap<(i32, i32), u64> = HashMap::new();
    let mut sky_sub = world.skylight_subtracted;
    let mut join_player_counter = 0;
    let mut weather_tail = 0;
    let mut next_tick = Instant::now();
    let mut idle_reported = false;
    // After a dimension change: RenderGlobal.changeWorld (every renderer rebuilt).
    macro_rules! switched_world {
        () => {
            world.leaves_opaque = !video.fancy;
            *biomes.borrow_mut() = biome_source(&world);
            sky_sub = world.skylight_subtracted;
            meshed.clear();
            sort_pos = (f64::MAX, f64::MAX, f64::MAX);
            idle_reported = false;
            let _ = tx.send(FromWorker::ResetMeshes);
        };
    }

    let mut paused = false;

    loop {
        // Messages.
        loop {
            match rx.try_recv() {
                Ok(ToWorker::Camera { x, y, z }) => cam = (x, y, z),
                Ok(ToWorker::RefreshTextures(next)) => {
                    col = next;
                    meshed.clear();
                    sort_pos = (f64::MAX, f64::MAX, f64::MAX);
                    idle_reported = false;
                    let _ = tx.send(FromWorker::ResetMeshes);
                }
                Ok(ToWorker::Settings(next)) => {
                    if next.remesh(video) {
                        meshed.clear();
                        sort_pos = (f64::MAX, f64::MAX, f64::MAX);
                        idle_reported = false;
                        let _ = tx.send(FromWorker::ResetMeshes);
                    }
                    world.leaves_opaque = !next.fancy;
                    video = next;
                }
                Ok(ToWorker::Input(f)) => {
                    // EntityRenderer.updateCameraAndRender turns the player every frame,
                    // between ticks (Entity.setAngles).
                    if let Some(e) = player.and_then(|id| world.entities.get_mut(&id)) {
                        e.set_angles(f.yaw_delta, f.pitch_delta);
                    }
                    input.merge(InputFrame { yaw_delta: 0.0, pitch_delta: 0.0, ..f });
                }
                Ok(ToWorker::MinecraftContext {screen,focused}) => {
                    let was_focused=minecraft_context.is_some_and(|(_,focus)|focus);
                    if focused && !was_focused {
                        if let Some(s)=player.and_then(|id|world.entities.get_mut(&id)).and_then(|e|e.player_state_mut()) {s.left_click_counter=10000;}
                    }
                    minecraft_context=Some((screen,focused));
                }
                Ok(ToWorker::MinecraftSettings(settings)) => {minecraft_settings=Some(settings);}
                Ok(ToWorker::MinecraftEvents {settings,events,screen,focused}) => {
                    if let Some(id)=player {
                        minecraft_input_active=true;
                        let settings=minecraft_settings.get_or_insert(settings);
                        let (screen,focused)=minecraft_context.unwrap_or((screen,focused));
                        let (actions,screen,focused)=minecraft_events(&mut world,id,settings,events,screen,focused,paused);
                        minecraft_context=Some((screen,focused));
                        let policy=crate::minecraft_input::tick_policy(&mut join_player_counter,settings.difficulty,paused,world.multiplayer_world);
                        world.difficulty=policy.difficulty;
                        if policy.world {world.spawn_hostile_mobs=settings.difficulty>0;world.spawn_peaceful_mobs=true;}
                        if policy.join {
                            if let Some(e)=world.entities.get(&id) {let pos=e.pos;world.join_entity_in_surroundings(pos);}
                        }
                        let _=tx.send(FromWorker::MinecraftActions {actions,settings:settings.clone()});
                    }
                }
                Ok(ToWorker::Container(command)) => {
                    if let Some(id) = player {
                        let opens = matches!(
                            command,
                            worldgen::container::ContainerCommand::OpenInventory
                                | worldgen::container::ContainerCommand::OpenBlock { .. }
                        );
                        if let Some(s) = world.container_command(id, command) {
                            let _ = tx.send(if opens {
                                FromWorker::ContainerOpened(s)
                            } else {
                                FromWorker::ContainerUpdate(s)
                            });
                        }
                    }
                }
                Ok(ToWorker::EditSign { position:(x,y,z), text }) => {
                    if let Some(worldgen::world::tile::TileEntity::Sign(sign))=world.get_block_tile_entity(x,y,z) { sign.text=text; }
                    if let Some(c)=world.chunks.get_mut(&(x>>4,z>>4)) {c.modified=true;}
                }
                Ok(ToWorker::Respawn) => {
                    // Minecraft.respawn(false, 0): out of the Nether first (it can't be
                    // respawned in), then a new player at the bed or the spawn point.
                    if let Some(old) = player {
                        // These are Minecraft fields, although the simulation adapter
                        // stores them beside the player. Replacing the player must not
                        // reset either click counter.
                        let click_counters=world.entities.get(&old).and_then(|e|e.player_state())
                            .map(|s|(s.left_click_counter,s.right_click_delay));
                        if world.dimension.is_hell_world() {
                            if let Some(next) = world.use_portal(old, fresh_seed(), fresh_seed(), &mut progress) {
                                world = next;
                                switched_world!();
                            }
                        }
                        if let Some(key) = world.respawn_player(old, &mut progress) {
                            let _ = tx.send(FromWorker::Chat(key.to_string()));
                        }
                        if let Some(e)=world.entities.get_mut(&old){e.initialize_player_mode(worldgen::entity::player::controller::PlayerControllerSP::saved_world_is_creative(world.info.game_type));}
                        if let (Some((left,right)),Some(s))=(click_counters,world.entities.get_mut(&old).and_then(|e|e.player_state_mut())) {
                            s.left_click_counter=left;s.right_click_delay=right;
                        }
                        life += 1;
                        if let Some(e) = world.entities.get(&old) {
                            cam = (e.pos.x, e.pos.y, e.pos.z);
                        }
                        let _ = tx.send(FromWorker::Ready { spawn: cam, seed: world.seed, name: world.info.name.clone() });
                    }
                }
                Ok(ToWorker::Pause(p)) => {
                    if paused && !p {
                        // The Timer kept running while paused; those ticks are not caught up.
                        next_tick = Instant::now();
                    }
                    paused = p;
                }
                Ok(ToWorker::QuickSave(n)) => {
                    if n == 0 {
                        world.save_level();
                    }
                    if world.session_conflict {
                        let _ = tx.send(FromWorker::SessionConflict);
                        return;
                    }
                    let done = world.save_chunks(false);
                    let _ = tx.send(FromWorker::QuickSaved(done));
                }
                Ok(ToWorker::Quit) | Err(mpsc::TryRecvError::Disconnected) => {
                    // Minecraft quitting to the title screen / server shutdown: save everything.
                    // changeWorld(null, ""): printText(""), then saveWorldIndirectly.
                    let _ = tx.send(FromWorker::Loading(Loading::Title(String::new())));
                    let _ = tx.send(FromWorker::Loading(Loading::Text("Saving chunks".into())));
                    if let Some(pm) = &mut players {
                        pm.remove_player(&mut world);
                    }
                    world.save_world_indirectly();
                    let _ = tx.send(FromWorker::Saved);
                    return;
                }
                Err(mpsc::TryRecvError::Empty) => break,
            }
        }

        // Ticks (at most a second's worth of catch-up).
        let mut ticks = 0;
        if paused {
            // No ticks while paused, but keep the clock moving so meshing isn't starved.
            next_tick = Instant::now() + Duration::from_millis(50);
        }
        while !paused && Instant::now() >= next_tick && ticks < 20 {
            if let Some(id) = player {
                // Minecraft.runTick with thePlayer: input/controller, joinEntityInSurroundings
                // every 30 ticks, updateEntities, World.tick, randomDisplayUpdates.
                if !minecraft_input_active {join_player_counter += 1;}
                // Mouse events come before keyboard events in runTick: every wheel event
                // is its own changeCurrentItem. Pick presses run after attack/use queues.
                if let Some(s) = world.entities.get_mut(&id).and_then(|e| e.player_state_mut()) {
                    for _ in 0..input.scroll.abs() {
                        s.inventory.change_current_item(input.scroll.signum());
                    }
                }
                input.scroll = 0;
                if minecraft_input_active {
                    let _=input.take_tick_input();
                    if world.lightning_sky_flash>0 {world.lightning_sky_flash-=1;}
                    if let Some(index)=world.entities.get(&id).and_then(|e|e.player_state()).and_then(|s|s.player_index) {
                        world.client_tick(index,1);
                    }
                } else {
                    world.player_tick(id, input.take_tick_input(), join_player_counter);
                }
                if let Some(e) = world.entities.get(&id) {
                    cam = (e.pos.x, e.pos.y, e.pos.z);
                }
            } else if players.is_none() {
                world.players.clear();
                world.players.push(cam);
                join_player_counter += 1;
                world.client_tick(0, join_player_counter);
            } else {
                world.players.clear();
                world.players.push(cam);
                // MinecraftServer.doTick: world tick, lighting, then entities.
                world.tick();
                world.drain_lighting();
                world.update_entities();
            }
            // The client relights a little every frame and the server once per tick; here
            // the queue is simply emptied.
            world.drain_lighting();
            if world.session_conflict {
                // Minecraft.run: theWorld = null, changeWorld1(null) (nothing saved), then
                // GuiConflictWarning.
                let _ = tx.send(FromWorker::SessionConflict);
                return;
            }
            if let Some(pm) = &mut players {
                // MinecraftServer.doTick: world tick, lighting, then player movement packets.
                pm.update_player(&mut world, cam.0, cam.2);
            }
            if world.renderers_dirty {
                // skylightSubtracted changed (dusk, dawn, rain): updateAllRenderers.
                world.renderers_dirty = false;
                sky_sub = world.skylight_subtracted;
                meshed.clear();
                idle_reported = false;
            }
            let weather = crate::render::weather::Weather {
                rain: world.raining_strength,
                prev_rain: world.prev_raining_strength,
                thunder: world.thundering_strength,
                prev_thunder: world.prev_thundering_strength,
                // Sky color reads field_27172_i (the bolt field), not the weather counter.
                // Vanilla never decrements this field; retain that bug.
                flash: world.lightning_sky_flash,
            };
            if weather.rain > 0.0 || weather.prev_rain > 0.0 {
                weather_tail = 40;
            }
            let terrain = if weather_tail > 0 {
                weather_tail -= 1;
                weather_snapshot(&mut world, cam)
            } else {
                crate::render::weather::Snapshot::default()
            };
            let brightness = entity_brightness(&mut world, cam);
            let l = world.get_block_light_value(cam.0.floor() as i32, cam.1.floor() as i32, cam.2.floor() as i32);
            let fog_brightness = world.light_brightness_table[l as usize];
            // Entities near the camera, with their brightness, for the renderer.
            let near: Vec<i32> = world
                .loaded_entity_list
                .iter()
                .copied()
                // RenderGlobal skips the view entity in first person.
                .filter(|id| Some(*id) != player)
                .filter(|id| {
                    world.entities.get(id).is_some_and(|e| {
                        let (dx, dy, dz) = (e.pos.x - cam.0, e.pos.y - cam.1, e.pos.z - cam.2);
                        // Entity.isInRangeToRenderDist (renderDistanceWeight 1): far mobs are not
                        // drawn; snowballs, eggs, fireballs and bobbers override it with 4x.
                        use worldgen::entity::EntityKind as K;
                        let range = e.bounding_box.average_edge_length()
                            * if matches!(e.kind, K::Fish(_) | K::Snowball(_) | K::Egg(_) | K::Fireball(_)) { 4.0 } else { 1.0 }
                            * 64.0;
                        dx * dx + dy * dy + dz * dz < range * range
                    })
                })
                .collect();
            let mut snapshot = Vec::with_capacity(near.len());
            for id in near {
                let e = world.entities[&id].clone();
                let b = living_brightness(&mut world, &e);
                let biomes = RefCell::new(biome_source(&world));
                let access = Access::new(&world, e.chunk_x, e.chunk_z, world.skylight_subtracted, &biomes);
                let fl = worldgen::math::floor_double;
                let (x, y, z) = (fl(e.pos.x), fl(e.pos.y), fl(e.pos.z));
                let ds = [[0, 0, 0], [0, -1, 0], [0, 1, 0], [0, 0, -1], [0, 0, 1], [-1, 0, 0], [1, 0, 0]];
                let falling_light = ds.map(|[dx, dy, dz]| {
                    world.light_brightness_table[access.light_value(x + dx, y + dy, z + dz) as usize]
                });
                let size = crate::render::entity::effects::radius(&e) as f64;
                let mut surfaces = Vec::new();
                // Cover both endpoints of the partial-tick interval, queried from loaded chunks only.
                for xx in fl(e.pos.x.min(e.last_tick_pos.x) - size)..=fl(e.pos.x.max(e.last_tick_pos.x) + size) {
                    for yy in fl(e.pos.y.min(e.last_tick_pos.y) + e.height as f64 / 2.0 - size)
                        ..=fl(e.pos.y.max(e.last_tick_pos.y) + e.height as f64 / 2.0)
                    {
                        for zz in fl(e.pos.z.min(e.last_tick_pos.z) - size)..=fl(e.pos.z.max(e.last_tick_pos.z) + size)
                        {
                            let block = access.block(xx, yy - 1, zz);
                            if block > 0 && access.light_value(xx, yy, zz) > 3 {
                                surfaces.push(crate::render::entity::effects::Surface {
                                    xyz: [xx, yy, zz],
                                    block,
                                    brightness: world.light_brightness_table[access.light_value(xx, yy, zz) as usize],
                                });
                            }
                        }
                    }
                }
                for xyz in crate::render::painting::light_positions(&e) {
                    let [x,y,z]=xyz;surfaces.push(crate::render::entity::effects::Surface{xyz,block:0,brightness:world.light_brightness_table[access.light_value(x,y,z) as usize]});
                }
                snapshot.push(crate::render::entity::effects::Snapshot {
                    vehicle_rails: if matches!(e.kind,worldgen::entity::EntityKind::Minecart(..)) {
                        Some(crate::render::entity::vehicles::RailSnapshot::capture(&e,|x,y,z|(access.block(x,y,z),access.meta(x,y,z))))
                    } else {None},
                    light: Some(entity_light(&mut world, &e)),
                    entity: e,
                    brightness: b,
                    falling_light,
                    surfaces,
                });
            }
            // Lightning lives in World.weatherEffects, separately from loadedEntityList.
            // Keep even inactive bolts so their next restrike uses the updated entity seed.
            snapshot.extend(world.weather_effects.iter().map(|e| crate::render::entity::effects::Snapshot {
                entity: e.clone(),
                brightness: 1.0,
                light: None,
                falling_light: [1.0; 7],
                vehicle_rails: None,
                surfaces: Vec::new(),
            }));
            let _ = tx.send(FromWorker::Entities(snapshot));
            let mut pistons=Vec::new();
            for id in &world.loaded_tile_entities {
                let object=&world.tile_objects[id];
                if object.invalid {continue;}
                if let Some(worldgen::world::tile::TileEntity::Piston(p))=world.tile_object(*id) {
                    let (x,y,z)=object.position;
                    if (x as f64-cam.0).abs()>128.0 || (z as f64-cam.2).abs()>128.0 {continue;}
                    let mut access=Access::new(&world,x>>4,z>>4,world.skylight_subtracted,&biomes);
                    access.fancy=video.fancy;
                    pistons.push(crate::render::pistons::Snapshot::new(&access,&col,video.ao,[x,y,z],p.clone()));
                }
            }
            let _=tx.send(FromWorker::BlockTiles {pistons,events:std::mem::take(&mut world.block_events)});
            let mut signs=Vec::new();
            for (x,y,z) in world.loaded_tile_entities.iter().filter(|id| !world.tile_objects[id].invalid).map(|id| world.tile_objects[id].position).collect::<Vec<_>>() {
                let d=(x as f64+0.5-cam.0).powi(2)+(y as f64+0.5-cam.1).powi(2)+(z as f64+0.5-cam.2).powi(2);
                if d>=4096.0 {continue;}
                let sign=match world.get_block_tile_entity(x,y,z) {Some(worldgen::world::tile::TileEntity::Sign(s))=>s.clone(),_=>continue};
                let block=world.get_block_id(x,y,z);let metadata=world.get_block_metadata(x,y,z);
                // b1.8.1 TileEntityRenderer: colour 1, lightmap at the tile's packed brightness.
                let light=world.packed_light181(x,y,z,0) as u32;
                signs.push(crate::render::sign::Snapshot{position:[x,y,z],block,metadata,sign,brightness:1.0,light});
            }
            // Chests are tile-entity models in b1.8.1 (TileEntityChestRenderer), not chunk geometry.
            let mut chests=Vec::new();
            let ids:Vec<_>=world.loaded_tile_entities.iter().copied().filter(|id| !world.tile_objects[id].invalid).collect();
            for id in ids {
                if !matches!(world.tile_object(id),Some(worldgen::world::tile::TileEntity::Chest(_))) {continue;}
                let (x,y,z)=world.tile_objects[&id].position;
                let d=(x as f64+0.5-cam.0).powi(2)+(y as f64+0.5-cam.1).powi(2)+(z as f64+0.5-cam.2).powi(2);
                if d>=4096.0 || world.get_block_id(x,y,z)!=54 {continue;}
                // The renderer orients unset chests (BlockChest.func_35306_h) and finds neighbours.
                if world.get_block_metadata(x,y,z)==0 {world.orient_chest181(x,y,z);}
                world.check_chest_adjacency181(id);
                let (metadata,light)=(world.get_block_metadata(x,y,z),world.packed_light181(x,y,z,0) as u32);
                let a=&world.chest_animations[&id];
                chests.push(crate::render::entity::chest::Tile{position:[x,y,z],metadata,
                    adjacent:a.adjacent.map(|o|o.is_some()),lid:a.lid,prev_lid:a.previous_lid,light});
            }
            // TileEntityMobSpawnerRenderer: the spinning mob, lit at the spawner.
            let mut spawners=Vec::new();
            let ids:Vec<_>=world.loaded_tile_entities.iter().copied().filter(|id| !world.tile_objects[id].invalid).collect();
            for id in ids {
                let Some(worldgen::world::tile::TileEntity::MobSpawner{mob,..})=world.tile_object(id) else {continue};
                let mob=mob.clone();
                let (x,y,z)=world.tile_objects[&id].position;
                let d=(x as f64+0.5-cam.0).powi(2)+(y as f64+0.5-cam.1).powi(2)+(z as f64+0.5-cam.2).powi(2);
                if d>=4096.0 {continue;}
                let (yaw,prev_yaw)=world.spawner_animations.get(&(x,y,z)).copied().unwrap_or((0.0,0.0));
                spawners.push(crate::render::spawner::Snapshot{position:[x,y,z],mob,yaw,prev_yaw,light:world.packed_light181(x,y,z,0) as u32});
            }
            let _=tx.send(FromWorker::Signs(signs,chests,spawners));
            if let Some(id) = player {
                let events = world.take_player_events(id);
                // Block activations open their container right here on the world thread.
                for e in &events {
                    if let Some(c) = crate::gui::container_api::activation_request(e) {
                        if let Some(s) = world.container_command(id, c) {
                            let _ = tx.send(FromWorker::ContainerOpened(s));
                        }
                    }
                }
                // The open container's tiles change on their own (furnaces): one snapshot a tick.
                if let Some(s) = world.container_snapshot(id) {
                    let _ = tx.send(FromWorker::ContainerUpdate(s));
                }
                // Dispenser sounds and smoke are drawn by the effects code once it is in.
                world.container_effects.clear();
                if events.iter().any(|e| matches!(e, worldgen::entity::player::PlayerEvent::ChangeDimension)) {
                    // Minecraft.usePortal: save, switch worlds, find or build the portal.
                    if let Some(next) = world.use_portal(id, fresh_seed(), fresh_seed(), &mut progress) {
                        world = next;
                        switched_world!();
                        if let Some(e) = world.entities.get(&id) {
                            cam = (e.pos.x, e.pos.y, e.pos.z);
                        }
                        let _ = tx.send(FromWorker::Ready { spawn: cam, seed: world.seed, name: world.info.name.clone() });
                    }
                }
                if let (Some(e), Some(hud)) = (world.entities.get(&id).cloned(), world.player_hud(id, 1.0)) {
                    let in_water = e.is_inside_material(&mut world, worldgen::block::table::MAT_WATER);
                    let score = e.player_state().map_or(0, |s| s.score);
                    let _ = tx.send(FromWorker::Player {
                        view: crate::render::player_view::Snapshot::capture(&world, &e),
                        entity: Box::new(e),
                        hud: Box::new(hud),
                        events,
                        in_water,
                        input_seq: input.seq,
                        score,
                        life,
                    });
                }
            }
            // World effects: drain once after the player and world tick, before Time.
            let events = world.drain_events();
            let effect_terrain = weather_snapshot_radius(&mut world, cam, 20);
            let _ = tx.send(FromWorker::Effects { events, terrain: effect_terrain });
            let chunks = world.chunks.len();
            let _ = tx.send(FromWorker::Time {
                time: world.world_time,
                brightness,
                fog_brightness,
                chunks,
                loaded_entities:world.loaded_entity_list.len(),
                weather,
                terrain,
            });
            next_tick += Duration::from_millis(50);
            ticks += 1;
        }
        if ticks == 20 {
            next_tick = Instant::now();
        }

        // Unloaded chunks lose their meshes.
        for (cx, cz) in std::mem::take(&mut world.unloaded) {
            if meshed.remove(&(cx, cz)).is_some() {
                let _ = tx.send(FromWorker::Drop { cx, cz });
            }
            idle_reported = false;
        }

        // Renderer window: re-centred after moving more than 4 blocks.
        let (dx, dy, dz) = (cam.0 - sort_pos.0, cam.1 - sort_pos.1, cam.2 - sort_pos.2);
        if dx * dx + dy * dy + dz * dz > 16.0 {
            sort_pos = cam;
            let xs = window_axis(cam.0.floor() as i32, video.chunks_wide());
            let zs = window_axis(cam.2.floor() as i32, video.chunks_wide());
            window.clear();
            for &x in &xs {
                for &z in &zs {
                    window.push((x, z));
                }
            }
            window_set = window.iter().copied().collect();
            let gone: Vec<(i32, i32)> = meshed.keys().filter(|k| !window_set.contains(k)).copied().collect();
            for (cx, cz) in gone {
                meshed.remove(&(cx, cz));
                let _ = tx.send(FromWorker::Drop { cx, cz });
            }
            idle_reported = false;
        }
        let (ccx, ccz) = ((cam.0.floor() as i32) >> 4, (cam.2.floor() as i32) >> 4);
        window.sort_by_key(|&(x, z)| (x - ccx) * (x - ccx) + (z - ccz) * (z - ccz));

        // Load (singleplayer) and mesh, nearest first, until the next tick is due.
        let mut did_work = false;
        for i in 0..window.len() {
            if Instant::now() >= next_tick {
                did_work = true;
                break;
            }
            let (cx, cz) = window[i];
            let mut complete = true;
            for nx in cx - 1..=cx + 1 {
                for nz in cz - 1..=cz + 1 {
                    if !world.chunk_exists(nx, nz) {
                        if players.is_none() {
                            // Singleplayer renderers pull chunks in through ChunkCache.
                            world.load_chunk(nx, nz);
                            world.drain_lighting();
                            did_work = true;
                        } else {
                            complete = false;
                        }
                    }
                }
            }
            if !complete {
                continue;
            }
            let mut version = 0;
            for nx in cx - 1..=cx + 1 {
                for nz in cz - 1..=cz + 1 {
                    version = version.max(world.chunk(nx, nz).version);
                }
            }
            if meshed.get(&(cx, cz)).is_some_and(|&v| v >= version) {
                continue;
            }
            let (opaque, translucent) = mesh_chunk(&world, cx, cz, sky_sub, &biomes, &col, video);
            meshed.insert((cx, cz), version);
            did_work = true;
            if tx.send(FromWorker::Mesh { cx, cz, opaque, translucent }).is_err() {
                return;
            }
        }
        if !did_work {
            if !idle_reported {
                idle_reported = true;
                let _ = tx.send(FromWorker::Idle);
            }
            let wait = next_tick.saturating_duration_since(Instant::now()).min(Duration::from_millis(50));
            std::thread::sleep(wait);
        }
    }
}

/// `Entity.getEntityBrightness` for a simulated entity.
fn living_brightness(world: &mut World, e: &worldgen::entity::Entity) -> f32 {
    e.brightness(world)
}
fn entity_light(world: &mut World, e: &worldgen::entity::Entity) -> u32 {
    e.packed_brightness(world, 0.0) as u32
}

/// `Entity.getEntityBrightness` for a player whose eyes are at `cam` (1.8 tall, eyes 1.62 up).
fn entity_brightness(world: &mut World, cam: (f64, f64, f64)) -> f32 {
    let feet = cam.1 - 1.62;
    let (x0, x1) = (cam.0 - 0.3, cam.0 + 0.3);
    let (z0, z1) = (cam.2 - 0.3, cam.2 + 0.3);
    let fl = worldgen::math::floor_double;
    if !world.check_chunks_exist(fl(x0), fl(feet), fl(z0), fl(x1), fl(feet + 1.8), fl(z1)) {
        return 0.0;
    }
    let h = (feet + 1.8 - feet) * 0.66;
    let (x, y, z) = (fl(cam.0), fl(cam.1 - 1.62 + h), fl(cam.2));
    let l = world.get_block_light_value(x, y, z);
    world.light_brightness_table[l.clamp(0, 15) as usize]
}

/// `WorldRenderer.updateRenderer` for a whole 16x128x16 chunk: both render passes.
pub fn mesh_chunk(
    world: &World,
    cx: i32,
    cz: i32,
    sky_sub: i32,
    biomes: &RefCell<BiomeSource>,
    col: &Colorizers,
    video: VideoSettings,
) -> (Vec<Vertex>, Vec<Vertex>) {
    let mut access = Access::new(world, cx, cz, sky_sub, biomes);
    access.fancy = video.fancy;
    let origin = [cx as f64 * 16.0, 0.0, cz as f64 * 16.0];
    let mut t0 = Tessellator::new(origin);
    let mut t1 = Tessellator::new(origin);
    let chunk = world.chunk(cx, cz);
    for y in 0..128 {
        for z in 0..16 {
            for x in 0..16 {
                let id = chunk.block(x, y, z);
                if id == 0 || !rprops(id).exists {
                    continue;
                }
                let t = if rprops(id).pass == 0 { &mut t0 } else { &mut t1 };
                let mut rb = RenderBlocks::new(&access, t, col);
                rb.ao = video.ao;
    rb.anaglyph = video.anaglyph;
                rb.render_block(id, cx * 16 + x, y, cz * 16 + z);
            }
        }
    }
    (t0.verts, t1.verts)
}

/// World.getLightBrightness uses World (y>=128 clamps to 127), not ChunkCache.
pub fn weather_snapshot(world: &mut World, cam: (f64, f64, f64)) -> crate::render::weather::Snapshot {
    weather_snapshot_radius(world, cam, 10)
}

fn weather_snapshot_radius(world: &mut World, cam: (f64, f64, f64), radius: i32) -> crate::render::weather::Snapshot {
    use crate::render::weather::{Column, Snapshot};
    let (cx, cz) = (worldgen::math::floor_double(cam.0), worldgen::math::floor_double(cam.2));
    // The Nether's single hell biome has no rain or snow.
    let mut bs = biome_source(world);
    let mut biomes = Vec::new();
    bs.biomes(&mut biomes, cx - radius, cz - radius, radius * 2 + 1, radius * 2 + 1);
    let mut columns = Vec::with_capacity(((radius * 2 + 1) * (radius * 2 + 1)) as usize);
    for x in cx - radius..=cx + radius {
        for z in cz - radius..=cz + radius {
            // b1.8.1 func_35461_e: the cached precipitation height.
            let top = world.precipitation_height181(x, z);
            let biome = biomes[columns.len()];
            let mut c = Column {
                x,
                z,
                top,
                biome,
                rain: biome.enable_rain(),
                snow: biome.enable_snow(),
                particle_rain: crate::render::weather::rains(bs.biome_at(x, z)),
                blocks: [0; 128],
                meta: [0; 128],
                brightness: [0.0; 129],
                packed_light: [0; 129],
            };
            for y in 0..128 {
                c.blocks[y] = world.get_block_id(x, y as i32, z);
                c.meta[y] = world.get_block_metadata(x, y as i32, z) as u8;
                c.packed_light[y] = world.packed_light181(x, y as i32, z, 0);
                let l = world.get_block_light_value(x, y as i32, z);
                c.brightness[y] = world.light_brightness_table[l as usize];
            }
            c.packed_light[128] = world.packed_light181(x, 128, z, 0);
            let light = world.get_block_light_value(x, 128, z);
            c.brightness[128] = world.light_brightness_table[light as usize];
            columns.push(c);
        }
    }
    Snapshot { columns }
}

/// The biome source the mesher colors with: the Nether's is a single hell biome.
fn biome_source(world: &World) -> BiomeSource {
    if world.dimension.is_hell_world() {
        BiomeSource::hell()
    } else {
        BiomeSource::new(world.seed)
    }
}

/// Stands in for an unseeded `new Random()`.
fn fresh_seed() -> i64 {
    let nanos = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_nanos() as i64);
    worldgen::JavaRandom::new(nanos ^ 0x5DEECE66D).next_long()
}
