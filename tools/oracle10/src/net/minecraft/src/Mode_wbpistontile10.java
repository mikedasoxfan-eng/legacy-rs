package net.minecraft.src;
import java.io.*;
import java.lang.reflect.*;
public final class Mode_wbpistontile10 {
    static Field progress,last;
    static void state(DataOutputStream o,TileEntityPiston p)throws Exception {
        for(int v:new int[]{Trace10.namedInt("p.xCoord", p.xCoord),Trace10.namedInt("p.yCoord", p.yCoord),Trace10.namedInt("p.zCoord", p.zCoord),Trace10.namedInt("p.getStoredBlockID()", p.getStoredBlockID()),Trace10.namedInt("p.getBlockMetadata()", p.getBlockMetadata()),Trace10.namedInt("p.func_31009_d()", p.func_31009_d())})EntityOracle.i(o,v);
        Trace10.writeBoolean(o, "p.func_31015_b()", p.func_31015_b());Trace10.writeBoolean(o, "p.func_31012_k()", p.func_31012_k());EntityOracle.f(o,Trace10.namedFloat("progress.getFloat(p)", progress.getFloat(p)));EntityOracle.f(o,Trace10.namedFloat("last.getFloat(p)", last.getFloat(p)));Trace10.writeBoolean(o, "p.isInvalid()", p.isInvalid());Mode_wbinventory7.nbt(o,p);
        for(float partial:new float[]{Float.NEGATIVE_INFINITY,-1F,-0F,0F,.5F,1F,2F,Float.POSITIVE_INFINITY,Float.NaN}) {
            EntityOracle.f(o,Trace10.namedFloat("p.func_31008_a(partial)", p.func_31008_a(partial)));float[] v=null;int e=0;try{v=new float[]{p.func_31017_b(partial),p.func_31014_c(partial),p.func_31013_d(partial)};}catch(Throwable ex){e=Mode_wbmachine10.error(ex);}Trace10.writeByte(o, "e", e);if(e==0)for(float value:v)EntityOracle.f(o,value);
        }
    }
    public static void main(String[] a)throws Exception {
        int init=Block.stone.blockID;progress=TileEntityPiston.class.getDeclaredField("progress");last=TileEntityPiston.class.getDeclaredField("lastProgress");progress.setAccessible(true);last.setAccessible(true);
        Field scanning=World.class.getDeclaredField("scanningTileEntities");scanning.setAccessible(true);
        int[] ids={Integer.MIN_VALUE,-257,-1,0,1,8,36,110,255,256,257,65537,Integer.MAX_VALUE};
        int[] faces={Integer.MIN_VALUE,-1,0,1,2,3,4,5,6,Integer.MAX_VALUE};
        int[] bits={0,0x80000000,0x3f000000,0x3f800000,0x3fc00000,0xbf800000,0x7f800000,0xff800000,0x7fc01234};
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            for(int id:ids)for(int face:faces)for(boolean extend:new boolean[]{false,true})for(boolean head:new boolean[]{false,true})for(int bit:bits) {
                TileEntityPiston p=new TileEntityPiston(id,-12345,face,extend,head);p.xCoord=-123;p.yCoord=64;p.zCoord=456;p.invalidate();progress.setFloat(p,Float.intBitsToFloat(bit));last.setFloat(p,.25F);state(o,p);
                NBTTagCompound n=new NBTTagCompound();p.writeToNBT(n);p=new TileEntityPiston(99,98,5,!extend,head);p.xCoord=7;p.yCoord=8;p.zCoord=9;p.readFromNBT(n);Trace10.writeByte(o, "0", 0);state(o,p);
            }
            for(String key:new String[]{"x","y","z","blockId","blockData","facing","progress","extending"})for(int ty=1;ty<=10;ty++) {
                TileEntityPiston source=new TileEntityPiston(-257,Integer.MIN_VALUE,Integer.MAX_VALUE,true,false);source.xCoord=-123;source.yCoord=64;source.zCoord=456;NBTTagCompound n=new NBTTagCompound();source.writeToNBT(n);n.setTag(key,Mode_wbmachine10.tag(ty));
                TileEntityPiston p=new TileEntityPiston(99,98,5,false,true);p.xCoord=7;p.yCoord=8;p.zCoord=9;p.invalidate();int e=0;try{p.readFromNBT(n);}catch(Throwable ex){e=Mode_wbmachine10.error(ex);}Trace10.writeByte(o, "e", e);state(o,p);
            }
            for(boolean remote:new boolean[]{false,true})for(int id:ids)for(int face:new int[]{-1,0,5,6})for(boolean extend:new boolean[]{false,true})for(int bit:new int[]{0,0x3f000000,0x3f800000,0x7fc01234})for(int config=0;config<3;config++)for(int hook=0;hook<2;hook++) {
                World w=Mode_worldblocks10.fixture(true);Mode_worldblocks10.raw(w,8,64,8,36,face);
                TileEntityPiston p=new TileEntityPiston(id,-12345,face,extend,true);progress.setFloat(p,Float.intBitsToFloat(bit));last.setFloat(p,Float.intBitsToFloat(bit));w.setBlockTileEntity(8,64,8,p);
                if(config==1)Mode_worldblocks10.raw(w,8,64,8,1,7);
                if(config==2){w.setBlockTileEntity(8,64,8,new TileEntityPiston(1,0,0,false,false));scanning.setBoolean(w,true);}
                w.multiplayerWorld=remote;Mode_wbtrapdoors6.seed(w);w.getChunkFromChunkCoords(0,0).isModified=false;
                int e=0;try{if(hook==0)p.updateEntity();else p.clearPistonTileEntity();}catch(Throwable ex){e=Mode_wbmachine10.error(ex);}Trace10.writeByte(o, "e", e);state(o,p);
                Trace10.writeBoolean(o, "w.loadedTileEntityList.contains(p)", w.loadedTileEntityList.contains(p));Trace10.writeBoolean(o, "w.getChunkFromChunkCoords(0,0).isModified", w.getChunkFromChunkCoords(0,0).isModified);Mode_wbtrapdoors6.result(o,w);
            }
        }
    }
}
