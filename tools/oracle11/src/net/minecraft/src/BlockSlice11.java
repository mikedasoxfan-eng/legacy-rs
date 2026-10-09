package net.minecraft.src;
import java.io.*;import java.util.*;
public final class BlockSlice11 {
 static final class C extends EntityChicken {C(World w){super(w);}void land(double d,boolean g){super.updateFallState(d,g);}}
 static void box(PrintWriter out,AxisAlignedBB b){if(b==null){out.print(" 0");return;}out.print(" 1");for(double d:new double[]{b.minX,b.minY,b.minZ,b.maxX,b.maxY,b.maxZ})out.print(" "+Double.doubleToRawLongBits(d));}
 public static void main(String[] args)throws Exception{
  int boot=Block.stone.blockID;File dir=new File(args[0]);PrintWriter out=new PrintWriter(new File(dir,"blockboxes11.txt"));
  for(int id:new int[]{65,107})for(int meta=0;meta<16;meta++)for(int x:new int[]{0,-19,16777217}){
   DoorSlice11.W w=DoorSlice11.world();w.put(x,64,x,id,meta);Block b=Block.blocksList[id];out.print(id+" "+meta+" "+x);box(out,b.getCollisionBoundingBoxFromPool(w,x,64,x));b.setBlockBoundsBasedOnState(w,x,64,x);box(out,b.getSelectedBoundingBoxFromPool(w,x,64,x));out.println();
  }out.close();
  out=new PrintWriter(new File(dir,"farmland11.txt"));
  for(long seed:new long[]{0,1,42,-1,341873128712L})for(float distance:new float[]{-1F,0F,.5F,Float.intBitsToFloat(0x3f000001),1F,1.5F,3F,10F})for(int ground=0;ground<2;ground++)for(int living=0;living<2;living++){
   DoorSlice11.W w=DoorSlice11.world();w.put(0,63,0,60,7);Entity c=living==1?new C(w):new EntityItem(w,0,64,0,new ItemStack(1,1,0));c.yOffset=0;c.setPosition(0,64,0);c.fallDistance=distance;w.rand.setSeed(seed);c.updateFallState(-.25D,ground==1);out.println(seed+" "+Float.floatToRawIntBits(distance)+" "+ground+" "+w.getBlockId(0,63,0)+" "+Float.floatToRawIntBits(c.fallDistance)+" "+w.rand.nextLong()+" "+living);
  }out.close();
  out=new PrintWriter(new File(dir,"farmwalking11.txt"));for(long seed:new long[]{0,1,42,-1,341873128712L}){DoorSlice11.W w=DoorSlice11.world();w.put(0,63,0,60,7);C c=new C(w);w.rand.setSeed(seed);Block.tilledField.onEntityWalking(w,0,63,0,c);out.println(seed+" "+w.getBlockId(0,63,0)+" "+w.rand.nextLong());}out.close();
  out=new PrintWriter(new File(dir,"gate11.txt"));
  for(int meta=0;meta<16;meta++)for(int changed:new int[]{0,1,64,69,93})for(int power=0;power<2;power++)for(int remote=0;remote<2;remote++){
   DoorSlice11.W w=DoorSlice11.world();w.put(0,64,0,107,meta);w.power=power==1;w.multiplayerWorld=remote==1;Block.fenceGate.onNeighborBlockChange(w,0,64,0,changed);out.println(meta+" "+changed+" "+power+" "+remote+" "+w.getBlockMetadata(0,64,0)+" "+w.sounds+" "+w.rand.nextLong());
  }out.close();
  out=new PrintWriter(new File(dir,"railadded11.txt"));
  for(int meta=0;meta<10;meta++)for(int power=0;power<2;power++)for(int remote=0;remote<2;remote++){
   DoorSlice11.W w=DoorSlice11.world();w.put(0,64,0,27,meta);w.power=power==1;w.multiplayerWorld=remote==1;Block.railPowered.onBlockAdded(w,0,64,0);out.println(meta+" "+power+" "+remote+" "+w.getBlockId(0,64,0)+" "+w.getBlockMetadata(0,64,0)+" "+w.rand.nextLong());
  }out.close();
  out=new PrintWriter(new File(dir,"web11.txt"));for(float distance:new float[]{0,1,10,Float.NaN}){DoorSlice11.W w=DoorSlice11.world();C c=new C(w);c.fallDistance=distance;Block.web.onEntityCollidedWithBlock(w,0,64,0,c);out.println(Float.floatToRawIntBits(distance)+" "+Float.floatToRawIntBits(c.fallDistance));}out.close();
 }
}
