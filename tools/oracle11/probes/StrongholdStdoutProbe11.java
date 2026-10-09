package net.minecraft.src;
import java.io.*;
import java.lang.reflect.*;
public final class StrongholdStdoutProbe11 {
    public static void main(String[] a)throws Exception {
        long seed=Long.parseLong(a[0]);World[] worlds=new World[2];int at=0;
        for(final EnumWorldType type:EnumWorldType.values()) {
            final WorldSettings settings=new WorldSettings(seed,0,false,false,type);
            worlds[at++]=new World(new Oracle.NullSaveHandler(seed){public WorldInfo loadWorldInfo(){return new WorldInfo(settings,"oracle");}},"oracle",settings);
        }
        Field coords=MapGenStronghold.class.getDeclaredField("structureCoords");coords.setAccessible(true);
        PrintStream original=System.out;
        try(DataOutputStream o=new DataOutputStream(new FileOutputStream(a[1]));PrintStream capture=new PrintStream(new FileOutputStream(a[2]),true,"UTF-8")) {
            System.setOut(capture);
            for(World w:worlds) {
                MapGenStronghold g=new MapGenStronghold();g.worldObj=w;g.rand.setSeed(777L);
                o.writeBoolean(g.canSpawnStructureAtCoords(0,0));o.writeLong(EntityOracle.rngState(g.rand));
                ChunkCoordIntPair[] c=(ChunkCoordIntPair[])coords.get(g);
                for(ChunkCoordIntPair p:c){o.writeInt(p.chunkXPos);o.writeInt(p.chunkZPos);}
                for(int[] q:new int[][]{{c[0].chunkXPos,c[0].chunkZPos},{c[0].chunkXPos,c[0].chunkZPos},{c[1].chunkXPos,c[1].chunkZPos},{c[1].chunkXPos+1,c[1].chunkZPos},{c[2].chunkXPos,c[2].chunkZPos},{0,0},{c[2].chunkXPos,c[2].chunkZPos},{c[1].chunkXPos,c[1].chunkZPos},{c[0].chunkXPos,c[0].chunkZPos}}) {
                    o.writeBoolean(g.canSpawnStructureAtCoords(q[0],q[1]));o.writeLong(EntityOracle.rngState(g.rand));
                }
                MapGenStronghold map=new MapGenStronghold();
                for(int i:new int[]{2,2,0,0,1}) {map.generate(w.getIChunkProvider(),w,c[i].chunkXPos,c[i].chunkZPos,new byte[32768]);o.writeLong(EntityOracle.rngState(map.rand));o.writeInt(map.coordMap.size());}
                MapGenStronghold locator=new MapGenStronghold();
                for(int[] q:new int[][]{{c[0].chunkXPos*16,64,c[0].chunkZPos*16},{c[0].chunkXPos*16,64,c[0].chunkZPos*16},{0,64,0},{c[2].chunkXPos*16,64,c[2].chunkZPos*16}}) {
                    ChunkPosition p=locator.func_40484_a(w,q[0],q[1],q[2]);o.writeBoolean(p!=null);if(p!=null){o.writeInt(p.x);o.writeInt(p.y);o.writeInt(p.z);}o.writeInt(locator.coordMap.size());
                }
            }
        } finally {System.setOut(original);}
    }
}
