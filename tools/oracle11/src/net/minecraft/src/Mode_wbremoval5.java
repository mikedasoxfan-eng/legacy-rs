package net.minecraft.src;
import java.io.*;import java.lang.reflect.*;import java.util.*;
public final class Mode_wbremoval5 {
    public static void main(String[] a)throws Exception {
        int init=Block.stone.blockID;
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            for(int block:new int[]{54,23,61,62})for(int config=0;config<8;config++) {
                World w=Mode_worldblocks10.fixture(true);w.setBlock(8,64,8,block);TileEntity tile=w.getBlockTileEntity(8,64,8);IInventory inv=(IInventory)tile;
                int size=inv.getSizeInventory();
                for(int slot=0;slot<size;slot++)if(slot%3==0) {
                    ItemStack stack=new ItemStack(slot%2==0?4:264,new int[]{0,1,9,10,21,64,-1,127}[config],slot+2);
                    inv.setInventorySlotContents(slot,stack);stack.stackSize=new int[]{0,1,9,10,21,64,-1,127}[config];
                }
                if(block==54)((TileEntityChest)tile).setInventorySlotContents(35,new ItemStack(1,99,3));
                if(config==7)tile.invalidate();
                Field random=Block.blocksList[block].getClass().getDeclaredField(block==54||block==23?"random":"furnaceRand");random.setAccessible(true);Random rng=(Random)random.get(Block.blocksList[block]);rng.setSeed(500);
                w.rand.setSeed(42);EntityOracle.mathRandom().setSeed(200);OracleHooks.reset(100);WorldEventOracle.attach(w);
                w.setBlockWithNotify(8,64,8,0);
                Trace10.writeBoolean(o, "tile.isInvalid()", tile.isInvalid());for(int slot=0;slot<(block==54?36:size);slot++)Mode_wbfurnace4.stack(o,inv.getStackInSlot(slot));
                EntityOracle.l(o,Trace10.namedLong("EntityOracle.rngState(rng)", EntityOracle.rngState(rng)));EntityOracle.l(o,Trace10.namedLong("EntityOracle.rngState(w.rand)", EntityOracle.rngState(w.rand)));EntityOracle.i(o,Trace10.namedInt("w.loadedEntityList.size()", w.loadedEntityList.size()));
                for(Object object:w.loadedEntityList) {EntityItem e=(EntityItem)object;Mode_wbfurnace4.stack(o,e.item);for(double v:new double[]{Trace10.namedDouble("e.posX", e.posX),Trace10.namedDouble("e.posY", e.posY),Trace10.namedDouble("e.posZ", e.posZ),Trace10.namedDouble("e.motionX", e.motionX),Trace10.namedDouble("e.motionY", e.motionY),Trace10.namedDouble("e.motionZ", e.motionZ)})EntityOracle.d(o,v);}
                WorldEventOracle.snapshot(w,o);
            }
        }
    }
}
