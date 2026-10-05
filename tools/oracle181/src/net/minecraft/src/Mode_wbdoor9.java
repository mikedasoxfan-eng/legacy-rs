package net.minecraft.src;
import java.io.*;
public final class Mode_wbdoor9 {
    public static void main(String[] a)throws Exception {
        int init=Block.stone.blockID;World w=Mode_worldblocks181.fixture(true);EntityPlayer p=PlayerOracle.player(w);Mode_wbcake9.attach(w);
        int[] values={Integer.MIN_VALUE,-100,-1,0,1,2,3,4,5,6,15,16,100,Integer.MAX_VALUE};
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            for(int id:new int[]{64,71}) {
                BlockDoor b=(BlockDoor)Block.blocksList[id];
                for(int m:values){EntityOracle.i(o,b.getState(m));o.writeBoolean(BlockDoor.isOpen(m));b.setDoorRotation(m);Mode_wbtrapdoor8.bounds(o,b);for(int side:values)EntityOracle.i(o,b.getBlockTextureFromSideAndMetadata(side,m));}
                for(boolean remote:new boolean[]{false,true})for(int m=0;m<16;m++)for(int config=0;config<16;config++)for(int hook=0;hook<5;hook++) {
                    for(int x=7;x<=9;x++)for(int y=62;y<=66;y++)for(int z=7;z<=9;z++)Mode_worldblocks181.raw(w,x,y,z,0,0);
                    Mode_worldblocks181.raw(w,8,63,8,(m&8)!=0?((config&1)!=0?id:0):((config&1)!=0?1:0),m&7);
                    Mode_worldblocks181.raw(w,8,65,8,(config&2)!=0?id:0,(m&7)|8);Mode_worldblocks181.raw(w,7,64,8,(config&4)!=0?76:0,5);Mode_worldblocks181.raw(w,8,64,8,id,m);
                    w.multiplayerWorld=remote;w.loadedEntityList.clear();Mode_wbtrapdoors6.seed(w);Mode_wbcake9.ranges.clear();
                    if(hook==0)b.onNeighborBlockChange(w,8,64,8,(config&8)!=0?76:1);else if(hook==1||hook==2)b.onPoweredBlockChange(w,8,64,8,hook==2);else if(hook==3)o.writeBoolean(b.blockActivated(w,8,64,8,p));else b.onBlockClicked(w,8,64,8,p);
                    Mode_wbtrapdoors6.result(o,w);Mode_wbcake9.ranges(o);
                }
                EntityOracle.FlatProvider provider=(EntityOracle.FlatProvider)w.chunkProvider;Chunk far=new Chunk(w,new byte[32768],16777216>>4,0);far.isChunkLoaded=true;provider.chunks.put(provider.key(16777216>>4,0),far);
                for(int x:new int[]{8,-17,16777216})for(int m=0;m<16;m++) {
                    Mode_worldblocks181.raw(w,x,64,8,id,m);b.setDoorRotation(-1);Mode_worldblocks181.box(o,b.getSelectedBoundingBoxFromPool(w,x,64,8));Mode_wbtrapdoor8.bounds(o,b);Mode_worldblocks181.box(o,b.getCollisionBoundingBoxFromPool(w,x,64,8));Mode_wbtrapdoor8.bounds(o,b);
                    for(int axis=0;axis<3;axis++)for(double offset:new double[]{0,0.1875,0.5,0.8125,1})for(boolean reverse:new boolean[]{false,true}) {
                        double[] s={x+offset,64+offset,8+offset},e=s.clone();s[axis]=new double[]{x,64,8}[axis]-1;e[axis]=new double[]{x,64,8}[axis]+2;
                        Vec3D start=Vec3D.createVector(s[0],s[1],s[2]),end=Vec3D.createVector(e[0],e[1],e[2]);Mode_wbtrapdoor8.hit(o,b.collisionRayTrace(w,x,64,8,reverse?end:start,reverse?start:end));Mode_wbtrapdoor8.bounds(o,b);
                    }
                }
            }
        }
    }
}
