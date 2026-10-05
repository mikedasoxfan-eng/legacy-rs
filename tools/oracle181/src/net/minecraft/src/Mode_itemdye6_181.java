package net.minecraft.src;
import java.io.*;import java.util.*;
/** Every ItemDye override, including exceptional damage values and both RNGs. */
public final class Mode_itemdye6_181 {
 static void i(DataOutputStream o,int n)throws Exception{EntityOracle.i(o,n);}
 static void raw(World w,int x,int y,int z,int b,int m){Mode_entityround6_181.raw(w,x,y,z,b);w.getChunkFromChunkCoords(x>>4,z>>4).setBlockMetadata(x&15,y,z&15,m);}
 public static void main(String[] a)throws Exception{int init=Block.stone.blockID;
  try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[3])))){
   if(a[1].equals("dyeproperties6")){
    for(int damage:new int[]{Integer.MIN_VALUE,-32768,-17,-16,-9,-8,-1,0,1,7,8,15,16,17,127,32767,Integer.MAX_VALUE}){
     i(o,Item.dyePowder.getIconFromDamage(damage));String name="",error="";try{name=Item.dyePowder.getItemNameIS(new ItemStack(351,1,damage));}catch(RuntimeException e){error=e.getClass().getSimpleName();}WorldEventOracle.text(o,name);WorldEventOracle.text(o,error);
    }
    for(int c:ItemDye.dyeColors)i(o,c);for(String c:ItemDye.dyeColorNames)WorldEventOracle.text(o,c);
   }else{
    for(int block:new int[]{0,1,2,6,39,40,59,104,105})for(int scene=0;scene<3;scene++)for(int seed=0;seed<4;seed++)for(int damage:new int[]{-1,0,15,16})for(int count:new int[]{Integer.MIN_VALUE,0,1})for(boolean remote:new boolean[]{false,true}){
     World w=Mode_entities181.world();w.multiplayerWorld=remote;EntityPlayer p=new Mode_entities181.Player(w);for(int x=-8;x<=8;x++)for(int z=-8;z<=8;z++)raw(w,x,63,z,block==59||block==104||block==105?60:2,0);
     raw(w,0,64,0,block,seed*2);if(scene==1)raw(w,0,65,0,1,0);if(scene==2)for(int x=-8;x<=8;x++)for(int z=-8;z<=8;z++)raw(w,x,65,z,1,0);
     w.rand.setSeed(seed);Item.itemRand.setSeed(600);ItemStack stack=new ItemStack(351,count,damage);
     o.writeBoolean(Item.dyePowder.onItemUse(stack,p,w,0,64,0,1));i(o,stack.stackSize);EntityOracle.l(o,EntityOracle.rngState(w.rand));EntityOracle.l(o,EntityOracle.rngState(Item.itemRand));EntityOracle.l(o,EntityOracle.rngState(EntityOracle.mathRandom()));
     for(int x=-8;x<=8;x++)for(int z=-8;z<=8;z++)for(int y=62;y<=80;y++){o.writeByte(w.getBlockId(x,y,z));o.writeByte(w.getBlockMetadata(x,y,z));}
     WorldEventOracle.snapshot(w,o);
    }
   }
  }
 }
}
