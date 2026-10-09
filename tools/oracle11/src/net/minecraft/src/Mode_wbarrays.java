package net.minecraft.src;
import java.io.*;
public final class Mode_wbarrays {
    static final String[] keys={"Blocks","Data","SkyLight","BlockLight","HeightMap"};
    static byte[] data(int len,int salt){byte[] a=new byte[len];for(int k=0;k<len;k++)a[k]=(byte)(k*31+salt);return a;}
    static void snapshot(DataOutputStream o,Chunk c)throws Exception {
        EntityOracle.i(o,Trace10.namedInt("c.xPosition", c.xPosition));EntityOracle.i(o,Trace10.namedInt("c.zPosition", c.zPosition));
        for(byte[] b:new byte[][]{c.blocks,c.data.data,c.skylightMap.data,c.blocklightMap.data,c.heightMap}){EntityOracle.i(o,Trace10.namedInt("b.length", b.length));Trace10.writeBytes(o, "b", b);}
        EntityOracle.i(o,Trace10.namedInt("c.lowestBlockHeight", c.lowestBlockHeight));Trace10.writeBoolean(o, "c.isModified", c.isModified);Trace10.writeBoolean(o, "c.isTerrainPopulated", c.isTerrainPopulated);Trace10.writeBoolean(o, "c.hasEntities", c.hasEntities);
        for(int p:c.precipitationHeightMap)EntityOracle.i(o,p);for(boolean b:c.updateSkylightColumns)Trace10.writeBoolean(o, "b", b);
    }
    public static void main(String[] a)throws Exception {
        int init=Block.stone.blockID;World w=Mode_worldblocks10.fixture(true);
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            for(int len:new int[]{0,1,127,256,32768})snapshot(o,new Chunk(w,data(len,3),-7,11));
            for(int field=0;field<5;field++)for(int len:new int[]{-1,0,1,127,256,16384,32768,65536}) {
                NBTTagCompound n=new NBTTagCompound();n.setInteger("xPos",-7);n.setInteger("zPos",11);n.setBoolean("TerrainPopulated",(len&1)!=0);
                for(int k=0;k<5;k++)if(k!=field || len>=0)n.setByteArray(keys[k],data(k==field?len:k==0?32768:k==4?256:16384,k+3));
                Chunk c=ChunkLoader.loadChunkIntoWorldFromCompound(w,n);snapshot(o,c);
                c.removeUnknownBlocks();EntityOracle.i(o,Trace10.namedInt("c.blocks.length", c.blocks.length));Trace10.writeBytes(o, "c.blocks", c.blocks);
            }
            for(String key:keys) {
                NBTTagCompound n=new NBTTagCompound();n.setInteger(key,1);
                try {ChunkLoader.loadChunkIntoWorldFromCompound(w,n);Trace10.writeBoolean(o, "false", false);}catch(ClassCastException e){Trace10.writeBoolean(o, "true", true);}
            }
        }
    }
}
