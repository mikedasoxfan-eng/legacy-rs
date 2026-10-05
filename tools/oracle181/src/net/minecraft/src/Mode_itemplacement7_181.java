package net.minecraft.src;
import java.io.*;
/** Original ItemBed/ItemSign callbacks, including failed World writes and editor calls. */
public final class Mode_itemplacement7_181 {
 static final class Player extends EntityPlayer {
  int edits; TileEntitySign edited;
  Player(World w){super(w);} public void func_6420_o(){}
  public void displayGUIEditSign(TileEntitySign s){edits++;edited=s;}
 }
 static void raw(World w,int x,int y,int z,int b){if(y>=0&&y<128)Mode_entityround6_181.raw(w,x,y,z,b);}
 static void scene(World w,int y,int scene){
  for(int x=-2;x<=2;x++)for(int z=-2;z<=2;z++)raw(w,x,y,z,scene==0?0:scene==1?1:scene==2?20:scene==3?44:scene==4?9:1);
  if(scene==5)raw(w,0,y+1,0,1);
  if(scene==6)for(int x=-2;x<=2;x++)for(int z=-2;z<=2;z++)if(x!=0||z!=0)raw(w,x,y+1,z,1);
  if(scene==7)raw(w,0,y+1,0,106);
 }
 static void snapshot(World w,int y,Player p,DataOutputStream o)throws Exception{
  for(int x=-2;x<=2;x++)for(int z=-2;z<=2;z++)for(int yy=y-1;yy<=y+2;yy++){o.writeByte(w.getBlockId(x,yy,z));EntityOracle.i(o,w.getBlockMetadata(x,yy,z));}
  EntityOracle.i(o,p.edits);
  if(p.edited!=null){EntityOracle.i(o,p.edited.xCoord);EntityOracle.i(o,p.edited.yCoord);EntityOracle.i(o,p.edited.zCoord);for(String s:p.edited.signText)WorldEventOracle.text(o,s);java.lang.reflect.Field f=TileEntitySign.class.getDeclaredField("isEditable");f.setAccessible(true);o.writeBoolean(f.getBoolean(p.edited));}
  EntityOracle.l(o,EntityOracle.rngState(w.rand));WorldEventOracle.snapshot(w,o);
 }
 public static void main(String[] a)throws Exception{
  int id=a[1].equals("beditem7")?355:323;
  try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[3])))){
   for(int scene=0;scene<8;scene++)for(int y:new int[]{-2,-1,0,63,125,126,127,128})for(int side:new int[]{-1,0,1,2,3,4,5,6})for(float yaw:new float[]{-450,-90,0,90,180,450})for(int count:new int[]{Integer.MIN_VALUE,-1,0,1,Integer.MAX_VALUE})for(boolean remote:new boolean[]{false,true}){
    World w=Mode_entities181.world();w.multiplayerWorld=remote;Player p=new Player(w);p.rotationYaw=yaw;scene(w,y,scene);
    // ItemSign intentionally ignores entity placement collision.
    EntityPig pig=new EntityPig(w);pig.setPosition(.5,y+1,.5);w.loadedEntityList.add(pig);
    ItemStack stack=new ItemStack(id,count,0);o.writeBoolean(stack.getItem().onItemUse(stack,p,w,0,y,0,side));EntityOracle.i(o,stack.stackSize);snapshot(w,y,p,o);
   }
  }
 }
}
