//! Compare RenderEngine's actual GL uploads and original TextureCompassFX/TextureWatchFX.
use std::io::Read;

pub fn check(path: &str) -> Result<(), String> {
    let mut f = std::io::BufReader::new(std::fs::File::open(path).map_err(|e| e.to_string())?);
    fn bytes<const N: usize>(f: &mut impl Read) -> Result<[u8; N], String> {
        let mut a = [0; N];
        f.read_exact(&mut a).map_err(|e| e.to_string())?;
        Ok(a)
    }
    if u32::from_be_bytes(bytes(&mut f)?) != 0x54504658 {
        return Err("texture oracle header".into());
    }
    let mut expected = vec![0; 256 * 256 * 4];
    let mut actual = expected.clone();
    for r in 0..256 {
        for g in 0..256 {
            for b in 0..256 {
                let i = (g * 256 + b) * 4;
                actual[i..i + 4].copy_from_slice(&[r as u8, g as u8, b as u8, (r ^ g ^ b) as u8]);
            }
        }
        crate::texture_packs::anaglyph(&mut actual);
        f.read_exact(&mut expected).map_err(|e| e.to_string())?;
        if actual != expected {
            return Err(format!("RenderEngine texture colors differ in red={r}"));
        }
    }
    let spawn =
        (i32::from_be_bytes(bytes(&mut f)?), i32::from_be_bytes(bytes(&mut f)?), i32::from_be_bytes(bytes(&mut f)?));
    let mut jar = crate::assets::Jar::open()?;
    let mut fx = super::item_fx::ItemFx::new(&jar.image("gui/items.png")?, &jar.image("misc/dial.png")?);
    let mut item_random = super::fx::Rng::new(888);
    for i in 0..1000 {
        let x = f64::from_be_bytes(bytes(&mut f)?);
        let z = f64::from_be_bytes(bytes(&mut f)?);
        let yaw = f32::from_be_bytes(bytes(&mut f)?);
        let angle = f32::from_be_bytes(bytes(&mut f)?);
        let ana = bytes::<1>(&mut f)?[0] != 0;
        let mode = i32::from_be_bytes(bytes(&mut f)?);
        let target = match mode {
            0 => super::item_fx::ItemFx::targets([x, 0.0, z], yaw, spawn, angle),
            1 => [
                item_random.random() * std::f64::consts::PI as f32 as f64 * 2.0,
                item_random.random() * std::f64::consts::PI as f32 as f64 * 2.0,
            ],
            _ => [0.0; 2],
        };
        fx.tick(target, ana);
        for (kind, image) in fx.images.iter().enumerate() {
            let expected = bytes::<1024>(&mut f)?;
            if *image != expected {
                return Err(format!("item animation {kind} differs at tick {i}"));
            }
        }
    }
    let mut effects = super::fx::TextureFx::terrain_set();
    let mut random = super::fx::Rng::new(12345);
    for tick in 0..400 {
        for effect in &mut effects {
            effect.tick_anaglyph(&mut random, tick % 3 == 0);
            let expected = bytes::<1024>(&mut f)?;
            if effect.image != expected {
                let pixel = effect.image.iter().zip(expected.iter()).position(|(a, b)| a != b).unwrap();
                return Err(format!(
                    "terrain animation {} differs at tick {tick}, byte {pixel}: rust {} java {}",
                    effect.icon, effect.image[pixel], expected[pixel]
                ));
            }
        }
    }
    super::color_mask::check(&mut f)?;
    if f.read(&mut [0]).map_err(|e| e.to_string())? != 0 {
        return Err("trailing texture oracle data".into());
    }
    println!("TEXTURES EXACT: 16777216 RGB colors and alpha, 1000 compass/watch frames (Overworld/Nether/no world/no player), 400 frames of all seven terrain animations (anaglyph on/off)");
    Ok(())
}
