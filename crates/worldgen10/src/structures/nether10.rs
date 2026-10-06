//! Release 1.0 Nether fortress weighted graph, candidate RNG and retained starts.
use super::{component::{self, Component}, long_map_order::LongMapOrder, BoundingBox as B, Piece};
use crate::{JavaRandom, World};
use std::{cmp::{min,max}, collections::HashMap};
#[path = "nether_generated10.rs"]
mod generated;
const BOXES: [[i32;6];14] = [
    [-1,-3,0,5,10,19], [-8,-3,0,19,10,19], [-2,0,0,7,9,7], [-2,0,0,7,11,7],
    [-2,0,0,7,8,9], [-5,-3,0,13,14,13], [-1,0,0,5,7,5], [-1,0,0,5,7,5],
    [-1,0,0,5,7,5], [-1,0,0,5,7,5], [-1,-7,0,5,14,10], [-3,0,0,9,7,9],
    [-5,-3,0,13,14,13], [-1,-3,0,5,10,8],
];
#[derive(Clone,Debug)]
pub struct NetherPiece { pub component: Component, pub kind: usize, pub end_seed: i32, spawner: bool }
impl NetherPiece {
    fn factory(pieces: &[Self], r: &mut JavaRandom, kind: usize, pos: [i32;3], dir: i32, depth: i32) -> Option<Self> {
        let [dx,dy,dz,sx,sy,sz] = BOXES[kind];
        let [x,y,z] = pos;
        let bounds = B::oriented(x,y,z,dx,dy,dz,sx,sy,sz,dir);
        if bounds.min_y <= 10 || pieces.iter().any(|p| p.component.bounds.intersects(bounds)) { return None; }
        Some(Self { component: Component { bounds, orientation: dir, depth }, kind,
            end_seed: if kind == 13 { r.next_int_unbounded() } else { 0 }, spawner: false })
    }
    fn spawner10(&mut self,w:&mut World,clip:B,x:i32,y:i32,z:i32,mob_id:&str) {
        let (x,y,z) = self.component.position(x,y,z);
        if !self.spawner && clip.contains(x,y,z) {
            self.spawner = true;
            w.set_block_with_notify(x,y,z,52);
            if let Some(crate::world::tile::TileEntity::MobSpawner {mob,..}) = w.get_block_tile_entity(x,y,z) { *mob = mob_id.into(); }
        }
    }
    fn lava10(&self,w:&mut World,r:&mut JavaRandom,clip:B) {
        let (x,y,z) = self.component.position(6,5,6);
        if clip.contains(x,y,z) {
            *w.prand() = component::random(w,r).clone();
            w.immediate_updates = true;
            crate::world::behavior::update_tick(w,10,x,y,z,crate::world::RandSel::Populate);
            w.immediate_updates = false;
            *r = w.prand().clone();
            w.structure_rng_epoch = Some(w.generator_epoch);
        }
    }
}
impl Piece for NetherPiece {
    fn bounds(&self)->B { self.component.bounds }
    fn bounds_mut(&mut self)->&mut B { &mut self.component.bounds }
    fn component_type(&self)->i32 { self.component.depth }
    fn generate(&mut self,w:&mut World,r:&mut JavaRandom,clip:B)->bool {
        match self.kind {
            0=>self.generate_0(w,r,clip),1=>self.generate_1(w,r,clip),2=>self.generate_2(w,r,clip),
            3=>self.generate_3(w,r,clip),4=>self.generate_4(w,r,clip),5=>self.generate_5(w,r,clip),
            6=>self.generate_6(w,r,clip),7=>self.generate_7(w,r,clip),8=>self.generate_8(w,r,clip),
            9=>self.generate_9(w,r,clip),10=>self.generate_10(w,r,clip),11=>self.generate_11(w,r,clip),
            12=>self.generate_12(w,r,clip),13=>self.generate_13(w,r,clip),_=>unreachable!()
        }
    }
}
struct Weight {kind:usize,weight:i32,limit:i32,count:i32,repeat:bool}
pub struct NetherStart {
    pub pieces:Vec<NetherPiece>, pub bounds:B, weights:[Vec<Weight>;2], previous:Option<usize>, pending:Vec<usize>,
}
impl NetherStart {
    pub fn new(r:&mut JavaRandom,cx:i32,cz:i32)->Self {
        let x=(cx<<4).wrapping_add(2); let z=(cz<<4).wrapping_add(2);
        let root=NetherPiece {component:Component {bounds:B::new([x,64,z,x.wrapping_add(18),73,z.wrapping_add(18)]),orientation:r.next_int(4),depth:0},kind:1,end_seed:0,spawner:false};
        let make=|data:&[(usize,i32,i32,bool)]| data.iter().map(|&(kind,weight,limit,repeat)| Weight {kind,weight,limit,repeat,count:0}).collect();
        let mut s=Self {pieces:vec![root],bounds:B::empty(),weights:[
            make(&[(0,30,0,true),(1,10,4,false),(2,10,4,false),(3,10,3,false),(4,5,2,false),(5,5,1,false)]),
            make(&[(6,25,0,true),(7,15,5,false),(8,5,10,false),(9,5,10,false),(10,10,3,true),(11,7,2,false),(12,5,2,false)])],previous:None,pending:vec![]};
        s.expand(0,r);
        while !s.pending.is_empty() {let n=r.next_int(s.pending.len() as i32) as usize;let p=s.pending.remove(n);s.expand(p,r);}
        for p in &s.pieces {s.bounds.encompass(p.bounds());}
        let room=70-48+1-s.bounds.sizes()[1];
        let y=if room>1 {48+r.next_int(room)} else {48};
        let dy=y.wrapping_sub(s.bounds.min_y);s.bounds.offset(0,dy,0);
        for p in &mut s.pieces {p.component.bounds.offset(0,dy,0);}
        s
    }
    fn select(&mut self,r:&mut JavaRandom,pos:[i32;3],dir:i32,depth:i32,inside:bool)->Option<NetherPiece> {
        let list=usize::from(inside);
        let total=if self.weights[list].iter().any(|w|w.limit>0&&w.count<w.limit) {self.weights[list].iter().map(|w|w.weight).sum()} else {-1};
        if total>0&&depth<=30 {
            for _ in 0..5 {
                let mut n=r.next_int(total);
                for i in 0..self.weights[list].len() {
                    let weight=&self.weights[list][i]; n-=weight.weight;
                    if n<0 {
                        if (weight.limit>0&&weight.count>=weight.limit)||(self.previous==Some(weight.kind)&&!weight.repeat) {break;}
                        let kind=weight.kind;
                        if let Some(p)=NetherPiece::factory(&self.pieces,r,kind,pos,dir,depth) {
                            self.previous=Some(kind);self.weights[list][i].count+=1;
                            if self.weights[list][i].limit>0&&self.weights[list][i].count>=self.weights[list][i].limit {self.weights[list].remove(i);}
                            return Some(p);
                        }
                    }
                }
            }
        }
        NetherPiece::factory(&self.pieces,r,13,pos,dir,depth)
    }
    fn connect(&mut self,p:&NetherPiece,r:&mut JavaRandom,side:i32,a:i32,b:i32,inside:bool) {
        let bb=p.bounds();let d=p.component.orientation;
        let (x,y,z,dir)=match (side,d) {
            (0,0)=>(bb.min_x.wrapping_add(a),bb.min_y.wrapping_add(b),bb.max_z.wrapping_add(1),d),
            (0,1)=>(bb.min_x.wrapping_sub(1),bb.min_y.wrapping_add(b),bb.min_z.wrapping_add(a),d),
            (0,2)=>(bb.min_x.wrapping_add(a),bb.min_y.wrapping_add(b),bb.min_z.wrapping_sub(1),d),
            (0,3)=>(bb.max_x.wrapping_add(1),bb.min_y.wrapping_add(b),bb.min_z.wrapping_add(a),d),
            (1,0|2)=>(bb.min_x.wrapping_sub(1),bb.min_y.wrapping_add(a),bb.min_z.wrapping_add(b),1),
            (1,1|3)=>(bb.min_x.wrapping_add(b),bb.min_y.wrapping_add(a),bb.min_z.wrapping_sub(1),2),
            (2,0|2)=>(bb.max_x.wrapping_add(1),bb.min_y.wrapping_add(a),bb.min_z.wrapping_add(b),3),
            (2,1|3)=>(bb.min_x.wrapping_add(b),bb.min_y.wrapping_add(a),bb.max_z.wrapping_add(1),0),_=>return,
        };
        if x.wrapping_sub(self.pieces[0].bounds().min_x).wrapping_abs()>112||z.wrapping_sub(self.pieces[0].bounds().min_z).wrapping_abs()>112 {
            // Vanilla constructs this terminator but does not append it to either list.
            NetherPiece::factory(&self.pieces,r,13,[x,y,z],dir,p.component.depth);
        } else if let Some(p)=self.select(r,[x,y,z],dir,p.component.depth+1,inside) {
            self.pending.push(self.pieces.len());self.pieces.push(p);
        }
    }
    fn expand(&mut self,index:usize,r:&mut JavaRandom) {
        let p=self.pieces[index].clone();
        match p.kind {
            0=>self.connect(&p,r,0,1,3,false),
            1=>{self.connect(&p,r,0,8,3,false);self.connect(&p,r,1,3,8,false);self.connect(&p,r,2,3,8,false);},
            2=>{self.connect(&p,r,0,2,0,false);self.connect(&p,r,1,0,2,false);self.connect(&p,r,2,0,2,false);},
            3=>self.connect(&p,r,2,6,2,false),5=>self.connect(&p,r,0,5,3,true),
            6|10=>self.connect(&p,r,0,1,0,true),
            7=>{self.connect(&p,r,0,1,0,true);self.connect(&p,r,1,0,1,true);self.connect(&p,r,2,0,1,true);},
            8=>self.connect(&p,r,2,0,1,true),9=>self.connect(&p,r,1,0,1,true),
            11=>{let b=if matches!(p.component.orientation,1|2) {5} else {1};let inside=r.next_int(8)>0;self.connect(&p,r,1,0,b,inside);let inside=r.next_int(8)>0;self.connect(&p,r,2,0,b,inside);},
            12=>{self.connect(&p,r,0,5,3,true);self.connect(&p,r,0,5,11,true);},_=>{}
        }
    }
}
#[derive(Default)]
pub struct NetherBridgeGenerator {pub starts:HashMap<i64,NetherStart>,order:LongMapOrder<i64>}
impl NetherBridgeGenerator {
    pub fn keys(&self)->Vec<i64> {self.order.iter().collect()}
    pub(crate) fn merge_start(&mut self,key:i64,start:NetherStart) {
        if let std::collections::hash_map::Entry::Vacant(e)=self.starts.entry(key) {e.insert(start);self.order.insert((key^((key as u64>>32) as i64)) as i32,key);}
    }
    pub fn generate(&mut self,seed:i64,cx:i32,cz:i32) {
        let mut r=JavaRandom::new(seed);let mx=r.next_long();let mz=r.next_long();
        for x in cx.wrapping_sub(8)..=cx.wrapping_add(8) {for z in cz.wrapping_sub(8)..=cz.wrapping_add(8) {
            let key=super::ChunkCoordIntPair{x,z}.packed();
            if self.starts.contains_key(&key) {continue;}
            r.set_seed((x as i64).wrapping_mul(mx)^(z as i64).wrapping_mul(mz)^seed);r.next_int_unbounded();
            let (rx,rz)=(x>>4,z>>4);r.set_seed((rx^(rz<<4)) as i64^seed);r.next_int_unbounded();
            if r.next_int(3)==0&&x==(rx<<4).wrapping_add(4).wrapping_add(r.next_int(8))&&z==(rz<<4).wrapping_add(4).wrapping_add(r.next_int(8)) {
                self.merge_start(key,NetherStart::new(&mut r,x,z));
            }
        }}
    }
    pub fn contains(&self,x:i32,y:i32,z:i32)->bool {
        self.starts.values().any(|s|s.bounds.intersects_region(x,z,x,z)&&s.pieces.iter().any(|p|p.bounds().contains(x,y,z)))
    }
    pub fn populate(&mut self,w:&mut World,r:&mut JavaRandom,cx:i32,cz:i32) {
        let epoch=w.structure_rng_epoch.replace(w.generator_epoch);
        let x=(cx<<4).wrapping_add(8);let z=(cz<<4).wrapping_add(8);let clip=B::region(x,z,x.wrapping_add(15),z.wrapping_add(15));
        for key in self.order.iter() {let s=self.starts.get_mut(&key).unwrap();if s.bounds.intersects_region(x,z,x.wrapping_add(15),z.wrapping_add(15)) {
            s.pieces.retain_mut(|p|!p.bounds().intersects(clip)||p.generate(w,r,clip));
        }}
        component::random(w,r);w.structure_rng_epoch=epoch;
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fortress_provider_list_preserves_order_weights_and_registered_factories() {
        let mut w = World::with_dimension(42, 7, crate::world::Dimension::Nether);
        let start = NetherStart::new(&mut JavaRandom::new(7), 4, -7);
        let [x, y, z] = start.pieces[0].bounds().center();
        w.nether_bridges10.merge_start(super::super::ChunkCoordIntPair { x: 4, z: -7 }.packed(), start);
        let entries = w.provider_spawn_list10(0, x, y, z);
        assert_eq!(entries.iter().map(|e| (e.entity, e.weight, e.min, e.max)).collect::<Vec<_>>(),
            [("Blaze", 10, 2, 3), ("PigZombie", 10, 4, 4), ("LavaSlime", 3, 4, 4)]);
        for entry in entries {
            assert!(crate::entity::registry::create(&mut w, entry.entity).is_some());
        }
        let outside = w.provider_spawn_list10(0, 0, 64, 0);
        assert_eq!(outside.iter().map(|e| e.entity).collect::<Vec<_>>(), ["Ghast", "PigZombie", "LavaSlime"]);
    }
    #[test]
    fn fortress_throne_uses_a_one_shot_blaze_spawner() {
        let mut w=World::with_dimension(42,7,crate::world::Dimension::Nether);
        w.chunks.insert((0,0),Box::new(crate::world::chunk::Chunk::new(0,0,vec![0;32768].into_boxed_slice())));
        let mut p=NetherPiece {component:Component {bounds:B::new([1,50,1,7,57,9]),orientation:0,depth:1},kind:4,end_seed:0,spawner:false};
        let clip=B::new([0,0,0,15,127,15]);
        let mut r=JavaRandom::new(7);
        assert!(p.generate(&mut w,&mut r,clip));
        let (x,y,z)=p.component.position(3,5,5);
        assert_eq!(w.get_block_id(x,y,z),52);
        let Some(crate::world::tile::TileEntity::MobSpawner {mob,delay})=w.get_block_tile_entity(x,y,z) else {panic!("missing Blaze spawner")};
        assert_eq!(mob,"Blaze");*delay=123;
        assert!(p.generate(&mut w,&mut r,clip));
        // The placement body may overwrite the spawner cell; the one-shot flag
        // still prevents a second spawner installation, exactly as in Java.
        assert!(p.spawner);
    }
    #[test]
    fn fortress_graphs_keep_limits_and_the_vertical_offset() {
        for seed in [0,1,42,-999] {
            let s=NetherStart::new(&mut JavaRandom::new(seed),4,-7);
            assert_eq!(s.bounds.min_y,48.max(s.bounds.min_y));
            for (kind,limit) in [(1,4),(2,4),(3,3),(4,2),(5,1),(7,5),(8,10),(9,10),(10,3),(11,2),(12,2)] {
                let count=s.pieces.iter().skip(1).filter(|p|p.kind==kind).count();assert!(count<=limit);
            }
            for i in 1..s.pieces.len() {assert!(!s.pieces[..i].iter().any(|p|p.bounds().intersects(s.pieces[i].bounds())));}
        }
    }
}
