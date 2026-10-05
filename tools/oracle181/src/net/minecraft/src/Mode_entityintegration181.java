package net.minecraft.src;
import java.io.*;
import java.util.*;
import java.lang.reflect.*;
import static net.minecraft.src.EntityOracle.*;
/** Inherited script with VM identity inputs recorded, unchanged snapshots. */
public final class Mode_entityintegration181 {
    public static void main(String[] a) throws Exception {
        objects(a[1], Integer.parseInt(a[2]), a[3]);
    }
    static void objects(String scenario,int ticks,String out) throws Exception {
        World w=scenario.equals("nether")?Oracle.newNetherWorld(0):Oracle.newWorld(0);w.chunkProvider=new FlatProvider(w);WorldEventOracle.attach(w);
        try (Mode_replayidentities181 identities = new Mode_replayidentities181(w, out)) {
        EntityOtherPlayerMP player=new EntityOtherPlayerMP(w,"Player");player.setPosition(0.5,65.62,0.5);w.playerEntities.add(player);
        w.difficultySetting=2;w.setAllowedMobSpawns(false,false);
        Field id=Entity.class.getDeclaredField("nextEntityID");id.setAccessible(true);id.setInt(null,0);
        OracleHooks.reset(100);mathRandom().setSeed(200);w.rand.setSeed(300);w.updateLCG=0;Field sound=World.class.getDeclaredField("soundCounter");sound.setAccessible(true);sound.setInt(w,0);Field chestRand=BlockChest.class.getDeclaredField("random");chestRand.setAccessible(true);((Random)chestRand.get(Block.chest)).setSeed(500);
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
            else if(scenario.equals("nbtall")){e=k==0?new EntityItem(w,x+0.5,70,5.5,new ItemStack(35,3,7)):k==1?new EntityFallingSand(w,x+0.5,77,5.5,12):k==2?new EntityTNTPrimed(w,x+0.5,70,5.5):k==3?new EntityArrow(w,x+0.5,70,5.5):EntityList.createEntityInWorld(new String[]{"Item","FallingSand","PrimedTnt","Arrow","Pig","Cow","Sheep","Chicken","Squid","Slime","Wolf","Zombie","Skeleton","Spider","Creeper","PigZombie","Giant","Ghast"}[k],w);e.setLocationAndAngles(-34+k*4+0.5,k==17?78:64,12.5,k*30,0);if(e instanceof EntityPig)((EntityPig)e).setSaddled(true);if(e instanceof EntitySheep){((EntitySheep)e).setSheared(true);((EntitySheep)e).setFleeceColor(14);}if(e instanceof EntityCreeper)((EntityCreeper)e).dataWatcher.updateObject(17,(byte)1);if(e instanceof EntityWolf){((EntityWolf)e).setIsTamed(true);((EntityWolf)e).dataWatcher.updateObject(16,(byte)5);((EntityWolf)e).dataWatcher.updateObject(17,"Player");}}
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
        if(scenario.equals("lightning")||scenario.equals("stormclient")){((Entity)w.loadedEntityList.get(0)).setLocationAndAngles(-0.5,64,8.5,0,0);w.worldInfo.setIsRaining(true);w.worldInfo.setIsThundering(true);w.worldInfo.setRainTime(1000000);w.worldInfo.setThunderTime(1000000);w.rainingStrength=w.prevRainingStrength=1;w.thunderingStrength=w.prevThunderingStrength=1;w.addWeatherEffect(new EntityLightningBolt(w,0.5,64,8.5));}
        if(scenario.equals("hooks")){w.setBlockWithNotify(0,64,12,54);TileEntityChest chest=(TileEntityChest)w.getBlockTileEntity(0,64,12);chest.setInventorySlotContents(0,new ItemStack(35,64,7));chest.setInventorySlotContents(5,new ItemStack(264,43,0));w.setBlockWithNotify(8,64,12,46);w.setBlockWithNotify(-8,68,12,1);w.setBlockWithNotify(-8,69,12,12);}
        if((scenario.equals("client")||scenario.equals("stormclient")))w.setAllowedMobSpawns(true,true);
        if(scenario.equals("spawner")){w.skylightSubtracted=11;w.worldInfo.setWorldTime(18000);for(Chunk c:((FlatProvider)w.chunkProvider).chunks.values())Arrays.fill(c.skylightMap.data,(byte)0);w.setBlockWithNotify(4,64,4,52);((TileEntityMobSpawner)w.getBlockTileEntity(4,64,4)).setMobID("Zombie");}
        if(scenario.equals("spawning")){w.skylightSubtracted=11;w.worldInfo.setWorldTime(18000);}
        try(DataOutputStream p=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(out)))) {
            if(scenario.equals("slimes")){player.setPosition(0.5,9.62,0.5);w.skylightSubtracted=11;w.worldInfo.setWorldTime(18000);}
            snapshot(w,p);for(int t=0;t<ticks;t++){if(scenario.equals("combat")&&(t==50||t==100))((Entity)w.loadedEntityList.get(t==50?2:5)).attackEntityFrom(DamageSource.func_35527_a(player),1);if(scenario.equals("hooks")&&t==0){w.setBlockWithNotify(0,64,12,0);Block.tnt.onBlockDestroyedByPlayer(w,8,64,12,1);w.setBlockWithNotify(8,64,12,0);w.setBlockWithNotify(-8,68,12,0);}if(scenario.equals("slimes")&&t==100)for(Object obj:new ArrayList(w.loadedEntityList))((Entity)obj).attackEntityFrom(DamageSource.field_35547_j,4);AxisAlignedBB.clearBoundingBoxPool();Vec3D.initialize();if((scenario.equals("client")||scenario.equals("stormclient"))&&(t+1)%30==0)joinSurroundings(w,player);w.updateEntities();if(scenario.equals("hooks")){w.worldInfo.setWorldTime(w.worldInfo.getWorldTime()+1);w.tickUpdates(false);w.randomDisplayUpdates(0,65,0);}if(scenario.equals("slimes")||scenario.equals("spawning")||scenario.equals("nether"))SpawnerAnimals.performSpawning(w,true,true);if((scenario.equals("client")||scenario.equals("stormclient"))){w.tick();w.randomDisplayUpdates(0,65,0);}if((scenario.equals("nbt")&&t%100==99)||(scenario.equals("nbtall")&&t%25==24)){ArrayList<NBTTagCompound> tags=new ArrayList<>();for(Object obj:w.loadedEntityList){NBTTagCompound n=new NBTTagCompound();if(((Entity)obj).addEntityID(n))tags.add(n);}w.loadedEntityList.clear();for(Chunk c:((FlatProvider)w.chunkProvider).chunks.values())for(List list:c.entities)list.clear();for(NBTTagCompound n:tags)w.entityJoinedWorld(EntityList.createEntityFromNBT(n,w));}snapshot(w,p);}
        }
        Oracle.dumpWorld(w,out+".world",0,0,0);try(OutputStream p=new FileOutputStream(out,true)){p.write(java.nio.file.Files.readAllBytes(new File(out+".world").toPath()));}new File(out+".world").delete();
        }
    }
    static void joinSurroundings(World w,Entity player){w.joinEntityInSurroundings(player);for(Iterator it=w.loadedEntityList.iterator();it.hasNext();)if(it.next()==player)it.remove();if(player.addedToChunk){for(List list:w.getChunkFromChunkCoords(player.chunkCoordX,player.chunkCoordZ).entities)for(Iterator it=list.iterator();it.hasNext();)if(it.next()==player)it.remove();player.addedToChunk=false;}}
}
