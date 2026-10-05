//! Diff the Rust generator against an oracle dump.
//!
//!   compare rawnether <seed> <oracle.bin>
//!   compare gridnether <seed> <wrand> <x0> <z0> <x1> <z1> <each|end|none> <oracle.bin>
//!   compare raw <seed> <oracle.bin>
//!   compare sin <oracle-sin.bin>
//!   compare grid <seed> <wrand> <x0> <z0> <x1> <z1> <each|end|none> <oracle.bin>
//!   compare preload <seed> <wrand> <oracle.bin> [fancy]
//!   compare ticks <seed> <wrand> <lcg> <sound> <n> <clear|rain|thunder> <dx> <dz> <oracle.bin> [fires]
//!   compare teleport <seed> <wrand> <0|-1> <x> <y> <z> <teleporter rand> <oracle.bin>
//!   compare useportal <seed> <wrand> <x> <y> <z> <new-world rand> <teleporter rand> <oracle.bin>
//!   compare respawn <seed> <wrand> <plain|bed|nobed> <oracle.bin>
//!   compare redstone <script> <oracle.bin>

use std::collections::BTreeMap;
use std::process::ExitCode;
use worldgen::scenario::{self, LightMode};
use worldgen::world::tile::TileEntity;
use worldgen::{math, OverworldGenerator, World};

const CHUNK_RECORD: usize = 8 + 32768 + 16384 + 256 + 256 * 16;

fn main() -> ExitCode {
    let a: Vec<String> = std::env::args().collect();
    let p = |i: usize| a[i].parse::<i64>().expect("number");
    match a.get(1).map(String::as_str) {
        Some("regions181") => {
            if a[2]=="write" {
                let bytes=worldgen::world::verify181::regions_write181(std::path::Path::new(&a[3]));std::fs::write(format!("{}.ops",a[3]),bytes).unwrap();ExitCode::SUCCESS
            } else {
                let mut rust=std::fs::read(format!("{}.ops",a[4])).unwrap();rust.extend(worldgen::world::verify181::regions_read181(std::path::Path::new(&format!("{}.mcr",a[3]))));
                let java=std::fs::read(&a[3]).unwrap();if java==rust{println!("EXACT MATCH: region writes, sector reuse/size deltas, and both Java/Rust readers ({} bytes)",rust.len());ExitCode::SUCCESS}else{eprintln!("region replay differs at {:?}",java.iter().zip(&rust).position(|(a,b)|a!=b));ExitCode::FAILURE}
            }
        }
        Some("worldblocks181") => {
            let java=std::fs::read(&a[3]).unwrap();
            let rust=worldgen::world::verify181::bytes(&a[2]);
            if java==rust {println!("EXACT MATCH: worldblocks181 {}, {} bytes",a[2],rust.len());ExitCode::SUCCESS} else {
                eprintln!("worldblocks181 {} differs at {:?}; Java/Rust {}/{} bytes",a[2],java.iter().zip(&rust).position(|(a,b)|a!=b),java.len(),rust.len());
                std::fs::write(format!("target/wb181-{}-rust.bin",a[2]),rust).unwrap();ExitCode::FAILURE
            }
        }
        Some("mapfiles") => {
            let java=std::fs::read(&a[2]).unwrap();let rust=worldgen::decoration_verify::map_files(std::path::Path::new(&format!("{}.save",a[2])),std::path::Path::new("target/map-rust-save"));verdict(java==rust)
        }
        Some("mapread") => {verdict(std::fs::read(&a[2]).unwrap()==std::fs::read(&a[3]).unwrap())}
        Some("fishing") => {
            let java=std::fs::read(&a[2]).unwrap();let rust=worldgen::decoration_verify::fishing();if java==rust {println!("EXACT MATCH: fishing 48000 ticks and events");ExitCode::SUCCESS}else{eprintln!("fishing differ at {:?}; {}/{} bytes",java.iter().zip(&rust).position(|(a,b)|a!=b),java.len(),rust.len());std::fs::write("target/fishing-rust.bin",rust).unwrap();ExitCode::FAILURE}
        }
        Some("maps") => {
            let java=std::fs::read(&a[2]).unwrap();let rust=worldgen::decoration_verify::maps();
            if java==rust {println!("EXACT MATCH: maps palette, terrain updates, markers, NBT and creation");ExitCode::SUCCESS} else {eprintln!("maps differ at {:?}; {}/{} bytes",java.iter().zip(&rust).position(|(a,b)|a!=b),java.len(),rust.len());std::fs::write("target/maps-rust.bin",rust).unwrap();ExitCode::FAILURE}
        }
        Some("paintings") => {
            let java=std::fs::read(&a[2]).unwrap();let rust=worldgen::decoration_verify::paintings();
            if java==rust {println!("EXACT MATCH: paintings selection, walls, overlaps, support ticks, damage, NBT");ExitCode::SUCCESS} else {eprintln!("paintings differ at {:?}; {}/{} bytes",java.iter().zip(&rust).position(|(a,b)|a!=b),java.len(),rust.len());std::fs::write("target/paintings-rust.bin",rust).unwrap();ExitCode::FAILURE}
        }
        Some("signs") => {
            let java=std::fs::read(&a[2]).unwrap();let len=i32::from_le_bytes(java[..4].try_into().unwrap()) as usize;
            let allowed=std::str::from_utf8(&java[4..4+len]).unwrap();let rust=worldgen::decoration_verify::signs(allowed);let java=&java[4+len..];
            if java==rust {println!("EXACT MATCH: signs placement, supports, drops/RNG, editing, NBT");ExitCode::SUCCESS} else {eprintln!("signs differ at {:?}; {}/{} bytes",java.iter().zip(&rust).position(|(a,b)|a!=b),java.len(),rust.len());std::fs::write("target/signs-rust.bin",rust).unwrap();ExitCode::FAILURE}
        }
        Some("musicevents") => {
            let java = std::fs::read(&a[2]).unwrap();
            let rust = worldgen::entity::verify::music_event_bytes();
            if rust == java {
                println!(
                    "EXACT MATCH: notes, jukebox records/drops, all 96 step/break sound properties"
                );
                ExitCode::SUCCESS
            } else {
                eprintln!(
                    "music events differ at {:?}: {}/{} bytes",
                    java.iter().zip(&rust).position(|(a, b)| a != b),
                    java.len(),
                    rust.len()
                );
                std::fs::write("target/music-rust.bin", rust).unwrap();
                ExitCode::FAILURE
            }
        }
        Some("displayevents") => {
            let java = std::fs::read(&a[2]).unwrap();
            let rust = worldgen::entity::verify::display_event_bytes();
            if java == rust {
                println!(
                    "EXACT MATCH: 89600 ambient block callbacks, ordered events and RNG states"
                );
                ExitCode::SUCCESS
            } else {
                eprintln!(
                    "display events differ at {:?}: {}/{} bytes",
                    java.iter().zip(&rust).position(|(a, b)| a != b),
                    java.len(),
                    rust.len()
                );
                std::fs::write("target/display-rust.bin", rust).unwrap();
                ExitCode::FAILURE
            }
        }
        Some("player") => {
            start_replay_identities181(&a[4]);
            let java = std::fs::read(&a[4]).expect("player dump");
            let rust = worldgen::entity::player::verify::player_bytes(&a[2], p(3) as i32);
            finish_replay_identities181();
            if java == rust {
                println!("EXACT MATCH: player {}, {} ticks and world", a[2], a[3]);
                ExitCode::SUCCESS
            } else {
                eprintln!(
                    "player differs at byte {:?}; lengths {}/{}",
                    java.iter().zip(&rust).position(|(a, b)| a != b),
                    java.len(),
                    rust.len()
                );
                std::fs::write("target/player-rust.bin", rust).unwrap();
                ExitCode::FAILURE
            }
        }
        Some("playerinventory") => {
            let java = std::fs::read(&a[2]).expect("inventory dump");
            let rust = worldgen::entity::player::verify::inventory_bytes();
            if java == rust {
                println!("EXACT MATCH: 10000 player inventory operations");
                ExitCode::SUCCESS
            } else {
                eprintln!(
                    "inventory differs at byte {:?}; lengths {}/{}",
                    java.iter().zip(&rust).position(|(a, b)| a != b),
                    java.len(),
                    rust.len()
                );
                std::fs::write("target/inventory-rust.bin", rust).unwrap();
                ExitCode::FAILURE
            }
        }
        Some("vehicleinventory") => {
            let java=std::fs::read(&a[2]).expect("vehicle inventory dump");let rust=worldgen::entity::vehicle_verify::inventory_bytes();
            if java==rust {println!("EXACT MATCH: 10000 original minecart inventory/NBT operations");ExitCode::SUCCESS}
            else {eprintln!("vehicle inventory differs at byte {:?}; lengths {}/{}",java.iter().zip(&rust).position(|(a,b)|a!=b),java.len(),rust.len());std::fs::write("target/vehicleinventory-rust.bin",rust).unwrap();ExitCode::FAILURE}
        }
        Some("vehicles") => {
            let java=std::fs::read(&a[4]).expect("vehicle dump");
            let rust=worldgen::entity::vehicle_verify::replay(&a[2],p(3) as i32);
            if java==rust { println!("EXACT MATCH: vehicles {}, {} ticks",a[2],a[3]); ExitCode::SUCCESS }
            else {
                eprintln!("vehicles differ at byte {:?}; lengths {}/{}",java.iter().zip(&rust).position(|(a,b)|a!=b),java.len(),rust.len());
                std::fs::write("target/vehicles-rust.bin",rust).unwrap(); ExitCode::FAILURE
            }
        }
        Some("entities") => {
            start_replay_identities181(&a[4]);
            let java = std::fs::read(&a[4]).expect("entity dump");
            let rust = worldgen::entity::verify::object_bytes(&a[2], p(3) as i32);
            finish_replay_identities181();
            if java == rust {
                println!("EXACT MATCH: {} entities, {} ticks", a[2], a[3]);
                ExitCode::SUCCESS
            } else {
                let offset = java.iter().zip(&rust).position(|(a, b)| a != b);
                eprintln!(
                    "entities differ at byte {offset:?}; lengths {}/{}",
                    java.len(),
                    rust.len()
                );
                std::fs::write("target/entities-rust.bin", rust).unwrap();
                ExitCode::FAILURE
            }
        }
        Some("entityblocks") => {
            let java = std::fs::read(&a[2]).expect("block geometry dump");
            let rust = worldgen::entity::verify::block_bytes();
            if java == rust {
                println!("EXACT MATCH: 10000 block collision/ray cases");
                ExitCode::SUCCESS
            } else {
                eprintln!(
                    "entity blocks differ at byte {:?}; lengths {}/{}",
                    java.iter().zip(&rust).position(|(a, b)| a != b),
                    java.len(),
                    rust.len()
                );
                std::fs::write("target/blocks-rust.bin", rust).unwrap();
                ExitCode::FAILURE
            }
        }
        Some("entitygeometry") => {
            let java = std::fs::read(&a[2]).expect("geometry dump");
            let rust = worldgen::entity::verify::geometry_bytes();
            if java == rust {
                println!("EXACT MATCH: 10000 geometry cases");
                ExitCode::SUCCESS
            } else {
                eprintln!(
                    "geometry differs at byte {:?}; lengths {}/{}",
                    java.iter().zip(&rust).position(|(a, b)| a != b),
                    java.len(),
                    rust.len()
                );
                ExitCode::FAILURE
            }
        }
        Some("raw") => raw(p(2), &a[3]),
        Some("rawnether") => raw_nether(p(2), &a[3]),
        Some("sin") => sin(&a[2]),
        Some("math") => java_math(&a[2]),
        Some("grid") | Some("gridnether") => {
            let light = match a[8].as_str() {
                "each" => LightMode::Each,
                "end" => LightMode::End,
                _ => LightMode::None,
            };
            let dimension = if a[1] == "gridnether" {
                worldgen::world::Dimension::Nether
            } else {
                worldgen::world::Dimension::Overworld
            };
            let mut w = World::with_dimension(p(2), p(3), dimension);
            let t = std::time::Instant::now();
            scenario::grid(
                &mut w,
                p(4) as i32,
                p(5) as i32,
                p(6) as i32,
                p(7) as i32,
                light,
            );
            eprintln!("rust: {} chunks in {:.2?}", w.chunks.len(), t.elapsed());
            world(&w, &a[9])
        }
        Some("serverwalk") => {
            let identities = std::fs::read(format!("{}.identities", a[7])).expect("server identity dump");
            assert_eq!(identities.len() % 4, 0, "truncated server identity stream");
            worldgen::world::tick::replay_chunk_identities181(identities.chunks_exact(4)
                .map(|b| i32::from_be_bytes(b.try_into().unwrap())).collect());

            // seed wrand radius dir ticks java.bin: the server oracle's scripted walk.
            let dir = std::path::PathBuf::from(&a[5]);
            // Pin item/drop RNGs before spawn search and population can create items.
            // This fixture, like ServerOracle, always creates a fresh server world.
            let mut w = World::new(p(2), p(3));
            w.set_entity_determinism(100, 200, 0);
            w.container_drop_random = std::array::from_fn(|i| worldgen::JavaRandom::new(700 + i as i64));
            w.server = Some(Default::default());
            w.storage = Some(worldgen::storage::Storage::open(&dir).expect("open server save"));
            w.info.name = dir.file_name().unwrap().to_string_lossy().into_owned();
            w.init_weather_and_light();
            w.find_initial_spawn();
            w.prepare_server_spawn_area();
            w.set_allowed_mob_spawns(false, false);
            let mut pm = worldgen::world::server::PlayerManager::new(p(4) as i32);
            let (sx, sz) = (w.spawn.0 as f64 + 0.5, w.spawn.2 as f64 + 0.5);
            pm.add_player(&mut w, sx, sz);
            // The oracle's player is Unsafe-allocated: posY stays 0.
            w.players.push((sx, 0.0, sz));
            w.update_lcg = 12345;
            w.sound_counter = 100;
            for t in 0..p(6) as i32 {
                w.tick();
                w.drain_lighting();
                let (x, z) = walk_path(sx, sz, t);
                w.players[0] = (x, 0.0, z);
                pm.update_player(&mut w, x, z);
            }
            pm.remove_player(&mut w);
            w.save_world(true);
            worldgen::world::tick::finish_chunk_identity_replay181();
            println!("rust: {} chunks loaded at the end", w.chunks.len());
            let state = format!("time {}\nlcg {}\nsound {}\nrand {}\n",
                w.world_time, w.update_lcg, w.sound_counter, w.rand.next_long());
            let java = std::fs::read_to_string(format!("{}.txt", a[7])).expect("server state").replace('\r', "");
            let result = world(&w, &a[7]);
            if state == java { result } else {
                eprintln!("server state differs:\n--- java\n{java}--- rust\n{state}");
                ExitCode::FAILURE
            }
        }
        Some("makesave") => {
            // Create a world in a save directory the way the viewer does (preload, then save).
            let (w, new) =
                World::open(Some(std::path::Path::new(&a[4])), p(2), p(3), false).expect("open");
            println!(
                "{} world, {} chunks, spawn {:?}",
                if new { "new" } else { "existing" },
                w.chunks.len(),
                w.spawn
            );
            ExitCode::SUCCESS
        }
        Some("terrain-makesave") => {
            // Match Mode_terrainsave181's isolated, deterministic save fixture.
            let mut w = World::new(p(2), p(3));
            w.set_entity_determinism(100, 200, 0);
            w.storage = Some(worldgen::storage::Storage::open(
                std::path::Path::new(&a[4]),
            ).expect("open terrain save"));
            w.find_initial_spawn();
            w.preload_spawn_area();
            w.save_world(true);
            println!("deterministic terrain save: {} chunks", w.chunks.len());
            ExitCode::SUCCESS
        }
        Some("preload-worldonly") => {
            let mut w = World::new(p(2), p(3));
            w.find_initial_spawn();
            w.preload_spawn_area();
            WORLD_ONLY.store(true, std::sync::atomic::Ordering::Relaxed);
            world(&w, &a[4])
        }
        Some("convertworld") => {
            // The port's SaveConverterMcRegion: prints every progress step like the oracle.
            let dir = std::path::Path::new(&a[2]).join(&a[3]);
            println!("old {}", worldgen::convert::is_old_map_format(&dir));
            let mut log = |step: worldgen::scenario::LoadStep| match step {
                worldgen::scenario::LoadStep::Progress(n) => println!("progress {n}"),
                _ => {}
            };
            worldgen::convert::convert_map_format(&dir, &mut log);
            println!("old {}", worldgen::convert::is_old_map_format(&dir));
            ExitCode::SUCCESS
        }
        Some("savecheck") => {
            // Load every chunk listed in the dump from a save directory, compare (saves keep
            // no scheduled ticks or pending light updates, so those are skipped).
            let dir = std::path::Path::new(&a[2]);
            let storage = worldgen::storage::Storage::open(dir).expect("open save");
            let info = storage.load_level().expect("level.dat");
            let mut w = World::new(info.seed, 0);
            w.spawn = info.spawn;
            w.world_time = info.time;
            w.storage = Some(storage);
            let data = std::fs::read(&a[3]).expect("read dump");
            for (cx, cz) in dump_chunk_list(&data) {
                w.load_chunk(cx, cz);
            }
            WORLD_ONLY.store(true, std::sync::atomic::Ordering::Relaxed);
            world(&w, &a[3])
        }
        Some("useportal") => {
            // seed wrand x y z new-world-rand teleporter-rand: the vanilla new-world flow, a
            // player at x y z, then Minecraft.usePortal into the Nether.
            let mut w = World::new(p(2), p(3));
            w.find_initial_spawn();
            w.preload_spawn_area();
            w.set_entity_determinism(100, 200, 1000);
            let f = |i: usize| a[i].parse::<f64>().expect("coordinate");
            let mut e = worldgen::entity::Entity::player(&mut w, "Player");
            e.set_location_and_angles(f(4), f(5), f(6), 30.0, 10.0);
            let id = e.entity_id;
            w.entity_joined_world(e);
            let t = std::time::Instant::now();
            let mut n = w.use_portal(id, p(7), p(8), &mut |_| {}).expect("player");
            let e = n.entities[&id].clone();
            eprintln!("rust: arrived at {:?} in {:.2?}", (e.pos.x, e.pos.y, e.pos.z), t.elapsed());
            let b = |v: f64| v.to_bits() as i64;
            let fb = |v: f32| v.to_bits() as i32;
            let state = format!(
                "pos {} {} {}
prev {} {} {}
look {} {} {} {}
dimension {} entities {} players {}
rand {}
",
                b(e.pos.x),
                b(e.pos.y),
                b(e.pos.z),
                b(e.last_tick_pos.x),
                b(e.last_tick_pos.y),
                b(e.last_tick_pos.z),
                fb(e.yaw),
                fb(e.pitch),
                fb(e.prev_yaw),
                fb(e.prev_pitch),
                e.player_state().map_or(0, |s| s.dimension),
                n.loaded_entity_list.len(),
                n.player_entities.len(),
                n.rand.next_long()
            );
            let java = std::fs::read_to_string(format!("{}.txt", a[9])).expect("read state").replace("\r", "");
            if java != state {
                eprintln!("use portal differs:
--- java
{java}--- rust
{state}");
                let _ = world(&n, &a[9]);
                return ExitCode::FAILURE;
            }
            world(&n, &a[9])
        }
        Some("respawn") => {
            // seed wrand plain|bed|nobed: Minecraft.respawn(false, 0) after a death.
            let mut w = World::new(p(2), p(3));
            w.find_initial_spawn();
            w.preload_spawn_area();
            // Ids continue past entities made while generating (Entity.nextEntityID is global).
            w.set_entity_determinism(100, 200, 1000);
            let sp = w.spawn;
            let mut e = worldgen::entity::Entity::player(&mut w, "Player");
            e.set_location_and_angles(sp.0 as f64 + 20.5, 90.0, sp.2 as f64 - 7.5, 45.0, 5.0);
            let id = e.entity_id;
            w.entity_joined_world(e);
            let (bx, bz) = (sp.0 + 4, sp.2);
            let mut by = 127;
            while by > 0 && w.get_block_id(bx, by - 1, bz) == 0 {
                by -= 1;
            }
            let scenario = a[4].as_str();
            if scenario == "bed" {
                for (z, meta) in [(bz, 0), (bz + 1, 8)] {
                    let c = w.chunk_mut(bx >> 4, z >> 4);
                    c.blocks[worldgen::world::chunk::idx(bx & 15, by, z & 15)] = 26;
                    c.data.set(bx & 15, by, z & 15, meta);
                }
            }
            if let Some(s) = w.entities.get_mut(&id).and_then(|e| e.player_state_mut()) {
                if scenario != "plain" {
                    s.spawn = Some((bx, by, bz));
                }
            }
            w.entities.get_mut(&id).unwrap().living.health = 0;
            let chat = w.respawn_player(id, &mut |_| {});
            let e = w.entities[&id].clone();
            let s = e.player_state().unwrap();
            let state = format!(
                "pos {} {} {}
look {} {}
id {} health {} chat {} home {}
entities {} players {}
rand {}
",
                e.pos.x.to_bits() as i64,
                e.pos.y.to_bits() as i64,
                e.pos.z.to_bits() as i64,
                e.yaw.to_bits() as i32,
                e.pitch.to_bits() as i32,
                e.entity_id,
                e.living.health,
                chat.is_some(),
                s.spawn.map_or("none".to_string(), |(x, y, z)| format!("{x},{y},{z}")),
                w.loaded_entity_list.iter().filter(|i| w.entities.get(i).is_some_and(|e| !e.dead)).count(),
                w.player_entities.len(),
                w.rand.next_long()
            );
            let java = std::fs::read_to_string(format!("{}.txt", a[5])).expect("read state").replace('\r', "");
            if java != state {
                eprintln!("respawn differs:
--- java
{java}--- rust
{state}");
                let _ = world(&w, &a[5]);
                return ExitCode::FAILURE;
            }
            world(&w, &a[5])
        }
        Some("redstone") => {
            // script: the flat block world of the oracle's redstone mode, driven by the
            // same commands (place, remove, use, tick).
            use worldgen::world::chunk::{idx, Chunk};
            let mut w = World::new(0, 300);
            w.set_allowed_mob_spawns(false, false);
            w.set_entity_determinism(100, 200, 1000);
            w.update_lcg = 0;
            w.sound_counter = 0;
            for cx in -3..=3 {
                for cz in -3..=3 {
                    let mut c = Chunk::new(cx, cz, vec![0; 32768].into_boxed_slice().try_into().unwrap());
                    for x in 0..16 {
                        for z in 0..16 {
                            c.blocks[idx(x, 63, z)] = 1;
                        }
                    }
                    c.loaded = true;
                    c.populated = true;
                    w.chunks.insert((cx, cz), Box::new(c));
                    w.chunk_list.push((cx, cz));
                }
            }
            let mut p = worldgen::entity::Entity::player(&mut w, "Player");
            p.set_location_and_angles(0.5, 64.0, 0.5, 0.0, 0.0);
            let mut ticks = 0;
            let mut probes=std::collections::HashMap::<u64,Vec<i32>>::new();
            let mut steps = String::new();
            let text = std::fs::read_to_string(&a[2]).expect("read script");
            for line in text.lines() {
                let line = line.split('#').next().unwrap_or("").trim();
                if line.is_empty() {
                    continue;
                }
                let words: Vec<&str> = line.split_whitespace().collect();
                let v: Vec<i32> = words[1..].iter().map(|s| s.parse().unwrap()).collect();
                match words[0] {
                    "place" => {
                        w.set_block_and_metadata_with_notify(v[0], v[1], v[2], v[3] as u8, v[4]);
                    }
                    "itemplace" => {
                        p.set_location_and_angles(v[5] as f64/1000.0,v[6] as f64/1000.0,v[7] as f64/1000.0,v[8] as f32/1000.0,0.0);
                        let mut stack=worldgen::entity::player::ItemStack::new(v[4],1,0);
                        p.place_block_item(&mut w,&mut stack,v[0],v[1],v[2],v[3]);
                    }
                    "placedby" => {
                        p.set_location_and_angles(v[3] as f64/1000.0,v[4] as f64/1000.0,v[5] as f64/1000.0,v[6] as f32/1000.0,0.0);
                        let id=w.get_block_id(v[0],v[1],v[2]);p.block_placed(&mut w,id,v[0],v[1],v[2],0);
                    }
                    "click" => { let id=w.get_block_id(v[0],v[1],v[2]);p.block_clicked(&mut w,id,v[0],v[1],v[2]); }
                    "item" => {
                        let e=worldgen::entity::Entity::item(&mut w,v[0] as f64/1000.0,v[1] as f64/1000.0,v[2] as f64/1000.0,worldgen::world::tile::ItemStack::new(v[3],v[4],v[5]));w.entity_joined_world(e);
                    }
                    "mob" => {
                        let name=match v[3] {90=>"Pig",91=>"Sheep",92=>"Cow",93=>"Chicken",54=>"Zombie",_=>panic!("mob id")};
                        let mut e=worldgen::entity::Entity::living(&mut w,name).unwrap();
                        e.set_location_and_angles(v[0] as f64/1000.0,v[1] as f64/1000.0,v[2] as f64/1000.0,0.0,0.0);w.entity_joined_world(e);
                    }
                    "reload" => {
                        let te=w.get_block_tile_entity(v[0],v[1],v[2]).unwrap().clone();
                        let te=match te {
                            worldgen::world::tile::TileEntity::Note(n)=>worldgen::world::tile::TileEntity::Note(worldgen::world::notes::Note::from_nbt(&n.to_nbt(v[0],v[1],v[2]))),
                            worldgen::world::tile::TileEntity::Piston(n)=>worldgen::world::tile::TileEntity::Piston(worldgen::world::pistons::Piston::from_nbt(&n.to_nbt(v[0],v[1],v[2]))),
                            _=>panic!("reload command expects Music/Piston")
                        };
                        w.remove_block_tile_entity(v[0],v[1],v[2]);w.set_block_tile_entity(v[0],v[1],v[2],te);
                    }
                    "properties" => {
                        for id in 1..=96u8 {if worldgen::block::props(id).exists {steps.push_str(&format!("property {id} {} {} {}\n",worldgen::world::pistons::mobility(id),worldgen::entity::player::registry::hardness(id).to_bits() as i32,worldgen::block::props(id).container as i32));}}
                    }
                    "boxes" => {
                        let (x,y,z)=(v[0],v[1],v[2]);let id=w.get_block_id(x,y,z);
                        steps.push_str("collision");
                        if let Some(b)=w.block_collision_box(id,x,y,z) {for d in [b.min_x,b.min_y,b.min_z,b.max_x,b.max_y,b.max_z] {steps.push_str(&format!(" {}",d.to_bits() as i64));}}else{steps.push_str(" null");}
                        steps.push('\n');
                        if id!=0 {w.set_block_bounds_based_on_state(id,x,y,z);let b=w.block_bounds[id as usize];steps.push_str("selection");for d in [b.min_x,b.min_y,b.min_z,b.max_x,b.max_y,b.max_z] {steps.push_str(&format!(" {}",d.to_bits() as i64));}steps.push('\n');}
                    }
                    "probe" => {
                        w.set_block_tile_entity(v[0],v[1],v[2],worldgen::world::tile::TileEntity::Note(worldgen::world::notes::Note {note:17,powered:false}));
                        probes.insert(w.next_tile_id-1,v[3..].to_vec());
                    }
                    "scan" => {
                        w.scan_tile_entities(|w,id| {
                            if let Some(v)=probes.remove(&id) {
                                let (x,y,z)=(v[1],v[2],v[3]);
                                match v[0] {
                                    0=>w.remove_block_tile_entity(x,y,z),
                                    1=>w.set_block_tile_entity(x,y,z,worldgen::world::tile::TileEntity::Note(worldgen::world::notes::Note {note:v[4] as i8,powered:false})),
                                    2=>{w.set_block_and_metadata_with_notify(x,y,z,v[4] as u8,0);},
                                    3=>{w.remove_block_tile_entity(x,y,z);w.get_block_tile_entity(x,y,z);w.get_block_tile_entity(x,y,z);},
                                    4=>{w.set_block_with_notify(x,y,z,0);},
                                    _=>panic!("probe action")
                                }
                                let (x,y,z)=w.tile_objects[&id].position;w.remove_block_tile_entity(x,y,z);
                            } else {w.tick_tile_entity(id);}
                        });
                    }
                    "lookup" => {
                        let note=match w.get_block_tile_entity(v[0],v[1],v[2]) {Some(worldgen::world::tile::TileEntity::Note(n))=>n.note as i32,None=>-1,_=>panic!("lookup note")};
                        steps.push_str(&format!("lookup {note}\n"));
                    }
                    "setnote" => {
                        let mut n=worldgen::nbt::Compound::new();n.put("note",worldgen::nbt::Tag::Byte(v[3] as i8));
                        w.set_block_tile_entity(v[0],v[1],v[2],worldgen::world::tile::TileEntity::Note(worldgen::world::notes::Note::from_nbt(&n)));
                    }
                    "chunkcycle" => {
                        w.unload_chunk_entities(v[0],v[1]);
                        w.chunk_mut(v[0],v[1]).loaded=true;
                        w.load_chunk_entities(v[0],v[1]);
                    }
                    "chunkreload" => {
                        let n=worldgen::storage::chunk_to_nbt(w.chunk(v[0],v[1]),w.world_time);
                        w.unload_chunk_entities(v[0],v[1]);
                        // This fixture replaces the provider's chunk reference.
                        // Retained update-list tiles keep the discarded chunk's
                        // payload, just as in the production unload path.
                        w.detach_unloaded_chunk_tiles181(v[0],v[1]);
                        let mut c=worldgen::storage::chunk_from_nbt(&n).unwrap();c.loaded=true;
                        w.chunks.insert((v[0],v[1]),Box::new(c));w.load_chunk_entities(v[0],v[1]);
                    }
                    "events" => { redstone_events(&mut w,&mut steps); }
                    "state" => { redstone_state(&w,&mut steps); }
                    "remove" => {
                        w.set_block_with_notify(v[0], v[1], v[2], 0);
                    }
                    "use" => {
                        if w.get_block_id(v[0], v[1], v[2]) > 0 {
                            p.activate_block(&mut w, v[0], v[1], v[2]);
                        }
                    }
                    "tick" => {
                        for _ in 0..v[0] {
                            w.update_entities();
                            w.tick();
                            w.drain_lighting();
                            ticks += 1;
                        }
                    }
                    "check" => {
                        let n = v.get(6).copied().unwrap_or(0);
                        for t in 0..=n {
                            if t > 0 {
                                w.update_entities();
                            w.tick();
                                w.drain_lighting();
                                ticks += 1;
                            }
                            steps.push_str(&format!("{ticks}:"));
                            for y in v[1]..=v[4] {
                                for z in v[2]..=v[5] {
                                    for x in v[0]..=v[3] {
                                        let id = w.get_block_id(x, y, z);
                                        let m = w.get_block_metadata(x, y, z);
                                        steps.push_str(&format!(" {id}/{m}"));
                                    }
                                }
                            }
                            steps.push('\n');
                        }
                    }
                    other => panic!("unknown command {other}"),
                }
                w.drain_lighting();
            }
            w.spawn = (0, 64, 0);
            let state = format!(
                "ticks {} time {} entities {}
rand {}
{steps}",
                ticks,
                w.world_time,
                w.loaded_entity_list.len(),
                w.rand.next_long()
            );
            let java = std::fs::read_to_string(format!("{}.txt", a[3])).expect("read state").replace('\r', "");
            if java != state {
                let (jl, rl): (Vec<&str>, Vec<&str>) = (java.lines().collect(), state.lines().collect());
                match jl.iter().zip(&rl).position(|(a, b)| a != b) {
                    Some(i) => eprintln!("redstone differs at line {i}:
 java {}
 rust {}", jl[i], rl[i]),
                    None => eprintln!("redstone differs in length: java {} lines, rust {}", jl.len(), rl.len()),
                }
                let _ = world(&w, &a[3]);
                return ExitCode::FAILURE;
            }
            world(&w, &a[3])
        }
        Some("teleport") => {
            // seed wrand dimension x y z teleporter-rand: Minecraft.usePortal's preload, then
            // Teleporter.func_4107_a for an entity at x y z.
            let dimension = if p(4) == -1 { worldgen::world::Dimension::Nether } else { worldgen::world::Dimension::Overworld };
            let mut w = World::with_dimension(p(2), p(3), dimension);
            let f = |i: usize| a[i].parse::<f64>().expect("coordinate");
            let pos = (f(5), f(6), f(7));
            w.spawn = (pos.0 as i32, 64, pos.2 as i32);
            w.preload_spawn_area();
            let mut rand = worldgen::JavaRandom::new(p(8));
            let t = std::time::Instant::now();
            let end = w.teleport_position(pos, &mut rand).unwrap_or(pos);
            eprintln!("rust: placed at {end:?} in {:.2?}", t.elapsed());
            let state = format!(
                "pos {} {} {}
rand {}
",
                end.0.to_bits() as i64,
                end.1.to_bits() as i64,
                end.2.to_bits() as i64,
                w.rand.next_long()
            );
            let java = std::fs::read_to_string(format!("{}.txt", a[9])).expect("read state").replace('\r', "");
            if java != state {
                eprintln!("teleport differs:
--- java
{java}--- rust
{state}");
                let _ = world(&w, &a[9]);
                return ExitCode::FAILURE;
            }
            world(&w, &a[9])
        }
        Some("ticks") => {
            let identities = std::fs::read(format!("{}.identities", a[10])).expect("tick identity dump (use ticksintegration181 oracle)");
            assert_eq!(identities.len() % 4, 0, "truncated identity stream");
            worldgen::world::tick::replay_chunk_identities181(identities.chunks_exact(4)
                .map(|b| i32::from_be_bytes(b.try_into().unwrap())).collect());
            let frames = std::fs::read(format!("{}.frames", a[10])).expect("tick frame states");
            assert_eq!(frames.len(), p(6) as usize * 24, "truncated tick frame states");

            let mut w = World::new(p(2), p(3));
            w.find_initial_spawn();
            w.preload_spawn_area();
            w.difficulty = 0; // The legacy terrain-only oracle leaves World.difficultySetting at its JVM default.
            w.set_allowed_mob_spawns(false, false);
            w.update_lcg = p(4) as i32;
            w.sound_counter = p(5) as i32;
            if a[7] != "clear" {
                w.info.raining = true;
                w.info.rain_time = 1000000;
                w.raining_strength = 1.0;
                w.prev_raining_strength = 1.0;
                if a[7] == "thunder" {
                    w.info.thundering = true;
                    w.info.thunder_time = 1000000;
                    w.thundering_strength = 1.0;
                    w.prev_thundering_strength = 1.0;
                }
            }
            w.skylight_subtracted = w.calculate_skylight_subtracted(1.0);
            let f = |i: usize| a[i].parse::<f64>().expect("number");
            let (sx, sy, sz) = w.spawn;
            w.players.push((
                sx as f64 + 0.5 + f(8),
                sy as f64 + 1.62,
                sz as f64 + 0.5 + f(9),
            ));
            for f in a
                .get(11)
                .map_or("", String::as_str)
                .split(',')
                .filter(|f| !f.is_empty())
            {
                let v: Vec<i32> = f.split(':').map(|v| v.parse().unwrap()).collect();
                let (fx, fz) = (sx + v[0], sz + v[1]);
                let fy = w.get_height_value(fx, fz);
                if v.len() >= 3 {
                    // dx:dz:id[:meta] places that block on top instead of fire.
                    w.set_block_and_metadata_with_notify(fx, fy, fz, v[2] as u8, v.get(3).copied().unwrap_or(0));
                } else {
                    w.set_block_with_notify(fx, fy, fz, worldgen::block::FIRE);
                }
            }
            w.drain_lighting();
            let t = std::time::Instant::now();
            w.set_entity_determinism(100, 200, 0);
            w.drain_events();
            let mut events = Vec::new();
            for frame in 0..p(6) {
                w.tick();
                w.drain_lighting();
                worldgen::world::events::write_events(&w.drain_events(), &mut events);
                let expected = &frames[frame as usize * 24..][..24];
                let mut actual = Vec::with_capacity(24);
                actual.extend_from_slice(&w.world_time.to_be_bytes());
                actual.extend_from_slice(&w.update_lcg.to_be_bytes());
                actual.extend_from_slice(&w.sound_counter.to_be_bytes());
                actual.extend_from_slice(&w.rand.state().to_be_bytes());
                if expected != actual {
                    eprintln!("tick frame {} differs: java time={} lcg={} sound={} rand={}; rust time={} lcg={} sound={} rand={}",
                        frame + 1, i64::from_be_bytes(expected[..8].try_into().unwrap()),
                        i32::from_be_bytes(expected[8..12].try_into().unwrap()),
                        i32::from_be_bytes(expected[12..16].try_into().unwrap()),
                        i64::from_be_bytes(expected[16..24].try_into().unwrap()),
                        w.world_time, w.update_lcg, w.sound_counter, w.rand.state());
                    let mut dump = Vec::new();
                    worldgen::entity::verify::append_world(&w, &mut dump);
                    for (i, coordinate) in [w.spawn.0, w.spawn.1, w.spawn.2].iter().enumerate() {
                        dump[8 + i * 4..12 + i * 4].copy_from_slice(&coordinate.to_le_bytes());
                    }
                    std::fs::write("target/tick-first-difference-rust.bin", dump).unwrap();
                    return ExitCode::FAILURE;
                }
            }
            worldgen::world::tick::finish_chunk_identity_replay181();
            let java_events =
                std::fs::read(format!("{}.events", a[10])).expect("read world events");
            let events_ok = events == java_events;
            if !events_ok {
                eprintln!(
                    "world events differ: lengths {}/{} first byte {:?}",
                    events.len(),
                    java_events.len(),
                    events.iter().zip(&java_events).position(|(a, b)| a != b)
                );
                std::fs::write("target/world-events-rust.bin", &events).unwrap();
            }
            eprintln!(
                "rust: {} ticks, {} chunks in {:.2?}",
                p(6),
                w.chunks.len(),
                t.elapsed()
            );
            let counts: Vec<String> = (0..256)
                .filter_map(|i| {
                    let n = worldgen::world::behavior::TICK_COUNTS[i]
                        .load(std::sync::atomic::Ordering::Relaxed);
                    (n > 0).then(|| {
                        format!(
                            "{}={n}",
                            worldgen::block::props(i as u8)
                                .name
                                .trim_start_matches("tile.")
                        )
                    })
                })
                .collect();
            eprintln!("block ticks: {}", counts.join(" "));
            let state = format!(
                "time {}\nlcg {}\nsound {}\nrain {} {} {}\nthunder {} {} {}\nskylight {}\nflash {} bolts {}\nrand {}\n",
                w.world_time,
                w.update_lcg,
                w.sound_counter,
                w.info.raining,
                w.info.rain_time,
                w.raining_strength.to_bits() as i32,
                w.info.thundering,
                w.info.thunder_time,
                w.thundering_strength.to_bits() as i32,
                w.skylight_subtracted,
                w.lightning_flash,
                w.lightning_strikes.len(),
                w.rand.next_long()
            );
            let java = std::fs::read_to_string(format!("{}.txt", a[10]))
                .expect("read state")
                .replace('\r', "");
            let state_ok = java == state;
            if !state_ok {
                eprintln!("tick state differs:\n--- java\n{java}--- rust\n{state}");
            }
            let r = world(&w, &a[10]);
            if state_ok && events_ok {
                r
            } else {
                ExitCode::FAILURE
            }
        }
        Some("preload") => {
            let t = std::time::Instant::now();
            let mut w = World::new(p(2), p(3));
            w.leaves_opaque = a.get(5).map(String::as_str) != Some("fancy");
            w.find_initial_spawn();
            w.preload_spawn_area();
            eprintln!(
                "rust: spawn {:?}, {} chunks in {:.2?}",
                w.spawn,
                w.chunks.len(),
                t.elapsed()
            );
            world(&w, &a[4])
        }
        _ => {
            eprintln!("see header of compare.rs for usage");
            ExitCode::from(2)
        }
    }
}

struct Reader<'a> {
    b: &'a [u8],
    o: usize,
}

impl<'a> Reader<'a> {
    fn take(&mut self, n: usize) -> &'a [u8] {
        let s = &self.b[self.o..self.o + n];
        self.o += n;
        s
    }
    fn u8(&mut self) -> u8 {
        self.take(1)[0]
    }
    fn i16(&mut self) -> i16 {
        i16::from_le_bytes(self.take(2).try_into().unwrap())
    }
    fn i32(&mut self) -> i32 {
        i32::from_le_bytes(self.take(4).try_into().unwrap())
    }
    fn i64(&mut self) -> i64 {
        i64::from_le_bytes(self.take(8).try_into().unwrap())
    }
    fn f64(&mut self) -> f64 {
        f64::from_le_bytes(self.take(8).try_into().unwrap())
    }
}

// ------------------------------------------------------------------ raw terrain

fn raw(seed: i64, path: &str) -> ExitCode {
    let data = std::fs::read(path).expect("read oracle dump");
    assert_eq!(&data[..8], b"B173RAW1", "not a raw oracle dump");
    let body = &data[8..];
    assert_eq!(body.len() % CHUNK_RECORD, 0, "truncated dump");

    let mut gen = OverworldGenerator::new(seed);
    let (mut chunks, mut bad_chunks, mut bad_blocks) = (0, 0, 0usize);
    let (mut bad_biomes, mut bad_climate) = (0usize, 0usize);
    for rec in body.chunks(CHUNK_RECORD) {
        let mut r = Reader { b: rec, o: 0 };
        let (cx, cz) = (r.i32(), r.i32());
        let blocks = r.take(32768);
        let _data = r.take(16384);
        let biomes = r.take(256);
        let chunk = gen.generate(cx, cz);
        chunks += 1;

        // Climate arrays still hold this chunk's 16x16 values right after generate().
        let temps: Vec<f64> = (0..256).map(|_| r.f64()).collect();
        let humid: Vec<f64> = (0..256).map(|_| r.f64()).collect();
        for i in 0..256 {
            let (t, h) = (temps[i], humid[i]);
            if t.to_bits() != gen.biome_source.temperature[i].to_bits()
                || h.to_bits() != gen.biome_source.humidity[i].to_bits()
            {
                if bad_climate == 0 {
                    eprintln!(
                        "climate mismatch chunk ({cx},{cz}) col {i}: java t={t:e} h={h:e}, rust t={:e} h={:e}",
                        gen.biome_source.temperature[i], gen.biome_source.humidity[i]
                    );
                }
                bad_climate += 1;
            }
            if biomes[i] != chunk.biomes[i] as u8 {
                bad_biomes += 1;
            }
        }

        let mut diff = 0;
        for (i, (&j, &r)) in blocks.iter().zip(chunk.blocks.iter()).enumerate() {
            if j != r {
                if diff < 5 {
                    let (x, z, y) = (i >> 11, (i >> 7) & 15, i & 127);
                    eprintln!("chunk ({cx},{cz}) block x={x} y={y} z={z}: java={j} rust={r}");
                }
                diff += 1;
            }
        }
        if diff > 0 {
            bad_chunks += 1;
            bad_blocks += diff;
        }
    }

    println!(
        "seed {seed}: {chunks} chunks, {bad_chunks} differ ({bad_blocks} blocks), \
         {bad_biomes} biome columns differ, {bad_climate} climate values differ"
    );
    verdict(bad_chunks == 0 && bad_biomes == 0 && bad_climate == 0)
}

fn raw_nether(seed: i64, path: &str) -> ExitCode {
    let data = std::fs::read(path).expect("read oracle dump");
    assert_eq!(&data[..8], b"B173RAW1", "not a raw oracle dump");
    let body = &data[8..];
    assert_eq!(body.len() % CHUNK_RECORD, 0, "truncated dump");

    let mut gen = worldgen::nether::NetherGenerator::new(seed);
    let (mut chunks, mut bad_chunks, mut bad_blocks) = (0, 0, 0usize);
    let (mut bad_biomes, mut bad_climate, mut bad_metadata) = (0usize, 0usize, 0usize);
    let mut climate = worldgen::biome::BiomeSource::hell();
    let mut biome_buffer = Vec::new();
    for rec in body.chunks(CHUNK_RECORD) {
        let mut r = Reader { b: rec, o: 0 };
        let (cx, cz) = (r.i32(), r.i32());
        let blocks = r.take(32768);
        let metadata = r.take(16384);
        let metadata_diff = metadata.iter().filter(|&&v| v != 0).count();
        bad_metadata += metadata_diff;
        if metadata_diff > 0 {
            eprintln!("chunk ({cx},{cz}): {metadata_diff} metadata bytes differ");
        }
        let biomes = r.take(256);
        let chunk = gen.generate(cx, cz);
        chunks += 1;

        // The Nether generator does not call its WorldChunkManager; the oracle
        // explicitly queries the manager after each provideChunk.
        climate.biomes(&mut biome_buffer, cx * 16, cz * 16, 16, 16);
        let temps: Vec<f64> = (0..256).map(|_| r.f64()).collect();
        let humid: Vec<f64> = (0..256).map(|_| r.f64()).collect();
        for i in 0..256 {
            let (t, h) = (temps[i], humid[i]);
            if t.to_bits() != climate.temperature[i].to_bits()
                || h.to_bits() != climate.humidity[i].to_bits()
            {
                if bad_climate == 0 {
                    eprintln!(
                        "climate mismatch chunk ({cx},{cz}) col {i}: java t={t:e} h={h:e}, rust t={:e} h={:e}",
                        climate.temperature[i], climate.humidity[i]
                    );
                }
                bad_climate += 1;
            }
            if biomes[i] != chunk.biomes[i] as u8 || biomes[i] != biome_buffer[i] as u8 {
                bad_biomes += 1;
            }
        }

        let mut diff = 0;
        for (i, (&j, &r)) in blocks.iter().zip(chunk.blocks.iter()).enumerate() {
            if j != r {
                if diff < 5 {
                    let (x, z, y) = (i >> 11, (i >> 7) & 15, i & 127);
                    eprintln!("chunk ({cx},{cz}) block x={x} y={y} z={z}: java={j} rust={r}");
                }
                diff += 1;
            }
        }
        if diff > 0 {
            bad_chunks += 1;
            bad_blocks += diff;
        }
    }

    println!(
        "seed {seed}: {chunks} chunks, {bad_chunks} differ ({bad_blocks} blocks), \
         {bad_biomes} biome columns differ, {bad_climate} climate values differ"
    );
    println!("{bad_metadata} metadata bytes differ");
    verdict(bad_chunks == 0 && bad_metadata == 0 && bad_biomes == 0 && bad_climate == 0)
}

fn verdict(ok: bool) -> ExitCode {
    if ok {
        println!("EXACT MATCH");
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

fn sin(path: &str) -> ExitCode {
    let data = std::fs::read(path).expect("read sin dump");
    let mut bad = 0;
    for (i, r) in math::sin_table_bits().enumerate() {
        let j = u32::from_le_bytes(data[i * 4..i * 4 + 4].try_into().unwrap());
        if j != r {
            if bad < 5 {
                eprintln!(
                    "sin[{i}]: java={:e} rust={:e}",
                    f32::from_bits(j),
                    f32::from_bits(r)
                );
            }
            bad += 1;
        }
    }
    println!("sin table: {bad} of 65536 entries differ");
    if bad == 0 {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

// ------------------------------------------------------------------ whole worlds

#[derive(Default)]
struct Tally {
    count: usize,
    shown: usize,
}

impl Tally {
    fn hit(&mut self, what: &str, msg: impl FnOnce() -> String) {
        self.count += 1;
        if self.shown < 8 {
            self.shown += 1;
            eprintln!("  {what}: {}", msg());
        }
    }
}

/// The server oracle's scripted walk (ServerOracle.path).
fn walk_path(sx: f64, sz: f64, t: i32) -> (f64, f64) {
    if t < 600 {
        return (sx + 3.0 * t as f64, sz + 1.0 * t as f64);
    }
    let (x600, z600) = (sx + 3.0 * 600.0, sz + 1.0 * 600.0);
    (x600 - 3.0 * (t - 600) as f64, z600 + 0.5 * (t - 600) as f64)
}

static WORLD_ONLY: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// Chunk coordinates in a world dump, in order.
fn dump_chunk_list(data: &[u8]) -> Vec<(i32, i32)> {
    let mut r = Reader { b: data, o: 20 };
    let n = r.i32() as usize;
    let mut out = Vec::new();
    for _ in 0..n {
        out.push((r.i32(), r.i32()));
        r.take(1 + 32768 + 16384 * 3 + 256);
        let nte = r.i32();
        for _ in 0..nte {
            r.take(12);
            match r.u8() {
                1 => {
                    r.take(27 * 5);
                }
                2 => {
                    let len = r.u8() as usize;
                    r.take(len);
                }
                3 => { r.take(3 * 5 + 4); }
                4 => { r.take(9 * 5); }
                5 => { r.take(2); }
                6 => { r.take(5 * 4 + 2); }
                tag => panic!("bad tile entity tag {tag}"),
            }
        }
    }
    out
}

const BLOCK_NAMES: fn(u8) -> &'static str = |id| worldgen::block::props(id).name;

fn world(w: &World, path: &str) -> ExitCode {
    let data = std::fs::read(path).expect("read world dump");
    let mut r = Reader { b: &data, o: 0 };
    assert_eq!(r.take(8), b"B173WLD1", "not a world dump");
    let spawn = (r.i32(), r.i32(), r.i32());
    let n = r.i32() as usize;

    let mut cats: BTreeMap<&str, Tally> = BTreeMap::new();
    let mut ok = true;
    if spawn != w.spawn {
        eprintln!("spawn differs: java {spawn:?} rust {:?}", w.spawn);
        ok = false;
    }
    if n != w.chunks.len() {
        eprintln!("chunk count differs: java {n} rust {}", w.chunks.len());
        ok = false;
    }

    let mut populated = 0;
    for _ in 0..n {
        let (cx, cz) = (r.i32(), r.i32());
        let pop = r.u8() != 0;
        let blocks = r.take(32768);
        let meta = r.take(16384);
        let sky = r.take(16384);
        let blk = r.take(16384);
        let height = r.take(256);
        let nte = r.i32();
        let mut tes = Vec::new();
        for _ in 0..nte {
            let (x, y, z) = (r.i32(), r.i32(), r.i32());
            let te = match r.u8() {
                1 => {
                    let mut slots = Box::new([None; 27]);
                    for s in slots.iter_mut() {
                        let (id, count, dmg) = (r.i16(), r.u8(), r.i16());
                        if id >= 0 {
                            *s = Some(worldgen::world::tile::ItemStack {
                                id,
                                count: count as i32,
                                damage: dmg,
                            });
                        }
                    }
                    TileEntity::Chest(slots)
                }
                2 => {
                    let len = r.u8() as usize;
                    let mob = String::from_utf8(r.take(len).to_vec()).unwrap();
                    TileEntity::MobSpawner { mob, delay: 20 }
                }
                3 => {
                    let mut f=worldgen::container::tiles::Furnace::default();
                    for s in &mut f.items { let (id,count,damage)=(r.i16(),r.u8(),r.i16());if id>=0 { *s=Some(worldgen::entity::player::ItemStack::new(id as i32,count as i32,damage as i32)); } }
                    f.burn=r.i16() as i32;f.cook=r.i16() as i32;
                    TileEntity::Furnace(f)
                }
                4 => {
                    let mut items=Box::new([None;9]);
                    for s in items.iter_mut() { let (id,count,damage)=(r.i16(),r.u8(),r.i16());if id>=0 { *s=Some(worldgen::entity::player::ItemStack::new(id as i32,count as i32,damage as i32)); } }
                    TileEntity::Dispenser(items)
                }
                5 => TileEntity::Note(worldgen::world::notes::Note {note:r.u8() as i8,powered:r.u8()!=0}),
                6 => {
                    let (id,meta,facing)=(r.i32() as u8,r.i32(),r.i32());
                    let (progress,last_progress)=(f32::from_bits(r.i32() as u32),f32::from_bits(r.i32() as u32));
                    let mut p=worldgen::world::pistons::Piston::new(id,meta,facing,r.u8()!=0,r.u8()!=0);
                    p.progress=progress;p.last_progress=last_progress;TileEntity::Piston(p)
                }
                t => panic!("bad tile entity tag {t}"),
            };
            tes.push(((x, y, z), te));
        }
        populated += pop as usize;

        let Some(c) = w.chunks.get(&(cx, cz)) else {
            cats.entry("missing chunk")
                .or_default()
                .hit("missing chunk", || format!("({cx},{cz})"));
            continue;
        };
        if c.populated != pop {
            cats.entry("populated flag")
                .or_default()
                .hit("populated", || format!("({cx},{cz}) java={pop}"));
        }
        for i in 0..32768 {
            if blocks[i] != c.blocks[i] {
                let (x, z, y) = (
                    cx * 16 + (i >> 11) as i32,
                    cz * 16 + ((i >> 7) & 15) as i32,
                    (i & 127) as i32,
                );
                cats.entry("block").or_default().hit("block", || {
                    format!(
                        "({x},{y},{z}) java={} ({}) rust={} ({})",
                        blocks[i],
                        BLOCK_NAMES(blocks[i]),
                        c.blocks[i],
                        BLOCK_NAMES(c.blocks[i])
                    )
                });
            }
        }
        for (name, java, rust) in [
            ("metadata", meta, &c.data.0[..]),
            ("skylight", sky, &c.sky_light.0[..]),
            ("blocklight", blk, &c.block_light.0[..]),
        ] {
            for i in 0..16384 {
                if java[i] != rust[i] {
                    let lo = i * 2;
                    let (x, z, y) = (
                        cx * 16 + (lo >> 11) as i32,
                        cz * 16 + ((lo >> 7) & 15) as i32,
                        (lo & 127) as i32,
                    );
                    cats.entry(name).or_default().hit(name, || {
                        format!(
                            "({x},{y}..{},{z}) java={:#04x} rust={:#04x}",
                            y + 1,
                            java[i],
                            rust[i]
                        )
                    });
                }
            }
        }
        if height != &c.height_map[..] {
            cats.entry("heightmap")
                .or_default()
                .hit("heightmap", || format!("chunk ({cx},{cz})"));
        }
        let mut rust_tes: Vec<_> = c
            .tile_entities
            .iter()
            .map(|(&(x, y, z), te)| ((cx * 16 + x, y, cz * 16 + z), te.clone()))
            .collect();
        rust_tes.sort_by_key(|(p, _)| *p);
        let norm = |t: &TileEntity| match t {
            // Oracle.worldBytes exposes getSizeInventory(), not the vanilla
            // constructor's 36-slot backing array (readFromNBT creates 27).
            TileEntity::Chest(slots) => TileEntity::Chest(slots[..27].to_vec().into()),
            TileEntity::MobSpawner { mob, .. } => TileEntity::MobSpawner {
                mob: mob.clone(),
                delay: 20,
            },
            TileEntity::Furnace(f) => { let mut f=f.clone();f.current_fuel=0;TileEntity::Furnace(f) },
            o => o.clone(),
        };
        let rust_tes: Vec<_> = rust_tes.into_iter().map(|(p, t)| (p, norm(&t))).collect();
        if rust_tes != tes {
            cats.entry("tile entities")
                .or_default()
                .hit("tile entities", || {
                    format!("chunk ({cx},{cz}) java={tes:?} rust={rust_tes:?}")
                });
        }
    }

    let nt = r.i32() as usize;
    let mut java_ticks = Vec::new();
    for _ in 0..nt {
        java_ticks.push((r.i32(), r.i32(), r.i32(), r.i32(), r.i64()));
    }
    // Both in processing order (scheduled time, then NextTickListEntry id).
    let rust_ticks: Vec<_> = w
        .tick_order
        .iter()
        .map(|&(t, _, k)| (k.x, k.y, k.z, k.block, t))
        .collect();
    let world_only = WORLD_ONLY.load(std::sync::atomic::Ordering::Relaxed);
    if java_ticks != rust_ticks && !world_only {
        // Entries present with different times, or only on one side (first few).
        let jm: std::collections::HashMap<_, _> = java_ticks
            .iter()
            .enumerate()
            .map(|(i, t)| ((t.0, t.1, t.2, t.3), (i, t.4)))
            .collect();
        let rm: std::collections::HashMap<_, _> = rust_ticks
            .iter()
            .enumerate()
            .map(|(i, t)| ((t.0, t.1, t.2, t.3), (i, t.4)))
            .collect();
        let mut shown = 0;
        for (k, &(ji, jt)) in &jm {
            let r = rm.get(k);
            if r.map(|&(_, rt)| rt) != Some(jt) && shown < 8 {
                shown += 1;
                eprintln!("  tick {k:?}: java #{ji} t={jt}, rust {r:?}");
            }
        }
        for (k, r) in &rm {
            if !jm.contains_key(k) && shown < 12 {
                shown += 1;
                eprintln!("  tick {k:?}: rust only {r:?}");
            }
        }
        let first = java_ticks.iter().zip(&rust_ticks).position(|(a, b)| a != b);
        cats.entry("pending ticks")
            .or_default()
            .hit("pending ticks", || {
                format!(
                    "java {} entries, rust {}; first difference at #{first:?}: java {:?} rust {:?}",
                    java_ticks.len(),
                    rust_ticks.len(),
                    first.map(|i| java_ticks[i]),
                    first.map(|i| rust_ticks[i])
                )
            });
    }
    let java_lq = r.i32() as usize;
    if java_lq != w.light_queue.len() && !world_only {
        cats.entry("light queue")
            .or_default()
            .hit("light queue", || {
                format!("java {java_lq} rust {}", w.light_queue.len())
            });
    }

    println!(
        "{n} chunks ({populated} populated), {} chests/spawners, {} pending ticks",
        w.chunks
            .values()
            .map(|c| c.tile_entities.len())
            .sum::<usize>(),
        nt
    );
    for (name, t) in &cats {
        println!("  {name}: {} mismatches", t.count);
        ok = false;
    }
    verdict(ok)
}

/// Our Java-math emulation vs Java's Math/StrictMath sin/cos over every input
/// WorldGenBigTree can produce, plus Math.pow(x, 2.0) vs x*x.
fn java_math(path: &str) -> ExitCode {
    use worldgen::jmath;
    let data = std::fs::read(path).expect("read math dump");
    let mut r = Reader { b: &data, o: 0 };
    let n = 1usize << 24;
    let arg = |i: usize| (i as f32 / (1u32 << 24) as f32) as f64 * 2.0 * 3.14159;
    let read = |r: &mut Reader| (0..n).map(|_| r.f64()).collect::<Vec<f64>>();
    let (msin, mcos, ssin, scos) = (read(&mut r), read(&mut r), read(&mut r), read(&mut r));
    let mut ok = true;
    let checks: [(&str, &Vec<f64>, fn(f64) -> f64); 4] = [
        ("StrictMath.sin vs fdlibm port", &ssin, jmath::fdlibm_sin),
        ("StrictMath.cos vs fdlibm port", &scos, jmath::fdlibm_cos),
        ("Math.sin vs java_sin", &msin, jmath::java_sin),
        ("Math.cos vs java_cos", &mcos, jmath::java_cos),
    ];
    for (name, java, f) in checks {
        let (mut small, mut large, mut shown) = (0, 0, 0);
        for i in 0..n {
            let a = arg(i);
            let v = f(a);
            if v.to_bits() != java[i].to_bits() {
                if a.abs() < std::f64::consts::FRAC_PI_4 {
                    small += 1
                } else {
                    large += 1
                }
                if shown < 4 {
                    shown += 1;
                    eprintln!("  {name}: x={a:e} java={:e} rust={v:e}", java[i]);
                }
                if std::env::var_os("JMATH_TABLE").is_some() {
                    println!(
                        "    ({:#018x}, {:#018x}), // {name} x={a:e}",
                        a.to_bits(),
                        java[i].to_bits()
                    );
                }
            }
        }
        let libm = (0..n)
            .filter(|&i| {
                let a = arg(i);
                let v = if name.contains("sin") {
                    a.sin()
                } else {
                    a.cos()
                };
                v.to_bits() != java[i].to_bits()
            })
            .count();
        println!(
            "{name}: {small} differ below pi/4, {large} above (plain libm would differ in {libm})"
        );
        ok &= small + large == 0;
    }
    let (mut bad_mul, mut pows) = (0, 0);
    while r.o < data.len() {
        let (x, java) = (r.f64(), r.f64());
        pows += 1;
        if (x * x).to_bits() != java.to_bits() {
            bad_mul += 1;
        }
    }
    println!("Math.pow(x, 2.0) vs x*x: {bad_mul} of {pows} differ");
    verdict(ok && bad_mul == 0)
}

fn redstone_state(w:&World,s:&mut String) {
    use worldgen::world::tile::TileEntity;
    s.push_str(&format!("tiles {}:",w.loaded_tile_entities.len()));
    for id in &w.loaded_tile_entities {
        let o=&w.tile_objects[id];let (x,y,z)=o.position;
        s.push_str(&format!(" {x},{y},{z},{}",o.invalid as i32));
        match w.tile_object(*id).unwrap() {
            TileEntity::Note(n)=>s.push_str(&format!(",M,{},{}",n.note,n.powered as i32)),
            TileEntity::Piston(p)=>s.push_str(&format!(",P,{},{},{},{},{},{},{}",p.block_id,p.block_data,p.facing,p.progress.to_bits() as i32,p.last_progress.to_bits() as i32,p.extending as i32,p.render_head as i32)),
            _=>{}
        }
    }
    s.push('\n');
    for id in &w.loaded_entity_list {
        let e=&w.entities[id];s.push_str(&format!("entity {id}"));
        for v in [e.pos.x,e.pos.y,e.pos.z,e.motion.x,e.motion.y,e.motion.z,e.bounding_box.min_x,e.bounding_box.min_y,e.bounding_box.min_z,e.bounding_box.max_x,e.bounding_box.max_y,e.bounding_box.max_z] {s.push_str(&format!(" {}",v.to_bits() as i64));}
        s.push('\n');
    }
}

fn redstone_events(w:&mut World,s:&mut String) {
    use worldgen::world::notes::BlockEvent;
    for e in std::mem::take(&mut w.block_events) {
        let (position,name,volume,pitch,note)=match e {
            BlockEvent::Note {position,instrument,pitch,note}=>(position,["note.harp","note.bd","note.snare","note.hat","note.bassattack"][instrument as usize],3.0f32,pitch,Some(note)),
            BlockEvent::PistonSound {position,extending,pitch}=>(position,if extending {"tile.piston.out"} else {"tile.piston.in"},0.5f32,pitch,None)
        };
        let (x,y,z)=position;
        s.push_str(&format!("sound {name}"));
        for v in [x as f64+0.5,y as f64+0.5,z as f64+0.5] {s.push_str(&format!(" {}",v.to_bits() as i64));}
        s.push_str(&format!(" {} {}\n",volume.to_bits() as i32,pitch.to_bits() as i32));
        if let Some(note)=note {
            s.push_str("particle note");
            for v in [x as f64+0.5,y as f64+1.2,z as f64+0.5,note as f64/24.0,0.0,0.0] {s.push_str(&format!(" {}",v.to_bits() as i64));}
            s.push('\n');
        }
    }
}

// Sidecars are optional for older non-ticking fixtures; present streams must be
// consumed exactly so an input mismatch cannot silently become a simulation diff.
fn start_replay_identities181(path: &str) {
    fn read(path: String) -> Option<Vec<i32>> {
        match std::fs::read(path) {
            Ok(bytes) => {
                assert_eq!(bytes.len() % 4, 0, "truncated replay identity stream");
                Some(bytes.chunks_exact(4).map(|b| i32::from_be_bytes(b.try_into().unwrap())).collect())
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => None,
            Err(e) => panic!("replay identity sidecar: {e}"),
        }
    }
    if let Some(ids) = read(format!("{path}.identities")) {
        worldgen::world::tick::replay_chunk_identities181(ids);
    }
    if let Some(ids) = read(format!("{path}.spawnidentities")) {
        worldgen::entity::spawning::replay_spawn_identities181(ids);
    }
}
fn finish_replay_identities181() {
    worldgen::world::tick::finish_chunk_identity_replay181();
    worldgen::entity::spawning::finish_spawn_identity_replay181();
}
