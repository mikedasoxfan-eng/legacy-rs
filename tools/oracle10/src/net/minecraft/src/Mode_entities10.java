package net.minecraft.src;
import java.io.*;
import java.util.*;
import java.lang.reflect.*;
/** Focused 1.0 entity replay; every call executes the original bytecode. */
public final class Mode_entities10 {
    static final class Player extends EntityPlayer {Player(World w){super(w);} public void func_6420_o(){}}
    static void i(DataOutputStream o,int v)throws Exception{EntityOracle.i(o,v);}
    static void l(DataOutputStream o,long v)throws Exception{EntityOracle.l(o,v);}
    static void f(DataOutputStream o,float v)throws Exception{EntityOracle.f(o,v);}
    static void d(DataOutputStream o,double v)throws Exception{EntityOracle.d(o,v);}
    static int get(Class<?> c,String n,Object e)throws Exception{return EntityOracle.intField(c,n,e);}
    static World world() throws Exception {
        World w=Oracle.newWorld(0);w.chunkProvider=new EntityOracle.FlatProvider(w);
        for(Object c0:((EntityOracle.FlatProvider)w.chunkProvider).chunks.values()) {Chunk c=(Chunk)c0;Arrays.fill(c.skylightMap.data,(byte)255);Arrays.fill(c.heightMap,(byte)64);}
        w.difficultySetting=2;w.skylightSubtracted=11;w.worldInfo.setWorldTime(18000);
        WorldEventOracle.attach(w);
        // WorldEventOracle now records playAuxSFX itself; keep one recording listener.
        Field id=Entity.class.getDeclaredField("nextEntityID");id.setAccessible(true);id.setInt(null,0);
        OracleHooks.reset(100);EntityOracle.mathRandom().setSeed(200);w.rand.setSeed(300);
        return w;
    }
    static void state(World w,Entity e,DataOutputStream o)throws Exception {
        for(double v:new double[]{Trace10.namedDouble("e.posX", e.posX),Trace10.namedDouble("e.posY", e.posY),Trace10.namedDouble("e.posZ", e.posZ),Trace10.namedDouble("e.motionX", e.motionX),Trace10.namedDouble("e.motionY", e.motionY),Trace10.namedDouble("e.motionZ", e.motionZ),Trace10.namedDouble("e.prevPosX", e.prevPosX),Trace10.namedDouble("e.prevPosY", e.prevPosY),Trace10.namedDouble("e.prevPosZ", e.prevPosZ),Trace10.namedDouble("e.boundingBox.minX", e.boundingBox.minX),Trace10.namedDouble("e.boundingBox.minY", e.boundingBox.minY),Trace10.namedDouble("e.boundingBox.minZ", e.boundingBox.minZ),Trace10.namedDouble("e.boundingBox.maxX", e.boundingBox.maxX),Trace10.namedDouble("e.boundingBox.maxY", e.boundingBox.maxY),Trace10.namedDouble("e.boundingBox.maxZ", e.boundingBox.maxZ)})d(o,v);
        for(float v:new float[]{Trace10.namedFloat("e.rotationYaw", e.rotationYaw),Trace10.namedFloat("e.rotationPitch", e.rotationPitch),Trace10.namedFloat("e.prevRotationYaw", e.prevRotationYaw),Trace10.namedFloat("e.prevRotationPitch", e.prevRotationPitch),Trace10.namedFloat("e.width", e.width),Trace10.namedFloat("e.height", e.height),Trace10.namedFloat("e.yOffset", e.yOffset),Trace10.namedFloat("e.ySize", e.ySize),Trace10.namedFloat("e.stepHeight", e.stepHeight),Trace10.namedFloat("e.fallDistance", e.fallDistance),Trace10.namedFloat("e.distanceWalkedModified", e.distanceWalkedModified)})f(o,v);
        for(int v:new int[]{Trace10.namedInt("e.ticksExisted", e.ticksExisted),Trace10.namedInt("Harness10.getInt(e,\"fire\")", Harness10.getInt(e,"fire")),Trace10.namedInt("e.func_41001_Z()", e.func_41001_Z())})i(o,v);
        Trace10.writeBoolean(o, "e.onGround", e.onGround);Trace10.writeBoolean(o, "e.isDead", e.isDead);Trace10.writeBoolean(o, "e.inWater", e.inWater);Trace10.writeBoolean(o, "e.beenAttacked", e.beenAttacked);Trace10.writeBoolean(o, "e.isAirBorne", e.isAirBorne);
        if(e instanceof EntityLiving) {
            EntityLiving a=(EntityLiving)e;
            for(int v:new int[]{Trace10.namedInt("a.health", a.health),Trace10.namedInt("a.prevHealth", a.prevHealth),Trace10.namedInt("a.heartsLife", a.heartsLife),Trace10.namedInt("a.hurtTime", a.hurtTime),Trace10.namedInt("a.maxHurtTime", a.maxHurtTime),Trace10.namedInt("a.deathTime", a.deathTime),Trace10.namedInt("a.attackTime", a.attackTime),Trace10.namedInt("a.entityAge", a.entityAge),Trace10.namedInt("a.naturalArmorRating", a.naturalArmorRating),Trace10.namedInt("a.field_35171_bJ", a.field_35171_bJ),Trace10.namedInt("a.field_35172_bP", a.field_35172_bP),Trace10.namedInt("a.field_35173_bQ", a.field_35173_bQ)})i(o,v);
            for(float v:new float[]{Trace10.namedFloat("a.attackedAtYaw", a.attackedAtYaw),Trace10.namedFloat("a.renderYawOffset", a.renderYawOffset),Trace10.namedFloat("a.prevRenderYawOffset", a.prevRenderYawOffset),Trace10.namedFloat("a.field_705_Q", a.field_705_Q),Trace10.namedFloat("a.field_704_R", a.field_704_R),Trace10.namedFloat("a.field_703_S", a.field_703_S),Trace10.namedFloat("a.moveStrafing", a.moveStrafing),Trace10.namedFloat("a.moveForward", a.moveForward),Trace10.namedFloat("a.randomYawVelocity", a.randomYawVelocity),Trace10.namedFloat("a.moveSpeed", a.moveSpeed),Trace10.namedFloat("a.landMovementFactor", a.landMovementFactor),Trace10.namedFloat("a.jumpMovementFactor", a.jumpMovementFactor)})f(o,v);
            Trace10.writeBoolean(o, "a.isJumping", a.isJumping);
            ArrayList<Integer> ids=new ArrayList<Integer>(a.activePotionsMap.keySet());Collections.sort(ids);i(o,Trace10.namedInt("ids.size()", ids.size()));
            for(int id:ids){PotionEffect p=(PotionEffect)a.activePotionsMap.get(id);i(o,id);i(o,Trace10.namedInt("p.getDuration()", p.getDuration()));i(o,Trace10.namedInt("p.getAmplifier()", p.getAmplifier()));}
            if(a instanceof EntityCreature) {EntityCreature c=(EntityCreature)a;i(o,Trace10.namedInt("c.fleeingTick", c.fleeingTick));i(o,Trace10.namedInt("c.entityToAttack==null?-1:c.entityToAttack.entityId", c.entityToAttack==null?-1:c.entityToAttack.entityId));}
        }
        if(e instanceof EntityEnderman){EntityEnderman a=(EntityEnderman)e;i(o,Trace10.namedInt("a.getCarried()", a.getCarried()));i(o,Trace10.namedInt("a.getCarryingData()", a.getCarryingData()));Trace10.writeBoolean(o, "a.isAttacking", a.isAttacking);i(o,Trace10.namedInt("get(EntityEnderman.class,\"teleportDelay\",a)", get(EntityEnderman.class,"teleportDelay",a)));i(o,Trace10.namedInt("get(EntityEnderman.class,\"field_35185_e\",a)", get(EntityEnderman.class,"field_35185_e",a)));}
        if(e instanceof EntitySilverfish)i(o,Trace10.namedInt("get(EntitySilverfish.class,\"field_35189_a\",e)", get(EntitySilverfish.class,"field_35189_a",e)));
        if(e instanceof EntityXPOrb){EntityXPOrb a=(EntityXPOrb)e;for(int v:new int[]{Trace10.namedInt("a.xpColor", a.xpColor),Trace10.namedInt("a.xpOrbAge", a.xpOrbAge),Trace10.namedInt("a.unusedPickupCooldown", a.unusedPickupCooldown),Trace10.namedInt("get(EntityXPOrb.class,\"xpOrbHealth\",a)", get(EntityXPOrb.class,"xpOrbHealth",a)),Trace10.namedInt("a.getXpValue()", a.getXpValue()),Trace10.namedInt("a.getTextureByXP()", a.getTextureByXP())})i(o,v);f(o,Trace10.namedFloat("Harness10.removedOrbPhase(a)", Harness10.removedOrbPhase(a)));}
        if(e instanceof EntityArrow){EntityArrow a=(EntityArrow)e;Trace10.writeBoolean(o, "a.arrowCritical", a.arrowCritical);i(o,Trace10.namedInt("get(EntityArrow.class,\"ticksInGround\",a)", get(EntityArrow.class,"ticksInGround",a)));i(o,Trace10.namedInt("get(EntityArrow.class,\"ticksInAir\",a)", get(EntityArrow.class,"ticksInAir",a)));}
        l(o,Trace10.namedLong("EntityOracle.rngState(e.rand)", EntityOracle.rngState(e.rand)));l(o,Trace10.namedLong("EntityOracle.rngState(w.rand)", EntityOracle.rngState(w.rand)));l(o,Trace10.namedLong("EntityOracle.rngState(EntityOracle.mathRandom())", EntityOracle.rngState(EntityOracle.mathRandom())));
        i(o,Trace10.namedInt("w.loadedEntityList.size()", w.loadedEntityList.size()));
        for(Object a0:w.loadedEntityList){Entity a=(Entity)a0;WorldEventOracle.text(o,EntityList.getEntityString(a));d(o,Trace10.namedDouble("a.posX", a.posX));d(o,Trace10.namedDouble("a.posY", a.posY));d(o,Trace10.namedDouble("a.posZ", a.posZ));if(a instanceof EntityXPOrb)i(o,Trace10.namedInt("((EntityXPOrb)a).getXpValue()", ((EntityXPOrb)a).getXpValue()));}
        WorldEventOracle.snapshot(w,o);
    }
    static void rules(DataOutputStream o)throws Exception {
        DamageSource[] sources={DamageSource.inFire,DamageSource.onFire,DamageSource.lava,DamageSource.inWall,DamageSource.drown,DamageSource.starve,DamageSource.cactus,DamageSource.fall,DamageSource.outOfWorld,DamageSource.generic,DamageSource.explosion,DamageSource.magic,new EntityDamageSource("mob",null),new EntityDamageSourceIndirect("arrow",null,null)};
        for(DamageSource s:sources){WorldEventOracle.text(o,s.damageType);Trace10.writeBoolean(o, "s.unblockable()", s.unblockable());Trace10.writeBoolean(o, "s.canHarmInCreative()", s.canHarmInCreative());f(o,Trace10.namedFloat("s.getHungerDamage()", s.getHungerDamage()));}
        for(int id=1;id<=19;id++)for(int amp=-1;amp<36;amp++)for(int ticks:new int[]{-1,0,1,2,7,12,24,25,50,100,999})Trace10.writeBoolean(o, "Potion.potionTypes[id].isReady(ticks,amp)", Potion.potionTypes[id].isReady(ticks,amp));
        for(int amp=0;amp<5;amp++)for(int newer=-1;newer<6;newer++) {PotionEffect p=new PotionEffect(19,200,amp);p.combine(new PotionEffect(19,7,newer));i(o,Trace10.namedInt("p.getPotionID()", p.getPotionID()));i(o,Trace10.namedInt("p.getDuration()", p.getDuration()));i(o,Trace10.namedInt("p.getAmplifier()", p.getAmplifier()));}
        World w=world();EntityEnderman e=new EntityEnderman(w);
        for(int id=0;id<256;id++){Field field=EntityEnderman.class.getDeclaredField("canCarryBlocks");field.setAccessible(true);Trace10.writeBoolean(o, "((boolean[])field.get(null))[id]", ((boolean[])field.get(null))[id]);}
        for(int v=-2;v<10000;v++){EntityXPOrb orb=new EntityXPOrb(w,0,0,0,v);i(o,Trace10.namedInt("EntityXPOrb.getXPSplit(v)", EntityXPOrb.getXPSplit(v)));i(o,Trace10.namedInt("orb.getTextureByXP()", orb.getTextureByXP()));}
        e.setCarried(35);e.setCarryingData(7);NBTTagCompound n=new NBTTagCompound();e.writeEntityToNBT(n);i(o,Trace10.namedInt("n.getShort(\"carried\")", n.getShort("carried")));i(o,Trace10.namedInt("n.getShort(\"carriedData\")", n.getShort("carriedData")));e.readEntityFromNBT(n);i(o,Trace10.namedInt("e.getCarried()", e.getCarried()));i(o,Trace10.namedInt("e.getCarryingData()", e.getCarryingData()));
        for(int bi=0;bi<10;bi++)for(EnumCreatureType cat:EnumCreatureType.values()){List list=BiomeGenBase.biomeList[bi].getSpawnableList(cat);i(o,Trace10.namedInt("list.size()", list.size()));for(Object a:list){SpawnListEntry s=(SpawnListEntry)a;WorldEventOracle.text(o,s.entityClass.getSimpleName().substring(6));i(o,Trace10.namedInt("s.field_35590_d", s.field_35590_d));i(o,Trace10.namedInt("s.field_35591_b", s.field_35591_b));i(o,Trace10.namedInt("s.field_35592_c", s.field_35592_c));}}
        w=world();Player player=new Player(w);
        for(int value:new int[]{1,3,7,17,37,73,149,307,617,1237,2477,10000,-2})for(int delay:new int[]{0,1})for(int cooldown:new int[]{0,1}) {
            EntityXPOrb orb=new EntityXPOrb(w,-3.5,68,4.5,value);orb.unusedPickupCooldown=delay;player.xpCooldown=cooldown;orb.onCollideWithPlayer(player);
            for(int v:new int[]{Trace10.namedInt("Float.floatToIntBits(player.currentXP)", Float.floatToIntBits(player.currentXP)),Trace10.namedInt("player.playerLevel", player.playerLevel),Trace10.namedInt("player.totalXP", player.totalXP),Trace10.namedInt("player.xpCooldown", player.xpCooldown)})i(o,v);Trace10.writeBoolean(o, "orb.isDead", orb.isDead);l(o,Trace10.namedLong("EntityOracle.rngState(orb.rand)", EntityOracle.rngState(orb.rand)));WorldEventOracle.snapshot(w,o);
        }

    }
    static void population(DataOutputStream o)throws Exception {
        for(int biome=0;biome<10;biome++)for(long seed=4096;seed<4128;seed++) {
            World w=world();Random random=new Random(seed);SpawnerAnimals.func_35957_a(w,BiomeGenBase.biomeList[biome],-8,-7,16,12,random);
            l(o,Trace10.namedLong("EntityOracle.rngState(random)", EntityOracle.rngState(random)));l(o,Trace10.namedLong("EntityOracle.rngState(w.rand)", EntityOracle.rngState(w.rand)));l(o,Trace10.namedLong("EntityOracle.rngState(EntityOracle.mathRandom())", EntityOracle.rngState(EntityOracle.mathRandom())));i(o,Trace10.namedInt("w.loadedEntityList.size()", w.loadedEntityList.size()));
            for(Object obj:w.loadedEntityList){Entity e=(Entity)obj;WorldEventOracle.text(o,EntityList.getEntityString(e));d(o,Trace10.namedDouble("e.posX", e.posX));d(o,Trace10.namedDouble("e.posY", e.posY));d(o,Trace10.namedDouble("e.posZ", e.posZ));f(o,Trace10.namedFloat("e.rotationYaw", e.rotationYaw));l(o,Trace10.namedLong("EntityOracle.rngState(e.rand)", EntityOracle.rngState(e.rand)));if(e instanceof EntitySheep)i(o,Trace10.namedInt("((EntitySheep)e).getFleeceColor()", ((EntitySheep)e).getFleeceColor()));}
        }
    }
    public static void main(String[] a)throws Exception {
        String kind=a[1];int ticks=Integer.parseInt(a[2]);
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[3])))) {
            if(kind.equals("rules")){rules(o);return;}if(kind.equals("population")){population(o);return;}
            World w=world();
            Entity e;
            if(kind.equals("XPOrb")||kind.equals("attract"))e=new EntityXPOrb(w,-3.5,68,4.5,149);
            else if(kind.equals("Boat"))e=new EntityBoat(w,-3.5,68,4.5);
            else if(kind.equals("Minecart"))e=new EntityMinecart(w,-3.5,68,4.5,0);
            else if(kind.equals("Item"))e=new EntityItem(w,-3.5,68,4.5,new ItemStack(263,1,0));
            else if(kind.equals("Fireball"))e=new EntityFireball(w,-3.5,68,4.5,.4,.2,1);
            else if(kind.equals("Snowball")){EntitySnowball a1=new EntitySnowball(w,-3.5,68,4.5);a1.setThrowableHeading(.4,.2,1,1.5F,1);e=a1;}
            else if(kind.equals("Egg")){EntityEgg a1=new EntityEgg(w,-3.5,68,4.5);a1.setThrowableHeading(.4,.2,1,1.5F,1);e=a1;}
            else if(kind.equals("Lightning"))e=new EntityLightningBolt(w,-3.5,64,4.5);
            else if(kind.equals("Arrow")){EntityArrow arrow=new EntityArrow(w,-3.5,68,4.5);arrow.setArrowHeading(0.4,0.2,1,1.5F,1);arrow.arrowCritical=true;e=arrow;}
            else {String name=kind.startsWith("kill-")?kind.substring(5):kind.equals("poison")?"Zombie":kind.equals("teleport")||kind.equals("carry")||kind.equals("indirect")||kind.startsWith("stare")?"Enderman":kind.equals("allies")?"Silverfish":kind.equals("potions")||kind.equals("damage")?"Pig":kind;e=EntityList.createEntityInWorld(name,w);e.setLocationAndAngles(-3.5,64,4.5,35,0);}
            if(kind.equals("carry"))for(int x=-8;x<=8;x++)for(int z=-8;z<=8;z++)EntityOracle.rawBlock(w,x,64,z,37,0);
            if(kind.equals("allies"))for(int x=-7;x<=7;x++)for(int z=-7;z<=7;z++)EntityOracle.rawBlock(w,x,63,z,97,0);
            if(kind.equals("potions")){EntityLiving mob=(EntityLiving)e;mob.health=8;for(int[] p:new int[][]{{1,80,1},{2,120,0},{10,150,0},{19,180,0},{6,3,0},{7,2,0}})mob.addPotionEffect(new PotionEffect(p[0],p[1],p[2]));}
            EntityPlayer killer=null;
            if(kind.startsWith("kill-")||kind.equals("attract")||kind.startsWith("stare")){killer=new Player(w);killer.entityId=10000;killer.setPosition(0.5,65.62,0.5);if(kind.startsWith("stare")){killer.rotationYaw=45;killer.rotationPitch=0.5F;if(kind.equals("starepumpkin"))killer.inventory.armorInventory[3]=new ItemStack(86,1,0);}w.playerEntities.add(killer);}
            if(kind.equals("poison")){EntityCaveSpider spider=new EntityCaveSpider(w);spider.setPosition(-3.5,64,3.5);spider.attackEntityAsMob(e);}
            if(kind.equals("allies")){EntityCaveSpider spider=new EntityCaveSpider(w);spider.setPosition(-3.5,64,3.5);e.attackEntityFrom(DamageSource.causeMobDamage(spider),2);}
            state(w,e,o);
            for(int t=0;t<ticks;t++) {
                if(kind.equals("indirect")&&t==0)Trace10.writeBoolean(o, "e.attackEntityFrom(DamageSource.causeArrowDamage(new EntityArrow(w),null),4)", e.attackEntityFrom(DamageSource.causeArrowDamage(new EntityArrow(w),null),4));
                if(kind.startsWith("kill-") && t==0)e.attackEntityFrom(DamageSource.causePlayerDamage(killer),1000);
                if(kind.equals("teleport") && t%25==0)Trace10.writeBoolean(o, "((EntityEnderman)e).teleportTo(t==25?1000:-4.5,70.75,5.5)", ((EntityEnderman)e).teleportTo(t==25?1000:-4.5,70.75,5.5));
                if(kind.equals("damage") && t%40==0)e.attackEntityFrom(DamageSource.generic,3);
                e.onUpdate();state(w,e,o);
            }
        }
    }
}
