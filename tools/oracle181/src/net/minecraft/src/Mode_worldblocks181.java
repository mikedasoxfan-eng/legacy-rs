package net.minecraft.src;

import java.io.*;
import java.util.*;

/** Focused world/block checks that call the original b1.8.1 bytecode. */
public final class Mode_worldblocks181 {
    static World fixture(boolean bright) {
        World w=Oracle.newWorld(0);
        w.chunkProvider=new EntityOracle.FlatProvider(w);
        w.rand.setSeed(300);
        for(Chunk c:((EntityOracle.FlatProvider)w.chunkProvider).chunks.values()) {
            Arrays.fill(c.heightMap,(byte)(bright?64:81));
            if(!bright)for(int x=0;x<16;x++)for(int z=0;z<16;z++)c.blocks[(x<<11)|(z<<7)|80]=1;
            if(bright) Arrays.fill(c.skylightMap.data,(byte)255);
        }
        return w;
    }
    static void raw(World w,int x,int y,int z,int id,int m) {EntityOracle.rawBlock(w,x,y,z,id,m);}
    static void i(DataOutputStream o,int n)throws Exception {EntityOracle.i(o,n);}
    static void l(DataOutputStream o,long n)throws Exception {EntityOracle.l(o,n);}
    static void box(DataOutputStream o,AxisAlignedBB b)throws Exception {
        o.writeBoolean(b!=null);
        if(b!=null) for(double d:new double[]{b.minX,b.minY,b.minZ,b.maxX,b.maxY,b.maxZ})EntityOracle.d(o,d);
    }
    static void plants(DataOutputStream o)throws Exception {
        for(int id:new int[]{104,105,106,39,40,18,60})for(int config=0;config<16;config++) {
            World w=fixture(id==104||id==105||id==18||id==60);
            w.rand.setSeed(1729+config*31+id);
            for(int x=3;x<=13;x++)for(int z=3;z<=13;z++)raw(w,x,63,z,id==104||id==105||id==60?60:1,config&1);
            raw(w,8,64,8,id,config);
            if(id==104||id==105) {
                if((config&2)!=0)raw(w,7,64,8,id,7);
                if((config&4)!=0)raw(w,8,64,7,id,7);
                if((config&8)!=0)raw(w,9,64,9,id,7);
            }
            if(id==106) {
                for(int k=0;k<4;k++) if((config&(1<<k))!=0) {
                    int dx=ModelBed.field_35871_a[k],dz=ModelBed.field_35870_b[k];
                    for(int y=62;y<=68;y++)raw(w,8+dx,y,8+dz,1,0);
                }
                raw(w,8,65,8,config==0?1:0,0);
                if(config==15)for(int x=4;x<=7;x++)raw(w,x,64,4,106,1);
            }
            if(id==39||id==40) {
                if((config&1)!=0)for(int x=4;x<=7;x++)raw(w,x,64,4,id,0);
                if((config&2)!=0)raw(w,9,64,8,1,0);
            }
            if(id==18 && (config&1)!=0)raw(w,8,64,9,17,0);
            if(id==60) {
                raw(w,8,63,8,1,0);
                if(config<4)raw(w,8,65,8,new int[]{0,59,104,105}[config],0);
                if((config&4)!=0)raw(w,10,64,8,9,0);
            }
            for(int t=0;t<128;t++) {
                if(w.getBlockId(8,64,8)==id)Block.blocksList[id].updateTick(w,8,64,8,w.rand);
                l(o,EntityOracle.rngState(w.rand));
                for(int x=3;x<=13;x++)for(int z=3;z<=13;z++)for(int y=62;y<=68;y++) {
                    o.writeByte(w.getBlockId(x,y,z));o.writeByte(w.getBlockMetadata(x,y,z));
                }
            }
        }
    }
    static void geometry(DataOutputStream o)throws Exception {
        World w=fixture(false);
        for(int id=97;id<=109;id++)for(int meta=0;meta<16;meta++)for(int mask=0;mask<16;mask++)for(int kind=0;kind<3;kind++) {
            raw(w,8,64,8,id,meta);
            for(int k=0;k<4;k++)raw(w,8+ModelBed.field_35871_a[k],64,8+ModelBed.field_35870_b[k],(mask&(1<<k))==0?0:kind==0?1:kind==1?20:id,0);
            raw(w,8,65,8,mask==0?1:0,0);
            Block b=Block.blocksList[id];
            ArrayList<AxisAlignedBB> boxes=new ArrayList<>();
            b.getCollidingBoundingBoxes(w,8,64,8,AxisAlignedBB.getBoundingBox(8,64,8,9,66,9),boxes);
            i(o,boxes.size());for(AxisAlignedBB bb:boxes)box(o,bb);
            box(o,b.getCollisionBoundingBoxFromPool(w,8,64,8));
            b.setBlockBoundsBasedOnState(w,8,64,8);
            box(o,AxisAlignedBB.getBoundingBox(b.minX,b.minY,b.minZ,b.maxX,b.maxY,b.maxZ));
        }
    }
    static void drops(DataOutputStream o)throws Exception {
        World w=fixture(false);OracleHooks.reset(100);EntityOracle.mathRandom().setSeed(12345);
        for(int id:new int[]{53,67,97,98,99,100,101,102,103,104,105,106,107,108,109})for(int meta=0;meta<16;meta++)for(int seed=0;seed<32;seed++) {
            w.loadedEntityList.clear();w.rand.setSeed(seed);
            Block.blocksList[id].dropBlockAsItemWithChance(w,8,64,8,meta,0.75F);
            l(o,EntityOracle.rngState(w.rand));i(o,w.loadedEntityList.size());
            for(Object obj:w.loadedEntityList) {
                EntityItem e=(EntityItem)obj;
                i(o,e.item.itemID);i(o,e.item.stackSize);i(o,e.item.getItemDamage());
                EntityOracle.d(o,e.posX);EntityOracle.d(o,e.posY);EntityOracle.d(o,e.posZ);
            }
        }
    }
    static void saves(DataOutputStream o)throws Exception {
        for(int game=-1;game<=2;game++)for(int features=-1;features<=1;features++)for(int player=0;player<2;player++) {
            NBTTagCompound n=new NBTTagCompound();n.setLong("RandomSeed",-4172144997902289642L);n.setInteger("GameType",game);
            if(features>=0)n.setBoolean("MapFeatures",features!=0);
            n.setInteger("SpawnX",-22);n.setInteger("SpawnY",64);n.setInteger("SpawnZ",900);
            n.setLong("Time",23999);n.setLong("SizeOnDisk",123456789012L);n.setInteger("version",19132);
            n.setInteger("rainTime",71);n.setInteger("thunderTime",17);n.setBoolean("raining",true);
            if(player!=0){NBTTagCompound p=new NBTTagCompound();p.setInteger("Dimension",-1);n.setCompoundTag("Player",p);}
            WorldInfo info=new WorldInfo(new WorldInfo(n));
            NBTTagCompound out=info.getNBTTagCompound();
            CompressedStreamTools.writeGzippedCompoundToOutputStream(out,new FileOutputStream("wb-level.dat"));
            out=CompressedStreamTools.loadGzippedCompoundFromOutputStream(new FileInputStream("wb-level.dat"));
            info=new WorldInfo(out);
            l(o,info.getRandomSeed());i(o,info.func_35918_q());o.writeBoolean(info.func_35917_r());
            i(o,info.getSpawnX());i(o,info.getSpawnY());i(o,info.getSpawnZ());l(o,info.getWorldTime());l(o,info.getSizeOnDisk());i(o,info.getDimension());
            i(o,out.getInteger("version"));i(o,out.getInteger("rainTime"));i(o,out.getInteger("thunderTime"));o.writeBoolean(out.getBoolean("raining"));o.writeBoolean(out.getBoolean("thundering"));
            o.writeBoolean(out.hasKey("GameType"));o.writeBoolean(out.hasKey("MapFeatures"));
        }
        new File("wb-level.dat").delete();
    }
    static void lights(DataOutputStream o)throws Exception {
        World w=fixture(false);
        for(Chunk c:((EntityOracle.FlatProvider)w.chunkProvider).chunks.values())c.generateSkylightMap();
        Random r=new Random(1007);int[] ids={0,1,18,20,89,98,101,102,103,107,109};
        for(int t=0;t<80;t++) {
            int x=r.nextInt(24)-4,y=64+r.nextInt(22),z=r.nextInt(24)-4,id=ids[r.nextInt(ids.length)],m=r.nextInt(16);
            o.writeBoolean(w.setBlockAndMetadata(x,y,z,id,m));
            if(t%8==0)w.getChunkFromChunkCoords(x>>4,z>>4).func_35841_j();
            for(int bx=-8;bx<=24;bx++)for(int bz=-8;bz<=24;bz++)for(int by=59;by<=88;by++) {
                o.writeByte(w.getBlockId(bx,by,bz));o.writeByte(w.getBlockMetadata(bx,by,bz));
                o.writeByte(w.getSavedLightValue(EnumSkyBlock.Sky,bx,by,bz));o.writeByte(w.getSavedLightValue(EnumSkyBlock.Block,bx,by,bz));
            }
            for(int cx=-1;cx<=1;cx++)for(int cz=-1;cz<=1;cz++) {
                Chunk c=w.getChunkFromChunkCoords(cx,cz);o.write(c.heightMap);
                for(boolean v:c.field_35844_d)o.writeBoolean(v);
            }
            i(o,w.func_35451_b(x,y,z,t%16));
        }
        WorldProvider[] providers={WorldProvider.getProviderForDimension(0),new WorldProviderHell()};
        for(WorldProvider p:providers){p.registerWorld(w);for(float f:p.lightBrightnessTable)EntityOracle.f(o,f);}
    }

    static void chests(DataOutputStream o)throws Exception {
        for(int pair=0;pair<5;pair++)for(int mask=0;mask<16;mask++)for(int meta=0;meta<6;meta++)for(float yaw:new float[]{-405,-90,0,90,180,270,405}) {
            World w=fixture(true);raw(w,8,64,8,54,0);
            int dx=new int[]{0,0,0,-1,1}[pair],dz=new int[]{0,-1,1,0,0}[pair];
            if(pair!=0)raw(w,8+dx,64,8+dz,54,meta);
            int[] xs={0,0,-1,1},zs={-1,1,0,0};
            for(int k=0;k<4;k++)if(w.getBlockId(8+xs[k],64,8+zs[k])!=54)raw(w,8+xs[k],64,8+zs[k],(mask&(1<<k))!=0?1:0,0);
            ((BlockChest)Block.chest).func_35306_h(w,8,64,8);i(o,w.getBlockMetadata(8,64,8));
            EntityLiving e=new EntityPig(w);e.rotationYaw=yaw;Block.chest.onBlockPlacedBy(w,8,64,8,e);
            i(o,w.getBlockMetadata(8,64,8));i(o,w.getBlockMetadata(8+dx,64,8+dz));
        }
        java.lang.reflect.Field ticks=TileEntityChest.class.getDeclaredField("field_35154_q");ticks.setAccessible(true);
        for(int pair=0;pair<5;pair++) {
            World w=fixture(true);w.rand.setSeed(4242);w.setBlock(8,64,8,54);
            int dx=new int[]{0,0,0,-1,1}[pair],dz=new int[]{0,-1,1,0,0}[pair];
            if(pair!=0)w.setBlock(8+dx,64,8+dz,54);
            final ArrayList<Object[]> sounds=new ArrayList<>();
            w.addWorldAccess((IWorldAccess)java.lang.reflect.Proxy.newProxyInstance(IWorldAccess.class.getClassLoader(),new Class[]{IWorldAccess.class},(proxy,m,a)->{if(m.getName().equals("playSound"))sounds.add(a);return null;}));
            TileEntityChest c=(TileEntityChest)w.getBlockTileEntity(8,64,8);
            for(int t=0;t<160;t++) {
                if(t==0||t==50||t==75||t==110)c.func_35142_x_();
                if(t==20||t==30||t==65||t==95||t==140)c.func_35141_y_();
                if(t==35)w.playNoteAt(8,64,8,1,0);
                if(t==100)c.func_35144_b();
                c.updateEntity();o.writeBoolean(c.field_35155_a);
                for(TileEntityChest adj:new TileEntityChest[]{c.field_35152_b,c.field_35153_c,c.field_35150_d,c.field_35151_e})o.writeBoolean(adj!=null);
                EntityOracle.f(o,c.field_35148_f);EntityOracle.f(o,c.field_35149_g);i(o,c.field_35156_h);i(o,ticks.getInt(c));l(o,EntityOracle.rngState(w.rand));
                i(o,sounds.size());for(Object[] sound:sounds){o.writeBoolean(sound[0].equals("random.door_open"));for(int k=1;k<=3;k++)EntityOracle.d(o,(Double)sound[k]);EntityOracle.f(o,(Float)sound[4]);EntityOracle.f(o,(Float)sound[5]);}sounds.clear();
            }
        }
    }


    static void helpers(DataOutputStream o)throws Exception {
        for(double v:new double[]{Double.NaN,Double.NEGATIVE_INFINITY,Double.POSITIVE_INFINITY,-1e30,1e30,-9223372036854775808.0,9223372036854775808.0,-2147483649.0,2147483648.0,-123.5,-1.0,-0.1,-0.0,0.0,0.1,1.0,123.5}) {
            l(o,MathHelper.func_35599_c(v));i(o,MathHelper.floor_double(v));i(o,MathHelper.floor_float((float)v));
        }
        for(int v:new int[]{Integer.MIN_VALUE,Integer.MIN_VALUE+1,-100,-1,0,1,100,Integer.MAX_VALUE})i(o,MathHelper.func_35597_a(v));
        Random r=new Random(6543);
        for(int t=0;t<1000;t++)for(int[] range:new int[][]{{-999,2000},{0,1},{1,1},{5,-5},{-7,-6},{2147483600,2147483647}}) {
            i(o,MathHelper.func_35598_a(r,range[0],range[1]));l(o,EntityOracle.rngState(r));
        }
        for(int bits=4;bits<=8;bits++) {
            NibbleArray a=new NibbleArray(256<<bits,bits);
            for(int x=0;x<16;x++)for(int z=0;z<16;z++)for(int y=0;y<1<<bits;y++)a.setNibble(x,y,z,x*100+z*10+y-200);
            for(int x=0;x<16;x++)for(int z=0;z<16;z++)for(int y=0;y<1<<bits;y++)o.writeByte(a.getNibble(x,y,z));
            o.write(a.data);a=new NibbleArray(a.data,bits);i(o,a.getNibble(15,0,15));
        }
        World w=fixture(true);Chunk c=w.getChunkFromChunkCoords(0,0);
        for(int id:new int[]{0,1,8,18,30,78,101,102,104,106})for(int y:new int[]{0,1,62,64,127}) {
            for(int yy=0;yy<128;yy++)c.blocks[(8<<11)|(8<<7)|yy]=0;c.field_35845_c[136]=-999;
            raw(w,8,y,8,id,0);i(o,w.func_35461_e(8,8));
            w.setBlock(8,y,8,0);i(o,w.func_35461_e(8,8));
            w.setBlock(8,127,8,1);i(o,w.func_35461_e(8,8));
            w.setBlock(8,127,8,1);i(o,w.func_35461_e(8,8));
        }
    }


    static void fluids(DataOutputStream o)throws Exception {
        World w=fixture(true);Random r=new Random(567);final ArrayList<Object[]> events=new ArrayList<>();
        w.addWorldAccess((IWorldAccess)java.lang.reflect.Proxy.newProxyInstance(IWorldAccess.class.getClassLoader(),new Class[]{IWorldAccess.class},(proxy,m,a)->{if(m.getName().equals("playSound")||m.getName().equals("spawnParticle"))events.add(a);return null;}));
        for(int id:new int[]{8,9,10,11})for(int meta=0;meta<16;meta++)for(int above:new int[]{0,1,8,79}) {
            raw(w,8,64,8,id,meta);raw(w,8,65,8,above,0);Block b=Block.blocksList[id];
            for(int t=0;t<200;t++) {
                b.randomDisplayTick(w,8,64,8,r);l(o,EntityOracle.rngState(r));i(o,events.size());
                for(Object[] a:events){boolean sound=a.length==6;o.writeByte(sound?0:1);byte[] name=((String)a[0]).getBytes("UTF-8");i(o,name.length);o.write(name);for(int k=1;k<=3;k++)EntityOracle.d(o,(Double)a[k]);for(int k=4;k<a.length;k++)EntityOracle.d(o,((Number)a[k]).doubleValue());}events.clear();
            }
            i(o,b.func_35275_c(w,8,64,8));for(int side=0;side<6;side++)o.writeBoolean(b.getIsBlockSolid(w,8,65,8,side));
        }
    }


    static void teleporter(DataOutputStream o)throws Exception {
        java.lang.reflect.Field rand=Teleporter.class.getDeclaredField("random");rand.setAccessible(true);
        for(int config=0;config<6;config++) {
            World w=fixture(true);EntityOracle.FlatProvider provider=(EntityOracle.FlatProvider)w.chunkProvider;
            for(int cx=-9;cx<=9;cx++)for(int cz=-9;cz<=9;cz++) {
                byte[] blocks=new byte[32768];for(int x=0;x<16;x++)for(int z=0;z<16;z++)blocks[(x<<11)|(z<<7)|63]=(byte)(config==2?0:config==3?9:1);
                Chunk c=new Chunk(w,blocks,cx,cz);Arrays.fill(c.skylightMap.data,(byte)255);Arrays.fill(c.heightMap,(byte)64);c.isTerrainPopulated=true;c.isChunkLoaded=true;provider.chunks.put(provider.key(cx,cz),c);
            }
            w.rand.setSeed(19);
            if(config>=4)for(int y=64;y<=66;y++)for(int v=0;v<2;v++)raw(w,config==4?5+v:5,y,config==4?6:6+v,90,0);
            Entity e=new EntityPig(w);e.setPosition(8.25,config==1?120:70,8.75);
            Teleporter t=new Teleporter();Random r=(Random)rand.get(t);r.setSeed(77+config);t.placeInPortal(w,e);
            EntityOracle.d(o,e.posX);EntityOracle.d(o,e.posY-e.yOffset);EntityOracle.d(o,e.posZ);l(o,EntityOracle.rngState(r));l(o,EntityOracle.rngState(w.rand));
            for(int x=-12;x<=28;x++)for(int z=-12;z<=28;z++)for(int y=35;y<=90;y++){o.writeByte(w.getBlockId(x,y,z));o.writeByte(w.getBlockMetadata(x,y,z));}
        }
    }


    static void items(DataOutputStream o)throws Exception {
        for(int id:new int[]{295,361,362})for(int side=0;side<6;side++)for(int soil:new int[]{1,60})for(int above:new int[]{0,1}) {
            World w=fixture(true);EntityPlayer p=PlayerOracle.player(w);raw(w,8,64,8,soil,0);raw(w,8,65,8,above,0);ItemStack s=new ItemStack(id,4,0);
            o.writeBoolean(s.getItem().onItemUse(s,p,w,8,64,8,side));i(o,s.stackSize);o.writeByte(w.getBlockId(8,65,8));i(o,w.getBlockMetadata(8,65,8));
        }
        for(int meta=0;meta<16;meta++)for(float yaw:new float[]{-450,-90,0,90,180,270,450}) {
            World w=fixture(true);EntityPlayer p=PlayerOracle.player(w);p.rotationYaw=yaw;raw(w,8,64,8,107,meta);
            o.writeBoolean(Block.blocksList[107].blockActivated(w,8,64,8,p));i(o,w.getBlockMetadata(8,64,8));Block.blocksList[107].onBlockPlacedBy(w,8,64,8,p);i(o,w.getBlockMetadata(8,64,8));
        }
        for(int id:new int[]{59,104,105})for(int meta=0;meta<8;meta++) {
            World w=fixture(true);EntityPlayer p=PlayerOracle.player(w);raw(w,8,63,8,60,0);raw(w,8,64,8,id,meta);ItemStack s=new ItemStack(351,4,15);
            o.writeBoolean(s.getItem().onItemUse(s,p,w,8,64,8,1));i(o,s.stackSize);i(o,w.getBlockMetadata(8,64,8));
        }
        for(int id:new int[]{0,1,97,109,260,351,359,360,361,362,367,2256,2257,2258,12345,30000})for(int count:new int[]{-128,-1,0,1,127}) {
            NBTTagCompound n=new NBTTagCompound();n.setShort("id",(short)id);n.setByte("Count",(byte)count);n.setShort("Damage",(short)-123);ItemStack s=ItemStack.func_35864_a(n);
            o.writeBoolean(s!=null);if(s!=null){i(o,s.itemID);i(o,s.stackSize);i(o,s.getItemDamage());}
        }
        for(int id:new int[]{18,106})for(int meta=0;meta<16;meta++) {
            World w=fixture(true);EntityPlayer p=PlayerOracle.player(w);w.rand.setSeed(42);EntityOracle.mathRandom().setSeed(123);OracleHooks.reset(100);p.inventory.mainInventory[0]=new ItemStack(359,1,0);
            Block.blocksList[id].harvestBlock(w,p,8,64,8,meta);l(o,EntityOracle.rngState(w.rand));i(o,w.loadedEntityList.size());
            for(Object obj:w.loadedEntityList){EntityItem e=(EntityItem)obj;i(o,e.item.itemID);i(o,e.item.stackSize);i(o,e.item.getItemDamage());EntityOracle.d(o,e.posX);EntityOracle.d(o,e.posY);EntityOracle.d(o,e.posZ);}
        }
    }

    public static void main(String[] a)throws Exception {
        int initialized=Block.stone.blockID;
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[2])))) {
            switch(a[1]) {
                case "plants":plants(o);break;
                case "geometry":geometry(o);break;
                case "drops":drops(o);break;
                case "saves":saves(o);break;
                case "lights":lights(o);break;
                case "chests":chests(o);break;
                case "helpers":helpers(o);break;
                case "fluids":fluids(o);break;
                case "teleporter":teleporter(o);break;
                case "items":items(o);break;
                default:throw new IllegalArgumentException(a[1]);
            }
        }
    }
}

