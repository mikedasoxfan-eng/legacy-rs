package net.minecraft.src;
import java.io.*;import java.lang.reflect.*;
public final class Mode_wbpiston5 {
    static Field progress;
    static void f(DataOutputStream o,float value)throws Exception{EntityOracle.i(o,Float.floatToIntBits(value));}
    static void tile(DataOutputStream o,TileEntityPiston p)throws Exception {
        EntityOracle.i(o,p.getStoredBlockID());EntityOracle.i(o,p.getBlockMetadata());EntityOracle.i(o,p.func_31009_d());
        o.writeBoolean(p.func_31015_b());o.writeBoolean(p.func_31012_k());f(o,progress.getFloat(p));f(o,p.func_31008_a(0));o.writeBoolean(p.isInvalid());
    }
    static void cells(DataOutputStream o,World w,int x,int y,int z,int face)throws Exception {
        for(int j=-1;j<=15;j++) {
            int px=x+PistonBlockTextures.offsetsXForSide[face]*j,py=y+PistonBlockTextures.offsetsYForSide[face]*j,pz=z+PistonBlockTextures.offsetsZForSide[face]*j;
            o.writeByte(w.getBlockId(px,py,pz));o.writeByte(w.getBlockMetadata(px,py,pz));
            TileEntity t=py<0||py>=128?null:w.getBlockTileEntity(px,py,pz);o.writeBoolean(t instanceof TileEntityPiston);if(t instanceof TileEntityPiston)tile(o,(TileEntityPiston)t);
        }
        EntityOracle.l(o,EntityOracle.rngState(w.rand));WorldEventOracle.snapshot(w,o);
    }
    public static void main(String[] a)throws Exception {
        int init=Block.stone.blockID;progress=TileEntityPiston.class.getDeclaredField("progress");progress.setAccessible(true);
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            for(int[] values:new int[][]{PistonBlockTextures.field_31057_a,PistonBlockTextures.offsetsXForSide,PistonBlockTextures.offsetsYForSide,PistonBlockTextures.offsetsZForSide})for(int value:values)EntityOracle.i(o,value);
            // Actual block extension/retraction, push limit, immovable blocks and new materials.
            for(int id:new int[]{29,33})for(int face=0;face<6;face++)for(int config=0;config<24;config++) {
                EntityOracle.i(o,id);EntityOracle.i(o,face);EntityOracle.i(o,config);World w=Mode_worldblocks181.fixture(true);w.rand.setSeed(42);OracleHooks.reset(100);EntityOracle.mathRandom().setSeed(200);
                int x=8,y=config==22?1:config==23?126:72,z=8;
                Mode_worldblocks181.raw(w,x,y,z,id,face);
                int lx=face==4?x:x-1,lz=face==4?z-1:z;
                Mode_worldblocks181.raw(w,lx,y-1,lz,1,0);Mode_worldblocks181.raw(w,lx,y,lz,69,13);
                int count=config<14?config:1,carried=config<14?1:new int[]{49,7,54,23,98,103,107,106}[Math.min(config-14,7)];
                for(int j=1;j<=count;j++)Mode_worldblocks181.raw(w,x+PistonBlockTextures.offsetsXForSide[face]*j,y+PistonBlockTextures.offsetsYForSide[face]*j,z+PistonBlockTextures.offsetsZForSide[face]*j,carried,0);
                WorldEventOracle.attach(w);Block.blocksList[id].onNeighborBlockChange(w,x,y,z,69);cells(o,w,x,y,z,face);
                for(int t=0;t<4;t++){w.updateEntities();cells(o,w,x,y,z,face);}
                Mode_worldblocks181.raw(w,lx,y,lz,0,0);Block.blocksList[id].onNeighborBlockChange(w,x,y,z,69);cells(o,w,x,y,z,face);
                for(int t=0;t<4;t++){w.updateEntities();cells(o,w,x,y,z,face);}
            }
            // Interpolation, signed metadata, NBT lastProgress and nonfinite progress.
            for(int face=0;face<6;face++)for(boolean extend:new boolean[]{false,true})for(int bits:new int[]{0,0x80000000,0x3f000000,0x3f800000,0x3fc00000,0xbf800000,0x7f800000,0xff800000,0x7fc01234}) {
                World w=Mode_worldblocks181.fixture(true);Mode_worldblocks181.raw(w,8,72,8,36,face);
                NBTTagCompound n=new NBTTagCompound();n.setInteger("blockId",1);n.setInteger("blockData",-123);n.setInteger("facing",face);n.setFloat("progress",Float.intBitsToFloat(bits));n.setBoolean("extending",extend);
                TileEntityPiston p=new TileEntityPiston();p.readFromNBT(n);w.setBlockTileEntity(8,72,8,p);
                for(int t=0;t<4;t++) {
                    tile(o,p);
                    for(float partial:new float[]{-1,0,0.5F,1,2,Float.NaN}) {f(o,p.func_31008_a(partial));f(o,p.func_31017_b(partial));f(o,p.func_31014_c(partial));f(o,p.func_31013_d(partial));}
                    NBTTagCompound saved=new NBTTagCompound();p.writeToNBT(saved);f(o,saved.getFloat("progress"));
                    Mode_worldblocks181.box(o,Block.pistonMoving.getCollisionBoundingBoxFromPool(w,8,72,8));if(!Float.isInfinite(Float.intBitsToFloat(bits)))p.updateEntity();
                }
            }
        }
    }
}
