package net.minecraft.src;
import java.io.*;
import java.util.*;
/** Original player damage/death callbacks, especially source-without-attacker and self shooter. */
public final class Mode_playerdamage8_10 {
    static final class Player extends EntityPlayer {
        Player(World w){super(w);username="Player";}
        public void func_6420_o(){}
        public void addStat(StatBase s,int n){PlayerOracle.stat(this,s,n);}
    }
    public static void main(String[] a)throws Exception {
        World w=Mode_entities10.world();
        DamageSource[] sources={DamageSource.inFire,DamageSource.onFire,DamageSource.lava,
            DamageSource.inWall,DamageSource.drown,DamageSource.starve,DamageSource.cactus,
            DamageSource.fall,DamageSource.outOfWorld,DamageSource.generic,DamageSource.explosion,DamageSource.magic};
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            for(int c=0;c<2400;c++) {
                w.loadedEntityList.clear();Player p=new Player(w);
                p.setLocationAndAngles(0.5,64,0.5,c*17%360,0);
                p.motionX=0.3;p.motionY=-0.1;p.motionZ=-0.4;
                p.health=c%3==0?2:20;p.prevHealth=17;p.heartsLife=c%5==0?20:0;p.naturalArmorRating=c%7;
                DamageSource source=c%2==0?sources[(c/2)%sources.length]:DamageSource.causeArrowDamage(null,p);
                Trace10.writeBoolean(o, "p.attackEntityFrom(source,c%23+1)", p.attackEntityFrom(source,c%23+1));
                for(int n:new int[]{Trace10.namedInt("p.health", p.health),Trace10.namedInt("p.prevHealth", p.prevHealth),Trace10.namedInt("p.heartsLife", p.heartsLife),Trace10.namedInt("p.hurtTime", p.hurtTime),Trace10.namedInt("p.maxHurtTime", p.maxHurtTime),Trace10.namedInt("p.naturalArmorRating", p.naturalArmorRating),Trace10.namedInt("EntityOracle.intField(EntityLiving.class,\"field_34905_c\",p)", EntityOracle.intField(EntityLiving.class,"recentlyHit",p))})EntityOracle.i(o,n);
                for(double n:new double[]{Trace10.namedDouble("p.motionX", p.motionX),Trace10.namedDouble("p.motionY", p.motionY),Trace10.namedDouble("p.motionZ", p.motionZ),Trace10.namedDouble("p.boundingBox.minY", p.boundingBox.minY),Trace10.namedDouble("p.boundingBox.maxY", p.boundingBox.maxY)})EntityOracle.d(o,n);
                NBTTagCompound food=new NBTTagCompound();p.getFoodStats().writeStatsToNBT(food);
                for(float n:new float[]{Trace10.namedFloat("p.attackedAtYaw", p.attackedAtYaw),Trace10.namedFloat("p.width", p.width),Trace10.namedFloat("p.height", p.height),Trace10.namedFloat("p.yOffset", p.yOffset),Trace10.namedFloat("food.getFloat(\"foodExhaustionLevel\")", food.getFloat("foodExhaustionLevel"))})EntityOracle.f(o,n);
                Trace10.writeBoolean(o, "p.beenAttacked", p.velocityChanged);Trace10.writeBoolean(o, "p.isAirBorne", p.isAirBorne);
                EntityOracle.l(o,Trace10.namedLong("EntityOracle.rngState(p.rand)", EntityOracle.rngState(p.rand)));EntityOracle.l(o,Trace10.namedLong("EntityOracle.rngState(EntityOracle.mathRandom())", EntityOracle.rngState(EntityOracle.mathRandom())));
                ArrayList<int[]> log=PlayerOracle.stats.remove(p);EntityOracle.i(o,Trace10.namedInt("log==null?0:log.size()", log==null?0:log.size()));
                if(log!=null)for(int[] pair:log){EntityOracle.i(o,Trace10.namedInt("pair[0]", pair[0]));EntityOracle.i(o,Trace10.namedInt("pair[1]", pair[1]));}
                WorldEventOracle.snapshot(w,o);
            }
        }
    }
}
