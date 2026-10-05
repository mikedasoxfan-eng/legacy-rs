package net.minecraft.src;

import java.io.*;

/** EntityPlayer's literal melee policy, including refused hits and potion mining. */
public final class Mode_playercombat181 {
    static class Player extends EntityPlayer {
        Player(World w){super(w);username="Player";}
        public void func_6420_o(){}
        public void addStat(StatBase s,int n){PlayerOracle.stat(this,s,n);}
    }
    public static void main(String[] args)throws Exception {
        World w=Mode_entities181.world();
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
                p.func_35113_c(c%3==0);if(c%11==0)PlayerControllerTest.func_35646_d(p);
                target.heartsLife=c%6==0?20:0;target.field_9346_af=c%13;
                p.attackTargetEntityWithCurrentItem(target);
                for(int v:new int[]{target.health,target.heartsLife,target.hurtTime,target.field_9346_af})EntityOracle.i(o,v);
                for(double v:new double[]{p.motionX,p.motionY,p.motionZ,target.motionX,target.motionY,target.motionZ})EntityOracle.d(o,v);
                o.writeBoolean(p.func_35117_Q());o.writeBoolean(target.field_35118_ao);
                NBTTagCompound food=new NBTTagCompound();p.func_35191_at().func_35763_b(food);EntityOracle.f(o,food.getFloat("foodExhaustionLevel"));
                PlayerOracle.stack(o,p.inventory.mainInventory[0]);
                java.util.ArrayList<int[]> stats=PlayerOracle.stats.get(p);EntityOracle.i(o,stats==null?0:stats.size());if(stats!=null)for(int[] row:stats){EntityOracle.i(o,row[0]);EntityOracle.i(o,row[1]);}
                EntityOracle.l(o,EntityOracle.rngState(p.rand));EntityOracle.l(o,EntityOracle.rngState(target.rand));WorldEventOracle.snapshot(w,o);
            }
            Player p=new Player(w);p.onGround=true;
            for(int block:new int[]{1,3,12,14,49,7,18,30})for(int weapon:new int[]{-1,257,269,359})for(int amp=-1;amp<8;amp++)for(int effect:new int[]{3,4})for(int state=0;state<4;state++) {
                p.field_35170_bR.clear();if(amp>=0)p.func_35165_a(new PotionEffect(effect,100,amp));
                p.inventory.mainInventory[0]=weapon<0?null:new ItemStack(weapon,1,0);p.onGround=(state&1)==0;
                EntityOracle.rawBlock(w,0,64,0,(state&2)!=0?9:0,0);
                p.setLocationAndAngles(0.5,64,0.5,0,0);
                EntityOracle.f(o,Block.blocksList[block].blockStrength(p));
            }
        }
    }
}
