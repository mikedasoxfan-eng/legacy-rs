package net.minecraft.src;
import java.util.*;import java.io.*;
public final class Mode_structurestarts181 {
 static final class Piece extends StructureComponent {
  Piece(int id,StructureBoundingBox box){super(id);field_35024_g=box;}
  public boolean func_35023_a(World w,Random r,StructureBoundingBox box){return r.nextBoolean();}
 }
 public static void main(String[] a)throws Exception {
  World w=Oracle.newWorld(0);Random r=new Random(Long.parseLong(a[1]));try(DataOutputStream o=new DataOutputStream(new FileOutputStream(a[2]))) {
   for(int k=0;k<64;k++) {
    StructureStart s=new StructureStart(){};
    for(int id=0;id<4;id++){int x=r.nextInt(32)-16,y=r.nextInt(32),z=r.nextInt(32)-16;int dx=r.nextInt(8),dy=r.nextInt(8),dz=r.nextInt(8);s.field_35717_a.add(new Piece(id,new StructureBoundingBox(x,y,z,x+dx,y+dy,z+dz)));}
    s.func_35714_b();Mode_terrainutils181.box(o,s.func_35712_a());s.func_35713_a(w,r,k%17);Mode_terrainutils181.box(o,s.func_35712_a());o.writeBoolean(s.func_35715_c());
    s.func_35711_a(w,r,new StructureBoundingBox(-8,0,-8,8,127,8));o.writeInt(s.field_35717_a.size());for(Object obj:s.field_35717_a){StructureComponent p=(StructureComponent)obj;o.writeInt(p.func_35012_c());Mode_terrainutils181.box(o,p.func_35021_b());}o.writeLong(EntityOracle.rngState(r));
   }
  }
 }
}
