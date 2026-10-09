package net.minecraft.src;

import java.io.*;
import java.util.*;
import net.minecraft.client.Minecraft;

/** Original Release bytecode; subclasses replace only UI/terrain infrastructure. */
public final class Mode_r10entities3 {
    static void v(String n,int x)throws Exception{Mode_r10entities2.value(n,x);}
    static void v(String n,float x)throws Exception{Mode_r10entities2.value(n,x);}
    static void v(String n,double x)throws Exception{Mode_r10entities2.value(n,x);}
    static void t(String n,String x)throws Exception{Mode_r10entities2.text(n,x);}
    static void player(EntityPlayer p)throws Exception {
        v("health",p.health);v("xp",p.currentXP);v("level",p.playerLevel);v("total",p.totalXP);v("score",p.score);
        FoodStats f=p.getFoodStats();
        for(String n:new String[]{"foodLevel","foodTickTimer","prevFoodLevel"})v(n,(Integer)Mode_r10entities2.field(f,n));
        for(String n:new String[]{"foodSaturationLevel","foodExhaustionLevel"})v(n,(Float)Mode_r10entities2.field(f,n));
        v("slot",p.inventory.currentItem);
        for(ItemStack s:p.inventory.mainInventory){v("item",s==null?-1:s.itemID);v("count",s==null?0:s.stackSize);v("damage",s==null?0:s.getItemDamage());}
        for(ItemStack s:p.inventory.armorInventory){v("armor",s==null?-1:s.itemID);v("count",s==null?0:s.stackSize);v("damage",s==null?0:s.getItemDamage());}
        v("dimension",p.dimension);v("flying",p.capabilities.isFlying?1:0);v("creative",p.capabilities.depleteBuckets?1:0);
    }
    static void copy()throws Exception {
        for(int scene=0;scene<24;scene++) {
            World w=Mode_entities10.world();
            EntityPlayer a=new Mode_entities10.Player(w),b=new Mode_entities10.Player(w),c=new Mode_entities10.Player(w);
            a.health=scene%20+1;a.currentXP=scene*0.03125F;a.playerLevel=scene;a.totalXP=scene*113;a.score=scene*37;
            a.dimension=1;a.capabilities.isFlying=true;a.capabilities.depleteBuckets=true;
            a.inventory.currentItem=scene%9;a.inventory.mainInventory[scene]=new ItemStack(35,scene+1,scene%16);
            a.inventory.armorInventory[scene%4]=new ItemStack(310+scene%4,1,scene*3);
            NBTTagCompound n=new NBTTagCompound();n.setInteger("foodLevel",scene%21);n.setInteger("foodTickTimer",scene*7);n.setFloat("foodSaturationLevel",scene*0.25F);n.setFloat("foodExhaustionLevel",scene*0.5F);a.getFoodStats().readStatsFromNBT(n);
            b.inventory.currentItem=8;b.copyPlayer(a);c.copyPlayer(b);
            v("sameFoodAB",a.getFoodStats()==b.getFoodStats()?1:0);v("sameFoodBC",b.getFoodStats()==c.getFoodStats()?1:0);
            v("deepStack",a.inventory.mainInventory[scene]!=b.inventory.mainInventory[scene]?1:0);
            b.inventory.mainInventory[scene].stackSize+=5;
            for(int tick=0;tick<240;tick++) {
                EntityPlayer p=tick%3==0?a:tick%3==1?b:c;
                if(tick%17==0)p.getFoodStats().addStats(2,0.3F);
                if(tick%7==0)p.getFoodStats().addExhaustion(3.25F);
                w.difficultySetting=(tick/60)%4;p.getFoodStats().onUpdate(p);
                if(tick==100){n.setInteger("foodLevel",0);n.setFloat("foodSaturationLevel",0);n.setFloat("foodExhaustionLevel",0);n.setInteger("foodTickTimer",79);b.getFoodStats().readStatsFromNBT(n);}
                player(a);player(b);player(c);
            }
        }
    }
    public static class Client extends Minecraft {
        int requested=-999;String screen;String title;
        public Client(){super(null,null,null,1,1,false);}
        public void displayUnexpectedThrowable(UnexpectedThrowable t){throw new RuntimeException();}
        public void usePortal(int dimension){requested=dimension;}
        public void displayGuiScreen(GuiScreen gui){screen=gui==null?null:gui.getClass().getSimpleName();}
        public void changeWorld(World w,String name,EntityPlayer p){theWorld=w;title=name;w.chunkProvider=new EntityOracle.FlatProvider(w);
            EntityOracle.FlatProvider f=(EntityOracle.FlatProvider)w.chunkProvider;
            for(int x=4;x<=9;x++)for(int z=-3;z<=3;z++){Chunk c=new Chunk(w,new byte[32768],x,z);c.isChunkLoaded=true;c.isTerrainPopulated=true;f.chunks.put(f.key(x,z),c);}
            w.spawnEntityInWorld(p);}
        void travel(int dimension){super.usePortal(dimension);}
    }
    static final class Player extends EntityPlayerSP {
        Player(Client c,World w){super(c,w,new Session("Player",""),0);}
        public void addStat(StatBase s,int n){PlayerOracle.stat(this,s,n);}
    }
    static Client client(World w)throws Exception {
        Client mc=(Client)Oracle.allocate(Client.class);mc.requested=-999;mc.theWorld=w;
        WorldEventOracle.Sounds sound=(WorldEventOracle.Sounds)Oracle.allocate(WorldEventOracle.Sounds.class);sound.world=w;mc.sndManager=sound;
        return mc;
    }
    static void endEvents()throws Exception {
        for(int seed=0;seed<16;seed++)for(boolean remote:new boolean[]{false,true})for(int from:new int[]{-1,0,1})for(int to:new int[]{-1,0,1,2}) {
            World w=Mode_entities10.world();w.multiplayerWorld=remote;Client mc=client(w);Player p=new Player(mc,w);mc.thePlayer=p;p.dimension=from;p.rand.setSeed(seed);
            p.func_40182_b(to);
            v("requested",mc.requested);t("screen",mc.screen);
            ArrayList<int[]> log=PlayerOracle.stats.remove(p);v("stats",log==null?0:log.size());if(log!=null)for(int[] s:log){v("stat",s[0]);v("amount",s[1]);}
            Mode_r10entities2.value("rng",EntityOracle.rngState(p.rand));
            ByteArrayOutputStream bytes=new ByteArrayOutputStream();WorldEventOracle.snapshot(w,new DataOutputStream(bytes));
            byte[] events=bytes.toByteArray();Mode_r10entities2.labels.println(Mode_r10entities2.offset+"\t"+events.length+"\tevents");Mode_r10entities2.offset+=events.length;Mode_r10entities2.out.write(events);
        }
    }
    static void travel()throws Exception {
        for(int from:new int[]{0,-1,1}) {
            World w=Mode_entities10.world();Client mc=client(w);Player p=new Player(mc,w);mc.thePlayer=p;p.dimension=from;
            p.setLocationAndAngles(-3.75,71.125,4.875,37,-12);p.motionX=.25;p.motionY=-.125;p.motionZ=.75;p.inventory.mainInventory[0]=new ItemStack(264,13,0);w.spawnEntityInWorld(p);
            mc.travel(from==1?0:1);w=mc.theWorld;
            t("title",mc.title);player(p);
            for(double x:new double[]{p.posX,p.posY,p.posZ,p.motionX,p.motionY,p.motionZ,p.boundingBox.minY})v("position",x);
            v("yaw",p.rotationYaw);v("pitch",p.rotationPitch);
            if(from!=1)for(int x=98;x<=102;x++)for(int y=51;y<=54;y++)for(int z=-2;z<=2;z++)v("platform",w.getBlockId(x,y,z));
        }
    }
    static final class Effects extends EffectRenderer {
        String kind;Entity target;
        Effects(World w){super(w,null);}
        public void addEffect(EntityFX fx){try{kind=(String)Mode_r10entities2.field(fx,"field_40105_ay");target=(Entity)Mode_r10entities2.field(fx,"field_35134_a");}catch(Exception ex){throw new RuntimeException(ex);}}
    }
    static void magic()throws Exception {
        for(int scene=0;scene<32;scene++) {
            World w=Mode_entities10.world();Client mc=client(w);Player p=new Player(mc,w);Effects fx=new Effects(w);mc.effectRenderer=fx;
            Entity target=scene%2==0?new EntityZombie(w):new EntityBoat(w);target.setPosition(scene*.25-4,64+scene*.125,scene*.5);p.func_40183_c(target);
            t("effect",fx.kind);v("sameTarget",fx.target==target?1:0);v("targetId",target.entityId);
            v("x",fx.target.posX);v("y",fx.target.posY);v("z",fx.target.posZ);
        }
    }
    static void paths()throws Exception {
        for(int scene=0;scene<10;scene++)for(float range:new float[]{0,1,4,8,16,24})for(String mob:new String[]{"Zombie","Spider","Giant"}) {
            World w=Mode_entities10.world();Entity e=EntityList.createEntityInWorld(mob,w);e.setPosition(-3.5,64,4.5);
            for(int z=-4;z<=14;z++)for(int y=64;y<=67;y++) {
                int block=scene==1?1:scene==2?9:scene==3?11:scene==4?85:scene==5?64:scene==6?71:scene==7?96:scene==8?1:0;
                if(scene!=8||z!=5)EntityOracle.rawBlock(w,0,y,z,block,0);
            }
            if(scene==9)for(int x=-2;x<=2;x++)for(int z=2;z<=7;z++)EntityOracle.rawBlock(w,x,63,z,0,0);
            PathEntity path=w.getEntityPathToXYZ(e,5,64,6,range);
            v("present",path==null?0:1);
            if(path!=null){PathPoint[] points=(PathPoint[])Mode_r10entities2.field(path,"points");v("length",points.length);for(PathPoint q:points){v("x",q.xCoord);v("y",q.yCoord);v("z",q.zCoord);}}
            Mode_r10entities2.value("rng",EntityOracle.rngState(e.rand));
        }
    }
    static void falls(String mob)throws Exception {
        for(int seed=0;seed<4;seed++)for(float distance:new float[]{-1,0,3,3.01F,4,8,30})for(boolean mounted:new boolean[]{false,true}) {
            World w=Mode_entities10.world();Entity e=EntityList.createEntityInWorld(mob,w);e.setPosition(-3.5,64,4.5);e.rand.setSeed(seed+1000);
            if(mounted){EntityZombie rider=new EntityZombie(w);rider.setPosition(-3.5,66,4.5);rider.ridingEntity=e;e.riddenByEntity=rider;w.spawnEntityInWorld(rider);}
            e.fall(distance);Mode_r10entities2.entity(e);Mode_r10entities2.worldState(w);
        }
    }
    static void respawn()throws Exception {
        for(boolean creative:new boolean[]{false,true})for(boolean keep:new boolean[]{false,true}) {
            World w=Mode_entities10.world();w.worldInfo.setSpawn(0,64,0);
            EntityOracle.FlatProvider provider=(EntityOracle.FlatProvider)w.chunkProvider;
            for(int x=-10;x<=10;x++)for(int z=-10;z<=10;z++)if(!provider.chunkExists(x,z)){
                byte[] blocks=new byte[32768];for(int bx=0;bx<16;bx++)for(int bz=0;bz<16;bz++)blocks[(bx<<11)|(bz<<7)|63]=1;
                Chunk chunk=new Chunk(w,blocks,x,z);chunk.isChunkLoaded=true;chunk.isTerrainPopulated=true;provider.chunks.put(provider.key(x,z),chunk);
            }
            Client mc=client(w);mc.session=new Session("Player","");mc.gameSettings=new GameSettings();mc.playerController=creative?new PlayerControllerCreative(mc):new PlayerControllerSP(mc);
            Player p=new Player(mc,w);mc.thePlayer=p;p.setPosition(0.5,70,0.5);p.health=7;p.currentXP=.625F;p.playerLevel=13;p.totalXP=321;p.score=456;p.inventory.currentItem=7;p.inventory.mainInventory[7]=new ItemStack(264,13,0);p.inventory.armorInventory[3]=new ItemStack(310,1,33);
            NBTTagCompound n=new NBTTagCompound();n.setInteger("foodLevel",9);n.setFloat("foodSaturationLevel",2.5F);n.setFloat("foodExhaustionLevel",3.75F);n.setInteger("foodTickTimer",37);p.getFoodStats().readStatsFromNBT(n);w.spawnEntityInWorld(p);
            mc.respawn(false,0,keep);EntityPlayer q=mc.thePlayer;
            player(q);v("sameFood",p.getFoodStats()==q.getFoodStats()?1:0);v("sameId",p.entityId==q.entityId?1:0);
            for(double x:new double[]{q.posX,q.posY,q.posZ,q.motionX,q.motionY,q.motionZ})v("position",x);v("yaw",q.rotationYaw);v("pitch",q.rotationPitch);
            q.getFoodStats().addStats(3,.1F);player(p);player(q);
        }
    }
    public static void main(String[] args)throws Exception {
        String kind=args[1],path=args[2];
        if(kind.startsWith("covered-")){Mode_r10entities2.main(new String[]{kind.substring(8),path});return;}
        Mode_r10entities2.out=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(path)));
        Mode_r10entities2.labels=new PrintWriter(path+".fields","UTF-8");Mode_r10entities2.offset=0;
        try {if(kind.startsWith("fall-"))falls(kind.substring(5));else if(kind.equals("player-copy-alias"))copy();else if(kind.equals("playersp-end-events"))endEvents();else if(kind.equals("player-end-travel"))travel();else if(kind.equals("playersp-magic-request"))magic();else if(kind.equals("pathfinder-obstacles"))paths();else if(kind.equals("player-respawn-copy"))respawn();else throw new IllegalArgumentException(kind);}
        finally {Mode_r10entities2.out.close();Mode_r10entities2.labels.close();}
    }
}
