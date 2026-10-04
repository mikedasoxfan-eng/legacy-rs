package net.minecraft.src;

import java.io.*;
import java.util.*;
import java.lang.reflect.*;
import net.minecraft.client.Minecraft;

/** Player tests call the original jar's inventory, entity and controller methods. */
public final class PlayerOracle {
    static final class HeadlessMinecraft extends Minecraft {
        int portalRequests;
        HeadlessMinecraft(){super(null,null,null,1,1,false);}
        public void displayUnexpectedThrowable(UnexpectedThrowable t) {throw new RuntimeException();}
        public void usePortal() {portalRequests++;}
    }
    static final class HeadlessPlayer extends EntityPlayerSP {
        HeadlessPlayer(Minecraft mc,World w) {super(mc,w,new Session("Player",""),0);}
        public void addStat(StatBase stat,int count) {}
        public void onItemPickup(Entity e,int count) {}
    }
    static HeadlessPlayer player(World w)throws Exception {
        Minecraft mc=(Minecraft)Oracle.allocate(HeadlessMinecraft.class);
        mc.theWorld=w;mc.sndManager=(SoundManager)Oracle.allocate(SoundManager.class);
        StatFileWriter stats=(StatFileWriter)Oracle.allocate(StatFileWriter.class);
        Field map=StatFileWriter.class.getDeclaredField("field_25102_a");map.setAccessible(true);Map values=new HashMap();values.put(AchievementList.openInventory,1);map.set(stats,values);mc.statFileWriter=stats;
        HeadlessPlayer p=new HeadlessPlayer(mc,w);mc.thePlayer=p;p.movementInput=new MovementInput();return p;
    }
    static void state(EntityPlayer p,DataOutputStream o)throws Exception {
        for(float v:new float[]{p.field_775_e,p.field_774_f,p.cameraPitch,p.field_9328_R,p.timeInPortal,p.prevTimeInPortal})EntityOracle.f(o,v);
        for(int v:new int[]{p.swingProgressInt,p.timeUntilPortal,p.func_22060_M(),p.inventory.getTotalArmorValue(),p.inventory.currentItem,EntityOracle.intField(EntityPlayer.class,"damageRemainder",p)})i(o,v);
        o.writeBoolean(p.isSwinging);o.writeBoolean(p.isPlayerSleeping());o.writeBoolean(p.isSneaking());
        for(double v:new double[]{p.field_20066_r,p.field_20065_s,p.field_20064_t,p.field_20063_u,p.field_20062_v,p.field_20061_w})EntityOracle.d(o,v);
        for(ItemStack s:p.inventory.mainInventory)stack(o,s);for(ItemStack s:p.inventory.armorInventory)stack(o,s);
    }
    static void simulation(String scenario,int ticks,String path)throws Exception {
        World w=Oracle.newWorld(0);w.chunkProvider=new EntityOracle.FlatProvider(w);w.difficultySetting=2;w.setAllowedMobSpawns(false,false);
        Field ids=Entity.class.getDeclaredField("nextEntityID");ids.setAccessible(true);ids.setInt(null,0);
        OracleHooks.reset(100);EntityOracle.mathRandom().setSeed(200);w.rand.setSeed(300);w.field_9437_g=0;Field sound=World.class.getDeclaredField("soundCounter");sound.setAccessible(true);sound.setInt(w,0);
        Field chest=BlockChest.class.getDeclaredField("random");chest.setAccessible(true);((Random)chest.get(Block.chest)).setSeed(500);
        HeadlessPlayer p=player(w);p.setLocationAndAngles(0.5,64,0.5,0,0);w.entityJoinedWorld(p);
        for(int z=3;z<=5;z++)for(int x=-2;x<=2;x++)EntityOracle.rawBlock(w,x,64,z,1,0);
        for(int x=4;x<=8;x++)for(int z=-2;z<=2;z++)EntityOracle.rawBlock(w,x,64,z,53,0);
        for(int x=10;x<=15;x++)for(int y=64;y<=67;y++)for(int z=-2;z<=2;z++)EntityOracle.rawBlock(w,x,y,z,9,0);
        for(int x=-15;x<=-10;x++)for(int y=64;y<=67;y++)for(int z=-2;z<=2;z++)EntityOracle.rawBlock(w,x,y,z,11,0);
        EntityOracle.rawBlock(w,1,64,-5,30,0);for(int y=64;y<=71;y++){EntityOracle.rawBlock(w,0,y,-8,65,3);EntityOracle.rawBlock(w,0,y,-9,1,0);}
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(path)))) {
            EntityOracle.snapshot(w,o);
            for(int t=0;t<ticks;t++) {
                if(!p.isDead) {
                    if(t%120==0) {
                        int phase=t/120%8;p.motionX=p.motionY=p.motionZ=0;
                        double x=phase==2?4.5:phase==3?11.5:phase==4?-12.5:0.5;
                        double z=phase==6?-7.5:phase==7?-4.5:0.5;
                        double y=phase==5?76:phase==3||phase==4?66:64;
                        p.setLocationAndAngles(x,y,z,phase==2?-90:phase>=6?180:0,0);
                        if(scenario.equals("movement")){p.health=20;p.heartsLife=0;}
                    }
                    if(scenario.equals("damage")&&t==1) {for(int a=0;a<4;a++)p.inventory.armorInventory[a]=new ItemStack(310+a,1,a*40);p.attackEntityFrom(null,3);}
                    if(scenario.equals("damage")&&t==30) {EntityZombie mob=new EntityZombie(w);mob.setLocationAndAngles(0.5,64,1.5,0,0);w.entityJoinedWorld(mob);p.attackEntityFrom(mob,5);}
                    if(scenario.equals("pickup")&&t%20==0){EntityItem item=new EntityItem(w,p.posX,p.posY-1.3,p.posZ,new ItemStack(t%40==0?35:263,40,t%3));w.entityJoinedWorld(item);}
                    p.movementInput.moveForward=t%120<80?1:0;p.movementInput.moveStrafe=t%120>=80?1:0;p.movementInput.jump=t%31<8;p.movementInput.sneak=t%120>=90;
                    if(p.movementInput.sneak){p.movementInput.moveForward=(float)((double)p.movementInput.moveForward*0.3);p.movementInput.moveStrafe=(float)((double)p.movementInput.moveStrafe*0.3);}
                    if(t%13==0)p.swingItem();
                }
                AxisAlignedBB.clearBoundingBoxPool();Vec3D.initialize();w.updateEntities();EntityOracle.snapshot(w,o);
            }
        }
        Oracle.dumpWorld(w,path+".world",0,0,0);
        try(FileOutputStream o=new FileOutputStream(path,true)){o.write(java.nio.file.Files.readAllBytes(new File(path+".world").toPath()));}new File(path+".world").delete();
    }
    static void i(DataOutputStream o,int n)throws IOException {EntityOracle.i(o,n);}
    static void stack(DataOutputStream o,ItemStack s)throws IOException {
        for(int n:s==null?new int[]{-1,0,0,0}:new int[]{s.itemID,s.stackSize,s.getItemDamage(),s.animationsToGo})i(o,n);
    }
    static void registry(String path)throws Exception {
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(path)))) {
            for(int id=0;id<=2257;id++) {
                Item a=Item.itemsList[id];
                i(o,a==null?0:a.getItemStackLimit());i(o,a==null?0:a.getMaxDamage());i(o,a!=null&&a.getHasSubtypes()?1:0);i(o,a instanceof ItemArmor?((ItemArmor)a).damageReduceAmount:0);
                for(int b=0;b<256;b++) {
                    Block block=Block.blocksList[b];
                    EntityOracle.f(o,a==null||block==null?1:a.getStrVsBlock(new ItemStack(id,1,0),block));
                    o.writeBoolean(a!=null&&block!=null&&a.canHarvestBlock(block));
                }
            }
            java.lang.reflect.Field strength=Block.class.getDeclaredField("blockHardness");strength.setAccessible(true);
            for(int b=0;b<256;b++) {Block a=Block.blocksList[b];EntityOracle.f(o,a==null?0:strength.getFloat(a));o.writeBoolean(a!=null&&a.blockMaterial.getIsHarvestable());}
        }
    }
    static void inventory(String path)throws Exception {
        World w=Oracle.newWorld(0);w.chunkProvider=new EntityOracle.FlatProvider(w);
        EntityOtherPlayerMP player=new EntityOtherPlayerMP(w,"Player");InventoryPlayer inv=player.inventory;
        Random r=new Random(1729);int[] ids={1,35,263,256,257,298,299,300,301,332,351,357,2256};
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(path)))) {
            for(int t=0;t<10000;t++) {
                int op=r.nextInt(7),id=ids[r.nextInt(ids.length)],count=r.nextInt(80)+1,damage=r.nextInt(150),result=0;
                if(op<=1) {ItemStack s=new ItemStack(id,count,damage);result=inv.addItemStackToInventory(s)?1:0;stack(o,s);}
                else if(op==2)result=inv.consumeInventoryItem(id)?1:0;
                else if(op==3)stack(o,inv.decrStackSize(r.nextInt(40),count));
                else if(op==4) {inv.changeCurrentItem(count-40);result=inv.currentItem;}
                else if(op==5) {int slot=r.nextInt(4);inv.armorInventory[slot]=new ItemStack(298+r.nextInt(20),1,damage);inv.damageArmor(count);result=inv.getTotalArmorValue();}
                else {inv.decrementAnimations();inv.readFromNBT(inv.writeToNBT(new NBTTagList()));}
                i(o,result);for(ItemStack s:inv.mainInventory)stack(o,s);for(ItemStack s:inv.armorInventory)stack(o,s);i(o,inv.currentItem);i(o,inv.getTotalArmorValue());
            }
        }
    }
}
