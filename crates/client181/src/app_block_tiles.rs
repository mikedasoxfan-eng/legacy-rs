//! Audio output from the piston/note tile snapshot, kept out of the frame loop.
use super::play_world_sound;
use worldgen::world::notes::BlockEvent;
#[allow(dead_code)]
pub(super) fn play_events(
    sound: &mut Option<crate::sound::SoundManager>,
    view: glam::DVec3,
    events: Vec<BlockEvent>,
) {
    for event in events {
        let (position, name, volume, pitch) = match event {
            BlockEvent::Note {
                position,
                instrument,
                pitch,
                ..
            } => (
                position,
                [
                    "note.harp",
                    "note.bd",
                    "note.snare",
                    "note.hat",
                    "note.bassattack",
                ][instrument as usize],
                3.0,
                pitch,
            ),
            BlockEvent::PistonSound {
                position,
                extending,
                pitch,
            } => (
                position,
                if extending {
                    "tile.piston.out"
                } else {
                    "tile.piston.in"
                },
                0.5,
                pitch,
            ),
        };
        if let Some(sound) = sound {
            let (x, y, z) = position;
            play_world_sound(
                sound,
                view,
                name,
                [x as f64 + 0.5, y as f64 + 0.5, z as f64 + 0.5],
                volume,
                pitch,
            );
        }
    }
}
