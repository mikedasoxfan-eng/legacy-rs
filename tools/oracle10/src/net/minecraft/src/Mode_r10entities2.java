package net.minecraft.src;

import java.io.*;
import java.lang.reflect.*;
import java.util.*;

/** Release-only checks. Game methods are loaded from the original jar; no game
 * method is recompiled here. The existing oracle supplies deterministic RNGs. */
public final class Mode_r10entities2 {
    static DataOutputStream out;
    static PrintWriter labels;
    static long offset;
    static void value(String name, int v) throws Exception {
        labels.println(offset+"\t4\t"+name); offset+=4; out.writeInt(Integer.reverseBytes(v));
    }
    static void value(String name, long v) throws Exception {
        labels.println(offset+"\t8\t"+name); offset+=8; out.writeLong(Long.reverseBytes(v));
    }
    static void value(String name, float v) throws Exception { value(name,Float.floatToIntBits(v)); }
    static void value(String name, double v) throws Exception { value(name,Double.doubleToLongBits(v)); }
    static void text(String name,String s) throws Exception {
        if(s==null){value(name+".length",-1);return;}byte[] b=s.getBytes("UTF-8");value(name+".length",b.length);labels.println(offset+"\t"+b.length+"\t"+name);offset+=b.length;out.write(b);
    }
    static Object field(Object obj, String name) throws Exception {
        Class<?> c=obj.getClass();
        while(c!=null) { try { Field f=c.getDeclaredField(name); f.setAccessible(true); return f.get(obj); }
            catch(NoSuchFieldException ex) {c=c.getSuperclass();} }
        throw new NoSuchFieldException(name);
    }
    static void set(Object obj,String name,Object v) throws Exception {
        Class<?> c=obj.getClass();
        while(c!=null) { try { Field f=c.getDeclaredField(name); f.setAccessible(true); f.set(obj,v); return; }
            catch(NoSuchFieldException ex) {c=c.getSuperclass();} }
        throw new NoSuchFieldException(name);
    }
    static void entity(Entity e) throws Exception {
        value("id",e.entityId);
        for(String n:new String[]{"posX","posY","posZ","motionX","motionY","motionZ","prevPosX","prevPosY","prevPosZ"}) value(n,(Double)field(e,n));
        AxisAlignedBB b=e.boundingBox;
        for(double v:new double[]{b.minX,b.minY,b.minZ,b.maxX,b.maxY,b.maxZ}) value("box",v);
        for(String n:new String[]{"rotationYaw","rotationPitch","prevRotationYaw","prevRotationPitch","width","height","yOffset","ySize","fallDistance","distanceWalkedModified"}) value(n,(Float)field(e,n));
        value("ticks",e.ticksExisted); value("fire",(Integer)field(e,"fire")); value("air",e.func_41001_Z());
        for(boolean v:new boolean[]{e.onGround,e.isDead,e.inWater,e.beenAttacked,e.isAirBorne,e.noClip,e.isCollidedHorizontally,e.isCollidedVertically}) value("flag",v?1:0);
        value("rand",EntityOracle.rngState(e.rand));
        if(e instanceof EntityLiving) {
            EntityLiving l=(EntityLiving)e;
            for(String n:new String[]{"health","prevHealth","heartsLife","hurtTime","maxHurtTime","deathTime","attackTime","naturalArmorRating","field_40129_bA","field_34905_c"}) value(n,(Integer)field(l,n));
            value("renderYaw",l.renderYawOffset); value("randomYaw",l.randomYawVelocity);
        }
    }
    static void dragon(World w,EntityDragon e) throws Exception {
        entity(e);
        value("targetX",e.field_40167_a); value("targetY",e.field_40165_b); value("targetZ",e.field_40166_c);
        value("historyIndex",e.field_40164_e);
        for(double[] h:e.field_40162_d) for(double v:h) value("history",v);
        value("previousAnimation",e.field_40173_aw); value("animation",e.field_40172_ax);
        value("forceTarget",e.field_40163_ay?1:0); value("slowed",e.field_40161_az?1:0);
        Entity t=(Entity)field(e,"field_40179_aC"); value("target",t==null?-1:t.entityId);
        value("crystal",e.field_41013_bH==null?-1:e.field_41013_bH.entityId);
        value("deathTicks",e.field_40178_aA); value("healthWatcher",e.func_41010_ax());
        for(DragonPart p:e.field_40176_ao) entity(p);
        value("worldRand",EntityOracle.rngState(w.rand)); value("mathRand",EntityOracle.rngState(EntityOracle.mathRandom()));
        value("nextSeed",OracleHooks.nextSeed());
        value("joined",w.loadedEntityList.size());
        for(Object obj:w.loadedEntityList) { Entity a=(Entity)obj; entity(a); if(a instanceof EntityXPOrb) value("xp",((EntityXPOrb)a).getXpValue()); }
        ByteArrayOutputStream bytes=new ByteArrayOutputStream(); WorldEventOracle.snapshot(w,new DataOutputStream(bytes));
        byte[] events=bytes.toByteArray(); labels.println(offset+"\t"+events.length+"\tevents"); offset+=events.length; out.write(events);
    }
    static void runDragon(String kind) throws Exception {
        if(kind.equals("dragonhistory")) {
            World w=Mode_entities10.world();EntityDragon e=new EntityDragon(w);
            for(int j=0;j<64;j++) { e.field_40162_d[j]=new double[]{j*173.25-7000,j*j*0.125-10,j*0.2}; }
            for(int idx:new int[]{-1,0,1,31,63}) for(int hp:new int[]{0,1,200}) for(int n:new int[]{0,1,5,10,12,16,63,64,129}) for(float p:new float[]{-0.5F,0,0.25F,1,1.5F}) {
                e.field_40164_e=idx;e.health=hp;for(double v:e.func_40160_a(n,p))value("offset",v);
            }
            return;
        }
        if(kind.equals("dragondamage")) {
            for(int source=0;source<4;source++) for(int part=0;part<7;part++) for(int damage:new int[]{-7,0,1,3,4,19,201}) {
                World w=Mode_entities10.world(); EntityDragon e=new EntityDragon(w);e.setPosition(-3.5,80,4.5);e.rotationYaw=173.25F;
                EntityLiving a=source==2?new Mode_entities10.Player(w):new EntityZombie(w);a.setPosition(4,78,-3);w.entityJoinedWorld(a);
                DamageSource s=source==0?DamageSource.generic:source==1?DamageSource.explosion:DamageSource.causeMobDamage(a);
                value("direct",e.attackEntityFrom(s,damage)?1:0);value("partAttack",e.field_40176_ao[part].attackEntityFrom(s,damage)?1:0);dragon(w,e);
                value("partSelf",e.field_40176_ao[part].func_41004_h(e.field_40176_ao[part])?1:0);value("partParent",e.field_40176_ao[part].func_41004_h(e)?1:0);
            } return;
        }
        int scenes=kind.equals("dragondeath")?2:12;
        for(int scene=0;scene<scenes;scene++) {
            World w=Mode_entities10.world();EntityDragon e=new EntityDragon(w);dragon(w,e);
            e.setPosition(-3.5,80,4.5);e.rotationYaw=179;e.rotationPitch=7;
            e.motionX=0.35;e.motionY=-0.125;e.motionZ=0.6;
            if(kind.equals("dragondeath")) {e.health=0;w.multiplayerWorld=scene==1;for(int tick=0;tick<201;tick++){e.func_40120_m_();dragon(w,e);} }
            else {
                w.multiplayerWorld=scene==1||scene==2;
                if(scene==2)e.setPositionAndRotation2(8,86,-4,-179,31,7);
                if(scene==3)e.field_40163_ay=true;
                if(scene==4)e.field_40161_az=true;
                if(scene==5) {e.field_40167_a=120;e.field_40165_b=78;e.field_40166_c=95;}
                if(scene==6||scene==7) {Mode_entities10.Player p=new Mode_entities10.Player(w);p.setPosition(4,78,-3);w.entityJoinedWorld(p);set(e,"field_40179_aC",p);if(scene==7)p.setPosition(1000,90,1000);}
                if(scene==8||scene==9) {EntityEnderCrystal c=new EntityEnderCrystal(w,0,80,0);w.entityJoinedWorld(c);e.field_41013_bH=c;e.health=190;c.isDead=scene==9;}
                if(scene==10) {for(int x=-10;x<=4;x++) for(int y=79;y<=82;y++)for(int z=-4;z<=9;z++)Mode_entityround6_10.raw(w,x,y,z,(x+z)%3==0?49:1);}
                if(scene==11) {e.health=0;}
                w.rand.setSeed(300);for(int tick=0;tick<96;tick++){e.ticksExisted=tick;e.onLivingUpdate();dragon(w,e);}
            }
            for(int x=-10;x<=6;x++) for(int y=63;y<=97;y++) for(int z=-5;z<=12;z++){value("block",w.getBlockId(x,y,z));value("metadata",w.getBlockMetadata(x,y,z));}
        }
    }
    static void fish(World w,EntityFishHook e) throws Exception {
        entity(e);
        for(String n:new String[]{"xTile","yTile","zTile","inTile","shake","ticksInGround","ticksInAir","ticksCatchable","fishPosRotationIncrements"})value(n,(Integer)field(e,n));
        value("inGround",(Boolean)field(e,"inGround")?1:0);
        value("owner",e.angler==null?-1:e.angler.entityId);value("hooked",e.bobber==null?-1:e.bobber.entityId);
        for(String n:new String[]{"fishX","fishY","fishZ","fishYaw","fishPitch","velocityX","velocityY","velocityZ"})value(n,(Double)field(e,n));
        value("ownerHook",e.angler==null||e.angler.fishEntity==null?-1:e.angler.fishEntity.entityId);
        value("worldRand",EntityOracle.rngState(w.rand));value("mathRand",EntityOracle.rngState(EntityOracle.mathRandom()));value("nextSeed",OracleHooks.nextSeed());
        value("joined",w.loadedEntityList.size());for(Object obj:w.loadedEntityList)entity((Entity)obj);
        ByteArrayOutputStream bytes=new ByteArrayOutputStream();WorldEventOracle.snapshot(w,new DataOutputStream(bytes));byte[] events=bytes.toByteArray();labels.println(offset+"\t"+events.length+"\tevents");offset+=events.length;out.write(events);
    }
    static void runFish() throws Exception {
        for(int seed=0;seed<8;seed++)for(int scene=0;scene<14;scene++) {
            World w=Mode_entities10.world();Mode_entities10.Player p=new Mode_entities10.Player(w);p.setPosition(-3.5,68,4.5);p.rotationYaw=seed*47;p.rotationPitch=seed*9-35;p.inventory.mainInventory[0]=new ItemStack(346,1,0);w.entityJoinedWorld(p);
            EntityFishHook e=new EntityFishHook(w,p);e.rand.setSeed(seed+1000);fish(w,e);
            if(scene==1){w.multiplayerWorld=true;p.inventory.mainInventory[0]=null;}
            if(scene==2){p.inventory.mainInventory[0]=null;}
            if(scene==3){p.health=0;}
            if(scene==4){p.isDead=true;}
            if(scene==5){p.setPosition(100,68,100);}
            if(scene==6){e.setVelocity(.2,-.1,.3);e.setPositionAndRotation2(4,71,-2,-179,22,5);}
            if(scene==7||scene==8){e.bobber=new EntityCow(w);e.bobber.setPosition(-2,66,4);e.bobber.isDead=scene==8;w.entityJoinedWorld(e.bobber);}
            if(scene==9||scene==10){set(e,"inGround",true);set(e,"xTile",-3);set(e,"yTile",63);set(e,"zTile",4);set(e,"inTile",scene==9?1:49);set(e,"ticksInGround",1197);}
            if(scene==11||scene==12){e.setPosition(-3.5,65,4.5);e.motionX=e.motionY=e.motionZ=0;for(int x=-8;x<=2;x++)for(int y=64;y<=68;y++)for(int z=0;z<=9;z++)Mode_entityround6_10.raw(w,x,y,z,9);if(scene==12)set(e,"ticksCatchable",5);}
            if(scene==13){EntityCow cow=new EntityCow(w);cow.setPosition(e.posX,e.posY,e.posZ+1);w.entityJoinedWorld(cow);e.setVelocity(0,0,.7);}
            for(int tick=0;tick<(scene==11?1000:12);tick++){e.onUpdate();fish(w,e);if(e.isDead)break;}
            if(!e.isDead){value("reel",e.catchFish());fish(w,e);}
            NBTTagCompound n=new NBTTagCompound();e.writeEntityToNBT(n);EntityFishHook copy=new EntityFishHook(w);copy.readEntityFromNBT(n);
            for(String name:new String[]{"xTile","yTile","zTile","inTile","shake"})value("nbt."+name,(Integer)field(copy,name));value("nbt.inGround",(Boolean)field(copy,"inGround")?1:0);
        }
    }
    static void throwable(World w,EntityThrowable e) throws Exception {
        entity(e);
        for(String n:new String[]{"field_40079_d","field_40080_e","field_40082_ao","field_40084_ap","field_40081_b","field_40087_aq","field_40086_ar"})value(n,(Integer)field(e,n));
        value("inGround",(Boolean)field(e,"field_40085_a")?1:0);Entity owner=(Entity)field(e,"field_40083_c");value("owner",owner==null?-1:owner.entityId);
        value("worldRand",EntityOracle.rngState(w.rand));value("mathRand",EntityOracle.rngState(EntityOracle.mathRandom()));value("nextSeed",OracleHooks.nextSeed());
        value("joined",w.loadedEntityList.size());for(Object obj:w.loadedEntityList){Entity a=(Entity)obj;entity(a);if(a instanceof EntityLiving){EntityLiving l=(EntityLiving)a;value("age",l instanceof EntityAnimal?((EntityAnimal)l).func_40146_g():0);ArrayList<Integer> ids=new ArrayList<Integer>(l.activePotionsMap.keySet());Collections.sort(ids);value("effects",ids.size());for(int id:ids){PotionEffect p=(PotionEffect)l.activePotionsMap.get(id);value("effect.id",id);value("effect.duration",p.getDuration());value("effect.amplifier",p.getAmplifier());}}}
        ByteArrayOutputStream bytes=new ByteArrayOutputStream();WorldEventOracle.snapshot(w,new DataOutputStream(bytes));byte[] events=bytes.toByteArray();labels.println(offset+"\t"+events.length+"\tevents");offset+=events.length;out.write(events);
    }
    static void runThrowable(String kind) throws Exception {
        int type=kind.equals("egg")?0:kind.equals("snowball")?1:kind.equals("pearl")?2:3;
        for(int seed=0;seed<128;seed++)for(int scene=0;scene<9;scene++) {
            World w=Mode_entities10.world();EntityLiving owner=scene==8?new EntityCow(w):new Mode_entities10.Player(w);owner.setPosition(-3.5,68,4.5);owner.rotationYaw=seed*17;owner.rotationPitch=seed%85-40;w.entityJoinedWorld(owner);
            int metadata=new int[]{16389,16396,16421,16428,16385,16386,16388,16392}[seed%8];
            EntityThrowable e;
            if(scene==0||scene==1)e=type==0?new EntityEgg(w,-3.5,68,4.5):type==1?new EntitySnowball(w,-3.5,68,4.5):type==2?new EntityEnderPearl(w,-3.5,68,4.5):new EntityPotion(w,-3.5,68,4.5,metadata);
            else e=type==0?new EntityEgg(w,owner):type==1?new EntitySnowball(w,owner):type==2?new EntityEnderPearl(w,owner):new EntityPotion(w,owner,metadata);
            throwable(w,e);e.rand.setSeed(seed+1000);
            if(scene==1){e.setVelocity(.25,-.5,.75);}
            if(scene==2||scene==3){set(e,"field_40085_a",true);set(e,"field_40079_d",-3);set(e,"field_40080_e",63);set(e,"field_40082_ao",4);set(e,"field_40084_ap",scene==2?1:49);set(e,"field_40087_aq",1197);}
            if(scene==4){e.setPosition(-3.5,65,4.5);e.setVelocity(.1,-.05,.1);for(int x=-8;x<=2;x++)for(int y=64;y<=68;y++)for(int z=0;z<=9;z++)Mode_entityround6_10.raw(w,x,y,z,9);}
            if(scene==5||scene==6||scene==7){e.setPosition(-3.5,65,4.5);e.setVelocity(0,-.8,0);if(scene==6){EntityLiving b=new EntityBlaze(w);b.setPosition(-3.5,64,4.5);w.entityJoinedWorld(b);}if(scene==7)w.multiplayerWorld=true;}
            for(int tick=0;tick<8;tick++){e.onUpdate();throwable(w,e);if(e.isDead)break;}
            NBTTagCompound n=new NBTTagCompound();e.writeEntityToNBT(n);e.readEntityFromNBT(n);
            for(String name:new String[]{"xTile","yTile","zTile"})value("nbt."+name,(int)n.getShort(name));for(String name:new String[]{"inTile","shake","inGround"})value("nbt."+name,(int)n.getByte(name));
            if(type==3)value("potionValue",n.getInteger("potionValue"));
        }
    }
    static void animal(World w,EntityAnimal e) throws Exception {
        entity(e);value("age",e.func_40146_g());value("love",(Integer)field(e,"field_39004_a"));value("breedingTicks",(Integer)field(e,"field_39005_b"));value("target",e.entityToAttack==null?-1:e.entityToAttack.entityId);value("hasAttacked",e.hasAttacked?1:0);
        value("color",e instanceof EntitySheep?((EntitySheep)e).getFleeceColor():0);
        value("worldRand",EntityOracle.rngState(w.rand));value("mathRand",EntityOracle.rngState(EntityOracle.mathRandom()));value("nextSeed",OracleHooks.nextSeed());
        value("joined",w.loadedEntityList.size());for(Object obj:w.loadedEntityList){Entity a=(Entity)obj;entity(a);if(a instanceof EntityAnimal){value("childAge",((EntityAnimal)a).func_40146_g());value("childColor",a instanceof EntitySheep?((EntitySheep)a).getFleeceColor():0);}}
        ByteArrayOutputStream bytes=new ByteArrayOutputStream();WorldEventOracle.snapshot(w,new DataOutputStream(bytes));byte[] events=bytes.toByteArray();labels.println(offset+"\t"+events.length+"\tevents");offset+=events.length;out.write(events);
    }
    static void runBreeding() throws Exception {
        for(String name:new String[]{"Cow","Pig","Sheep","Chicken","MushroomCow"})for(int scene=0;scene<8;scene++) {
            World w=Mode_entities10.world();EntityAnimal e=(EntityAnimal)EntityList.createEntityInWorld(name,w);e.setPosition(-3.5,64,4.5);
            Mode_entities10.Player p=new Mode_entities10.Player(w);p.setPosition(-2,65,4);p.inventory.mainInventory[0]=new ItemStack(scene==1?1:296,scene==2?1:3,0);p.capabilities.depleteBuckets=scene==3;w.entityJoinedWorld(p);
            e.func_40142_a_(scene==4?-24000:scene==5?6000:0);value("feed",e.interact(p)?1:0);value("wheat",p.inventory.mainInventory[0]==null?-1:p.inventory.mainInventory[0].stackSize);animal(w,e);
            EntityAnimal mate=(EntityAnimal)EntityList.createEntityInWorld(name,w);mate.setPosition(-2.5,64,4.5);set(mate,"field_39004_a",600);if(mate instanceof EntitySheep){((EntitySheep)mate).setFleeceColor(14);((EntitySheep)e).setFleeceColor(3);}w.entityJoinedWorld(mate);
            if(scene==6){mate.entityToAttack=p;}if(scene==7){mate.func_40142_a_(-20);e.func_40142_a_(100);set(e,"field_39004_a",0);}
            for(int tick=0;tick<65;tick++){e.attackEntity(mate,scene==0&&tick<3?4:2);animal(w,e);}
            NBTTagCompound n=new NBTTagCompound();e.writeEntityToNBT(n);value("nbtAge",n.getInteger("Age"));value("nbtLove",n.getInteger("InLove"));
        }
        for(boolean mushroom:new boolean[]{false,true}) {
            World w=Mode_entities10.world();EntityAnimal e=mushroom?new EntityMooshroom(w):new EntityCow(w);e.setPosition(0,64,0);set(e,"field_39004_a",600);
            EntityAnimal mate=mushroom?new EntityCow(w):new EntityMooshroom(w);mate.setPosition(2,64,2);set(mate,"field_39004_a",600);w.entityJoinedWorld(mate);Entity t=e.findPlayerToAttack();value("mixedTarget",t==null?-1:t.entityId);
        }
    }
    static void worldState(World w) throws Exception {
        value("worldRand",EntityOracle.rngState(w.rand));value("mathRand",EntityOracle.rngState(EntityOracle.mathRandom()));value("nextSeed",OracleHooks.nextSeed());
        value("joined",w.loadedEntityList.size());for(Object obj:w.loadedEntityList){Entity a=(Entity)obj;entity(a);if(a instanceof EntityItem){ItemStack s=((EntityItem)a).item;value("item.id",s.itemID);value("item.count",s.stackSize);value("item.damage",s.getItemDamage());}if(a instanceof EntityFireball){for(String n:new String[]{"accelerationX","accelerationY","accelerationZ"})value(n,(Double)field(a,n));}}
        ByteArrayOutputStream bytes=new ByteArrayOutputStream();WorldEventOracle.snapshot(w,new DataOutputStream(bytes));byte[] events=bytes.toByteArray();labels.println(offset+"\t"+events.length+"\tevents");offset+=events.length;out.write(events);
    }
    static void runDrops(String name) throws Exception {
        for(int seed=0;seed<16;seed++)for(int looting=0;looting<5;looting++)for(int flags=0;flags<4;flags++) {
            World w=Mode_entities10.world();EntityLiving e=(EntityLiving)EntityList.createEntityInWorld(name,w);e.setPosition(-3.5,64,4.5);e.rand.setSeed(seed+1000);if((flags&2)!=0)e.func_40046_d(10);
            entity(e);value("maxHealth",e.getMaxHealth());value("attribute",e.func_40124_t().ordinal());
            e.damageEntity(DamageSource.generic,7);entity(e);e.damageEntity(DamageSource.generic,3);entity(e);
            e.dropFewItems((flags&1)!=0,looting);entity(e);worldState(w);
        }
    }
    static void blaze(World w,EntityBlaze e) throws Exception {
        entity(e);value("hoverOffset",(Float)field(e,"field_40154_a"));value("heightTimer",(Integer)field(e,"field_40153_b"));value("attackStep",(Integer)field(e,"field_40152_d"));value("watcher",(int)e.dataWatcher.getWatchableObjectByte(16));value("hasAttacked",e.hasAttacked?1:0);value("burning",e.isBurning()?1:0);value("brightness",e.getEntityBrightness(0.5F));value("packedLight",e.getEntityBrightnessForRender(0.5F));worldState(w);
    }
    static void runBlaze() throws Exception {
        for(int seed=0;seed<8;seed++)for(int scene=0;scene<5;scene++) {
            World w=Mode_entities10.world();EntityBlaze e=new EntityBlaze(w);e.setPosition(-3.5,68,4.5);e.rand.setSeed(1000+seed);EntityCow t=new EntityCow(w);t.setPosition(scene==1?-3:4,scene==2?79:68,4.5);w.entityJoinedWorld(t);e.entityToAttack=t;
            if(scene==3)w.multiplayerWorld=true;
            if(scene==4){for(int x=-5;x<=-2;x++)for(int y=67;y<=71;y++)for(int z=3;z<=6;z++)Mode_entityround6_10.raw(w,x,y,z,9);}
            for(int tick=0;tick<180;tick++){if(e.attackTime>0)e.attackTime--;e.attackEntity(t,scene==1?1:scene==2?31:8);blaze(w,e);}
            // Autonomous hover, wet damage, smoke and fire RNG, and superclass movement.
            e.entityToAttack=null;for(int tick=0;tick<110;tick++){e.onUpdate();blaze(w,e);}
        }
    }
    static void runEye() throws Exception {
        for(int seed=0;seed<32;seed++)for(int scene=0;scene<5;scene++) {
            World w=Mode_entities10.world();EntityEnderEye e=new EntityEnderEye(w,-3.5,68,4.5);e.rand.setSeed(1000+seed);w.multiplayerWorld=scene==1;
            e.func_40090_a(scene==2?1000:scene==3?-3:2,scene==4?60:77,scene==2?1000:scene==3?4:8);if(scene==1)e.setVelocity(.2,-.1,.3);
            for(int tick=0;tick<82;tick++) {e.onUpdate();entity(e);for(String n:new String[]{"field_40094_b","field_40095_c","field_40091_d"})value(n,(Double)field(e,n));value("lifetime",(Integer)field(e,"field_40092_e"));value("drops",(Boolean)field(e,"field_40093_ao")?1:0);worldState(w);if(e.isDead)break;}
        }
    }
    static void runPlayerXP() throws Exception {
        World w=Mode_entities10.world();Mode_entities10.Player p=new Mode_entities10.Player(w);
        for(int level:new int[]{0,1,2,10,100,1000})for(int amount:new int[]{-7,0,1,3,7,73,10000}) {
            p.currentXP=.375F;p.playerLevel=level;p.totalXP=17;p.score=29;
            value("capBefore",p.xpBarCap());p.increaseXP(amount);value("xp",p.currentXP);value("level",p.playerLevel);value("total",p.totalXP);value("score",p.score);value("capAfter",p.xpBarCap());p.func_40184_i(level+3);value("spentLevel",p.playerLevel);value("spentXP",p.currentXP);
        }
        for(int flags=0;flags<16;flags++) {
            PlayerCapabilities c=new PlayerCapabilities();c.disableDamage=(flags&1)!=0;c.isFlying=(flags&2)!=0;c.allowFlying=(flags&4)!=0;c.depleteBuckets=(flags&8)!=0;
            NBTTagCompound n=new NBTTagCompound();c.func_40601_a(n);NBTTagCompound a=n.getCompoundTag("abilities");for(String key:new String[]{"invulnerable","flying","mayfly","instabuild"})value(key,a.getBoolean(key)?1:0);
            PlayerCapabilities loaded=new PlayerCapabilities();loaded.func_40600_b(n);for(boolean b:new boolean[]{loaded.disableDamage,loaded.isFlying,loaded.allowFlying,loaded.depleteBuckets})value("loaded",b?1:0);
        }
    }
    static void runPotionRegistry() throws Exception {
        for(int id=0;id<32;id++){Potion p=Potion.potionTypes[id];value("present",p==null?0:1);if(p!=null){value("id",p.id);text("name",p.func_40623_c());value("harmful",p.func_40615_f()?1:0);value("color",p.func_40621_j());value("icon",p.func_40611_e());value("hasIcon",p.func_40617_d()?1:0);value("effectiveness",p.func_40610_g());value("usable",p.func_40612_i()?1:0);value("instant",p.func_40622_b()?1:0);}}
        for(int id=1;id<=19;id++)for(int amp:new int[]{-1,0,1,2,31,32})for(int duration:new int[]{-100,-1,0,1,20,1200,123456}){PotionEffect e=new PotionEffect(id,duration,amp);PotionEffect copy=new PotionEffect(e);value("hash",e.hashCode());value("equal",e.equals(copy)?1:0);text("description",e.toString());text("duration",Potion.func_40620_a(e));value("ready",Potion.potionTypes[id].isReady(duration,amp)?1:0);}
        for(String name:new String[]{"Cow","Zombie","Spider"})for(int id:new int[]{6,7})for(int amp=0;amp<4;amp++)for(double strength:new double[]{0,.005,.1,.5,1,1.5})for(boolean hasOwner:new boolean[]{false,true}){
            World w=Mode_entities10.world();EntityLiving e=(EntityLiving)EntityList.createEntityInWorld(name,w);e.setPosition(-3.5,64,4.5);e.health=5;EntityCow owner=new EntityCow(w);owner.setPosition(4,64,4);w.entityJoinedWorld(owner);Potion.potionTypes[id].func_40613_a(hasOwner?owner:null,e,amp,strength);entity(e);worldState(w);
        }
    }
    static void runDamageSources() throws Exception {
        World w=Mode_entities10.world();EntityCow cow=new EntityCow(w);Mode_entities10.Player p=new Mode_entities10.Player(w);EntityArrow a=new EntityArrow(w);EntityFireball b=new EntityFireball(w);
        ArrayList<DamageSource> sources=new ArrayList<DamageSource>(Arrays.asList(DamageSource.inFire,DamageSource.onFire,DamageSource.lava,DamageSource.inWall,DamageSource.drown,DamageSource.starve,DamageSource.cactus,DamageSource.fall,DamageSource.outOfWorld,DamageSource.generic,DamageSource.explosion,DamageSource.magic));
        for(boolean owner:new boolean[]{false,true}){sources.add(DamageSource.causeMobDamage(owner?cow:null));sources.add(DamageSource.causePlayerDamage(owner?p:null));sources.add(DamageSource.causeArrowDamage(a,owner?cow:null));sources.add(DamageSource.causeFireballDamage(b,owner?cow:null));sources.add(DamageSource.causeThrownDamage(a,owner?cow:null));sources.add(DamageSource.func_40542_b(a,owner?cow:null));}
        for(DamageSource s:sources){text("name",s.func_40545_l());value("bypass",s.unblockable()?1:0);value("fire",s.func_40543_k()?1:0);value("projectile",s.func_40547_b()?1:0);value("creative",s.canHarmInCreative()?1:0);value("exhaustion",s.getHungerDamage());value("entity",s.getEntity()==null?-1:s.getEntity().entityId);value("source",s.getSourceOfDamage()==null?-1:s.getSourceOfDamage().entityId);}
    }
    static void runPathValues() throws Exception {
        for(double x:new double[]{Double.NEGATIVE_INFINITY,-2147483649D,-1.25,-1,-0.0,0,.99,2147483648D,Double.POSITIVE_INFINITY,Double.NaN}){
            ChunkPosition p=new ChunkPosition(Vec3D.createVector(x,x+.125,-x));value("x",p.x);value("y",p.y);value("z",p.z);value("hash",p.hashCode());value("equal",p.equals(new ChunkPosition(p.x,p.y,p.z))?1:0);
        }
        World w=Mode_entities10.world();Entity e=new EntityCow(w);
        for(float width:new float[]{-.99F,0,.3F,.9F,1,2.9F,16,Float.NaN,Float.POSITIVE_INFINITY}){e.width=width;PathEntity p=new PathEntity(new PathPoint[]{new PathPoint(-3,64,7),new PathPoint(0,65,-9),new PathPoint(1000,-1,2)});value("length",p.pathLength);for(int n=0;n<3;n++){value("finished",p.isFinished()?1:0);Vec3D v=p.getPosition(e);value("pathX",v.xCoord);value("pathY",v.yCoord);value("pathZ",v.zCoord);p.incrementPathIndex();}value("finished",p.isFinished()?1:0);}
    }
    static void runRegistry() throws Exception {
        Field f=EntityList.class.getDeclaredField("IDtoClassMapping");f.setAccessible(true);Map ids=(Map)f.get(null);f=EntityList.class.getDeclaredField("classToStringMapping");f.setAccessible(true);Map names=(Map)f.get(null);
        for(int id=-10;id<=255;id++){Class c=(Class)ids.get(id);text("class",c==null?null:c.getSimpleName());text("name",c==null?null:(String)names.get(c));}
    }

    static final class SpawnProvider implements IChunkProvider {
        final EntityOracle.FlatProvider base;
        SpawnProvider(EntityOracle.FlatProvider base){this.base=base;}
        public List func_40377_a(EnumCreatureType type,int x,int y,int z){return base.world.getWorldChunkManager().getBiomeGenAtChunkCoord(new ChunkCoordIntPair(x>>4,z>>4)).getSpawnableList(type);}
        public ChunkPosition func_40376_a(World w,String structure,int x,int y,int z){return null;}
        public boolean chunkExists(int x,int z){return base.chunkExists(x,z);}
        public Chunk provideChunk(int x,int z){return base.provideChunk(x,z);}
        public Chunk loadChunk(int x,int z){return base.loadChunk(x,z);}
        public void populate(IChunkProvider p,int x,int z){}
        public boolean saveChunks(boolean all,IProgressUpdate p){return true;}
        public boolean unload100OldestChunks(){return false;}
        public boolean canSave(){return false;}
        public String makeString(){return "entity spawn-list fixture";}
    }
 static void runSpawning(DataOutputStream o)throws Exception {
  for(int scenario=0;scenario<4;scenario++)for(int seed=0;seed<16;seed++){
   World w=Mode_entities10.world();EntityOracle.FlatProvider provider=(EntityOracle.FlatProvider)w.chunkProvider;w.chunkProvider=new SpawnProvider(provider);
   for(int x=-10;x<=10;x++)for(int z=-10;z<=10;z++){
    byte[] b=new byte[32768];for(int bx=0;bx<16;bx++)for(int bz=0;bz<16;bz++)b[(bx<<11)|(bz<<7)|63]=(byte)(scenario==3?1:2);
    Chunk c=new Chunk(w,b,x,z);c.isChunkLoaded=true;c.isTerrainPopulated=true;Arrays.fill(c.heightMap,(byte)64);Arrays.fill(c.skylightMap.data,(byte)(scenario==0?0:255));
    if(scenario==3)for(int bx=0;bx<16;bx++)for(int bz=0;bz<16;bz++)for(int y=64;y<80;y++)c.blocks[(bx<<11)|(bz<<7)|y]=9;
    provider.chunks.put(provider.key(x,z),c);
   }
   java.lang.reflect.Field field=SpawnerAnimals.class.getDeclaredField("eligibleChunksForSpawning");field.setAccessible(true);field.set(null,new Mode_entitybranches10.PinnedSpawnMap());
   w.skylightSubtracted=scenario==0?11:0;
   for(String name:new String[]{"spawnX","spawnY","spawnZ"}){java.lang.reflect.Field sp=WorldInfo.class.getDeclaredField(name);sp.setAccessible(true);sp.setInt(w.worldInfo,name.equals("spawnY")?64:0);}
   w.rand.setSeed(seed);
   Mode_entities10.Player p=new Mode_entities10.Player(w);p.setPosition(.5,64,.5);w.playerEntities.add(p);
   for(int t=0;t<2;t++){
    EntityOracle.i(o,Trace10.namedInt("SpawnerAnimals.performSpawning(w,scenario==0,scenario!=0)", SpawnerAnimals.performSpawning(w,scenario==0,scenario!=0)));EntityOracle.l(o,Trace10.namedLong("EntityOracle.rngState(w.rand)", EntityOracle.rngState(w.rand)));EntityOracle.l(o,Trace10.namedLong("EntityOracle.rngState(EntityOracle.mathRandom())", EntityOracle.rngState(EntityOracle.mathRandom())));EntityOracle.i(o,Trace10.namedInt("w.loadedEntityList.size()", w.loadedEntityList.size()));
    for(Object obj:w.loadedEntityList){Entity e=(Entity)obj;Mode_entities10.state(w,e,o);if(e instanceof EntitySheep)EntityOracle.i(o,Trace10.namedInt("((EntitySheep)e).getFleeceColor()", ((EntitySheep)e).getFleeceColor()));}
   }
  }
 }
    static void runPotionExpiry(String path) throws Exception {
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(path)))) {
            for(boolean remote:new boolean[]{false,true})for(int id:new int[]{1,2,6,7,10,13,17,19})for(int duration:new int[]{-1,0,1,2,25})for(int amplifier:new int[]{0,1,31}) {
                World w=Mode_entities10.world();w.multiplayerWorld=remote;EntityPig e=new EntityPig(w);e.health=7;e.addPotionEffect(new PotionEffect(id,duration,amplifier));
                for(int tick=0;tick<4;tick++){e.updatePotionEffects();Mode_entityround5_10.state(w,e,o);}
            }
        }
    }
    static void runFortressSpawns() throws Exception {
        for(int seed=0;seed<8;seed++){
            World w=Mode_entities10.world();ChunkProviderHell provider=new ChunkProviderHell(w,0);w.chunkProvider=provider;
            StructureNetherBridgeStart start=new StructureNetherBridgeStart(w,new Random(seed),-2,1);provider.field_40378_c.coordMap.put(Long.valueOf(0),start);
            value("pieces",start.func_40560_b().size());
            for(Object component:start.func_40560_b()){
                StructureBoundingBox b=((StructureComponent)component).getBoundingBox();for(int v:new int[]{b.minX,b.minY,b.minZ,b.maxX,b.maxY,b.maxZ})value("piece.bound",v);
                for(int x:new int[]{b.minX-1,b.minX,b.maxX,b.maxX+1})for(int y:new int[]{b.minY-1,b.minY,b.maxY,b.maxY+1}){
                    int z=(b.minZ+b.maxZ)/2;value("contains",provider.field_40378_c.func_40483_a(x,y,z)?1:0);
                    for(EnumCreatureType type:EnumCreatureType.values()){
                        value("list.length",provider.func_40377_a(type,x,y,z).size());
                        for(int draw=0;draw<3;draw++){
                            SpawnListEntry s=w.func_40474_a(type,x,y,z);text("selected",s==null?null:s.entityClass.getSimpleName().substring(6));
                            if(s!=null)for(int v:new int[]{s.field_35590_d,s.field_35591_b,s.field_35592_c})value("entry",v);value("rand",EntityOracle.rngState(w.rand));
                        }
                    }
                }
            }
        }
    }
    static final class PickupPlayer extends EntityPlayer {
        final ArrayList<Integer> stats=new ArrayList<Integer>(),pickups=new ArrayList<Integer>();
        PickupPlayer(World w){super(w);}
        public void func_6420_o(){}
        public void addStat(StatBase s,int count){if(stats!=null){stats.add(s.statId);stats.add(count);}}
        public void onItemPickup(Entity e,int count){pickups.add(e.entityId);pickups.add(count);}
    }
    static void runItemPickup() throws Exception {
        for(int seed=0;seed<8;seed++)for(int item:new int[]{17,334,264,369,370,371,1})for(int scene=0;scene<6;scene++){
            World w=Mode_entities10.world();w.multiplayerWorld=scene==5;PickupPlayer p=new PickupPlayer(w);p.stats.clear();p.capabilities.depleteBuckets=scene==3;
            if(scene==1||scene==2||scene==3)Arrays.fill(p.inventory.mainInventory,new ItemStack(1,64,0));
            if(scene==2)p.inventory.mainInventory[0]=new ItemStack(item,63,0);
            EntityItem e=new EntityItem(w,-3.5,64,4.5,new ItemStack(item,5,0));e.rand.setSeed(seed+1000);e.delayBeforeCanPickup=scene==4?1:0;e.onCollideWithPlayer(p);
            entity(e);value("item.count",e.item.stackSize);
            for(ItemStack s:p.inventory.mainInventory){value("inventory.id",s==null?-1:s.itemID);value("inventory.count",s==null?0:s.stackSize);}
            value("stats.count",p.stats.size()/2);for(int v:p.stats)value("stat",v);value("pickups.count",p.pickups.size()/2);for(int v:p.pickups)value("pickup",v);worldState(w);
        }
    }
    static void runSleep() throws Exception {
        for(int direction=0;direction<4;direction++)for(int scene=0;scene<7;scene++)for(boolean client:new boolean[]{false,true}){
            World w=Mode_entities10.world();w.multiplayerWorld=client;Mode_entities10.Player p=new Mode_entities10.Player(w);p.setLocationAndAngles(.5,64,.5,0,0);EntityOracle.rawBlock(w,0,64,0,26,direction);
            if(scene==1)p.health=0;if(scene==2)p.setPosition(10,64,.5);if(scene==3)w.skylightSubtracted=0;if(scene==4)w.worldProvider.isNether=true;
            if(scene==5||scene==6){EntityLiving mob=scene==5?new EntityZombie(w):new EntityCow(w);mob.setPosition(2,64,2);w.entityJoinedWorld(mob);}
            text("status",p.sleepInBedAt(0,64,0).name());entity(p);value("sleeping",p.isPlayerSleeping()?1:0);worldState(w);
        }
    }
    static void runFallingSand() throws Exception {
        for(int seed=0;seed<8;seed++)for(int scene=0;scene<7;scene++){
            World w=Mode_entities10.world();w.multiplayerWorld=scene==2;double y=scene==3||scene==4?64.5:70.25;EntityFallingSand e=new EntityFallingSand(w,-3.5,y,4.5,scene==6?0:12);e.rand.setSeed(seed+1000);EntityOracle.rawBlock(w,-4,(int)y,4,scene==1||scene==2?1:scene==4?36:scene==5||scene==6?0:12,0);if(scene==4)e.fallTime=2;if(scene==5)e.fallTime=100;
            for(int tick=0;tick<32;tick++){if(tick!=0)e.onUpdate();entity(e);value("tile",e.blockID);value("fallTime",e.fallTime);worldState(w);for(int yy=63;yy<=71;yy++)value("block",w.getBlockId(-4,yy,4));if(e.isDead)break;}
        }
    }
    static void runInventoryCopy() throws Exception {
        World w=Mode_entities10.world();Mode_entities10.Player a=new Mode_entities10.Player(w),b=new Mode_entities10.Player(w);
        for(int c=0;c<64;c++){
            a.inventory=new InventoryPlayer(a);b.inventory=new InventoryPlayer(b);a.inventory.currentItem=c%9;b.inventory.currentItem=8;
            for(int slot=0;slot<40;slot++)if((slot+c)%3!=0){ItemStack s=new ItemStack(slot<36?267:310,c+slot+1,c*7+slot);s.animationsToGo=19;NBTTagCompound n=new NBTTagCompound(),nested=new NBTTagCompound();nested.setInteger("value",c*100+slot);n.setCompoundTag("nested",nested);NBTTagList ench=new NBTTagList();NBTTagCompound entry=new NBTTagCompound();entry.setShort("id",(short)16);entry.setShort("lvl",(short)(c%5+1));ench.setTag(entry);n.setTag("ench",ench);set(s,"field_40715_d",n);if(slot<36)a.inventory.mainInventory[slot]=s;else a.inventory.armorInventory[slot-36]=s;}
            b.inventory.func_41022_a(a.inventory);
            for(int step=0;step<2;step++){
                value("selected",b.inventory.currentItem);
                for(int slot=0;slot<40;slot++){
                    ItemStack source=slot<36?a.inventory.mainInventory[slot]:a.inventory.armorInventory[slot-36],copy=slot<36?b.inventory.mainInventory[slot]:b.inventory.armorInventory[slot-36];value("present",copy==null?0:1);
                    if(copy!=null){NBTTagCompound tag=(NBTTagCompound)field(copy,"field_40715_d");for(int v:new int[]{copy.itemID,copy.stackSize,copy.getItemDamage(),copy.animationsToGo})value("stack",v);value("sameStack",copy==source?1:0);value("sameTag",tag==field(source,"field_40715_d")?1:0);value("nested",tag.getCompoundTag("nested").getInteger("value"));value("level",((NBTTagCompound)tag.getTagList("ench").tagAt(0)).getShort("lvl"));
                        NBTTagCompound original=(NBTTagCompound)field(source,"field_40715_d");original.getCompoundTag("nested").setInteger("value",-c-slot);((NBTTagCompound)original.getTagList("ench").tagAt(0)).setShort("lvl",(short)99);source.stackSize=-1;
                    }
                }
            }
        }
    }
    static final class FoodPlayer extends EntityPlayer {
        int action;
        FoodPlayer(World w){super(w);}
        public void func_6420_o(){}
        public void heal(int amount){action=1;health=Math.min(health+amount,20);}
        public boolean attackEntityFrom(DamageSource source,int amount){action=2;health-=amount;return true;}
    }
    static void runFood() throws Exception {
        World w=Mode_entities10.world();FoodPlayer p=new FoodPlayer(w);
        for(int difficulty=0;difficulty<4;difficulty++)for(int health:new int[]{0,1,10,11,19,20})for(int level:new int[]{0,1,17,18,20})for(float saturation:new float[]{0,.5F,5}){
            FoodStats food=new FoodStats();w.difficultySetting=difficulty;p.health=health;NBTTagCompound n=new NBTTagCompound();n.setInteger("foodLevel",level);n.setFloat("foodSaturationLevel",saturation);n.setFloat("foodExhaustionLevel",40);food.readStatsFromNBT(n);
            for(int tick=0;tick<200;tick++){p.action=0;food.onUpdate(p);n=new NBTTagCompound();food.writeStatsToNBT(n);value("level",food.getFoodLevel());value("previous",food.getPrevFoodLevel());value("timer",n.getInteger("foodTickTimer"));value("saturation",n.getFloat("foodSaturationLevel"));value("exhaustion",n.getFloat("foodExhaustionLevel"));value("health",p.health);value("action",p.action);value("needFood",food.needFood()?1:0);}
        }
    }
    static void runSettings() throws Exception {
        for(long seed:new long[]{0,42,-1,Long.MIN_VALUE,Long.MAX_VALUE})for(int mode:new int[]{0,1,2})for(boolean features:new boolean[]{false,true})for(boolean hardcore:new boolean[]{false,true}){
            WorldSettings s=new WorldSettings(seed,mode,features,hardcore);value("seed",s.getSeed());value("mode",s.getGameType());value("features",s.isMapFeaturesEnabled()?1:0);value("hardcore",s.getHardcoreEnabled()?1:0);
        }
        World w=Mode_entities10.world();Mode_entities10.Player p=new Mode_entities10.Player(w);
        for(int flags=0;flags<16;flags++){
            p.capabilities.disableDamage=(flags&1)!=0;p.capabilities.isFlying=(flags&2)!=0;p.capabilities.allowFlying=(flags&4)!=0;p.capabilities.depleteBuckets=(flags&8)!=0;
            PlayerControllerCreative.func_35646_d(p);for(boolean b:new boolean[]{p.capabilities.disableDamage,p.capabilities.isFlying,p.capabilities.allowFlying,p.capabilities.depleteBuckets})value("creativeFlag",b?1:0);
            PlayerControllerCreative.func_35645_e(p);for(boolean b:new boolean[]{p.capabilities.disableDamage,p.capabilities.isFlying,p.capabilities.allowFlying,p.capabilities.depleteBuckets})value("survivalFlag",b?1:0);
        }
    }
    static void mobState(World w,EntityLiving e) throws Exception {
        entity(e);value("maxHealth",e.getMaxHealth());value("attribute",e.func_40124_t().ordinal());value("moveSpeed",e.moveSpeed);value("landMovement",e.landMovementFactor);value("age",e.entityAge);
        if(e instanceof EntitySlime){EntitySlime s=(EntitySlime)e;value("size",s.getSlimeSize());value("squishTarget",s.field_40139_a);value("squish",s.field_768_a);value("previousSquish",s.field_767_b);}
        if(e instanceof EntityVillager){text("texture",e.getEntityTexture());NBTTagCompound n=new NBTTagCompound();e.writeEntityToNBT(n);value("profession",n.getInteger("Profession"));}
        value("burning",e.isBurning()?1:0);value("brightness",e.getEntityBrightness(1));value("packed",e.getEntityBrightnessForRender(1));worldState(w);
    }
    static void runMob(String name) throws Exception {
        for(int seed=0;seed<8;seed++)for(int scene=0;scene<5;scene++){
            World w=Mode_entities10.world();EntityLiving e=(EntityLiving)EntityList.createEntityInWorld(name,w);e.setPosition(-3.5,64,4.5);e.rand.setSeed(seed+1000);w.multiplayerWorld=scene==1;w.difficultySetting=scene==2?0:2;
            if(e instanceof EntitySlime)((EntitySlime)e).setSlimeSize(new int[]{1,2,4,1,4}[scene]);
            if(e instanceof EntityVillager){NBTTagCompound n=new NBTTagCompound();e.writeToNBT(n);n.setInteger("Profession",scene==4?99:scene);e.readFromNBT(n);}
            if(scene==3){for(int x=-5;x<=-2;x++)for(int z=3;z<=6;z++)EntityOracle.rawBlock(w,x,64,z,9,0);}
            EntityZombie target=new EntityZombie(w);target.setPosition(2,64,8);w.entityJoinedWorld(target);
            mobState(w,e);
            for(int t=0;t<160;t++){
                if(e instanceof EntitySnowman && scene==4){((EntitySnowman)e).attackEntity(target,t%3==0?11:7);}
                if(t==100&&scene==4)e.attackEntityFrom(DamageSource.generic,100);
                e.onUpdate();mobState(w,e);if(e.isDead)break;
            }
        }
    }
    static void runMooshroom() throws Exception {
        for(int seed=0;seed<8;seed++)for(int age:new int[]{-24000,-1,0,6000})for(int item:new int[]{281,359,296,0})for(boolean creative:new boolean[]{false,true}){
            World w=Mode_entities10.world();EntityMooshroom e=new EntityMooshroom(w);e.setLocationAndAngles(-3.5,64,4.5,37,-12);e.health=7;e.renderYawOffset=73;e.func_40142_a_(age);e.rand.setSeed(seed+1000);
            Mode_entities10.Player p=new Mode_entities10.Player(w);p.capabilities.depleteBuckets=creative;if(item!=0)p.inventory.mainInventory[0]=new ItemStack(item,5,0);w.entityJoinedWorld(p);
            value("handled",e.interact(p)?1:0);entity(e);ItemStack held=p.inventory.getCurrentItem();for(int v:new int[]{held==null?-1:held.itemID,held==null?0:held.stackSize,held==null?0:held.getItemDamage()})value("held",v);worldState(w);
        }
    }
    static void blocks(World w) throws Exception {
        for(int x=-12;x<=8;x++)for(int y=56;y<=80;y++)for(int z=-4;z<=16;z++){value("block",w.getBlockId(x,y,z));value("metadata",w.getBlockMetadata(x,y,z));}
    }
    static void runCrystal() throws Exception {
        for(int seed=0;seed<4;seed++)for(boolean client:new boolean[]{false,true}){
            World w=Mode_entities10.world();w.multiplayerWorld=client;EntityEnderCrystal e=new EntityEnderCrystal(w,-3.5,64,4.5);
            for(int tick=0;tick<25;tick++){
                if(tick==21||tick==23)value("attack",e.attackEntityFrom(DamageSource.magic,-1)?1:0);
                if(tick!=0)e.onUpdate();entity(e);value("rotation",e.field_41032_a);value("health",e.field_41031_b);value("watcher",e.dataWatcher.getWatchableObjectInt(8));worldState(w);
            }
            blocks(w);
        }
    }
    static void fireballState(World w,EntityFireball e) throws Exception {
        entity(e);for(String n:new String[]{"xTile","yTile","zTile","inTile","ticksAlive","ticksInAir"})value(n,(Integer)field(e,n));value("inGround",(Boolean)field(e,"inGround")?1:0);value("owner",e.shootingEntity==null?-1:e.shootingEntity.entityId);
        for(double v:new double[]{e.accelerationX,e.accelerationY,e.accelerationZ})value("acceleration",v);value("collision",e.canBeCollidedWith()?1:0);value("brightness",e.getEntityBrightness(1));value("packed",e.getEntityBrightnessForRender(1));worldState(w);
    }
    static void runFireball(boolean small) throws Exception {
        for(int seed=0;seed<8;seed++)for(int scene=0;scene<12;scene++){
            World w=Mode_entities10.world();EntityCow owner=new EntityCow(w);owner.setLocationAndAngles(-3.5,68,4.5,27,-13);w.entityJoinedWorld(owner);
            EntityFireball e=small?new EntitySmallFireball(w,owner,.4,.2,1):new EntityFireball(w,owner,.4,.2,1);e.rand.setSeed(seed+1000);w.multiplayerWorld=scene==1;fireballState(w,e);
            if(scene>=2&&scene<8){EntityOracle.rawBlock(w,-3,67,4,1,0);e.func_40071_a(new MovingObjectPosition(-3,67,4,scene-2,Vec3D.createVector(-3,67,4)));}
            else if(scene==8||scene==9){EntityLiving target=scene==8?new EntityCow(w):new EntityBlaze(w);target.setPosition(-2,68,5);w.entityJoinedWorld(target);e.func_40071_a(new MovingObjectPosition(target));}
            else if(scene==10){owner.setEntityDead();}
            else if(scene==11){value("deflect",e.attackEntityFrom(DamageSource.causeMobDamage(owner),1)?1:0);}
            for(int tick=0;tick<24;tick++){e.onUpdate();fireballState(w,e);if(e.isDead)break;}
            if(scene>=2&&scene<10)blocks(w);
        }
    }
    public static void main(String[] args) throws Exception {
        try {
            Trace10.outputFile=new File(args[1]).getCanonicalPath();
            if(args[0].equals("branches-spawnbranches")){try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(args[1])))){runSpawning(o);}System.exit(0);}
            if(args[0].equals("round5-potionexpiry5")){runPotionExpiry(args[1]);System.exit(0);}
            if(args[0].startsWith("branches-")){Mode_entitybranches10.main(new String[]{"entitybranches10",args[0].substring(9),"0",args[1]});System.exit(0);}
            if(args[0].startsWith("round4-")){Mode_entityround4_10.main(new String[]{"entityround4_10",args[0].substring(7),"0",args[1]});System.exit(0);}
            if(args[0].startsWith("round5-")){Mode_entityround5_10.main(new String[]{"entityround5_10",args[0].substring(7),"0",args[1]});System.exit(0);}
            if(args[0].startsWith("player-")){Mode_playertick10.main(new String[]{"playertick10",args[0].substring(7),"240",args[1]});System.exit(0);}
            if(args[0].equals("playersp")){Mode_playersp10.main(new String[]{"playersp10",args[1]});System.exit(0);}
            if(args[0].equals("explosion")){Mode_explosion10.main(new String[]{"explosion10","","0",args[1]});System.exit(0);}
            if(args[0].equals("inventory")){PlayerOracle.inventory(args[1]);System.exit(0);}
            if(args[0].equals("cartinventory")){VehicleOracle.inventory(args[1]);System.exit(0);}
            if(args[0].startsWith("vehicle-")){VehicleOracle.replay(args[0].substring(8),240,args[1]);System.exit(0);}
            if(args[0].startsWith("api-")){Mode_entityapi10.main(new String[]{"entityapi10",args[0].substring(4),"0",args[1]});System.exit(0);}
            if(args[0].equals("living6")||args[0].equals("livingdamage6")||args[0].equals("livingnbt6")) {
                Mode_entityround6_10.main(new String[]{"entityround6_10",args[0],"0",args[1]});System.exit(0);
            }
            if(args[0].startsWith("replay-")||args[0].equals("rules")) {
                Mode_entities10.main(new String[]{"entities10",args[0].equals("rules")?"rules":args[0].substring(7),args[0].equals("replay-Egg")||args[0].equals("replay-Snowball")?"24":"240",args[1]});System.exit(0);
            }
            out=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(args[1])));
            labels=new PrintWriter(args[1]+".fields","UTF-8");
            if(args[0].equals("fortressspawns"))runFortressSpawns();else if(args[0].equals("itempickup"))runItemPickup();else if(args[0].equals("sleep"))runSleep();else if(args[0].equals("fallingsand"))runFallingSand();else if(args[0].equals("inventorycopy"))runInventoryCopy();else if(args[0].equals("food"))runFood();else if(args[0].equals("settings"))runSettings();else if(args[0].equals("crystal"))runCrystal();else if(args[0].equals("smallfireball")||args[0].equals("fireball"))runFireball(args[0].equals("smallfireball"));else if(args[0].startsWith("mob-"))runMob(args[0].substring(4));else if(args[0].equals("mooshroom"))runMooshroom();else if(args[0].equals("potionregistry"))runPotionRegistry();else if(args[0].equals("damagesources"))runDamageSources();else if(args[0].equals("pathvalues"))runPathValues();else if(args[0].equals("registry10"))runRegistry();else if(args[0].equals("playerxp"))runPlayerXP();else if(args[0].equals("endereye"))runEye();else if(args[0].startsWith("drops-"))runDrops(args[0].substring(6));else if(args[0].equals("blaze"))runBlaze();else if(args[0].equals("breeding"))runBreeding();else if(args[0].equals("fishhook"))runFish();else if(args[0].equals("egg")||args[0].equals("snowball")||args[0].equals("pearl")||args[0].equals("splashpotion"))runThrowable(args[0]);else runDragon(args[0]);out.close();labels.close();System.exit(0);
        } catch(Throwable ex) {ex.printStackTrace();System.exit(1);}
    }
}
