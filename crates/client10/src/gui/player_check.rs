//! Bytecode comparison for the new options and key event queues.
use super::{settings::*,lang::Lang};
use std::fmt::Write;
pub fn check(path:&str)->bool {
 let mut jar=crate::assets::Jar::open().unwrap();let mut lang=Lang::default();lang.load(jar.text("lang/en_US.lang").unwrap().as_bytes());lang.load(jar.text("lang/stats_US.lang").unwrap().as_bytes());
 let mut s=GameSettings::default();let mut out=String::new();
 for e in EnumOptions::ALL {writeln!(out,"{} {} {} {} {}",e.ordinal(),e.key(),e.is_float(),e.is_bool(),s.get_key_binding(&lang,e)).unwrap();}
 for e in [EnumOptions::Fov,EnumOptions::Gamma,EnumOptions::Sensitivity,EnumOptions::Music,EnumOptions::Sound] {for f in [-1.0,0.0,0.01,0.333,0.5,0.99,1.0,2.0] {s.set_option_float_value(e,f);writeln!(out,"{}",s.get_key_binding(&lang,e)).unwrap();}}
 for k in &s.key_bindings{writeln!(out,"{} {}",k.description,k.code()).unwrap();}
 let mut r=worldgen::JavaRandom::new(181);
 for _ in 0..10000 {let op=r.next_int(5);let index=r.next_int(s.key_bindings.len() as i32) as usize;let code=r.next_int(220)-105;let down=r.next_bool();
  match op {0=>{s.key_bindings[index].set_code(code);KeyBinding::rebuild_registry();},1=>s.key_tick(code),2=>s.key_state(code,down),3=>{writeln!(out,"{}",s.key_bindings[index].take_press()).unwrap();},_=>s.release_keys()}
  for k in &s.key_bindings{write!(out,"{},{},{};",k.code(),k.pressed(),k.press_count()).unwrap();}out.push('\n');
 }
 let expected=std::fs::read_to_string(path).unwrap().replace("\r\n","\n");
 use super::stats_table as table;
 for (id,kind,name) in table::GENERAL {writeln!(out,"general\t{id}\t{}\t{name}",match kind {table::StatType::Simple=>"simple",table::StatType::Time=>"time",table::StatType::Distance=>"distance"}).unwrap();}
 for (name,rows) in [("block",&table::BLOCKS[..]),("item",&table::ITEMS[..]),("mine",&table::MINE[..]),("craft",&table::CRAFT[..]),("use",&table::USE[..]),("break",&table::BREAK[..])] {for (a,b) in rows {writeln!(out,"{name}\t{a}\t{b}").unwrap();}}
 for achievement in super::stats::ACHIEVEMENTS {writeln!(out,"achievement\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",achievement.id,achievement.column,achievement.row,achievement.icon.0,achievement.parent.map_or(-1,|p|super::stats::ACHIEVEMENTS[p].id),achievement.special,lang.translate_key(&format!("achievement.{}",achievement.key)),lang.translate_key(&format!("achievement.{}.desc",achievement.key))).unwrap();}
 let (min_x,min_y,max_x,max_y)=super::stats::display_bounds();writeln!(out,"bounds\t{min_x}\t{min_y}\t{max_x}\t{max_y}").unwrap();
 let kinds=[table::StatType::Simple,table::StatType::Time,table::StatType::Distance];
 for kind in kinds {for v in [i32::MIN,-1,0,1,9,10,49,50,51,599,600,601,35999,36000,36001,863999,864000,864001,50000,50001,315359999,315360000,315360001,i32::MAX] {writeln!(out,"format\t{v}\t{}",super::screens::stats::format_stat(kind,v)).unwrap();}}
 let mut r=worldgen::JavaRandom::new(181);for _ in 0..5000 {let v=r.next_int_unbounded();for kind in kinds {writeln!(out,"format\t{v}\t{}",super::screens::stats::format_stat(kind,v)).unwrap();}}
 if expected==out {println!("OPTIONS, KEYBINDINGS AND STATS EXACT: 10000 key events, registry, 17 achievements, 15072 format cases");true}else {std::fs::write("target/options181-rust.txt",out).unwrap();println!("OPTIONS, KEYBINDINGS OR STATS DIFFER");false}
}
