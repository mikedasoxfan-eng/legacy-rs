//! GuiBrewingStand's original float progress and seven-step bubble animation.
pub fn progress(time: i32) -> Option<(i32, i32)> {
    if time <= 0 {
        return None;
    }
    let fill = (28.0f32 * (1.0f32 - time as f32 / 400.0f32)) as i32;
    let bubbles = [29, 24, 20, 16, 11, 6, 0][(time / 2 % 7) as usize];
    Some((fill, bubbles))
}
pub fn rectangles(time: i32, x: i32, y: i32) -> Vec<[i32; 6]> {
    let mut out = vec![[x, y, 0, 0, 176, 166]];
    if let Some((fill, bubbles)) = progress(time) {
        if fill > 0 {
            out.push([x + 97, y + 16, 176, 0, 9, fill]);
        }
        if bubbles > 0 {
            out.push([x + 65, y + 43 - bubbles, 185, 29 - bubbles, 12, bubbles]);
        }
    }
    out
}

#[cfg(test)]
mod original_tests {
    #[test]
    #[ignore = "requires original 1.1 GUI fixture"]
    fn brewing_background_matches_original_11() {
        let path = std::env::var("GUI_BREWING11_FIXTURE").unwrap();
        std::fs::create_dir_all("target/r10gui2").unwrap();
        assert!(crate::gui::check10_round2::check("brewing", &path));
    }
}
