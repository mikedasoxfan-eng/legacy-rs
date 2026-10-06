package net.minecraft.src;
import java.io.*;
// A saved 1.0 world (original code) with a lit Nether portal around the spawn point, for
// play-testing dimension travel in the Rust client. Usage: portalworld <seed> <wrand> <dir>
public final class Mode_portalworld {
 public static void main(String[] a)throws Exception {
  long seed=Long.parseLong(a[1]);String dir=a[3];
  World w=new World(Oracle.presetSaveHandler(dir,seed),new File(dir).getName(),Oracle.settings(seed));
  w.rand.setSeed(Long.parseLong(a[2]));w.getInitialSpawnLocation();ChunkCoordinates sp=w.getSpawnPoint();
  for(int dx=-128;dx<=128;dx+=16)for(int dz=-128;dz<=128;dz+=16){w.getBlockId(sp.posX+dx,64,sp.posZ+dz);Oracle.drainLighting(w);}
  int x=sp.posX,z=sp.posZ,y=w.getHeightValue(x,z);
  for(int i=-1;i<=2;i++)for(int j=-1;j<=3;j++){
   boolean frame=i==-1||i==2||j==-1||j==3;
   w.setBlock(x+i,y+j,z,frame?Block.obsidian.blockID:Block.portal.blockID);
   for(int k=-1;k<=1;k+=2)if(!frame&&w.getBlockId(x+i,y+j,z+k)!=0)w.setBlock(x+i,y+j,z+k,0);
  }
  Oracle.drainLighting(w);w.saveWorld(true,null);
  System.out.println("portal at "+x+" "+y+" "+z);
 }
}
