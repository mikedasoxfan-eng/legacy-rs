package net.minecraft.src;
import java.io.*;
import java.lang.reflect.*;
import java.util.*;

/** Original bow and arrow methods, with inert world sound/spawn sinks. */
public final class BowSlice11 {
    static final class ProbeWorld extends World {
        EntityArrow arrow;float pitch;Entity target;
        ProbeWorld(){super((ISaveHandler)null,"",new WorldSettings(0L,0,false,false,EnumWorldType.DEFAULT));}
        public void playSoundAtEntity(Entity entity,String name,float volume,float pitch){this.pitch=pitch;}
        public boolean spawnEntityInWorld(Entity entity){arrow=(EntityArrow)entity;return true;}
        public int getBlockId(int x,int y,int z){return 0;}
        public MovingObjectPosition rayTraceBlocks_do_do(Vec3D from,Vec3D to,boolean liquids,boolean ignore){return null;}
        public List getEntitiesWithinAABBExcludingEntity(Entity entity,AxisAlignedBB box){return target==null?Collections.emptyList():Collections.singletonList(target);}
        public void spawnParticle(String name,double x,double y,double z,double vx,double vy,double vz){}
    }
    static final class ProbeArrow extends EntityArrow {
        ProbeArrow(World world){super(world);}
        public void onEntityUpdate(){}
        public boolean isInWater(){return false;}
    }
    static final class ProbeTarget extends EntityLiving {
        boolean accepts;int damage;
        ProbeTarget(World world){super(world);setSize(0.9F,1.3F);}
        public int getMaxHealth(){return 1000;}
        public boolean attackEntityFrom(DamageSource source,int damage){this.damage=damage;return accepts;}
    }
    static sun.misc.Unsafe unsafe() throws Exception {
        Field f=sun.misc.Unsafe.class.getDeclaredField("theUnsafe");f.setAccessible(true);return (sun.misc.Unsafe)f.get(null);
    }
    static int integer(Object value,Class type,String name)throws Exception{
        Field f=type.getDeclaredField(name);f.setAccessible(true);return f.getInt(value);
    }
    public static void main(String[] args)throws Exception {
        int bootstrap=Block.stone.blockID;File directory=new File(args[0]);directory.mkdirs();
        PrintWriter out=new PrintWriter(new File(directory,"bows11.txt"));
        for(int creative=0;creative<=1;creative++)for(int ammo=0;ammo<=1;ammo++)
            for(int charge:new int[]{0,1,2,3,10,20,40})for(int mask=0;mask<16;mask++) {
                ProbeWorld w=(ProbeWorld)unsafe().allocateInstance(ProbeWorld.class);
                EntityPlayerSP p=(EntityPlayerSP)unsafe().allocateInstance(EntityPlayerSP.class);
                p.worldObj=w;p.capabilities=new PlayerCapabilities();p.capabilities.depleteBuckets=creative==1;
                p.inventory=new InventoryPlayer(p);p.rand=new Random(2);
                p.dataWatcher=new DataWatcher();p.dataWatcher.addObject(0,Byte.valueOf((byte)0));
                ItemStack bow=new ItemStack(261,1,0);p.inventory.mainInventory[0]=bow;
                if(ammo==1)p.inventory.mainInventory[1]=new ItemStack(262,3,0);
                for(int bit=0;bit<4;bit++)if((mask&(1<<bit))!=0)bow.addEnchantment(Enchantment.enchantmentsList[48+bit],bit<2?2:1);
                Item.itemRand.setSeed(42);
                Item.bow.onItemRightClick(bow,w,p);boolean began=p.isUsingItem();
                Item.bow.onPlayerStoppedUsing(bow,w,p,72000-charge);
                EntityArrow arrow=w.arrow;
                int arrows=p.inventory.mainInventory[1]==null?0:p.inventory.mainInventory[1].stackSize;
                out.println(creative+" "+ammo+" "+charge+" "+mask+" "+(began?1:0)+" "+(arrow==null?0:1)+" "+
                    bow.getItemDamage()+" "+arrows+" "+(arrow==null?0:Double.doubleToRawLongBits(arrow.func_46025_l()))+" "+
                    (arrow==null?0:integer(arrow,EntityArrow.class,"field_46027_au"))+" "+
                    (arrow==null?0:integer(arrow,Entity.class,"fire"))+" "+(arrow!=null&&arrow.doesArrowBelongToPlayer?1:0)+" "+
                    (arrow!=null&&arrow.arrowCritical?1:0)+" "+Float.floatToRawIntBits(w.pitch)+" "+Item.itemRand.nextLong());
            }
        out.close();
        PrintWriter selections=new PrintWriter(new File(directory,"bowenchant11.txt"));
        for(long seed:new long[]{0L,1L,42L,-1L,3257840388504953787L})for(int power=0;power<=60;power++){
            Random random=new Random(seed);List list=EnchantmentHelper.buildEnchantmentList(random,new ItemStack(261,1,0),power);
            selections.print(seed+" "+power+" "+random.nextLong()+" "+(list==null?-1:list.size()));
            if(list!=null)for(Object value:list){EnchantmentData data=(EnchantmentData)value;selections.print(" "+data.enchantmentobj.effectId+" "+data.enchantmentLevel);}
            selections.println();
        }
        selections.close();
        PrintWriter hits=new PrintWriter(new File(directory,"arrowhits11.txt"));
        for(int accepts=0;accepts<=1;accepts++)for(int burning=0;burning<=1;burning++)for(int critical=0;critical<=1;critical++)
            for(int punch=0;punch<=2;punch++)for(double damage:new double[]{2D,3.5D,5D}) {
                ProbeWorld w=(ProbeWorld)unsafe().allocateInstance(ProbeWorld.class);
                ProbeTarget target=new ProbeTarget(w);target.accepts=accepts==1;target.setPosition(0D,64D,1D);w.target=target;
                ProbeArrow arrow=new ProbeArrow(w);arrow.setPosition(0D,64.5D,0D);arrow.motionX=0.2D;arrow.motionY=0.1D;arrow.motionZ=1.25D;
                arrow.rand.setSeed(42);arrow.arrowCritical=critical==1;arrow.func_46024_b(damage);arrow.func_46023_b(punch);
                if(burning==1)arrow.setFire(10);
                arrow.onUpdate();
                hits.println(accepts+" "+burning+" "+critical+" "+punch+" "+Double.doubleToRawLongBits(damage)+" "+target.damage+" "+
                    integer(target,Entity.class,"fire")+" "+Double.doubleToRawLongBits(target.motionX)+" "+
                    Double.doubleToRawLongBits(target.motionY)+" "+Double.doubleToRawLongBits(target.motionZ)+" "+
                    (arrow.isDead?1:0)+" "+arrow.rand.nextLong());
            }
        hits.close();
        PrintWriter saves=new PrintWriter(new File(directory,"arrownbt11.txt"));
        for(double initial:new double[]{2D,3.5D})for(int type:new int[]{0,5,6}) {
            ProbeWorld w=(ProbeWorld)unsafe().allocateInstance(ProbeWorld.class);
            EntityArrow arrow=new EntityArrow(w);arrow.func_46024_b(initial);arrow.func_46023_b(2);
            NBTTagCompound n=new NBTTagCompound();
            if(type==5)n.setFloat("damage",3.25F);if(type==6)n.setDouble("damage",3.25D);
            boolean cast=false;try{arrow.readEntityFromNBT(n);}catch(ClassCastException expected){cast=true;}
            NBTTagCompound written=new NBTTagCompound();arrow.writeEntityToNBT(written);
            saves.println(Double.doubleToRawLongBits(initial)+" "+type+" "+(cast?1:0)+" "+
                Double.doubleToRawLongBits(arrow.func_46025_l())+" "+integer(arrow,EntityArrow.class,"field_46027_au")+" "+written.getTag("damage").getType());
        }
        saves.close();
        System.out.println("Generated original 1.1 bow release and enchant selection vectors");
    }
}
