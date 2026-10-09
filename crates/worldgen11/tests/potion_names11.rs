use std::collections::HashMap;
struct Input { bytes: Vec<u8>, at:usize }
impl Input {
 fn int(&mut self)->i32 {let value=i32::from_be_bytes(self.bytes[self.at..self.at+4].try_into().unwrap());self.at+=4;value}
 fn text(&mut self)->String {let n=self.int();let units:Vec<u16>=(0..n).map(|_|{let u=u16::from_be_bytes(self.bytes[self.at..self.at+2].try_into().unwrap());self.at+=2;u}).collect();String::from_utf16(&units).unwrap()}
}
#[test]
#[ignore = "requires freshly generated unchanged-original 1.1 fixture"]
fn original_11_potion_names_and_tooltips() {
 let mut input=Input{bytes:std::fs::read(std::env::var("POTION_NAMES11_FIXTURE").unwrap()).unwrap(),at:0};assert_eq!(input.int(),0x504e3131);
 let mut item=worldgen::item10::PotionItem::default();let mut cases=0;
 for _ in 0..input.int(){let code=input.text();let mut table=HashMap::new();for _ in 0..input.int(){let key=input.text();let value=input.text();table.insert(key,value);}let _base_key=input.text();let base=input.text();
  for _ in 0..input.int(){let damage=input.int();let expected=input.text();let translate=|key:&str|table.get(key).cloned().unwrap_or_else(||key.into());assert_eq!(item.display_name(damage,&base,translate),expected,"{code} damage={damage}");let expected:Vec<String>=(0..input.int()).map(|_|input.text()).collect();assert_eq!(item.tooltip(damage,translate),expected,"{code} damage={damage}");cases+=1;}
 }
 assert_eq!(input.at,input.bytes.len());println!("EXACT original 1.1 potion names and tooltip lines: {cases} metadata/locale cases");
}

