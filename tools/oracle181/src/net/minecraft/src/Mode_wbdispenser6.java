package net.minecraft.src;
import java.io.*;
import java.lang.reflect.*;
import java.util.*;

/** BlockDispenser callbacks complement dispenser4, removal5 and the visual tables. */
public final class Mode_wbdispenser6 {
    static final class Player extends EntityPlayerSP {
        int opened;
        Player(World w) throws Exception { super((net.minecraft.client.Minecraft)Oracle.allocate(PlayerOracle.HeadlessMinecraft.class),w,new Session("Player",""),0); }
        public void displayGUIDispenser(TileEntityDispenser d) { opened++; }
        public void addStat(StatBase stat,int count) {}
    }
    static void snapshot(DataOutputStream o, World w) throws Exception {
        EntityOracle.i(o,w.getBlockMetadata(8,64,8));
        TileEntity t=w.getBlockTileEntity(8,64,8);
        o.writeBoolean(t instanceof TileEntityDispenser);
        if(t instanceof TileEntityDispenser)for(int n=0;n<9;n++)Mode_wbfurnace4.stack(o,((TileEntityDispenser)t).getStackInSlot(n));
        TreeSet<NextTickListEntry> q=(TreeSet<NextTickListEntry>)Mode_wbnextticks.field("scheduledTickTreeSet").get(w);
        EntityOracle.i(o,q.size());
        for(NextTickListEntry e:q){for(int v:new int[]{e.xCoord,e.yCoord,e.zCoord,e.blockID})EntityOracle.i(o,v);EntityOracle.l(o,e.scheduledTime);}
        EntityOracle.l(o,EntityOracle.rngState(w.rand));
        EntityOracle.i(o,w.loadedEntityList.size());
        for(Object object:w.loadedEntityList){Entity e=(Entity)object;for(double v:new double[]{e.posX,e.posY,e.posZ,e.motionX,e.motionY,e.motionZ})EntityOracle.d(o,v);}
        WorldEventOracle.snapshot(w,o);
    }
    public static void main(String[] a)throws Exception {
        int init=Block.stone.blockID;
        Field random=TileEntityDispenser.class.getDeclaredField("dispenserRandom");random.setAccessible(true);
        Block b=Block.dispenser;
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            for(boolean remote:new boolean[]{false,true})for(int meta=0;meta<16;meta++)for(int mask=0;mask<16;mask++) {
                World w=Mode_worldblocks181.fixture(true);
                int[][] sides={{0,-1},{0,1},{-1,0},{1,0}};
                for(int k=0;k<4;k++)Mode_worldblocks181.raw(w,8+sides[k][0],64,8+sides[k][1],(mask&(1<<k))!=0?1:0,0);
                Mode_worldblocks181.raw(w,8,64,8,23,meta);w.multiplayerWorld=remote;Mode_wbtrapdoors6.seed(w);
                b.onBlockAdded(w,8,64,8);snapshot(o,w);
            }
            for(boolean remote:new boolean[]{false,true})for(boolean invalid:new boolean[]{false,true})for(int meta=0;meta<16;meta++) {
                World w=Mode_worldblocks181.fixture(true);Player p=new Player(w);
                Mode_worldblocks181.raw(w,8,64,8,23,meta);TileEntity t=w.getBlockTileEntity(8,64,8);if(invalid)t.invalidate();
                w.multiplayerWorld=remote;Mode_wbtrapdoors6.seed(w);
                o.writeBoolean(b.blockActivated(w,8,64,8,p));EntityOracle.i(o,p.opened);snapshot(o,w);
            }
            for(boolean remote:new boolean[]{false,true})for(boolean invalid:new boolean[]{false,true}) {
                World w=Mode_worldblocks181.fixture(true);Mode_worldblocks181.raw(w,8,64,8,23,15);
                TileEntityDispenser old=(TileEntityDispenser)w.getBlockTileEntity(8,64,8);old.setInventorySlotContents(0,new ItemStack(4,32,2));if(invalid)old.invalidate();
                w.multiplayerWorld=remote;Mode_wbtrapdoors6.seed(w);b.onBlockAdded(w,8,64,8);
                o.writeBoolean(w.getBlockTileEntity(8,64,8)!=old);o.writeBoolean(old.isInvalid());EntityOracle.i(o,w.loadedTileEntityList.size());Mode_wbfurnace4.stack(o,old.getStackInSlot(0));snapshot(o,w);
            }
            for(boolean remote:new boolean[]{false,true})for(float yaw:new float[]{Float.NEGATIVE_INFINITY,-Float.MAX_VALUE,-720F,-360F,-135F,-45F,-.0F,0F,44.999F,45F,90F,135F,180F,225F,270F,315F,360F,720F,Float.MAX_VALUE,Float.POSITIVE_INFINITY,Float.NaN}) {
                World w=Mode_worldblocks181.fixture(true);Player p=new Player(w);p.rotationYaw=yaw;
                Mode_worldblocks181.raw(w,8,64,8,23,15);w.multiplayerWorld=remote;Mode_wbtrapdoors6.seed(w);
                b.onBlockPlacedBy(w,8,64,8,p);snapshot(o,w);
            }
            for(boolean remote:new boolean[]{false,true})for(boolean invalid:new boolean[]{false,true})for(int meta=0;meta<16;meta++)for(int mask=0;mask<4;mask++)for(int changed:new int[]{0,1,55,76})for(int hook=0;hook<2;hook++) {
                World w=Mode_worldblocks181.fixture(true);
                Mode_worldblocks181.raw(w,8,64,8,23,meta);
                Mode_worldblocks181.raw(w,7,64,8,(mask&1)!=0?76:0,5);Mode_worldblocks181.raw(w,7,65,8,(mask&2)!=0?76:0,5);
                TileEntityDispenser d=(TileEntityDispenser)w.getBlockTileEntity(8,64,8);d.setInventorySlotContents(0,new ItemStack(4,3,2));((Random)random.get(d)).setSeed(55);if(invalid)d.invalidate();
                w.multiplayerWorld=remote;Mode_wbtrapdoors6.seed(w);
                if(hook==0)b.onNeighborBlockChange(w,8,64,8,changed);else b.updateTick(w,8,64,8,w.rand);
                snapshot(o,w);
            }
        }
    }
}
