package net.minecraft.src;
import java.io.*;
public final class Mode_wbfileclasses {
    static final String[] names={"c.0.0.dat","c.-1.1a.dat","c.-zik0zk.zik0zj.dat","c.zik0zk.0.dat","c.-zik0zl.0.dat","c.00001.-0.dat","c.a.b.dat.bak","c.A.b.dat","c.+1.0.dat","c..0.dat","c.1.2.3.dat","x","c.zzzzzzzzzz.0.dat","c.1.1.dat\n"};
    public static void main(String[] a)throws Exception {
        int init=Block.stone.blockID;
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            java.lang.reflect.Constructor<ChunkFilePattern> fc=ChunkFilePattern.class.getDeclaredConstructor();fc.setAccessible(true);ChunkFilePattern filter=fc.newInstance();
            for(String name:names) {
                Trace10.writeBoolean(o, "filter.accept(null,name)", filter.accept(null,name));
                try {ChunkFile c=new ChunkFile(new File(name));Trace10.writeBoolean(o, "true", true);EntityOracle.i(o,Trace10.namedInt("c.getXChunk()", c.getXChunk()));EntityOracle.i(o,Trace10.namedInt("c.getYChunk()", c.getYChunk()));Trace10.writeBoolean(o, "c.getChunkFile().getName().equals(name)", c.getChunkFile().getName().equals(name));}
                catch(NumberFormatException e){Trace10.writeBoolean(o, "false", false);}
            }
            for(String left:names)for(String right:names) {
                try {EntityOracle.i(o,Trace10.namedInt("new ChunkFile(new File(left)).compareChunks(new ChunkFile(new File(right)))", new ChunkFile(new File(left)).compareChunks(new ChunkFile(new File(right)))));}
                catch(NumberFormatException e){EntityOracle.i(o,Trace10.namedInt("Integer.MIN_VALUE", Integer.MIN_VALUE));}
            }
            String[] strings={"","a","aa","A","z","world","world2","\u00e9","\ud83d\ude00","\ue000"};
            long[] times={Long.MIN_VALUE,-1,0,1,Long.MAX_VALUE};
            for(long t:times)for(long u:times)for(String left:strings)for(String right:strings) {
                SaveFormatComparator x=new SaveFormatComparator(left,"display",t,u,3,true, false),y=new SaveFormatComparator(right,"other",u,t,0,false, false);
                EntityOracle.i(o,Trace10.namedInt("x.compareTo(y)", x.compareTo(y)));EntityOracle.l(o,Trace10.namedLong("x.getLastTimePlayed()", x.getLastTimePlayed()));EntityOracle.l(o,Trace10.namedLong("x.getLastTimePlayed()", x.getLastTimePlayed()));EntityOracle.i(o,Trace10.namedInt("x.getGameType()", x.getGameType()));Trace10.writeBoolean(o, "x.func_22161_d()", x.func_22161_d());Trace10.writeBoolean(o, "x.getFileName().equals(left)", x.getFileName().equals(left));Trace10.writeBoolean(o, "x.getDisplayName().equals(\"display\")", x.getDisplayName().equals("display"));
            }
            File root=new File("wb10-fileclasses-folders");root.mkdirs();java.lang.reflect.Constructor<ChunkFolderPattern> dc=ChunkFolderPattern.class.getDeclaredConstructor();dc.setAccessible(true);ChunkFolderPattern folders=dc.newInstance();
            for(String name:new String[]{"0","z","00","az","zz","000","A","-1","a.b","a_"}) {
                File dir=new File(root,name);dir.mkdir();Trace10.writeBoolean(o, "folders.accept(dir)", folders.accept(dir));dir.delete();
                try(FileOutputStream f=new FileOutputStream(dir)){}Trace10.writeBoolean(o, "folders.accept(dir)", folders.accept(dir));dir.delete();
            }
            root.delete();
            World w=Mode_worldblocks10.fixture(true);
            for(int below=0;below<=109;below++)if(below==0||Block.blocksList[below]!=null)for(int target=0;target<=109;target++)if(target==0||Block.blocksList[target]!=null) {
                Mode_worldblocks10.raw(w,8,63,8,below,0);Mode_worldblocks10.raw(w,8,64,8,target,0);
                Trace10.writeBoolean(o, "Block.blocksList[107].canPlaceBlockAt(w,8,64,8)", Block.blocksList[107].canPlaceBlockAt(w,8,64,8));
            }
        }
    }
}
