package net.minecraft.src;
import java.io.*;import java.lang.reflect.*;import java.util.*;
public final class LeavesWire11 {
 static class W extends World {
  HashMap<String,int[]> blocks=new HashMap<String,int[]>();ArrayList<EntityItem> drops=new ArrayList<EntityItem>();ArrayList<String> notifications=new ArrayList<String>();boolean power;int schedules;
  W(){super((ISaveHandler)null,"",new WorldSettings(0L,0,false,false,EnumWorldType.DEFAULT));}
  String key(int x,int y,int z){return x+","+y+","+z;}void put(int x,int y,int z,int id,int meta){blocks.put(key(x,y,z),new int[]{id,meta});}
  public int getBlockId(int x,int y,int z){int[] b=blocks.get(key(x,y,z));return b==null?0:b[0];}
  public int getBlockMetadata(int x,int y,int z){int[] b=blocks.get(key(x,y,z));return b==null?0:b[1];}
  public boolean func_41082_b(int x,int y,int z,boolean fallback){return getBlockId(x,y,z)==1;}
  public boolean isBlockNormalCube(int x,int y,int z){return getBlockId(x,y,z)==1;}
  public boolean isBlockIndirectlyGettingPowered(int x,int y,int z){return power;}
  public void setBlockMetadataWithNotify(int x,int y,int z,int m){put(x,y,z,getBlockId(x,y,z),m);}
  public boolean setBlockWithNotify(int x,int y,int z,int id){put(x,y,z,id,0);return true;}
  public boolean checkChunksExist(int a,int b,int c,int d,int e,int f){return true;}
  public boolean spawnEntityInWorld(Entity e){drops.add((EntityItem)e);return true;}
  public void notifyBlocksOfNeighborChange(int x,int y,int z,int id){notifications.add(x+" "+y+" "+z+" "+id);super.notifyBlocksOfNeighborChange(x,y,z,id);}
  public void scheduleBlockUpdate(int x,int y,int z,int id,int delay){schedules++;}
  public void markBlocksDirty(int a,int b,int c,int d,int e,int f){}
 }
 static W world()throws Exception{W w=(W)SheepSlice11.unsafe().allocateInstance(W.class);w.blocks=new HashMap<String,int[]>();w.drops=new ArrayList<EntityItem>();w.notifications=new ArrayList<String>();w.rand=new Random(42);return w;}
 public static void main(String[] args)throws Exception{
  int boot=Block.stone.blockID;File dir=new File(args[0]);PrintWriter o=new PrintWriter(new File(dir,"leavesdrops11.txt"));
  for(int mode=0;mode<2;mode++)for(int seed=0;seed<400;seed++)for(int meta=0;meta<4;meta++)for(int fortune:new int[]{0,3})for(float chance:new float[]{0F,1F})for(int remote=0;remote<2;remote++){
   if(mode==1&&(fortune!=0||chance!=1))continue;W w=world();w.rand.setSeed(seed);w.multiplayerWorld=remote!=0;w.put(0,64,0,18,meta|8);
   if(mode==0)Block.leaves.dropBlockAsItemWithChance(w,0,64,0,meta,chance,fortune);else Block.leaves.updateTick(w,0,64,0,w.rand);
   o.print(mode+" "+seed+" "+meta+" "+fortune+" "+Float.floatToRawIntBits(chance)+" "+remote+" "+w.getBlockId(0,64,0)+" "+w.drops.size());
   for(EntityItem e:w.drops)o.print(" "+e.item.itemID+" "+e.item.stackSize+" "+e.item.getItemDamage()+" "+Double.doubleToRawLongBits(e.posX)+" "+Double.doubleToRawLongBits(e.posY)+" "+Double.doubleToRawLongBits(e.posZ)+" "+e.delayBeforeCanPickup);
   o.println(" "+SmallBlocks11.rng(w.rand));
  }o.close();o=new PrintWriter(new File(dir,"wireconnect11.txt"));for(int id:new int[]{0,1,55,69,75,76,93,94})for(int meta=0;meta<16;meta++)for(int side=-2;side<=6;side++){W w=world();w.put(0,64,0,id,meta);o.println(id+" "+meta+" "+side+" "+(BlockRedstoneWire.isPowerProviderOrWire(w,0,64,0,side)?1:0));}o.close();
  Field updates=BlockRedstoneWire.class.getDeclaredField("blocksNeedingUpdate");updates.setAccessible(true);Method calc=BlockRedstoneWire.class.getDeclaredMethod("calculateCurrentChanges",World.class,int.class,int.class,int.class,int.class,int.class,int.class);calc.setAccessible(true);
  o=new PrintWriter(new File(dir,"wirerise11.txt"));for(int old=0;old<16;old++)for(int powered=0;powered<2;powered++){W w=world();w.put(0,64,0,55,old);w.power=powered!=0;Set positions=(Set)updates.get(Block.redstoneWire);positions.clear();calc.invoke(Block.redstoneWire,w,0,64,0,0,64,0);o.print(old+" "+powered+" "+w.getBlockMetadata(0,64,0)+" "+positions.size());for(Object p:positions){ChunkPosition v=(ChunkPosition)p;o.print(" "+v.x+" "+v.y+" "+v.z);}o.println(" "+SmallBlocks11.rng(w.rand));}o.close();
  o=new PrintWriter(new File(dir,"wireremove11.txt"));for(int remote=0;remote<2;remote++)for(int torch=0;torch<2;torch++){W w=world();w.multiplayerWorld=remote!=0;if(torch!=0){w.put(2,63,0,1,0);w.put(2,64,0,76,5);}((Set)updates.get(Block.redstoneWire)).clear();Block.redstoneWire.onBlockRemoval(w,0,64,0);o.print(remote+" "+torch+" "+w.notifications.size()+" "+w.schedules);for(String n:w.notifications)o.print(" "+n);o.println(" "+SmallBlocks11.rng(w.rand));}o.close();
 }
}
