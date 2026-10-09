package net.minecraft.src;
import java.io.*;
import java.util.Random;
public final class Mode_wbtileentity10 {
    static void snapshot(DataOutputStream o,TileEntity t,World w)throws Exception {
        for(int v:new int[]{Trace10.namedInt("t.xCoord", t.xCoord),Trace10.namedInt("t.yCoord", t.yCoord),Trace10.namedInt("t.zCoord", t.zCoord),Trace10.namedInt("t.blockMetadata", t.blockMetadata),Trace10.namedInt("t.field_35146_o==null?0:t.field_35146_o.blockID", t.blockType==null?0:t.blockType.blockID)})EntityOracle.i(o,v);
        Trace10.writeBoolean(o, "t.isInvalid()", t.isInvalid());Trace10.writeBoolean(o, "w.getChunkFromChunkCoords(0,0).isModified", w.getChunkFromChunkCoords(0,0).isModified);
    }
    static void nbt(DataOutputStream o,NBTTagCompound n)throws Exception {
        ByteArrayOutputStream b=new ByteArrayOutputStream();CompressedStreamTools.writeTo(n,new DataOutputStream(b));EntityOracle.i(o,Trace10.namedInt("b.size()", b.size()));Trace10.writeBytes(o, "b.toByteArray()", b.toByteArray());
    }
    public static void main(String[] a)throws Exception {
        int init=Block.stone.blockID;
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            World w=Mode_worldblocks10.fixture(true);TileEntity t=new TileEntity();t.xCoord=8;t.yCoord=64;t.zCoord=8;Random rng=new Random(1729);
            for(int step=0;step<20000;step++) {
                int op=rng.nextInt(10),block=new int[]{0,1,23,61,84,110,255}[rng.nextInt(7)],meta=rng.nextInt(16);
                Mode_worldblocks10.raw(w,8,64,8,block,meta);w.getChunkFromChunkCoords(0,0).isModified=false;t.worldObj=step%3!=0?w:null;
                if(op==0||op==1){int e=0,v=0;try{if(op==0)v=t.getBlockMetadata();else{Block b=t.getBlockType();v=b==null?0:b.blockID;}}catch(Throwable ex){e=Mode_wbmachine10.error(ex);}Trace10.writeByte(o, "e", e);if(e==0)EntityOracle.i(o,v);}
                if(op==2)t.onInventoryChanged();if(op==3)t.invalidate();if(op==4)t.validate();if(op==5)t.updateContainingBlockInfo();if(op==6)t.onTileEntityPowered(Integer.MIN_VALUE,Integer.MAX_VALUE);if(op==7)t.updateEntity();
                if(op==8){NBTTagCompound n=new NBTTagCompound();n.setByte("keep",(byte)7);int e=0;try{if(t.worldObj!=null){TileEntityNote note=new TileEntityNote();note.xCoord=t.xCoord;note.yCoord=t.yCoord;note.zCoord=t.zCoord; // call just the superclass writer with its mapped concrete class
                    java.lang.invoke.MethodHandle h=Mode_wbcontainer7.lookup().findSpecial(TileEntity.class,"writeToNBT",java.lang.invoke.MethodType.methodType(void.class,NBTTagCompound.class),TileEntityNote.class);
                    h.invoke(note,n);
                }else t.writeToNBT(n);}catch(Throwable ex){e=Mode_wbmachine10.error(ex);}Trace10.writeByte(o, "e", e);nbt(o,n);}
                if(op==9){NBTTagCompound n=new NBTTagCompound();n.setInteger("x",8);n.setInteger("y",64);n.setInteger("z",8);t.readFromNBT(n);Trace10.writeByte(o, "0", 0);}
                snapshot(o,t,w);
            }
            for(String key:new String[]{"x","y","z"})for(int ty=1;ty<=10;ty++) {
                t=new TileEntity();t.xCoord=7;t.yCoord=8;t.zCoord=9;t.blockMetadata=11;t.blockType=Block.stone;t.invalidate();NBTTagCompound n=Mode_wbmachine10.input(15,0);n.setTag(key,Mode_wbmachine10.tag(ty));int e=0;try{t.readFromNBT(n);}catch(Throwable ex){e=Mode_wbmachine10.error(ex);}Trace10.writeByte(o, "e", e);snapshot(o,t,w);
            }
            for(int[] pos:new int[][]{{0,0,0},{Integer.MIN_VALUE,Integer.MAX_VALUE,-12345},{-1,64,-1}}) {
                t.xCoord=pos[0];t.yCoord=pos[1];t.zCoord=pos[2];
                for(double v:new double[]{Double.NEGATIVE_INFINITY,-Double.MAX_VALUE,-123.5,-0.0,0,0.5,123.5,Double.MAX_VALUE,Double.POSITIVE_INFINITY,Double.NaN})EntityOracle.l(o,Trace10.namedLong("Double.doubleToLongBits(t.getDistanceFrom(v,v,v))", Double.doubleToLongBits(t.getDistanceFrom(v,v,v))));
            }
            for(String name:new String[]{"Furnace","Chest","RecordPlayer","Trap","Sign","MobSpawner","Music","Piston","","Unknown","furnace","Furnace\0"})for(int malformed=0;malformed<5;malformed++) {
                NBTTagCompound n=new NBTTagCompound();n.setString("id",name);n.setInteger("x",-123);n.setInteger("y",64);n.setInteger("z",456);
                if(malformed==1)n.setByte("x",(byte)1);if(malformed==2)n.setInteger("id",1);if(malformed==3)n.setInteger("Items",1);if(malformed==4)n.setByte("BurnTime",(byte)1);
                t=null;int e=0;try{t=TileEntity.createAndLoadEntity(n);}catch(Throwable ex){e=Mode_wbmachine10.error(ex);}Trace10.writeByte(o, "e", e);Trace10.writeBoolean(o, "t!=null", t!=null);if(t!=null){snapshot(o,t,w);Mode_wbinventory7.nbt(o,t);}
            }
        }
    }
}
