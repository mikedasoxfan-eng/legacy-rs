package net.minecraft.src;
import java.io.*;
public final class Mode_wbtrapdoors6 {
    static void seed(World w)throws Exception {w.rand.setSeed(42);EntityOracle.mathRandom().setSeed(200);OracleHooks.reset(100);WorldEventOracle.attach(w);}
    static void result(DataOutputStream o,World w)throws Exception {
        Mode_wbplacement4.cells(o,w,7,9,62,65);EntityOracle.l(o,Trace10.namedLong("EntityOracle.rngState(w.rand)", EntityOracle.rngState(w.rand)));EntityOracle.i(o,Trace10.namedInt("w.loadedEntityList.size()", w.loadedEntityList.size()));
        for(Object object:w.loadedEntityList) {
            EntityItem e=(EntityItem)object;EntityOracle.i(o,Trace10.namedInt("e.item.itemID", e.item.itemID));EntityOracle.i(o,Trace10.namedInt("e.item.stackSize", e.item.stackSize));EntityOracle.i(o,Trace10.namedInt("e.item.getItemDamage()", e.item.getItemDamage()));EntityOracle.i(o,Trace10.namedInt("e.delayBeforeCanPickup", e.delayBeforeCanPickup));
            for(double v:new double[]{Trace10.namedDouble("e.posX", e.posX),Trace10.namedDouble("e.posY", e.posY),Trace10.namedDouble("e.posZ", e.posZ),Trace10.namedDouble("e.motionX", e.motionX),Trace10.namedDouble("e.motionY", e.motionY),Trace10.namedDouble("e.motionZ", e.motionZ)})EntityOracle.d(o,v);
        }
        WorldEventOracle.snapshot(w,o);
    }
    public static void main(String[] a)throws Exception {
        int init=Block.stone.blockID;World w=Mode_worldblocks10.fixture(true);BlockTrapDoor b=(BlockTrapDoor)Block.trapdoor;
        int[][] offsets={{-1,0},{1,0},{0,-1},{0,1}},sides={{0,1},{0,-1},{1,0},{-1,0}};
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            for(boolean remote:new boolean[]{false,true})for(int m=0;m<16;m++)for(int support=0;support<=109;support++)for(int change:new int[]{0,1,55,76}) {
                // Raw fixture block replacement bypasses TileEntity cleanup.
                // Release chest callbacks cast the cached tile, so each support
                // case must start without a tile from the previous block kind.
                for(Chunk chunk:((EntityOracle.FlatProvider)w.chunkProvider).chunks.values())chunk.chunkTileEntityMap.clear();
                w.loadedTileEntityList.clear();
                for(int[] p:offsets)Mode_worldblocks10.raw(w,8+p[0],64,8+p[1],0,0);
                int[] p=sides[m&3];Mode_worldblocks10.raw(w,8+p[0],64,8+p[1],support,0);Mode_worldblocks10.raw(w,8,64,8,96,m);
                w.multiplayerWorld=remote;w.loadedEntityList.clear();seed(w);b.onNeighborBlockChange(w,8,64,8,change);result(o,w);
            }
            w.multiplayerWorld=false;
            for(int m=0;m<16;m++)for(boolean powered:new boolean[]{false,true}) {
                for(int[] p:offsets)Mode_worldblocks10.raw(w,8+p[0],64,8+p[1],1,0);
                Mode_worldblocks10.raw(w,8,64,8,96,m);w.loadedEntityList.clear();seed(w);b.onPoweredBlockChange(w,8,64,8,powered);result(o,w);
            }
        }
    }
}
