package net.minecraft.src;
import java.io.*;import java.lang.reflect.*;import java.util.*;
public final class DoorSlice11 {
 static final class W extends World {
  Map<String,int[]> blocks;boolean power;int sounds;
  W(){super((ISaveHandler)null,"",new WorldSettings(0L,0,false,false,EnumWorldType.DEFAULT));}
  String key(int x,int y,int z){return x+","+y+","+z;}
  void put(int x,int y,int z,int id,int meta){blocks.put(key(x,y,z),new int[]{id,meta});}
  public int getBlockId(int x,int y,int z){int[] b=blocks.get(key(x,y,z));return b==null?0:b[0];}
  public int getBlockMetadata(int x,int y,int z){int[] b=blocks.get(key(x,y,z));return b==null?0:b[1];}
  public boolean isBlockNormalCube(int x,int y,int z){return getBlockId(x,y,z)==1;}
  public boolean isBlockIndirectlyGettingPowered(int x,int y,int z){return power;}
  public boolean setBlockAndMetadataWithNotify(int x,int y,int z,int id,int meta){put(x,y,z,id,meta);notifyBlocksOfNeighborChange(x,y,z,id);return true;}
  public void setBlockMetadataWithNotify(int x,int y,int z,int meta){put(x,y,z,getBlockId(x,y,z),meta);}
  public boolean setBlockWithNotify(int x,int y,int z,int id){put(x,y,z,id,0);notifyBlocksOfNeighborChange(x,y,z,id);return true;}
  public void markBlocksDirty(int a,int b,int c,int d,int e,int f){}
  public void playAuxSFXAtEntity(EntityPlayer p,int code,int x,int y,int z,int data){sounds++;}
  public void notifyBlocksOfNeighborChange(int x,int y,int z,int id){if(editingBlocks)return;for(int[] d:new int[][]{{-1,0,0},{1,0,0},{0,-1,0},{0,1,0},{0,0,-1},{0,0,1}}){int a=x+d[0],b=y+d[1],c=z+d[2];if(getBlockId(a,b,c)==64)Block.doorWood.onNeighborBlockChange(this,a,b,c,id);}}
 }
 static W world()throws Exception{Field f=sun.misc.Unsafe.class.getDeclaredField("theUnsafe");f.setAccessible(true);W w=(W)((sun.misc.Unsafe)f.get(null)).allocateInstance(W.class);w.blocks=new HashMap<String,int[]>();w.rand=new Random(42);w.put(0,63,0,1,0);return w;}
 public static void main(String[] args)throws Exception{
  int boot=Block.stone.blockID;File dir=new File(args[0]);PrintWriter out=new PrintWriter(new File(dir,"doors11.txt"));
  for(int dirn=0;dirn<4;dirn++)for(int left=0;left<3;left++)for(int right=0;right<3;right++)for(int doors=0;doors<4;doors++){
   W w=world();int dx=dirn==1?-1:dirn==3?1:0,dz=dirn==0?1:dirn==2?-1:0;
   for(int i=0;i<left;i++)w.put(-dx,64+i,-dz,1,0);for(int i=0;i<right;i++)w.put(dx,64+i,dz,1,0);
   if((doors&1)!=0)w.put(-dx,64,-dz,64,0);if((doors&2)!=0)w.put(dx,64,dz,64,0);
   // Adjacent doors have valid supports/upper halves, matching real structures.
   if((doors&1)!=0){w.put(-dx,63,-dz,1,0);w.put(-dx,65,-dz,64,8);}if((doors&2)!=0){w.put(dx,63,dz,1,0);w.put(dx,65,dz,64,8);}
   ItemDoor.placeDoorBlock(w,0,64,0,dirn,Block.doorWood);
   out.println("0 "+dirn+" "+left+" "+right+" "+doors+" "+w.getBlockMetadata(0,64,0)+" "+w.getBlockMetadata(0,65,0)+" "+w.sounds+" "+w.rand.nextLong());
  }
  for(int meta=0;meta<8;meta++)for(int changed:new int[]{0,1,64,69,93})for(int power=0;power<2;power++)for(int upper=0;upper<2;upper++){
   W w=world();w.put(0,64,0,64,meta);w.put(0,65,0,64,meta+8);w.power=power==1;Block.doorWood.onNeighborBlockChange(w,0,64+upper,0,changed);
   out.println("1 "+meta+" "+changed+" "+power+" "+upper+" "+w.getBlockMetadata(0,64,0)+" "+w.getBlockMetadata(0,65,0)+" "+w.sounds+" "+w.rand.nextLong());
  }out.close();
 }
}
