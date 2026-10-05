use std::{env, fs, process};
fn main() {
    let a: Vec<_> = env::args().collect();
    if a.len() != 4 {
        eprintln!("usage: entity_compare181 CASE TICKS ORACLE_FILE");
        process::exit(2);
    }
    let expected = fs::read(&a[3]).unwrap();
    let actual = match a[1].as_str() {
        "entitylist10" => worldgen::entity::round10_verify181::entitylist(),
        "itemblock10" => worldgen::entity::round10_verify181::itemblock(),
        "itembase10" => worldgen::entity::round10_verify181::itembase(),
        "itemmap8" => worldgen::entity::round8_verify181::itemmap(),
        "wolfdespawn8" => worldgen::entity::round8_verify181::wolf_despawn(),
        "beditem7" | "signitem7" | "metadata7" | "buckethit7" | "bucketray7" | "logconstructor7" | "vineconstructor7" => worldgen::entity::round7_verify181::replay(&a[1]),
        "blockitem6" => worldgen::entity::round6_verify181::block_items_replay(&expected),
        "spawninputs6" => worldgen::entity::round6_verify181::spawn_inputs_replay(&expected),
        "livingdamage6" | "living6" | "livingnbt6" | "sleep6" | "dyeproperties6" | "dyeblocks6" | "doorhelper6" | "dooritem6" => worldgen::entity::round6_verify181::replay(&a[1]),
        "recorditem5" | "paintingitem5" | "vehicleriders5" | "dyesheep5" | "potionexpiry5"
        | "watchers5" | "wolfremote5" | "health5" | "wolfattack5" | "wolffollow5"
        | "vehiclecollision5" | "vehicleinteract5" | "skeletondeath5" => {
            worldgen::entity::round5_verify181::replay(&a[1])
        }
        "watchers" | "coordinates" | "light" | "registry" => {
            worldgen::entity::api_verify181::replay(&a[1])
        }
        "creeperdeath4" | "sun4" | "network4" | "lightning4" | "retaliation4" | "impact4"
        | "flying4" | "explosionblocks4" | "creeper4" => {
            worldgen::entity::round4_verify181::replay(&a[1])
        }
        "combat" => worldgen::entity::combat_verify181::replay(),
        "explosion" => worldgen::entity::combat_verify181::explosion_replay(),
        "slimebranches" | "skeletonbranches" | "wolfbranches" | "vehiclebranches"
        | "fireballbranches" | "spawnbranches" | "spawnorder" | "factorybranches" => {
            worldgen::entity::branches_verify181::replay(&a[1])
        }
        _ => worldgen::entity::verify181::replay(&a[1], a[2].parse().unwrap()),
    };
    if actual == expected {
        println!("EXACT: entities181 {} ({} bytes)", a[1], actual.len());
        return;
    }
    let path = std::path::Path::new(&a[3]).with_extension("rust.bin");
    fs::write(&path, &actual).unwrap();
    let offset = actual
        .iter()
        .zip(&expected)
        .position(|(a, b)| a != b)
        .unwrap_or(actual.len().min(expected.len()));
    eprintln!(
        "DIFF: {} byte {} (Rust {} bytes, Java {} bytes); actual {}",
        a[1],
        offset,
        actual.len(),
        expected.len(),
        path.display()
    );
    process::exit(1);
}
