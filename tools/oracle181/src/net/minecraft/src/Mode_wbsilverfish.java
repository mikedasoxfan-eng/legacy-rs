package net.minecraft.src;
import java.io.*;
import java.util.*;
public final class Mode_wbsilverfish {
    public static void main(String[] a)throws Exception {
        int init=Block.stone.blockID;
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            for(int seed=0;seed<64;seed++) {
                World w=Mode_worldblocks181.fixture(true);OracleHooks.reset(seed);EntityOracle.mathRandom().setSeed(123);final ArrayList<Object[]> events=new ArrayList<>();
                w.addWorldAccess((IWorldAccess)java.lang.reflect.Proxy.newProxyInstance(IWorldAccess.class.getClassLoader(),new Class[]{IWorldAccess.class},(p,m,v)->{if(m.getName().equals("spawnParticle"))events.add(v);return null;}));
                Block.blocksList[97].onBlockDestroyedByPlayer(w,8,64,8,seed%16);EntityOracle.i(o,w.loadedEntityList.size());Entity e=(Entity)w.loadedEntityList.get(0);
                for(double v:new double[]{e.posX,e.posY,e.posZ})EntityOracle.d(o,v);EntityOracle.f(o,e.width);EntityOracle.f(o,e.height);EntityOracle.i(o,((EntityLiving)e).health);EntityOracle.l(o,EntityOracle.rngState(e.rand));EntityOracle.l(o,EntityOracle.rngState(EntityOracle.mathRandom()));
                EntityOracle.i(o,events.size());for(Object[] v:events)for(int j=1;j<v.length;j++)EntityOracle.d(o,((Number)v[j]).doubleValue());
            }
            for(int id=0;id<=109;id++){o.writeBoolean(BlockSilverfish.func_35305_d(id));EntityOracle.i(o,BlockSilverfish.func_35304_f(id));}
        }
    }
}
