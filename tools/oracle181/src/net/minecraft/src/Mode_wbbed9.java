package net.minecraft.src;
import java.io.*;import java.lang.reflect.*;import java.util.*;
/** Original BlockBed bytecode, with its separately owned sleep/explosion callees recorded. */
public final class Mode_wbbed9 {
    static class BedWorld extends World {
        ArrayList<double[]> blasts;
        BedWorld(){super((ISaveHandler)null,"",(WorldProvider)null,(WorldSettings)null);}
        public Explosion newExplosion(Entity e,double x,double y,double z,float strength,boolean fire){blasts.add(new double[]{x,y,z,strength,fire?1:0});return null;}
    }
    static class BedPlayer extends EntityPlayer {
        EnumStatus result=EnumStatus.OK;ArrayList<int[]> calls=new ArrayList<int[]>();ArrayList<String> chat=new ArrayList<String>();boolean asleep;
        BedPlayer(World w){super(w);}
        public void func_6420_o(){}
        public EnumStatus sleepInBedAt(int x,int y,int z){calls.add(new int[]{x,y,z});return result;}
        public void addChatMessage(String s){chat.add(s);}
        public boolean isPlayerSleeping(){return asleep;}
    }
    static BedWorld world()throws Exception {
        World original=Mode_worldblocks181.fixture(true);BedWorld w=(BedWorld)Oracle.allocate(BedWorld.class);
        for(Field f:World.class.getDeclaredFields())if(!Modifier.isStatic(f.getModifiers())){f.setAccessible(true);f.set(w,f.get(original));}
        w.blasts=new ArrayList<double[]>();w.worldProvider.worldObj=w;
        for(int x=-3;x<=3;x++)for(int z=-3;z<=3;z++)w.getChunkFromChunkCoords(x,z).worldObj=w;
        return w;
    }
    static void string(DataOutputStream o,String s)throws Exception {byte[] bytes=s.getBytes("UTF-8");EntityOracle.i(o,bytes.length);o.write(bytes);}
    public static void main(String[] a)throws Exception {
        int init=Block.stone.blockID;BedWorld w=world();BedPlayer p=new BedPlayer(w),other=new BedPlayer(w);BlockBed b=(BlockBed)Block.bed;
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            for(boolean remote:new boolean[]{false,true})for(boolean hell:new boolean[]{false,true})for(int m=0;m<16;m++)for(int config=0;config<16;config++)for(EnumStatus status:EnumStatus.values()) {
                for(int x=6;x<=10;x++)for(int y=62;y<=65;y++)for(int z=6;z<=10;z++)Mode_worldblocks181.raw(w,x,y,z,0,0);
                Mode_worldblocks181.raw(w,8,63,8,1,0);Mode_worldblocks181.raw(w,8,64,8,26,m);int[] d=BlockBed.headBlockToFootBlockMap[m&3];
                if((config&1)!=0)Mode_worldblocks181.raw(w,8+d[0],64,8+d[1],26,8|(m&7));
                if((config&2)!=0)Mode_worldblocks181.raw(w,8-d[0],64,8-d[1],26,m&7);
                w.playerEntities.clear();if((config&4)!=0)w.playerEntities.add(other);p.asleep=(config&8)!=0;if(p.asleep)w.playerEntities.add(p);other.asleep=true;other.bedChunkCoordinates=new ChunkCoordinates((m&8)!=0?8:8+d[0],64,(m&8)!=0?8:8+d[1]);
                Field provider=World.class.getDeclaredField("worldProvider");provider.setAccessible(true);provider.set(w,hell?new WorldProviderHell():new WorldProviderSurface());p.bedChunkCoordinates=other.bedChunkCoordinates;w.worldProvider.worldObj=w;w.multiplayerWorld=remote;w.loadedEntityList.clear();Mode_wbtrapdoors6.seed(w);p.calls.clear();p.chat.clear();w.blasts.clear();p.result=status;
                o.writeBoolean(b.blockActivated(w,8,64,8,p));Mode_wbtrapdoors6.result(o,w);EntityOracle.i(o,p.calls.size());for(int[] call:p.calls)for(int n:call)EntityOracle.i(o,n);EntityOracle.i(o,w.blasts.size());for(double[] blast:w.blasts)for(double v:blast)EntityOracle.d(o,v);EntityOracle.i(o,p.chat.size());for(String s:p.chat)string(o,s);
            }
            for(boolean remote:new boolean[]{false,true})for(int m=0;m<16;m++)for(int paired:new int[]{0,1,26}) {
                for(int x=7;x<=9;x++)for(int z=7;z<=9;z++)Mode_worldblocks181.raw(w,x,64,z,0,0);
                Mode_worldblocks181.raw(w,8,64,8,26,m);int[] d=BlockBed.headBlockToFootBlockMap[m&3];Mode_worldblocks181.raw(w,8+((m&8)!=0?-d[0]:d[0]),64,8+((m&8)!=0?-d[1]:d[1]),paired,0);
                w.multiplayerWorld=remote;w.loadedEntityList.clear();Mode_wbtrapdoors6.seed(w);b.onNeighborBlockChange(w,8,64,8,1);Mode_wbtrapdoors6.result(o,w);
            }
            for(int m=0;m<16;m++)for(boolean occupied:new boolean[]{false,true}) {Mode_worldblocks181.raw(w,8,64,8,26,m);BlockBed.setBedOccupied(w,8,64,8,occupied);EntityOracle.i(o,w.getBlockMetadata(8,64,8));}
            for(int m=0;m<16;m++)for(int mask=0;mask<16;mask++)for(int skip:new int[]{Integer.MIN_VALUE,-1,0,1,8,9,17,18,Integer.MAX_VALUE}) {
                for(int x=6;x<=10;x++)for(int z=6;z<=10;z++){Mode_worldblocks181.raw(w,x,63,z,((x+z)&1)==0&&(mask&1)!=0?0:1,0);Mode_worldblocks181.raw(w,x,64,z,(mask&2)!=0&&x==8?1:0,0);Mode_worldblocks181.raw(w,x,65,z,(mask&4)!=0&&z==8?1:0,0);}
                Mode_worldblocks181.raw(w,8,64,8,26,m);ChunkCoordinates c=BlockBed.getNearestEmptyChunkCoordinates(w,8,64,8,skip);o.writeBoolean(c!=null);if(c!=null){EntityOracle.i(o,c.posX);EntityOracle.i(o,c.posY);EntityOracle.i(o,c.posZ);}
            }
            for(int m=0;m<16;m++){Mode_worldblocks181.raw(w,8,64,8,26,m);b.setBlockBounds(-.25F,0,.25F,1.25F,1,.75F);Mode_worldblocks181.box(o,b.getSelectedBoundingBoxFromPool(w,8,64,8));Mode_worldblocks181.box(o,b.getCollisionBoundingBoxFromPool(w,8,64,8));b.setBlockBoundsBasedOnState(w,8,64,8);Mode_wbtrapdoor8.bounds(o,b);}
            // Finding an occupant must not skip a later sleeping player's null bed.
            for(boolean matchingFirst:new boolean[]{false,true}) {
                Field provider=World.class.getDeclaredField("worldProvider");provider.setAccessible(true);provider.set(w,new WorldProviderSurface());w.worldProvider.worldObj=w;w.multiplayerWorld=false;
                w.playerEntities.clear();p.asleep=other.asleep=true;p.bedChunkCoordinates=null;other.bedChunkCoordinates=new ChunkCoordinates(8,64,8);
                w.playerEntities.add(matchingFirst?other:p);w.playerEntities.add(matchingFirst?p:other);Mode_worldblocks181.raw(w,8,64,8,26,12);w.loadedEntityList.clear();Mode_wbtrapdoors6.seed(w);
                try {b.blockActivated(w,8,64,8,p);o.writeBoolean(true);}catch(NullPointerException e){o.writeBoolean(false);}Mode_wbtrapdoors6.result(o,w);
            }
        }
    }
}
