package net.minecraft.src;
import java.io.*;
import java.util.*;
public final class Mode_wbweather {
    static void state(DataOutputStream o,World w)throws Exception {
        WorldInfo n=w.getWorldInfo();EntityOracle.i(o,n.getRainTime());EntityOracle.i(o,n.getThunderTime());
        o.writeBoolean(n.getIsRaining());o.writeBoolean(n.getIsThundering());EntityOracle.i(o,w.lastLightningBolt);
        for(float f:new float[]{w.rainingStrength,w.prevRainingStrength,w.thunderingStrength,w.prevThunderingStrength})EntityOracle.f(o,f);
        EntityOracle.l(o,EntityOracle.rngState(w.rand));
        for(float partial:new float[]{0.0F,0.5F,1.0F,-1.0F,2.0F}) {
            EntityOracle.f(o,w.getRainStrength(partial));EntityOracle.f(o,w.getWeightedThunderStrength(partial));EntityOracle.i(o,w.calculateSkylightSubtracted(partial));
        }
        o.writeBoolean(w.isRaining());o.writeBoolean(w.getIsThundering());
    }
    public static void main(String[] a)throws Exception {
        int init=Block.stone.blockID;Random r=new Random(81731);World w=Mode_worldblocks181.fixture(true);
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            for(int dim=0;dim<2;dim++) {
                w.worldProvider.hasNoSky=dim!=0;w.worldProvider.isHellWorld=dim!=0;
                // calculateCelestialAngle dispatch must use the original provider class.
                if(dim!=0)w=new World(w,new WorldProviderHell());
                for(int n=0;n<512;n++) {
                    w.rand.setSeed(r.nextLong());w.getWorldInfo().setWorldTime(r.nextLong());
                    w.getWorldInfo().setRainTime(new int[]{-1,0,1,2,20,Integer.MAX_VALUE}[n%6]);
                    w.getWorldInfo().setThunderTime(new int[]{-1,0,1,2,20,Integer.MAX_VALUE}[(n/6)%6]);
                    w.getWorldInfo().setIsRaining((n&1)!=0);w.getWorldInfo().setIsThundering((n&2)!=0);w.lastLightningBolt=n%4-1;
                    w.rainingStrength=n%4==0?Float.intBitsToFloat(r.nextInt()):r.nextFloat();
                    w.thunderingStrength=n%4==0?Float.intBitsToFloat(r.nextInt()):r.nextFloat();
                    w.prevRainingStrength=-0.0F;w.prevThunderingStrength=1.0F;
                    for(int tick=0;tick<32;tick++){w.updateWeather();state(o,w);}
                }
            }
        }
    }
}
