//! Bytecode comparison for the new options and key event queues.
use super::{settings::*,lang::Lang};
use std::fmt::Write;
pub fn check(path:&str)->bool {
 let mut jar=crate::assets::Jar::open().unwrap();let mut lang=Lang::default();lang.load(jar.text("lang/en_US.lang").unwrap().as_bytes());lang.load(jar.text("lang/stats_US.lang").unwrap().as_bytes());
 let mut s=GameSettings::default();let mut out=String::new();
 for e in EnumOptions::ALL {writeln!(out,"{} {} {} {} {}",e.ordinal(),e.key(),e.is_float(),e.is_bool(),s.get_key_binding(&lang,e)).unwrap();}
 for e in [EnumOptions::Fov,EnumOptions::Gamma,EnumOptions::Sensitivity,EnumOptions::Music,EnumOptions::Sound] {for f in [-1.0,0.0,0.01,0.333,0.5,0.99,1.0,2.0] {s.set_option_float_value(e,f);writeln!(out,"{}",s.get_key_binding(&lang,e)).unwrap();}}
 for k in &s.key_bindings{writeln!(out,"{} {}",k.description,k.code).unwrap();}
 let mut r=worldgen::JavaRandom::new(181);
 for _ in 0..10000 {let op=r.next_int(5);let index=r.next_int(s.key_bindings.len() as i32) as usize;let code=r.next_int(220)-105;let down=r.next_bool();
  match op {0=>s.key_bindings[index].code=code,1=>s.key_tick(code),2=>s.key_state(code,down),3=>{writeln!(out,"{}",s.key_bindings[index].take_press()).unwrap();},_=>s.release_keys()}
  for k in &s.key_bindings{write!(out,"{},{},{};",k.code,k.pressed,k.press_count).unwrap();}out.push('\n');
 }
 let expected=std::fs::read_to_string(path).unwrap().replace("\r\n","\n");
 if expected==out {println!("OPTIONS AND KEYBINDINGS EXACT: 10000 events");true}else {std::fs::write("target/options181-rust.txt",out).unwrap();println!("OPTIONS AND KEYBINDINGS DIFFER");false}
}
