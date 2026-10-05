//! Focused original-bytecode replays for the world-blocks port.
use crate::{entity::AxisAlignedBB as BB, nbt::{Compound,Tag}, storage::LevelInfo, World};
use super::{behavior,chunk::{Chunk,idx},RandSel};

fn i(o:&mut Vec<u8>,v:i32){o.extend(v.to_le_bytes());}
fn l(o:&mut Vec<u8>,v:i64){o.extend(v.to_le_bytes());}
fn b(o:&mut Vec<u8>,bb:Option<BB>){o.push(bb.is_some() as u8);if let Some(bb)=bb {for v in [bb.min_x,bb.min_y,bb.min_z,bb.max_x,bb.max_y,bb.max_z]{o.extend(v.to_bits().to_le_bytes());}}}
fn raw(w:&mut World,x:i32,y:i32,z:i32,id:u8,m:i32){let c=w.chunk_mut(x>>4,z>>4);c.blocks[idx(x&15,y,z&15)]=id;c.data.set(x&15,y,z&15,m);}
fn fixture(bright:bool)->World {
    let mut w=World::new(0,300);
    for cx in -3..=3 {for cz in -3..=3 {
        let mut c=Chunk::new(cx,cz,vec![0;32768].into_boxed_slice().try_into().unwrap());
        for x in 0..16 {for z in 0..16 {c.blocks[idx(x,63,z)]=1;if !bright {c.blocks[idx(x,80,z)]=1;}}}
        c.height_map.fill(if bright {64}else{81});if bright {c.sky_light.0.fill(255);}
        c.loaded=true;c.populated=true;w.chunks.insert((cx,cz),Box::new(c));
    }}
    w
}
pub fn bytes(kind:&str)->Vec<u8> {
    let mut o=Vec::new();
    match kind {"plants"=>plants(&mut o),"geometry"=>geometry(&mut o),"drops"=>drops(&mut o),"saves"=>saves(&mut o),"lights"=>lights(&mut o),"chests"=>chests(&mut o),"helpers"=>helpers(&mut o),"fluids"=>fluids(&mut o),"teleporter"=>teleporter(&mut o),"items"=>items(&mut o),
        "registry"=>o.extend_from_slice(include_bytes!("../entity/player/registry.bin")),_=>panic!("unknown worldblocks181 suite {kind}")};
    o
}
fn plants(o:&mut Vec<u8>){
    for id in [104u8,105,106,39,40,18,60] {for config in 0..16 {
        let mut w=fixture(matches!(id,104|105|18|60));w.rand.set_seed(1729+config as i64*31+id as i64);
        for x in 3..=13 {for z in 3..=13 {raw(&mut w,x,63,z,if matches!(id,104|105|60){60}else{1},config&1);}}
        raw(&mut w,8,64,8,id,config);
        if matches!(id,104|105){
            if config&2!=0 {raw(&mut w,7,64,8,id,7);}
            if config&4!=0 {raw(&mut w,8,64,7,id,7);}
            if config&8!=0 {raw(&mut w,9,64,9,id,7);}
        }
        if id==106 {
            for (k,(dx,dz)) in [(0,1),(-1,0),(0,-1),(1,0)].into_iter().enumerate(){
                if config&(1<<k)!=0 {for y in 62..=68 {raw(&mut w,8+dx,y,8+dz,1,0);}}
            }
            raw(&mut w,8,65,8,if config==0{1}else{0},0);
            if config==15 {for x in 4..=7 {raw(&mut w,x,64,4,106,1);}}
        }
        if matches!(id,39|40){
            if config&1!=0 {for x in 4..=7 {raw(&mut w,x,64,4,id,0);}}
            if config&2!=0 {raw(&mut w,9,64,8,1,0);}
        }
        if id==18&&config&1!=0 {raw(&mut w,8,64,9,17,0);}
        if id==60 {raw(&mut w,8,63,8,1,0);if config<4 {raw(&mut w,8,65,8,[0,59,104,105][config as usize],0);}if config&4!=0 {raw(&mut w,10,64,8,9,0);}}
        for _ in 0..128 {
            if w.get_block_id(8,64,8)==id {behavior::update_tick(&mut w,id,8,64,8,RandSel::World);}
            l(o,w.rand.state());
            for x in 3..=13 {for z in 3..=13 {for y in 62..=68 {o.push(w.get_block_id(x,y,z));o.push(w.get_block_metadata(x,y,z) as u8);}}}
        }
    }}
}
fn geometry(o:&mut Vec<u8>){
    let mut w=fixture(false);
    for id in 97..=109 {for meta in 0..16 {for mask in 0..16 {for kind in 0..3 {
        raw(&mut w,8,64,8,id,meta);
        for (k,(dx,dz)) in [(0,1),(-1,0),(0,-1),(1,0)].into_iter().enumerate(){raw(&mut w,8+dx,64,8+dz,if mask&(1<<k)==0{0}else{match kind{0=>1,1=>20,_=>id}},0);}
        raw(&mut w,8,65,8,if mask==0{1}else{0},0);
        let boxes=if matches!(id,101|102) {
            let boxes=w.pane_boxes(id,8,64,8);
            for bb in &boxes {w.block_bounds[id as usize]=*bb;}
            boxes.into_iter().map(|bb|bb.translated(8.0,64.0,8.0)).collect()
        } else {w.get_colliding_bounding_boxes(None,BB::new(8.0,64.0,8.0,9.0,66.0,9.0))};
        // The query includes the block above. Java calls the target block directly.
        let boxes:Vec<_>=boxes.into_iter().filter(|bb|bb.min_y<65.0).collect();
        i(o,boxes.len() as i32);for bb in boxes {b(o,Some(bb));}
        b(o,w.block_collision_box(id,8,64,8));w.set_block_bounds_based_on_state(id,8,64,8);b(o,Some(w.block_bounds[id as usize]));
    }}}}
}
fn drops(o:&mut Vec<u8>){
    let mut w=fixture(false);w.entity_seed_source=crate::entity::EntitySeedSource::Deterministic(100);w.math_random.set_seed(12345);
    for id in [53,67,97,98,99,100,101,102,103,104,105,106,107,108,109] {for meta in 0..16 {for seed in 0..32 {
        w.loaded_entity_list.clear();w.rand.set_seed(seed);behavior::drop_block_with_chance(&mut w,id,8,64,8,meta,0.75);
        l(o,w.rand.state());i(o,w.loaded_entity_list.len() as i32);
        for eid in &w.loaded_entity_list {let e=&w.entities[eid];if let crate::entity::EntityKind::Item{item:stack,..}=&e.kind {
            i(o,stack.id as i32);i(o,stack.count as i32);i(o,stack.damage as i32);for v in [e.pos.x,e.pos.y,e.pos.z]{o.extend(v.to_bits().to_le_bytes());}
        } else {panic!("non-item drop");}}
    }}}
}
fn saves(o:&mut Vec<u8>){
    for game in -1..=2 {for features in -1..=1 {for player in 0..2 {
        let mut n=Compound::new();n.put("RandomSeed",Tag::Long(-4172144997902289642));n.put("GameType",Tag::Int(game));
        if features>=0 {n.put("MapFeatures",Tag::Byte((features!=0) as i8));}
        for (key,v) in [("SpawnX",-22),("SpawnY",64),("SpawnZ",900),("version",19132),("rainTime",71),("thunderTime",17)]{n.put(key,Tag::Int(v));}
        n.put("Time",Tag::Long(23999));n.put("SizeOnDisk",Tag::Long(123456789012));n.put("raining",Tag::Byte(1));
        if player!=0 {let mut p=Compound::new();p.put("Dimension",Tag::Int(-1));n.put("Player",Tag::Compound(p));}
        let info=LevelInfo::from_nbt(&n).clone();let encoded=crate::nbt::write_gzip(&info.to_nbt()).unwrap();let n=crate::nbt::read_gzip(&encoded).unwrap();let info=LevelInfo::from_nbt(&n);
        l(o,info.seed);i(o,info.game_type);o.push(info.map_features as u8);i(o,info.spawn.0);i(o,info.spawn.1);i(o,info.spawn.2);l(o,info.time);l(o,info.size_on_disk);i(o,info.dimension());
        i(o,n.int("version"));i(o,n.int("rainTime"));i(o,n.int("thunderTime"));o.push(n.boolean("raining") as u8);o.push(n.boolean("thundering") as u8);o.push(n.has("GameType") as u8);o.push(n.has("MapFeatures") as u8);
    }}}
}
fn lights(o:&mut Vec<u8>){
    use super::{chunk::LightKind,Dimension};
    let mut w=fixture(false);
    for c in w.chunks.values_mut(){c.init_height_and_sky();}
    let mut r=crate::JavaRandom::new(1007);let ids=[0,1,18,20,89,98,101,102,103,107,109];
    for t in 0..80 {
        let x=r.next_int(24)-4;let y=64+r.next_int(22);let z=r.next_int(24)-4;let id=ids[r.next_int(ids.len() as i32) as usize];let m=r.next_int(16);
        o.push(w.set_block_and_metadata(x,y,z,id,m) as u8);
        if t%8==0 {w.recheck_skylight181(x>>4,z>>4);}
        for bx in -8..=24 {for bz in -8..=24 {for by in 59..=88 {
            o.push(w.get_block_id(bx,by,bz));o.push(w.get_block_metadata(bx,by,bz) as u8);
            o.push(w.get_saved_light_value(LightKind::Sky,bx,by,bz) as u8);o.push(w.get_saved_light_value(LightKind::Block,bx,by,bz) as u8);
        }}}
        for cx in -1..=1 {for cz in -1..=1 {let c=w.chunk(cx,cz);o.extend(c.height_map);o.extend(c.skylight_occlusion.map(|v|v as u8));}}
        i(o,w.packed_light181(x,y,z,t%16));
    }
    for dimension in [Dimension::Overworld,Dimension::Nether]{for f in dimension.light_brightness_table(){o.extend(f.to_bits().to_le_bytes());}}
}

fn chests(o:&mut Vec<u8>) {
    for pair in 0..5 {for mask in 0..16 {for meta in 0..6 {for yaw in [-405f32,-90.0,0.0,90.0,180.0,270.0,405.0] {
        let mut w=fixture(true);
        raw(&mut w,8,64,8,54,0);
        let (dx,dz)=[(0,0),(0,-1),(0,1),(-1,0),(1,0)][pair];
        if pair!=0{raw(&mut w,8+dx,64,8+dz,54,meta);}
        for (k,(sx,sz)) in [(0,-1),(0,1),(-1,0),(1,0)].into_iter().enumerate(){
            if w.get_block_id(8+sx,64,8+sz)!=54{raw(&mut w,8+sx,64,8+sz,if mask&(1<<k)!=0{1}else{0},0);}
        }
        w.orient_chest181(8,64,8);i(o,w.get_block_metadata(8,64,8));
        w.chest_placed181(8,64,8,yaw);i(o,w.get_block_metadata(8,64,8));i(o,w.get_block_metadata(8+dx,64,8+dz));
    }}}}
    for pair in 0..5 {
        let mut w=fixture(true);w.rand.set_seed(4242);
        w.set_block(8,64,8,54);
        let (dx,dz)=[(0,0),(0,-1),(0,1),(-1,0),(1,0)][pair];
        if pair!=0{w.set_block(8+dx,64,8+dz,54);}
        let id=w.chest_id181(8,64,8).unwrap();
        for t in 0..160 {
            if [0,50,75,110].contains(&t){w.chest_use181(8,64,8,true);}
            if [20,30,65,95,140].contains(&t){w.chest_use181(8,64,8,false);}
            if t==35{w.play_note_at(8,64,8,1,0);}
            if t==100 {w.chest_neighbor181(8,64,8);}
            w.tick_chest181(id);
            let a=&w.chest_animations[&id];o.push(a.adjacency_checked as u8);
            for adjacent in a.adjacent {o.push(adjacent.is_some() as u8);}
            o.extend(a.lid.to_bits().to_le_bytes());o.extend(a.previous_lid.to_bits().to_le_bytes());i(o,a.users);i(o,a.ticks);l(o,w.rand.state());
            let sounds:Vec<_>=w.drain_events().into_iter().filter(|e|matches!(e,super::events::WorldEvent::Sound{..})).collect();
            i(o,sounds.len() as i32);
            for e in sounds {if let super::events::WorldEvent::Sound{name,position,volume,pitch}=e{
                o.push((name.as_deref()==Some("random.door_open")) as u8);
                for d in [position.x,position.y,position.z]{o.extend(d.to_bits().to_le_bytes());}
                o.extend(volume.to_bits().to_le_bytes());o.extend(pitch.to_bits().to_le_bytes());
            }}
        }
    }
}

fn helpers(o:&mut Vec<u8>) {
    for v in [f64::NAN,f64::NEG_INFINITY,f64::INFINITY,-1e30,1e30,-9223372036854775808.0,9223372036854775808.0,-2147483649.0,2147483648.0,-123.5,-1.0,-0.1,-0.0,0.0,0.1,1.0,123.5] {
        l(o,crate::math::floor_long(v));i(o,crate::math::floor_double(v));i(o,crate::math::floor_float(v as f32));
    }
    for v in [i32::MIN,i32::MIN+1,-100,-1,0,1,100,i32::MAX]{i(o,crate::math::abs_int(v));}
    let mut r=crate::JavaRandom::new(6543);
    for _ in 0..1000 {for (min,max) in [(-999,2000),(0,1),(1,1),(5,-5),(-7,-6),(2147483600,2147483647)]{
        i(o,crate::math::next_int_inclusive(&mut r,min,max));l(o,r.state());
    }}
    for bits in 4..=8 {
        let mut a=super::chunk::NibbleArray181::new(256<<bits,bits);
        for x in 0..16 {for z in 0..16 {for y in 0..1<<bits {a.set(x,y,z,x*100+z*10+y-200);}}}
        for x in 0..16 {for z in 0..16 {for y in 0..1<<bits {o.push(a.get(x,y,z) as u8);}}}
        o.extend(&a.data);
        let a=super::chunk::NibbleArray181::from_data(a.data,bits);i(o,a.get(15,0,15));
    }
    let mut w=fixture(true);
    for id in [0,1,8,18,30,78,101,102,104,106] {for y in [0,1,62,64,127] {
        let c=w.chunk_mut(0,0);for yy in 0..128{c.blocks[idx(8,yy,8)]=0;}c.precipitation_height[136]=-999;
        raw(&mut w,8,y,8,id,0);i(o,w.precipitation_height181(8,8));
        w.set_block(8,y,8,0);i(o,w.precipitation_height181(8,8));
        w.set_block(8,127,8,1);i(o,w.precipitation_height181(8,8));
        w.set_block(8,127,8,1);i(o,w.precipitation_height181(8,8));
    }}
}

fn fluids(o:&mut Vec<u8>) {
    let mut w=fixture(true);let mut r=crate::JavaRandom::new(567);
    for id in [8,9,10,11] {for m in 0..16 {for above in [0,1,8,79] {
        raw(&mut w,8,64,8,id,m);raw(&mut w,8,65,8,above,0);
        for _ in 0..200 {
            w.block_display_tick(id,8,64,8,&mut r);l(o,r.state());
            let events=w.drain_events();i(o,events.len() as i32);
            for e in events {
                let (kind,name,p,values)=match e {
                    super::events::WorldEvent::Sound{name,position,volume,pitch}=>(0,name.unwrap(),position,vec![volume as f64,pitch as f64]),
                    super::events::WorldEvent::Particle{name,position,velocity}=>(1,name,position,vec![velocity.x,velocity.y,velocity.z]),
                    _=>panic!("unexpected display event")};
                o.push(kind);i(o,name.len() as i32);o.extend(name.as_bytes());
                for d in [p.x,p.y,p.z]{o.extend(d.to_bits().to_le_bytes());}
                for d in values{ o.extend(d.to_bits().to_le_bytes());}
            }
        }
        i(o,w.fluid_packed_light181(8,64,8));
        for side in 0..6{let material=crate::block::material_of(id);o.push(w.fluid_side_solid181(material,8,65,8,side) as u8);}
    }}}
}

fn teleporter(o:&mut Vec<u8>) {
    for config in 0..6 {
        let mut w=fixture(true);
        for cx in -9..=9 {for cz in -9..=9 {
            let mut c=Chunk::new(cx,cz,vec![0;32768].into_boxed_slice().try_into().unwrap());
            for x in 0..16{for z in 0..16{c.blocks[idx(x,63,z)]=if config==2{0}else if config==3{9}else{1};}}
            c.sky_light.0.fill(255);c.height_map.fill(64);c.loaded=true;c.populated=true;w.chunks.insert((cx,cz),Box::new(c));
        }}
        w.rand.set_seed(19);
        if config>=4 {
            for y in 64..=66 {for v in 0..2 {
                raw(&mut w,if config==4{5+v}else{5},y,if config==4{6}else{6+v},90,0);
            }}
        }
        let pos=(8.25,if config==1{120.0}else{70.0},8.75);let mut r=crate::JavaRandom::new(77+config);
        let p=w.teleport_position(pos,&mut r).unwrap();for d in [p.0,p.1,p.2]{o.extend(d.to_bits().to_le_bytes());}
        l(o,r.state());l(o,w.rand.state());
        for x in -12..=28 {for z in -12..=28 {for y in 35..=90 {o.push(w.get_block_id(x,y,z));o.push(w.get_block_metadata(x,y,z) as u8);}}}
    }
}

fn items(o:&mut Vec<u8>) {
    use crate::entity::{Entity,player::ItemStack};
    for id in [295,361,362] {for side in 0..6 {for soil in [1,60] {for above in [0,1] {
        let mut w=fixture(true);let mut p=Entity::player(&mut w,"oracle");
        raw(&mut w,8,64,8,soil,0);raw(&mut w,8,65,8,above,0);
        p.player_state_mut().unwrap().inventory.main[0]=Some(ItemStack::new(id,4,0));
        o.push(p.use_on_block(&mut w,8,64,8,side) as u8);i(o,p.held_item().unwrap().count);o.push(w.get_block_id(8,65,8));i(o,w.get_block_metadata(8,65,8));
    }}}}
    for meta in 0..16 {for yaw in [-450f32,-90.0,0.0,90.0,180.0,270.0,450.0] {
        let mut w=fixture(true);let mut p=Entity::player(&mut w,"oracle");p.yaw=yaw;raw(&mut w,8,64,8,107,meta);
        o.push(p.activate_block(&mut w,8,64,8) as u8);i(o,w.get_block_metadata(8,64,8));
        p.block_placed(&mut w,107,8,64,8,1);i(o,w.get_block_metadata(8,64,8));
    }}
    for id in [59,104,105] {for meta in 0..8 {
        let mut w=fixture(true);let mut p=Entity::player(&mut w,"oracle");raw(&mut w,8,63,8,60,0);raw(&mut w,8,64,8,id,meta);
        p.player_state_mut().unwrap().inventory.main[0]=Some(ItemStack::new(351,4,15));
        o.push(p.use_on_block(&mut w,8,64,8,1) as u8);i(o,p.held_item().unwrap().count);i(o,w.get_block_metadata(8,64,8));
    }}
    for id in [0,1,97,109,260,351,359,360,361,362,367,2256,2257,2258,12345,30000] {for count in [-128,-1,0,1,127] {
        let mut n=Compound::new();n.put("id",Tag::Short(id));n.put("Count",Tag::Byte(count));n.put("Damage",Tag::Short(-123));
        let stack=ItemStack::try_from_nbt181(&n);o.push(stack.is_some() as u8);if let Some(s)=stack{i(o,s.id);i(o,s.count);i(o,s.damage);}
    }}
    for id in [18,106] {for meta in 0..16 {
        let mut w=fixture(true);let mut p=Entity::player(&mut w,"oracle");w.rand.set_seed(42);w.math_random.set_seed(123);w.entity_seed_source=crate::entity::EntitySeedSource::Deterministic(100);
        p.player_state_mut().unwrap().inventory.main[0]=Some(ItemStack::new(359,1,0));
        p.harvest_block(&mut w,id,8,64,8,meta);l(o,w.rand.state());i(o,w.loaded_entity_list.len() as i32);
        for eid in &w.loaded_entity_list {let e=&w.entities[eid];if let crate::entity::EntityKind::Item{item,..}=&e.kind{i(o,item.id as i32);i(o,item.count as i32);i(o,item.damage as i32);for d in [e.pos.x,e.pos.y,e.pos.z]{o.extend(d.to_bits().to_le_bytes());}}}
    }}
}
