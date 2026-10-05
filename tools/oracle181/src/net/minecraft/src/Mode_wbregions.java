package net.minecraft.src;
import java.io.*;
import java.util.Random;
public final class Mode_wbregions {
    public static void main(String[] a)throws Exception {
        File dest=new File(a[2]+".mcr");if(dest.exists()&&!dest.delete())throw new IOException("delete fixture");
        RegionFile r=new RegionFile(dest);
        try(DataOutputStream out=new DataOutputStream(new FileOutputStream(a[2]))) {
            EntityOracle.i(out,r.getSizeDelta());Random rng=new Random(881);
            for(int t=0;t<36;t++) {
                int len=new int[]{0,1,4090,8190,15000,65536}[t%6];byte[] data=new byte[len];for(int k=0;k<len;k++)data[k]=(byte)rng.nextInt(256);
                DataOutputStream chunk=r.getChunkDataOutputStream(t%4,t%3);chunk.write(data);chunk.close();EntityOracle.i(out,r.getSizeDelta());
            }
            r.close();r=new RegionFile(new File(a[1]));
            for(int x=-1;x<=32;x++)for(int z=-1;z<=32;z++) {
                DataInputStream in=r.getChunkDataInputStream(x,z);if(in==null){EntityOracle.i(out,-1);continue;}
                ByteArrayOutputStream bytes=new ByteArrayOutputStream();byte[] buf=new byte[4096];int n;while((n=in.read(buf))!=-1)bytes.write(buf,0,n);in.close();byte[] data=bytes.toByteArray();EntityOracle.i(out,data.length);out.write(data);
            }
            r.close();
        }
    }
}
