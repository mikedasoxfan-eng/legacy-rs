package net.minecraft.src;
import java.io.*;
import java.util.*;
import java.lang.reflect.*;

/** Real original-bytecode mineshaft starts, pieces, state and clipped block generation. */
public final class Mode_structures10 {
    static void bounds(DataOutputStream out, StructureBoundingBox b) throws Exception {
        Trace10.writeInt(out, "b.minX", b.minX);Trace10.writeInt(out, "b.minY", b.minY);Trace10.writeInt(out, "b.minZ", b.minZ);
        Trace10.writeInt(out, "b.maxX", b.maxX);Trace10.writeInt(out, "b.maxY", b.maxY);Trace10.writeInt(out, "b.maxZ", b.maxZ);
    }
    static Object field(Object o,String n) throws Exception {Class c=o.getClass();while(c!=null){try{Field f=c.getDeclaredField(n);f.setAccessible(true);return f.get(o);}catch(NoSuchFieldException e){c=c.getSuperclass();}}throw new NoSuchFieldException(n);}
    static void pieces(DataOutputStream out, StructureStart s) throws Exception {
        bounds(out,s.getBoundingBox());Trace10.writeInt(out, "s.components.size()", s.components.size());
        for(Object o:s.components) {
            StructureComponent p=(StructureComponent)o;out.writeUTF(p.getClass().getSimpleName());
            Trace10.writeInt(out, "p.field_35026_i", p.field_35026_i);Trace10.writeInt(out, "p.coordBaseMode", p.coordBaseMode);bounds(out,p.getBoundingBox());
            if(p instanceof ComponentMineshaftRoom) {List es=(List)field(p,"field_35065_a");Trace10.writeInt(out, "es.size()", es.size());for(Object e:es)bounds(out,(StructureBoundingBox)e);}
            if(p instanceof ComponentMineshaftCorridor) {Trace10.writeBoolean(out, "(Boolean)field(p,\"field_35070_a\")", (Boolean)field(p,"field_35070_a"));Trace10.writeBoolean(out, "(Boolean)field(p,\"field_35068_b\")", (Boolean)field(p,"field_35068_b"));Trace10.writeBoolean(out, "(Boolean)field(p,\"field_35069_c\")", (Boolean)field(p,"field_35069_c"));Trace10.writeInt(out, "(Integer)field(p,\"field_35067_d\")", (Integer)field(p,"field_35067_d"));}
            if(p instanceof ComponentMineshaftCross) {Trace10.writeInt(out, "(Integer)field(p,\"field_35073_a\")", (Integer)field(p,"field_35073_a"));Trace10.writeBoolean(out, "(Boolean)field(p,\"field_35072_b\")", (Boolean)field(p,"field_35072_b"));}
            if(p instanceof ComponentVillage) {
                String name=p.getClass().getSimpleName();String h=null;
                if(p instanceof ComponentVillageWell)h="field_35103_b";
                else if(p instanceof ComponentVillageHouse4_Garden)h="field_35084_a";
                else if(p instanceof ComponentVillageChurch)h="field_35098_a";
                else if(p instanceof ComponentVillageHouse1)h="averageGroundLevel";
                else if(p instanceof ComponentVillageWoodHut)h="field_35094_a";
                else if(p instanceof ComponentVillageHall)h="field_35079_a";
                else if(p instanceof ComponentVillageField)h="field_35081_a";
                else if(p instanceof ComponentVillageField2)h="field_35090_a";
                else if(p instanceof ComponentVillageHouse2)h="field_35086_a";
                else if(p instanceof ComponentVillageHouse3)h="field_35102_a";
                else if(p instanceof ComponentVillageTorch)h="field_35100_a";
                Trace10.writeInt(out, "h==null?-1:(Integer)field(p,h)", h==null?-1:(Integer)field(p,h));
                Trace10.writeBoolean(out, "p instanceof ComponentVillageHouse4_Garden?(Boolean)field(p,\"field_35083_b\"):p instanceof ComponentVillageWoodHut?(Boolean)field(p,\"field_35092_b\"):true", p instanceof ComponentVillageHouse4_Garden?(Boolean)field(p,"field_35083_b"):p instanceof ComponentVillageWoodHut?(Boolean)field(p,"field_35092_b"):true);
                Trace10.writeInt(out, "p instanceof ComponentVillageWoodHut?(Integer)field(p,\"field_35093_c\"):0", p instanceof ComponentVillageWoodHut?(Integer)field(p,"field_35093_c"):0);
                Trace10.writeInt(out, "Math.max(p.getBoundingBox().getXSize(),p.getBoundingBox().getZSize())", Math.max(p.getBoundingBox().getXSize(),p.getBoundingBox().getZSize()));
            }
        }
    }
    public static void main(String[] a) throws Exception {
        String mode=a[1];long seed=Long.parseLong(a[2]);Random r=new Random(seed);
        World w=Oracle.newWorld(mode.equals("map")||mode.equals("village-map")?seed:0);EntityOracle.FlatProvider provider=new EntityOracle.FlatProvider(w);w.chunkProvider=provider;
        for(int cx=-8;cx<=8;cx++)for(int cz=-8;cz<=8;cz++)if(!provider.chunkExists(cx,cz)){Chunk c=new Chunk(w,new byte[32768],cx,cz);c.isTerrainPopulated=true;c.isChunkLoaded=true;provider.chunks.put(provider.key(cx,cz),c);}
        try(DataOutputStream out=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[3])))) {
            if(mode.equals("layout")||mode.equals("village-layout")) {
                if(mode.startsWith("village"))w.worldProvider.worldChunkMgr=new WorldChunkManagerHell(BiomeGenBase.plains,0.5F,0);
                for(int i=0;i<16;i++) {StructureStart s=mode.startsWith("village")?new StructureVillageStart(w,r,i%4-2,i/4-2):new StructureMineshaftStart(w,r,i%4-2,i/4-2);pieces(out,s);if(mode.startsWith("village"))Trace10.writeBoolean(out, "s.isSizeableStructure()", s.isSizeableStructure());Trace10.writeLong(out, "EntityOracle.rngState(r)", EntityOracle.rngState(r));}
            } else if(mode.equals("village-map")) {
                MapGenVillage g=new MapGenVillage();w.rand.setSeed(300);
                ArrayList<int[]> queries=new ArrayList<int[]>(Arrays.asList(new int[][]{{0,0},{40,-30},{-40,30},{784425,0},{-3,-784425},{40,-30}}));
                for(int x=-3;x<=3;x++)for(int z=-3;z<=3;z++){Random candidate=new Random((long)x*341873128712L+(long)z*132897987541L+seed+10387312L);queries.add(new int[]{x*32+candidate.nextInt(24),z*32+candidate.nextInt(24)});}
                for(int[] q:queries){g.generate(provider,w,q[0],q[1],null);Trace10.writeLong(out, "EntityOracle.rngState(g.rand)", EntityOracle.rngState(g.rand));Trace10.writeLong(out, "EntityOracle.rngState(w.rand)", EntityOracle.rngState(w.rand));Trace10.writeInt(out, "g.coordMap.size()", g.coordMap.size());for(Object key:g.coordMap.keySet()){Trace10.writeLong(out, "(Long)key", (Long)key);StructureStart s=(StructureStart)g.coordMap.get(key);pieces(out,s);Trace10.writeBoolean(out, "s.isSizeableStructure()", s.isSizeableStructure());}}
            } else if(mode.equals("map")) {
                MapGenMineshaft g=new MapGenMineshaft();
                for(int[] q:new int[][]{{0,0},{40,-30},{-40,30},{784425,0},{-3,-784425},{40,-30}}){g.generate(provider,w,q[0],q[1],null);Trace10.writeLong(out, "EntityOracle.rngState(g.rand)", EntityOracle.rngState(g.rand));Trace10.writeInt(out, "g.coordMap.size()", g.coordMap.size());for(Object key:g.coordMap.keySet()){Trace10.writeLong(out, "(Long)key", (Long)key);pieces(out,(StructureStart)g.coordMap.get(key));}}
            } else if(mode.equals("blocks")||mode.equals("village-blocks")) {
                if(mode.startsWith("village"))w.worldProvider.worldChunkMgr=new WorldChunkManagerHell(BiomeGenBase.plains,0.5F,0);
                StructureStart s=mode.startsWith("village")?new StructureVillageStart(w,r,0,0):new StructureMineshaftStart(w,r,0,0);pieces(out,s);Trace10.writeLong(out, "EntityOracle.rngState(r)", EntityOracle.rngState(r));
                // A finite solid stone fixture, avoiding unrelated providers and population.
                for(int cx=-7;cx<=7;cx++)for(int cz=-7;cz<=7;cz++) {Chunk c=w.getChunkFromChunkCoords(cx,cz);Arrays.fill(c.blocks,(byte)0);Arrays.fill(c.data.data,(byte)0);for(int x=0;x<16;x++)for(int z=0;z<16;z++)for(int y=0;y<64;y++)c.blocks[(x<<11)|(z<<7)|y]=(byte)(mode.startsWith("village")?(y<61?1:y<63?3:2):1);c.generateSkylightMap();}
                w.rand.setSeed(300);BlockSand.fallInstantly=true;
                for(int cx=-7;cx<7;cx++)for(int cz=-7;cz<7;cz++) {int x=cx*16+8,z=cz*16+8;StructureBoundingBox clip=new StructureBoundingBox(x,z,x+15,z+15);if(s.getBoundingBox().intersectsWith(clip))s.generateStructure(w,r,clip);}
                pieces(out,s);Trace10.writeLong(out, "EntityOracle.rngState(r)", EntityOracle.rngState(r));Trace10.writeLong(out, "EntityOracle.rngState(w.rand)", EntityOracle.rngState(w.rand));
                for(int cx=-7;cx<=7;cx++)for(int cz=-7;cz<=7;cz++){Chunk c=w.getChunkFromChunkCoords(cx,cz);Trace10.writeBytes(out, "c.blocks", c.blocks);Trace10.writeBytes(out, "c.data.data", c.data.data);}
                tiles(out,w,-112,127);
            } else throw new IllegalArgumentException(mode);
        }
    }
    static void tiles(DataOutputStream out,World w,int min,int max) throws Exception {
        for(int x=min;x<=max;x++)for(int z=min;z<=max;z++)for(int y=0;y<128;y++) {
            int id=w.getBlockId(x,y,z);if(id!=54 && id!=52)continue;
            Trace10.writeInt(out, "x", x);Trace10.writeInt(out, "y", y);Trace10.writeInt(out, "z", z);Trace10.writeInt(out, "id", id);
            TileEntity t=w.getBlockTileEntity(x,y,z);
            if(t instanceof TileEntityChest) {TileEntityChest c=(TileEntityChest)t;for(int n=0;n<27;n++){ItemStack s=c.getStackInSlot(n);Trace10.writeBoolean(out, "s!=null", s!=null);if(s!=null){Trace10.writeInt(out, "s.itemID", s.itemID);Trace10.writeInt(out, "s.stackSize", s.stackSize);Trace10.writeInt(out, "s.getItemDamage()", s.getItemDamage());}}}
            else {NBTTagCompound n=new NBTTagCompound();t.writeToNBT(n);out.writeUTF(n.getString("EntityId"));Trace10.writeInt(out, "n.getShort(\"Delay\")", n.getShort("Delay"));}
        }
        Trace10.writeInt(out, "Integer.MIN_VALUE", Integer.MIN_VALUE);
    }
}
