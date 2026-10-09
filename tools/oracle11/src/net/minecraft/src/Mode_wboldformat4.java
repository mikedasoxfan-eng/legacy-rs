package net.minecraft.src;
import java.io.*;
import java.util.*;
public final class Mode_wboldformat4 {
    static void text(DataOutputStream o,String s)throws Exception {byte[] b=s.getBytes("UTF-8");EntityOracle.i(o,Trace10.namedInt("b.length", b.length));Trace10.writeBytes(o, "b", b);}
    public static void main(String[] a)throws Exception {
        File root=new File("wb10-oldformat4-java");Mode_wbsavelife.clean(root);SaveFormatOld f=new SaveFormatOld(root);
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            Trace10.writeBoolean(o, "root.isDirectory()", root.isDirectory());text(o,f.getFormatName());
            for(int n=1;n<=7;n++) {
                File dir=new File(root,"World"+n);dir.mkdirs();NBTTagCompound data=new NBTTagCompound();data.setLong("RandomSeed",n);data.setInteger("GameType",n-2);data.setLong("LastPlayed",n*100);data.setLong("SizeOnDisk",n*1000);data.setString("LevelName","name"+n);
                NBTTagCompound tag=new NBTTagCompound();if(n==3)tag.setInteger("Data",4);else if(n!=2)tag.setCompoundTag("Data",data);
                CompressedStreamTools.writeGzippedCompoundToOutputStream(tag,new FileOutputStream(new File(dir,"level.dat")));
                if(n==3){NBTTagCompound old=new NBTTagCompound();old.setCompoundTag("Data",data);CompressedStreamTools.writeGzippedCompoundToOutputStream(old,new FileOutputStream(new File(dir,"level.dat_old")));}
            }
            List worlds=f.getSaveList();EntityOracle.i(o,Trace10.namedInt("worlds.size()", worlds.size()));
            for(Object object:worlds) {SaveFormatComparator w=(SaveFormatComparator)object;text(o,w.getFileName());text(o,w.getDisplayName());EntityOracle.l(o,Trace10.namedLong("w.getLastTimePlayed()", w.getLastTimePlayed()));EntityOracle.l(o,Trace10.namedLong("w.getLastTimePlayed()", w.getLastTimePlayed()));EntityOracle.i(o,Trace10.namedInt("w.getGameType()", w.getGameType()));Trace10.writeBoolean(o, "w.func_22161_d()", w.requiresConversion());}
            for(int n=1;n<=8;n++) {
                String name="World"+n;f.renameWorld(name,"renamed");WorldInfo info=f.getWorldInfo(name);Trace10.writeBoolean(o, "info!=null", info!=null);
                if(info!=null){EntityOracle.l(o,Trace10.namedLong("info.getRandomSeed()", info.getRandomSeed()));EntityOracle.i(o,Trace10.namedInt("info.getGameType()", info.getGameType()));text(o,info.getWorldName());}
                Trace10.writeBoolean(o, "f.isOldMapFormat(name)", f.isOldMapFormat(name));Trace10.writeBoolean(o, "f.convertMapFormat(name,null)", f.convertMapFormat(name,null));
            }
            for(boolean players:new boolean[]{false,true}) {String name=players?"withplayers":"withoutplayers";ISaveHandler s=f.getSaveLoader(name,players);s.checkSessionLock();Trace10.writeBoolean(o, "true", true);Trace10.writeBoolean(o, "new File(root,name+\"/players\").exists()", new File(root,name+"/players").exists());Trace10.writeBoolean(o, "new File(root,name+\"/data\").exists()", new File(root,name+"/data").exists());}
            f.flushCache();f.deleteWorldDirectory("World1");Trace10.writeBoolean(o, "new File(root,\"World1\").exists()", new File(root,"World1").exists());f.deleteWorldDirectory("absent");f.deleteWorldDirectory("World3");Trace10.writeBoolean(o, "new File(root,\"World3\").exists()", new File(root,"World3").exists());Mode_wbsavelife.clean(root);
        }
    }
}
