package net.minecraft.src;
import java.io.*;
import java.lang.reflect.*;
public final class Mode_wbplate9 {
    static void schedules(DataOutputStream o,World w)throws Exception {Mode_wbrepeater8.queue(o,w);}
    public static void main(String[] a)throws Exception {
        int init=Block.stone.blockID;World w=Mode_worldblocks181.fixture(true);Mode_wbcake9.attach(w);
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            for(int id:new int[]{70,72}) {
                Block b=Block.blocksList[id];b.setBlockBoundsForItemRender();Mode_wbtrapdoor8.bounds(o,b);
                for(int m=0;m<16;m++){Mode_worldblocks181.raw(w,8,64,8,id,m);b.setBlockBoundsBasedOnState(w,8,64,8);Mode_wbtrapdoor8.bounds(o,b);for(int side:new int[]{Integer.MIN_VALUE,-1,0,1,2,3,4,5,6,Integer.MAX_VALUE}){o.writeBoolean(b.isPoweringTo(w,8,64,8,side));o.writeBoolean(b.isIndirectlyPoweringTo(w,8,64,8,side));}}
                for(boolean remote:new boolean[]{false,true})for(int m=0;m<16;m++)for(int kind=0;kind<4;kind++)for(int position=0;position<9;position++)for(boolean dead:new boolean[]{false,true})for(int hook=0;hook<3;hook++) {
                    w.loadedEntityList.clear();for(int cx=-3;cx<=3;cx++)for(int cz=-3;cz<=3;cz++){Chunk c=w.getChunkFromChunkCoords(cx,cz);for(Object list:c.entities)((java.util.List)list).clear();}
                    for(int x=7;x<=9;x++)for(int y=62;y<=65;y++)for(int z=7;z<=9;z++)Mode_worldblocks181.raw(w,x,y,z,0,0);
                    Mode_wbrepeater8.clear(w);Mode_worldblocks181.raw(w,8,63,8,1,0);Mode_worldblocks181.raw(w,8,64,8,id,m);w.multiplayerWorld=remote;Mode_wbtrapdoors6.seed(w);Mode_wbcake9.ranges.clear();
                    Entity e=kind==0?new EntityPig(w):kind==1?new EntityItem(w):kind==2?PlayerOracle.player(w):new EntitySnowball(w);
                    double[][] ps={{8.5,64,8.5},{7.5,64,8.5},{9.5,64,8.5},{8.5,64,7.5},{8.5,64,9.5},{8.5,63,8.5},{8.5,64.25,8.5},{8.5,64.249999,8.5},{8.5,64.5,8.5}};
                    e.setPosition(ps[position][0],ps[position][1],ps[position][2]);w.entityJoinedWorld(e);e.isDead=dead;
                    if(hook==0)b.onEntityCollidedWithBlock(w,8,64,8,e);else if(hook==1)b.updateTick(w,8,64,8,w.rand);else b.onBlockRemoval(w,8,64,8);
                    Mode_wbplacement4.cells(o,w,7,9,62,65);EntityOracle.l(o,EntityOracle.rngState(w.rand));WorldEventOracle.snapshot(w,o);schedules(o,w);Mode_wbcake9.ranges(o);
                }
            }
        }
    }
}
