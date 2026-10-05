package net.minecraft.src;
import java.io.*;
public final class Mode_wbactivation4 {
    static final class Player extends EntityPlayerSP {
        int opened;
        Player(net.minecraft.client.Minecraft mc,World w) {super(mc,w,new Session("Player",""),0);}
        public void displayGUIChest(IInventory inv) {opened++;}
        public void displayGUIFurnace(TileEntityFurnace tile) {opened++;}
        public void displayGUIDispenser(TileEntityDispenser tile) {opened++;}
        public void addStat(StatBase stat,int count) {}
    }
    public static void main(String[] a)throws Exception {
        int init=Block.stone.blockID;
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            for(int id:new int[]{23,25,54,61,62})for(int config=0;config<4;config++) {
                World w=Mode_worldblocks181.fixture(true);Player p=new Player((net.minecraft.client.Minecraft)Oracle.allocate(PlayerOracle.HeadlessMinecraft.class),w);w.setBlock(8,64,8,id);TileEntity tile=w.getBlockTileEntity(8,64,8);
                if((config&1)!=0)tile.invalidate();if((config&2)!=0)Mode_worldblocks181.raw(w,8,65,8,1,0);
                w.getChunkFromChunkCoords(0,0).isModified=false;WorldEventOracle.attach(w);w.rand.setSeed(42);
                o.writeBoolean(Block.blocksList[id].blockActivated(w,8,64,8,p));EntityOracle.i(o,p.opened);o.writeBoolean(w.getChunkFromChunkCoords(0,0).isModified);EntityOracle.l(o,EntityOracle.rngState(w.rand));WorldEventOracle.snapshot(w,o);
            }
        }
    }
}
