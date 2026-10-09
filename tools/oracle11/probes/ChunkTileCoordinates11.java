package net.minecraft.src;
import java.io.*;import java.lang.reflect.*;import java.util.*;
public final class ChunkTileCoordinates11 {
 static Field field(String name)throws Exception{Field f=World.class.getDeclaredField(name);f.setAccessible(true);return f;}
 public static void main(String[] a)throws Exception{
  Block.stone.getClass();PrintWriter out=new PrintWriter(a[0]);
  for(int[] pos:new int[][]{{-33,-17},{-17,16},{-1,-1},{0,0},{15,15},{16,33}})for(int block:new int[]{23,25,36,52,54,61,63,117})for(int scan=0;scan<2;scan++)for(int remote=0;remote<2;remote++)for(int meta=0;meta<2;meta++)for(int setter=0;setter<2;setter++){
   World w=FirstPersonOracle.world();w.multiplayerWorld=remote!=0;field("scanningTileEntities").setBoolean(w,scan!=0);boolean changed=setter!=0?w.setBlockAndMetadata(pos[0],70,pos[1],block,meta):w.setBlock(pos[0],70,pos[1],block);
   out.print(pos[0]+" "+pos[1]+" "+block+" "+scan+" "+remote+" "+meta+" "+setter+" "+(changed?1:0));
   for(String name:new String[]{"loadedTileEntityList","addedTileEntityList"}){List list=(List)field(name).get(w);out.print(" "+list.size());for(Object q:list){TileEntity t=(TileEntity)q;out.print(" "+t.xCoord+" "+t.yCoord+" "+t.zCoord);}}
   out.println();
  }out.close();System.out.println("ChunkTileCoordinates11:768 original both setter ownership cases");
 }
}
