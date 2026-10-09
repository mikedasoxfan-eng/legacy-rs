//! Original 1.1 GuiLanguage and GuiSlotLanguage callbacks.
use crate::gui::slot::{GuiSlot,SlotList};
use crate::gui::widgets::GuiButton;
use crate::gui::{Screen,ScreenBase,Ui};
pub struct GuiLanguage{base:ScreenBase,parent:Option<Box<dyn Screen>>,refresh_timer:i32,slot:Option<GuiSlot>}
impl GuiLanguage{pub fn new(parent:Box<dyn Screen>)->Self{Self{base:ScreenBase::default(),parent:Some(parent),refresh_timer:-1,slot:None}}}
struct Languages{entries:Vec<(String,String)>,selected:String,background:(i32,i32),done:Option<String>}
impl Languages{
 fn new(ui:&Ui,background:(i32,i32))->Self{Self{entries:ui.lang.languages().into_iter().map(|(code,name)|(code.into(),name.into())).collect(),selected:ui.lang.language().into(),background,done:None}}
}
impl SlotList for Languages{
 fn size(&self)->i32{self.entries.len() as i32}
 fn content_height(&self)->Option<i32>{Some(self.size()*18)}
 fn element_clicked(&mut self,ui:&mut Ui,index:i32,_double:bool){
  let code=&self.entries[index as usize].0;
  if let Err(error)=ui.set_language(code){eprintln!("language: {error}");}
  self.selected=ui.lang.language().into();self.done=Some(ui.translate("gui.done"));
 }
 fn is_selected(&self,index:i32)->bool{self.entries[index as usize].0==self.selected}
 fn draw_background(&mut self,ui:&mut Ui){ui.draw_world_background(self.background.0,self.background.1,0);}
 fn draw_slot(&mut self,ui:&mut Ui,index:i32,_x:i32,y:i32,_height:i32){
  ui.font.bidi=true;
  ui.draw_centered_string(&self.entries[index as usize].1,self.background.0/2,y+1,16777215);
  ui.font.bidi=crate::gui::lang::Lang::language_is_bidi(&ui.settings.language);
 }
}
impl Screen for GuiLanguage{
 fn base(&mut self)->&mut ScreenBase{&mut self.base}
 fn init_gui(&mut self,ui:&mut Ui){
  let (w,h)=(self.base.width,self.base.height);
  self.base.buttons.push(GuiButton::small(6,w/2-75,h-38,ui.translate("gui.done")));
  let mut slot=GuiSlot::new(w,h,32,h-65+4,18);slot.register_scroll_buttons(7,8);self.slot=Some(slot);
 }
 fn action_performed(&mut self,ui:&mut Ui,id:i32){
  if id==6{if self.base.buttons.iter().any(|b|b.id==6&&b.enabled){ui.settings.save();ui.display(self.parent.take());}}
  else if id!=5{if let Some(slot)=&mut self.slot{slot.action_performed(&Languages::new(ui,(self.base.width,self.base.height)),id,true);}}
 }
 fn update_screen(&mut self,_ui:&mut Ui){self.refresh_timer-=1;}
 fn draw_screen(&mut self,ui:&mut Ui,mx:i32,my:i32,_partial:f32){
  let (w,h)=(self.base.width,self.base.height);let mut languages=Languages::new(ui,(w,h));
  if let Some(slot)=&mut self.slot{slot.draw_screen(ui,&mut languages,mx,my);}
  if let Some(done)=languages.done{self.base.button(6).text=done;}
  if self.refresh_timer<=0{ui.packs.update();self.refresh_timer+=20;}
  ui.draw_centered_string(&ui.translate("options.language"),w/2,16,16777215);
  ui.draw_centered_string(&format!("({})",ui.translate("options.languageWarning")),w/2,h-56,8421504);
  self.draw_buttons(ui,mx,my);
 }
}
#[cfg(test)]
mod tests{
 use super::*;
 struct Parent(ScreenBase);impl Screen for Parent{fn base(&mut self)->&mut ScreenBase{&mut self.0}}
 struct Input{bytes:Vec<u8>,at:usize}
 impl Input{
  fn byte(&mut self)->u8{let b=self.bytes[self.at];self.at+=1;b}
  fn int(&mut self)->i32{(0..4).fold(0u32,|n,_|(n<<8)|self.byte() as u32) as i32}
  fn text(&mut self)->String{let len=self.int();String::from_utf16(&(0..len).map(|_|(self.byte() as u16)<<8|self.byte() as u16).collect::<Vec<_>>()).unwrap()}
  fn buttons(&mut self,screen:&mut dyn Screen){let expected=self.int() as usize;assert_eq!(screen.base().buttons.len(),expected);for button in &screen.base().buttons{for actual in [button.id,button.x,button.y,button.width,button.height]{assert_eq!(actual,self.int());}assert_eq!(button.enabled,self.byte()!=0);assert_eq!(button.visible,self.byte()!=0);assert_eq!(button.text,self.text());}}
 }
 #[test]
 #[ignore="requires original LANGUAGE_SCREEN11_FIXTURE and installed assets"]
 fn original_language_callbacks(){
  let mut input=Input{bytes:std::fs::read(std::env::var("LANGUAGE_SCREEN11_FIXTURE").unwrap()).unwrap(),at:0};assert_eq!(input.int(),0x4c534331);let resolutions=input.int();let mut selections=0;
  for _ in 0..resolutions{
   let (w,h)=(input.int(),input.int());let mut ui=crate::gui::utility_check::fixture_ui("target/r11/language-callbacks".into());let mut jar=crate::assets::Jar::open().unwrap();ui.lang.load_languages(&jar.bytes("lang/languages.txt").unwrap());
   let mut options=super::super::options::GuiOptions::new(Box::new(Parent(ScreenBase::default())));crate::gui::set_world_and_resolution(&mut options,&mut ui,w,h);input.buttons(&mut options);
   let mut screen=GuiLanguage::new(Box::new(options));crate::gui::set_world_and_resolution(&mut screen,&mut ui,w,h);input.buttons(&mut screen);
   let mut languages=Languages::new(&ui,(w,h));assert_eq!(languages.size(),input.int());for (code,name) in &languages.entries{assert_eq!(code,&input.text());assert_eq!(name,&input.text());}
   for index in 0..languages.size(){languages.element_clicked(&mut ui,index,index&1!=0);screen.base.button(6).text=languages.done.take().unwrap();assert_eq!(ui.settings.language,input.text());assert_eq!(ui.lang.language(),input.text());assert_eq!(ui.font.unicode,input.byte()!=0);assert_eq!(ui.font.bidi,input.byte()!=0);assert_eq!(screen.base.button(6).text,input.text());for k in 0..languages.size(){assert_eq!(languages.is_selected(k),input.byte()!=0);}selections+=1;}
  }
  assert_eq!(input.at,input.bytes.len());println!("LANGUAGE SCREEN EXACT: {resolutions} resolutions, {selections} live selections/options and language buttons");
 }
}
