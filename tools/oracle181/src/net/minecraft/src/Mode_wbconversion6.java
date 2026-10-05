package net.minecraft.src;
import java.io.*;import java.util.*;import java.util.zip.*;
public final class Mode_wbconversion6 {
    static final int[][] COORDS={{0,0},{1,1},{31,31},{-1,-1},{32,0},{0,32},{1,1}};
    static void chunks(File dir,int dimension)throws Exception {
        for(int n=0;n<COORDS.length;n++) {
            int[] p=COORDS[n];File sub=new File(dir,new String[]{"0","1","b"}[n%3]+"/0");sub.mkdirs();
            byte[] data=new byte[128+n*23];for(int v=0;v<data.length;v++)data[v]=(byte)(v*37+n*11+dimension*13);
            try(OutputStream out=new GZIPOutputStream(new FileOutputStream(new File(sub,"c."+Integer.toString(p[0],36)+"."+Integer.toString(p[1],36)+".dat")))){out.write(data);}
        }
    }
    static void write(File dir,int x,int z,byte[] data)throws Exception {try(DataOutputStream o=RegionFileCache.getChunkOutputStream(dir,x,z)){o.write(data);}}
    public static void main(String[] a)throws Exception {
        File root=new File("wb181-conversion6-java");RegionFileCache.clearRegionFileReferences();Mode_wbsavelife.clean(root);
        SaveConverterMcRegion converter=new SaveConverterMcRegion(root);
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            for(int config=0;config<8;config++) {
                String name="world"+config;File dir=new File(root,name),nether=new File(dir,"DIM-1");
                NBTTagCompound data=new NBTTagCompound();data.setLong("RandomSeed",900+config);data.setInteger("GameType",config-2);data.setBoolean("MapFeatures",(config&1)!=0);data.setInteger("version",0);data.setLong("SizeOnDisk",1234);data.setString("LevelName","converted");
                ISaveHandler storage=converter.getSaveLoader(name,false);storage.saveWorldInfo(new WorldInfo(data));
                if((config&4)==0)chunks(dir,0);else new File(dir,"a").mkdirs();
                if((config&1)!=0){nether.mkdirs();if((config&4)==0)chunks(nether,1);}
                if((config&2)!=0){byte[] bytes=new byte[167];Arrays.fill(bytes,(byte)90);write(dir,0,0,bytes);}
                File foreign=new File(root,"foreign");byte[] bytes=new byte[100];Arrays.fill(bytes,(byte)5);write(foreign,config,0,bytes);
                o.writeBoolean(converter.isOldMapFormat(name));final List<Integer> progress=new ArrayList<Integer>();
                o.writeBoolean(converter.convertMapFormat(name,new IProgressUpdate(){public void func_594_b(String s){}public void displayLoadingString(String s){}public void setLoadingProgress(int p){progress.add(p);}}));
                EntityOracle.i(o,progress.size());for(int p:progress)EntityOracle.i(o,p);o.writeBoolean(converter.isOldMapFormat(name));
                WorldInfo info=converter.getWorldInfo(name);EntityOracle.l(o,info.getRandomSeed());EntityOracle.i(o,info.func_35918_q());o.writeBoolean(info.func_35917_r());EntityOracle.i(o,info.getSaveVersion());EntityOracle.l(o,info.getSizeOnDisk());
                for(String s:new String[]{"players","0","1","b","a","DIM-1/0","DIM-1/1","DIM-1/b"})o.writeBoolean(new File(dir,s).exists());
                EntityOracle.i(o,RegionFileCache.getSizeDelta(foreign,config,0));
                for(File d:new File[]{dir,nether})for(int[] p:COORDS) {
                    DataInputStream in=RegionFileCache.getChunkInputStream(d,p[0],p[1]);ByteArrayOutputStream payload=new ByteArrayOutputStream();
                    if(in==null)EntityOracle.i(o,-1);else{int v;while((v=in.read())!=-1)payload.write(v);in.close();EntityOracle.i(o,payload.size());o.write(payload.toByteArray());}
                    try(RandomAccessFile file=new RandomAccessFile(new File(d,"region/r."+(p[0]>>5)+"."+(p[1]>>5)+".mcr"),"r")){file.seek(((p[0]&31)+(p[1]&31)*32)*4);EntityOracle.i(o,file.readInt());}
                }
                RegionFileCache.clearRegionFileReferences();
            }
        }
        Mode_wbsavelife.clean(root);
    }
}
