//! `GameSettings` / `EnumOptions`, stored in `options.txt` in the original format.

use super::lang::Lang;
use std::path::PathBuf;

const RENDER_DISTANCES: [&str; 4] =
    ["options.renderDistance.far", "options.renderDistance.normal", "options.renderDistance.short", "options.renderDistance.tiny"];
const DIFFICULTIES: [&str; 4] =
    ["options.difficulty.peaceful", "options.difficulty.easy", "options.difficulty.normal", "options.difficulty.hard"];
const GUISCALES: [&str; 4] = ["options.guiScale.auto", "options.guiScale.small", "options.guiScale.normal", "options.guiScale.large"];
const LIMIT_FRAMERATES: [&str; 3] = ["performance.max", "performance.balanced", "performance.powersaver"];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnumOptions {
    Music,
    Sound,
    InvertMouse,
    Sensitivity,
    Fov,
    Gamma,
    RenderDistance,
    ViewBobbing,
    Anaglyph,
    AdvancedOpengl,
    FramerateLimit,
    Difficulty,
    Graphics,
    AmbientOcclusion,
    GuiScale,
    RenderClouds,
    Particles,
}

impl EnumOptions {
    pub const ALL: [EnumOptions; 17] = [
        EnumOptions::Music,
        EnumOptions::Sound,
        EnumOptions::InvertMouse,
        EnumOptions::Sensitivity,
        EnumOptions::Fov,
        EnumOptions::Gamma,
        EnumOptions::RenderDistance,
        EnumOptions::ViewBobbing,
        EnumOptions::Anaglyph,
        EnumOptions::AdvancedOpengl,
        EnumOptions::FramerateLimit,
        EnumOptions::Difficulty,
        EnumOptions::Graphics,
        EnumOptions::AmbientOcclusion,
        EnumOptions::GuiScale,
        EnumOptions::RenderClouds,
        EnumOptions::Particles,
    ];

    pub fn ordinal(self) -> i32 {
        Self::ALL.iter().position(|&o| o == self).unwrap() as i32
    }

    pub fn from_ordinal(i: i32) -> Option<EnumOptions> {
        Self::ALL.get(i as usize).copied()
    }

    pub fn key(self) -> &'static str {
        match self {
            EnumOptions::Music => "options.music",
            EnumOptions::Sound => "options.sound",
            EnumOptions::InvertMouse => "options.invertMouse",
            EnumOptions::Sensitivity => "options.sensitivity",
            EnumOptions::Fov => "options.fov",
            EnumOptions::Gamma => "options.gamma",
            EnumOptions::RenderDistance => "options.renderDistance",
            EnumOptions::ViewBobbing => "options.viewBobbing",
            EnumOptions::Anaglyph => "options.anaglyph",
            EnumOptions::AdvancedOpengl => "options.advancedOpengl",
            EnumOptions::FramerateLimit => "options.framerateLimit",
            EnumOptions::Difficulty => "options.difficulty",
            EnumOptions::Graphics => "options.graphics",
            EnumOptions::AmbientOcclusion => "options.ao",
            EnumOptions::GuiScale => "options.guiScale",
            EnumOptions::RenderClouds => "options.renderClouds",
            EnumOptions::Particles => "options.particles",
        }
    }

    pub fn is_float(self) -> bool {
        matches!(self, EnumOptions::Music | EnumOptions::Sound | EnumOptions::Sensitivity | EnumOptions::Fov | EnumOptions::Gamma)
    }

    pub fn is_bool(self) -> bool {
        matches!(
            self,
            EnumOptions::InvertMouse
                | EnumOptions::ViewBobbing
                | EnumOptions::Anaglyph
                | EnumOptions::AdvancedOpengl
                | EnumOptions::AmbientOcclusion
                | EnumOptions::RenderClouds
        )
    }
}

/// `KeyBinding`: description key and LWJGL key code.
#[derive(Clone, Debug)]
pub struct KeyBinding {
    pub description: &'static str,
    state: std::sync::Arc<std::sync::Mutex<KeyState>>,
}

#[derive(Debug)]
struct KeyState { code: i32, pressed: bool, press_count: i32 }
#[derive(Default)]
struct KeyRegistry {
    bindings: Vec<std::sync::Arc<std::sync::Mutex<KeyState>>>,
    by_code: std::collections::HashMap<i32, std::sync::Arc<std::sync::Mutex<KeyState>>>,
}
fn key_registry() -> &'static std::sync::Mutex<KeyRegistry> {
    static REGISTRY: std::sync::OnceLock<std::sync::Mutex<KeyRegistry>> = std::sync::OnceLock::new();
    REGISTRY.get_or_init(Default::default)
}

#[derive(Clone, Debug)]
pub struct GameSettings {
    pub music_volume: f32,
    pub sound_volume: f32,
    pub mouse_sensitivity: f32,
    pub fov:f32,
    pub gamma:f32,
    pub invert_mouse: bool,
    pub render_distance: i32,
    pub view_bobbing: bool,
    pub anaglyph: bool,
    pub advanced_opengl: bool,
    pub limit_framerate: i32,
    pub particles: i32,
    pub clouds: bool,
    pub fancy_graphics: bool,
    pub ambient_occlusion: bool,
    pub skin: String,
    pub key_bindings: Vec<KeyBinding>,
    pub difficulty: i32,
    pub hide_gui: bool,
    pub third_person_view: i32,
    pub show_debug_info: bool,
    pub last_server: String,
    pub smooth_camera: bool,
    /// Minecraft's transient wheel zoom fields (not persisted in options.txt).
    pub smooth_camera_zoom: bool,
    pub smooth_camera_zoom_amount: f32,
    pub gui_scale: i32,
    file: Option<PathBuf>,
}

impl Default for GameSettings {
    fn default() -> Self {
        // Field construction order precedes the differently ordered Controls array.
        let bindings: Vec<_> = [("key.forward",17),("key.left",30),("key.back",31),("key.right",32),
            ("key.jump",57),("key.inventory",18),("key.drop",16),("key.chat",20),("key.sneak",42),
            ("key.attack",-100),("key.use",-99),("key.playerlist",15),("key.pickItem",-98)]
            .into_iter().map(|(d,c)|KeyBinding::new(d,c)).collect();
        GameSettings {
            music_volume: 1.0,
            sound_volume: 1.0,
            mouse_sensitivity: 0.5,
            fov:0.0,gamma:0.0,
            invert_mouse: false,
            render_distance: 0,
            view_bobbing: true,
            anaglyph: false,
            advanced_opengl: false,
            limit_framerate: 1,
            particles: 0,
            clouds: true,
            fancy_graphics: true,
            ambient_occlusion: true,
            skin: "Default".into(),
            key_bindings: [9,10,0,1,2,3,4,8,6,5,7,11,12].map(|i|bindings[i].clone()).to_vec(),
            difficulty: 2,
            hide_gui: false,
            third_person_view: 0,
            show_debug_info: false,
            last_server: String::new(),
            smooth_camera: false,
            smooth_camera_zoom: false,
            smooth_camera_zoom_amount: 1.0,
            gui_scale: 0,
            file: None,
        }
    }
}

/// Java's `Float.toString` for the values options take (shortest round-trip, always a dot).
fn java_float(f: f32) -> String {
    format!("{f:?}")
}

impl GameSettings {
    pub fn should_render_clouds(&self) -> bool {
        self.render_distance < 2 && self.clouds
    }
    pub fn input_event(&mut self,code:i32,pressed:bool) {
        self.key_state(code,pressed);
        if pressed {self.key_tick(code);}
    }
    pub fn binding_pressed(&self,name:&str)->bool {
        self.key_bindings.iter().any(|k|k.description==name&&k.pressed())
    }
    pub fn take_binding_presses(&mut self,name:&str)->i32 {
        let Some(k)=self.key_bindings.iter_mut().find(|k|k.description==name) else {return 0;};
        k.take_all_presses()
    }
    /// `new GameSettings(mc, dir)`: defaults, then `options.txt` if present.
    pub fn load(dir: &std::path::Path) -> GameSettings {
        let mut s = GameSettings { file: Some(dir.join("options.txt")), ..Default::default() };
        let Ok(text) = std::fs::read_to_string(dir.join("options.txt")) else { return s };
        for line in text.lines() {
            let p: Vec<&str> = super::font::java_split(line, ':');
            if !s.apply_line(&p) {
                eprintln!("Skipping bad option: {line}");
            }
        }
        KeyBinding::rebuild_registry();
        s
    }

    fn apply_line(&mut self, p: &[&str]) -> bool {
        let v = || p.get(1).copied().ok_or(());
        let float = |s: &str| -> Result<f32, ()> {
            match s {
                "true" => Ok(1.0),
                "false" => Ok(0.0),
                _ => s.trim().parse().map_err(|_| ()),
            }
        };
        let int = |s: &str| -> Result<i32, ()> { s.parse().map_err(|_| ()) };
        let r: Result<(), ()> = (|| {
            match p[0] {
                "music" => self.music_volume = float(v()?)?,
                "sound" => self.sound_volume = float(v()?)?,
                "mouseSensitivity" => self.mouse_sensitivity = float(v()?)?,
                "fov"=>self.fov=float(v()?)?,"gamma"=>self.gamma=float(v()?)?,
                "invertYMouse" => self.invert_mouse = v()? == "true",
                "viewDistance" => self.render_distance = int(v()?)?,
                "guiScale" => self.gui_scale = int(v()?)?,
                "bobView" => self.view_bobbing = v()? == "true",
                "anaglyph3d" => self.anaglyph = v()? == "true",
                "advancedOpengl" => self.advanced_opengl = v()? == "true",
                "fpsLimit" => self.limit_framerate = int(v()?)?,
                "particles" => self.particles = int(v()?)?,
                "difficulty" => self.difficulty = int(v()?)?,
                "fancyGraphics" => self.fancy_graphics = v()? == "true",
                "ao" => self.ambient_occlusion = v()? == "true",
                "clouds" => self.clouds = v()? == "true",
                "skin" => self.skin = v()?.to_string(),
                "lastServer" if p.len() >= 2 => self.last_server = p[1].to_string(),
                _ => {}
            }
            for kb in &mut self.key_bindings {
                if p[0] == format!("key_{}", kb.description) {
                    kb.set_code(int(v()?)?);
                }
            }
            Ok(())
        })();
        r.is_ok()
    }

    pub fn save(&self) {
        let Some(path) = &self.file else { return };
        let mut o = String::new();
        let mut line = |k: &str, v: String| {
            o.push_str(k);
            o.push(':');
            o.push_str(&v);
            o.push_str("\r\n");
        };
        line("music", java_float(self.music_volume));
        line("sound", java_float(self.sound_volume));
        line("invertYMouse", self.invert_mouse.to_string());
        line("mouseSensitivity", java_float(self.mouse_sensitivity));
        line("fov",java_float(self.fov));line("gamma",java_float(self.gamma));
        line("viewDistance", self.render_distance.to_string());
        line("guiScale", self.gui_scale.to_string());
        line("particles", self.particles.to_string());
        line("bobView", self.view_bobbing.to_string());
        line("anaglyph3d", self.anaglyph.to_string());
        line("advancedOpengl", self.advanced_opengl.to_string());
        line("fpsLimit", self.limit_framerate.to_string());
        line("difficulty", self.difficulty.to_string());
        line("fancyGraphics", self.fancy_graphics.to_string());
        line("ao", self.ambient_occlusion.to_string());
        line("clouds", self.clouds.to_string());
        line("skin", self.skin.clone());
        line("lastServer", self.last_server.clone());
        for kb in &self.key_bindings {
            line(&format!("key_{}", kb.description), kb.code().to_string());
        }
        if let Err(e) = std::fs::write(path, o) {
            eprintln!("Failed to save options: {e}");
        }
    }

    pub fn set_option_float_value(&mut self, o: EnumOptions, v: f32) {
        match o {
            EnumOptions::Music => self.music_volume = v,
            EnumOptions::Sound => self.sound_volume = v,
            EnumOptions::Sensitivity => self.mouse_sensitivity = v,
            EnumOptions::Fov=>self.fov=v,EnumOptions::Gamma=>self.gamma=v,
            _ => {}
        }
    }

    /// `setOptionValue`; the caller reloads renderers etc. on `SettingsChanged`.
    pub fn set_option_value(&mut self, o: EnumOptions, d: i32) {
        match o {
            EnumOptions::InvertMouse => self.invert_mouse = !self.invert_mouse,
            EnumOptions::RenderDistance => self.render_distance = (self.render_distance + d) & 3,
            EnumOptions::GuiScale => self.gui_scale = (self.gui_scale + d) & 3,
            EnumOptions::ViewBobbing => self.view_bobbing = !self.view_bobbing,
            EnumOptions::AdvancedOpengl => self.advanced_opengl = !self.advanced_opengl,
            EnumOptions::Anaglyph => self.anaglyph = !self.anaglyph,
            EnumOptions::FramerateLimit => self.limit_framerate = (self.limit_framerate + d + 3) % 3,
            EnumOptions::Difficulty => self.difficulty = (self.difficulty + d) & 3,
            EnumOptions::Graphics => self.fancy_graphics = !self.fancy_graphics,
            EnumOptions::AmbientOcclusion => self.ambient_occlusion = !self.ambient_occlusion,
            EnumOptions::RenderClouds => self.clouds = !self.clouds,
            EnumOptions::Particles => self.particles = self.particles.wrapping_add(d) % 3,
            _ => {}
        }
        self.save();
    }

    pub fn get_option_float_value(&self, o: EnumOptions) -> f32 {
        match o {
            EnumOptions::Music => self.music_volume,
            EnumOptions::Sound => self.sound_volume,
            EnumOptions::Sensitivity => self.mouse_sensitivity,
            EnumOptions::Fov=>self.fov,EnumOptions::Gamma=>self.gamma,
            _ => 0.0,
        }
    }

    pub fn get_option_ordinal_value(&self, o: EnumOptions) -> bool {
        match o {
            EnumOptions::InvertMouse => self.invert_mouse,
            EnumOptions::ViewBobbing => self.view_bobbing,
            EnumOptions::Anaglyph => self.anaglyph,
            EnumOptions::AdvancedOpengl => self.advanced_opengl,
            EnumOptions::AmbientOcclusion => self.ambient_occlusion,
            EnumOptions::RenderClouds => self.clouds,
            _ => false,
        }
    }

    /// `getKeyBinding(EnumOptions)`: the button label.
    pub fn get_key_binding(&self, lang: &Lang, o: EnumOptions) -> String {
        let t = |k: &str| lang.translate_key(k);
        let head = format!("{}: ", t(o.key()));
        if o.is_float() {
            let v = self.get_option_float_value(o);
            if o == EnumOptions::Sensitivity {
                if v == 0.0 {
                    head + &t("options.sensitivity.min")
                } else if v == 1.0 {
                    head + &t("options.sensitivity.max")
                } else {
                    format!("{head}{}%", (v * 200.0) as i32)
                }
            } else if o==EnumOptions::Fov {
                if v==0.0 {head+&t("options.fov.min")}else if v==1.0 {head+&t("options.fov.max")}else {format!("{head}{}",(70.0+v*40.0) as i32)}
            } else if o==EnumOptions::Gamma {
                if v==0.0 {head+&t("options.gamma.min")}else if v==1.0 {head+&t("options.gamma.max")}else {format!("{head}+{}%",(v*100.0) as i32)}
            } else if v == 0.0 {
                head + &t("options.off")
            } else {
                format!("{head}{}%", (v * 100.0) as i32)
            }
        } else if o.is_bool() {
            head + &t(if self.get_option_ordinal_value(o) { "options.on" } else { "options.off" })
        } else {
            match o {
                EnumOptions::RenderDistance => head + &t(RENDER_DISTANCES[self.render_distance as usize & 3]),
                EnumOptions::Difficulty => head + &t(DIFFICULTIES[self.difficulty as usize & 3]),
                EnumOptions::GuiScale => head + &t(GUISCALES[self.gui_scale as usize & 3]),
                EnumOptions::FramerateLimit => head + &t(LIMIT_FRAMERATES[self.limit_framerate.clamp(0, 2) as usize]),
                EnumOptions::Particles => head + &t(["options.particles.all", "options.particles.decreased", "options.particles.minimal"][self.particles as usize]),
                EnumOptions::Graphics => {
                    head + &t(if self.fancy_graphics { "options.graphics.fancy" } else { "options.graphics.fast" })
                }
                _ => head,
            }
        }
    }

    pub fn key_binding_description(&self, lang: &Lang, i: usize) -> String {
        lang.translate_key(self.key_bindings[i].description)
    }

    pub fn set_key_binding(&mut self, i: usize, code: i32) {
        self.key_bindings[i].set_code(code);
        self.save();
    }
}

impl KeyBinding {
    pub fn new(description: &'static str, code: i32) -> Self {
        let state = std::sync::Arc::new(std::sync::Mutex::new(KeyState { code, pressed: false, press_count: 0 }));
        let mut registry=key_registry().lock().unwrap();
        registry.bindings.push(state.clone());registry.by_code.insert(code,state.clone());
        Self { description,state }
    }
    pub fn code(&self)->i32 {self.state.lock().unwrap().code}
    pub fn set_code(&self,code:i32) {self.state.lock().unwrap().code=code;}
    pub fn pressed(&self)->bool {self.state.lock().unwrap().pressed}
    pub fn press_count(&self)->i32 {self.state.lock().unwrap().press_count}
    pub fn take_press(&self)->bool {let mut s=self.state.lock().unwrap();if s.press_count==0 {false}else {s.press_count=s.press_count.wrapping_sub(1);true}}
    pub fn take_all_presses(&self)->i32 {let mut s=self.state.lock().unwrap();let n=s.press_count;s.press_count=0;n}
    /// Rebinding alone leaves the old hash lookup until this explicit vanilla callback.
    pub fn rebuild_registry() {
        let mut registry=key_registry().lock().unwrap();registry.by_code.clear();
        for state in registry.bindings.clone() {let code=state.lock().unwrap().code;registry.by_code.insert(code,state);}
    }
    pub fn tick(code:i32) {let registry=key_registry().lock().unwrap();if let Some(state)=registry.by_code.get(&code){let mut s=state.lock().unwrap();s.press_count=s.press_count.wrapping_add(1);}}
    pub fn set_state(code:i32,pressed:bool) {let registry=key_registry().lock().unwrap();if let Some(state)=registry.by_code.get(&code){state.lock().unwrap().pressed=pressed;}}
    pub fn release_all() {let registry=key_registry().lock().unwrap();for state in &registry.bindings {let mut s=state.lock().unwrap();s.pressed=false;s.press_count=0;}}
}
impl GameSettings {
    pub fn key_tick(&mut self,code:i32){KeyBinding::tick(code);}
    pub fn key_state(&mut self,code:i32,pressed:bool){KeyBinding::set_state(code,pressed);}
    pub fn release_keys(&mut self){KeyBinding::release_all();}
}

/// Independent original-bytecode fixture; runs in a fresh client process.
pub fn check_key_registry(path:&str)->bool {
    use std::fmt::Write;
    let mut keys=Vec::new();
    for _ in 0..4 {keys.extend(GameSettings::default().key_bindings);}
    for code in [0,-100,-99,-98,17,17,i32::MIN,i32::MAX] {keys.push(KeyBinding::new("custom",code));}
    let mut random=worldgen::JavaRandom::new(4181);let mut out=String::new();
    for _ in 0..20000 {
        let op=random.next_int(7);let index=random.next_int(keys.len()as i32)as usize;let code=random.next_int(220)-105;let pressed=random.next_bool();
        match op {
            0=>keys[index].set_code(code),1=>KeyBinding::tick(code),2=>KeyBinding::set_state(code,pressed),
            3=>{writeln!(out,"{}",keys[index].take_press()).unwrap();},4=>KeyBinding::release_all(),5=>KeyBinding::rebuild_registry(),
            _=>{KeyBinding::tick(keys[index].code());KeyBinding::set_state(keys[index].code(),pressed);}
        }
        for k in &keys {write!(out,"{},{},{};",k.code(),k.pressed(),k.press_count()).unwrap();}out.push('\n');
    }
    keys[0].state.lock().unwrap().press_count=i32::MAX;keys[0].set_code(12345);KeyBinding::rebuild_registry();KeyBinding::tick(12345);
    writeln!(out,"{}",keys[0].press_count()).unwrap();writeln!(out,"{}",keys[0].take_press()).unwrap();writeln!(out,"{}",keys[0].press_count()).unwrap();
    let expected=std::fs::read_to_string(path).unwrap().replace("\r\n","\n");
    if expected==out {println!("GLOBAL KEY REGISTRY EXACT: 20000 events, 4 settings, 8 standalone bindings, overflow");true}
    else {std::fs::write("target/keyregistry181-rust.txt",out).unwrap();eprintln!("GLOBAL KEY REGISTRY DIFFERS");false}
}
