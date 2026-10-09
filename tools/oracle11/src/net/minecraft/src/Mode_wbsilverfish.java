package net.minecraft.src;
import java.io.*;
import java.util.*;
public final class Mode_wbsilverfish {
    public static void main(String[] a)throws Exception {
        int init=Block.stone.blockID;
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            for(int seed=0;seed<64;seed++) {
                World w=Mode_worldblocks10.fixture(true);OracleHooks.reset(seed);EntityOracle.mathRandom().setSeed(123);final ArrayList<Object[]> events=new ArrayList<>();
                w.addWorldAccess((IWorldAccess)java.lang.reflect.Proxy.newProxyInstance(IWorldAccess.class.getClassLoader(),new Class[]{IWorldAccess.class},(p,m,v)->{if(m.getName().equals("spawnParticle"))events.add(v);return null;}));
                Block.blocksList[97].onBlockDestroyedByPlayer(w,8,64,8,seed%16);EntityOracle.i(o,Trace10.namedInt("w.loadedEntityList.size()", w.loadedEntityList.size()));Entity e=(Entity)w.loadedEntityList.get(0);
                for(double v:new double[]{Trace10.namedDouble("e.posX", e.posX),Trace10.namedDouble("e.posY", e.posY),Trace10.namedDouble("e.posZ", e.posZ)})EntityOracle.d(o,v);EntityOracle.f(o,Trace10.namedFloat("e.width", e.width));EntityOracle.f(o,Trace10.namedFloat("e.height", e.height));EntityOracle.i(o,Trace10.namedInt("((EntityLiving)e).health", ((EntityLiving)e).health));EntityOracle.l(o,Trace10.namedLong("EntityOracle.rngState(e.rand)", EntityOracle.rngState(e.rand)));EntityOracle.l(o,Trace10.namedLong("EntityOracle.rngState(EntityOracle.mathRandom())", EntityOracle.rngState(EntityOracle.mathRandom())));
                EntityOracle.i(o,Trace10.namedInt("events.size()", events.size()));for(Object[] v:events)for(int j=1;j<v.length;j++)EntityOracle.d(o,Trace10.namedDouble("((Number)v[j]).doubleValue()", ((Number)v[j]).doubleValue()));
            }
            for(int id=0;id<=109;id++){Trace10.writeBoolean(o, "BlockSilverfish.getPosingIdByMetadata(id)", BlockSilverfish.getPosingIdByMetadata(id));EntityOracle.i(o,Trace10.namedInt("BlockSilverfish.func_35304_f(id)", BlockSilverfish.func_35304_f(id)));}
        }
    }
}
