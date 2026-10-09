package net.minecraft.src;
import java.io.*;
import java.util.*;

/** Stronghold graphs, retained map/RNG order, clipped blocks and chest inventories.
 * Only the harness is compiled here: all structure classes come from Mojang's jar. */
public final class Mode_strongholds10 {
    static Object field(Object o,String n) throws Exception {return Mode_structures10.field(o,n);}
    static void piece(DataOutputStream out,StructureComponent p) throws Exception {
        out.writeUTF(p.getClass().getSimpleName());Trace10.writeInt(out, "p.field_35026_i", p.componentType);Trace10.writeInt(out, "p.coordBaseMode", p.coordBaseMode);Mode_structures10.bounds(out,p.getBoundingBox());
        String door=null;boolean[] flags=new boolean[4];int style=0;
        if(p instanceof ComponentStrongholdStraight){door="field_35050_a";flags[0]=(Boolean)field(p,"field_35048_b");flags[1]=(Boolean)field(p,"field_35049_c");}
        else if(p instanceof ComponentStrongholdPrison)door="field_35064_a";
        else if(p instanceof ComponentStrongholdLeftTurn)door="field_35046_a";
        else if(p instanceof ComponentStrongholdRoomCrossing){door="field_35062_a";style=(Integer)field(p,"field_35060_b");}
        else if(p instanceof ComponentStrongholdStairsStraight)door="field_35054_a";
        else if(p instanceof ComponentStrongholdStairs){door="doorType";flags[0]=(Boolean)field(p,"field_35036_a");}
        else if(p instanceof ComponentStrongholdCrossing){door="field_35044_a";String[] f={"field_35042_b","field_35043_c","field_35040_d","field_35041_e"};for(int i=0;i<4;i++)flags[i]=(Boolean)field(p,f[i]);}
        else if(p instanceof ComponentStrongholdLibrary){door="field_35058_a";flags[0]=(Boolean)field(p,"field_35057_c");}
        Trace10.writeInt(out, "door==null?0:((EnumDoor)field(p,door)).ordinal()", door==null?0:((EnumDoor)field(p,door)).ordinal());for(boolean f:flags)Trace10.writeBoolean(out, "f", f);Trace10.writeInt(out, "style", style);
        Trace10.writeInt(out, "p.coordBaseMode==0||p.coordBaseMode==2?p.getBoundingBox().getZSize():p.getBoundingBox().getXSize()", p.coordBaseMode==0||p.coordBaseMode==2?p.getBoundingBox().getZSize():p.getBoundingBox().getXSize());
    }
    static void pieces(DataOutputStream out,StructureStart s) throws Exception {Mode_structures10.bounds(out,s.getBoundingBox());Trace10.writeInt(out, "s.components.size()", s.components.size());for(Object p:s.components)piece(out,(StructureComponent)p);}
    static StructureComponent fixture(int kind,Random r,int orientation) throws Exception {
        int[][] a={{5,5,7},{9,5,11},{5,5,5},{5,5,5},{11,7,11},{5,11,8},{5,11,5},{10,9,11},{14,11,15},{5,5,3},{14,6,15}};
        int[] size=a[kind];StructureBoundingBox b=StructureBoundingBox.getComponentToAddBoundingBox(0,30,0,0,0,0,size[0],size[1],size[2],orientation);
        Class[] cs={ComponentStrongholdStraight.class,ComponentStrongholdPrison.class,ComponentStrongholdLeftTurn.class,ComponentStrongholdRightTurn.class,ComponentStrongholdRoomCrossing.class,ComponentStrongholdStairsStraight.class,ComponentStrongholdStairs.class,ComponentStrongholdCrossing.class,ComponentStrongholdLibrary.class,ComponentStrongholdCorridor.class,ComponentStrongholdLibrary.class};
        return (StructureComponent)cs[kind].getConstructor(int.class,Random.class,StructureBoundingBox.class,int.class).newInstance(7,r,b,orientation);
    }
    static void solid(World w) {
        for(int cx=-8;cx<=8;cx++)for(int cz=-8;cz<=8;cz++) {Chunk c=w.getChunkFromChunkCoords(cx,cz);Arrays.fill(c.blocks,(byte)0);Arrays.fill(c.data.data,(byte)0);for(int x=0;x<16;x++)for(int z=0;z<16;z++)for(int y=0;y<64;y++)c.blocks[(x<<11)|(z<<7)|y]=1;c.generateSkylightMap();}
    }
    static void blocks(DataOutputStream out,World w) throws Exception {for(int cx=-7;cx<=7;cx++)for(int cz=-7;cz<=7;cz++){Chunk c=w.getChunkFromChunkCoords(cx,cz);Trace10.writeBytes(out, "c.blocks", c.blocks);Trace10.writeBytes(out, "c.data.data", c.data.data);}Mode_structures10.tiles(out,w,-112,127);}
    public static void main(String[] a) throws Exception {
        String mode=a[1];long seed=Long.parseLong(a[2]);Random r=new Random(seed);
        if(mode.equals("grid")) {
            World w=Oracle.newWorld(seed);w.rand.setSeed(300);
            MapGenStronghold probe=new MapGenStronghold();probe.worldObj=w;probe.canSpawnStructureAtCoords(0,0);ChunkCoordIntPair[] candidates=(ChunkCoordIntPair[])field(probe,"structureCoords");
            for(ChunkCoordIntPair p:candidates)for(int x=p.chunkXPos-3;x<=p.chunkXPos+3;x++)for(int z=p.chunkZPos-3;z<=p.chunkZPos+3;z++)w.getChunkFromChunkCoords(x,z);
            ArrayList<Chunk> cs=new ArrayList<Chunk>((Collection<Chunk>)field(w.chunkProvider,"chunkList"));cs.sort((p,q)->p.xPosition!=q.xPosition?Integer.compare(p.xPosition,q.xPosition):Integer.compare(p.zPosition,q.zPosition));
            ChunkProviderGenerate gen=(ChunkProviderGenerate)field(w.chunkProvider,"chunkProvider");
            try(DataOutputStream out=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[3])))) {
                Trace10.writeLong(out, "EntityOracle.rngState((Random)field(gen,\"rand\"))", EntityOracle.rngState((Random)field(gen,"rand")));Trace10.writeLong(out, "EntityOracle.rngState(w.rand)", EntityOracle.rngState(w.rand));Trace10.writeInt(out, "cs.size()", cs.size());
                for(Chunk c:cs){Trace10.writeInt(out, "c.xPosition", c.xPosition);Trace10.writeInt(out, "c.zPosition", c.zPosition);Trace10.writeBoolean(out, "c.isTerrainPopulated", c.isTerrainPopulated);Trace10.writeBytes(out, "c.blocks", c.blocks);Trace10.writeBytes(out, "c.data.data", c.data.data);Trace10.writeBytes(out, "c.skylightMap.data", c.skylightMap.data);Trace10.writeBytes(out, "c.blocklightMap.data", c.blocklightMap.data);Trace10.writeBytes(out, "c.heightMap", c.heightMap);}
                Trace10.writeInt(out, "strongholdGenerator.coordMap.size()", ((MapGenStronghold)R11Access.get(gen,"strongholdGenerator")).coordMap.size());for(Object k:((MapGenStronghold)R11Access.get(gen,"strongholdGenerator")).coordMap.keySet()){Trace10.writeLong(out, "(Long)k", (Long)k);pieces(out,(StructureStart)((MapGenStronghold)R11Access.get(gen,"strongholdGenerator")).coordMap.get(k));}
            }return;
        }
        World w=Oracle.newWorld(mode.equals("map")?seed:0);EntityOracle.FlatProvider provider=new EntityOracle.FlatProvider(w);w.chunkProvider=provider;
        for(int cx=-8;cx<=8;cx++)for(int cz=-8;cz<=8;cz++)if(!provider.chunkExists(cx,cz)){Chunk c=new Chunk(w,new byte[32768],cx,cz);c.isTerrainPopulated=true;c.isChunkLoaded=true;provider.chunks.put(provider.key(cx,cz),c);}
        try(DataOutputStream out=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[3])))) {
            if(mode.equals("layout")) {for(int i=0;i<16;i++){StructureStart s=new StructureStrongholdStart(w,r,i%4-2,i/4-2);pieces(out,s);Trace10.writeLong(out, "EntityOracle.rngState(r)", EntityOracle.rngState(r));}}
            else if(mode.equals("map")) {
                MapGenStronghold probe=new MapGenStronghold();probe.worldObj=w;probe.canSpawnStructureAtCoords(0,0);ChunkCoordIntPair[] candidates=(ChunkCoordIntPair[])field(probe,"structureCoords");
                ArrayList<int[]> qs=new ArrayList<int[]>(Arrays.asList(new int[][]{{0,0},{40,-30},{-40,30},{784425,0},{-3,-784425},{40,-30}}));for(ChunkCoordIntPair p:candidates)qs.add(new int[]{p.chunkXPos,p.chunkZPos});for(ChunkCoordIntPair p:candidates)qs.add(new int[]{p.chunkXPos+7,p.chunkZPos-7});
                MapGenStronghold g=new MapGenStronghold();
                for(int[] q:qs){g.generate(provider,w,q[0],q[1],null);Trace10.writeLong(out, "EntityOracle.rngState(g.rand)", EntityOracle.rngState(g.rand));for(ChunkCoordIntPair p:(ChunkCoordIntPair[])field(g,"structureCoords")){Trace10.writeInt(out, "p.chunkXPos", p.chunkXPos);Trace10.writeInt(out, "p.chunkZPos", p.chunkZPos);}Trace10.writeInt(out, "g.coordMap.size()", g.coordMap.size());for(Object key:g.coordMap.keySet()){Trace10.writeLong(out, "(Long)key", (Long)key);pieces(out,(StructureStart)g.coordMap.get(key));}}
            } else if(mode.equals("blocks")||mode.equals("wet")) {
                StructureStart s=new StructureStrongholdStart(w,r,0,0);pieces(out,s);Trace10.writeLong(out, "EntityOracle.rngState(r)", EntityOracle.rngState(r));solid(w);w.rand.setSeed(300);BlockSand.fallInstantly=true;
                if(mode.equals("wet"))for(int x=-80;x<=80;x++)for(int z=-80;z<=80;z++)w.getChunkFromChunkCoords(x>>4,z>>4).blocks[((x&15)<<11)|((z&15)<<7)|25]=9;
                for(int cx=-7;cx<7;cx++)for(int cz=-7;cz<7;cz++){int x=cx*16+8,z=cz*16+8;StructureBoundingBox clip=new StructureBoundingBox(x,z,x+15,z+15);if(s.getBoundingBox().intersectsWith(clip))s.generateStructure(w,r,clip);}
                pieces(out,s);Trace10.writeLong(out, "EntityOracle.rngState(r)", EntityOracle.rngState(r));Trace10.writeLong(out, "EntityOracle.rngState(w.rand)", EntityOracle.rngState(w.rand));blocks(out,w);
            } else if(mode.equals("pieces")) {
                for(int k=0;k<11;k++)for(int d=0;d<4;d++){
                    solid(w);w.rand.setSeed(300);BlockSand.fallInstantly=true;StructureComponent p=fixture(k,r,d);piece(out,p);Trace10.writeLong(out, "EntityOracle.rngState(r)", EntityOracle.rngState(r));
                    // Split the clip so selector draws, existing-block tests and chests run twice.
                    Trace10.writeBoolean(out, "p.addComponentParts(w,r,new StructureBoundingBox(-32,0,-32,6,127,32))", p.addComponentParts(w,r,new StructureBoundingBox(-32,0,-32,6,127,32)));
                    Trace10.writeBoolean(out, "p.addComponentParts(w,r,new StructureBoundingBox(7,0,-32,32,127,32))", p.addComponentParts(w,r,new StructureBoundingBox(7,0,-32,32,127,32)));
                    piece(out,p);Trace10.writeLong(out, "EntityOracle.rngState(r)", EntityOracle.rngState(r));Trace10.writeLong(out, "EntityOracle.rngState(w.rand)", EntityOracle.rngState(w.rand));
                    for(int cx=-2;cx<=2;cx++)for(int cz=-2;cz<=2;cz++){Chunk c=w.getChunkFromChunkCoords(cx,cz);Trace10.writeBytes(out, "c.blocks", c.blocks);Trace10.writeBytes(out, "c.data.data", c.data.data);}Mode_structures10.tiles(out,w,-32,47);
                }
            } else throw new IllegalArgumentException(mode);
        }
    }
}
