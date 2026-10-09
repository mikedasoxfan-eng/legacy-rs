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
  if self.base.buttons.iter().any(|button|button.id==id&&!button.enabled){return;}
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
  fn state(&mut self,ui:&Ui,screen:&GuiLanguage){assert_eq!(ui.settings.language,self.text());assert_eq!(ui.lang.language(),self.text());assert_eq!(ui.font.unicode,self.byte()!=0);assert_eq!(ui.font.bidi,self.byte()!=0);assert_eq!(screen.base.buttons.iter().find(|b|b.id==6).unwrap().text,self.text());assert_eq!(screen.refresh_timer,self.int());}
  fn vertices(&mut self,ui:&Ui,context:&str){
   let expected=self.int() as usize;let draws:Vec<_>=ui.gl.draws.iter().filter(|d|d.state.texture.is_some_and(|t|t==ui.font.texture||t&0x8000_0000!=0)).collect();assert_eq!(draws.len()*4,expected,"{context} font vertex count");
   for draw in draws{for i in [0,1,2,5]{let texture=draw.state.texture.unwrap();assert_eq!(self.int() as u32,if texture==ui.font.texture{256}else{texture&255},"{context} texture");let vertex=&ui.gl.verts[draw.first as usize+i];for value in vertex.pos.into_iter().chain(vertex.uv).chain(vertex.color){assert_eq!(value.to_bits(),self.int() as u32,"{context} geometry/color");}}}
  }
 }
 fn saved_language(dir:&std::path::Path)->String{std::fs::read_to_string(dir.join("options.txt")).ok().and_then(|s|s.lines().find_map(|l|l.strip_prefix("lang:").map(String::from))).unwrap_or_else(||"<missing>".into())}
 #[test]
 #[ignore="requires original LANGUAGE_WORKFLOW11_FIXTURE and installed assets"]
 fn original_language_workflow(){
  let mut input=Input{bytes:std::fs::read(std::env::var("LANGUAGE_WORKFLOW11_FIXTURE").unwrap()).unwrap(),at:0};assert_eq!(input.int(),0x4c574631);let resolutions=input.int();let mut frames=0;let mut list_draws=0;
  let root=std::path::PathBuf::from(format!("target/r11/language-workflow11-rust-{}",std::process::id()));
  for _ in 0..resolutions{
   let (w,h,count,content)=(input.int(),input.int(),input.int(),input.int());let dir=root.join(format!("{w}-{h}"));std::fs::create_dir_all(&dir).unwrap();let _=std::fs::remove_file(dir.join("options.txt"));
   let mut ui=crate::gui::utility_check::fixture_ui(dir.clone());ui.settings=crate::gui::settings::GameSettings::load(&dir);let mut jar=crate::assets::Jar::open().unwrap();ui.lang.load_languages(&jar.bytes("lang/languages.txt").unwrap());ui.textures.insert("gui/background.png".into(),1);ui.textures.insert("gui/gui.png".into(),2);assert_eq!(Languages::new(&ui,(w,h)).size(),count);assert_eq!(content,count*18);
   for code in ["en_US","ar_SA","he_IL","ja_JP","el_GR","de_DE"]{
    let parent=Box::new(Parent(ScreenBase::default()));let parent_ptr=&*parent as *const Parent as *const ();let mut screen=GuiLanguage::new(parent);crate::gui::set_world_and_resolution(&mut screen,&mut ui,w,h);let mut languages=Languages::new(&ui,(w,h));let index=languages.entries.iter().position(|e|e.0==code).unwrap();languages.element_clicked(&mut ui,index as i32,false);screen.base.button(6).text=languages.done.take().unwrap();input.state(&ui,&screen);assert_eq!(saved_language(&dir),input.text());
    for row in 0..count{ui.gl.begin_frame();languages.draw_slot(&mut ui,row,-99,37+row%3*18,14);input.vertices(&ui,&format!("{code} row{row}"));assert_eq!(ui.font.bidi,input.byte()!=0);list_draws+=1;}
    // Observable pack list changes prove the actual update call, not just its timer.
    let pack=ui.packs.folder.join("workflow-proof.zip");std::fs::write(&pack,b"invalid zip retained by original pack scanner").unwrap();
    for (frame,updates) in [0,1,20].into_iter().enumerate(){
     for _ in 0..updates{screen.update_screen(&mut ui);}ui.gl.begin_frame();let due=screen.refresh_timer<=0;screen.draw_screen(&mut ui,0,0,0.0);assert_eq!(input.int(),due as i32);input.state(&ui,&screen);input.vertices(&ui,&format!("{code} frame{frame}"));assert_eq!(ui.packs.available.iter().any(|p|p.name=="workflow-proof.zip"),frame<2);if frame==0{std::fs::remove_file(&pack).unwrap();}frames+=1;
    }
    for (i,id) in [8,8,7,8].into_iter().enumerate(){
     if i==3{let mut button=GuiButton::new(8,0,0,"disabled scroll");button.enabled=false;screen.base.buttons.push(button);}
     screen.action_performed(&mut ui,id);if i==3{screen.base.buttons.pop();}ui.gl.begin_frame();let due=screen.refresh_timer<=0;screen.draw_screen(&mut ui,0,0,0.0);assert_eq!(input.int(),due as i32);input.state(&ui,&screen);input.vertices(&ui,&format!("{code} scroll{i}"));frames+=1;
    }
    ui.actions.clear();screen.base.button(6).enabled=false;screen.action_performed(&mut ui,6);assert_eq!(ui.actions.len() as i32,input.int());assert_eq!(saved_language(&dir),input.text());screen.base.button(6).enabled=true;
    screen.action_performed(&mut ui,5);assert_eq!(ui.actions.len() as i32,input.int());assert_eq!(saved_language(&dir),input.text());
    screen.action_performed(&mut ui,6);assert_eq!(ui.actions.len() as i32,input.int());let returned=match ui.actions.pop().unwrap(){crate::gui::Action::Display(Some(parent))=>(&*parent as *const dyn Screen as *const ())==parent_ptr,_=>false};assert_eq!(returned,input.byte()!=0);assert_eq!(saved_language(&dir),input.text());assert_eq!(saved_language(&dir),input.text());
    let reopened=crate::gui::settings::GameSettings::load(&dir);assert_eq!(reopened.language,input.text());let mut restarted=crate::gui::utility_check::fixture_ui(dir.join("restart"));restarted.settings=reopened;restarted.set_language(code).unwrap();assert_eq!(restarted.font.unicode,input.byte()!=0);assert_eq!(restarted.font.bidi,input.byte()!=0);
   }
  }
  assert_eq!(input.at,input.bytes.len());println!("LANGUAGE WORKFLOW EXACT: {resolutions} resolutions,18selections,{list_draws} listdraws,{frames} frames,18saves/reopens");
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
