package net.minecraft.src;

import java.io.*;
import java.util.*;
import java.lang.reflect.*;

/** Vehicle replays execute the original EntityMinecart/Boat and World bytecode. */
public final class VehicleOracle {
    static boolean recordStats;
    static final ArrayList<int[]> stats=new ArrayList<>();
    static void stat(StatBase stat,int count) {
        if(recordStats&&(stat==StatList.distanceByMinecartStat||stat==StatList.distanceByBoatStat||stat==StatList.distanceByPigStat||stat==AchievementList.onARail))stats.add(new int[]{stat.statId,count});
    }
    static void raw(World w,int x,int y,int z,int id,int meta) { EntityOracle.rawBlock(w,x,y,z,id,meta); }
    static EntityMinecart cart(World w,double x,double y,double z,int type,double vx,double vz) {
        EntityMinecart e=new EntityMinecart(w,x,y,z,type);e.motionX=vx;e.motionZ=vz;w.entityJoinedWorld(e);return e;
    }
    static World fixture() throws Exception {
        World w=Oracle.newWorld(0);EntityOracle.FlatProvider f=new EntityOracle.FlatProvider(w);w.chunkProvider=f;
        for(int x=-6;x<=6;x++)for(int z=-6;z<=6;z++)if(!f.chunkExists(x,z)) {
            byte[] blocks=new byte[32768];for(int bx=0;bx<16;bx++)for(int bz=0;bz<16;bz++)blocks[(bx<<11)|(bz<<7)|63]=1;
            Chunk c=new Chunk(w,blocks,x,z);c.isTerrainPopulated=true;c.isChunkLoaded=true;f.chunks.put(f.key(x,z),c);
        }
        w.difficultySetting=2;w.setAllowedMobSpawns(false,false);
        Field ids=Entity.class.getDeclaredField("nextEntityID");ids.setAccessible(true);ids.setInt(null,0);
        OracleHooks.reset(100);EntityOracle.mathRandom().setSeed(200);w.rand.setSeed(300);w.field_9437_g=0;
        Field sound=World.class.getDeclaredField("soundCounter");sound.setAccessible(true);sound.setInt(w,0);
        WorldEventOracle.attach(w);return w;
    }
    static void rails(World w) {
        for(int k=0;k<16;k++) {
            int x=-30+(k%4)*18,z=-30+(k/4)*18;
            int id=k<10?66:k<14?27:28,meta=k<10?k:k<12?k-10:k<14?(k-12)|8:k-14;
            for(int dx=-6;dx<=6;dx++)for(int dz=-6;dz<=6;dz++)raw(w,x+dx,64,z+dz,id,meta);
            EntityMinecart e=cart(w,x+0.3,64,z+0.7,k%3,(k%2==0?1:-1)*0.43,(k%3-1)*0.27);
            if(e.minecartType==2) {e.pushX=0.7;e.pushZ=-0.3;e.fuel=10;}
        }
    }
    static void train(World w) {
        for(int x=-40;x<=40;x++)raw(w,x,64,0,66,1);
        EntityMinecart furnace=cart(w,-8.5,64,0.5,2,0.2,0);furnace.pushX=1;furnace.fuel=1200;
        cart(w,-6.5,64,0.5,0,0,0);EntityMinecart chest=cart(w,-4.5,64,0.5,1,0,0);
        chest.setInventorySlotContents(0,new ItemStack(35,64,7));chest.setInventorySlotContents(26,new ItemStack(264,43,0));
        EntityPig pig=new EntityPig(w);pig.setLocationAndAngles(5.5,64,0.5,0,0);w.entityJoinedWorld(pig);
        cart(w,12.5,64,0.5,0,-0.25,0);
        cart(w,12.5,68,10.5,0,1,-1);
    }
    static void boats(World w) {
        for(int x=-35;x<=35;x++)for(int z=-20;z<=20;z++)raw(w,x,64,z,9,0);
        for(int x=8;x<=20;x++)for(int z=-20;z<=20;z++)raw(w,x,64,z,8,(x+z)&7);
        for(int z=-20;z<=20;z++){raw(w,25,64,z,1,0);raw(w,25,65,z,1,0);}
        for(int k=0;k<8;k++) {
            EntityBoat b=new EntityBoat(w,-20.5+k*6,65+(k%3)*0.2,-12.5+k*3);
            b.motionX=k%2==0?0.38:-0.17;b.motionY=-0.04;b.motionZ=(k%3-1)*0.21;b.rotationYaw=k*33.7F;w.entityJoinedWorld(b);
        }
        EntityBoat a=new EntityBoat(w,0,65,18);a.motionX=0.2;w.entityJoinedWorld(a);
        EntityBoat b=new EntityBoat(w,1.2,65,18);b.motionX=-0.2;w.entityJoinedWorld(b);
    }
    static void riding(World w) throws Exception {
        PlayerOracle.HeadlessPlayer p=PlayerOracle.player(w);p.setLocationAndAngles(-15.5,64,0.5,0,0);w.entityJoinedWorld(p);
        for(int x=-20;x<=8;x++)raw(w,x,64,0,x==0?28:66,1);
        raw(w,8,64,0,66,7);for(int z=1;z<=25;z++)raw(w,8,64,z,66,0);
        EntityMinecart c=cart(w,-15.5,64,0.5,0,0.38,0);c.interact(p);
        for(int x=-20;x<=0;x++)for(int z=10;z<=25;z++)raw(w,x,64,z,9,0);
        EntityBoat b=new EntityBoat(w,-10,65,15);w.entityJoinedWorld(b);
        EntityMinecart chest=cart(w,-10.5,64,5.5,1,0,0);chest.setInventorySlotContents(2,new ItemStack(263,64,0));
        cart(w,-7.5,64,5.5,2,0,0);
        EntityPig pig=new EntityPig(w);pig.setSaddled(true);pig.setLocationAndAngles(-5.5,64,5.5,0,0);w.entityJoinedWorld(pig);
    }
    static void placing(World w) throws Exception {
        PlayerOracle.HeadlessPlayer p=PlayerOracle.player(w);p.setLocationAndAngles(.5,66,.5,0,90);w.entityJoinedWorld(p);
        for(int x=-8;x<=8;x++){raw(w,x,64,8,66,1);raw(w,x,64,0,9,0);}
    }
    static void longrail(World w) throws Exception {
        EntityOracle.FlatProvider f=(EntityOracle.FlatProvider)w.chunkProvider;
        for(int x=7;x<=70;x++)for(int z=-6;z<=6;z++) {
            byte[] blocks=new byte[32768];for(int bx=0;bx<16;bx++)for(int bz=0;bz<16;bz++)blocks[(bx<<11)|(bz<<7)|63]=1;
            Chunk c=new Chunk(w,blocks,x,z);c.isTerrainPopulated=true;c.isChunkLoaded=true;f.chunks.put(f.key(x,z),c);
        }
        PlayerOracle.HeadlessPlayer p=PlayerOracle.player(w);p.setLocationAndAngles(-35.5,64,0.5,0,0);w.entityJoinedWorld(p);
        for(int x=-40;x<=1100;x++)raw(w,x,64,0,x==0?28:27,x==0?1:9);
        EntityMinecart c=cart(w,-35.5,64,0.5,0,0.38,0);c.interact(p);
    }
    static void nbt(Entity e,DataOutputStream o) throws Exception {
        NBTTagCompound tag=new NBTTagCompound();e.addEntityID(tag);
        ByteArrayOutputStream bytes=new ByteArrayOutputStream();DataOutputStream b=new DataOutputStream(bytes);
        b.writeByte(10);b.writeUTF("");PlayerOracle.canonical(tag,b);
        EntityOracle.i(o,bytes.size());o.write(bytes.toByteArray());
    }
    static void extra(World w,DataOutputStream o) throws Exception {
        for(int id=0;id<16;id++) {
            Entity e=null;for(Object a:w.loadedEntityList)if(((Entity)a).entityId==id)e=(Entity)a;
            if(e instanceof EntityMinecart) {
                EntityMinecart c=(EntityMinecart)e;
                for(double offset:new double[]{0,0.3F,-0.3F,1.2,-1.2}) {
                    Vec3D v=offset==0?c.func_514_g(c.posX,c.posY,c.posZ):c.func_515_a(c.posX,c.posY,c.posZ,offset);
                    o.writeBoolean(v!=null);if(v!=null){EntityOracle.d(o,v.xCoord);EntityOracle.d(o,v.yCoord);EntityOracle.d(o,v.zCoord);}
                }
            }
        }
        EntityOracle.i(o,stats.size());for(int[] stat:stats){EntityOracle.i(o,stat[0]);EntityOracle.i(o,stat[1]);}stats.clear();
        ArrayList<Entity> vehicles=new ArrayList<>();for(Object e:w.loadedEntityList)if(e instanceof EntityMinecart||e instanceof EntityBoat)vehicles.add((Entity)e);
        Collections.sort(vehicles,(a,b)->Integer.compare(a.entityId,b.entityId));EntityOracle.i(o,vehicles.size());
        for(Entity e:vehicles){EntityOracle.i(o,e.entityId);nbt(e,o);}
    }
    static void inventory(String path) throws Exception {
        World w=fixture();EntityMinecart c=cart(w,.5,64,.5,1,0,0);EntityPlayer p=PlayerOracle.player(w);Random r=new Random(1729);
        int[] items={1,35,263,264,267,280,351,332,5,61,54,328};
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(path)))) {
            o.write("B173VIN1".getBytes("UTF-8"));EntityOracle.i(o,10000);
            for(int k=0;k<10000;k++) {
                int op=r.nextInt(6),slot=r.nextInt(27),count=r.nextInt(100)-10,item=items[r.nextInt(items.length)];ItemStack result=null;boolean usable=false;
                if(op==0)c.setInventorySlotContents(slot,new ItemStack(item,count,slot));
                else if(op==1)result=c.decrStackSize(slot,count);
                else if(op==2)result=c.getStackInSlot(slot);
                else if(op==3){c.minecartType=r.nextInt(3);c.fuel=r.nextInt(131072)-65536;c.pushX=r.nextDouble()*4-2;c.pushZ=r.nextDouble()*4-2;NBTTagCompound n=new NBTTagCompound();c.addEntityID(n);c.readFromNBT(n);}
                else if(op==4){p.setPosition(r.nextDouble()*16-8,64,.5);usable=c.canInteractWith(p);}
                else {c.minecartCurrentDamage=r.nextInt(80);c.performHurtAnimation();}
                PlayerOracle.stack(o,result);o.writeBoolean(usable);EntityOracle.snapshot(w,o);nbt(c,o);
            }
        }
    }
    static void replay(String scene,int ticks,String path) throws Exception {
        recordStats=true;stats.clear();
        World w=fixture();
        if(scene.equals("rails"))rails(w);else if(scene.equals("train")||scene.equals("nbt"))train(w);
        else if(scene.equals("boats"))boats(w);else if(scene.equals("longrail"))longrail(w);else if(scene.equals("placing"))placing(w);else riding(w);
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(path)))) {
            o.write("B173VEH1".getBytes("UTF-8"));EntityOracle.i(o,ticks);EntityOracle.snapshot(w,o);extra(w,o);
            for(int t=0;t<ticks;t++) {
                if(scene.equals("placing")&&t%20==0) {
                    EntityPlayer p=(EntityPlayer)w.playerEntities.get(0);p.setLocationAndAngles(.5,66,.5,0,90);p.motionX=p.motionY=p.motionZ=0;
                    int phase=t/20%5,id=new int[]{328,342,343,333,333}[phase];ItemStack stack=new ItemStack(id,1,0);p.inventory.mainInventory[0]=stack;
                    if(phase<3)stack.useItem(p,w,(phase-1)*3,64,8,1);  // ItemStack.useItem, as a right click does (counts the use)
                    else {raw(w,0,65,0,phase==4?78:0,0);Item.itemsList[id].onItemRightClick(stack,w,p);}
                    if(stack.stackSize==0)p.inventory.mainInventory[0]=null;
                }
                if(scene.equals("longrail")) {
                    PlayerOracle.HeadlessPlayer p=(PlayerOracle.HeadlessPlayer)w.playerEntities.get(0);
                    if(t==80)((Entity)w.loadedEntityList.get(1)).setPosition(1000.5,64.5,0.5);
                    p.movementInput.sneak=t>=80&&t<100;
                    p.movementInput.moveForward=p.movementInput.moveStrafe=p.movementInput.sneak?.3F:1;
                }
                if(scene.equals("riding")) {
                    PlayerOracle.HeadlessPlayer p=(PlayerOracle.HeadlessPlayer)w.playerEntities.get(0);
                    p.movementInput.moveForward=t%40<30?1:0;p.movementInput.moveStrafe=t%60<15?1:0;
                    p.func_346_d((t%7-3)*7.3F,(t%5-2)*1.7F);
                    if(t==40)((EntityMinecart)w.loadedEntityList.get(1)).interact(p);
                    if(t==45)((EntityBoat)w.loadedEntityList.get(2)).interact(p);
                    if(t==80)((EntityBoat)w.loadedEntityList.get(2)).attackEntityFrom(p,5);
                    if(t==90){p.inventory.mainInventory[0]=new ItemStack(263,3,0);((EntityMinecart)w.loadedEntityList.get(3)).interact(p);}
                    if(t==100)((EntityMinecart)w.loadedEntityList.get(2)).attackEntityFrom(p,5);
                    if(t==110){for(Object a:w.loadedEntityList)if(a instanceof EntityPig){p.mountEntity((Entity)a);break;}}
                    if(t==140)p.mountEntity(null);
                }
                if(scene.equals("train")&&t==80) {
                    for(Object a:new ArrayList(w.loadedEntityList))if(a instanceof EntityMinecart&&((EntityMinecart)a).minecartType==1)((Entity)a).attackEntityFrom(null,5);
                }
                if(scene.equals("boats")&&t==50)for(Object a:new ArrayList(w.loadedEntityList))if(a instanceof EntityBoat&&((Entity)a).entityId%3==0)((Entity)a).attackEntityFrom(null,5);
                w.updateEntities();
                if(scene.equals("riding")||scene.equals("longrail")){w.worldInfo.setWorldTime(w.worldInfo.getWorldTime()+1);w.TickUpdates(false);}
                if((scene.equals("nbt")||scene.equals("ridingnbt"))&&t%25==24) {
                    ArrayList<NBTTagCompound> tags=new ArrayList<>();for(Object a:w.loadedEntityList){NBTTagCompound n=new NBTTagCompound();if(a instanceof EntityPlayer){((Entity)a).writeToNBT(n);n.setString("id","Player");tags.add(n);}else if(((Entity)a).addEntityID(n))tags.add(n);}
                    w.loadedEntityList.clear();w.playerEntities.clear();for(Chunk c:((EntityOracle.FlatProvider)w.chunkProvider).chunks.values())for(List list:c.entities)list.clear();
                    for(NBTTagCompound n:tags) {Entity e;if(n.getString("id").equals("Player")){e=PlayerOracle.player(w);e.readFromNBT(n);}else e=EntityList.createEntityFromNBT(n,w);w.entityJoinedWorld(e);}
                }
                EntityOracle.snapshot(w,o);extra(w,o);
            }
        }
        Oracle.dumpWorld(w,path+".world",0,0,0);try(OutputStream o=new FileOutputStream(path,true)){o.write(java.nio.file.Files.readAllBytes(new File(path+".world").toPath()));}new File(path+".world").delete();
    }
}
