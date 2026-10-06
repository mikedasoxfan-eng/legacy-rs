package net.minecraft.src;
import java.io.*;
public final class Mode_wbregioncache4 {
    public static void main(String[] args)throws Exception {
        File root=new File("wb10-regioncache4-java");RegionFileCache.clearRegionFileReferences();Mode_wbsavelife.clean(root);
        File a=new File(root,"a"),b=new File(root,"b");
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(args[1])))) {
            for(File d:new File[]{a,a,b,a})EntityOracle.i(o,Trace10.namedInt("Harness10.regionSizeDelta(d,0,0)", Harness10.regionSizeDelta(d,0,0)));
            for(int n=0;n<36;n++) {
                int cx=(n%4)*32+n%3,cz=-(n%3)*32+n%5;File d=(n&1)==0?a:b;
                byte[] raw=new byte[n*47+1];for(int v=0;v<raw.length;v++)raw[v]=(byte)v;
                try(DataOutputStream chunk=RegionFileCache.getChunkOutputStream(d,cx,cz)){chunk.write(raw);}
                EntityOracle.i(o,Trace10.namedInt("Harness10.regionSizeDelta(d,cx,cz)", Harness10.regionSizeDelta(d,cx,cz)));
                ByteArrayOutputStream data=new ByteArrayOutputStream();try(DataInputStream chunk=RegionFileCache.getChunkInputStream(d,cx,cz)){int v;while((v=chunk.read())!=-1)data.write(v);}
                EntityOracle.i(o,Trace10.namedInt("data.size()", data.size()));Trace10.writeBytes(o, "data.toByteArray()", data.toByteArray());EntityOracle.i(o,Trace10.namedInt("Harness10.regionSizeDelta(d,cx,cz)", Harness10.regionSizeDelta(d,cx,cz)));
            }
            RegionFileCache.clearRegionFileReferences();EntityOracle.i(o,Trace10.namedInt("Harness10.regionSizeDelta(a,9000,0)", Harness10.regionSizeDelta(a,9000,0)));RegionFileCache.createOrLoadRegionFile(a,9000,0);
            for(int n=0;n<256;n++)RegionFileCache.createOrLoadRegionFile(b,n*32,4000);
            EntityOracle.i(o,Trace10.namedInt("Harness10.regionSizeDelta(a,9000,0)", Harness10.regionSizeDelta(a,9000,0)));EntityOracle.i(o,Trace10.namedInt("Harness10.regionSizeDelta(b,255*32,4000)", Harness10.regionSizeDelta(b,255*32,4000)));
            RegionFileCache.clearRegionFileReferences();Mode_wbsavelife.clean(root);Trace10.writeBoolean(o, "!root.exists()", !root.exists());
        }
    }
}
