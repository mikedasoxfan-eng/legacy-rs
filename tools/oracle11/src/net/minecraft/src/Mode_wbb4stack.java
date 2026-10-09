package net.minecraft.src;
import java.io.*;
import java.util.*;

/** Release ItemStack.damageItem: held-item Unbreaking (player inventory), damage/break/stat/count/reset, RNG and break effects. */
public final class Mode_wbb4stack {
    static final class StatPlayer extends EntityPlayerSP {
        ArrayList<Integer> stats=new ArrayList<Integer>();
        StatPlayer(net.minecraft.client.Minecraft mc,World w){super(mc,w,new Session("Player",""),0);}
        public void addStat(StatBase s,int n){stats.add(s==null?-1:s.statId);stats.add(n);}
    }
    public static void main(String[] a)throws Exception {
        int init=Block.stone.blockID;
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            for(int held=0;held<4;held++)for(int target=0;target<6;target++)for(int amount:new int[]{0,1,2,7,-1})for(int victim=0;victim<2;victim++) {
                World w=Mode_worldblocks10.fixture(true);
                StatPlayer p=new StatPlayer((net.minecraft.client.Minecraft)Oracle.allocate(PlayerOracle.HeadlessMinecraft.class),w);Blocks4Oracle.place(p,8.5,65,8.5,20F,10F);
                Entity zombie=EntityList.createEntityInWorld("Zombie",w);zombie.setLocationAndAngles(9.5,65,8.5,0F,0F);zombie.rand.setSeed(99);
                ItemStack h=held==0?null:new ItemStack(held==3?1:276,1,0);if(held==1)h.addEnchantment(Enchantment.enchantmentsList[34],1);if(held==2)h.addEnchantment(Enchantment.enchantmentsList[34],3);
                p.inventory.mainInventory[0]=h;
                ItemStack s;
                switch(target){case 0:s=h==null?new ItemStack(276,1,5):h;break;case 1:s=new ItemStack(310,1,0);break;case 2:s=new ItemStack(311,1,527);break;case 3:s=new ItemStack(257,1,249);break;case 4:s=new ItemStack(1,3,0);break;default:s=new ItemStack(261,2,383);}
                Blocks4Oracle.seeds(w);
                for(int k=0;k<3;k++){s.damageItem(amount,victim==0?p:(EntityLiving)zombie);Blocks4Oracle.stack(o,s);}
                Blocks4Oracle.rngs(o,w);EntityOracle.l(o,Trace10.namedLong("p.rand",EntityOracle.rngState(p.rand)));EntityOracle.l(o,Trace10.namedLong("zombie.rand",EntityOracle.rngState(zombie.rand)));
                WorldEventOracle.snapshot(w,o);Blocks4Oracle.i(o,p.stats.size());for(int v:p.stats)Blocks4Oracle.i(o,v);
            }
        }
    }
}
