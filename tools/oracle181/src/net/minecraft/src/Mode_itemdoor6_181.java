package net.minecraft.src;
import java.io.*;
public final class Mode_itemdoor6_181 {
 static void raw(World w,int x,int y,int z,int b){if(y>=0&&y<128)Mode_entityround6_181.raw(w,x,y,z,b);}
 static void scene(World w,int y,int id,int scene){raw(w,0,y,0,scene==0?0:1);if(scene==2)raw(w,0,y+2,0,1);if(scene==3){raw(w,0,y+1,1,id);raw(w,1,y+1,0,id);}if(scene==4){raw(w,-1,y+1,0,1);raw(w,-1,y+2,0,1);}if(scene==5){raw(w,0,y+1,0,106);raw(w,0,y+2,0,31);}}
 static void snapshot(World w,int y,DataOutputStream o)throws Exception{for(int x=-1;x<=1;x++)for(int z=-1;z<=1;z++)for(int yy=y;yy<=y+3;yy++){o.writeByte(w.getBlockId(x,yy,z));EntityOracle.i(o,w.getBlockMetadata(x,yy,z));}o.writeBoolean(w.editingBlocks);WorldEventOracle.snapshot(w,o);}
 public static void main(String[] a)throws Exception {
  try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[3])))){
   if(a[1].equals("doorhelper6")){for(int id:new int[]{64,71})for(int scene=0;scene<6;scene++)for(int dir:new int[]{Integer.MIN_VALUE,-1,0,1,2,3,4,Integer.MAX_VALUE}){
    World w=Mode_entities181.world();scene(w,63,id,scene);ItemDoor.func_35434_a(w,0,64,0,dir,Block.blocksList[id]);snapshot(w,63,o);
   }}else{for(int id:new int[]{324,330})for(int scene=0;scene<6;scene++)for(int y:new int[]{-1,0,63,125,126,127,128})for(int side:new int[]{-1,0,1,2,5,6})for(float yaw:new float[]{-450,-90,0,90,180,450})for(int count:new int[]{Integer.MIN_VALUE,0,1}){
    World w=Mode_entities181.world();EntityPlayer p=new Mode_entities181.Player(w);p.rotationYaw=yaw;scene(w,y,id==324?64:71,scene);ItemStack stack=new ItemStack(id,count,0);o.writeBoolean(stack.getItem().onItemUse(stack,p,w,0,y,0,side));EntityOracle.i(o,stack.stackSize);snapshot(w,y,o);
   }}
  }
 }
}
