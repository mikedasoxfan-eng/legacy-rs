package net.minecraft.src;
import java.io.*;
import java.util.*;
public final class Mode_wbshears4 {
    public static void main(String[] a)throws Exception {
        int init=Block.stone.blockID;
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            for(int id=0;id<=109;id++)for(int damage:new int[]{0,237,238,239,32767,-1}) {
                World w=Mode_worldblocks10.fixture(true);EntityPlayer p=PlayerOracle.player(w);ItemStack s=new ItemStack(359,1,damage);p.inventory.mainInventory[0]=s;
                s.onDestroyBlock(id,8,64,8,p);Trace10.writeBoolean(o, "s.stackSize!=0", s.stackSize!=0);if(s.stackSize!=0){EntityOracle.i(o,Trace10.namedInt("s.stackSize", s.stackSize));EntityOracle.i(o,Trace10.namedInt("s.getItemDamage()", s.getItemDamage()));}
                ArrayList<int[]> stats=PlayerOracle.stats.remove(p);EntityOracle.i(o,Trace10.namedInt("stats==null?0:stats.size()", stats==null?0:stats.size()));if(stats!=null)for(int[] v:stats){EntityOracle.i(o,Trace10.namedInt("v[0]", v[0]));EntityOracle.i(o,Trace10.namedInt("v[1]", v[1]));}
            }
        }
    }
}
