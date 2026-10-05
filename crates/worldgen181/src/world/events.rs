//! Ordered presentation callbacks. Enqueuing never draws randomness.
use crate::{
    entity::{Entity, Vec3D},
    World,
};
#[derive(Clone, Debug, PartialEq)]
pub enum WorldEvent {
    Sound {
        name: Option<String>,
        position: Vec3D,
        volume: f32,
        pitch: f32,
    },
    Particle {
        name: String,
        position: Vec3D,
        velocity: Vec3D,
    },
    Record {
        name: Option<String>,
        position: (i32, i32, i32),
    },
    /// World.func_28106_e / IWorldAccess.func_28136_a.
    Auxiliary {
        code: i32,
        position: (i32, i32, i32),
        data: i32,
        random: crate::JavaRandom,
    },
    /// SoundManager calls bypass RenderGlobal's distance check.
    DirectSound {
        name: String,
        position: Option<Vec3D>,
        volume: f32,
        pitch: f32,
    },
    BlockHit {
        position: (i32, i32, i32),
        side: i32,
        block: u8,
        metadata: i32,
        bounds: [f64; 6],
    },
}
impl World {
    pub fn drain_events(&mut self) -> Vec<WorldEvent> {
        std::mem::take(&mut self.events)
    }
    pub fn play_sound(&mut self, name: &str, position: Vec3D, volume: f32, pitch: f32) {
        self.events.push(WorldEvent::Sound {
            name: Some(name.into()),
            position,
            volume,
            pitch,
        });
    }
    pub fn play_sound_at_entity(&mut self, e: &Entity, name: &str, volume: f32, pitch: f32) {
        self.play_sound(
            name,
            e.pos.add(0.0, -(e.y_offset as f64), 0.0),
            volume,
            pitch,
        );
    }
    pub fn play_null_sound_at_entity(&mut self, e: &Entity, volume: f32, pitch: f32) {
        self.events.push(WorldEvent::Sound {
            name: None,
            position: e.pos.add(0.0, -(e.y_offset as f64), 0.0),
            volume,
            pitch,
        });
    }
    pub fn spawn_particle(&mut self, name: &str, position: Vec3D, velocity: Vec3D) {
        self.events.push(WorldEvent::Particle {
            name: name.into(),
            position,
            velocity,
        });
    }
    pub fn play_record(&mut self, name: Option<&str>, position: (i32, i32, i32)) {
        self.events.push(WorldEvent::Record {
            name: name.map(str::to_owned),
            position,
        });
    }
    pub fn auxiliary_effect(&mut self, code: i32, position: (i32, i32, i32), data: i32) {
        self.events.push(WorldEvent::Auxiliary {
            code,
            position,
            data,
            random: self.rand.clone(),
        });
    }
    pub fn direct_sound(&mut self, name: &str, position: Option<Vec3D>, volume: f32, pitch: f32) {
        self.events.push(WorldEvent::DirectSound {
            name: name.into(),
            position,
            volume,
            pitch,
        });
    }
}
/// Little-endian bits, length-prefixed UTF-8 names. Also used by the bytecode oracle.
pub fn write_events(events: &[WorldEvent], out: &mut Vec<u8>) {
    fn i(o: &mut Vec<u8>, n: i32) {
        o.extend(n.to_le_bytes());
    }
    fn text(o: &mut Vec<u8>, s: &str) {
        i(o, s.len() as i32);
        o.extend(s.as_bytes());
    }
    fn pos(o: &mut Vec<u8>, v: Vec3D) {
        for d in [v.x, v.y, v.z] {
            o.extend(
                (if d.is_nan() {
                    0x7ff8000000000000u64
                } else {
                    d.to_bits()
                })
                .to_le_bytes(),
            );
        }
    }
    fn cell(o: &mut Vec<u8>, p: (i32, i32, i32)) {
        for n in [p.0, p.1, p.2] {
            i(o, n);
        }
    }
    i(out, events.len() as i32);
    for e in events {
        match e {
            WorldEvent::Sound {
                name,
                position,
                volume,
                pitch,
            } => {
                out.push(0);
                if let Some(n) = name {
                    text(out, n);
                } else {
                    i(out, -1);
                }
                pos(out, *position);
                i(out, volume.to_bits() as i32);
                i(out, pitch.to_bits() as i32);
            }
            WorldEvent::Particle {
                name,
                position,
                velocity,
            } => {
                out.push(1);
                text(out, name);
                pos(out, *position);
                pos(out, *velocity);
            }
            WorldEvent::Record { name, position } => {
                out.push(2);
                i(out, if name.is_some() { 1 } else { 0 });
                if let Some(n) = name {
                    text(out, n);
                }
                cell(out, *position);
            }
            WorldEvent::Auxiliary {
                code,
                position,
                data,
                ..
            } => {
                out.push(3);
                i(out, *code);
                cell(out, *position);
                i(out, *data);
            }
            WorldEvent::DirectSound {
                name,
                position,
                volume,
                pitch,
            } => {
                out.push(4);
                text(out, name);
                out.push(position.is_some() as u8);
                if let Some(p) = position {
                    pos(out, *p);
                }
                i(out, volume.to_bits() as i32);
                i(out, pitch.to_bits() as i32);
            }
            WorldEvent::BlockHit {
                position,
                side,
                block,
                metadata,
                ..
            } => {
                out.push(5);
                cell(out, *position);
                i(out, *side);
                i(out, *block as i32);
                i(out, *metadata);
            }
        }
    }
}

/// Block.stepSound (names, volume and pitch), including the glass/sand overrides.
pub fn step_sound(id: u8) -> (&'static str, f32, f32) {
    match id {
        5 | 17 | 47 | 50 | 51 | 53 | 54 | 58 | 63 | 64 | 65 | 68 | 69 | 72 | 75 | 76 | 85 | 86
        | 91 | 93 | 94 | 95 | 96 | 99 | 100 | 103 | 104 | 105 | 107 => ("step.wood", 1.0, 1.0),
        2 | 6 | 18 | 19 | 31 | 32 | 37 | 38 | 39 | 40 | 46 | 59 | 83 | 106 => ("step.grass", 1.0, 1.0),
        3 | 13 | 60 | 82 => ("step.gravel", 1.0, 1.0),
        12 | 88 => ("step.sand", 1.0, 1.0),
        35 | 78 | 80 | 81 | 92 => ("step.cloth", 1.0, 1.0),
        27 | 28 | 41 | 42 | 52 | 57 | 66 | 71 | 101 => ("step.stone", 1.0, 1.5),
        _ => ("step.stone", 1.0, 1.0),
    }
}

/// StepSound.stepSoundDir differs from footsteps for sand and glass.
pub fn break_sound(id: u8) -> (&'static str, f32, f32) {
    let (name, v, p) = step_sound(id);
    (
        match id {
            20 | 79 | 89 | 90 | 102 => "random.glass",
            12 | 88 => "step.gravel",
            _ => name,
        },
        v,
        p,
    )
}
