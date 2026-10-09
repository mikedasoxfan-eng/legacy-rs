package net.minecraft.src;
import java.io.*;
import java.lang.reflect.*;
import java.util.*;

/** Shared state writers for the release blocks round-4 fixtures (original bytecode only). */
final class Blocks4Oracle {
    static void i(DataOutputStream o,int n)throws IOException {EntityOracle.i(o,n);}
    static void bool(DataOutputStream o,String name,boolean b)throws IOException {Trace10.writeBoolean(o,name,b);}
    static void stack(DataOutputStream o,ItemStack s)throws IOException {
        bool(o,"s!=null",s!=null);
        if(s!=null){i(o,Trace10.namedInt("s.itemID",s.itemID));i(o,Trace10.namedInt("s.stackSize",s.stackSize));i(o,Trace10.namedInt("s.getItemDamage()",s.getItemDamage()));bool(o,"s.hasTagCompound()",s.stackTagCompound!=null);}
    }
    static void seeds(World w)throws Exception {w.rand.setSeed(42);Item.itemRand.setSeed(123);OracleHooks.reset(100);EntityOracle.mathRandom().setSeed(456);WorldEventOracle.attach(w);}
    static void rngs(DataOutputStream o,World w)throws Exception {
        EntityOracle.l(o,Trace10.namedLong("w.rand",EntityOracle.rngState(w.rand)));EntityOracle.l(o,Trace10.namedLong("itemRand",EntityOracle.rngState(Item.itemRand)));EntityOracle.l(o,Trace10.namedLong("Math.random",EntityOracle.rngState(EntityOracle.mathRandom())));
    }
    static void entities(DataOutputStream o,World w)throws Exception {
        i(o,Trace10.namedInt("w.loadedEntityList.size()",w.loadedEntityList.size()));
        for(Object obj:w.loadedEntityList) {
            Entity e=(Entity)obj;String name=EntityList.getEntityString(e);WorldEventOracle.text(o,name==null?e.getClass().getSimpleName():name);
            for(double v:new double[]{Trace10.namedDouble("e.posX",e.posX),Trace10.namedDouble("e.posY",e.posY),Trace10.namedDouble("e.posZ",e.posZ),Trace10.namedDouble("e.motionX",e.motionX),Trace10.namedDouble("e.motionY",e.motionY),Trace10.namedDouble("e.motionZ",e.motionZ)})EntityOracle.d(o,v);
            EntityOracle.f(o,Trace10.namedFloat("e.rotationYaw",e.rotationYaw));EntityOracle.f(o,Trace10.namedFloat("e.rotationPitch",e.rotationPitch));
            if(e instanceof EntityItem){stack(o,((EntityItem)e).item);i(o,Trace10.namedInt("delayBeforeCanPickup",((EntityItem)e).delayBeforeCanPickup));}
            if(e instanceof EntityPotion)i(o,Trace10.namedInt("potionDamage",((EntityPotion)e).getPotionDamage()));
            if(e instanceof EntityArrow){bool(o,"arrowCritical",((EntityArrow)e).arrowCritical);bool(o,"doesArrowBelongToPlayer",((EntityArrow)e).doesArrowBelongToPlayer);}
        }
    }
    static void effects(DataOutputStream o,EntityLiving e)throws IOException {
        ArrayList<PotionEffect> list=new ArrayList<PotionEffect>(e.func_40118_aO());
        Collections.sort(list,(a,b)->Integer.compare(a.getPotionID(),b.getPotionID()));
        i(o,Trace10.namedInt("effects.size()",list.size()));
        for(PotionEffect p:list){i(o,Trace10.namedInt("p.getPotionID()",p.getPotionID()));i(o,Trace10.namedInt("p.getDuration()",p.getDuration()));i(o,Trace10.namedInt("p.getAmplifier()",p.getAmplifier()));}
    }
    static void player(DataOutputStream o,EntityPlayer p)throws Exception {
        for(ItemStack s:p.inventory.mainInventory)stack(o,s);
        ItemStack use=p.getItemInUse();stack(o,use);i(o,Trace10.namedInt("p.getItemInUseCount()",p.func_35205_Y()));
        bool(o,"use==held",use!=null&&use==p.inventory.mainInventory[p.inventory.currentItem]);
        i(o,Trace10.namedInt("food",p.getFoodStats().getFoodLevel()));EntityOracle.f(o,Trace10.namedFloat("saturation",p.getFoodStats().getFoodSaturationLevel()));
        effects(o,p);EntityOracle.l(o,Trace10.namedLong("p.rand",EntityOracle.rngState(p.rand)));
    }
    /** World.scheduledTickTreeSet in order (the global entry counter itself is not written). */
    static void queue(DataOutputStream o,World w)throws Exception {
        Field f=World.class.getDeclaredField("scheduledTickTreeSet");f.setAccessible(true);
        TreeSet<NextTickListEntry> set=(TreeSet<NextTickListEntry>)f.get(w);i(o,Trace10.namedInt("set.size()",set.size()));
        for(NextTickListEntry t:set){i(o,t.xCoord);i(o,t.yCoord);i(o,t.zCoord);i(o,t.blockID);EntityOracle.l(o,Trace10.namedLong("t.scheduledTime",t.scheduledTime));}
    }
    static EntityPlayer newPlayer(World w)throws Exception {
        return new Mode_wbplacement4.Player((net.minecraft.client.Minecraft)Oracle.allocate(PlayerOracle.HeadlessMinecraft.class),w);
    }
    static void place(EntityPlayer p,double x,double y,double z,float yaw,float pitch) {
        p.setLocationAndAngles(x,y,z,yaw,pitch);p.prevRotationYaw=yaw;p.prevRotationPitch=pitch;p.rand.setSeed(77);
    }
}
