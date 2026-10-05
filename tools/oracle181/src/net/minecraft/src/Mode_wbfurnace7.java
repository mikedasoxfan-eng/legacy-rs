package net.minecraft.src;
import java.io.*;
import java.lang.reflect.*;
import java.util.*;

/** Complete BlockFurnace callbacks; existing furnace4/removal5 cover ticks/drops. */
public final class Mode_wbfurnace7 {
    static World fixture(){World w=Mode_worldblocks181.fixture(true);WorldEventOracle.attach(w);return w;}
    static final class Player extends EntityPlayerSP {
        int opened;
        Player(World w)throws Exception {super((net.minecraft.client.Minecraft)Oracle.allocate(PlayerOracle.HeadlessMinecraft.class),w,new Session("Player",""),0);}
        public void displayGUIFurnace(TileEntityFurnace f){opened++;}
        public void addStat(StatBase s,int n){}
    }
    static void state(DataOutputStream o,World w,int x,int z,TileEntityFurnace old)throws Exception {
        EntityOracle.i(o,w.getBlockId(x,64,z));EntityOracle.i(o,w.getBlockMetadata(x,64,z));
        TileEntity t=w.getBlockTileEntity(x,64,z);
        o.writeBoolean(t instanceof TileEntityFurnace);o.writeBoolean(t==old);
        if(t instanceof TileEntityFurnace)for(int n=0;n<3;n++)Mode_wbfurnace4.stack(o,((TileEntityFurnace)t).getStackInSlot(n));
        if(old!=null){o.writeBoolean(old.isInvalid());for(int n=0;n<3;n++)Mode_wbfurnace4.stack(o,old.getStackInSlot(n));EntityOracle.i(o,old.field_35145_n);EntityOracle.i(o,old.field_35146_o==null?0:old.field_35146_o.blockID);}
        EntityOracle.i(o,w.loadedTileEntityList.size());
        Field added=World.class.getDeclaredField("addedTileEntityList");added.setAccessible(true);EntityOracle.i(o,((List)added.get(w)).size());
        WorldEventOracle.snapshot(w,o);
    }
    public static void main(String[] a)throws Exception {
        int init=Block.stone.blockID;
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            for(int id:new int[]{61,62})for(boolean remote:new boolean[]{false,true})for(int m=0;m<16;m++)for(int mask=0;mask<16;mask++) {
                World w=fixture();int[][] sides={{0,-1},{0,1},{-1,0},{1,0}};
                for(int k=0;k<4;k++)Mode_worldblocks181.raw(w,8+sides[k][0],64,8+sides[k][1],(mask&(1<<k))!=0?1:0,0);
                Mode_worldblocks181.raw(w,8,64,8,id,m);w.multiplayerWorld=remote;Block.blocksList[id].onBlockAdded(w,8,64,8);state(o,w,8,8,null);
            }
            for(int id:new int[]{61,62})for(boolean remote:new boolean[]{false,true})for(boolean invalid:new boolean[]{false,true})for(int hook=0;hook<2;hook++) {
                World w=fixture();Mode_worldblocks181.raw(w,8,64,8,id,15);
                TileEntityFurnace old=(TileEntityFurnace)w.getBlockTileEntity(8,64,8);old.setInventorySlotContents(0,new ItemStack(15,32,2));if(invalid)old.invalidate();w.multiplayerWorld=remote;
                if(hook==0)Block.blocksList[id].onBlockAdded(w,8,64,8);
                else {Player p=new Player(w);o.writeBoolean(Block.blocksList[id].blockActivated(w,8,64,8,p));EntityOracle.i(o,p.opened);}
                state(o,w,8,8,old);
            }
            for(int id:new int[]{61,62})for(boolean remote:new boolean[]{false,true})for(float yaw:new float[]{Float.NEGATIVE_INFINITY,-Float.MAX_VALUE,-720F,-135F,-45F,-.0F,0F,44.999F,45F,90F,135F,180F,225F,270F,315F,720F,Float.MAX_VALUE,Float.POSITIVE_INFINITY,Float.NaN}) {
                World w=fixture();Player p=new Player(w);p.rotationYaw=yaw;Mode_worldblocks181.raw(w,8,64,8,id,15);w.multiplayerWorld=remote;
                Block.blocksList[id].onBlockPlacedBy(w,8,64,8,p);state(o,w,8,8,null);
            }
            for(int id:new int[]{61,62})for(int m=0;m<16;m++)for(int x:new int[]{-30000000,-17,8,16777217,30000000}) {
                World w=fixture();Random r=new Random(567);
                EntityOracle.FlatProvider provider=(EntityOracle.FlatProvider)w.chunkProvider;
                provider.chunks.put(provider.key(x>>4,0),new Chunk(w,new byte[32768],x>>4,0));
                Mode_worldblocks181.raw(w,x,64,8,id,m);
                for(int t=0;t<8;t++){Block.blocksList[id].randomDisplayTick(w,x,64,8,r);EntityOracle.l(o,EntityOracle.rngState(r));WorldEventOracle.snapshot(w,o);}
            }
            for(int id:new int[]{61,62})for(boolean lit:new boolean[]{false,true})for(boolean scanning:new boolean[]{false,true})for(boolean remote:new boolean[]{false,true})for(int x:new int[]{-24,8,24}) {
                World w=fixture();Mode_worldblocks181.raw(w,x,64,8,id,5);TileEntityFurnace old=(TileEntityFurnace)w.getBlockTileEntity(x,64,8);old.setInventorySlotContents(0,new ItemStack(15,32,2));old.getBlockMetadata();old.getBlockType();
                Field scan=World.class.getDeclaredField("scanningTileEntities");scan.setAccessible(true);scan.setBoolean(w,scanning);w.multiplayerWorld=remote;
                BlockFurnace.updateFurnaceBlockState(lit,w,x,64,8);state(o,w,x,8,old);
            }
        }
    }
}
