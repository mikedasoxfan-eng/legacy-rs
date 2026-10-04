//! Loads textures from the player's own copy of the b1.7.3 client jar.

use crate::render::gpu::Image;
use std::io::Read;
use std::path::PathBuf;

pub struct Assets {
    pub terrain: Image,
    pub sun: Image,
    pub moon: Image,
    pub clouds: Image,
    pub rain: Image,
    pub snow: Image,
    pub particles: Image,
    pub grass_color: Vec<u32>,
    pub foliage_color: Vec<u32>,
    pub mobs: std::collections::HashMap<String, Image>,
}

/// `%APPDATA%\.minecraft\versions\b1.7.3\b1.7.3.jar`, or `BETA_JAR` if set.
pub fn jar_path() -> PathBuf {
    if let Ok(p) = std::env::var("BETA_JAR") {
        return PathBuf::from(p);
    }
    let appdata = std::env::var("APPDATA").unwrap_or_default();
    PathBuf::from(appdata).join(".minecraft").join("versions").join("b1.7.3").join("b1.7.3.jar")
}

pub fn decode_png_pub(bytes: &[u8]) -> Image {
    decode_png(bytes)
}

fn decode_png(bytes: &[u8]) -> Image {
    let mut dec = png::Decoder::new(std::io::Cursor::new(bytes));
    dec.set_transformations(png::Transformations::EXPAND | png::Transformations::STRIP_16);
    let mut r = dec.read_info().expect("png header");
    let mut buf = vec![0; r.output_buffer_size().expect("png size")];
    let info = r.next_frame(&mut buf).expect("png data");
    let (w, h) = (info.width, info.height);
    let px = &buf[..info.buffer_size()];
    let rgba = match info.color_type {
        png::ColorType::Rgba => px.to_vec(),
        png::ColorType::Rgb => px.chunks(3).flat_map(|c| [c[0], c[1], c[2], 255]).collect(),
        png::ColorType::GrayscaleAlpha => px.chunks(2).flat_map(|c| [c[0], c[0], c[0], c[1]]).collect(),
        png::ColorType::Grayscale => px.iter().flat_map(|&g| [g, g, g, 255]).collect(),
        png::ColorType::Indexed => unreachable!("expanded"),
    };
    Image { width: w, height: h, rgba }
}

fn colormap(img: &Image) -> Vec<u32> {
    img.rgba.chunks(4).map(|c| (c[0] as u32) << 16 | (c[1] as u32) << 8 | c[2] as u32).collect()
}

pub fn load() -> Result<Assets, String> {
    let path = jar_path();
    let file = std::fs::File::open(&path)
        .map_err(|e| format!("can't open {} ({e}). Install b1.7.3 in the launcher first.", path.display()))?;
    let mut zip = zip::ZipArchive::new(file).map_err(|e| format!("bad jar: {e}"))?;
    let mut read = |name: &str| -> Result<Image, String> {
        let mut f = zip.by_name(name).map_err(|e| format!("{name} missing from jar: {e}"))?;
        let mut v = Vec::new();
        f.read_to_end(&mut v).map_err(|e| e.to_string())?;
        Ok(decode_png(&v))
    };
    let terrain = read("terrain.png")?;
    let sun = read("terrain/sun.png")?;
    let moon = read("terrain/moon.png")?;
    let clouds = read("environment/clouds.png")?;
    let rain = read("environment/rain.png")?;
    let snow = read("environment/snow.png")?;
    let particles = read("particles.png")?;
    let grass = read("misc/grasscolor.png")?;
    let foliage = read("misc/foliagecolor.png")?;
    let mut mobs = std::collections::HashMap::new();
    for &name in crate::render::entity::living::TEXTURES {
        mobs.insert(name.to_owned(), read(name)?);
    }
    Ok(Assets {
        terrain,
        sun,
        moon,
        clouds,
        rain,
        snow,
        particles,
        grass_color: colormap(&grass),
        foliage_color: colormap(&foliage),
        mobs,
    })
}

/// The b1.7.3 client jar, for reading resources by path.
pub struct Jar {
    zip: zip::ZipArchive<std::fs::File>,
}

impl Jar {
    pub fn open() -> Result<Jar, String> {
        let path = jar_path();
        let file = std::fs::File::open(&path)
            .map_err(|e| format!("can't open {} ({e}). Install b1.7.3 in the launcher first.", path.display()))?;
        Ok(Jar { zip: zip::ZipArchive::new(file).map_err(|e| format!("bad jar: {e}"))? })
    }

    pub fn bytes(&mut self, name: &str) -> Result<Vec<u8>, String> {
        let mut f = self.zip.by_name(name).map_err(|e| format!("{name} missing from jar: {e}"))?;
        let mut v = Vec::new();
        f.read_to_end(&mut v).map_err(|e| e.to_string())?;
        Ok(v)
    }

    /// UTF-8 text (as `InputStreamReader(..., "UTF-8")`).
    pub fn text(&mut self, name: &str) -> Result<String, String> {
        Ok(String::from_utf8_lossy(&self.bytes(name)?).into_owned())
    }

    pub fn image(&mut self, name: &str) -> Result<Image, String> {
        Ok(decode_png(&self.bytes(name)?))
    }
}

pub fn save_png(img: &Image, path: &std::path::Path) -> std::io::Result<()> {
    let f = std::fs::File::create(path)?;
    let mut enc = png::Encoder::new(std::io::BufWriter::new(f), img.width, img.height);
    enc.set_color(png::ColorType::Rgba);
    enc.set_depth(png::BitDepth::Eight);
    let mut w = enc.write_header().map_err(std::io::Error::other)?;
    w.write_image_data(&img.rgba).map_err(std::io::Error::other)?;
    Ok(())
}
