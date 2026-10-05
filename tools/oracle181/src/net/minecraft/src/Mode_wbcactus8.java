package net.minecraft.src;
import java.io.*;
public final class Mode_wbcactus8 {
    public static void main(String[] a)throws Exception {
        int init=Block.stone.blockID;World w=Mode_worldblocks181.fixture(true);Block b=Block.cactus;
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            for(int side:new int[]{Integer.MIN_VALUE,-1,0,1,2,3,4,5,6,Integer.MAX_VALUE})EntityOracle.i(o,b.getBlockTextureFromSide(side));
            for(int x:new int[]{Integer.MIN_VALUE,-30000001,-16777217,-17,0,8,16777216,30000000,Integer.MAX_VALUE})for(int y:new int[]{Integer.MIN_VALUE,-1,0,64,127,16777216,Integer.MAX_VALUE})for(int z:new int[]{Integer.MIN_VALUE,-17,8,16777216,Integer.MAX_VALUE}) {
                Mode_worldblocks181.box(o,b.getCollisionBoundingBoxFromPool(w,x,y,z));Mode_worldblocks181.box(o,b.getSelectedBoundingBoxFromPool(w,x,y,z));
            }
            for(boolean remote:new boolean[]{false,true})for(int floor=0;floor<=109;floor++)for(int neighbor=0;neighbor<=109;neighbor++)for(int side=0;side<4;side++) {
                w.getChunkFromChunkCoords(0,0).chunkTileEntityMap.clear();w.loadedTileEntityList.clear();
                int[][] ds={{-1,0},{1,0},{0,-1},{0,1}};for(int[] d:ds)Mode_worldblocks181.raw(w,8+d[0],64,8+d[1],0,0);int[] d=ds[side];Mode_worldblocks181.raw(w,8+d[0],64,8+d[1],neighbor,0);
                Mode_worldblocks181.raw(w,8,63,8,floor,0);Mode_worldblocks181.raw(w,8,64,8,81,15);w.multiplayerWorld=remote;w.loadedEntityList.clear();Mode_wbtrapdoors6.seed(w);
                o.writeBoolean(b.canBlockStay(w,8,64,8));b.onNeighborBlockChange(w,8,64,8,1);Mode_wbtrapdoors6.result(o,w);
            }
            for(boolean remote:new boolean[]{false,true})for(int m=0;m<16;m++)for(int height=1;height<=5;height++)for(int above:new int[]{0,1,81}) {
                for(int x=7;x<=9;x++)for(int y=58;y<=69;y++)for(int z=7;z<=9;z++)Mode_worldblocks181.raw(w,x,y,z,0,0);
                int bottom=65-height;Mode_worldblocks181.raw(w,8,bottom-1,8,12,0);for(int y=bottom;y<=64;y++)Mode_worldblocks181.raw(w,8,y,8,81,y==64?m:0);Mode_worldblocks181.raw(w,8,65,8,above,0);
                w.multiplayerWorld=remote;w.loadedEntityList.clear();Mode_wbtrapdoors6.seed(w);
                for(int n=0;n<32;n++){b.updateTick(w,8,64,8,w.rand);Mode_wbplacement4.cells(o,w,7,9,58,69);EntityOracle.l(o,EntityOracle.rngState(w.rand));WorldEventOracle.snapshot(w,o);}
            }
            for(boolean remote:new boolean[]{false,true})for(int health:new int[]{-1,1,10,20})for(int immunity:new int[]{0,11,20}) {
                w.multiplayerWorld=remote;w.loadedEntityList.clear();Mode_wbtrapdoors6.seed(w);EntityPig e=new EntityPig(w);e.setPosition(8,64,8);e.health=health;e.heartsLife=immunity;e.field_9346_af=0;
                for(int n=0;n<3;n++){b.onEntityCollidedWithBlock(w,8,64,8,e);for(int v:new int[]{e.health,e.prevHealth,e.heartsLife,e.hurtTime,e.maxHurtTime,e.field_9346_af})EntityOracle.i(o,v);o.writeBoolean(e.beenAttacked);o.writeBoolean(e.isDead);EntityOracle.l(o,EntityOracle.rngState(e.rand));Mode_wbtrapdoors6.result(o,w);}
            }
        }
    }
}
