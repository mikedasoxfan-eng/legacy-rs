package net.minecraft.src;
import java.io.*;
import java.util.*;
/** Original player damage/death callbacks, especially source-without-attacker and self shooter. */
public final class Mode_playerdamage8_181 {
    static final class Player extends EntityPlayer {
        Player(World w){super(w);username="Player";}
        public void func_6420_o(){}
        public void addStat(StatBase s,int n){PlayerOracle.stat(this,s,n);}
    }
    public static void main(String[] a)throws Exception {
        World w=Mode_entities181.world();
        DamageSource[] sources={DamageSource.field_35542_a,DamageSource.field_35540_b,DamageSource.field_35541_c,
            DamageSource.field_35538_d,DamageSource.field_35539_e,DamageSource.field_35536_f,DamageSource.field_35537_g,
            DamageSource.field_35549_h,DamageSource.field_35550_i,DamageSource.field_35547_j,DamageSource.field_35548_k,DamageSource.field_35545_l};
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            for(int c=0;c<2400;c++) {
                w.loadedEntityList.clear();Player p=new Player(w);
                p.setLocationAndAngles(0.5,64,0.5,c*17%360,0);
                p.motionX=0.3;p.motionY=-0.1;p.motionZ=-0.4;
                p.health=c%3==0?2:20;p.prevHealth=17;p.heartsLife=c%5==0?20:0;p.field_9346_af=c%7;
                DamageSource source=c%2==0?sources[(c/2)%sources.length]:DamageSource.func_35535_a(null,p);
                o.writeBoolean(p.attackEntityFrom(source,c%23+1));
                for(int n:new int[]{p.health,p.prevHealth,p.heartsLife,p.hurtTime,p.maxHurtTime,p.field_9346_af,EntityOracle.intField(EntityLiving.class,"field_34905_c",p)})EntityOracle.i(o,n);
                for(double n:new double[]{p.motionX,p.motionY,p.motionZ,p.boundingBox.minY,p.boundingBox.maxY})EntityOracle.d(o,n);
                NBTTagCompound food=new NBTTagCompound();p.func_35191_at().func_35763_b(food);
                for(float n:new float[]{p.attackedAtYaw,p.width,p.height,p.yOffset,food.getFloat("foodExhaustionLevel")})EntityOracle.f(o,n);
                o.writeBoolean(p.beenAttacked);o.writeBoolean(p.field_35118_ao);
                EntityOracle.l(o,EntityOracle.rngState(p.rand));EntityOracle.l(o,EntityOracle.rngState(EntityOracle.mathRandom()));
                ArrayList<int[]> log=PlayerOracle.stats.remove(p);EntityOracle.i(o,log==null?0:log.size());
                if(log!=null)for(int[] pair:log){EntityOracle.i(o,pair[0]);EntityOracle.i(o,pair[1]);}
                WorldEventOracle.snapshot(w,o);
            }
        }
    }
}
