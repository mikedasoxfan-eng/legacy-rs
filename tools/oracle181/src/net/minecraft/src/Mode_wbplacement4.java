package net.minecraft.src;
import java.io.*;
import java.lang.reflect.*;
import java.util.*;
/** Calls original Item.onItemUse; compares its result, stack, blocks, RNG and callbacks. */
public final class Mode_wbplacement4 {
    static final class Player extends EntityPlayerSP {
        Player(net.minecraft.client.Minecraft mc,World w) {super(mc,w,new Session("Player",""),0);}
        public void displayGUIEditSign(TileEntitySign sign) {}
        public void addStat(StatBase stat,int count) {}
    }
    static void cells(DataOutputStream o,World w,int lo,int hi,int bottom,int top)throws Exception {
        for(int x=lo;x<=hi;x++)for(int y=bottom;y<=top;y++)for(int z=lo;z<=hi;z++) {
            o.writeByte(w.getBlockId(x,y,z));o.writeByte(w.getBlockMetadata(x,y,z));
        }
    }
    public static void main(String[] a)throws Exception {
        int init=Block.stone.blockID;Field f=Item.class.getDeclaredField("itemRand");f.setAccessible(true);Random itemRand=(Random)f.get(null);
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            for(int id:new int[]{1,6,17,18,35,44,98,106,107,108,109,338,354,356,331,259,290,291,292,293,294,324,330,355,323,295,361,362,351})for(int side=0;side<6;side++)for(int config=0;config<12;config++) {
                World w=Mode_worldblocks181.fixture(true);EntityPlayer p=new Player((net.minecraft.client.Minecraft)Oracle.allocate(PlayerOracle.HeadlessMinecraft.class),w);
                w.rand.setSeed(42);itemRand.setSeed(123);OracleHooks.reset(100);EntityOracle.mathRandom().setSeed(456);
                p.rotationYaw=-450.0F+90.0F*config;
                Mode_worldblocks181.raw(w,8,64,8,new int[]{0,1,2,3,60,78,106,31,8,9,51,1}[config],config&15);
                if(config>=10)for(int[] v:new int[][]{{-1,0},{1,0},{0,-1},{0,1}})Mode_worldblocks181.raw(w,8+v[0],64,8+v[1],1,0);
                ItemStack s=new ItemStack(id,new int[]{0,1,4,-1}[config%4],id==351?15:config&3);
                WorldEventOracle.attach(w);
                o.writeBoolean(s.getItem().onItemUse(s,p,w,8,64,8,side));EntityOracle.i(o,s.itemID);EntityOracle.i(o,s.stackSize);EntityOracle.i(o,s.getItemDamage());
                cells(o,w,6,10,63,67);EntityOracle.l(o,EntityOracle.rngState(w.rand));EntityOracle.l(o,EntityOracle.rngState(itemRand));WorldEventOracle.snapshot(w,o);
            }
        }
    }
}
