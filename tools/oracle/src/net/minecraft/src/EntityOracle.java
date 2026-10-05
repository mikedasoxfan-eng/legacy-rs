package net.minecraft.src;

import java.io.*;
import java.util.*;
import java.lang.reflect.*;
import java.util.concurrent.atomic.AtomicLong;

/** Entity oracle cases always execute the original jar, except the independent Entity/Explosion RNG seed hooks. */
public final class EntityOracle {
    static final class FlatProvider implements IChunkProvider {
        final World world; final Map<Long,Chunk> chunks=new HashMap<>();
        FlatProvider(World w) {world=w;for(int x=-3;x<=3;x++)for(int z=-3;z<=3;z++) {
            byte[] blocks=new byte[32768];
            for(int bx=0;bx<16;bx++)for(int bz=0;bz<16;bz++) blocks[(bx<<11)|(bz<<7)|63]=1;
            Chunk c=new Chunk(w,blocks,x,z);c.isTerrainPopulated=true;c.isChunkLoaded=true;chunks.put(key(x,z),c);
        }}
        long key(int x,int z){return ((long)x<<32)^(z&0xffffffffL);}
        public boolean chunkExists(int x,int z){return chunks.containsKey(key(x,z));}
        public Chunk provideChunk(int x,int z){return chunks.get(key(x,z));}
        public Chunk prepareChunk(int x,int z){return provideChunk(x,z);}
        public void populate(IChunkProvider p,int x,int z){}
        public boolean saveChunks(boolean all,IProgressUpdate p){return true;}
        public boolean unload100OldestChunks(){return false;}
        public boolean canSave(){return false;}
        public String makeString(){return "flat oracle fixture";}
    }
    static void rawBlock(World w,int x,int y,int z,int id,int meta) {
        Chunk c=w.getChunkFromChunkCoords(x>>4,z>>4);c.blocks[((x&15)<<11)|((z&15)<<7)|y]=(byte)id;c.data.setNibble(x&15,y,z&15,meta);
    }
    static Random mathRandom() throws Exception {
        Class<?> holder=Class.forName("java.lang.Math$RandomNumberGeneratorHolder");
        Field f=holder.getDeclaredField("randomNumberGenerator");f.setAccessible(true);return (Random)f.get(null);
    }
    static long rngState(Random r) throws Exception {
        Field f=Random.class.getDeclaredField("seed");f.setAccessible(true);return ((AtomicLong)f.get(r)).get();
    }
    static void i(DataOutputStream out,int n)throws IOException{out.writeInt(Integer.reverseBytes(n));}
    static void l(DataOutputStream out,long n)throws IOException{out.writeLong(Long.reverseBytes(n));}
    static void d(DataOutputStream out,double n)throws IOException{l(out,Double.doubleToLongBits(n));}
    static void f(DataOutputStream out,float n)throws IOException{i(out,Float.floatToIntBits(n));}
    static float floatField(Class<?> type,String name,Object o)throws Exception{Field f=type.getDeclaredField(name);f.setAccessible(true);return f.getFloat(o);}
    static boolean booleanField(Class<?> type,String name,Object o)throws Exception{Field f=type.getDeclaredField(name);f.setAccessible(true);return f.getBoolean(o);}
    static int intField(Class<?> type,String name,Object o)throws Exception{Field f=type.getDeclaredField(name);f.setAccessible(true);return f.getInt(o);}
    static void snapshot(World w,DataOutputStream out) throws Exception {
        ArrayList<Entity> entities=new ArrayList<Entity>(w.loadedEntityList);entities.addAll(w.weatherEffects);Collections.sort(entities,(a,b)->Integer.compare(a.entityId,b.entityId));
        i(out,entities.size());
        for(Entity e:entities) {
            i(out,e.entityId);out.writeUTF(e instanceof EntityPlayer?"Player":e instanceof EntityFireball?"Fireball":e instanceof EntityLightningBolt?"LightningBolt":e instanceof EntityEgg?"ThrownEgg":EntityList.getEntityString(e));
            for(double n:new double[]{e.posX,e.posY,e.posZ,e.motionX,e.motionY,e.motionZ,e.prevPosX,e.prevPosY,e.prevPosZ,e.lastTickPosX,e.lastTickPosY,e.lastTickPosZ,e.boundingBox.minX,e.boundingBox.minY,e.boundingBox.minZ,e.boundingBox.maxX,e.boundingBox.maxY,e.boundingBox.maxZ})d(out,n);
            for(float n:new float[]{e.rotationYaw,e.rotationPitch,e.prevRotationYaw,e.prevRotationPitch,e.fallDistance,e.ySize,e.distanceWalkedModified})f(out,n);
            for(int n:new int[]{e.fire,e.air,e.ticksExisted,e.chunkCoordX,e.chunkCoordY,e.chunkCoordZ})i(out,n);
            out.writeBoolean(e.onGround);out.writeBoolean(e.isDead);out.writeBoolean(e.inWater);out.writeBoolean(e.addedToChunk);
            if(e instanceof EntityItem) {
                EntityItem a=(EntityItem)e;i(out,a.age);i(out,a.delayBeforeCanPickup);i(out,a.item.itemID);i(out,a.item.stackSize);i(out,a.item.getItemDamage());f(out,a.field_804_d);
                Field h=EntityItem.class.getDeclaredField("health");h.setAccessible(true);i(out,h.getInt(e));
            }else if(e instanceof EntityFallingSand){i(out,((EntityFallingSand)e).blockID);i(out,((EntityFallingSand)e).fallTime);}
            if(e instanceof EntityLiving) {
                EntityLiving a=(EntityLiving)e;
                for(int n:new int[]{a.health,a.prevHealth,a.heartsLife,a.heartsHalvesLife,a.hurtTime,a.maxHurtTime,a.deathTime,a.attackTime,intField(EntityLiving.class,"livingSoundTime",a),a.entityAge,a.field_9346_af})i(out,n);
                for(float n:new float[]{a.attackedAtYaw,a.renderYawOffset,a.prevRenderYawOffset,a.field_703_S,a.field_704_R,a.field_705_Q,a.swingProgress,a.prevSwingProgress,a.field_9361_v,a.field_9362_u,a.field_9360_w,a.field_9359_x,a.field_9365_p,a.field_9363_r,a.field_9325_U,a.moveStrafing,a.moveForward,a.randomYawVelocity})f(out,n);
                out.writeBoolean(a.isJumping);
                if(e instanceof EntityPig)out.writeBoolean(((EntityPig)e).getSaddled());
                if(e instanceof EntitySheep){out.writeBoolean(((EntitySheep)e).getSheared());i(out,((EntitySheep)e).getFleeceColor());}
                if(e instanceof EntityChicken){EntityChicken c=(EntityChicken)e;for(float n:new float[]{c.field_752_b,c.field_756_e,c.destPos,c.field_757_d,c.field_755_h})f(out,n);i(out,c.timeUntilNextEgg);}
            }
            if(e instanceof EntityTNTPrimed)i(out,((EntityTNTPrimed)e).fuse);
            if(e instanceof EntitySnowball||e instanceof EntityEgg){boolean egg=e instanceof EntityEgg;Class c=egg?EntityEgg.class:EntitySnowball.class;for(String name:egg?new String[]{"field_20056_b","field_20055_c","field_20054_d","field_20053_e"}:new String[]{"xTileSnowball","yTileSnowball","zTileSnowball","inTileSnowball"})i(out,intField(c,name,e));Field g=c.getDeclaredField(egg?"field_20052_f":"inGroundSnowball");g.setAccessible(true);out.writeBoolean(g.getBoolean(e));i(out,intField(c,egg?"field_20057_a":"shakeSnowball",e));Field owner=c.getDeclaredField(egg?"field_20051_g":"thrower");owner.setAccessible(true);Entity a=(Entity)owner.get(e);i(out,a==null?-1:a.entityId);i(out,intField(c,egg?"field_20050_h":"ticksInGroundSnowball",e));i(out,intField(c,egg?"field_20049_i":"ticksInAirSnowball",e));}
            if(e instanceof EntityArrow){EntityArrow a=(EntityArrow)e;for(String name:new String[]{"xTile","yTile","zTile","inTile","field_28019_h"})i(out,intField(EntityArrow.class,name,a));Field ground=EntityArrow.class.getDeclaredField("inGround");ground.setAccessible(true);out.writeBoolean(ground.getBoolean(a));out.writeBoolean(a.doesArrowBelongToPlayer);i(out,a.arrowShake);i(out,a.owner==null?-1:a.owner.entityId);i(out,intField(EntityArrow.class,"ticksInGround",a));i(out,intField(EntityArrow.class,"ticksInAir",a));}
            if(e instanceof EntityCreeper){EntityCreeper a=(EntityCreeper)e;i(out,intField(EntityCreeper.class,"timeSinceIgnited",a));i(out,intField(EntityCreeper.class,"lastActiveTime",a));i(out,a.dataWatcher.getWatchableObjectByte(16));out.writeBoolean(a.getPowered());}
            if(e instanceof EntityPigZombie){i(out,intField(EntityPigZombie.class,"angerLevel",e));i(out,intField(EntityPigZombie.class,"randomSoundDelay",e));}
            if(e instanceof EntityWolf){EntityWolf a=(EntityWolf)e;i(out,a.dataWatcher.getWatchableObjectByte(16));out.writeUTF(a.getWolfOwner());i(out,a.dataWatcher.getWatchableObjectInt(18));for(String name:new String[]{"looksWithInterest","isWolfShaking","field_25052_g"}){Field f=EntityWolf.class.getDeclaredField(name);f.setAccessible(true);out.writeBoolean(f.getBoolean(a));}for(String name:new String[]{"field_25048_b","field_25054_c","timeWolfIsShaking","prevTimeWolfIsShaking"})f(out,floatField(EntityWolf.class,name,a));}
            if(e instanceof EntityGhast){EntityGhast a=(EntityGhast)e;i(out,a.courseChangeCooldown);d(out,a.waypointX);d(out,a.waypointY);d(out,a.waypointZ);i(out,intField(EntityGhast.class,"aggroCooldown",a));i(out,a.prevAttackCounter);i(out,a.attackCounter);out.writeBoolean(a.dataWatcher.getWatchableObjectByte(16)==1);}
            if(e instanceof EntityFireball){EntityFireball a=(EntityFireball)e;for(String name:new String[]{"field_9402_e","field_9401_f","field_9400_g","field_9399_h"})i(out,intField(EntityFireball.class,name,a));Field g=EntityFireball.class.getDeclaredField("field_9398_i");g.setAccessible(true);out.writeBoolean(g.getBoolean(a));i(out,a.field_9406_a);i(out,a.field_9397_j==null?-1:a.field_9397_j.entityId);i(out,intField(EntityFireball.class,"field_9396_k",a));i(out,intField(EntityFireball.class,"field_9395_l",a));d(out,a.field_9405_b);d(out,a.field_9404_c);d(out,a.field_9403_d);}
            if(e instanceof EntitySlime){EntitySlime a=(EntitySlime)e;i(out,a.getSlimeSize());f(out,a.field_768_a);f(out,a.field_767_b);i(out,intField(EntitySlime.class,"slimeJumpDelay",a));}
            if(e instanceof EntitySquid){EntitySquid a=(EntitySquid)e;for(float n:new float[]{a.field_21089_a,a.field_21088_b,a.field_21087_c,a.field_21086_f,a.field_21085_g,a.field_21084_h,a.field_21083_i,a.field_21082_j})f(out,n);for(String name:new String[]{"randomMotionSpeed","field_21080_l","field_21079_m","randomMotionVecX","randomMotionVecY","randomMotionVecZ"})f(out,floatField(EntitySquid.class,name,a));}
            if(e instanceof EntityLightningBolt){i(out,intField(EntityLightningBolt.class,"field_27028_b",e));l(out,((EntityLightningBolt)e).field_27029_a);i(out,intField(EntityLightningBolt.class,"field_27030_c",e));}
            if(e instanceof EntityPlayer)PlayerOracle.state((EntityPlayer)e,out);
            if(e instanceof EntityMinecart) {
                EntityMinecart c=(EntityMinecart)e;
                for(int v:new int[]{c.minecartType,c.minecartCurrentDamage,c.minecartTimeSinceHit,c.minecartRockDirection,c.fuel})i(out,v);
                d(out,c.pushX);d(out,c.pushZ);
                Field rev=EntityMinecart.class.getDeclaredField("field_856_i");rev.setAccessible(true);out.writeBoolean(rev.getBoolean(c));
                Field items=EntityMinecart.class.getDeclaredField("cargoItems");items.setAccessible(true);ItemStack[] a=(ItemStack[])items.get(c);i(out,a.length);
                for(ItemStack item:a)PlayerOracle.stack(out,item);
            }
            if(e instanceof EntityBoat) {EntityBoat b=(EntityBoat)e; i(out,b.boatCurrentDamage);i(out,b.boatTimeSinceHit);i(out,b.boatRockDirection);}
            if(e instanceof EntityBoat||e instanceof EntityMinecart) {
                for(boolean v:new boolean[]{e.isCollidedHorizontally,e.isCollidedVertically,e.isCollided,e.beenAttacked,booleanField(Entity.class,"isFirstUpdate",e),e.preventEntitySpawning})out.writeBoolean(v);
                for(float v:new float[]{e.entityCollisionReduction,e.width,e.height,e.yOffset,e.stepHeight,e.prevDistanceWalkedModified})f(out,v);
            }
            i(out,e.ridingEntity==null?-1:e.ridingEntity.entityId);i(out,e.riddenByEntity==null?-1:e.riddenByEntity.entityId);
            for(String name:new String[]{"entityRiderYawDelta","entityRiderPitchDelta"}) {Field a=Entity.class.getDeclaredField(name);a.setAccessible(true);d(out,a.getDouble(e));}
            l(out,rngState(e.rand));
        }
        l(out,rngState(w.rand));l(out,rngState(mathRandom()));l(out,OracleHooks.nextSeed());l(out,OracleHooks.explosionSeed());l(out,w.worldInfo.getWorldTime());for(int n:new int[]{w.field_9437_g,intField(World.class,"soundCounter",w),w.skylightSubtracted,w.field_27168_F,w.field_27172_i,w.worldInfo.getRainTime(),w.worldInfo.getThunderTime()})i(out,n);out.writeBoolean(w.worldInfo.getRaining());out.writeBoolean(w.worldInfo.getThundering());for(float v:new float[]{w.rainingStrength,w.prevRainingStrength,w.thunderingStrength,w.prevThunderingStrength})f(out,v);
        ArrayList<TileEntityMobSpawner> spawners=new ArrayList<>();for(Object te:w.loadedTileEntityList)if(te instanceof TileEntityMobSpawner)spawners.add((TileEntityMobSpawner)te);Collections.sort(spawners,(a,b)->a.xCoord!=b.xCoord?Integer.compare(a.xCoord,b.xCoord):a.yCoord!=b.yCoord?Integer.compare(a.yCoord,b.yCoord):Integer.compare(a.zCoord,b.zCoord));i(out,spawners.size());for(TileEntityMobSpawner te:spawners){i(out,te.xCoord);i(out,te.yCoord);i(out,te.zCoord);i(out,te.delay);d(out,te.yaw);d(out,te.yaw2);}
        WorldEventOracle.snapshot(w,out);
    }
    static void objects(String scenario,int ticks,String out) throws Exception {
        World w=scenario.equals("nether")?Oracle.newNetherWorld(0):Oracle.newWorld(0);w.chunkProvider=new FlatProvider(w);WorldEventOracle.attach(w);
        EntityOtherPlayerMP player=new EntityOtherPlayerMP(w,"Player");player.setPosition(0.5,65.62,0.5);w.playerEntities.add(player);
        w.difficultySetting=2;w.setAllowedMobSpawns(false,false);
        Field id=Entity.class.getDeclaredField("nextEntityID");id.setAccessible(true);id.setInt(null,0);
        OracleHooks.reset(100);mathRandom().setSeed(200);w.rand.setSeed(300);w.field_9437_g=0;Field sound=World.class.getDeclaredField("soundCounter");sound.setAccessible(true);sound.setInt(w,0);Field chestRand=BlockChest.class.getDeclaredField("random");chestRand.setAccessible(true);((Random)chestRand.get(Block.chest)).setSeed(500);
        boolean passive=scenario.equals("combat") || scenario.equals("nbtall") || scenario.equals("slimes") || scenario.equals("passive") || scenario.equals("aquatic") || scenario.equals("hostile") || scenario.equals("special") || scenario.equals("spawning") || scenario.equals("spawner") || (scenario.equals("client")||scenario.equals("stormclient")) || scenario.equals("nbt") || scenario.equals("nether") || scenario.equals("lightning") || scenario.equals("rider");
        if(passive) {
            FlatProvider provider=(FlatProvider)w.chunkProvider;
            for(int x=-12;x<=12;x++)for(int z=-12;z<=12;z++) {
                if(!provider.chunkExists(x,z)){byte[] blocks=new byte[32768];Chunk c=new Chunk(w,blocks,x,z);c.isTerrainPopulated=true;c.isChunkLoaded=true;provider.chunks.put(provider.key(x,z),c);}
                Chunk c=provider.provideChunk(x,z);Arrays.fill(c.skylightMap.data,(byte)255);Arrays.fill(c.heightMap,(byte)(scenario.equals("slimes")?8:64));
                for(int bx=0;bx<16;bx++)for(int bz=0;bz<16;bz++){if(scenario.equals("slimes"))c.blocks[(bx<<11)|(bz<<7)|63]=0;c.blocks[(bx<<11)|(bz<<7)|(scenario.equals("slimes")?7:63)]=2;}
            }
        }
        for(int x=10;x<=15;x++)for(int y=64;y<=67;y++)for(int z=-2;z<=2;z++)rawBlock(w,x,y,z,9,(x+y+z)&7);
        for(int x=-15;x<=-10;x++)for(int y=64;y<=67;y++)for(int z=-2;z<=2;z++)rawBlock(w,x,y,z,11,0);
        if(scenario.equals("hostile")){w.skylightSubtracted=11;w.worldInfo.setWorldTime(18000);}
        int[] ground={1,79,88,53,67,85,81,78};
        for(int k=0;k<(scenario.equals("nbtall")?18:scenario.equals("spawning")||scenario.equals("spawner")?0:passive?8:16);k++) {
            int x=-24+k*3;
            rawBlock(w,x,63,5,ground[k%8],k%4);
            if(scenario.equals("hooks")){rawBlock(w,x,64,5,new int[]{30,81,70,72,90,0,0,0}[k%8],0);if(k%8>=5)rawBlock(w,x,63,5,new int[]{60,73,74}[k%8-5],0);}
            Entity e;
            if(scenario.equals("slimes")){e=new EntitySlime(w);((EntitySlime)e).setSlimeSize(4);e.setLocationAndAngles(-14+k*4+0.5,8,8.5,k*30,0);}
            else if(scenario.equals("nbtall")){e=k==0?new EntityItem(w,x+0.5,70,5.5,new ItemStack(35,3,7)):k==1?new EntityFallingSand(w,x+0.5,77,5.5,12):k==2?new EntityTNTPrimed(w,x+0.5,70,5.5):k==3?new EntityArrow(w,x+0.5,70,5.5):EntityList.createEntityInWorld(new String[]{"Item","FallingSand","PrimedTnt","Arrow","Pig","Cow","Sheep","Chicken","Squid","Slime","Wolf","Zombie","Skeleton","Spider","Creeper","PigZombie","Giant","Ghast"}[k],w);e.setLocationAndAngles(-34+k*4+0.5,k==17?78:64,12.5,k*30,0);if(e instanceof EntityPig)((EntityPig)e).setSaddled(true);if(e instanceof EntitySheep){((EntitySheep)e).setSheared(true);((EntitySheep)e).setFleeceColor(14);}if(e instanceof EntityCreeper)((EntityCreeper)e).dataWatcher.updateObject(17,(byte)1);if(e instanceof EntityWolf){((EntityWolf)e).setWolfTamed(true);((EntityWolf)e).dataWatcher.updateObject(16,(byte)5);((EntityWolf)e).dataWatcher.updateObject(17,"Player");}}
            else if(scenario.equals("aquatic")){e=EntityList.createEntityInWorld(k<4?"Squid":"Slime",w);e.setLocationAndAngles(k<4?10.5+k: -14+(k-4)*4+0.5,k<4?65:64,k<4?0.5:8.5,k*30,0);}
            else if(scenario.equals("rider")){e=EntityList.createEntityInWorld(k%2==0?"Spider":"Skeleton",w);e.setLocationAndAngles(-14+(k/2)*6+0.5,64,8.5,k*30,0);if(k%2==1)e.mountEntity((Entity)w.loadedEntityList.get(w.loadedEntityList.size()-1));}
            else if(scenario.equals("nether")){e=EntityList.createEntityInWorld(k<4?"Ghast":"PigZombie",w);e.setLocationAndAngles(-14+k*4+0.5,k<4?78:64,8.5,k*30,0);}
            else if(scenario.equals("nbt")){e=k==0?new EntityItem(w,x+0.5,70,5.5,new ItemStack(35,3,7)):k==1?new EntityFallingSand(w,x+0.5,77,5.5,12):EntityList.createEntityInWorld(new String[]{"Item","FallingSand","Pig","Cow","Sheep","Chicken","Squid","Slime"}[k],w);e.setLocationAndAngles(-14+k*4+0.5,64,8.5,k*30,0);}
            else if(scenario.equals("combat")){e=EntityList.createEntityInWorld(k<4?"Wolf":"PigZombie",w);e.setLocationAndAngles(-14+k*4+0.5,64,8.5,k*30,0);}
            else if(scenario.equals("special")){e=EntityList.createEntityInWorld(k<4?"Wolf":"Ghast",w);e.setLocationAndAngles(-14+k*4+0.5,k<4?64:78,8.5,k*30,0);}
            else if(scenario.equals("hostile")){e=EntityList.createEntityInWorld(new String[]{"Zombie","Skeleton","Spider","Creeper","PigZombie","Giant","Zombie","Skeleton"}[k],w);e.setLocationAndAngles(-14+k*4+0.5,64,8.5,k*30,0);}
            else if(scenario.equals("tnt")){e=new EntityTNTPrimed(w,x+0.5,70+k%4,5.5);}
            else if(scenario.equals("arrows")){EntityArrow arrow=new EntityArrow(w,x+0.5,70+k%4,k<8?0.5:5.5);arrow.setArrowHeading(0.2+k*0.03,-0.2,0.1,1.5F,1.0F);e=arrow;}
            else if(passive){e=EntityList.createEntityInWorld(scenario.equals("lightning")&&k==4?"Creeper":new String[]{"Pig","Cow","Sheep","Chicken"}[k%4],w);e.setLocationAndAngles(-14+k*4+0.5,64,8.5,k*30,0);}
            else if(scenario.equals("sand")) {rawBlock(w,x,77+k%4,5,k%2==0?12:13,0);e=new EntityFallingSand(w,x+0.5,77+k%4+0.5,5.5,k%2==0?12:13);}
            else e=new EntityItem(w,x+0.5,scenario.equals("hooks")?64.2:70+k%4,scenario.equals("hooks")?5.5:k<8?0.5:5.5,new ItemStack(1+k,1+k%3,k%4));
            w.entityJoinedWorld(e);
        }
        if(scenario.equals("lightning")||scenario.equals("stormclient")){((Entity)w.loadedEntityList.get(0)).setLocationAndAngles(-0.5,64,8.5,0,0);w.worldInfo.setRaining(true);w.worldInfo.setThundering(true);w.worldInfo.setRainTime(1000000);w.worldInfo.setThunderTime(1000000);w.rainingStrength=w.prevRainingStrength=1;w.thunderingStrength=w.prevThunderingStrength=1;w.addWeatherEffect(new EntityLightningBolt(w,0.5,64,8.5));}
        if(scenario.equals("hooks")){w.setBlockWithNotify(0,64,12,54);TileEntityChest chest=(TileEntityChest)w.getBlockTileEntity(0,64,12);chest.setInventorySlotContents(0,new ItemStack(35,64,7));chest.setInventorySlotContents(5,new ItemStack(264,43,0));w.setBlockWithNotify(8,64,12,46);w.setBlockWithNotify(-8,68,12,1);w.setBlockWithNotify(-8,69,12,12);}
        if((scenario.equals("client")||scenario.equals("stormclient")))w.setAllowedMobSpawns(true,true);
        if(scenario.equals("spawner")){w.skylightSubtracted=11;w.worldInfo.setWorldTime(18000);for(Chunk c:((FlatProvider)w.chunkProvider).chunks.values())Arrays.fill(c.skylightMap.data,(byte)0);w.setBlockWithNotify(4,64,4,52);((TileEntityMobSpawner)w.getBlockTileEntity(4,64,4)).setMobID("Zombie");}
        if(scenario.equals("spawning")){w.skylightSubtracted=11;w.worldInfo.setWorldTime(18000);}
        try(DataOutputStream p=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(out)))) {
            if(scenario.equals("slimes")){player.setPosition(0.5,9.62,0.5);w.skylightSubtracted=11;w.worldInfo.setWorldTime(18000);}
            snapshot(w,p);for(int t=0;t<ticks;t++){if(scenario.equals("combat")&&(t==50||t==100))((Entity)w.loadedEntityList.get(t==50?2:5)).attackEntityFrom(player,1);if(scenario.equals("hooks")&&t==0){w.setBlockWithNotify(0,64,12,0);Block.tnt.onBlockDestroyedByPlayer(w,8,64,12,1);w.setBlockWithNotify(8,64,12,0);w.setBlockWithNotify(-8,68,12,0);}if(scenario.equals("slimes")&&t==100)for(Object obj:new ArrayList(w.loadedEntityList))((Entity)obj).attackEntityFrom(null,4);AxisAlignedBB.clearBoundingBoxPool();Vec3D.initialize();if((scenario.equals("client")||scenario.equals("stormclient"))&&(t+1)%30==0)joinSurroundings(w,player);w.updateEntities();if(scenario.equals("hooks")){w.worldInfo.setWorldTime(w.worldInfo.getWorldTime()+1);w.TickUpdates(false);w.randomDisplayUpdates(0,65,0);}if(scenario.equals("slimes")||scenario.equals("spawning")||scenario.equals("nether"))SpawnerAnimals.performSpawning(w,true,true);if((scenario.equals("client")||scenario.equals("stormclient"))){w.tick();w.randomDisplayUpdates(0,65,0);}if((scenario.equals("nbt")&&t%100==99)||(scenario.equals("nbtall")&&t%25==24)){ArrayList<NBTTagCompound> tags=new ArrayList<>();for(Object obj:w.loadedEntityList){NBTTagCompound n=new NBTTagCompound();if(((Entity)obj).addEntityID(n))tags.add(n);}w.loadedEntityList.clear();for(Chunk c:((FlatProvider)w.chunkProvider).chunks.values())for(List list:c.entities)list.clear();for(NBTTagCompound n:tags)w.entityJoinedWorld(EntityList.createEntityFromNBT(n,w));}snapshot(w,p);}
        }
        Oracle.dumpWorld(w,out+".world",0,0,0);try(OutputStream p=new FileOutputStream(out,true)){p.write(java.nio.file.Files.readAllBytes(new File(out+".world").toPath()));}new File(out+".world").delete();
    }
    static void joinSurroundings(World w,Entity player){w.joinEntityInSurroundings(player);for(Iterator it=w.loadedEntityList.iterator();it.hasNext();)if(it.next()==player)it.remove();if(player.addedToChunk){for(List list:w.getChunkFromChunkCoords(player.chunkCoordX,player.chunkCoordZ).entities)for(Iterator it=list.iterator();it.hasNext();)if(it.next()==player)it.remove();player.addedToChunk=false;}}
    static void blockPhysics(String out)throws Exception {
        World w=Oracle.newWorld(0);w.chunkProvider=new FlatProvider(w);Random r=new Random(979);
        try(DataOutputStream p=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(out)))){for(int k=0;k<10000;k++){
            AxisAlignedBB.clearBoundingBoxPool();Vec3D.initialize();int id=1+r.nextInt(96);int meta=r.nextInt(16);rawBlock(w,8,64,8,id,meta);
            double x=8.5+r.nextDouble()*3-1.5,y=64.5+r.nextDouble()*3-1.5,z=8.5+r.nextDouble()*3-1.5;
            AxisAlignedBB b=AxisAlignedBB.getBoundingBox(x,y,z,x+r.nextDouble()*2,y+r.nextDouble()*2,z+r.nextDouble()*2);List<AxisAlignedBB> boxes=w.getCollidingBoundingBoxes(null,b);i(p,boxes.size());for(AxisAlignedBB a:boxes)for(double v:new double[]{a.minX,a.minY,a.minZ,a.maxX,a.maxY,a.maxZ})d(p,v);
            Vec3D start=Vec3D.createVector(8.5+r.nextDouble()*4-2,64.5+r.nextDouble()*4-2,8.5+r.nextDouble()*4-2);Vec3D end=Vec3D.createVector(8.5+r.nextDouble()*4-2,64.5+r.nextDouble()*4-2,8.5+r.nextDouble()*4-2);
            MovingObjectPosition hit=w.func_28105_a(start,end,r.nextBoolean(),r.nextBoolean());i(p,hit==null?-1:hit.sideHit);if(hit!=null){i(p,hit.blockX);i(p,hit.blockY);i(p,hit.blockZ);d(p,hit.hitVec.xCoord);d(p,hit.hitVec.yCoord);d(p,hit.hitVec.zCoord);}
        }}
    }
    static String box(AxisAlignedBB b) {
        if (b == null) return "None";
        return "Some(AxisAlignedBB::new(" + b.minX + "," + b.minY + "," + b.minZ + "," + b.maxX + "," + b.maxY + "," + b.maxZ + "))";
    }
    static void blockGeometry(String out) throws Exception {
        World w = Oracle.newWorld(0);
        Chunk c = w.getChunkFromChunkCoords(0, 0);
        int idx = (8 << 11) | (8 << 7) | 64;
        try (PrintWriter p = new PrintWriter(out, "UTF-8")) {
            p.println("// @generated by EntityOracle.blockGeometry from the original b1.7.3 jar.");
            p.println("use super::geometry::AxisAlignedBB;");
            p.println("pub const INITIAL_BOUNDS: [AxisAlignedBB;256] = [");
            for (int id=0;id<256;id++) {
                Block b=Block.blocksList[id];
                p.println(b == null ? "AxisAlignedBB::new(0.0,0.0,0.0,0.0,0.0,0.0)," :
                    "AxisAlignedBB::new("+b.minX+","+b.minY+","+b.minZ+","+b.maxX+","+b.maxY+","+b.maxZ+"),");
            }
            p.println("];");
            p.println("pub const STATE_BOUNDS: [[AxisAlignedBB;16];256] = [");
            for(int id=0;id<256;id++) {
                Block b=Block.blocksList[id];p.print("[");c.blocks[idx]=(byte)id;
                double[] saved=b==null?null:new double[]{b.minX,b.minY,b.minZ,b.maxX,b.maxY,b.maxZ};
                for(int meta=0;meta<16;meta++) {
                    if(b!=null) {
                        b.setBlockBounds((float)saved[0],(float)saved[1],(float)saved[2],(float)saved[3],(float)saved[4],(float)saved[5]);
                        c.data.setNibble(8,64,8,meta);b.setBlockBoundsBasedOnState(w,8,64,8);
                    }
                    p.print(b==null?"AxisAlignedBB::new(0.0,0.0,0.0,0.0,0.0,0.0),":"AxisAlignedBB::new("+b.minX+","+b.minY+","+b.minZ+","+b.maxX+","+b.maxY+","+b.maxZ+"),");
                }
                if(b!=null)b.setBlockBounds((float)saved[0],(float)saved[1],(float)saved[2],(float)saved[3],(float)saved[4],(float)saved[5]);
                p.println("],");
            }p.println("];");
            p.println("pub const HAS_STATE_BOUNDS: [bool;256] = [");
            for(Block b:Block.blocksList)p.println((b!=null && b.getClass().getMethod("setBlockBoundsBasedOnState",IBlockAccess.class,int.class,int.class,int.class).getDeclaringClass()!=Block.class)+",");
            p.println("];");
            p.println("pub const BASE_COLLISION: [bool;256] = [");
            for(Block b:Block.blocksList)p.println((b!=null && b.getClass().getMethod("getCollisionBoundingBoxFromPool",World.class,int.class,int.class,int.class).getDeclaringClass()==Block.class)+",");
            p.println("];");
            p.println("pub const MULTI_COLLISION: [[&[AxisAlignedBB];16];256] = [");
            for(int id=0;id<256;id++) {
                Block b=Block.blocksList[id];p.print("[");c.blocks[idx]=(byte)id;
                for(int meta=0;meta<16;meta++) {
                    c.data.setNibble(8,64,8,meta);ArrayList<AxisAlignedBB> boxes=new ArrayList<>();
                    if(b instanceof BlockStairs || b instanceof BlockPistonExtension) b.getCollidingBoundingBoxes(w,8,64,8,AxisAlignedBB.getBoundingBox(-100,-100,-100,100,200,100),boxes);
                    p.print("&[");for(AxisAlignedBB bb:boxes)p.print(box(bb.getOffsetBoundingBox(-8,-64,-8)).replace("Some(","").replace("))", ")")+",");p.print("],");
                }p.println("],");
            }p.println("];");
            p.println("pub const COLLISION: [[Option<AxisAlignedBB>;16];256] = [");
            for (int id=0;id<256;id++) {
                Block b=Block.blocksList[id]; p.print("[");
                double[] saved=b==null?null:new double[]{b.minX,b.minY,b.minZ,b.maxX,b.maxY,b.maxZ};
                c.blocks[idx]=(byte)id;
                for (int meta=0;meta<16;meta++) {
                    if (b != null) b.setBlockBounds((float)saved[0],(float)saved[1],(float)saved[2],(float)saved[3],(float)saved[4],(float)saved[5]);
                    c.data.setNibble(8,64,8,meta);
                    AxisAlignedBB bb=b==null?null:b.getCollisionBoundingBoxFromPool(w,8,64,8);
                    if (bb!=null) bb=bb.getOffsetBoundingBox(-8,-64,-8);
                    p.print(box(bb)+",");
                }
                if (b != null) b.setBlockBounds((float)saved[0],(float)saved[1],(float)saved[2],(float)saved[3],(float)saved[4],(float)saved[5]);
                p.println("], // "+id);
            }
            p.println("];");
            p.println("pub const SLIPPERINESS: [f32;256] = [");
            for (Block b:Block.blocksList) p.println((b==null?0.6F:b.slipperiness)+"f32,");
            p.println("];");
            p.println("pub const RESISTANCE: [f32;256] = [");
            for (Block b:Block.blocksList) p.println((b==null?0.0F:b.getExplosionResistance(null))+"f32,");
            p.println("];");
        }
    }
    static void geometry(String out) throws Exception {
        Random r=new Random(1729);
        try (DataOutputStream p=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(out)))) {
            for(int i=0;i<10000;i++) {
                AxisAlignedBB.clearBoundingBoxPool(); Vec3D.initialize();
                double x=r.nextDouble()*200-100, y=r.nextDouble()*200-100,z=r.nextDouble()*200-100;
                AxisAlignedBB a=AxisAlignedBB.getBoundingBox(x,y,z,x+r.nextDouble()*4,y+r.nextDouble()*4,z+r.nextDouble()*4);
                Vec3D v=Vec3D.createVector(r.nextDouble()*200-100,r.nextDouble()*200-100,r.nextDouble()*200-100);
                Vec3D u=Vec3D.createVector(r.nextDouble()*200-100,r.nextDouble()*200-100,r.nextDouble()*200-100);
                Vec3D n=v.normalize(),c=v.crossProduct(u);
                for(double d:new double[]{n.xCoord,n.yCoord,n.zCoord,c.xCoord,c.yCoord,c.zCoord,v.distanceTo(u),v.squareDistanceTo(u),v.lengthVector()}) p.writeLong(Long.reverseBytes(Double.doubleToLongBits(d)));
                float angle=r.nextFloat()*20-10;
                v.rotateAroundX(angle); v.rotateAroundY(angle);
                for(double d:new double[]{v.xCoord,v.yCoord,v.zCoord}) p.writeLong(Long.reverseBytes(Double.doubleToLongBits(d)));
                MovingObjectPosition hit=a.func_1169_a(v,u);
                p.writeInt(Integer.reverseBytes(hit==null?-1:hit.sideHit));
                if(hit!=null) for(double d:new double[]{hit.hitVec.xCoord,hit.hitVec.yCoord,hit.hitVec.zCoord}) p.writeLong(Long.reverseBytes(Double.doubleToLongBits(d)));
                AxisAlignedBB b=a.getOffsetBoundingBox(r.nextDouble()*8-4,r.nextDouble()*8-4,r.nextDouble()*8-4);
                double delta=r.nextDouble()*8-4;
                for(double d:new double[]{a.calculateXOffset(b,delta),a.calculateYOffset(b,delta),a.calculateZOffset(b,delta),a.getAverageEdgeLength()}) p.writeLong(Long.reverseBytes(Double.doubleToLongBits(d)));
                p.writeBoolean(a.intersectsWith(b)); p.writeBoolean(a.isVecInside(v));
                d(p,r.nextGaussian());l(p,rngState(r));d(p,Math.atan2(v.xCoord,u.zCoord));
            }
        }
    }
}




