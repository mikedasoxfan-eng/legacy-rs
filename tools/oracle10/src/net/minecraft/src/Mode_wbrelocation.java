package net.minecraft.src;
import java.io.*;import java.util.*;
public final class Mode_wbrelocation {
    public static void main(String[] a)throws Exception {
        int init=Block.stone.blockID;File dir=new File("wb10-relocation-java");dir.mkdirs();File region=new File(dir,"region/r.0.0.mcr");RegionFileCache.clearRegionFileReferences();region.delete();World w=Mode_worldblocks10.fixture(true);
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            for(int k=0;k<10;k++) {
                NBTTagCompound root=new NBTTagCompound(),n=new NBTTagCompound();root.setCompoundTag("Level",n);n.setInteger("xPos",-5-k);n.setInteger("zPos",7+k);
                byte[] blocks=new byte[32768];for(int j=0;j<blocks.length;j++)blocks[j]=(byte)((j+k*13)&255);n.setByteArray("Blocks",blocks);n.setByteArray("Data",new byte[16384]);n.setByteArray("SkyLight",new byte[16384]);n.setByteArray("BlockLight",new byte[16384]);n.setByteArray("HeightMap",new byte[256]);n.setTag("Entities",new NBTTagList());n.setTag("TileEntities",new NBTTagList());
                if(k==8)root=new NBTTagCompound();if(k==9){n=new NBTTagCompound();root.setCompoundTag("Level",n);}
                DataOutputStream data=RegionFileCache.getChunkOutputStream(dir,k,0);CompressedStreamTools.writeTo(root,data);data.close();Chunk c=new ThreadedChunkLoader(dir).loadChunk(w,k,0);Trace10.writeBoolean(o, "c!=null", c!=null);
                if(c!=null){EntityOracle.i(o,Trace10.namedInt("c.xPosition", c.xPosition));EntityOracle.i(o,Trace10.namedInt("c.zPosition", c.zPosition));Trace10.writeBytes(o, "c.blocks", c.blocks);Trace10.writeBytes(o, "c.data.data", c.data.data);Trace10.writeBytes(o, "c.skylightMap.data", c.skylightMap.data);Trace10.writeBytes(o, "c.blocklightMap.data", c.blocklightMap.data);Trace10.writeBytes(o, "c.heightMap", c.heightMap);}
            }
        }RegionFileCache.clearRegionFileReferences();
    }
}
