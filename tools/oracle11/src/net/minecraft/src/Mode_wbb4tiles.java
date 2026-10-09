package net.minecraft.src;
import java.io.*;

/** Release TileEntity factory: the new Cauldron/EnchantTable/Airportal mappings next to the old ones,
 * well-formed and malformed loads, and the written NBT of every loaded tile. */
public final class Mode_wbb4tiles {
    static NBTTagCompound slot(int s,int id,int count,int damage){NBTTagCompound n=new NBTTagCompound();n.setByte("Slot",(byte)s);n.setShort("id",(short)id);n.setByte("Count",(byte)count);n.setShort("Damage",(short)damage);return n;}
    public static void main(String[] a)throws Exception {
        int init=Block.stone.blockID;
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            World w=Mode_worldblocks10.fixture(true);
            for(String name:new String[]{"Cauldron","EnchantTable","Airportal","Furnace","Chest","Trap","Sign","MobSpawner","Music","Piston","RecordPlayer","cauldron","Airportal\0","EnchantmentTable"})for(int malformed=0;malformed<7;malformed++) {
                NBTTagCompound n=new NBTTagCompound();n.setString("id",name);n.setInteger("x",-123);n.setInteger("y",64);n.setInteger("z",456);
                if(malformed==1)n.setByte("x",(byte)1);if(malformed==2)n.setInteger("id",1);if(malformed==3)n.setInteger("Items",1);if(malformed==4)n.setInteger("BrewTime",5);
                if(malformed>=5){NBTTagList list=new NBTTagList();list.setTag(slot(0,373,1,8193));list.setTag(slot(3,372,2,0));list.setTag(slot(2,374,64,0));if(malformed==6){list.setTag(slot(4,1,1,0));list.setTag(slot(-1,1,1,0));}n.setTag("Items",list);n.setShort("BrewTime",(short)(malformed==5?77:-3));}
                TileEntity t=null;int e=0;try{t=TileEntity.createAndLoadEntity(n);}catch(Throwable ex){e=Mode_wbmachine10.error(ex);}
                Trace10.writeByte(o,"e",e);Trace10.writeBoolean(o,"t!=null",t!=null);
                if(t!=null){Mode_wbtileentity10.snapshot(o,t,w);Mode_wbinventory7.nbt(o,t);}
            }
        }
    }
}
