//! Original ContainerCreative catalog and GUI click policy.
use super::*;
use std::sync::OnceLock;
pub fn catalog() -> &'static [ItemStack] {
 static ITEMS: OnceLock<Vec<ItemStack>> = OnceLock::new();
 ITEMS.get_or_init(|| {
  let blocks = [4,1,56,14,15,16,21,73,98,98,98,82,57,41,42,7,22,45,48,44,44,44,44,44,44,49,87,88,89,17,17,17,18,3,2,12,24,13,30,5,6,6,6,32,19,79,80,37,38,39,40,83,81,103,86,91,106,101,102,54,58,20,46,47,35,35,35,35,35,35,35,35,35,35,35,35,35,35,35,35,23,61,25,84,29,33,85,107,65,66,27,28,50,53,67,108,109,69,70,72,76,77,92,96];
  let mut counters = [0;5]; let mut items = Vec::new();
  for id in blocks {
   let c = match id {35=>Some(0),44=>Some(1),17=>Some(2),6=>Some(3),98=>Some(4),_=>None};
   let damage=c.map_or(0,|i|{let n=counters[i];counters[i]+=1;n});
   items.push(ItemStack::new(id,1,damage));
  }
  for id in 256..32000 {if registry::item(id).is_some(){items.push(ItemStack::new(id,1,0));}}
  for damage in 1..16 {items.push(ItemStack::new(351,1,damage));} items
 })
}
pub fn contents(position:f32)->Vec<Option<ItemStack>> {
 let rows=catalog().len()/8-8+1;
 let offset=(((position * rows as f32) as f64 + 0.5) as i32).max(0) as usize * 8;
 (0..72).map(|i|catalog().get(offset+i).copied()).collect()
}
impl Container {
 pub fn scroll_creative(&mut self,position:f32) {if self.kind==ContainerKind::Creative {self.tile=contents(position);}}
 pub(super) fn creative_click(&mut self,p:&mut InventoryPlayer,slot:i32,button:i32,shift:bool)->Option<ItemStack> {
  if slot<0 {return self.slot_click(p,-999,button,shift);}
  match (p.cursor,self.stack(p,slot as usize)) {
   (Some(mut held),Some(item)) if held.id==item.id => {
    if button==0 {if shift {held.count=held.max_stack();}else if held.count<held.max_stack(){held.count+=1;}p.cursor=Some(held);}
    else if held.count<=1 {p.cursor=None;}else {held.count-=1;p.cursor=Some(held);}
   }
   (Some(_),_)=>p.cursor=None,
   (None,Some(mut item))=>{if shift {item.count=item.max_stack();}p.cursor=Some(item);},
   _=>p.cursor=None,
  } None
 }
}
