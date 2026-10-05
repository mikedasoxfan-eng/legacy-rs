package net.minecraft.src;
import java.io.*;import java.util.*;
/** Original ItemBucket, with controlled ray results plus the real World ray tracer. */
public final class Mode_itembucket7_181 {
 static final class BucketWorld extends World {
  MovingObjectPosition hit;boolean scripted;Vec3D start,end;boolean liquids;final ArrayList<int[]> writes=new ArrayList<int[]>();
  BucketWorld(){super(new Oracle.NullSaveHandler(0),"oracle",Oracle.settings(0));}
  public MovingObjectPosition rayTraceBlocks_do(Vec3D s,Vec3D e,boolean b){start=Vec3D.createVectorHelper(s.xCoord,s.yCoord,s.zCoord);end=Vec3D.createVectorHelper(e.xCoord,e.yCoord,e.zCoord);liquids=b;return scripted?hit:super.rayTraceBlocks_do(s,e,b);}
  public boolean setBlockWithNotify(int x,int y,int z,int id){if(!scripted)return super.setBlockWithNotify(x,y,z,id);writes.add(new int[]{0,x,y,z,id,0});return false;}
  public boolean setBlockAndMetadataWithNotify(int x,int y,int z,int id,int meta){if(!scripted)return super.setBlockAndMetadataWithNotify(x,y,z,id,meta);writes.add(new int[]{1,x,y,z,id,meta});return false;}
 }
 static BucketWorld world(boolean hell,boolean remote)throws Exception {
  BucketWorld w=new BucketWorld();w.chunkProvider=new EntityOracle.FlatProvider(w);w.multiplayerWorld=remote;w.worldProvider.isHellWorld=hell;
  for(Chunk c:((EntityOracle.FlatProvider)w.chunkProvider).chunks.values()){Arrays.fill(c.skylightMap.data,(byte)255);Arrays.fill(c.heightMap,(byte)64);}
  WorldEventOracle.attach(w);OracleHooks.reset(100);EntityOracle.mathRandom().setSeed(200);w.rand.setSeed(300);return w;
 }
 static void configure(EntityPlayer p,int config){
  p.setPosition(3.25,66.5,-2.75);p.prevPosX=-1.25;p.prevPosY=61.25;p.prevPosZ=4.5;p.rotationYaw=35;p.prevRotationYaw=-17;p.rotationPitch=27;p.prevRotationPitch=-65;
  if(config==1){p.prevPosX=1e16;p.prevPosY=-1e16;p.prevPosZ=1e16;p.prevRotationYaw=1e20F;p.prevRotationPitch=-1e20F;}
  if(config==2){p.yOffset=-.75F;p.rotationYaw=-450;p.rotationPitch=-90;}
 }
 static void snapshot(BucketWorld w,ItemStack original,ItemStack result,DataOutputStream o)throws Exception {
  o.writeBoolean(original==result);for(int v:new int[]{result.itemID,result.stackSize,result.getItemDamage()})EntityOracle.i(o,v);
  EntityOracle.i(o,w.writes.size());for(int[] write:w.writes)for(int v:write)EntityOracle.i(o,v);
  for(Vec3D v:new Vec3D[]{w.start,w.end}){EntityOracle.d(o,v.xCoord);EntityOracle.d(o,v.yCoord);EntityOracle.d(o,v.zCoord);}o.writeBoolean(w.liquids);
  EntityOracle.l(o,EntityOracle.rngState(w.rand));EntityOracle.l(o,EntityOracle.rngState(EntityOracle.mathRandom()));
  for(int x=-1;x<=1;x++)for(int z=-1;z<=1;z++)for(int y=63;y<=65;y++){o.writeByte(w.getBlockId(x,y,z));EntityOracle.i(o,w.getBlockMetadata(x,y,z));}
  WorldEventOracle.snapshot(w,o);
 }
 public static void main(String[] a)throws Exception {
  try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[3])))){
   if(a[1].equals("buckethit7")){
    for(int id:new int[]{325,326,327,335})for(int hit=-1;hit<3;hit++)for(int block:new int[]{0,1,8,9,10,11,31,106})for(int meta:new int[]{0,1,15})for(int side:new int[]{-1,0,1,2,3,4,5,6})for(int config=0;config<3;config++)for(boolean creative:new boolean[]{false,true})for(boolean hell:new boolean[]{false,true})for(boolean remote:new boolean[]{false,true}){
     BucketWorld w=world(hell,remote);w.scripted=true;EntityPlayer p=new Mode_entities181.Player(w);configure(p,config);p.field_35212_aW.field_35756_d=creative;
     EntityOracle.rawBlock(w,0,64,0,block,meta);
     w.hit=hit<0?null:hit==0?new MovingObjectPosition(0,64,0,side,Vec3D.createVector(0,64,0)):new MovingObjectPosition(hit==1?new EntityCow(w):new EntityPig(w));
     ItemStack original=new ItemStack(id,config==1?Integer.MIN_VALUE:config==2?0:1,-32769);ItemStack result=original.getItem().onItemRightClick(original,w,p);snapshot(w,original,result,o);
    }
   }else{
    for(int id:new int[]{325,326,327,335})for(int block:new int[]{0,1,8,9,10,11,31,106})for(int meta:new int[]{0,1,15})for(float yaw:new float[]{0,90,180,270,395})for(float pitch:new float[]{-90,0,45,90})for(boolean hell:new boolean[]{false,true})for(boolean creative:new boolean[]{false,true}){
     BucketWorld w=world(hell,false);EntityPlayer p=new Mode_entities181.Player(w);p.setPosition(.5,65,.5);p.prevPosX=-.25;p.prevPosY=64;p.prevPosZ=.25;p.rotationYaw=yaw;p.prevRotationYaw=-17;p.rotationPitch=pitch;p.prevRotationPitch=-65;p.field_35212_aW.field_35756_d=creative;
     for(int x=-1;x<=1;x++)for(int z=-1;z<=1;z++)EntityOracle.rawBlock(w,x,64,z,block,meta);
     ItemStack original=new ItemStack(id,1,17);snapshot(w,original,original.getItem().onItemRightClick(original,w,p),o);
    }
   }
  }
 }
}
