package net.minecraft.src;
import java.io.*;
public final class Mode_wbarrays {
    static final String[] keys={"Blocks","Data","SkyLight","BlockLight","HeightMap"};
    static byte[] data(int len,int salt){byte[] a=new byte[len];for(int k=0;k<len;k++)a[k]=(byte)(k*31+salt);return a;}
    static void snapshot(DataOutputStream o,Chunk c)throws Exception {
        EntityOracle.i(o,c.xPosition);EntityOracle.i(o,c.zPosition);
        for(byte[] b:new byte[][]{c.blocks,c.data.data,c.skylightMap.data,c.blocklightMap.data,c.heightMap}){EntityOracle.i(o,b.length);o.write(b);}
        EntityOracle.i(o,c.lowestBlockHeight);o.writeBoolean(c.isModified);o.writeBoolean(c.isTerrainPopulated);o.writeBoolean(c.hasEntities);
        for(int p:c.field_35845_c)EntityOracle.i(o,p);for(boolean b:c.field_35844_d)o.writeBoolean(b);
    }
    public static void main(String[] a)throws Exception {
        int init=Block.stone.blockID;World w=Mode_worldblocks181.fixture(true);
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            for(int len:new int[]{0,1,127,256,32768})snapshot(o,new Chunk(w,data(len,3),-7,11));
            for(int field=0;field<5;field++)for(int len:new int[]{-1,0,1,127,256,16384,32768,65536}) {
                NBTTagCompound n=new NBTTagCompound();n.setInteger("xPos",-7);n.setInteger("zPos",11);n.setBoolean("TerrainPopulated",(len&1)!=0);
                for(int k=0;k<5;k++)if(k!=field || len>=0)n.setByteArray(keys[k],data(k==field?len:k==0?32768:k==4?256:16384,k+3));
                Chunk c=ChunkLoader.loadChunkIntoWorldFromCompound(w,n);snapshot(o,c);
                c.func_25124_i();EntityOracle.i(o,c.blocks.length);o.write(c.blocks);
            }
            for(String key:keys) {
                NBTTagCompound n=new NBTTagCompound();n.setInteger(key,1);
                try {ChunkLoader.loadChunkIntoWorldFromCompound(w,n);o.writeBoolean(false);}catch(ClassCastException e){o.writeBoolean(true);}
            }
        }
    }
}
