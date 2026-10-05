package net.minecraft.src;
import java.io.*;
/** Calls real tile methods; the companion modes select one class at a time. */
public final class Mode_wbmachine10 {
    static int error(Throwable t) {
        if(t instanceof ClassCastException)return 1;
        if(t instanceof ArrayIndexOutOfBoundsException)return 2;
        if(t instanceof NullPointerException)return 3;
        if(t instanceof RuntimeException)return 4;
        throw new AssertionError(t);
    }
    static NBTBase tag(int ty) {
        switch(ty) {
            case 1:return new NBTTagByte((byte)-1);case 2:return new NBTTagShort((short)-1);
            case 3:return new NBTTagInt(-1);case 4:return new NBTTagLong(-1);
            case 5:return new NBTTagFloat(-1);case 6:return new NBTTagDouble(-1);
            case 7:return new NBTTagByteArray(new byte[]{(byte)255});case 8:return new NBTTagString("bad");
            case 9:NBTTagList l=new NBTTagList();l.setTag(new NBTTagByte((byte)-1));return l;
            case 10:return new NBTTagCompound();default:throw new AssertionError();
        }
    }
    static NBTTagCompound input(int id,int slot) {
        NBTTagCompound n=new NBTTagCompound();n.setInteger("x",Integer.MIN_VALUE);n.setInteger("y",Integer.MAX_VALUE);n.setInteger("z",-12345);
        n.setShort("BurnTime",(short)-32768);n.setShort("CookTime",(short)32767);
        NBTTagCompound s=new NBTTagCompound();s.setByte("Slot",(byte)slot);s.setShort("id",(short)id);s.setByte("Count",(byte)id);s.setShort("Damage",(short)(id*17));
        NBTTagList l=new NBTTagList();l.setTag(s);n.setTag("Items",l);return n;
    }
    static void one(DataOutputStream o,int block,NBTTagCompound n)throws Exception {
        TileEntity t=block==61?new TileEntityFurnace():new TileEntityDispenser();IInventory inv=(IInventory)t;
        for(int k=0;k<inv.getSizeInventory();k++)inv.setInventorySlotContents(k,new ItemStack(263,12,7));
        t.xCoord=7;t.yCoord=8;t.zCoord=9;t.field_35145_n=11;t.field_35146_o=Block.stone;t.invalidate();
        if(block==61){TileEntityFurnace f=(TileEntityFurnace)t;f.furnaceBurnTime=99;f.furnaceCookTime=-99;f.currentItemBurnTime=123;}
        int e=0;try{t.readFromNBT(n);}catch(Throwable ex){e=error(ex);}o.writeByte(e);
        inv.func_35142_x_();inv.func_35141_y_();
        if(block==61){TileEntityFurnace f=(TileEntityFurnace)t;EntityOracle.i(o,f.furnaceBurnTime);EntityOracle.i(o,f.currentItemBurnTime);EntityOracle.i(o,f.furnaceCookTime);}
        EntityOracle.i(o,t.field_35145_n);EntityOracle.i(o,t.field_35146_o.blockID);o.writeBoolean(t.isInvalid());Mode_wbinventory7.nbt(o,t);
    }
    static void run(String[] a,int block)throws Exception {
        int init=Block.stone.blockID;
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            for(int id=-32768;id<=32767;id++)one(o,block,input(id,0));
            for(int slot=-128;slot<=127;slot++)for(int id:new int[]{-32768,-1,0,15,263,500,31999,32000,32767})one(o,block,input(id,slot));
            for(String key:new String[]{"x","y","z","Items","BurnTime","CookTime"})for(int ty=1;ty<=10;ty++){NBTTagCompound n=input(263,1);n.setTag(key,tag(ty));one(o,block,n);}
            for(String key:new String[]{"Slot","id","Count","Damage"})for(int ty=1;ty<=10;ty++){NBTTagCompound n=input(-1,1);((NBTTagCompound)n.getTagList("Items").tagAt(0)).setTag(key,tag(ty));one(o,block,n);}
            for(int ty=1;ty<=10;ty++){NBTTagCompound n=input(15,0);n.getTagList("Items").setTag(tag(ty));one(o,block,n);}
            for(int id:new int[]{263,500,0,-1,32000}){NBTTagCompound n=input(15,0);n.getTagList("Items").setTag(input(id,0).getTagList("Items").tagAt(0));one(o,block,n);}
            one(o,block,new NBTTagCompound());
        }
    }
}
