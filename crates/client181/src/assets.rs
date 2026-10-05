//! Loads textures from the player's own copy of the b1.8.1 client jar.

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
    pub footprint: Image,
    pub grass_color: Vec<u32>,
    pub foliage_color: Vec<u32>,
    pub mobs: std::collections::HashMap<String, Image>,
}

/// `%APPDATA%\.minecraft\versions\b1.8.1\b1.8.1.jar`, or `BETA_JAR` if set.
pub fn jar_path() -> PathBuf {
    if let Ok(p) = std::env::var("BETA_JAR") {
        return PathBuf::from(p);
    }
    let appdata = std::env::var("APPDATA").unwrap_or_default();
    PathBuf::from(appdata).join(".minecraft").join("versions").join("b1.8.1").join("b1.8.1.jar")
}

pub fn decode_png_pub(bytes: &[u8]) -> Image {
    decode_png(bytes)
}

fn decode_png(bytes: &[u8]) -> Image {
    try_decode_png(bytes).expect("png data")
}

fn try_decode_png(bytes: &[u8]) -> Result<Image, String> {
    let mut dec = png::Decoder::new(std::io::Cursor::new(bytes));
    dec.set_transformations(png::Transformations::EXPAND | png::Transformations::STRIP_16);
    let mut r = dec.read_info().map_err(|e| e.to_string())?;
    let mut buf = vec![0; r.output_buffer_size().ok_or("png size")?];
    let info = r.next_frame(&mut buf).map_err(|e| e.to_string())?;
    let (w, h) = (info.width, info.height);
    let px = &buf[..info.buffer_size()];
    let rgba = match info.color_type {
        png::ColorType::Rgba => px.to_vec(),
        png::ColorType::Rgb => px.chunks(3).flat_map(|c| [c[0], c[1], c[2], 255]).collect(),
        png::ColorType::GrayscaleAlpha => px.chunks(2).flat_map(|c| [c[0], c[0], c[0], c[1]]).collect(),
        png::ColorType::Grayscale => px.iter().flat_map(|&g| [g, g, g, 255]).collect(),
        png::ColorType::Indexed => unreachable!("expanded"),
    };
    Ok(Image { width: w, height: h, rgba })
}

fn colormap(img: &Image) -> Vec<u32> {
    img.rgba.chunks(4).map(|c| (c[0] as u32) << 16 | (c[1] as u32) << 8 | c[2] as u32).collect()
}

pub fn load() -> Result<Assets, String> {
    load_with(None, false)
}

pub fn load_with(pack: Option<&std::path::Path>, anaglyph: bool) -> Result<Assets, String> {
    let mut jar = Jar::with_pack(pack)?;
    let grass = jar.image("misc/grasscolor.png")?;
    let foliage = jar.image("misc/foliagecolor.png")?;
    let mut read = |name: &str| -> Result<Image, String> {
        let mut image = jar.image(name)?;
        if anaglyph { crate::texture_packs::anaglyph(&mut image.rgba); }
        Ok(image)
    };
    let terrain = read("terrain.png")?;
    let sun = read("terrain/sun.png")?;
    let moon = read("terrain/moon.png")?;
    let clouds = read("environment/clouds.png")?;
    let rain = read("environment/rain.png")?;
    let snow = read("environment/snow.png")?;
    let particles = read("particles.png")?;
    let footprint=read("misc/footprint.png")?;
    let mut mobs = std::collections::HashMap::new();
    for &name in crate::render::entity::living::TEXTURES {
        mobs.insert(name.to_owned(), read(name)?);
    }
    for name in crate::render::entity::player::ARMOR.into_iter().chain([
        "misc/water.png",
        "misc/mapbg.png",
        "misc/mapicons.png",
        "item/sign.png",
        "item/chest.png",
        "item/largechest.png",
        "misc/explosion.png",
        "art/kz.png",
        "particles.png",
    ]) {
        mobs.insert(name.to_owned(), read(name)?);
    }
    let mut font = Jar::open()?.image("font/default.png")?;
    if anaglyph { crate::texture_packs::anaglyph(&mut font.rgba); }
    mobs.insert("font/default.png".into(), font);
    mobs.insert(
        "__map".into(),
        crate::render::map::image(&Default::default(), false),
    );
    Ok(Assets {
        terrain,
        sun,
        moon,
        clouds,
        rain,
        snow,
        particles,
        footprint,
        grass_color: colormap(&grass),
        foliage_color: colormap(&foliage),
        mobs,
    })
}

/// The b1.8.1 client jar, for reading resources by path.
pub struct Jar {
    zip: zip::ZipArchive<std::fs::File>,
    pack: Option<Box<Jar>>,
}

impl Jar {
    pub fn open() -> Result<Jar, String> {
        Self::archive(&jar_path())
    }

    pub fn with_pack(path: Option<&std::path::Path>) -> Result<Self, String> {
        let mut jar = Self::open()?;
        jar.pack = path.and_then(|p| Self::archive(p).ok()).map(Box::new);
        Ok(jar)
    }

    pub fn archive(path: &std::path::Path) -> Result<Self, String> {
        let file = std::fs::File::open(&path).map_err(|e| {
            format!(
                "can't open {} ({e}). Install b1.8.1 in the launcher first.",
                path.display()
            )
        })?;
        Ok(Jar { zip: zip::ZipArchive::new(file).map_err(|e| format!("bad jar: {e}"))?, pack: None })
    }

    pub fn bytes(&mut self, name: &str) -> Result<Vec<u8>, String> {
        let name = name.trim_start_matches('/');
        if let Some(pack) = &mut self.pack {
            if let Ok(bytes) = pack.bytes(name) { return Ok(bytes); }
        }
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
        try_decode_png(&self.bytes(name)?)
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
