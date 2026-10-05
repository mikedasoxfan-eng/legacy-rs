package net.minecraft.src;
import java.io.*;
public final class Mode_wbthin7 {
    public static void main(String[] a)throws Exception {
        int init=Block.stone.blockID;World w=Mode_worldblocks181.fixture(false);
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            for(int id:new int[]{37,38})for(boolean remote:new boolean[]{false,true})for(int floor=0;floor<=109;floor++)for(int light=0;light<16;light++)for(int m:new int[]{0,15})for(int hook=0;hook<2;hook++) {
                Mode_worldblocks181.raw(w,8,63,8,floor,0);Mode_worldblocks181.raw(w,8,64,8,id,m);w.getChunkFromChunkCoords(0,0).blocklightMap.setNibble(8,64,8,light);w.multiplayerWorld=remote;w.loadedEntityList.clear();Mode_wbtrapdoors6.seed(w);
                o.writeBoolean(Block.blocksList[id].canBlockStay(w,8,64,8));if(hook==0)Block.blocksList[id].onNeighborBlockChange(w,8,64,8,1);else Block.blocksList[id].updateTick(w,8,64,8,w.rand);Mode_wbtrapdoors6.result(o,w);
            }
            for(boolean remote:new boolean[]{false,true})for(int m=0;m<16;m++)for(int light=0;light<16;light++) {
                Mode_worldblocks181.raw(w,8,63,8,1,0);Mode_worldblocks181.raw(w,8,64,8,80,m);w.getChunkFromChunkCoords(0,0).blocklightMap.setNibble(8,64,8,light);w.multiplayerWorld=remote;w.loadedEntityList.clear();Mode_wbtrapdoors6.seed(w);
                Block.blockSnow.updateTick(w,8,64,8,w.rand);Mode_wbtrapdoors6.result(o,w);
            }
            for(int kind=0;kind<4;kind++)for(boolean initial:new boolean[]{false,true})for(float fall:new float[]{-1F,0F,20F,Float.NaN,Float.POSITIVE_INFINITY}) {
                Entity e=kind==0?new EntityPig(w):kind==1?new EntitySpider(w):kind==2?new EntityCaveSpider(w):new EntityItem(w,8,64,8,new ItemStack(1,1,0));e.isInWeb=initial;e.fallDistance=fall;e.motionX=.125;e.motionY=-.25;e.motionZ=.5;
                Block.web.onEntityCollidedWithBlock(w,8,64,8,e);o.writeBoolean(e.isInWeb);EntityOracle.f(o,e.fallDistance);for(double v:new double[]{e.motionX,e.motionY,e.motionZ})EntityOracle.d(o,v);
            }
            w=Mode_worldblocks181.fixture(true);WorldEventOracle.attach(w);Mode_worldblocks181.raw(w,8,64,8,25,0);
            for(int instrument:new int[]{Integer.MIN_VALUE,-1,0,1,2,3,4,5,Integer.MAX_VALUE})for(int note:new int[]{Integer.MIN_VALUE,-1000,-128,-1,0,12,24,127,1000,Integer.MAX_VALUE}) {
                Block.music.playBlock(w,8,64,8,instrument,note);WorldEventOracle.snapshot(w,o);
            }
        }
    }
}
