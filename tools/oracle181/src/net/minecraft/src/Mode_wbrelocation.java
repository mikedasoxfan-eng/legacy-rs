package net.minecraft.src;
import java.io.*;import java.util.*;
public final class Mode_wbrelocation {
    public static void main(String[] a)throws Exception {
        int init=Block.stone.blockID;File dir=new File("wb181-relocation-java");dir.mkdirs();File region=new File(dir,"region/r.0.0.mcr");RegionFileCache.clearRegionFileReferences();region.delete();World w=Mode_worldblocks181.fixture(true);
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            for(int k=0;k<10;k++) {
                NBTTagCompound root=new NBTTagCompound(),n=new NBTTagCompound();root.setCompoundTag("Level",n);n.setInteger("xPos",-5-k);n.setInteger("zPos",7+k);
                byte[] blocks=new byte[32768];for(int j=0;j<blocks.length;j++)blocks[j]=(byte)((j+k*13)&255);n.setByteArray("Blocks",blocks);n.setByteArray("Data",new byte[16384]);n.setByteArray("SkyLight",new byte[16384]);n.setByteArray("BlockLight",new byte[16384]);n.setByteArray("HeightMap",new byte[256]);n.setTag("Entities",new NBTTagList());n.setTag("TileEntities",new NBTTagList());
                if(k==8)root=new NBTTagCompound();if(k==9){n=new NBTTagCompound();root.setCompoundTag("Level",n);}
                DataOutputStream data=RegionFileCache.getChunkOutputStream(dir,k,0);CompressedStreamTools.func_1139_a(root,data);data.close();Chunk c=new McRegionChunkLoader(dir).loadChunk(w,k,0);o.writeBoolean(c!=null);
                if(c!=null){EntityOracle.i(o,c.xPosition);EntityOracle.i(o,c.zPosition);o.write(c.blocks);o.write(c.data.data);o.write(c.skylightMap.data);o.write(c.blocklightMap.data);o.write(c.heightMap);}
            }
        }RegionFileCache.clearRegionFileReferences();
    }
}

