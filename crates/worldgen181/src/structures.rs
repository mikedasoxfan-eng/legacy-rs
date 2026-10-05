//! Literal inclusive bounding-box operations from `StructureBoundingBox`.
#[derive(Clone,Copy,Debug,PartialEq,Eq,Default)]
pub struct BoundingBox {pub min_x:i32,pub min_y:i32,pub min_z:i32,pub max_x:i32,pub max_y:i32,pub max_z:i32}
impl BoundingBox {
 pub fn new(v:[i32;6])->Self {Self{min_x:v[0],min_y:v[1],min_z:v[2],max_x:v[3],max_y:v[4],max_z:v[5]}}
 pub fn values(self)->[i32;6] {[self.min_x,self.min_y,self.min_z,self.max_x,self.max_y,self.max_z]}
 pub fn empty()->Self {Self::new([i32::MAX,i32::MAX,i32::MAX,i32::MIN,i32::MIN,i32::MIN])}
 pub fn region(x:i32,z:i32,x1:i32,z1:i32)->Self {Self::new([x,0,z,x1,65536,z1])}
 pub fn oriented(x:i32,y:i32,z:i32,dx:i32,dy:i32,dz:i32,sx:i32,sy:i32,sz:i32,dir:i32)->Self {
  // Every arithmetic operation is a Java int operation.
  let a=|x:i32,y:i32|x.wrapping_add(y);let sub=|x:i32,y:i32|x.wrapping_sub(y);
  let ymax=a(a(y,sy.wrapping_sub(1)),dy);
  match dir {
   1=>Self::new([a(a(sub(x,sz),1),dz),a(y,dy),a(z,dx),a(x,dz),ymax,a(a(z,sx.wrapping_sub(1)),dx)]),
   2=>Self::new([a(x,dx),a(y,dy),a(a(sub(z,sz),1),dz),a(a(x,sx.wrapping_sub(1)),dx),ymax,a(z,dz)]),
   3=>Self::new([a(x,dz),a(y,dy),a(z,dx),a(a(x,sz.wrapping_sub(1)),dz),ymax,a(a(z,sx.wrapping_sub(1)),dx)]),
   _=>Self::new([a(x,dx),a(y,dy),a(z,dz),a(a(x,sx.wrapping_sub(1)),dx),ymax,a(a(z,sz.wrapping_sub(1)),dz)])
  }
 }
 pub fn intersects(self,b:Self)->bool {self.max_x>=b.min_x&&self.min_x<=b.max_x&&self.max_z>=b.min_z&&self.min_z<=b.max_z&&self.max_y>=b.min_y&&self.min_y<=b.max_y}
 pub fn intersects_region(self,x:i32,z:i32,x1:i32,z1:i32)->bool {self.max_x>=x&&self.min_x<=x1&&self.max_z>=z&&self.min_z<=z1}
 pub fn encompass(&mut self,b:Self) {self.min_x=self.min_x.min(b.min_x);self.min_y=self.min_y.min(b.min_y);self.min_z=self.min_z.min(b.min_z);self.max_x=self.max_x.max(b.max_x);self.max_y=self.max_y.max(b.max_y);self.max_z=self.max_z.max(b.max_z);}
 pub fn offset(&mut self,x:i32,y:i32,z:i32) {self.min_x=self.min_x.wrapping_add(x);self.max_x=self.max_x.wrapping_add(x);self.min_y=self.min_y.wrapping_add(y);self.max_y=self.max_y.wrapping_add(y);self.min_z=self.min_z.wrapping_add(z);self.max_z=self.max_z.wrapping_add(z);}
 pub fn contains(self,x:i32,y:i32,z:i32)->bool {x>=self.min_x&&x<=self.max_x&&z>=self.min_z&&z<=self.max_z&&y>=self.min_y&&y<=self.max_y}
 pub fn sizes(self)->[i32;3] {[self.max_x.wrapping_sub(self.min_x).wrapping_add(1),self.max_y.wrapping_sub(self.min_y).wrapping_add(1),self.max_z.wrapping_sub(self.min_z).wrapping_add(1)]}
}
impl std::fmt::Display for BoundingBox {fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result {write!(f,"({}, {}, {}; {}, {}, {})",self.min_x,self.min_y,self.min_z,self.max_x,self.max_y,self.max_z)}}
