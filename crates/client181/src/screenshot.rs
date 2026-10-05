//! ScreenShotHelper: local date, unique _2/_3 suffixes, opaque PNG and chat feedback.
use crate::render::gpu::Image;
use std::path::Path;
pub fn save(dir: &Path, image: &Image) -> String {
    save_at(dir, image, &chrono::Local::now().format("%Y-%m-%d_%H.%M.%S").to_string())
}
fn save_at(dir: &Path, image: &Image, stamp: &str) -> String {
    save_named(dir, image, stamp, None)
}
/// ScreenShotHelper.func_35879_a (b1.8.1): with a name the file is written (or overwritten)
/// as given; without one the date stamp gets the first free _2/_3 suffix.
pub fn save_named(dir: &Path, image: &Image, stamp: &str, name: Option<&str>) -> String {
    let result = (|| -> std::io::Result<String> {
        let folder = dir.join("screenshots");
        if !folder.exists() {
            std::fs::create_dir(&folder)?;
        }
        let mut n = 1;
        let path = if let Some(name) = name {
            folder.join(name)
        } else {
            loop {
                let name = if n == 1 { format!("{stamp}.png") } else { format!("{stamp}_{n}.png") };
                let path = folder.join(name);
                if !path.exists() {
                    break path;
                }
                n += 1;
            }
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
/// `--screenshotcheck <dir> <oracle.bin>`: the same sequence as Mode_screenshot181, with the
/// headless framebuffer pattern, compared message by message and pixel by pixel.
pub fn check(dir: &Path, path: &str) -> bool {
    let data = std::fs::read(path).expect("screenshot oracle dump");
    let mut at = 0;
    fn int(data: &[u8], at: &mut usize) -> i32 { let v = i32::from_le_bytes(data[*at..*at + 4].try_into().unwrap()); *at += 4; v }
    let _ = std::fs::remove_dir_all(dir);
    std::fs::create_dir_all(dir).unwrap();
    let names = [None, None, Some("named.png"), Some("named.png"), None];
    let dims = [(6u32, 6u32), (5, 3), (2, 7), (4, 4), (3, 2)];
    let (mut checks, mut bad) = (0, 0);
    for (name, (w, h)) in names.into_iter().zip(dims) {
        // glReadPixels rows start at the bottom; the image is stored top-down.
        let mut rgba = vec![255u8; (w * h * 4) as usize];
        for y in 0..h {
            for x in 0..w {
                for c in 0..3 {
                    let i = (((h - 1 - y) * w + x) * 3 + c) as i32;
                    rgba[((y * w + x) * 4 + c) as usize] = (i.wrapping_mul(37) + 11) as u8;
                }
            }
        }
        let message = save_named(dir, &Image { width: w, height: h, rgba }, "oracle", name);
        let n = int(&data, &mut at) as usize;
        let expected = String::from_utf8(data[at..at + n].to_vec()).unwrap();
        at += n;
        checks += 1;
        if message != expected {
            eprintln!("message: java {expected:?}, rust {message:?}");
            bad += 1;
        }
        let file = message.trim_start_matches("Saved screenshot as ");
        let png = crate::assets::decode_png_pub(&std::fs::read(dir.join("screenshots").join(file)).unwrap());
        let (jw, jh) = (int(&data, &mut at) as u32, int(&data, &mut at) as u32);
        checks += 1;
        if (jw, jh) != (png.width, png.height) {
            eprintln!("{file}: java {jw}x{jh}, rust {}x{}", png.width, png.height);
            bad += 1;
        }
        for p in 0..(jw * jh) as usize {
            let j = int(&data, &mut at) as u32;
            let r = u32::from_be_bytes([255, png.rgba[p * 4], png.rgba[p * 4 + 1], png.rgba[p * 4 + 2]]);
            checks += 1;
            if j != r {
                bad += 1;
            }
        }
    }
    assert_eq!(at, data.len());
    println!("screenshots: {checks} values, {bad} differences");
    bad == 0
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
