package net.minecraft.src;
import java.io.*;
import java.lang.reflect.*;
import java.util.*;

/** Release block callbacks: dragon-egg falling, random ticks (nether wart, mycelium, grass, vine, stems, End fire),
 * onBlockAdded (pumpkin snowman, End portal, fire portal guard), GUI activation, brewing-stand removal,
 * splash-potion dispensing, display ticks (fluid drips, portal ambience) and material mobility. */
public final class Mode_wbb4ticks {
    static final class GuiPlayer extends EntityPlayerSP {
        int enchant,brewing;
        GuiPlayer(net.minecraft.client.Minecraft mc,World w){super(mc,w,new Session("Player",""),0);}
        public void displayGUIEnchantment(int x,int y,int z){enchant++;}
        public void displayGUIBrewingStand(TileEntityBrewingStand t){brewing++;}
        public void addStat(StatBase s,int n){}
    }
    static final class EndPlayer extends EntityPlayerSP {
        int calls;
        EndPlayer(net.minecraft.client.Minecraft mc,World w){super(mc,w,new Session("Player",""),0);}
        public void func_40182_b(int d){calls+=d==1?1:100;}
        public void addStat(StatBase s,int n){}
    }
    static void endCollision(DataOutputStream o)throws Exception {
        for(int who=0;who<4;who++)for(boolean remote:new boolean[]{false,true}) {
            World w=Mode_worldblocks10.fixture(true);raw(w,8,64,8,119,0);
            EndPlayer p=new EndPlayer((net.minecraft.client.Minecraft)Oracle.allocate(PlayerOracle.HeadlessMinecraft.class),w);Blocks4Oracle.place(p,8.5,64,8.5,0F,0F);
            Entity pig=EntityList.createEntityInWorld("Pig",w);Entity zombie=EntityList.createEntityInWorld("Zombie",w);
            if(who==1)p.ridingEntity=pig;if(who==2)p.riddenByEntity=pig;
            w.multiplayerWorld=remote;Blocks4Oracle.seeds(w);
            Block.blocksList[119].onEntityCollidedWithBlock(w,8,64,8,who==3?zombie:p);
            Blocks4Oracle.i(o,p.calls);
        }
    }
    static void provider(World w,int dimension)throws Exception {
        if(dimension==0)return;
        WorldProvider p=dimension==-1?new WorldProviderHell():new WorldProviderEnd();
        Field f=World.class.getDeclaredField("worldProvider");f.setAccessible(true);f.set(w,p);p.registerWorld(w);
    }
    static void state(DataOutputStream o,World w,int x,int y,int z)throws Exception {
        for(int dx=-2;dx<=2;dx++)for(int dy=-3;dy<=2;dy++)for(int dz=-2;dz<=2;dz++){Trace10.writeByte(o,"id",w.getBlockId(x+dx,y+dy,z+dz));Trace10.writeByte(o,"meta",w.getBlockMetadata(x+dx,y+dy,z+dz));}
        Blocks4Oracle.queue(o,w);Blocks4Oracle.entities(o,w);Blocks4Oracle.rngs(o,w);WorldEventOracle.snapshot(w,o);
        Blocks4Oracle.i(o,Trace10.namedInt("loadedTileEntityList.size()",w.loadedTileEntityList.size()));
    }
    static void raw(World w,int x,int y,int z,int id,int m){Mode_worldblocks10.raw(w,x,y,z,id,m);}
    static void egg(DataOutputStream o)throws Exception {
        for(int hook=0;hook<3;hook++)for(boolean instant:new boolean[]{false,true})for(int x:new int[]{8,40})for(int y:new int[]{64,20})for(int below:new int[]{1,0,9,51,11,78})for(boolean remote:new boolean[]{false,true}) {
            World w=Mode_worldblocks10.fixture(true);
            if(y==20)raw(w,x,10,8,1,0);
            raw(w,x,y-1,8,below,0);raw(w,x,y,8,122,5);
            w.multiplayerWorld=remote;BlockSand.fallInstantly=instant;Blocks4Oracle.seeds(w);
            Block b=Block.blocksList[122];
            if(hook==0)b.onBlockAdded(w,x,y,8);else if(hook==1)b.onNeighborBlockChange(w,x,y,8,1);else b.updateTick(w,x,y,8,w.rand);
            BlockSand.fallInstantly=false;
            state(o,w,x,y,8);for(int yy=y-11;yy<y;yy++){Trace10.writeByte(o,"id",w.getBlockId(x,yy,8));}
        }
    }
    static void setup(World w,int id,int s) {
        if(id==110||id==2){
            if(s==1)raw(w,8,65,8,1,0);if(s==2)raw(w,8,65,8,20,0);
            for(int[] d:new int[][]{{7,63,8},{9,64,8},{8,62,9},{7,64,7},{9,63,9},{8,61,7}})raw(w,d[0],d[1],d[2],3,0);
        } else if(id==106) {
            if(s!=3){raw(w,8,64,9,1,0);raw(w,8,65,9,1,0);}
            if(s==1){raw(w,8,63,8,0,0);raw(w,8,62,8,0,0);}
            if(s==2){raw(w,9,65,8,1,0);raw(w,7,65,8,1,0);raw(w,9,64,9,1,0);raw(w,7,64,9,1,0);}
        } else if(id==104||id==105) {
            raw(w,8,63,8,60,s==1?7:0);raw(w,7,63,8,3,0);raw(w,9,63,8,2,0);raw(w,8,63,7,60,0);raw(w,8,63,9,1,0);
            if(s==2)raw(w,9,64,8,id==104?86:103,0);
            if(s==3){raw(w,7,64,7,59,3);raw(w,7,63,7,60,7);raw(w,9,64,9,104,2);raw(w,9,63,9,60,0);}
        } else if(id==115) {
            raw(w,8,63,8,s==1?3:88,0);
        } else if(id==51) {
            raw(w,8,63,8,new int[]{7,87,1,0}[s],0);raw(w,9,64,8,5,0);raw(w,8,65,8,17,0);
        }
    }
    static void ticks(DataOutputStream o)throws Exception {
        int[][] cases={{115,0},{115,2},{115,3},{110,0},{2,0},{106,1},{106,2},{106,4},{106,8},{106,5},{106,15},{106,0},{104,0},{104,6},{104,7},{105,0},{105,7},{51,0},{51,5},{51,15}};
        for(int[] c:cases)for(int s=0;s<4;s++)for(int dimension:new int[]{0,-1,1})for(boolean bright:new boolean[]{true,false})for(long seed=0;seed<6;seed++)for(boolean remote:new boolean[]{false,true}) {
            if(dimension!=0&&c[0]!=115&&c[0]!=51)continue;
            if(!bright&&c[0]!=110&&c[0]!=2)continue;
            World w=Mode_worldblocks10.fixture(bright);provider(w,dimension);setup(w,c[0],s);raw(w,8,64,8,c[0],c[1]);
            w.multiplayerWorld=remote;Blocks4Oracle.seeds(w);w.rand.setSeed(seed*31+7);
            Block.blocksList[c[0]].updateTick(w,8,64,8,w.rand);
            state(o,w,8,64,8);
        }
    }
    static void added(DataOutputStream o)throws Exception {
        Field allow=BlockEndPortal.class.getDeclaredField("bossDefeated");
        for(int id:new int[]{86,91})for(int snow=0;snow<4;snow++)for(boolean remote:new boolean[]{false,true}) {
            World w=Mode_worldblocks10.fixture(true);
            if((snow&1)!=0)raw(w,8,63,8,80,0);if((snow&2)!=0)raw(w,8,62,8,80,0);raw(w,8,64,8,id,2);
            w.multiplayerWorld=remote;Blocks4Oracle.seeds(w);Block.blocksList[id].onBlockAdded(w,8,64,8);state(o,w,8,64,8);
        }
        for(boolean flag:new boolean[]{false,true})for(int dimension:new int[]{0,-1,1})for(boolean remote:new boolean[]{false,true}) {
            World w=Mode_worldblocks10.fixture(true);provider(w,dimension);raw(w,8,64,8,119,0);
            w.multiplayerWorld=remote;allow.setBoolean(null,flag);Blocks4Oracle.seeds(w);Block.blocksList[119].onBlockAdded(w,8,64,8);allow.setBoolean(null,false);state(o,w,8,64,8);
        }
        for(int frame=0;frame<3;frame++)for(int dimension:new int[]{0,-1,1})for(boolean remote:new boolean[]{false,true}) {
            World w=Mode_worldblocks10.fixture(true);provider(w,dimension);
            if(frame>=1)raw(w,8,63,8,49,0);
            if(frame==2){for(int x=7;x<=10;x++){raw(w,x,63,8,49,0);raw(w,x,67,8,49,0);}for(int y=64;y<=66;y++){raw(w,7,y,8,49,0);raw(w,10,y,8,49,0);}}
            raw(w,8,64,8,51,0);w.multiplayerWorld=remote;Blocks4Oracle.seeds(w);Block.blocksList[51].onBlockAdded(w,8,64,8);state(o,w,8,64,8);
        }
    }
    static void activation(DataOutputStream o)throws Exception {
        for(int id:new int[]{116,117})for(boolean tile:new boolean[]{false,true})for(boolean remote:new boolean[]{false,true}) {
            World w=Mode_worldblocks10.fixture(true);
            GuiPlayer p=new GuiPlayer((net.minecraft.client.Minecraft)Oracle.allocate(PlayerOracle.HeadlessMinecraft.class),w);
            raw(w,8,64,8,id,0);if(tile)w.getBlockTileEntity(8,64,8);
            w.multiplayerWorld=remote;Blocks4Oracle.seeds(w);
            Blocks4Oracle.bool(o,"blockActivated",Block.blocksList[id].blockActivated(w,8,64,8,p));Blocks4Oracle.i(o,p.enchant);Blocks4Oracle.i(o,p.brewing);state(o,w,8,64,8);
        }
    }
    static void removal(DataOutputStream o)throws Exception {
        Field rng=BlockBrewingStand.class.getDeclaredField("field_40214_a");rng.setAccessible(true);Random r=(Random)rng.get(Block.brewingStand);
        int[][] fills={{},{0,373,1,8193},{3,372,64,0},{0,373,1,16385,1,373,1,0,2,373,1,8204,3,331,45,0},{3,348,0,0},{1,373,-3,5}};
        for(int f=0;f<fills.length;f++)for(long seed=0;seed<4;seed++)for(int hook=0;hook<2;hook++) {
            World w=Mode_worldblocks10.fixture(true);w.setBlockWithNotify(8,64,8,117);
            TileEntityBrewingStand t=(TileEntityBrewingStand)w.getBlockTileEntity(8,64,8);
            for(int n=0;n+3<fills[f].length;n+=4)t.setInventorySlotContents(fills[f][n],new ItemStack(fills[f][n+1],fills[f][n+2],fills[f][n+3]));
            Blocks4Oracle.seeds(w);r.setSeed(seed);
            if(hook==0)w.setBlockWithNotify(8,64,8,0);else w.setBlockWithNotify(8,64,8,1);
            EntityOracle.l(o,Trace10.namedLong("brewing rng",EntityOracle.rngState(r)));state(o,w,8,64,8);
            for(int n=0;n<4;n++)Blocks4Oracle.stack(o,t.getStackInSlot(n));Blocks4Oracle.bool(o,"t.isInvalid()",t.isInvalid());
        }
        for(int id:new int[]{116,119})for(int hook=0;hook<2;hook++) {
            World w=Mode_worldblocks10.fixture(true);BlockEndPortal.bossDefeated=true;w.setBlockWithNotify(8,64,8,id);BlockEndPortal.bossDefeated=false;
            TileEntity t=w.getBlockTileEntity(8,64,8);Blocks4Oracle.seeds(w);
            if(hook==0)w.setBlockWithNotify(8,64,8,0);else w.setBlockWithNotify(8,64,8,116+119-id);
            state(o,w,8,64,8);Blocks4Oracle.bool(o,"t!=null",t!=null);if(t!=null)Blocks4Oracle.bool(o,"t.isInvalid()",t.isInvalid());
            TileEntity now=w.getBlockTileEntity(8,64,8);Blocks4Oracle.bool(o,"now==t",now==t);WorldEventOracle.text(o,now==null?"null":now.getClass().getSimpleName());
        }
    }
    static void dispense(DataOutputStream o)throws Exception {
        Field random=TileEntityDispenser.class.getDeclaredField("dispenserRandom");random.setAccessible(true);
        Method dispense=BlockDispenser.class.getDeclaredMethod("dispenseItem",World.class,int.class,int.class,int.class,Random.class);dispense.setAccessible(true);
        int[] damages={16385,8193,16396,0,32767,16384};
        for(int facing=2;facing<6;facing++)for(int config=0;config<8;config++)for(boolean remote:new boolean[]{false,true}) {
            World w=Mode_worldblocks10.fixture(true);w.setBlock(8,64,8,23);w.setBlockMetadata(8,64,8,facing);
            TileEntityDispenser d=new TileEntityDispenser();for(int n=0;n<9;n++)if(((config>>(n%3))&1)!=0)d.setInventorySlotContents(n,new ItemStack(373,1+n%2,damages[(n+config)%damages.length]));
            w.setBlockTileEntity(8,64,8,d);((Random)random.get(d)).setSeed(55+config);
            w.multiplayerWorld=remote;Blocks4Oracle.seeds(w);
            for(int k=0;k<4;k++){dispense.invoke(Block.dispenser,w,8,64,8,w.rand);for(int n=0;n<9;n++)Blocks4Oracle.stack(o,d.getStackInSlot(n));EntityOracle.l(o,Trace10.namedLong("dispenserRandom",EntityOracle.rngState((Random)random.get(d))));state(o,w,8,64,8);}
        }
        for(int config=0;config<4;config++)for(boolean remote:new boolean[]{false,true}) {
            World w=Mode_worldblocks10.fixture(true);w.setBlock(8,64,8,23);w.setBlockMetadata(8,64,8,3);
            if((config&1)!=0)raw(w,8,64,7,76,5);if((config&2)!=0)raw(w,8,65,8,1,0);
            w.multiplayerWorld=remote;Blocks4Oracle.seeds(w);Block.dispenser.onNeighborBlockChange(w,8,64,8,76);state(o,w,8,64,8);
        }
    }
    static void display(DataOutputStream o)throws Exception {
        for(int id:new int[]{8,9,10,11,90,110})for(int meta:new int[]{0,3})for(int config=0;config<4;config++)for(long seed=0;seed<24;seed++) {
            World w=Mode_worldblocks10.fixture(true);
            raw(w,8,64,8,id,meta);if((config&1)!=0)raw(w,8,63,8,0,0);if((config&2)!=0)raw(w,8,62,8,1,0);
            WorldEventOracle.attach(w);Random r=new Random(seed);Block.blocksList[id].randomDisplayTick(w,8,64,8,r);
            EntityOracle.l(o,Trace10.namedLong("r",EntityOracle.rngState(r)));WorldEventOracle.snapshot(w,o);
        }
        for(int id=0;id<256;id++){Block b=Block.blocksList[id];Blocks4Oracle.bool(o,"b!=null",b!=null);if(b!=null){Blocks4Oracle.i(o,b.getMobilityFlag());Blocks4Oracle.i(o,b.blockMaterial.materialMapColor.colorIndex);Material m=b.blockMaterial;for(boolean v:new boolean[]{m.getIsLiquid(),m.isSolid(),m.getIsSolid(),m.getIsOpaque(),m.getCanBurn(),m.getIsGroundCover(),m.getCanBlockGrass()})Blocks4Oracle.bool(o,"material",v);}}
    }
    public static void main(String[] a)throws Exception {
        int init=Block.stone.blockID;
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            egg(o);ticks(o);added(o);activation(o);endCollision(o);removal(o);dispense(o);display(o);
        }
    }
}
