use worldgen::{container::{ContainerCommand as Command,ContainerKind},entity::Entity};
fn i(o:&mut Vec<u8>,v:i32){o.extend(v.to_le_bytes());}
fn main(){
    let args:Vec<_>=std::env::args().collect();let mut out=Vec::new();
    for layout in 0..5 {
        let mut w=worldgen::entity::verify181::world();let mut p=Entity::player(&mut w,"Player");p.set_location_and_angles(8.5,64.0,8.5,0.0,0.0);let id=p.entity_id;w.entities.insert(id,p);
        let mut positions=vec![(8,64,8)];if layout!=0 {positions.push((8+[0,-1,1,0,0][layout],64,8+[0,0,0,-1,1][layout]));}
        for &(x,y,z) in &positions {let c=w.chunks.get_mut(&(x>>4,z>>4)).unwrap();c.blocks[worldgen::world::chunk::idx(x&15,y,z&15)]=54;c.data.set(x&15,y,z&15,3);w.set_block_tile_entity(x,y,z,worldgen::world::tile::TileEntity::for_block(54));}
        w.drain_events();
        for t in 0..240 {
            let phase=t%24;
            if [0,10,20].contains(&phase){w.container_command(id,Command::OpenBlock{block:54,position:(8,64,8)});}
            if [7,14].contains(&phase) {if let Some(c)=w.container_sessions[&id].active.as_ref(){let window=c.window;w.container_command(id,Command::Close{window});}}
            if phase==23 {w.container_command(id,Command::OpenInventory);}
            for &(x,y,z) in &positions {let tile=w.chest_id181(x,y,z).unwrap();w.tick_chest181(tile);let a=&w.chest_animations[&tile];i(&mut out,a.users);out.extend(a.lid.to_le_bytes());out.extend(a.previous_lid.to_le_bytes());}
            let c=w.container_sessions[&id].active.as_ref();i(&mut out,c.map_or(0,|c|c.slots.len()as i32));if let Some(c)=c {assert!(matches!(c.kind,ContainerKind::Chest{..}|ContainerKind::Player));for slot in &c.slots {i(&mut out,slot.x);i(&mut out,slot.y);i(&mut out,slot.stack_limit());}}
            out.extend(w.rand.state().to_le_bytes());worldgen::world::events::write_events(&w.drain_events(),&mut out);
        }
    }
    std::fs::write(&args[1],&out).unwrap();let java=std::fs::read(&args[2]).unwrap();if out!=java{let at=out.iter().zip(&java).position(|(a,b)|a!=b).unwrap_or(out.len().min(java.len()));eprintln!("CHEST CONTAINER HOOKS DIFFER at {at}, lengths {}/{}",java.len(),out.len());std::process::exit(1);}println!("CHEST CONTAINER HOOKS EXACT: 1200 ticks, 5 chest layouts, replacement and close ({} bytes)",out.len());
}
