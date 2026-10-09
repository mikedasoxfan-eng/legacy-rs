package net.minecraft.src;
import java.util.*;import java.io.*;
public final class Mode_structurestarts10 {
 static final class Piece extends StructureComponent {
  Piece(int id,StructureBoundingBox box){super(id);boundingBox=box;}
  public boolean addComponentParts(World w,Random r,StructureBoundingBox box){return r.nextBoolean();}
 }
 public static void main(String[] a)throws Exception {
  World w=Oracle.newWorld(0);Random r=new Random(Long.parseLong(a[1]));try(DataOutputStream o=new DataOutputStream(new FileOutputStream(a[2]))) {
   for(int k=0;k<64;k++) {
    StructureStart s=new StructureStart(){};
    for(int id=0;id<4;id++){int x=r.nextInt(32)-16,y=r.nextInt(32),z=r.nextInt(32)-16;int dx=r.nextInt(8),dy=r.nextInt(8),dz=r.nextInt(8);s.components.add(new Piece(id,new StructureBoundingBox(x,y,z,x+dx,y+dy,z+dz)));}
    s.updateBoundingBox();Mode_terrainutils10.box(o,s.getBoundingBox());s.markAvailableHeight(w,r,k%17);Mode_terrainutils10.box(o,s.getBoundingBox());Trace10.writeBoolean(o, "s.isSizeableStructure()", s.isSizeableStructure());
    s.generateStructure(w,r,new StructureBoundingBox(-8,0,-8,8,127,8));Trace10.writeInt(o, "s.components.size()", s.components.size());for(Object obj:s.components){StructureComponent p=(StructureComponent)obj;Trace10.writeInt(o, "p.func_35012_c()", p.getComponentType());Mode_terrainutils10.box(o,p.getBoundingBox());}Trace10.writeLong(o, "EntityOracle.rngState(r)", EntityOracle.rngState(r));
   }
  }
 }
}
