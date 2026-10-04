use worldgen::{container::{ContainerCommand,ContainerKind,ItemStack},world::{chunk::{idx,Chunk},tile::TileEntity},entity::Entity,World};
fn fixture()->(World,i32) {
    let mut w=World::new(0,300);w.set_entity_determinism(100,200,0);
    for x in -1..=1 {for z in -1..=1 {let mut c=Chunk::new(x,z,vec![0;32768].into_boxed_slice().try_into().unwrap());c.loaded=true;c.populated=true;for bx in 0..16{for bz in 0..16{c.blocks[idx(bx,63,bz)]=1;}}w.chunks.insert((x,z),Box::new(c));}}
    let mut p=Entity::player(&mut w,"Player");p.set_location_and_angles(0.5,64.0,0.5,0.0,0.0);let id=p.entity_id;w.entity_joined_world(p);(w,id)
}
#[test]
fn double_chest_order_and_window_messages_update_real_inventory() {
    let (mut w,id)=fixture();w.set_block_with_notify(1,64,1,54);w.set_block_with_notify(2,64,1,54);
    if let Some(TileEntity::Chest(slots))=w.get_block_tile_entity(1,64,1){slots[0]=Some(ItemStack::new(264,32,0).into());}
    let snapshot=w.container_command(id,ContainerCommand::OpenBlock{block:54,position:(2,64,1)}).unwrap();
    assert_eq!(snapshot.kind,ContainerKind::Chest{rows:6});assert_eq!(snapshot.contents[0],Some(ItemStack::new(264,32,0)));let window=snapshot.window;
    let stale=w.container_command(id,ContainerCommand::Click{window:window+1,slot:0,button:0,shift:true}).unwrap();assert_eq!(stale.contents[0],snapshot.contents[0]);
    let moved=w.container_command(id,ContainerCommand::Click{window,slot:0,button:0,shift:true}).unwrap();assert!(moved.contents[0].is_none());assert_eq!(moved.contents[89],snapshot.contents[0]);
    assert!(matches!(w.get_block_tile_entity(1,64,1),Some(TileEntity::Chest(items)) if items[0].is_none()));
    assert!(w.container_usable(id));
    w.entities.get_mut(&id).unwrap().set_position(40.0,64.0,40.0);assert!(!w.container_usable(id));
    assert!(w.container_command(id,ContainerCommand::Close{window}).is_none());assert!(w.container_snapshot(id).is_none());
}
#[test]
fn close_drops_cursor_and_crafting_inputs_as_entities() {
    let (mut w,id)=fixture();let snapshot=w.container_command(id,ContainerCommand::OpenInventory).unwrap();
    let session=w.container_sessions.get_mut(&id).unwrap();let c=session.active.as_mut().unwrap();c.matrix[0]=Some(ItemStack::new(5,4,0));c.matrix_changed();
    w.entities.get_mut(&id).unwrap().player_state_mut().unwrap().inventory.cursor=Some(ItemStack::new(264,3,0));
    w.container_command(id,ContainerCommand::Close{window:snapshot.window});
    assert!(w.entities[&id].player_state().unwrap().inventory.cursor.is_none());
    let mut items:Vec<_>=w.entities.values().filter_map(|e|if let worldgen::entity::EntityKind::Item{item,..}=e.kind{Some((item.id,item.count))}else{None}).collect();items.sort();assert_eq!(items,vec![(5,4),(264,3)]);
}
#[test]
fn machine_nbt_roundtrip_preserves_signed_fields_and_recomputes_fuel() {
    use worldgen::container::tiles::Furnace;
    let f=Furnace{items:[Some(ItemStack::new(15,12,0)),Some(ItemStack::new(263,3,1)),Some(ItemStack::new(265,8,0))],burn:777,cook:103,current_fuel:99};
    let restored=Furnace::from_nbt(&f.to_nbt(8,64,8));assert_eq!(restored.items,f.items);assert_eq!((restored.burn,restored.cook,restored.current_fuel),(777,103,1600));
}
