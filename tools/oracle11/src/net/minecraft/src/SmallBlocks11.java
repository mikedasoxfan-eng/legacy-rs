package net.minecraft.src;
import java.io.*;import java.lang.reflect.*;import java.util.*;
public final class SmallBlocks11 {
 static final class W extends World {
  Map<String,int[]> blocks;boolean power;int sounds,schedules;EntityMinecart cart;AxisAlignedBB queried;
  W(){super((ISaveHandler)null,"",new WorldSettings(0L,0,false,false,EnumWorldType.DEFAULT));}
  String key(int x,int y,int z){return x+","+y+","+z;}
  void put(int x,int y,int z,int id,int m){blocks.put(key(x,y,z),new int[]{id,m});}
  public int getBlockId(int x,int y,int z){int[] b=blocks.get(key(x,y,z));return b==null?0:b[0];}
  public int getBlockMetadata(int x,int y,int z){int[] b=blocks.get(key(x,y,z));return b==null?0:b[1];}
  public Material getBlockMaterial(int x,int y,int z){int id=getBlockId(x,y,z);return id==0?Material.air:Block.blocksList[id].blockMaterial;}
  public boolean isBlockNormalCube(int x,int y,int z){return getBlockId(x,y,z)==1;}
  public boolean isBlockIndirectlyGettingPowered(int x,int y,int z){return power;}
  public void setBlockMetadataWithNotify(int x,int y,int z,int m){put(x,y,z,getBlockId(x,y,z),m);}
  public boolean setBlockWithNotify(int x,int y,int z,int id){put(x,y,z,id,0);return true;}
  public void notifyBlocksOfNeighborChange(int x,int y,int z,int id){}
  public void markBlocksDirty(int a,int b,int c,int d,int e,int f){}
  public void scheduleBlockUpdate(int x,int y,int z,int id,int delay){schedules++;}
  public void playAuxSFXAtEntity(EntityPlayer p,int code,int x,int y,int z,int data){sounds++;}
  public int getBlockLightValue(int x,int y,int z){return 15;}
  public int getFullBlockLightValue(int x,int y,int z){return 15;}
  public boolean canBlockSeeTheSky(int x,int y,int z){return true;}
  public List getEntitiesWithinAABB(Class cls,AxisAlignedBB box){queried=box;return cart!=null&&box.intersectsWith(cart.boundingBox)?Collections.singletonList(cart):Collections.emptyList();}
 }
 static W world()throws Exception{W w=(W)SheepSlice11.unsafe().allocateInstance(W.class);w.blocks=new HashMap<String,int[]>();w.rand=new Random(42);w.worldHeight=128;return w;}
 static long rng(Random r)throws Exception{Field f=Random.class.getDeclaredField("seed");f.setAccessible(true);return((java.util.concurrent.atomic.AtomicLong)f.get(r)).get();}
 public static void main(String[] args)throws Exception{
  int boot=Block.stone.blockID;File dir=new File(args[0]);PrintWriter out=new PrintWriter(new File(dir,"snow11.txt"));
  for(int below:new int[]{0,1,18,20,44,60,78,79,85,107})for(int fancy=0;fancy<2;fancy++){((BlockLeaves)Block.leaves).setGraphicsLevel(fancy!=0);W w=world();w.put(0,63,0,below,0);w.put(0,64,0,78,0);boolean can=Block.snow.canPlaceBlockAt(w,0,64,0);Block.snow.onNeighborBlockChange(w,0,64,0,below);out.println(below+" "+fancy+" "+(can?1:0)+" "+w.getBlockId(0,64,0)+" "+rng(w.rand));}out.close();
  out=new PrintWriter(new File(dir,"stem11.txt"));for(int id:new int[]{104,105})for(int soil:new int[]{0,1,2,3,60})for(int adjacent=0;adjacent<2;adjacent++)for(int seed=0;seed<100;seed++){
   W w=world();for(int x=-1;x<=1;x++)for(int z=-1;z<=1;z++)w.put(x,63,z,soil,0);w.put(0,63,0,60,7);w.put(0,64,0,id,7);if(adjacent!=0)w.put(-1,64,0,id==104?86:103,0);Random r=w.rand;r.setSeed(seed);Block.blocksList[id].updateTick(w,0,64,0,r);out.print(id+" "+soil+" "+adjacent+" "+seed);for(int[] p:new int[][]{{-1,0},{1,0},{0,-1},{0,1}})out.print(" "+w.getBlockId(p[0],64,p[1]));out.println(" "+rng(r)+" "+rng(w.rand));
  }out.close();
  out=new PrintWriter(new File(dir,"trapdoor11.txt"));for(int meta=0;meta<16;meta++)for(int changed:new int[]{0,1,69,93,96})for(int power=0;power<2;power++)for(int remote=0;remote<2;remote++){
   W w=world();int[] p=new int[][]{{0,1},{0,-1},{1,0},{-1,0}}[meta&3];w.put(p[0],64,p[1],1,0);w.put(0,64,0,96,meta);w.power=power!=0;w.multiplayerWorld=remote!=0;Block.trapdoor.onNeighborBlockChange(w,0,64,0,changed);out.println(meta+" "+changed+" "+power+" "+remote+" "+w.getBlockMetadata(0,64,0)+" "+w.sounds+" "+rng(w.rand));
  }out.close();
  Method update=BlockDetectorRail.class.getDeclaredMethod("setStateIfMinecartInteractsWithRail",World.class,int.class,int.class,int.class,int.class);update.setAccessible(true);out=new PrintWriter(new File(dir,"detector11.txt"));
  for(int x:new int[]{0,-19,16777217})for(int meta:new int[]{0,8})for(double dy:new double[]{-.125,0,.25,.5,.75,.874999999,.875,1}){
   W w=world();w.put(x,64,x,28,meta);w.cart=new EntityMinecart(w);w.cart.setPosition(x+.5,64+dy+.0625,x+.5);w.cart.boundingBox.setBounds(x+.25,64+dy,x+.25,x+.75,64+dy+.125,x+.75);update.invoke(Block.railDetector,w,x,64,x,meta);out.print(x+" "+meta+" "+Double.doubleToRawLongBits(dy)+" "+w.getBlockMetadata(x,64,x)+" "+w.schedules);for(double v:new double[]{w.queried.minX,w.queried.minY,w.queried.minZ,w.queried.maxX,w.queried.maxY,w.queried.maxZ})out.print(" "+Double.doubleToRawLongBits(v));out.println(" "+rng(w.rand));
  }out.close();
 }
}
