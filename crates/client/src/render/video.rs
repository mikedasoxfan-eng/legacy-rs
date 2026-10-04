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

    /// Java integer division in `EntityRenderer.updateCameraAndRender`.
    pub fn frame_nanos(self) -> Option<u64> {
        match self.limit {
            1 => Some(1_000_000_000 / 120),
            2 => Some(1_000_000_000 / 40),
            _ => None,
        }
    }

    /// The Power saver sleep guard and menu-only negative-delay adjustment.
    pub fn power_sleep_millis(remaining_nanos: i64, world: bool) -> u64 {
        let mut millis = remaining_nanos / 1_000_000;
        if !world && millis < 0 {
            millis += 10;
        }
        if millis > 0 && millis < 500 {
            millis as u64
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn power_saver_retains_java_sleep_boundaries() {
        assert_eq!(VideoSettings::power_sleep_millis(25_999_999, true), 25);
        assert_eq!(VideoSettings::power_sleep_millis(999_999, true), 0);
        assert_eq!(VideoSettings::power_sleep_millis(-1_500_000, true), 0);
        assert_eq!(VideoSettings::power_sleep_millis(-1_500_000, false), 9);
        assert_eq!(VideoSettings::power_sleep_millis(-10_000_000, false), 0);
        assert_eq!(VideoSettings::power_sleep_millis(499_999_999, true), 499);
        assert_eq!(VideoSettings::power_sleep_millis(500_000_000, true), 0);
    }
}
