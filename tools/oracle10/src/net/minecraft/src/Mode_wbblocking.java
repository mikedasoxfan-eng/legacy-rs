package net.minecraft.src;
import java.io.*;import java.lang.reflect.*;
public final class Mode_wbblocking {
    public static void main(String[] a)throws Exception {
        int init=Block.stone.blockID;Method hit=EntityPlayer.class.getDeclaredMethod("damageEntity",DamageSource.class,int.class);hit.setAccessible(true);Field rem=EntityLiving.class.getDeclaredField("field_40129_bA");rem.setAccessible(true);
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            for(int armor=0;armor<4;armor++)for(int blocking=0;blocking<2;blocking++)for(int bypass=0;bypass<2;bypass++)for(int amount=0;amount<=40;amount++) {
                World w=Mode_worldblocks10.fixture(true);EntityPlayer p=PlayerOracle.player(w);p.inventory.mainInventory[0]=new ItemStack(267,1,0);if(blocking!=0)p.setItemInUse(p.inventory.mainInventory[0],72000);
                if(armor!=0)for(int k=0;k<4;k++)p.inventory.armorInventory[k]=new ItemStack(310+3-k,1,armor==1?0:armor==2?50:500);
                hit.invoke(p,bypass!=0?DamageSource.starve:DamageSource.cactus,amount);
                EntityOracle.i(o,Trace10.namedInt("p.health", p.health));EntityOracle.i(o,Trace10.namedInt("rem.getInt(p)", rem.getInt(p)));NBTTagCompound n=new NBTTagCompound();p.getFoodStats().writeStatsToNBT(n);EntityOracle.f(o,Trace10.namedFloat("n.getFloat(\"foodExhaustionLevel\")", n.getFloat("foodExhaustionLevel")));
                for(ItemStack s:p.inventory.armorInventory){Trace10.writeBoolean(o, "s!=null", s!=null);if(s!=null){EntityOracle.i(o,Trace10.namedInt("s.itemID", s.itemID));EntityOracle.i(o,Trace10.namedInt("s.stackSize", s.stackSize));EntityOracle.i(o,Trace10.namedInt("s.getItemDamage()", s.getItemDamage()));}}
            }
        }
    }
}
