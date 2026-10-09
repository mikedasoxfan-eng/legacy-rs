package net.minecraft.src;
import java.io.*;
import java.lang.reflect.*;
import java.util.*;

/** New Block access methods and the base harvest callback use original bytecode. */
public final class Mode_wbbase6 {
    public static void main(String[] a)throws Exception {
        int init=Block.stone.blockID;
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            for(int id=1;id<=109;id++)if(Block.blocksList[id]!=null && (id<8||id>11))for(boolean normal:new boolean[]{false,true})for(int[] pos:new int[][]{{8,64,8},{-17,-1,31},{Integer.MIN_VALUE,Integer.MAX_VALUE,-30000001}}) {
                final ArrayList<int[]> calls=new ArrayList<int[]>();
                IBlockAccess access=(IBlockAccess)Proxy.newProxyInstance(IBlockAccess.class.getClassLoader(),new Class[]{IBlockAccess.class},(p,m,v)->{
                    int x=(Integer)v[0],y=(Integer)v[1],z=(Integer)v[2];
                    if(m.getName().equals("isBlockNormalCube")){calls.add(new int[]{x,y,z});return normal;}
                    int min=(Integer)v[3];calls.add(new int[]{x,y,z,min});
                    if(m.getName().equals("getBrightness"))return min*.125F;
                    if(m.getName().equals("getLightBrightnessForSkyBlocks"))return x*31^y*17^z*7^min;
                    throw new UnsupportedOperationException(m.getName());
                });
                Block b=Block.blocksList[id];
                EntityOracle.f(o,Trace10.namedFloat("b.getAmbientOcclusionLightValue(access,pos[0],pos[1],pos[2])", b.getAmbientOcclusionLightValue(access,pos[0],pos[1],pos[2])));
                EntityOracle.f(o,Trace10.namedFloat("b.getBlockBrightness(access,pos[0],pos[1],pos[2])", b.getBlockBrightness(access,pos[0],pos[1],pos[2])));
                EntityOracle.i(o,Trace10.namedInt("b.getMixedBrightnessForBlock(access,pos[0],pos[1],pos[2])", b.getMixedBrightnessForBlock(access,pos[0],pos[1],pos[2])));
                if(id!=2&&id!=18&&id!=31&&id!=106)EntityOracle.i(o,Trace10.namedInt("b.getBlockColor()", b.getBlockColor()));
                EntityOracle.i(o,Trace10.namedInt("calls.size()", calls.size()));for(int[] call:calls){EntityOracle.i(o,Trace10.namedInt("call.length", call.length));for(int v:call)EntityOracle.i(o,v);}
            }
            World w=Mode_worldblocks10.fixture(true);EntityPlayer p=PlayerOracle.player(w);
            Field exhaustion=FoodStats.class.getDeclaredField("foodExhaustionLevel");exhaustion.setAccessible(true);
            for(int id:new int[]{1,3,4,7,22,45,48,57,87,98})for(boolean remote:new boolean[]{false,true})for(boolean disabled:new boolean[]{false,true})for(int meta=0;meta<16;meta++)for(float start:new float[]{-1F,0F,39.99F,40F,Float.POSITIVE_INFINITY,Float.NaN}) {
                w.multiplayerWorld=remote;p.capabilities.disableDamage=disabled;exhaustion.setFloat(p.foodStats,start);
                w.loadedEntityList.clear();PlayerOracle.stats.remove(p);Mode_wbtrapdoors6.seed(w);
                Block.blocksList[id].harvestBlock(w,p,8,64,8,meta);
                EntityOracle.f(o,Trace10.namedFloat("exhaustion.getFloat(p.foodStats)", exhaustion.getFloat(p.foodStats)));
                ArrayList<int[]> stats=PlayerOracle.stats.remove(p);EntityOracle.i(o,Trace10.namedInt("stats==null?0:stats.size()", stats==null?0:stats.size()));if(stats!=null)for(int[] stat:stats)for(int v:stat)EntityOracle.i(o,v);
                Mode_wbtrapdoors6.result(o,w);
            }
        }
    }
}
