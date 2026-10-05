package net.minecraft.src;
import java.io.*;import java.util.*;import java.lang.reflect.*;
public final class Mode_wbpush6 {
    public static void main(String[] a)throws Exception {
        int init=Block.stone.blockID;Field progress=TileEntityPiston.class.getDeclaredField("progress"),last=TileEntityPiston.class.getDeclaredField("lastProgress");progress.setAccessible(true);last.setAccessible(true);
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            for(int face=0;face<6;face++)for(boolean extend:new boolean[]{false,true})for(int stored:new int[]{1,44,85,34,36,0})for(int scene=0;scene<8;scene++) {
                World w=Mode_worldblocks181.fixture(true);Mode_wbtrapdoors6.seed(w);Mode_worldblocks181.raw(w,8,72,8,36,face);
                if((scene&1)!=0)for(int x=6;x<=10;x++)for(int z=6;z<=10;z++)Mode_worldblocks181.raw(w,x,71,z,1,0);
                if((scene&2)!=0)for(int y=71;y<=74;y++)Mode_worldblocks181.raw(w,9,y,8,1,0);
                TileEntityPiston p=new TileEntityPiston(stored,face,face,extend,false);progress.setFloat(p,(scene&4)!=0?.5F:0F);last.setFloat(p,progress.getFloat(p));w.setBlockTileEntity(8,72,8,p);
                List<EntityItem> entities=new ArrayList<EntityItem>();int dx=PistonBlockTextures.offsetsXForSide[face],dy=PistonBlockTextures.offsetsYForSide[face],dz=PistonBlockTextures.offsetsZForSide[face];
                for(int j=0;j<9;j++) {double offset=(j-4)*.25D;EntityItem e=new EntityItem(w,8.5D+dx*offset,72.5D+dy*offset,8.5D+dz*offset,new ItemStack(1,1,0));e.noClip=j==8;e.motionX=.1D;e.motionY=-.2D;e.motionZ=.3D;w.entityJoinedWorld(e);entities.add(e);}
                WorldEventOracle.snapshot(w,new DataOutputStream(new ByteArrayOutputStream()));
                for(int t=0;t<4;t++) {
                    p.updateEntity();EntityOracle.f(o,progress.getFloat(p));EntityOracle.f(o,last.getFloat(p));o.writeBoolean(p.isInvalid());o.writeByte(w.getBlockId(8,72,8));EntityOracle.i(o,w.getBlockMetadata(8,72,8));
                    for(EntityItem e:entities) {AxisAlignedBB b=e.boundingBox;for(double v:new double[]{e.posX,e.posY,e.posZ,e.motionX,e.motionY,e.motionZ,b.minX,b.minY,b.minZ,b.maxX,b.maxY,b.maxZ})EntityOracle.d(o,v);
                        for(boolean v:new boolean[]{e.onGround,e.isCollidedHorizontally,e.isCollidedVertically,e.isCollided})o.writeBoolean(v);EntityOracle.f(o,e.fallDistance);EntityOracle.f(o,e.distanceWalkedModified);
                    }
                    EntityOracle.l(o,EntityOracle.rngState(w.rand));WorldEventOracle.snapshot(w,o);
                }
            }
        }
    }
}
