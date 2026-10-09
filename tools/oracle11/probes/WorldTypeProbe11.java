package net.minecraft.src;
import java.io.*;
/** Calls the original classes, without copying their implementations. */
public final class WorldTypeProbe11 {
    static void s(DataOutputStream o,String s)throws Exception {byte[] b=s.getBytes("UTF-8");o.writeInt(b.length);o.write(b);}
    static void info(DataOutputStream o,WorldInfo w)throws Exception {
        o.writeBoolean(w!=null);if(w==null)return;
        s(o,w.func_46133_t().name());o.writeBoolean(w.isMapFeaturesEnabled());o.writeInt(w.getSaveVersion());
        o.writeInt(w.getDimension());
        o.writeLong(w.getRandomSeed());o.writeInt(w.getGameType());o.writeBoolean(w.isHardcoreModeEnabled());s(o,w.getWorldName());
    }
    static void corrupt(File f)throws Exception {
        NBTTagCompound d=new NBTTagCompound();d.setInteger("generatorName",1);
        NBTTagCompound r=new NBTTagCompound();r.setTag("Data",d);
        CompressedStreamTools.writeGzippedCompoundToOutputStream(r,new FileOutputStream(f));
    }
    public static void main(String[] a)throws Exception {
        long seed=Long.parseLong(a[0]);
        try(DataOutputStream o=new DataOutputStream(new FileOutputStream(a[1]))) {
            for(String name:new String[]{"DEFAULT","FLAT","default","flat",""," DEFAULT","FLAT ","unknown"}) {
                EnumWorldType t=EnumWorldType.func_46135_a(name);o.writeBoolean(t!=null);if(t!=null)s(o,t.func_46136_a());
                NBTTagCompound d=new NBTTagCompound();d.setString("generatorName",name);info(o,new WorldInfo(d));
            }
            info(o,new WorldInfo(new NBTTagCompound()));
            NBTTagCompound bad=new NBTTagCompound();bad.setInteger("generatorName",1);
            try {new WorldInfo(bad);o.writeBoolean(false);}catch(ClassCastException e){o.writeBoolean(true);}
            for(int id:new int[]{0,-1,1})for(EnumWorldType t:EnumWorldType.values()) {
                final WorldSettings settings=new WorldSettings(seed,0,false,false,t);
                World w=new World(new Oracle.NullSaveHandler(seed){public WorldInfo loadWorldInfo(){return new WorldInfo(settings,"oracle");}},"oracle",settings,WorldProvider.getProviderForDimension(id));
                WorldProvider p=w.worldProvider;
                o.writeBoolean(p.isHellWorld);o.writeBoolean(p.isAlternateDimension);o.writeBoolean(p.hasNoSky);
                o.writeBoolean(p.canRespawnHere());o.writeBoolean(p.func_28112_c());o.writeFloat(p.getCloudHeight());
                o.writeInt(p.func_46066_g());o.writeBoolean(p.func_46064_i());o.writeDouble(p.func_46065_j());o.writeDouble(w.func_46068_G());
                for(float v:p.lightBrightnessTable)o.writeFloat(v);
            }
            for(EnumWorldType t:EnumWorldType.values()) {
                WorldSettings settings=new WorldSettings(seed,1,false,true,t);
                WorldInfo fresh=new WorldInfo(settings,"fresh");info(o,fresh);
                SaveOldDir old=new SaveOldDir(new File(a[2]),"fresh-"+t.name(),false);
                old.saveWorldInfoAndPlayer(fresh,new java.util.ArrayList());info(o,fresh);info(o,old.loadWorldInfo());
            }
            SaveHandler h=new SaveHandler(new File(a[2]),"save",false);
            WorldInfo flat=new WorldInfo(new WorldSettings(seed,1,false,true,EnumWorldType.FLAT),"save");flat.setSaveVersion(19132);
            h.saveWorldInfo(flat);info(o,h.loadWorldInfo());
            WorldInfo normal=new WorldInfo(new WorldSettings(seed,0,true,false,EnumWorldType.DEFAULT),"save");normal.setSaveVersion(19132);
            h.saveWorldInfo(normal);info(o,h.loadWorldInfo());
            corrupt(new File(a[2],"save/level.dat"));info(o,h.loadWorldInfo());
            corrupt(new File(a[2],"save/level.dat_old"));info(o,h.loadWorldInfo());
            SaveOldDir routing=new SaveOldDir(new File(a[2]),"routing",false);
            for(int id:new int[]{0,-1,1}) {
                routing.getChunkLoader(WorldProvider.getProviderForDimension(id));
                o.writeBoolean(new File(a[2],"routing/DIM-1").isDirectory());
                o.writeBoolean(new File(a[2],"routing/DIM1").isDirectory());
            }
        }
    }
}
