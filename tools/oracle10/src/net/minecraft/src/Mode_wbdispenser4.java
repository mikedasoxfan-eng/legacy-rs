package net.minecraft.src;
import java.io.*;
import java.lang.reflect.*;
import java.util.*;
public final class Mode_wbdispenser4 {
    public static void main(String[] a)throws Exception {
        int init=Block.stone.blockID;Field random=TileEntityDispenser.class.getDeclaredField("dispenserRandom");random.setAccessible(true);
        Method dispense=BlockDispenser.class.getDeclaredMethod("dispenseItem",World.class,int.class,int.class,int.class,Random.class);dispense.setAccessible(true);
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            for(int facing=0;facing<6;facing++)for(int config=0;config<16;config++) {
                World w=Mode_worldblocks10.fixture(true);w.setBlock(8,64,8,23);w.setBlockMetadata(8,64,8,facing);w.rand.setSeed(42);EntityOracle.mathRandom().setSeed(123);OracleHooks.reset(100);
                TileEntityDispenser d=new TileEntityDispenser();for(int n=0;n<9;n++)if((config&(1<<(n%4)))!=0)d.setInventorySlotContents(n,new ItemStack(new int[]{262,344,332,4}[n%4],2+n%3,0));w.setBlockTileEntity(8,64,8,d);((Random)random.get(d)).setSeed(55);WorldEventOracle.attach(w);
                for(int t=0;t<8;t++) {
                    dispense.invoke(Block.dispenser,w,8,64,8,w.rand);for(int n=0;n<9;n++)Mode_wbfurnace4.stack(o,d.getStackInSlot(n));EntityOracle.l(o,Trace10.namedLong("EntityOracle.rngState(w.rand)", EntityOracle.rngState(w.rand)));EntityOracle.l(o,Trace10.namedLong("EntityOracle.rngState((Random)random.get(d))", EntityOracle.rngState((Random)random.get(d))));EntityOracle.i(o,Trace10.namedInt("w.loadedEntityList.size()", w.loadedEntityList.size()));
                    for(Object object:w.loadedEntityList) {Entity e=(Entity)object;EntityOracle.i(o,Trace10.namedInt("e instanceof EntityArrow?0:e instanceof EntityEgg?1:e instanceof EntitySnowball?2:3", e instanceof EntityArrow?0:e instanceof EntityEgg?1:e instanceof EntitySnowball?2:3));for(double v:new double[]{Trace10.namedDouble("e.posX", e.posX),Trace10.namedDouble("e.posY", e.posY),Trace10.namedDouble("e.posZ", e.posZ),Trace10.namedDouble("e.motionX", e.motionX),Trace10.namedDouble("e.motionY", e.motionY),Trace10.namedDouble("e.motionZ", e.motionZ)})EntityOracle.d(o,v);}
                    WorldEventOracle.snapshot(w,o);
                }
            }
        }
    }
}
