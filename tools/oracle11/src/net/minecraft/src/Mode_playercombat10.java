package net.minecraft.src;

import java.io.*;

/** EntityPlayer's literal melee policy, including refused hits and potion mining. */
public final class Mode_playercombat10 {
    static class Player extends EntityPlayer {
        Player(World w){super(w);username="Player";}
        public void func_6420_o(){}
        public void addStat(StatBase s,int n){PlayerOracle.stat(this,s,n);}
    }
    public static void main(String[] args)throws Exception {
        World w=Mode_entities10.world();
        int[] weapons={-1,268,272,267,276,283,257,258,256,329};
        try(DataOutputStream o=new DataOutputStream(new FileOutputStream(args[1]))) {
            for(int c=0;c<600;c++) {
                Player p=new Player(w);EntityPig target=new EntityPig(w);
                p.setLocationAndAngles(0.5,64,0.5,c*17%360,0);
                target.setLocationAndAngles(0.5,64,1.5,0,0);target.health=1000;
                int weapon=weapons[c%weapons.length];
                if(weapon>=0)p.inventory.mainInventory[0]=new ItemStack(weapon,1,c%3==0?Item.itemsList[weapon].getMaxDamage():0);
                p.motionX=0.3;p.motionZ=-0.4;p.motionY=c%2==0?-0.1:0.1;
                p.onGround=c%4==0;p.inWater=c%5==0;
                EntityOracle.rawBlock(w,0,64,0,c%7==0?65:0,3);
                p.setSprinting(c%3==0);if(c%11==0)PlayerControllerCreative.enableAbilities(p);
                target.heartsLife=c%6==0?20:0;target.naturalArmorRating=c%13;
                p.attackTargetEntityWithCurrentItem(target);
                for(int v:new int[]{Trace10.namedInt("target.health", target.health),Trace10.namedInt("target.heartsLife", target.heartsLife),Trace10.namedInt("target.hurtTime", target.hurtTime),Trace10.namedInt("target.naturalArmorRating", target.naturalArmorRating)})EntityOracle.i(o,v);
                for(double v:new double[]{Trace10.namedDouble("p.motionX", p.motionX),Trace10.namedDouble("p.motionY", p.motionY),Trace10.namedDouble("p.motionZ", p.motionZ),Trace10.namedDouble("target.motionX", target.motionX),Trace10.namedDouble("target.motionY", target.motionY),Trace10.namedDouble("target.motionZ", target.motionZ)})EntityOracle.d(o,v);
                Trace10.writeBoolean(o, "p.isSprinting()", p.isSprinting());Trace10.writeBoolean(o, "target.isAirBorne", target.isAirBorne);
                NBTTagCompound food=new NBTTagCompound();p.getFoodStats().writeStatsToNBT(food);EntityOracle.f(o,Trace10.namedFloat("food.getFloat(\"foodExhaustionLevel\")", food.getFloat("foodExhaustionLevel")));
                PlayerOracle.stack(o,p.inventory.mainInventory[0]);
                java.util.ArrayList<int[]> stats=PlayerOracle.stats.get(p);EntityOracle.i(o,Trace10.namedInt("stats==null?0:stats.size()", stats==null?0:stats.size()));if(stats!=null)for(int[] row:stats){EntityOracle.i(o,Trace10.namedInt("row[0]", row[0]));EntityOracle.i(o,Trace10.namedInt("row[1]", row[1]));}
                EntityOracle.l(o,Trace10.namedLong("EntityOracle.rngState(p.rand)", EntityOracle.rngState(p.rand)));EntityOracle.l(o,Trace10.namedLong("EntityOracle.rngState(target.rand)", EntityOracle.rngState(target.rand)));WorldEventOracle.snapshot(w,o);
            }
            Player p=new Player(w);p.onGround=true;
            for(int block:new int[]{1,3,12,14,49,7,18,30})for(int weapon:new int[]{-1,257,269,359})for(int amp=-1;amp<8;amp++)for(int effect:new int[]{3,4})for(int state=0;state<4;state++) {
                p.activePotionsMap.clear();if(amp>=0)p.addPotionEffect(new PotionEffect(effect,100,amp));
                p.inventory.mainInventory[0]=weapon<0?null:new ItemStack(weapon,1,0);p.onGround=(state&1)==0;
                EntityOracle.rawBlock(w,0,64,0,(state&2)!=0?9:0,0);
                p.setLocationAndAngles(0.5,64,0.5,0,0);
                EntityOracle.f(o,Trace10.namedFloat("Block.blocksList[block].blockStrength(p)", Block.blocksList[block].blockStrength(p)));
            }
            for(int c=0;c<100;c++) {
                w.loadedEntityList.clear();
                Player dropper=new Player(w);dropper.setLocationAndAngles(0.5,64,0.5,c*17%360,c%90-45);
                dropper.inventory.mainInventory[0]=new ItemStack(260,c%65,0);
                for(int n=0;n<c%70;n++)dropper.dropCurrentItem();
                PlayerOracle.stack(o,dropper.inventory.mainInventory[0]);EntityOracle.l(o,Trace10.namedLong("EntityOracle.rngState(dropper.rand)", EntityOracle.rngState(dropper.rand)));
                java.util.ArrayList<int[]> stats=PlayerOracle.stats.get(dropper);EntityOracle.i(o,Trace10.namedInt("stats==null?0:stats.size()", stats==null?0:stats.size()));if(stats!=null)for(int[] row:stats){EntityOracle.i(o,Trace10.namedInt("row[0]", row[0]));EntityOracle.i(o,Trace10.namedInt("row[1]", row[1]));}
                EntityOracle.i(o,Trace10.namedInt("w.loadedEntityList.size()", w.loadedEntityList.size()));
                for(Object item:w.loadedEntityList)Mode_entities10.state(w,(Entity)item,o);
            }
        }
    }
}
