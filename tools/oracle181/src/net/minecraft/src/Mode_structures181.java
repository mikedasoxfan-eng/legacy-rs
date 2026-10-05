package net.minecraft.src;
import java.io.*;
import java.util.*;
import java.lang.reflect.*;

/** Real original-bytecode mineshaft starts, pieces, state and clipped block generation. */
public final class Mode_structures181 {
    static void bounds(DataOutputStream out, StructureBoundingBox b) throws Exception {
        out.writeInt(b.field_35753_a);out.writeInt(b.field_35751_b);out.writeInt(b.field_35752_c);
        out.writeInt(b.field_35749_d);out.writeInt(b.field_35750_e);out.writeInt(b.field_35748_f);
    }
    static Object field(Object o,String n) throws Exception {Class c=o.getClass();while(c!=null){try{Field f=c.getDeclaredField(n);f.setAccessible(true);return f.get(o);}catch(NoSuchFieldException e){c=c.getSuperclass();}}throw new NoSuchFieldException(n);}
    static void pieces(DataOutputStream out, StructureStart s) throws Exception {
        bounds(out,s.func_35712_a());out.writeInt(s.field_35717_a.size());
        for(Object o:s.field_35717_a) {
            StructureComponent p=(StructureComponent)o;out.writeUTF(p.getClass().getSimpleName());
            out.writeInt(p.field_35026_i);out.writeInt(p.field_35025_h);bounds(out,p.func_35021_b());
            if(p instanceof ComponentMineshaftRoom) {List es=(List)field(p,"field_35065_a");out.writeInt(es.size());for(Object e:es)bounds(out,(StructureBoundingBox)e);}
            if(p instanceof ComponentMineshaftCorridor) {out.writeBoolean((Boolean)field(p,"field_35070_a"));out.writeBoolean((Boolean)field(p,"field_35068_b"));out.writeBoolean((Boolean)field(p,"field_35069_c"));out.writeInt((Integer)field(p,"field_35067_d"));}
            if(p instanceof ComponentMineshaftCross) {out.writeInt((Integer)field(p,"field_35073_a"));out.writeBoolean((Boolean)field(p,"field_35072_b"));}
            if(p instanceof ComponentVillage) {
                String name=p.getClass().getSimpleName();String h=null;
                if(p instanceof ComponentVillageWell)h="field_35103_b";
                else if(p instanceof ComponentVillageHouse4_Garden)h="field_35084_a";
                else if(p instanceof ComponentVillageChurch)h="field_35098_a";
                else if(p instanceof ComponentVillageHouse1)h="field_35096_a";
                else if(p instanceof ComponentVillageWoodHut)h="field_35094_a";
                else if(p instanceof ComponentVillageHall)h="field_35079_a";
                else if(p instanceof ComponentVillageField)h="field_35081_a";
                else if(p instanceof ComponentVillageField2)h="field_35090_a";
                else if(p instanceof ComponentVillageHouse2)h="field_35086_a";
                else if(p instanceof ComponentVillageHouse3)h="field_35102_a";
                else if(p instanceof ComponentVillageTorch)h="field_35100_a";
                out.writeInt(h==null?-1:(Integer)field(p,h));
                out.writeBoolean(p instanceof ComponentVillageHouse4_Garden?(Boolean)field(p,"field_35083_b"):p instanceof ComponentVillageWoodHut?(Boolean)field(p,"field_35092_b"):true);
                out.writeInt(p instanceof ComponentVillageWoodHut?(Integer)field(p,"field_35093_c"):0);
                out.writeInt(Math.max(p.func_35021_b().func_35744_b(),p.func_35021_b().func_35739_d()));
            }
        }
    }
    public static void main(String[] a) throws Exception {
        String mode=a[1];long seed=Long.parseLong(a[2]);Random r=new Random(seed);
        World w=Oracle.newWorld(mode.equals("map")||mode.equals("village-map")?seed:0);EntityOracle.FlatProvider provider=new EntityOracle.FlatProvider(w);w.chunkProvider=provider;
        for(int cx=-8;cx<=8;cx++)for(int cz=-8;cz<=8;cz++)if(!provider.chunkExists(cx,cz)){Chunk c=new Chunk(w,new byte[32768],cx,cz);c.isTerrainPopulated=true;c.isChunkLoaded=true;provider.chunks.put(provider.key(cx,cz),c);}
        try(DataOutputStream out=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[3])))) {
            if(mode.equals("layout")||mode.equals("village-layout")) {
                if(mode.startsWith("village"))w.worldProvider.worldChunkMgr=new WorldChunkManagerHell(BiomeGenBase.field_35485_c,0.5F,0);
                for(int i=0;i<16;i++) {StructureStart s=mode.startsWith("village")?new StructureVillageStart(w,r,i%4-2,i/4-2):new StructureMineshaftStart(w,r,i%4-2,i/4-2);pieces(out,s);if(mode.startsWith("village"))out.writeBoolean(s.func_35715_c());out.writeLong(EntityOracle.rngState(r));}
            } else if(mode.equals("village-map")) {
                MapGenVillage g=new MapGenVillage();w.rand.setSeed(300);
                ArrayList<int[]> queries=new ArrayList<int[]>(Arrays.asList(new int[][]{{0,0},{40,-30},{-40,30},{784425,0},{-3,-784425},{40,-30}}));
                for(int x=-3;x<=3;x++)for(int z=-3;z<=3;z++){Random candidate=new Random((long)x*341873128712L+(long)z*132897987541L+seed+10387312L);queries.add(new int[]{x*32+candidate.nextInt(24),z*32+candidate.nextInt(24)});}
                for(int[] q:queries){g.generate(provider,w,q[0],q[1],null);out.writeLong(EntityOracle.rngState(g.rand));out.writeLong(EntityOracle.rngState(w.rand));out.writeInt(g.field_35631_e.size());for(Object key:g.field_35631_e.keySet()){out.writeLong((Long)key);StructureStart s=(StructureStart)g.field_35631_e.get(key);pieces(out,s);out.writeBoolean(s.func_35715_c());}}
            } else if(mode.equals("map")) {
                MapGenMineshaft g=new MapGenMineshaft();
                for(int[] q:new int[][]{{0,0},{40,-30},{-40,30},{784425,0},{-3,-784425},{40,-30}}){g.generate(provider,w,q[0],q[1],null);out.writeLong(EntityOracle.rngState(g.rand));out.writeInt(g.field_35631_e.size());for(Object key:g.field_35631_e.keySet()){out.writeLong((Long)key);pieces(out,(StructureStart)g.field_35631_e.get(key));}}
            } else if(mode.equals("blocks")||mode.equals("village-blocks")) {
                if(mode.startsWith("village"))w.worldProvider.worldChunkMgr=new WorldChunkManagerHell(BiomeGenBase.field_35485_c,0.5F,0);
                StructureStart s=mode.startsWith("village")?new StructureVillageStart(w,r,0,0):new StructureMineshaftStart(w,r,0,0);pieces(out,s);out.writeLong(EntityOracle.rngState(r));
                // A finite solid stone fixture, avoiding unrelated providers and population.
                for(int cx=-7;cx<=7;cx++)for(int cz=-7;cz<=7;cz++) {Chunk c=w.getChunkFromChunkCoords(cx,cz);Arrays.fill(c.blocks,(byte)0);Arrays.fill(c.data.data,(byte)0);for(int x=0;x<16;x++)for(int z=0;z<16;z++)for(int y=0;y<64;y++)c.blocks[(x<<11)|(z<<7)|y]=(byte)(mode.startsWith("village")?(y<61?1:y<63?3:2):1);c.generateSkylightMap();}
                w.rand.setSeed(300);BlockSand.fallInstantly=true;
                for(int cx=-7;cx<7;cx++)for(int cz=-7;cz<7;cz++) {int x=cx*16+8,z=cz*16+8;StructureBoundingBox clip=new StructureBoundingBox(x,z,x+15,z+15);if(s.func_35712_a().func_35740_a(clip))s.func_35711_a(w,r,clip);}
                pieces(out,s);out.writeLong(EntityOracle.rngState(r));out.writeLong(EntityOracle.rngState(w.rand));
                for(int cx=-7;cx<=7;cx++)for(int cz=-7;cz<=7;cz++){Chunk c=w.getChunkFromChunkCoords(cx,cz);out.write(c.blocks);out.write(c.data.data);}
                tiles(out,w,-112,127);
            } else throw new IllegalArgumentException(mode);
        }
    }
    static void tiles(DataOutputStream out,World w,int min,int max) throws Exception {
        for(int x=min;x<=max;x++)for(int z=min;z<=max;z++)for(int y=0;y<128;y++) {
            int id=w.getBlockId(x,y,z);if(id!=54 && id!=52)continue;
            out.writeInt(x);out.writeInt(y);out.writeInt(z);out.writeInt(id);
            TileEntity t=w.getBlockTileEntity(x,y,z);
            if(t instanceof TileEntityChest) {TileEntityChest c=(TileEntityChest)t;for(int n=0;n<27;n++){ItemStack s=c.getStackInSlot(n);out.writeBoolean(s!=null);if(s!=null){out.writeInt(s.itemID);out.writeInt(s.stackSize);out.writeInt(s.getItemDamage());}}}
            else {NBTTagCompound n=new NBTTagCompound();t.writeToNBT(n);out.writeUTF(n.getString("EntityId"));out.writeInt(n.getShort("Delay"));}
        }
        out.writeInt(Integer.MIN_VALUE);
    }
}
