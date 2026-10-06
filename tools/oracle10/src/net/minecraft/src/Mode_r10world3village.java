package net.minecraft.src;
import java.io.*;
import java.util.*;
import java.lang.reflect.*;

/** Clipped Release inhabitant construction, professions, IDs, seeds and live membership. */
public final class Mode_r10world3village {
    static final Class[] CLASSES={ComponentVillageHouse4_Garden.class,ComponentVillageChurch.class,ComponentVillageHouse1.class,
        ComponentVillageWoodHut.class,ComponentVillageHall.class,ComponentVillageField.class,ComponentVillageField2.class,
        ComponentVillageHouse2.class,ComponentVillageHouse3.class};
    static final int[][] SIZES={{5,6,5},{5,12,9},{9,9,6},{4,6,5},{9,7,11},{13,4,9},{7,4,9},{10,6,7},{9,7,12}};
    public static void main(String[] a)throws Exception {
        int kind=Integer.parseInt(a[1]);Random r=new Random(Long.parseLong(a[2]));
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[3])))) {
            for(int dir=0;dir<4;dir++) {
                World w=Mode_r10world3.flat(0);BlockSand.fallInstantly=true;OracleHooks.reset(500);
                Field ids=Entity.class.getDeclaredField("nextEntityID");ids.setAccessible(true);ids.setInt(null,100);
                for(int x=-3;x<=3;x++)for(int z=-3;z<=3;z++){
                    Chunk c=w.getChunkFromChunkCoords(x,z);for(int bx=0;bx<16;bx++)for(int bz=0;bz<16;bz++)for(int y=0;y<64;y++)c.blocks[(bx<<11)|(bz<<7)|y]=(byte)(y<61?1:y<63?3:2);c.generateSkylightMap();
                }
                int[] sz=SIZES[kind];StructureBoundingBox bb=StructureBoundingBox.getComponentToAddBoundingBox(0,50,0,0,0,0,sz[0],sz[1],sz[2],dir);
                ComponentVillage p=(ComponentVillage)CLASSES[kind].getConstructor(int.class,Random.class,StructureBoundingBox.class,int.class).newInstance(7,r,bb,dir);
                o.writeLong(EntityOracle.rngState(r));
                for(StructureBoundingBox clip:new StructureBoundingBox[]{new StructureBoundingBox(-32,0,-32,2,127,47),new StructureBoundingBox(3,0,-32,47,127,47),new StructureBoundingBox(-32,0,-32,47,127,47)}) {
                    o.writeBoolean(p.addComponentParts(w,r,clip));o.writeLong(EntityOracle.rngState(r));o.writeLong(EntityOracle.rngState(w.rand));
                    o.writeInt((Integer)Mode_structures10.field(p,"field_39009_a"));Mode_terrainutils10.box(o,p.getBoundingBox());o.writeInt(ids.getInt(null));o.writeLong(OracleHooks.nextSeed());
                    o.writeInt(w.loadedEntityList.size());for(Object obj:w.loadedEntityList){EntityVillager e=(EntityVillager)obj;o.writeInt(e.entityId);o.writeInt((Integer)Mode_structures10.field(e,"field_40141_a"));o.writeDouble(e.posX);o.writeDouble(e.posY);o.writeDouble(e.posZ);o.writeFloat(e.rotationYaw);o.writeLong(EntityOracle.rngState(e.rand));o.writeBoolean(e.addedToChunk);o.writeInt(e.chunkCoordX);o.writeInt(e.chunkCoordY);o.writeInt(e.chunkCoordZ);}
                    Mode_r10world3structures.blocks(o,w);
                }
            }
        }
    }
}
