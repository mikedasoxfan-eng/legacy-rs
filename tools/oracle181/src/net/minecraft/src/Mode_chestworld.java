package net.minecraft.src;
import java.io.*;
// A saved b1.8.1 world (original code) with chests and a sign just south of spawn, for looking
// at tile-entity rendering in the Rust client. Usage: chestworld <seed> <wrand> <dir>
public final class Mode_chestworld {
 public static void main(String[] a)throws Exception {
  long seed=Long.parseLong(a[1]);String dir=a[3];
  World w=new World(Oracle.presetSaveHandler(dir,seed),new File(dir).getName(),Oracle.settings(seed));
  w.rand.setSeed(Long.parseLong(a[2]));w.getInitialSpawnLocation();ChunkCoordinates sp=w.getSpawnPoint();
  for(int dx=-128;dx<=128;dx+=16)for(int dz=-128;dz<=128;dz+=16){w.getBlockId(sp.posX+dx,64,sp.posZ+dz);Oracle.drainLighting(w);}
  int[][] spots={{0,3},{-3,4},{-2,4},{3,4},{3,5}};
  for(int[] s:spots){int x=sp.posX+s[0],z=sp.posZ+s[1];int y=w.getHeightValue(x,z);w.setBlockWithNotify(x,y,z,Block.chest.blockID);}
  int x=sp.posX+1,z=sp.posZ+3,y=w.getHeightValue(x,z);w.setBlockAndMetadataWithNotify(x,y,z,Block.signPost.blockID,8);
  TileEntitySign t=(TileEntitySign)w.getBlockTileEntity(x,y,z);if(t!=null){t.signText[1]="b1.8.1";t.signText[2]="chests";}
  String[] mobs={"Zombie","Spider","Pig"};int[][] sp2={{-1,3},{2,3},{-2,2}};
  for(int i=0;i<3;i++){int sx=sp.posX+sp2[i][0],sz=sp.posZ+sp2[i][1],sy=w.getHeightValue(sx,sz);w.setBlockWithNotify(sx,sy,sz,Block.mobSpawner.blockID);
   TileEntityMobSpawner t2=(TileEntityMobSpawner)w.getBlockTileEntity(sx,sy,sz);if(t2!=null)t2.setMobID(mobs[i]);}
  Oracle.drainLighting(w);w.saveWorld(true,null);
  System.out.println("spawn "+sp.posX+" "+sp.posY+" "+sp.posZ);
 }
}
