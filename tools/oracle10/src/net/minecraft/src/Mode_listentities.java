package net.minecraft.src;
import java.io.*;
// Debug: load a save with the original code and list its entities. Usage: listentities <dir>
public final class Mode_listentities {
 public static void main(String[] a)throws Exception {
  World w=new World(Oracle.saveHandler(a[1]),new File(a[1]).getName(),Oracle.settings(0L));
  for(File f:new File(a[1],"region").listFiles()){String[] p=f.getName().split("[.]");if(p.length!=4)continue;int rx=Integer.parseInt(p[1]),rz=Integer.parseInt(p[2]);
   try(RandomAccessFile r=new RandomAccessFile(f,"r")){for(int i=0;i<1024;i++)if(r.readInt()!=0)w.getChunkFromChunkCoords(rx*32+(i&31),rz*32+(i>>5));}}
  for(Object o:w.loadedEntityList){Entity e=(Entity)o;int id=w.getBlockId(MathHelper.floor_double(e.posX),MathHelper.floor_double(e.posY),MathHelper.floor_double(e.posZ));
   System.out.printf("%s %.2f %.2f %.2f in=%d%n",EntityList.getEntityString(e),e.posX,e.posY,e.posZ,id);}
 }
}
