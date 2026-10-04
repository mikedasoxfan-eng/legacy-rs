//! `SoundManager` / `SoundPool` with the paulscode SoundSystem behaviour the game relies on:
//! named pools built from `.minecraft/resources` (sound, newsound, streaming, music,
//! newmusic), positional sources with linear distance falloff (16 blocks, or 16 x volume
//! when louder than 1), pitch as playback rate, UI sounds placed at the world origin, and
//! the background-music timer. Output goes through a small mixer on `cpal`.

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use worldgen::JavaRandom;

/// Decoded audio, interleaved f32.
pub struct Clip {
    samples: Vec<f32>,
    channels: usize,
    rate: f32,
}

fn decode_ogg(path: &Path) -> Option<Clip> {
    let f = std::fs::File::open(path).ok()?;
    let mut r = lewton::inside_ogg::OggStreamReader::new(std::io::BufReader::new(f)).ok()?;
    let channels = r.ident_hdr.audio_channels as usize;
    let rate = r.ident_hdr.audio_sample_rate as f32;
    let mut samples = Vec::new();
    while let Ok(Some(p)) = r.read_dec_packet_itl() {
        samples.extend(p.iter().map(|&s| s as f32 / 32768.0));
    }
    Some(Clip { samples, channels: channels.max(1), rate })
}

#[derive(Clone)]
struct PoolEntry {
    path: PathBuf,
}

/// `SoundPool`: names like `random.click` -> every file `random/click*.ogg`.
struct SoundPool {
    rand: JavaRandom,
    by_name: HashMap<String, Vec<PoolEntry>>,
    all: Vec<PoolEntry>,
    /// `field_1657_b`: strip trailing digits (off for streaming).
    strip_digits: bool,
}

impl SoundPool {
    fn new(seed: i64, strip_digits: bool) -> SoundPool {
        SoundPool { rand: JavaRandom::new(seed), by_name: HashMap::new(), all: Vec::new(), strip_digits }
    }

    fn add(&mut self, name: &str, path: PathBuf) {
        let Some(dot) = name.find('.') else { return };
        let mut n = name[..dot].to_string();
        if self.strip_digits {
            while n.chars().last().is_some_and(|c| c.is_ascii_digit()) {
                n.pop();
            }
        }
        let n = n.replace('/', ".");
        let e = PoolEntry { path };
        self.by_name.entry(n).or_default().push(e.clone());
        self.all.push(e);
    }

    fn random_for(&mut self, name: &str) -> Option<PoolEntry> {
        let list = self.by_name.get(name)?;
        let i = self.rand.next_int(list.len() as i32) as usize;
        Some(list[i].clone())
    }

    fn random(&mut self) -> Option<PoolEntry> {
        if self.all.is_empty() {
            return None;
        }
        let i = self.rand.next_int(self.all.len() as i32) as usize;
        Some(self.all[i].clone())
    }
}

/// One playing source in the mixer.
struct Voice {
    clip: Arc<Clip>,
    /// Position in source frames.
    pos: f64,
    /// Source frames per output frame (pitch x rate ratio).
    step: f64,
    gain: [f32; 2],
    /// "BgMusic" / "streaming" / sound_N
    name: String,
}

#[derive(Default)]
struct Mixer {
    voices: Vec<Voice>,
    out_rate: f32,
}

impl Mixer {
    fn render(&mut self, out: &mut [f32], channels: usize) {
        out.iter_mut().for_each(|s| *s = 0.0);
        let frames = out.len() / channels;
        for v in &mut self.voices {
            let c = v.clip.channels;
            let n = v.clip.samples.len() / c;
            for f in 0..frames {
                let i = v.pos as usize;
                if i + 1 >= n {
                    v.pos = n as f64;
                    break;
                }
                let t = (v.pos - i as f64) as f32;
                let sample = |ch: usize| {
                    let a = v.clip.samples[i * c + ch.min(c - 1)];
                    let b = v.clip.samples[(i + 1) * c + ch.min(c - 1)];
                    a + (b - a) * t
                };
                let (l, r) = if c == 1 {
                    let m = sample(0);
                    (m * v.gain[0], m * v.gain[1])
                } else {
                    (sample(0) * v.gain[0], sample(1) * v.gain[1])
                };
                if channels >= 2 {
                    out[f * channels] += l;
                    out[f * channels + 1] += r;
                } else {
                    out[f] += (l + r) * 0.5;
                }
                v.pos += v.step;
            }
        }
        self.voices.retain(|v| (v.pos as usize) + 1 < v.clip.samples.len() / v.clip.channels);
        for s in out.iter_mut() {
            *s = s.clamp(-1.0, 1.0);
        }
    }
}

pub struct SoundManager {
    sounds: SoundPool,
    streaming: SoundPool,
    music: SoundPool,
    counter: i32,
    rand: JavaRandom,
    ticks_before_music: i32,
    mixer: Arc<Mutex<Mixer>>,
    _stream: Option<cpal::Stream>,
    clips: HashMap<PathBuf, Arc<Clip>>,
    listener: ([f32; 3], [f32; 3]),
    pub sound_volume: f32,
    pub music_volume: f32,
}

impl SoundManager {
    /// Open the default output device and index `.minecraft/resources` like
    /// `ThreadDownloadResources.loadResource` + `Minecraft.installResource`.
    pub fn new(resources: &Path, seed: i64) -> SoundManager {
        let mut rand = JavaRandom::new(seed);
        let ticks = rand.next_int(12000);
        let mut m = SoundManager {
            sounds: SoundPool::new(seed ^ 1, true),
            streaming: SoundPool::new(seed ^ 2, false),
            music: SoundPool::new(seed ^ 3, true),
            counter: 0,
            rand,
            ticks_before_music: ticks,
            mixer: Arc::new(Mutex::new(Mixer::default())),
            _stream: None,
            clips: HashMap::new(),
            listener: ([0.0; 3], [0.0, 0.0, -1.0]),
            sound_volume: 1.0,
            music_volume: 1.0,
        };
        m.load_dir(resources, "");
        m._stream = m.open_output();
        m
    }

    fn load_dir(&mut self, dir: &Path, prefix: &str) {
        let Ok(rd) = std::fs::read_dir(dir) else { return };
        let mut entries: Vec<_> = rd.filter_map(|e| e.ok()).collect();
        entries.sort_by_key(|e| e.file_name());
        for e in entries {
            let name = e.file_name().to_string_lossy().into_owned();
            let path = e.path();
            if path.is_dir() {
                self.load_dir(&path, &format!("{prefix}{name}/"));
            } else {
                self.install_resource(&format!("{prefix}{name}"), path);
            }
        }
    }

    /// `Minecraft.installResource`
    fn install_resource(&mut self, name: &str, path: PathBuf) {
        let Some(slash) = name.find('/') else { return };
        let (kind, rest) = (&name[..slash], &name[slash + 1..]);
        match kind.to_ascii_lowercase().as_str() {
            "sound" | "newsound" => self.sounds.add(rest, path),
            "streaming" => self.streaming.add(rest, path),
            "music" | "newmusic" => self.music.add(rest, path),
            _ => {}
        }
    }

    fn open_output(&self) -> Option<cpal::Stream> {
        let host = cpal::default_host();
        let device = host.default_output_device()?;
        let supported = device.default_output_config().ok()?;
        let config: cpal::StreamConfig = supported.clone().into();
        let channels = config.channels as usize;
        self.mixer.lock().unwrap().out_rate = config.sample_rate as f32;
        let mixer = self.mixer.clone();
        let err = |e: cpal::Error| eprintln!("audio: {e}");
        let stream = match supported.sample_format() {
            cpal::SampleFormat::F32 => device
                .build_output_stream::<f32, _, _>(
                    config,
                    move |out: &mut [f32], _| mixer.lock().unwrap().render(out, channels),
                    err,
                    None,
                )
                .ok()?,
            cpal::SampleFormat::I16 => {
                let mut buf = Vec::new();
                device
                    .build_output_stream::<i16, _, _>(
                        config,
                        move |out: &mut [i16], _| {
                            buf.resize(out.len(), 0.0f32);
                            mixer.lock().unwrap().render(&mut buf, channels);
                            for (o, s) in out.iter_mut().zip(&buf) {
                                *o = (s * 32767.0) as i16;
                            }
                        },
                        err,
                        None,
                    )
                    .ok()?
            }
            f => {
                eprintln!("audio: unsupported sample format {f}");
                return None;
            }
        };
        stream.play().ok()?;
        Some(stream)
    }

    fn clip(&mut self, path: &Path) -> Option<Arc<Clip>> {
        if let Some(c) = self.clips.get(path) {
            return Some(c.clone());
        }
        let c = Arc::new(decode_ogg(path)?);
        self.clips.insert(path.to_path_buf(), c.clone());
        Some(c)
    }

    /// `func_338_a`: the listener at the player's interpolated position, facing its yaw.
    pub fn set_listener(&mut self, pos: [f64; 3], yaw: f32) {
        let a = -yaw * (std::f32::consts::PI / 180.0) - std::f32::consts::PI;
        let (c, s) = (worldgen::math::cos(a), worldgen::math::sin(a));
        self.listener = ([pos[0] as f32, pos[1] as f32, pos[2] as f32], [-s, 0.0, -c]);
    }

    /// Stereo gains for a mono source: linear falloff over `fade` blocks (0 = none) and
    /// equal-power panning by direction relative to the listener.
    fn spatial(&self, at: [f32; 3], fade: f32) -> [f32; 2] {
        let (lp, fwd) = self.listener;
        let rel = [at[0] - lp[0], at[1] - lp[1], at[2] - lp[2]];
        let dist = (rel[0] * rel[0] + rel[1] * rel[1] + rel[2] * rel[2]).sqrt();
        let gain = if fade > 0.0 { (1.0 - dist / fade).clamp(0.0, 1.0) } else { 1.0 };
        // Right = forward x up.
        let right = [-fwd[2], 0.0, fwd[0]];
        let pan = if dist > 1e-4 { (rel[0] * right[0] + rel[2] * right[2]) / dist } else { 0.0 };
        let a = (pan.clamp(-1.0, 1.0) + 1.0) * std::f32::consts::FRAC_PI_4;
        [gain * a.cos() * std::f32::consts::SQRT_2, gain * a.sin() * std::f32::consts::SQRT_2]
    }

    fn start(&mut self, name: String, clip: Arc<Clip>, pitch: f32, gain: [f32; 2]) {
        let mut m = self.mixer.lock().unwrap();
        let out_rate = if m.out_rate > 0.0 { m.out_rate } else { 44100.0 };
        let step = pitch.max(0.01) as f64 * clip.rate as f64 / out_rate as f64;
        m.voices.retain(|v| v.name != name);
        m.voices.push(Voice { clip, pos: 0.0, step, gain, name });
    }

    fn playing(&self, name: &str) -> bool {
        self.mixer.lock().unwrap().voices.iter().any(|v| v.name == name)
    }

    fn stop(&self, name: &str) {
        self.mixer.lock().unwrap().voices.retain(|v| v.name != name);
    }

    /// `playSound(name, x, y, z, volume, pitch)`
    pub fn play_sound(&mut self, name: &str, pos: [f32; 3], volume: f32, pitch: f32) {
        if self.sound_volume == 0.0 {
            return;
        }
        let Some(e) = self.sounds.random_for(name) else { return };
        if volume <= 0.0 {
            return;
        }
        self.counter = (self.counter + 1) % 256;
        let id = format!("sound_{}", self.counter);
        let mut fade = 16.0f32;
        if volume > 1.0 {
            fade *= volume;
        }
        let vol = volume.min(1.0) * self.sound_volume;
        let Some(clip) = self.clip(&e.path) else { return };
        let mut g = if clip.channels == 1 { self.spatial(pos, fade) } else { [1.0, 1.0] };
        g = [g[0] * vol, g[1] * vol];
        self.start(id, clip, pitch, g);
    }

    /// `playSoundFX(name, volume, pitch)`: no falloff, but positioned at the world origin.
    pub fn play_sound_fx(&mut self, name: &str, volume: f32, pitch: f32) {
        if self.sound_volume == 0.0 {
            return;
        }
        let Some(e) = self.sounds.random_for(name) else { return };
        self.counter = (self.counter + 1) % 256;
        let id = format!("sound_{}", self.counter);
        let vol = volume.min(1.0) * 0.25 * self.sound_volume;
        let Some(clip) = self.clip(&e.path) else { return };
        let mut g = if clip.channels == 1 { self.spatial([0.0; 3], 0.0) } else { [1.0, 1.0] };
        g = [g[0] * vol, g[1] * vol];
        self.start(id, clip, pitch, g);
    }

    /// `playRandomMusicIfReady` (called every game tick by the player controller).
    pub fn play_random_music_if_ready(&mut self) {
        if self.music_volume == 0.0 || self.playing("BgMusic") || self.playing("streaming") {
            return;
        }
        if self.ticks_before_music > 0 {
            self.ticks_before_music -= 1;
            return;
        }
        let Some(e) = self.music.random() else { return };
        self.ticks_before_music = self.rand.next_int(12000) + 12000;
        if let Some(clip) = self.clip(&e.path) {
            let v = self.music_volume;
            self.start("BgMusic".into(), clip, 1.0, [v, v]);
        }
    }

    /// `onSoundOptionsChanged`
    pub fn options_changed(&mut self, sound: f32, music: f32) {
        self.sound_volume = sound;
        self.music_volume = music;
        if music == 0.0 {
            self.stop("BgMusic");
        } else {
            let mut m = self.mixer.lock().unwrap();
            for v in m.voices.iter_mut().filter(|v| v.name == "BgMusic") {
                v.gain = [music, music];
            }
        }
    }

    /// Stop everything (leaving a world: `playStreaming(null, ...)` stops records).
    pub fn stop_streaming(&mut self) {
        self.stop("streaming");
    }

    pub fn stop_all(&mut self) {
        self.mixer.lock().unwrap().voices.clear();
    }
}

/// `.minecraft/resources` (where the launcher keeps b1.7.3's sounds).
pub fn resources_dir() -> PathBuf {
    let appdata = std::env::var("APPDATA").unwrap_or_default();
    PathBuf::from(appdata).join(".minecraft").join("resources")
}

/// `beta --soundcheck`: index the resources and decode a few sounds, without playing.
pub fn check() -> bool {
    let dir = resources_dir();
    let mut m = SoundManager {
        sounds: SoundPool::new(1, true),
        streaming: SoundPool::new(2, false),
        music: SoundPool::new(3, true),
        counter: 0,
        rand: JavaRandom::new(0),
        ticks_before_music: 0,
        mixer: Arc::new(Mutex::new(Mixer::default())),
        _stream: None,
        clips: HashMap::new(),
        listener: ([0.0; 3], [0.0, 0.0, -1.0]),
        sound_volume: 1.0,
        music_volume: 1.0,
    };
    m.load_dir(&dir, "");
    println!(
        "{}: {} sound names ({} files), {} streaming, {} music",
        dir.display(),
        m.sounds.by_name.len(),
        m.sounds.all.len(),
        m.streaming.all.len(),
        m.music.all.len()
    );
    let mut ok = true;
    for name in ["random.click", "step.grass", "ambient.weather.rain", "ambient.cave.cave", "mob.pig"] {
        let variants = m.sounds.by_name.get(name).map_or(0, |v| v.len());
        let decoded = m.sounds.random_for(name).and_then(|e| m.clip(&e.path));
        match decoded {
            Some(c) => println!("  {name}: {variants} variants, {:.2}s {}ch {}Hz", c.samples.len() as f32 / c.channels as f32 / c.rate, c.channels, c.rate),
            None => {
                println!("  {name}: MISSING");
                ok = false;
            }
        }
    }
    ok
}
