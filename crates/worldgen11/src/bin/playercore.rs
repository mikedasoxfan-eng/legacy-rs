use std::io::{self, Write};
use worldgen::{
    entity::player::food::{Experience, FoodAction, FoodStats, PlayerCapabilities}, nbt::{Compound, Tag},
    JavaRandom,};
fn int(w: &mut impl Write,v:i32)->io::Result<()> { w.write_all(&v.to_be_bytes()) }
fn state(w:&mut impl Write,f:&FoodStats,health:i32,action:i32)->io::Result<()> {
    int(w,f.level)?;int(w,f.prev_level)?;int(w,f.tick_timer)?;w.write_all(&f.saturation.to_be_bytes())?;w.write_all(&f.exhaustion.to_be_bytes())?;int(w,health)?;int(w,action)?;w.write_all(&[f.needs_food() as u8])
}
fn tick(w:&mut impl Write,f:&mut FoodStats,difficulty:i32,health:&mut i32,)->io::Result<()> {
    let action=match f.tick(difficulty,*health) { FoodAction::None=>0,FoodAction::Heal=>{*health=(*health+1).min(20);1}FoodAction::Starve=>{*health-=1;2} };state(w,f,*health,action)
}
fn main()->io::Result<()> {
    let path=std::env::args().nth(1).expect("playercore181 output.bin");
    let mut out=io::BufWriter::new(std::fs::File::create(path)?);
    for difficulty in 0..4 { for initial_health in [0,1,10,11,19,20] { for level in [0,1,17,18,20] { for saturation in [0.0,0.5,5.0] {
        let mut food=FoodStats::default();let mut health=initial_health;let mut n=Compound::new();n.put("foodLevel",Tag::Int(level));n.put("foodSaturationLevel",Tag::Float(saturation));n.put("foodExhaustionLevel",Tag::Float(40.0));food.read_nbt(&n);
        for _ in 0..200 { tick(&mut out,&mut food,difficulty,&mut health)?; }
    }}}}
    let mut random=JavaRandom::new(181);let mut food=FoodStats::default();let mut health=11;
    for t in 0..10000 {
        if t%7==0 { food.add_food(random.next_int(8)-2,random.next_float()); }
        if t%3==0 { food.add_exhaustion(random.next_float()*10.0); }
        if t%113==0 { let mut n=Compound::new();food.write_nbt(&mut n);food=FoodStats::default();food.read_nbt(&n); }
        if t%97==0 { food.read_nbt(&Compound::new()); }
        tick(&mut out,&mut food,2,&mut health)?;
    }
    let mut xp=Experience::default();
    for _ in 0..10000 { xp.add(random.next_int(200)-3);
        out.write_all(&xp.points.to_be_bytes())?;for value in [xp.level,xp.total,xp.bar_cap()] { int(&mut out,value)?; } }
    let mut c=PlayerCapabilities::default();out.write_all(&[c.disable_damage as u8,c.flying as u8,c.allow_flying as u8,c.creative as u8,])?;
    c.set_creative(true);out.write_all(&[c.disable_damage as u8,c.flying as u8,c.allow_flying as u8,c.creative as u8,])?;
    c.flying=true;c.set_creative(false);out.write_all(&[c.disable_damage as u8,c.flying as u8,c.allow_flying as u8,c.creative as u8,])?;
    for seed in [0i64,181,-1,i64::MIN,i64::MAX] { for mode in [0,1,2] { for features in [false,true] {
        let settings=worldgen::entity::player::food::WorldSettings::new(seed,mode,features);
        out.write_all(&settings.seed.to_be_bytes())?;int(&mut out,settings.game_mode)?;out.write_all(&[settings.map_features as u8])?;
    }}}
    for mask in 0..64 {
        let input=worldgen::entity::player::PlayerInput {forward:mask&1!=0,back:mask&2!=0,left:mask&4!=0,right:mask&8!=0,jump:mask&16!=0,sneak:mask&32!=0,..Default::default()};
        let (strafe,forward)=input.movement();out.write_all(&strafe.to_be_bytes())?;out.write_all(&forward.to_be_bytes())?;out.write_all(&[input.jump as u8,input.sneak as u8])?;
    }
    let mut world=worldgen::World::new(0,0);let mut player=worldgen::entity::Entity::player(&mut world,"Player");
    for flying in [false,true] {for speed in [0.02,0.1,0.13,0.2] {for draw in [-1,0,1,10,20,21,71999] {
        let state=player.player_state_mut().unwrap();state.capabilities.flying=flying;state.ground_speed=speed;
        state.item_in_use=if draw<0 {None}else {Some(worldgen::entity::player::ItemStack::new(261,1,0))};state.item_use_ticks=72000-draw;
        out.write_all(&player.fov_multiplier().to_be_bytes())?;
    }}}
    out.flush()
}
