package net.minecraft.src;
import java.io.*;
/** ContainerChest's open/close callbacks through single and directional double chests. */
public final class Mode_chesthooks10 {
    static final class Player extends EntityPlayer {
        Container active;
        Player(World w){super(w);}
        public void func_6420_o(){}
        public void displayGUIChest(IInventory inv){active=new ContainerChest(inventory,inv);}
    }
    public static void main(String[] args)throws Exception {
        try(DataOutputStream o=new DataOutputStream(new FileOutputStream(args[1]))) {
            for(int layout=0;layout<5;layout++) {
                World w=Mode_entities10.world();Player p=new Player(w);p.setLocationAndAngles(8.5,64,8.5,0,0);
                int[][] pos=layout==0?new int[][]{{8,64,8}}:new int[][]{{8,64,8},{8+new int[]{0,-1,1,0,0}[layout],64,8+new int[]{0,0,0,-1,1}[layout]}};
                for(int[] xyz:pos) {EntityOracle.rawBlock(w,xyz[0],xyz[1],xyz[2],54,3);w.setBlockTileEntity(xyz[0],xyz[1],xyz[2],new TileEntityChest());}
                WorldEventOracle.snapshot(w,new DataOutputStream(new ByteArrayOutputStream()));
                for(int t=0;t<240;t++) {
                    int phase=t%24;
                    if(phase==0||phase==10||phase==20) {if(p.active!=null)p.active.onCraftGuiClosed(p);p.active=null;Block.chest.blockActivated(w,8,64,8,p);}
                    if(phase==7||phase==14) {if(p.active!=null)p.active.onCraftGuiClosed(p);p.active=null;}
                    if(phase==23) {if(p.active!=null)p.active.onCraftGuiClosed(p);p.active=new ContainerPlayer(p.inventory,!w.multiplayerWorld);}
                    for(int[] xyz:pos) {
                        TileEntityChest chest=(TileEntityChest)w.getBlockTileEntity(xyz[0],xyz[1],xyz[2]);chest.updateEntity();
                        EntityOracle.i(o,Trace10.namedInt("chest.field_35156_h", chest.numUsingPlayers));EntityOracle.f(o,Trace10.namedFloat("chest.field_35148_f", chest.lidAngle));EntityOracle.f(o,Trace10.namedFloat("chest.field_35149_g", chest.prevLidAngle));
                    }
                    EntityOracle.i(o,Trace10.namedInt("p.active==null?0:p.active.inventorySlots.size()", p.active==null?0:p.active.inventorySlots.size()));
                    if(p.active!=null)for(Object object:p.active.inventorySlots){Slot slot=(Slot)object;EntityOracle.i(o,Trace10.namedInt("slot.xDisplayPosition", slot.xDisplayPosition));EntityOracle.i(o,Trace10.namedInt("slot.yDisplayPosition", slot.yDisplayPosition));EntityOracle.i(o,Trace10.namedInt("slot.getSlotStackLimit()", slot.getSlotStackLimit()));}
                    EntityOracle.l(o,Trace10.namedLong("EntityOracle.rngState(w.rand)", EntityOracle.rngState(w.rand)));WorldEventOracle.snapshot(w,o);
                }
            }
        }
    }
}
