package net.minecraft.src;
import java.io.*;
import java.util.*;
public final class Mode_wboldformat4 {
    static void text(DataOutputStream o,String s)throws Exception {byte[] b=s.getBytes("UTF-8");EntityOracle.i(o,b.length);o.write(b);}
    public static void main(String[] a)throws Exception {
        File root=new File("wb181-oldformat4-java");Mode_wbsavelife.clean(root);SaveFormatOld f=new SaveFormatOld(root);
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            o.writeBoolean(root.isDirectory());text(o,f.func_22178_a());
            for(int n=1;n<=7;n++) {
                File dir=new File(root,"World"+n);dir.mkdirs();NBTTagCompound data=new NBTTagCompound();data.setLong("RandomSeed",n);data.setInteger("GameType",n-2);data.setLong("LastPlayed",n*100);data.setLong("SizeOnDisk",n*1000);data.setString("LevelName","name"+n);
                NBTTagCompound tag=new NBTTagCompound();if(n==3)tag.setInteger("Data",4);else if(n!=2)tag.setCompoundTag("Data",data);
                CompressedStreamTools.writeGzippedCompoundToOutputStream(tag,new FileOutputStream(new File(dir,"level.dat")));
                if(n==3){NBTTagCompound old=new NBTTagCompound();old.setCompoundTag("Data",data);CompressedStreamTools.writeGzippedCompoundToOutputStream(old,new FileOutputStream(new File(dir,"level.dat_old")));}
            }
            List worlds=f.func_22176_b();EntityOracle.i(o,worlds.size());
            for(Object object:worlds) {SaveFormatComparator w=(SaveFormatComparator)object;text(o,w.getFileName());text(o,w.getDisplayName());EntityOracle.l(o,w.func_22163_e());EntityOracle.l(o,w.func_22159_c());EntityOracle.i(o,w.func_35719_f());o.writeBoolean(w.func_22161_d());}
            for(int n=1;n<=8;n++) {
                String name="World"+n;f.func_22170_a(name,"renamed");WorldInfo info=f.getWorldInfo(name);o.writeBoolean(info!=null);
                if(info!=null){EntityOracle.l(o,info.getRandomSeed());EntityOracle.i(o,info.func_35918_q());text(o,info.getWorldName());}
                o.writeBoolean(f.isOldMapFormat(name));o.writeBoolean(f.convertMapFormat(name,null));
            }
            for(boolean players:new boolean[]{false,true}) {String name=players?"withplayers":"withoutplayers";ISaveHandler s=f.getSaveLoader(name,players);s.checkSessionLock();o.writeBoolean(true);o.writeBoolean(new File(root,name+"/players").exists());o.writeBoolean(new File(root,name+"/data").exists());}
            f.flushCache();f.func_22172_c("World1");o.writeBoolean(new File(root,"World1").exists());f.func_22172_c("absent");f.func_22172_c("World3");o.writeBoolean(new File(root,"World3").exists());Mode_wbsavelife.clean(root);
        }
    }
}
