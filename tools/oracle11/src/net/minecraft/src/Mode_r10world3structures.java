package net.minecraft.src;
import java.io.*;
import java.util.*;
import java.lang.reflect.*;

/** Complete fortress graphs and clipped placements, including release stronghold rooms. */
public final class Mode_r10world3structures {
    static final Class[] CLASSES={ComponentNetherBridgeStraight.class,ComponentNetherBridgeCrossing3.class,
        ComponentNetherBridgeCrossing.class,ComponentNetherBridgeStairs.class,ComponentNetherBridgeThrone.class,
        ComponentNetherBridgeEntrance.class,ComponentNetherBridgeCorridor5.class,ComponentNetherBridgeCrossing2.class,
        ComponentNetherBridgeCorridor2.class,ComponentNetherBridgeCorridor.class,ComponentNetherBridgeCorridor3.class,
        ComponentNetherBridgeCorridor4.class,ComponentNetherBridgeNetherStalkRoom.class,StructureNetherBridgeEnd.class};
    static final int[][] SIZES={{5,10,19},{19,10,19},{7,9,7},{7,11,7},{7,8,9},{13,14,13},{5,7,5},
        {5,7,5},{5,7,5},{5,7,5},{5,14,10},{9,7,9},{13,14,13},{5,10,8}};
    static int kind(StructureComponent p){for(int j=0;j<CLASSES.length;j++)if(CLASSES[j].isInstance(p))return j;throw new AssertionError(p);}
    static void piece(DataOutputStream o,StructureComponent p)throws Exception {
        int kind=kind(p);Mode_r10world3.i(o,kind);Mode_r10world3.i(o,p.coordBaseMode);Mode_r10world3.i(o,p.componentType);
        Mode_terrainutils10.box(o,p.getBoundingBox());
        Mode_r10world3.i(o,kind==13?(Integer)Mode_structures10.field(p,"field_40024_a"):0);
        Mode_r10world3.b(o,kind==4?(Boolean)Mode_structures10.field(p,"field_40027_a"):false);
    }
    static void layout(DataOutputStream o,long seed)throws Exception {
        Random r=new Random(seed);World w=Mode_r10world3.flat(-1);
        for(int j=0;j<16;j++) {
            StructureNetherBridgeStart s=new StructureNetherBridgeStart(w,r,j%4-2,j/4-2);
            Mode_terrainutils10.box(o,s.getBoundingBox());Mode_r10world3.i(o,s.components.size());
            for(Object p:s.components)piece(o,(StructureComponent)p);
            Mode_r10world3.l(o,EntityOracle.rngState(r));
        }
    }
    static void map(DataOutputStream o,long seed)throws Exception {
        World w=new World(new Oracle.NullSaveHandler(seed),"oracle",Oracle.settings(seed),new WorldProviderHell());
        MapGenNetherBridge g=new MapGenNetherBridge();
        for(int[] q:new int[][]{{0,0},{4,4},{-8,-8},{40,-30},{-40,30},{784425,-3},{0,0}}) {
            g.generate(w.chunkProvider,w,q[0],q[1],null);
            Mode_r10world3.i(o,g.coordMap.size());
            for(Object key:g.coordMap.keySet()) {
                Mode_r10world3.l(o,(Long)key);StructureStart s=(StructureStart)g.coordMap.get(key);
                Mode_terrainutils10.box(o,s.getBoundingBox());Mode_r10world3.i(o,s.components.size());
                for(Object p:s.components)piece(o,(StructureComponent)p);
            }
        }
    }
    static World solid(boolean nether)throws Exception {
        World w=Mode_r10world3.flat(nether?-1:0);BlockSand.fallInstantly=true;
        for(int x=-3;x<=3;x++)for(int z=-3;z<=3;z++) {
            Chunk c=w.getChunkFromChunkCoords(x,z);
            for(int bx=0;bx<16;bx++)for(int bz=0;bz<16;bz++)for(int y=0;y<(nether?32:64);y++)c.blocks[(bx<<11)|(bz<<7)|y]=1;
            c.generateSkylightMap();
        }
        w.rand.setSeed(300);return w;
    }
    static void blocks(DataOutputStream o,World w)throws Exception {
        for(int x=-2;x<=2;x++)for(int z=-2;z<=2;z++){Chunk c=w.getChunkFromChunkCoords(x,z);o.write(c.blocks);o.write(c.data.data);}
        Mode_structures10.tiles(o,w,-32,47);
    }
    static void placement(DataOutputStream o,long seed,int kind,boolean stronghold)throws Exception {
        Random r=new Random(seed);
        for(int dir=0;dir<4;dir++) {
            World w=solid(!stronghold);
            int[] sz=stronghold?(kind==12?new int[]{11,8,16}:new int[]{5,5,7}):SIZES[kind];
            StructureBoundingBox bb=StructureBoundingBox.getComponentToAddBoundingBox(0,50,0,0,0,0,sz[0],sz[1],sz[2],dir);
            Class c=stronghold?(kind==12?ComponentStrongholdPortalRoom.class:ComponentStrongholdChestCorridor.class):CLASSES[kind];
            StructureComponent p=(StructureComponent)c.getConstructor(int.class,Random.class,StructureBoundingBox.class,int.class).newInstance(7,r,bb,dir);
            Mode_r10world3.l(o,EntityOracle.rngState(r));
            for(StructureBoundingBox clip:new StructureBoundingBox[]{new StructureBoundingBox(-32,0,-32,6,127,47),new StructureBoundingBox(7,0,-32,47,127,47),new StructureBoundingBox(-32,0,-32,47,127,47)}) {
                Mode_r10world3.b(o,p.addComponentParts(w,r,clip));Mode_r10world3.l(o,EntityOracle.rngState(r));Mode_r10world3.l(o,EntityOracle.rngState(w.rand));
                if(stronghold){Mode_r10world3.b(o,(Boolean)Mode_structures10.field(p,kind==12?"field_40015_a":"field_40012_c"));}
                else piece(o,p);
                blocks(o,w);
            }
        }
    }
    public static void main(String[] a)throws Exception {
        String mode=a[1];long seed=Long.parseLong(a[2]);
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[3])))) {
            if(mode.equals("layout"))layout(o,seed);
            else if(mode.equals("map"))map(o,seed);
            else if(mode.startsWith("fortress"))placement(o,seed,Integer.parseInt(mode.substring(8)),false);
            else if(mode.equals("portal"))placement(o,seed,12,true);
            else if(mode.equals("chest"))placement(o,seed,11,true);
            else throw new IllegalArgumentException(mode);
        }
    }
}
