//! Compare ambient meshes to recorded calls from Mojang's renderer bytecode.
use super::weather::*;
use worldgen::Biome;

pub fn check(a: &[String]) {
    let seed: i64 = a[2].parse().unwrap();
    let cam = [a[3].parse().unwrap(), a[4].parse().unwrap(), a[5].parse().unwrap()];
    let tick = a[6].parse().unwrap();
    let partial = a[7].parse().unwrap();
    let rain = a[8].parse().unwrap();
    let fancy = a.get(12).map(String::as_str) != Some("fast");
    let data = std::fs::read(&a[9]).expect("weather oracle dump");
    let mut d = Reader { data: &data, offset: 0 };
    assert_eq!(d.bytes(8), b"B173WTH1");
    let initial_counter = d.i32();
    let mut s = Snapshot::default();
    for _ in 0..441 {
        let (x, z, top) = (d.i32(), d.i32(), d.i32());
        let kind = d.bytes(1)[0];
        let biome = match kind {
            0 => Biome::Taiga,
            1 => Biome::Plains,
            _ => Biome::Desert,
        };
        let light = std::array::from_fn(|_| d.i32() as u32);
        let particle_rain = d.bytes(1)[0] != 0;
        let blocks = d.bytes(128).try_into().unwrap();
        let meta = d.bytes(128).try_into().unwrap();
        let brightness = std::array::from_fn(|_| f32::from_bits(d.i32() as u32));
        s.columns.push(Column { x, z, top, biome, rain: kind == 1, snow: kind == 0, packed_light: light.map(|l: u32| l as i32), particle_rain, blocks, meta, brightness });
    }
    let mut rng = ClientRandom::new(0);
    let rain_snow = super::weather::precipitation181(&s, cam, tick, partial, rain, fancy, &mut rng);
    let mut counter = initial_counter;
    let (splashes, sound) = add_rain_particles(&s, cam, tick, rain, fancy, &mut rng, &mut counter);
    assert_eq!(splashes.len(), d.i32() as usize, "splash count");
    for (i, p) in splashes.iter().enumerate() {
        assert_eq!(p.smoke, d.bytes(1)[0] != 0, "particle kind {i}");
        for c in p.pos {
            assert_eq!(c.to_bits(), d.i64() as u64, "particle position {i}");
        }
    }
    assert_eq!(rng.random.next_long(), d.i64(), "renderer RNG after particles");
    assert_eq!(counter, d.i32(), "rainSoundCounter");
    assert_eq!(sound.is_some(), d.i32() != 0, "rain sound triggered");
    if let Some(sound) = sound {
        for p in sound.pos {
            assert_eq!(p.to_bits(), d.i64() as u64, "rain sound position");
        }
        assert_eq!(sound.volume.to_bits(), d.i32() as u32, "rain sound volume");
        assert_eq!(sound.pitch.to_bits(), d.i32() as u32, "rain sound pitch");
    }
    let bolts = lightning(Lightning { pos: [0.25, 10.5, -2.75], bolt_vertex: seed }, [0.0; 3]);
    let vertices: Vec<(i32, WeatherVertex)> = rain_snow
        .iter()
        .map(|&(tex, v, _)| (tex as i32, v))
        .chain(bolts.into_iter().map(|v| (3, v)))
        .collect();
    let lights: Vec<u32> = rain_snow.iter().map(|&(_, _, l)| l).chain(std::iter::repeat(u32::MAX).take(vertices.len() - rain_snow.len())).collect();
    assert_eq!(vertices.len(), d.i32() as usize, "weather vertex count");
    for (i, (texture, v)) in vertices.iter().enumerate() {
        let fields = [
            *texture as f32,
            v.pos[0],
            v.pos[1],
            v.pos[2],
            v.uv[0],
            v.uv[1],
            v.color[0],
            v.color[1],
            v.color[2],
            v.color[3],
        ];
        for (j, f) in fields.into_iter().enumerate() {
            // Float.floatToIntBits canonicalises NaN (the camera's own column faces 0/0).
            let bits = if f.is_nan() { 0x7fc0_0000 } else { f.to_bits() };
            assert_eq!(bits, d.i32() as u32, "vertex {i} field {j} ({f})");
        }
    }
    for (i, l) in lights.iter().enumerate() {
        assert_eq!(*l, d.i32() as u32, "vertex {i} lightmap brightness");
    }
    let mut gaussian = ClientRandom::new(seed);
    for i in 0..256 {
        assert_eq!(gaussian.gaussian().to_bits(), d.i64() as u64, "StrictMath Gaussian {i}");
    }
    gaussian.set_seed(seed);
    for _ in 0..3 {
        assert_eq!(gaussian.gaussian().to_bits(), d.i64() as u64, "Gaussian reseed");
    }
    gaussian.set_seed(seed);
    assert_eq!(gaussian.gaussian().to_bits(), d.i64() as u64, "Gaussian clears cached sample");
    assert_eq!(d.offset, data.len(), "unconsumed oracle data");
    println!(
        "weather exact: {} vertices, {} particle spawns, renderer RNG and sound counter",
        vertices.len(),
        splashes.len()
    );
}

struct Reader<'a> {
    data: &'a [u8],
    offset: usize,
}
impl<'a> Reader<'a> {
    fn bytes(&mut self, n: usize) -> &'a [u8] {
        let o = self.offset;
        self.offset += n;
        &self.data[o..o + n]
    }
    fn i32(&mut self) -> i32 {
        i32::from_le_bytes(self.bytes(4).try_into().unwrap())
    }
    fn i64(&mut self) -> i64 {
        i64::from_le_bytes(self.bytes(8).try_into().unwrap())
    }
}
