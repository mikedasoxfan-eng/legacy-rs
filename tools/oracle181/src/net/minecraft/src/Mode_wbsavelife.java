package net.minecraft.src;
import java.io.*;
import java.util.*;
public final class Mode_wbsavelife {
    static void clean(File f){if(f.isDirectory())for(File child:f.listFiles())clean(child);f.delete();}
    static void write(File dir,String file,int kind,long seed)throws Exception {
        if(kind==0){try(FileOutputStream f=new FileOutputStream(new File(dir,file))){f.write(new byte[]{1,2,3});}return;}
        NBTTagCompound n=new NBTTagCompound();
        if(kind==1){NBTTagCompound data=new NBTTagCompound();data.setLong("RandomSeed",seed);data.setInteger("GameType",(int)seed);n.setCompoundTag("Data",data);}
        if(kind==3)n.setInteger("Data",4);
        CompressedStreamTools.writeGzippedCompoundToOutputStream(n,new FileOutputStream(new File(dir,file)));
    }
    static void info(DataOutputStream o,WorldInfo n)throws Exception {
        o.writeBoolean(n!=null);if(n==null)return;
        EntityOracle.l(o,n.getRandomSeed());EntityOracle.i(o,n.func_35918_q());o.writeBoolean(n.func_35917_r());EntityOracle.i(o,n.getSaveVersion());
    }
    static void state(DataOutputStream o,File dir,SaveHandler s)throws Exception {
        for(String name:new String[]{"data","players","region","level.dat","level.dat_old","level.dat_new"})o.writeBoolean(new File(dir,name).exists());
        info(o,s.loadWorldInfo());
    }
    static boolean session(SaveHandler s){try{s.checkSessionLock();return true;}catch(MinecraftException e){return false;}}
    public static void main(String[] a)throws Exception {
        int init=Block.stone.blockID;File root=new File("wb181-savelife-java");clean(root);root.mkdirs();
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            for(int n=0;n<8;n++) {
                File dir=new File(root,"world"+n);SaveHandler s=new SaveHandler(root,"world"+n,(n&1)!=0);state(o,dir,s);
                byte[] lock=new byte[8];try(FileInputStream f=new FileInputStream(new File(dir,"session.lock"))){f.read(lock);}
                o.writeBoolean(session(s));try(FileOutputStream f=new FileOutputStream(new File(dir,"session.lock"))){f.write(lock);f.write(99);}o.writeBoolean(session(s));
                try(FileOutputStream f=new FileOutputStream(new File(dir,"session.lock"))){f.write(new byte[7]);}o.writeBoolean(session(s));
                new File(dir,"session.lock").delete();o.writeBoolean(session(s));
                try(FileOutputStream f=new FileOutputStream(new File(dir,"session.lock"))){f.write(lock);}
                if(n>0)write(dir,"level.dat",n==1?2:n==2?3:n==3?0:1,71);
                if(n>=2)write(dir,"level.dat_old",1,99);
                state(o,dir,s);
                if(n>=6){new File(dir,"level.dat").delete();new File(dir,"level.dat").mkdir();new File(dir,"level.dat/keep").createNewFile();}
                if(n==7){new File(dir,"level.dat_old").delete();new File(dir,"level.dat_old").mkdir();new File(dir,"level.dat_old/keep").createNewFile();}
                NBTTagCompound tag=new NBTTagCompound();tag.setLong("RandomSeed",123);tag.setInteger("GameType",1);tag.setInteger("version",19132);
                s.saveWorldInfo(new WorldInfo(tag));state(o,dir,s);
                s.getChunkLoader(new WorldProviderHell());o.writeBoolean(new File(dir,"DIM-1").isDirectory());o.writeBoolean(new File(dir,"DIM-1/region").exists());
                o.writeBoolean(s.func_28113_a("map_1").equals(new File(dir,"data/map_1.dat")));
            }
        }
    }
}
