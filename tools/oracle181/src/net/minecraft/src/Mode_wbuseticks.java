package net.minecraft.src;
import java.io.*;
public final class Mode_wbuseticks {
    static class Player extends EntityPlayer {
        Player(World w){super(w);}
        public void func_6420_o(){}
        public void onLivingUpdate(){} // Keep the original onUpdate, without movement/AI.
        public void addStat(StatBase stat,int count){}
    }
    public static void main(String[] a)throws Exception {
        int init=Block.stone.blockID;
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            for(int id:new int[]{260,282,322,365,367,267,261})for(int op=0;op<5;op++) {
                World w=Mode_worldblocks181.fixture(true);Player p=new Player(w);p.func_35191_at().func_35764_a(12);w.rand.setSeed(0);
                p.inventory.mainInventory[0]=new ItemStack(id,3,0);p.inventory.mainInventory[1]=new ItemStack(262,64,0);Item.itemsList[id].onItemRightClick(p.inventory.mainInventory[0],w,p);
                for(int tick=0;tick<40;tick++) {
                    if(tick==8) {
                        if(op==1)p.inventory.mainInventory[0]=new ItemStack(id,3,0);
                        if(op==2){p.inventory.mainInventory[2]=p.inventory.mainInventory[0];p.inventory.currentItem=2;}
                        if(op==3)p.func_35206_ab();
                        if(op==4)p.inventory.mainInventory[0].stackSize=2;
                    }
                    p.onUpdate();Mode_wbuse.state(o,p);
                }
            }
        }
    }
}
