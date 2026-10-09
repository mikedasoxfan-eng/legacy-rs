package net.minecraft.src;
import java.io.*;
public final class BlockTint11 {
 static int color(int x,int y,int z,int seed,boolean foliage){return (x*733+z*1237+y*19+seed*73471+(foliage?0x619fe2:0x325711))&0xffffff;}
 static class B extends BiomeGenBase {int seed;B(){super(255);}public int getGrassColorAtCoords(IBlockAccess w,int x,int y,int z){return color(x,y,z,seed,false);}public int getFoliageColorAtCoords(IBlockAccess w,int x,int y,int z){return color(x,y,z,seed,true);}}
 static class C extends WorldChunkManager {B b;StringBuilder queries=new StringBuilder();public BiomeGenBase getBiomeGenAt(int x,int z){queries.append(" ").append(x).append(" ").append(z);b.waterColorMultiplier=color(x,0,z,b.seed,false);return b;}}
 static class W extends World {C c;int meta;W(){super((ISaveHandler)null,"",new WorldSettings(0L,0,false,false,EnumWorldType.DEFAULT));}public WorldChunkManager getWorldChunkManager(){return c;}public int getBlockMetadata(int x,int y,int z){return meta;}}
 public static void main(String[]args)throws Exception{int boot=Block.stone.blockID;PrintWriter o=new PrintWriter(new File(args[0],"blocktint11.txt"));B b=new B();
  for(int id:new int[]{2,8,9,10,11,18,31,106})for(int meta=0;meta<16;meta++)for(int seed=0;seed<20;seed++)for(int x:new int[]{-17,-1,0,17,Integer.MAX_VALUE}){W w=(W)SheepSlice11.unsafe().allocateInstance(W.class);w.c=new C();w.c.b=b;b.seed=seed;w.meta=meta;int z=x==Integer.MAX_VALUE?Integer.MIN_VALUE:-x;int tint=Block.blocksList[id].colorMultiplier(w,x,64,z);o.println(id+" "+meta+" "+seed+" "+x+" "+z+" "+tint+w.c.queries);}
  o.close();
 }
}
