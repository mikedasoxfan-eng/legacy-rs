package net.minecraft.src;
import java.io.*;
import java.lang.reflect.*;
import java.util.*;
public final class Mode_wbstairs8 {
    public static void main(String[] a)throws Exception {
        int init=Block.stone.blockID;World w=Mode_worldblocks10.fixture(true);EntityPlayer p=PlayerOracle.player(w);Random r=new Random(91);
        int[] values={Integer.MIN_VALUE,-100,-1,0,1,2,3,4,5,6,15,16,100,Integer.MAX_VALUE};
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            for(int id:new int[]{53,67,108,109}) {
                Block b=Block.blocksList[id];int model=id==53?5:id==67?4:id==108?45:98;Block mb=Block.blocksList[model];
                for(int side:values)for(int m:values){EntityOracle.i(o,Trace10.namedInt("b.getBlockTextureFromSide(side)", b.getBlockTextureFromSide(side)));EntityOracle.i(o,Trace10.namedInt("b.getBlockTextureFromSideAndMetadata(side,m)", b.getBlockTextureFromSideAndMetadata(side,m)));}
                final ArrayList<int[]> calls=new ArrayList<int[]>();
                IBlockAccess access=(IBlockAccess)Proxy.newProxyInstance(IBlockAccess.class.getClassLoader(),new Class[]{IBlockAccess.class},(obj,method,args)->{int x=(Integer)args[0],y=(Integer)args[1],z=(Integer)args[2],min=(Integer)args[3];calls.add(new int[]{x,y,z,min});if(method.getName().equals("getBrightness"))return .625F;return x*31^y*17^z*7^min;});
                EntityOracle.f(o,Trace10.namedFloat("b.getBlockBrightness(access,-17,64,8)", b.getBlockBrightness(access,-17,64,8)));EntityOracle.i(o,Trace10.namedInt("b.getMixedBrightnessForBlock(access,-17,64,8)", b.getMixedBrightnessForBlock(access,-17,64,8)));EntityOracle.i(o,Trace10.namedInt("calls.size()", calls.size()));for(int[] c:calls)for(int v:c)EntityOracle.i(o,v);
                EntityOracle.f(o,Trace10.namedFloat("b.getExplosionResistance(null)", b.getExplosionResistance(null)));
                for(int x:new int[]{8,-17,16777216})for(int m=0;m<16;m++)for(int shape=0;shape<3;shape++) {
                    mb.setBlockBounds(shape==0?0F:shape==1?-.25F:.125F,0F,0F,shape==0?1F:shape==1?1.25F:.875F,1F,1F);
                    b.setBlockBounds(.25F,.25F,.25F,.75F,.75F,.75F);Mode_worldblocks10.box(o,b.getSelectedBoundingBoxFromPool(w,x,64,8));Mode_wbtrapdoor8.bounds(o,b);
                }
                mb.setBlockBounds(0,0,0,1,1,1);
                for(boolean remote:new boolean[]{false,true})for(int m=0;m<16;m++)for(int hook=0;hook<6;hook++) {
                    Mode_worldblocks10.raw(w,8,64,8,id,m);w.multiplayerWorld=remote;w.loadedEntityList.clear();Mode_wbtrapdoors6.seed(w);
                    if(hook==0)b.onBlockAdded(w,8,64,8);else if(hook==1)b.onBlockRemoval(w,8,64,8);else if(hook==2)b.onBlockClicked(w,8,64,8,p);else if(hook==3)b.updateTick(w,8,64,8,w.rand);else if(hook==4)Trace10.writeBoolean(o, "b.blockActivated(w,8,64,8,p)", b.blockActivated(w,8,64,8,p));else b.onEntityWalking(w,8,64,8,p);
                    Mode_wbtrapdoors6.result(o,w);Mode_wbrepeater8.queue(o,w);b.randomDisplayTick(w,8,64,8,r);EntityOracle.l(o,Trace10.namedLong("EntityOracle.rngState(r)", EntityOracle.rngState(r)));WorldEventOracle.snapshot(w,o);
                }
                for(float yaw:new float[]{Float.NEGATIVE_INFINITY,-Float.MAX_VALUE,-360,-45,0,45,180,360,Float.MAX_VALUE,Float.POSITIVE_INFINITY,Float.NaN}) {
                    Mode_worldblocks10.raw(w,8,64,8,id,15);p.rotationYaw=yaw;Mode_wbtrapdoors6.seed(w);b.onBlockPlacedBy(w,8,64,8,p);Mode_wbtrapdoors6.result(o,w);
                }
            }
        }
    }
}
