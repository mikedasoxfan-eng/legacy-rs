//! Renderer-facing snapshot of `GameSettings`. The GUI owns the persisted options.

use crate::gui::settings::GameSettings;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VideoSettings {
    pub distance: i32,
    pub fancy: bool,
    pub ao: bool,
    pub limit: i32,
    pub anaglyph: bool,
    pub advanced_opengl: bool,
}

impl From<&GameSettings> for VideoSettings {
    fn from(s: &GameSettings) -> Self {
        Self {
            distance: s.render_distance.clamp(0, 3),
            fancy: s.fancy_graphics,
            ao: s.ambient_occlusion,
            limit: s.limit_framerate.clamp(0, 2),
            anaglyph: s.anaglyph,
            advanced_opengl: s.advanced_opengl,
        }
    }
}

impl Default for VideoSettings {
    fn default() -> Self {
        Self::from(&GameSettings::default())
    }
}

impl VideoSettings {
    /// `RenderGlobal.loadRenderers`: integer division, including the extra renderer.
    pub fn chunks_wide(self) -> i32 {
        (64i32 << (3 - self.distance)).min(400) / 16 + 1
    }

    /// `EntityRenderer.setupCameraTransform` (projection's far clip is twice this).
    pub fn far(self) -> f32 {
        (256 >> self.distance) as f32
    }

    pub fn sky(self) -> bool {
        self.distance < 2
    }

    pub fn remesh(self, previous: Self) -> bool {
        self.distance != previous.distance || self.fancy != previous.fancy || self.ao != previous.ao
    }

    /// `EntityRenderer.updateRenderer`: brightness at the view entity's position.
    pub fn fog_target(self, brightness: f32) -> f32 {
        let blend = (3 - self.distance) as f32 / 3.0f32;
        brightness * (1.0f32 - blend) + blend
    }
}
