package net.minecraft.src;
import java.io.*;
import java.util.*;
import java.lang.reflect.*;
/** Focused b1.8.1 entity replay; every call executes the original bytecode. */
public final class Mode_entities181 {
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
        for(double v:new double[]{e.posX,e.posY,e.posZ,e.motionX,e.motionY,e.motionZ,e.prevPosX,e.prevPosY,e.prevPosZ,e.boundingBox.minX,e.boundingBox.minY,e.boundingBox.minZ,e.boundingBox.maxX,e.boundingBox.maxY,e.boundingBox.maxZ})d(o,v);
        for(float v:new float[]{e.rotationYaw,e.rotationPitch,e.prevRotationYaw,e.prevRotationPitch,e.width,e.height,e.yOffset,e.ySize,e.stepHeight,e.fallDistance,e.distanceWalkedModified})f(o,v);
        for(int v:new int[]{e.ticksExisted,e.fire,e.air})i(o,v);
        o.writeBoolean(e.onGround);o.writeBoolean(e.isDead);o.writeBoolean(e.inWater);o.writeBoolean(e.beenAttacked);o.writeBoolean(e.field_35118_ao);
        if(e instanceof EntityLiving) {
            EntityLiving a=(EntityLiving)e;
            for(int v:new int[]{a.health,a.prevHealth,a.heartsLife,a.hurtTime,a.maxHurtTime,a.deathTime,a.attackTime,a.entityAge,a.field_9346_af,a.field_35171_bJ,a.field_35172_bP,a.field_35173_bQ})i(o,v);
            for(float v:new float[]{a.attackedAtYaw,a.renderYawOffset,a.prevRenderYawOffset,a.field_705_Q,a.field_704_R,a.field_703_S,a.moveStrafing,a.moveForward,a.randomYawVelocity,a.moveSpeed,a.field_35169_bv,a.field_35168_bw})f(o,v);
            o.writeBoolean(a.isJumping);
            ArrayList<Integer> ids=new ArrayList<Integer>(a.field_35170_bR.keySet());Collections.sort(ids);i(o,ids.size());
            for(int id:ids){PotionEffect p=(PotionEffect)a.field_35170_bR.get(id);i(o,id);i(o,p.func_35802_b());i(o,p.func_35801_c());}
            if(a instanceof EntityCreature) {EntityCreature c=(EntityCreature)a;i(o,c.field_35174_at);i(o,c.entityToAttack==null?-1:c.entityToAttack.entityId);}
        }
        if(e instanceof EntityEnderman){EntityEnderman a=(EntityEnderman)e;i(o,a.func_35176_r());i(o,a.func_35180_s());o.writeBoolean(a.field_35187_a);i(o,get(EntityEnderman.class,"field_35184_d",a));i(o,get(EntityEnderman.class,"field_35185_e",a));}
        if(e instanceof EntitySilverfish)i(o,get(EntitySilverfish.class,"field_35189_a",e));
        if(e instanceof EntityXPOrb){EntityXPOrb a=(EntityXPOrb)e;for(int v:new int[]{a.field_35127_a,a.field_35124_b,a.field_35126_c,get(EntityXPOrb.class,"field_35123_e",a),a.func_35119_j_(),a.func_35120_i()})i(o,v);f(o,a.field_35122_d);}
        if(e instanceof EntityArrow){EntityArrow a=(EntityArrow)e;o.writeBoolean(a.field_35140_d);i(o,get(EntityArrow.class,"ticksInGround",a));i(o,get(EntityArrow.class,"ticksInAir",a));}
        l(o,EntityOracle.rngState(e.rand));l(o,EntityOracle.rngState(w.rand));l(o,EntityOracle.rngState(EntityOracle.mathRandom()));
        i(o,w.loadedEntityList.size());
        for(Object a0:w.loadedEntityList){Entity a=(Entity)a0;WorldEventOracle.text(o,EntityList.getEntityString(a));d(o,a.posX);d(o,a.posY);d(o,a.posZ);if(a instanceof EntityXPOrb)i(o,((EntityXPOrb)a).func_35119_j_());}
        WorldEventOracle.snapshot(w,o);
    }
    static void rules(DataOutputStream o)throws Exception {
        DamageSource[] sources={DamageSource.field_35542_a,DamageSource.field_35540_b,DamageSource.field_35541_c,DamageSource.field_35538_d,DamageSource.field_35539_e,DamageSource.field_35536_f,DamageSource.field_35537_g,DamageSource.field_35549_h,DamageSource.field_35550_i,DamageSource.field_35547_j,DamageSource.field_35548_k,DamageSource.field_35545_l,new EntityDamageSource("mob",null),new EntityDamageSourceIndirect("arrow",null,null)};
        for(DamageSource s:sources){WorldEventOracle.text(o,s.field_35546_m);o.writeBoolean(s.func_35534_b());o.writeBoolean(s.func_35529_d());f(o,s.func_35533_c());}
        for(int id=1;id<=19;id++)for(int amp=-1;amp<36;amp++)for(int ticks:new int[]{-1,0,1,2,7,12,24,25,50,100,999})o.writeBoolean(Potion.field_35678_a[id].func_35660_a(ticks,amp));
        for(int amp=0;amp<5;amp++)for(int newer=-1;newer<6;newer++) {PotionEffect p=new PotionEffect(19,200,amp);p.func_35796_a(new PotionEffect(19,7,newer));i(o,p.func_35799_a());i(o,p.func_35802_b());i(o,p.func_35801_c());}
        World w=world();EntityEnderman e=new EntityEnderman(w);
        for(int id=0;id<256;id++){Field field=EntityEnderman.class.getDeclaredField("field_35186_b");field.setAccessible(true);o.writeBoolean(((boolean[])field.get(null))[id]);}
        for(int v=-2;v<10000;v++){EntityXPOrb orb=new EntityXPOrb(w,0,0,0,v);i(o,EntityXPOrb.func_35121_b(v));i(o,orb.func_35120_i());}
        e.func_35177_b(35);e.func_35181_c(7);NBTTagCompound n=new NBTTagCompound();e.writeEntityToNBT(n);i(o,n.getShort("carried"));i(o,n.getShort("carriedData"));e.readEntityFromNBT(n);i(o,e.func_35176_r());i(o,e.func_35180_s());
        for(int bi=0;bi<10;bi++)for(EnumCreatureType cat:EnumCreatureType.values()){List list=BiomeGenBase.field_35486_a[bi].getSpawnableList(cat);i(o,list.size());for(Object a:list){SpawnListEntry s=(SpawnListEntry)a;WorldEventOracle.text(o,s.entityClass.getSimpleName().substring(6));i(o,s.field_35590_d);i(o,s.field_35591_b);i(o,s.field_35592_c);}}
        w=world();Player player=new Player(w);
        for(int value:new int[]{1,3,7,17,37,73,149,307,617,1237,2477,10000,-2})for(int delay:new int[]{0,1})for(int cooldown:new int[]{0,1}) {
            EntityXPOrb orb=new EntityXPOrb(w,-3.5,68,4.5,value);orb.field_35126_c=delay;player.field_35214_aG=cooldown;orb.onCollideWithPlayer(player);
            for(int v:new int[]{player.field_35211_aX,player.field_35210_aY,player.field_35209_aZ,player.field_35214_aG})i(o,v);o.writeBoolean(orb.isDead);l(o,EntityOracle.rngState(orb.rand));WorldEventOracle.snapshot(w,o);
        }

    }
    static void population(DataOutputStream o)throws Exception {
        for(int biome=0;biome<10;biome++)for(long seed=4096;seed<4128;seed++) {
            World w=world();Random random=new Random(seed);SpawnerAnimals.func_35957_a(w,BiomeGenBase.field_35486_a[biome],-8,-7,16,12,random);
            l(o,EntityOracle.rngState(random));l(o,EntityOracle.rngState(w.rand));l(o,EntityOracle.rngState(EntityOracle.mathRandom()));i(o,w.loadedEntityList.size());
            for(Object obj:w.loadedEntityList){Entity e=(Entity)obj;WorldEventOracle.text(o,EntityList.getEntityString(e));d(o,e.posX);d(o,e.posY);d(o,e.posZ);f(o,e.rotationYaw);l(o,EntityOracle.rngState(e.rand));if(e instanceof EntitySheep)i(o,((EntitySheep)e).getFleeceColor());}
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
            else if(kind.equals("Snowball")){EntitySnowball a1=new EntitySnowball(w,-3.5,68,4.5);a1.setSnowballHeading(.4,.2,1,1.5F,1);e=a1;}
            else if(kind.equals("Egg")){EntityEgg a1=new EntityEgg(w,-3.5,68,4.5);a1.setEggHeading(.4,.2,1,1.5F,1);e=a1;}
            else if(kind.equals("Lightning"))e=new EntityLightningBolt(w,-3.5,64,4.5);
            else if(kind.equals("Arrow")){EntityArrow arrow=new EntityArrow(w,-3.5,68,4.5);arrow.setArrowHeading(0.4,0.2,1,1.5F,1);arrow.field_35140_d=true;e=arrow;}
            else {String name=kind.startsWith("kill-")?kind.substring(5):kind.equals("poison")?"Zombie":kind.equals("teleport")||kind.equals("carry")||kind.equals("indirect")||kind.startsWith("stare")?"Enderman":kind.equals("allies")?"Silverfish":kind.equals("potions")||kind.equals("damage")?"Pig":kind;e=EntityList.createEntityInWorld(name,w);e.setLocationAndAngles(-3.5,64,4.5,35,0);}
            if(kind.equals("carry"))for(int x=-8;x<=8;x++)for(int z=-8;z<=8;z++)EntityOracle.rawBlock(w,x,64,z,37,0);
            if(kind.equals("allies"))for(int x=-7;x<=7;x++)for(int z=-7;z<=7;z++)EntityOracle.rawBlock(w,x,63,z,97,0);
            if(kind.equals("potions")){EntityLiving mob=(EntityLiving)e;mob.health=8;for(int[] p:new int[][]{{1,80,1},{2,120,0},{10,150,0},{19,180,0},{6,3,0},{7,2,0}})mob.func_35165_a(new PotionEffect(p[0],p[1],p[2]));}
            EntityPlayer killer=null;
            if(kind.startsWith("kill-")||kind.equals("attract")||kind.startsWith("stare")){killer=new Player(w);killer.entityId=10000;killer.setPosition(0.5,65.62,0.5);if(kind.startsWith("stare")){killer.rotationYaw=45;killer.rotationPitch=0.5F;if(kind.equals("starepumpkin"))killer.inventory.armorInventory[3]=new ItemStack(86,1,0);}w.playerEntities.add(killer);}
            if(kind.equals("poison")){EntityCaveSpider spider=new EntityCaveSpider(w);spider.setPosition(-3.5,64,3.5);spider.func_35175_b(e);}
            if(kind.equals("allies")){EntityCaveSpider spider=new EntityCaveSpider(w);spider.setPosition(-3.5,64,3.5);e.attackEntityFrom(DamageSource.func_35525_a(spider),2);}
            state(w,e,o);
            for(int t=0;t<ticks;t++) {
                if(kind.equals("indirect")&&t==0)o.writeBoolean(e.attackEntityFrom(DamageSource.func_35535_a(new EntityArrow(w),null),4));
                if(kind.startsWith("kill-") && t==0)e.attackEntityFrom(DamageSource.func_35527_a(killer),1000);
                if(kind.equals("teleport") && t%25==0)o.writeBoolean(((EntityEnderman)e).func_35179_a_(t==25?1000:-4.5,70.75,5.5));
                if(kind.equals("damage") && t%40==0)e.attackEntityFrom(DamageSource.field_35547_j,3);
                e.onUpdate();state(w,e,o);
            }
        }
    }
}
