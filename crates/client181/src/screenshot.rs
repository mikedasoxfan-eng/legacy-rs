//! ScreenShotHelper: local date, unique _2/_3 suffixes, opaque PNG and chat feedback.
use crate::render::gpu::Image;
use std::path::Path;
pub fn save(dir: &Path, image: &Image) -> String {
    save_at(dir, image, &chrono::Local::now().format("%Y-%m-%d_%H.%M.%S").to_string())
}
fn save_at(dir: &Path, image: &Image, stamp: &str) -> String {
    let result = (|| -> std::io::Result<String> {
        let folder = dir.join("screenshots");
        if !folder.exists() {
            std::fs::create_dir(&folder)?;
        }
        let mut n = 1;
        let path = loop {
            let name = if n == 1 { format!("{stamp}.png") } else { format!("{stamp}_{n}.png") };
            let path = folder.join(name);
            if !path.exists() {
                break path;
            }
            n += 1;
        };
        let mut opaque = Image { width: image.width, height: image.height, rgba: image.rgba.clone() };
        for c in opaque.rgba.chunks_exact_mut(4) {
            c[3] = 255;
        }
        crate::assets::save_png(&opaque, &path)?;
        Ok(format!("Saved screenshot as {}", path.file_name().unwrap().to_string_lossy()))
    })();
    result.unwrap_or_else(|e| format!("Failed to save: {e}"))
}
#[cfg(test)]
mod tests {
    #[test]
    fn suffix_and_alpha() {
        let dir = std::env::temp_dir().join(format!("beta-screenshot-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let img = crate::render::gpu::Image { width: 1, height: 1, rgba: vec![3, 5, 7, 0] };
        assert_eq!(super::save_at(&dir, &img, "oracle"), "Saved screenshot as oracle.png");
        assert_eq!(super::save_at(&dir, &img, "oracle"), "Saved screenshot as oracle_2.png");
        let png = std::fs::read(dir.join("screenshots/oracle.png")).unwrap();
        assert_eq!(crate::assets::decode_png_pub(&png).rgba, [3, 5, 7, 255]);
        std::fs::remove_file(dir.join("screenshots/oracle.png")).unwrap();
        std::fs::remove_file(dir.join("screenshots/oracle_2.png")).unwrap();
        std::fs::remove_dir(dir.join("screenshots")).unwrap();
        std::fs::remove_dir(dir).unwrap();
    }
}
