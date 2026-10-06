use worldgen::{genlayer, OverworldGenerator};
fn main() {
    let a: Vec<String> = std::env::args().skip(1).collect();
    if a[0] == "component" {
        compare(
            worldgen::structures::verify::component(a[1].parse().unwrap()),
            std::fs::read(&a[2]).unwrap(),
        );
        return;
    }
    if a[0] == "round2" {
        compare(
            worldgen::terrain_verify::round2(&a[1], a[2].parse().unwrap()),
            std::fs::read(&a[3]).unwrap(),
        );
        return;
    }
    if a[0] == "structures" {
        compare(
            worldgen::structures::verify::dump(&a[1], a[2].parse().unwrap()),
            std::fs::read(&a[3]).unwrap(),
        );
        return;
    }
    if a[0] == "starts" {
        compare(
            worldgen::terrain_verify::starts(a[1].parse().unwrap()),
            std::fs::read(&a[2]).unwrap(),
        );
        return;
    }
    if a[0] == "cache" {
        compare(
            worldgen::terrain_verify::cache(a[1].parse().unwrap()),
            std::fs::read(&a[2]).unwrap(),
        );
        return;
    }
    if a[0] == "utilities" {
        compare(
            worldgen::terrain_verify::utilities(a[1].parse().unwrap()),
            std::fs::read(&a[2]).unwrap(),
        );
        return;
    }
    if a[0] == "biomes" {
        compare(
            worldgen::terrain_verify::biomes(a[1].parse().unwrap()),
            std::fs::read(&a[2]).unwrap(),
        );
        return;
    }
    if a[0] == "spawn" {
        let mut w = worldgen::World::new(a[1].parse().unwrap(), 1);
        w.find_initial_spawn();
        let mut actual = Vec::new();
        for n in [w.spawn.0, w.spawn.1, w.spawn.2] {
            actual.extend_from_slice(&n.to_be_bytes());
        }
        println!("spawn {:?}", w.spawn);
        compare(actual, std::fs::read(&a[2]).unwrap());
        return;
    }
    if a[0] == "feature" {
        let actual =
            worldgen::terrain_verify::feature(&a[1], a[2].parse().unwrap(), a[3].parse().unwrap());
        compare(actual, std::fs::read(&a[4]).unwrap());
        return;
    }
    let seed: i64 = a[1].parse().unwrap();
    let x: i32 = a[2].parse().unwrap();
    let z: i32 = a[3].parse().unwrap();
    let w: i32 = a[4].parse().unwrap();
    let h: i32 = a[5].parse().unwrap();
    let expected = std::fs::read(&a[6]).unwrap();
    let mut actual = Vec::new();
    match a[0].as_str() {
        "layers" => {
            for layer in genlayer::stack(seed) {
                for v in layer.get(x, z, w, h).into_iter().take((w * h) as usize) {
                    actual.extend_from_slice(&v.to_be_bytes());
                }
            }
        }
        "raw" => {
            let mut gen = OverworldGenerator::new(seed);
            for cx in x..x + w {
                for cz in z..z + h {
                    actual.extend_from_slice(&gen.generate(cx, cz).blocks[..]);
                }
            }
        }
        "sky" => {
            let mut gen = worldgen::sky::SkyGenerator::new(seed);
            for cx in x..x + w {
                for cz in z..z + h {
                    actual.extend_from_slice(&gen.generate(cx, cz).blocks[..]);
                }
            }
        }
        _ => panic!("unknown terrain mode"),
    }
    compare(actual, expected);
}
fn compare(actual: Vec<u8>, expected: Vec<u8>) {
    if let Ok(path) = std::env::var("TERRAIN_COMPARE_DUMP") {
        std::fs::write(path, &actual).expect("write diagnostic terrain dump");
    }
    let differences = actual.iter().zip(&expected).filter(|(a, b)| a != b).count()
        + actual.len().abs_diff(expected.len());
    if differences == 0 {
        println!("EXACT MATCH ({} bytes)", actual.len());
    } else {
        let first = actual.iter().zip(&expected).position(|(a, b)| a != b);
        eprintln!(
            "DIFFERS: {differences} bytes, first {first:?}, lengths {} / {}",
            actual.len(),
            expected.len()
        );
        for (i, (a, b)) in actual
            .iter()
            .zip(&expected)
            .enumerate()
            .filter(|(_, (a, b))| a != b)
            .take(12)
        {
            eprintln!("{i}: Rust {a}, Java {b}");
        }
        std::process::exit(1);
    }
}
