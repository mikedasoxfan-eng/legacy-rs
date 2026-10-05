package net.minecraft.src;
import java.io.*;
public final class Mode_wbchestopen5 {
    static final class Player extends EntityPlayerSP {
        IInventory inventory;
        Player(net.minecraft.client.Minecraft mc,World w){super(mc,w,new Session("Player",""),0);}
        public void displayGUIChest(IInventory inv){inventory=inv;}
        public void addStat(StatBase s,int count){}
    }
    public static void main(String[] a)throws Exception {
        int init=Block.stone.blockID;
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            for(int mask=0;mask<16;mask++)for(int invalid=0;invalid<32;invalid++)for(int roof=0;roof<6;roof++) {
                World w=Mode_worldblocks181.fixture(true);Player p=new Player((net.minecraft.client.Minecraft)Oracle.allocate(PlayerOracle.HeadlessMinecraft.class),w);
                int[][] positions={{8,8},{7,8},{9,8},{8,7},{8,9}};
                for(int n=0;n<5;n++)if(n==0||(mask&(1<<(n-1)))!=0){int x=positions[n][0],z=positions[n][1];Mode_worldblocks181.raw(w,x,64,z,54,0);TileEntityChest t=(TileEntityChest)w.getBlockTileEntity(x,64,z);t.setInventorySlotContents(0,new ItemStack(4+n,n+1,n));if((invalid&(1<<n))!=0)t.invalidate();}
                if(roof>0)Mode_worldblocks181.raw(w,positions[roof-1][0],65,positions[roof-1][1],1,0);
                o.writeBoolean(Block.chest.blockActivated(w,8,64,8,p));o.writeBoolean(p.inventory!=null);
                if(p.inventory!=null){EntityOracle.i(o,p.inventory.getSizeInventory());for(int n=0;n<p.inventory.getSizeInventory();n++)Mode_wbfurnace4.stack(o,p.inventory.getStackInSlot(n));}
            }
        }
    }
}
