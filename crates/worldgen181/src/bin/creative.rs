use worldgen::container::*;
use worldgen::JavaRandom;
use std::io::Write;
fn stack(o:&mut Vec<u8>,s:Option<ItemStack>){for n in [s.map_or(-1,|s|s.id),s.map_or(0,|s|s.count),s.map_or(0,|s|s.damage)]{o.extend(n.to_be_bytes());}}
fn state(o:&mut Vec<u8>,c:&Container,p:&InventoryPlayer){for i in 0..c.slots.len(){stack(o,c.stack(p,i));}stack(o,p.cursor);}
fn main(){let mut o=Vec::new();o.extend((creative::catalog().len() as i32).to_be_bytes());for s in creative::catalog(){stack(&mut o,Some(*s));}
 let mut c=Container::new(ContainerKind::Creative,0);let mut p=InventoryPlayer::default();
 for f in [-1.0,0.0,0.01,0.1,0.5,0.99,1.0,1.5]{c.scroll_creative(f);state(&mut o,&c,&p);}
 let mut r=JavaRandom::new(181);
 for i in 0..10000 {if i%13==0{c.scroll_creative(r.next_float());}let slot=r.next_int(81);let button=r.next_int(2);let shift=r.next_bool();c.slot_click(&mut p,slot,button,shift);state(&mut o,&c,&p);}
 std::fs::File::create(std::env::args().nth(1).expect("output")).unwrap().write_all(&o).unwrap();}
