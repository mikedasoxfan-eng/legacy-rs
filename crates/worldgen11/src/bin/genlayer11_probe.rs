//! Mirrors tools/oracle11/probes/LayerProbe11.java's primitive big-endian stream.
use std::io::{BufWriter, Write};
fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.get(3).is_some_and(|s|s=="spawn") { spawn(&args); return; }
    if args.get(3).is_some_and(|s|s=="worldtype") { world_type(&args); return; }
    if args.get(3).is_some_and(|s|s=="depthfog") { depth_fog(&args); return; }
    if args.get(3).is_some_and(|s|s=="flat") { flat(&args); return; }
    if args.get(3).is_some_and(|s|s=="flatworld") { flat_world(&args); return; }
    let layers = worldgen::genlayer::stack(args[1].parse().unwrap());
    let mut out = BufWriter::new(std::fs::File::create(&args[2]).unwrap());
    for [x,z,w,h] in [[0,0,16,16],[-19,31,17,13],[-1,-1,1,1],[512,-1024,32,16],[-33,-41,9,7],[-512,-512,64,64]] {
        for layer in &layers {
            for value in layer.get(x,z,w,h).into_iter().take((w*h) as usize) { out.write_all(&value.to_be_bytes()).unwrap(); }
        }
    }
}
fn spawn(args: &[String]) {
    use worldgen::{World,world::Dimension,storage::WorldType};
    fn run(o:&mut Vec<u8>,seed:i64,dim:Dimension,ty:WorldType,shape:i32,start:Option<(i32,i32,i32)>) {
        let mut w=World::with_dimension(seed,seed,dim);w.info.map_features=false;w.set_world_type(ty);w.server=Some(Default::default());
        w.rand.set_seed(seed);let mut trace=Vec::new();let mut calls=0i32;
        let mut surface=|w:&mut World,x:i32,z:i32| {
            let block=if shape==0 {2} else if shape==1 {if calls<6 {0} else {2}} else if shape==3 {1} else {0};
            calls+=1;trace.extend(x.to_be_bytes());trace.extend(z.to_be_bytes());trace.extend((block as i32).to_be_bytes());
            trace.push(w.server.as_ref().unwrap().world_override as u8);trace.extend(w.rand.state().to_le_bytes());block
        };
        if let Some(p)=start {w.spawn=p;w.info.spawn=p;w.set_spawn_location_with_surface(&mut surface);}else{w.find_initial_spawn_with_surface(&mut surface);}
        o.extend(calls.to_be_bytes());o.extend((trace.len() as i32).to_be_bytes());o.extend(trace);
        assert_eq!(w.info.spawn,w.spawn);for p in [w.info.spawn.0,w.info.spawn.1,w.info.spawn.2]{o.extend(p.to_be_bytes());}
        o.push(w.server.as_ref().unwrap().world_override as u8);o.extend(w.rand.state().to_le_bytes());
        worldgen::world::events::write_events(&w.drain_events(),o);assert!(w.chunks.is_empty());
    }
    let seed=args[1].parse().unwrap();let mut o=Vec::new();
    for dim in [Dimension::Overworld,Dimension::Nether,Dimension::End]{for ty in [WorldType::Default,WorldType::Flat]{for shape in 0..4 {run(&mut o,seed,dim,ty,shape,None);}}}
    for dim in [Dimension::Overworld,Dimension::Nether,Dimension::End]{for ty in [WorldType::Default,WorldType::Flat]{for shape in 0..4 {for start in [(-17,-1,31),(5,0,-9),(0,4,0),(i32::MAX,128,i32::MIN)] {run(&mut o,seed,dim,ty,shape,Some(start));}}}}
    for dim in [Dimension::Overworld,Dimension::Nether,Dimension::End]{for ty in [WorldType::Default,WorldType::Flat]{
        use worldgen::{storage::{Storage,LevelInfo},nbt::{Compound,Tag},world::chunk::Chunk,entity::player::food::WorldSettings};
        let dir=std::path::Path::new(&args[4]).join(format!("saved-{}-{}",dim.id(),ty.saved_name()));
        let storage=Storage::open_with_players(&dir,false).unwrap();let mut saved=LevelInfo::from_settings(WorldSettings::with_hardcore(seed,1,false,true).with_world_type(ty),"saved");saved.spawn=(-17,-1,31);
        let mut p=Compound::new();p.put("Dimension",Tag::Int(dim.id()));saved.player=Some(p);storage.save_level(&saved).unwrap();
        let mut routed=storage.for_dimension(dim).unwrap();
        for cx in -10..=6 {for cz in -7..=9 {let mut c=Chunk::new(cx,cz,vec![0;32768].into_boxed_slice());c.populated=true;routed.save_chunk(&mut c,0).unwrap();}}
        routed.flush_region_cache();
        let requested=WorldSettings::new(seed^17,0,true).with_world_type(if ty==WorldType::Default {WorldType::Flat}else{WorldType::Default});
        let (mut w,is_new)=World::open_with_settings(Some(&dir),seed^17,seed,false,Some("saved"),Some(requested),&mut |_|{}).unwrap();
        assert_eq!(w.chunks.len(),289);assert!(w.chunks.values().all(|c|c.populated&&c.blocks.iter().all(|b|*b==0)));
        o.push(is_new as u8);o.extend(w.dimension.id().to_be_bytes());o.extend(w.info.seed.to_be_bytes());o.extend(w.info.game_type.to_be_bytes());o.push(w.info.map_features as u8);o.push(w.info.hardcore as u8);o.push((w.info.world_type==WorldType::Flat) as u8);
        for n in [w.spawn.0,w.spawn.1,w.spawn.2]{o.extend(n.to_be_bytes());}o.extend(0i32.to_be_bytes());
        w.drain_events();w.rand.set_seed(seed);let mut trace=Vec::new();let mut calls=0i32;
        w.set_spawn_location_with_surface(|w,x,z|{let b=if calls<6 {0u8}else{2};calls+=1;trace.extend(x.to_be_bytes());trace.extend(z.to_be_bytes());trace.extend((b as i32).to_be_bytes());trace.push(0);trace.extend(w.rand.state().to_le_bytes());b});
        o.extend(calls.to_be_bytes());o.extend((trace.len() as i32).to_be_bytes());o.extend(trace);for n in [w.info.spawn.0,w.info.spawn.1,w.info.spawn.2]{o.extend(n.to_be_bytes());}o.push(0);o.extend(w.rand.state().to_le_bytes());worldgen::world::events::write_events(&w.drain_events(),&mut o);
    }}
    for dim in [Dimension::Overworld,Dimension::Nether,Dimension::End]{for ty in [WorldType::Default,WorldType::Flat]{for shape in 0..4 {
        let mut w=World::with_dimension(seed,seed,dim);w.set_world_type(ty);
        for cx in -3..=3 {for cz in -3..=3 {
            let mut blocks=vec![0u8;32768];for x in 0..16 {for z in 0..16 {for y in 0..128 {
                blocks[(x<<11)|(z<<7)|y]=if shape==0&&y==3||shape==1&&y==63 {2} else if shape==2&&y==63||shape==3 {1} else if shape==2&&y>63&&y<=67 {9} else {0};
            }}}
            let mut c=worldgen::world::chunk::Chunk::new(cx,cz,blocks.into_boxed_slice());c.populated=true;w.chunks.insert((cx,cz),Box::new(c));
        }}
        w.rand.set_seed(seed);for [x,z] in [[0,0],[-17,31],[47,-48],[-48,47]]{o.extend((w.get_first_uncovered_block(x,z) as i32).to_be_bytes());}
        o.extend(w.rand.state().to_le_bytes());worldgen::world::events::write_events(&w.drain_events(),&mut o);
    }}}
    std::fs::write(&args[2],o).unwrap();
}
fn world_type(args: &[String]) {
    use worldgen::{World,world::Dimension,storage::{Storage,WorldType,LevelInfo},nbt::{Compound,Tag}};
    fn s(o:&mut Vec<u8>,s:&str){o.extend((s.len() as i32).to_be_bytes());o.extend(s.as_bytes());}
    fn info(o:&mut Vec<u8>,w:Option<LevelInfo>){o.push(w.is_some() as u8);if let Some(w)=w{s(o,w.world_type.saved_name());o.push(w.map_features as u8);o.extend(w.version.to_be_bytes());o.extend(w.dimension().to_be_bytes());o.extend(w.seed.to_be_bytes());o.extend(w.game_type.to_be_bytes());o.push(w.hardcore as u8);s(o,&w.name);}}
    fn corrupt(p:&std::path::Path){let mut d=Compound::new();d.put("generatorName",Tag::Int(1));let mut r=Compound::new();r.put("Data",Tag::Compound(d));std::fs::write(p,worldgen::nbt::write_gzip(&r).unwrap()).unwrap();}
    let seed=args[1].parse().unwrap();let mut o=Vec::new();
    for name in ["DEFAULT","FLAT","default","flat",""," DEFAULT","FLAT ","unknown"] {
        let t=WorldType::parse_saved_name(name);o.push(t.is_some() as u8);if let Some(t)=t{s(&mut o,t.translation_key());}
        let mut d=Compound::new();d.put("generatorName",Tag::String(name.into()));info(&mut o,Some(LevelInfo::from_nbt(&d)));
    }
    info(&mut o,Some(LevelInfo::from_nbt(&Compound::new())));
    let mut bad=Compound::new();bad.put("generatorName",Tag::Int(1));o.push(LevelInfo::try_from_nbt(&bad).is_err() as u8);
    for dim in [Dimension::Overworld,Dimension::Nether,Dimension::End] {for t in [WorldType::Default,WorldType::Flat] {
        let mut w=World::with_dimension(seed,seed,dim);w.set_world_type(t);
        for b in [dim.is_hell_world(),dim.is_alternate_dimension(),dim.has_no_sky(),dim.can_respawn_here(),dim.is_surface_world()]{o.push(b as u8);}
        o.extend(dim.cloud_height().to_be_bytes());o.extend(w.provider_spawn_height().to_be_bytes());o.push(w.has_depth_fog() as u8);
        o.extend(w.void_fog_y_factor().to_be_bytes());o.extend(w.horizon().to_be_bytes());for v in dim.light_brightness_table(){o.extend(v.to_be_bytes());}
    }}
    for t in [WorldType::Default,WorldType::Flat] {
        let settings=worldgen::entity::player::food::WorldSettings::with_hardcore(seed,1,false,true).with_world_type(t);
        let fresh=LevelInfo::from_settings(settings,"fresh");info(&mut o,Some(fresh.clone()));
        let dir=std::path::Path::new(&args[4]).join(format!("fresh-{}",t.saved_name()));
        let mut w=World::new(seed,seed);w.info=fresh;w.storage=Some(Storage::open_with_players(&dir,false).unwrap());w.save_level();
        info(&mut o,Some(w.info.clone()));info(&mut o,w.storage.as_ref().unwrap().load_level());
    }
    let dir=std::path::Path::new(&args[4]).join("save");let h=Storage::open_with_players(&dir,false).unwrap();
    let mut f=LevelInfo::new(seed,"save");f.world_type=WorldType::Flat;f.game_type=1;f.map_features=false;f.hardcore=true;
    h.save_level(&f).unwrap();info(&mut o,h.load_level());let n=LevelInfo::new(seed,"save");h.save_level(&n).unwrap();info(&mut o,h.load_level());
    corrupt(&dir.join("level.dat"));info(&mut o,h.load_level());corrupt(&dir.join("level.dat_old"));info(&mut o,h.load_level());
    let dir=std::path::Path::new(&args[4]).join("routing");let h=Storage::open_with_players(&dir,false).unwrap();
    for dim in [Dimension::Overworld,Dimension::Nether,Dimension::End] {
        let _routed=h.for_dimension(dim).unwrap();
        o.push(dir.join("DIM-1").is_dir() as u8);o.push(dir.join("DIM1").is_dir() as u8);
    }
    std::fs::write(&args[2],o).unwrap();
}
fn depth_fog(args: &[String]) {
    use worldgen::{World,world::{Dimension,chunk::Chunk},storage::WorldType};
    let seed=args[1].parse().unwrap();
    let mut out=Vec::new();
    for dim in [Dimension::Overworld,Dimension::Nether,Dimension::End] {
        for ty in [WorldType::Default,WorldType::Flat] { for shape in 0..3 {
            let mut w=World::with_dimension(seed,seed,dim);w.info.map_features=false;w.set_world_type(ty);
            for cx in -3..=3 { for cz in -3..=3 {
                let mut blocks=vec![0u8;32768];
                for x in 0..16 { for z in 0..16 { for y in 0..128 {
                    blocks[(x<<11)|(z<<7)|y]=if shape==1||shape==2&&(x+z+y)%2==0 {1} else {0};
                }}}
                let mut c=Chunk::new(cx,cz,blocks.into_boxed_slice());c.populated=true;w.chunks.insert((cx,cz),Box::new(c));
            }}
            w.rand.set_seed(seed);out.push(w.has_depth_fog() as u8);
            for [x,y,z] in [[0,64,0],[0,4,0],[0,1,0],[-9,10,7],[5,0,-5],[0,-3,0],[0,127,0],[0,128,0]] {
                for _ in 0..2 {
                    w.random_display_updates(x,y,z);
                    worldgen::world::events::write_events(&w.drain_events(),&mut out);
                    out.extend(w.rand.state().to_le_bytes());
                }
            }
        }}
    }
    std::fs::write(&args[2],out).unwrap();
}
fn flat(args: &[String]) {
    let seed=args[1].parse().unwrap();
    let mut source=worldgen::biome::BiomeSource::flat(seed);
    let mut r=worldgen::JavaRandom::new(seed);
    let mut o=BufWriter::new(std::fs::File::create(&args[2]).unwrap());
    for [x,z,w,h] in [[0,0,16,16],[-19,31,17,13],[i32::MAX,i32::MIN,1,1]] {
        o.write_all(&(source.biome_at(x,z) as i32).to_be_bytes()).unwrap();
        for values in [source.generation_biomes(x,z,w,h),source.block_biomes(x,z,w,h)] {
            for b in values {o.write_all(&(b as i32).to_be_bytes()).unwrap();}
        }
        let mut t=Vec::new();source.temperatures(&mut t,x,z,w,h);
        for values in [t,source.rainfalls(x,z,w,h)] { for v in values {o.write_all(&(v as f32).to_bits().to_be_bytes()).unwrap();} }
        o.write_all(&source.temperature_at_height(x,64,z).to_bits().to_be_bytes()).unwrap();
        o.write_all(&(source.raw_rainfall(x,z) as f32).to_bits().to_be_bytes()).unwrap();
    }
    for radius in [0,4,256] { for allowed in [&[worldgen::Biome::Desert][..],&[worldgen::Biome::Plains][..]] {
        o.write_all(&[source.viable(-17,31,radius,allowed) as u8]).unwrap();
        let p=source.find_position(-17,31,radius,allowed,&mut r);
        o.write_all(&[p.is_some() as u8]).unwrap();
        if let Some((x,z))=p {o.write_all(&x.to_be_bytes()).unwrap();o.write_all(&z.to_be_bytes()).unwrap();}
        o.write_all(&r.next_long().to_be_bytes()).unwrap();
    } }
    let mut reused=vec![worldgen::Biome::Desert;7];source.biomes(&mut reused,0,0,2,2);
    for b in reused {o.write_all(&(b as i32).to_be_bytes()).unwrap();}
}
fn flat_world(args: &[String]) {
    let seed=args[1].parse().unwrap();
    let mut w=worldgen::World::new(seed,300);
    w.info.map_features=false;
    w.set_world_type(worldgen::storage::WorldType::Flat);
    let mut o=BufWriter::new(std::fs::File::create(&args[2]).unwrap());
    for [x,z] in [[0,0],[-19,31],[512,-1024]] {
        w.provide_chunk(x,z);
        let c=w.chunk(x,z);
        for bytes in [&c.blocks[..],&c.data.0[..],&c.sky_light.0[..],&c.block_light.0[..],&c.height_map[..]] { o.write_all(bytes).unwrap(); }
        o.write_all(&c.lowest_block_height.to_be_bytes()).unwrap();
        worldgen::flat::populate(&mut w,x,z);o.write_all(&w.prand().next_long().to_be_bytes()).unwrap();
    }
}
