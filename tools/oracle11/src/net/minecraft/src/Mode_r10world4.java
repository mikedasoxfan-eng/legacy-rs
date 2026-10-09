package net.minecraft.src;

import java.io.*;
import java.util.*;

/** Round four: all expected results come from original Release game methods. */
public final class Mode_r10world4 {
    static final String[] KEYS={"xPos","zPos","Blocks","Data","Aa","BB","Health","Items","Pos","Motion","Level","Player","name","\u0000","\u00e9","\ud83d\ude00"};
    static NBTBase tag(Random r,int type,int depth) {
        NBTBase t=NBTBase.createTagOfType((byte)type,"");
        switch(type) {
        case 1: ((NBTTagByte)t).byteValue=(byte)r.nextInt();break;
        case 2: ((NBTTagShort)t).shortValue=(short)r.nextInt();break;
        case 3: ((NBTTagInt)t).intValue=r.nextInt();break;
        case 4: ((NBTTagLong)t).longValue=r.nextLong();break;
        case 5: ((NBTTagFloat)t).floatValue=Float.intBitsToFloat(r.nextInt());break;
        case 6: ((NBTTagDouble)t).doubleValue=Double.longBitsToDouble(r.nextLong());break;
        case 7: {byte[] b=new byte[r.nextInt(65)];r.nextBytes(b);((NBTTagByteArray)t).byteArray=b;break;}
        case 8: ((NBTTagString)t).stringValue=KEYS[r.nextInt(KEYS.length)]+KEYS[r.nextInt(KEYS.length)];break;
        case 9: {int ty=1+r.nextInt(depth>0?10:8),n=r.nextInt(9);for(int i=0;i<n;i++)((NBTTagList)t).setTag(tag(r,ty,depth-1));break;}
        case 10: {int n=1+r.nextInt(20);for(int i=0;i<n;i++){String k=KEYS[r.nextInt(KEYS.length)];((NBTTagCompound)t).setTag(k,tag(r,1+r.nextInt(depth>0?10:8),depth-1));}break;}
        }
        return t;
    }
    static void nbt(DataOutputStream o,long seed)throws Exception {
        Random r=new Random(seed);
        NBTBase.writeTag(new NBTTagEnd(),o);
        for(int type=1;type<=10;type++)for(int n=0;n<32;n++) {
            NBTBase t=tag(r,type,3);t.setKey(KEYS[n%KEYS.length]);
            ByteArrayOutputStream bytes=new ByteArrayOutputStream();NBTBase.writeTag(t,new DataOutputStream(bytes));
            o.write(bytes.toByteArray());NBTBase.writeTag(t.cloneTag(),o);
            NBTBase read=NBTBase.readTag(new DataInputStream(new ByteArrayInputStream(bytes.toByteArray())));NBTBase.writeTag(read,o);
            if(type==10) {
                NBTTagCompound c=(NBTTagCompound)t;c.setKey("");
                byte[] gz=CompressedStreamTools.writeMapToByteArray(c);
                NBTBase.writeTag(CompressedStreamTools.loadMapFromByteArray(gz),o);
                ByteArrayOutputStream stream=new ByteArrayOutputStream();CompressedStreamTools.writeGzippedCompoundToOutputStream(c,stream);
                NBTBase.writeTag(CompressedStreamTools.loadGzippedCompoundFromOutputStream(new ByteArrayInputStream(stream.toByteArray())),o);
            }
        }
        o.writeLong(EntityOracle.rngState(r));
    }
    static void colors(DataOutputStream o,long seed)throws Exception {
        int[] grass=new int[65536],foliage=new int[65536];
        for(int i=0;i<65536;i++){grass[i]=((i*1234567)^0x654321)&0xffffff;foliage[i]=((i*7654321)^0x123456)&0xffffff;}
        ColorizerGrass.setGrassBiomeColorizer(grass);ColorizerFoliage.getFoilageBiomeColorizer(foliage);
        World w=Oracle.newWorld(seed);
        for(int id=0;id<16;id++)for(int x:new int[]{-4097,-17,-1,0,8,31,2048})for(int z:new int[]{-33,0,17})for(int y:new int[]{0,64,127}) {
            o.writeInt(BiomeGenBase.biomeList[id].getGrassColorAtCoords(w,x,y,z));
            o.writeInt(BiomeGenBase.biomeList[id].getFoliageColorAtCoords(w,x,y,z));
        }
    }
    static void nearest(DataOutputStream o,long seed)throws Exception {
        World w=Oracle.newWorld(seed);MapGenStronghold g=new MapGenStronghold();
        for(int stage=0;stage<3;stage++) {
            if(stage==1){g.worldObj=w;g.canSpawnStructureAtCoords(0,0);ChunkCoordIntPair[] ps=(ChunkCoordIntPair[])Mode_structures10.field(g,"structureCoords");for(ChunkCoordIntPair p:ps)g.generate(w.chunkProvider,w,p.chunkXPos,p.chunkZPos,null);}
            for(int[] p:new int[][]{{0,64,0},{100,0,-200},{-512,127,256},{30000000,65,-30000000},{Integer.MIN_VALUE,Integer.MAX_VALUE,Integer.MAX_VALUE}}) {
                ChunkPosition q=g.func_40484_a(w,p[0],p[1],p[2]);o.writeBoolean(q!=null);if(q!=null){o.writeInt(q.x);o.writeInt(q.y);o.writeInt(q.z);}
            }
        }
    }
    static void intcache(DataOutputStream o)throws Exception {
        // No world/layer is constructed first: starts with the original static pool.
        for(int round=0;round<8;round++) {
            IntCache.resetIntCache();
            for(int n:new int[]{1,256,257,300,33,512,260,17}) {
                int[] a=IntCache.getIntCache(n);o.writeInt(a.length);for(int v:a)o.writeInt(v);
                for(int i=0;i<n;i++)a[i]=round*1000+n+i;
            }
        }
    }
    static void population(DataOutputStream o,long seed,int dimension)throws Exception {
        World w=new World(new Oracle.NullSaveHandler(seed),"oracle",Oracle.settings(seed),WorldProvider.getProviderForDimension(dimension));
        w.rand.setSeed(300);OracleHooks.reset(500);
        java.lang.reflect.Field next=Entity.class.getDeclaredField("nextEntityID");next.setAccessible(true);next.setInt(null,100);
        for(int x=-2;x<=2;x++)for(int z=-2;z<=2;z++)w.getChunkFromChunkCoords(x,z);
        Object generator=Mode_structures10.field(w.chunkProvider,"chunkProvider");
        Random random=(Random)Mode_structures10.field(generator,dimension==0?"rand":"hellRNG");
        o.writeLong(EntityOracle.rngState(random));o.writeLong(EntityOracle.rngState(w.rand));o.writeLong(OracleHooks.nextSeed());o.writeInt(next.getInt(null));
        o.writeBoolean(BlockSand.fallInstantly);
        ArrayList<Chunk> cs=new ArrayList<Chunk>((Collection<Chunk>)Mode_structures10.field(w.chunkProvider,"chunkList"));
        Collections.sort(cs,(p,q)->p.xPosition!=q.xPosition?Integer.compare(p.xPosition,q.xPosition):Integer.compare(p.zPosition,q.zPosition));
        o.writeInt(cs.size());for(Chunk c:cs){o.writeInt(c.xPosition);o.writeInt(c.zPosition);o.writeBoolean(c.isTerrainPopulated);o.write(c.blocks);o.write(c.data.data);o.write(c.skylightMap.data);o.write(c.blocklightMap.data);o.write(c.heightMap);}
        o.writeInt(w.loadedEntityList.size());for(Object obj:w.loadedEntityList){Entity e=(Entity)obj;o.writeInt(e.entityId);o.writeUTF(EntityList.getEntityString(e));o.writeDouble(e.posX);o.writeDouble(e.posY);o.writeDouble(e.posZ);o.writeFloat(e.rotationYaw);o.writeLong(EntityOracle.rngState(e.rand));}
    }
    /** identityHashCode of every NextTickListEntry added to scheduledTickSet, in add order. */
    static ArrayList<Integer> identities;
    static void recordTickIdentities(World w)throws Exception {
        final ArrayList<Integer> ids=new ArrayList<Integer>();identities=ids;
        java.lang.reflect.Field set=World.class.getDeclaredField("scheduledTickSet");set.setAccessible(true);
        set.set(w,new HashSet<Object>(){public boolean add(Object e){boolean added=super.add(e);if(added)ids.add(System.identityHashCode(e));return added;}});
    }
    static void ticks(DataOutputStream o,long seed,int dimension)throws Exception {
        World w=new World(new Oracle.NullSaveHandler(seed),"oracle",Oracle.settings(seed),WorldProvider.getProviderForDimension(dimension));
        recordTickIdentities(w);
        w.rand.setSeed(300);OracleHooks.reset(500);
        java.lang.reflect.Field next=Entity.class.getDeclaredField("nextEntityID");next.setAccessible(true);next.setInt(null,100);
        for(int x=-2;x<=2;x++)for(int z=-2;z<=2;z++)w.getChunkFromChunkCoords(x,z);
        for(String name:new String[]{"scheduledTickTreeSet","scheduledTickSet"}) {
            Collection set=(Collection)Mode_structures10.field(w,name);o.writeInt(set.size());
            for(Object obj:set){NextTickListEntry e=(NextTickListEntry)obj;o.writeInt(e.xCoord);o.writeInt(e.yCoord);o.writeInt(e.zCoord);o.writeInt(e.blockID);o.writeLong(e.scheduledTime);}
        }
        for(int x=-2;x<=2;x++)for(int z=-2;z<=2;z++){List list=w.func_41081_a(w.getChunkFromChunkCoords(x,z),false);o.writeInt(list==null?-1:list.size());}
    }
    static void waterlily(DataOutputStream o,long seed)throws Exception {
        for(int fixture=0;fixture<8;fixture++) {
            World w=Mode_r10world3.flat(0);Random r=new Random(seed);w.rand.setSeed(300);
            for(int x=-12;x<=28;x++)for(int z=-12;z<=28;z++) {
                EntityOracle.rawBlock(w,x,63,z,new int[]{9,8,9,8,1,9,9,9}[fixture],fixture==2||fixture==3?1:0);
                if(fixture==5)EntityOracle.rawBlock(w,x,64,z,1,0);
            }
            boolean result=new MapGenWaterlily().generate(w,r,8,fixture==6?127:fixture==7?0:64,8);
            o.writeBoolean(result);o.writeLong(EntityOracle.rngState(r));o.writeLong(EntityOracle.rngState(w.rand));
            for(int x=-1;x<=1;x++)for(int z=-1;z<=1;z++){Chunk c=w.getChunkFromChunkCoords(x,z);o.write(c.blocks);o.write(c.data.data);}
        }
    }
    static void misc(DataOutputStream o)throws Exception {
        int init=Block.stone.blockID;
        for(EnumAction action:EnumAction.values()){o.writeInt(action.ordinal());o.writeUTF(action.name());}
        for(int id:new int[]{1,260,261,267,282,319,322,335,373,375,376,382})o.writeInt(Item.itemsList[id].getItemUseAction(new ItemStack(id,1,0)).ordinal());
        o.writeInt(Empty3.class.getDeclaredFields().length);o.writeInt(Empty3.class.getDeclaredMethods().length);o.writeInt(Empty3.class.getDeclaredConstructors().length);
        java.lang.reflect.Method apply=EnchantmentHelper.class.getDeclaredMethod("applyEnchantmentModifierArray",IEnchantmentModifier.class,ItemStack[].class);apply.setAccessible(true);
        for(int round=0;round<32;round++) {
            final ArrayList<Integer> trace=new ArrayList<Integer>();
            IEnchantmentModifier modifier=new IEnchantmentModifier(){public void calculateModifier(Enchantment e,int level){trace.add(e.effectId);trace.add(level);trace.add(e==Enchantment.enchantmentsList[e.effectId]?1:0);}};
            ItemStack a=new ItemStack(276,1,0),b=new ItemStack(267,1,0);
            a.addEnchantment(Enchantment.enchantmentsList[16],round+1);a.addEnchantment(Enchantment.enchantmentsList[17],3);a.addEnchantment(Enchantment.enchantmentsList[16],2);
            b.addEnchantment(Enchantment.enchantmentsList[19],4);
            apply.invoke(null,modifier,new ItemStack[]{null,a,new ItemStack(1,1,0),b});o.writeInt(trace.size());for(int n:trace)o.writeInt(n);
        }
    }
    static void providerapi(DataOutputStream o,long seed)throws Exception {
        for(int dimension:new int[]{-1,0,1}) {
            World w=new World(new Oracle.NullSaveHandler(seed),"oracle",Oracle.settings(seed),WorldProvider.getProviderForDimension(dimension));WorldProvider p=w.worldProvider;
            o.writeInt(p.worldType);o.writeBoolean(p.hasNoSky);o.writeBoolean(p.isAlternateDimension);o.writeBoolean(p.canRespawnHere());o.writeBoolean(p.func_28112_c());o.writeFloat(p.getCloudHeight());
            for(long time:new long[]{Long.MIN_VALUE,-24001,-1,0,24000,192000,Long.MAX_VALUE})o.writeInt(p.func_40470_b(time,0));
            IChunkProvider generator=p.getChunkProvider();
            ChunkProviderLoadOrGenerate loadOrGenerate=(ChunkProviderLoadOrGenerate)Mode_lightmap.unsafeAlloc(ChunkProviderLoadOrGenerate.class);
            java.lang.reflect.Field wrapped=ChunkProviderLoadOrGenerate.class.getDeclaredField("chunkProvider");wrapped.setAccessible(true);wrapped.set(loadOrGenerate,generator);
            IChunkProvider[] providers={generator,new ChunkProvider(w,null,generator),loadOrGenerate};
            for(IChunkProvider cp:providers)for(int[] q:new int[][]{{0,64,0},{-17,1,31},{1024,127,-512}})for(EnumCreatureType type:EnumCreatureType.values()) {
                List list=cp.func_40377_a(type,q[0],q[1],q[2]);o.writeInt(list==null?-1:list.size());
                if(list!=null)for(Object obj:list){SpawnListEntry entry=(SpawnListEntry)obj;o.writeUTF(entry.entityClass.getSimpleName().substring(6));o.writeInt(entry.itemWeight);o.writeInt(entry.field_35591_b);o.writeInt(entry.field_35592_c);}
            }
            for(IChunkProvider cp:providers)for(String name:new String[]{"Stronghold","stronghold","Village",""}) {
                ChunkPosition q=cp.func_40376_a(w,name,0,64,0);o.writeBoolean(q!=null);if(q!=null){o.writeInt(q.x);o.writeInt(q.y);o.writeInt(q.z);}
            }
        }
        o.writeBoolean(WorldProvider.getProviderForDimension(2)==null);
    }
    static void worldinfo(DataOutputStream o,long seed)throws Exception {
        for(int fixture=0;fixture<12;fixture++) {
            NBTTagCompound tag=new NBTTagCompound();tag.setLong("RandomSeed",seed);tag.setInteger("GameType",fixture%2);
            if(fixture%3!=0)tag.setBoolean("hardcore",fixture%3==2);
            if(fixture%4!=0)tag.setBoolean("MapFeatures",fixture%4==2);
            tag.setString("LevelName","World \u00e9\ud83d\ude00");tag.setLong("Time",-1L*fixture);tag.setInteger("version",19132);
            NBTTagCompound player=new NBTTagCompound();player.setInteger("Dimension",new int[]{0,-1,1}[fixture%3]);tag.setCompoundTag("Player",player);
            WorldInfo original=new WorldInfo(tag),copy=new WorldInfo(original);
            for(WorldInfo info:new WorldInfo[]{original,copy}) {
                o.writeBoolean(info.isHardcoreModeEnabled());o.writeBoolean(info.isMapFeaturesEnabled());o.writeInt(info.getDimension());
                NBTTagCompound result=info.getNBTTagCompound();result.setLong("LastPlayed",0);NBTBase.writeTag(result,o);
            }
        }
    }
    static void savefolders(DataOutputStream o,long seed)throws Exception {
        File root=new File("world4-savefolders-java");Mode_wbsavelife.clean(root);root.mkdirs();
        try {
            for(boolean region:new boolean[]{false,true})for(String name:new String[]{"world","World \u00e9\ud83d\ude00"}) {
                SaveHandler handler=region?new SaveOldDir(root,name,true):new SaveHandler(root,name,true);
                o.writeUTF(handler.getSaveDirectoryName());
                for(int dimension:new int[]{0,-1,1}) {
                    IChunkLoader loader=handler.getChunkLoader(WorldProvider.getProviderForDimension(dimension));
                    o.writeBoolean(loader instanceof ThreadedChunkLoader);
                    File folder=new File(root,name+(dimension==0?"":dimension==-1?"/DIM-1":"/DIM1"));o.writeBoolean(folder.isDirectory());
                }
                ThreadedFileIOBase.threadedIOInstance.waitForFinish();o.writeBoolean(new File(root,name+"/data").isDirectory());o.writeBoolean(new File(root,name+"/players").isDirectory());
            }
        } finally {RegionFileCache.clearRegionFileReferences();Mode_wbsavelife.clean(root);}
    }
    static final class Setter extends WorldGenerator {
        Setter(boolean notify){super(notify);}
        public boolean generate(World w,Random r,int x,int y,int z){return false;}
        void place(World w,int id,int meta){func_41060_a(w,8,64,8,id,meta);}
    }
    static void notify(DataOutputStream o)throws Exception {
        for(boolean notify:new boolean[]{false,true}) {
            World w=Mode_r10world3.flat(0);Setter generator=new Setter(notify);
            for(int[] q:new int[][]{{1,7},{1,3},{17,2},{17,2},{0,5},{1,0}}) {
                generator.place(w,q[0],q[1]);o.writeInt(w.getBlockId(8,64,8));o.writeInt(w.getBlockMetadata(8,64,8));o.writeLong(EntityOracle.rngState(w.rand));
            }
        }
    }
    static void endteleport(DataOutputStream o)throws Exception {
        for(double[] q:new double[][]{{8.25,70,8.75},{-0.25,64.5,-16.1},{32,50,0},{8,1,8},{8,128,8}}) {
            World w=Mode_r10world3.flat(1);OracleHooks.reset(500);Entity e=new EntityPig(w);e.setPosition(q[0],q[1],q[2]);e.rotationYaw=123;e.rotationPitch=-20;e.motionX=1;e.motionY=2;e.motionZ=3;
            Teleporter t=new Teleporter();Random r=(Random)Mode_structures10.field(t,"random");r.setSeed(42);t.placeInPortal(w,e);
            for(double d:new double[]{e.posX,e.posY,e.posZ,e.motionX,e.motionY,e.motionZ})o.writeDouble(d);o.writeFloat(e.rotationYaw);o.writeFloat(e.rotationPitch);o.writeLong(EntityOracle.rngState(r));o.writeLong(EntityOracle.rngState(w.rand));
            for(int x=-3;x<=3;x++)for(int z=-3;z<=3;z++){Chunk c=w.getChunkFromChunkCoords(x,z);o.write(c.blocks);o.write(c.data.data);}
        }
    }
    static void gzipbytes(DataOutputStream o,long seed)throws Exception {
        Random r=new Random(seed);
        for(int n=0;n<16;n++) {
            NBTTagCompound c=(NBTTagCompound)tag(r,10,3);
            byte[] bytes=CompressedStreamTools.writeMapToByteArray(c);o.writeInt(bytes.length);o.write(bytes);
        }
        // Large payloads cross the 32 KiB window and the 16 KiB symbol-buffer block limit.
        for(int n=0;n<10;n++) {
            byte[] b=new byte[n==9?0:1+r.nextInt(120000)];int mode=n%3;
            for(int i=0;i<b.length;i++) {
                if(mode==0)b[i]=(byte)r.nextInt();
                else if(mode==1)b[i]=i>0&&r.nextInt(5)!=0?b[Math.max(0,i-1-r.nextInt(i<40000?300:40000))]:(byte)r.nextInt(16);
                else b[i]=(byte)(r.nextInt(8)==0?r.nextInt(3):0);
            }
            NBTTagCompound c=new NBTTagCompound();c.setByteArray("Blocks",b);c.setInteger("xPos",n);
            ByteArrayOutputStream stream=new ByteArrayOutputStream();CompressedStreamTools.writeGzippedCompoundToOutputStream(c,stream);
            byte[] bytes=stream.toByteArray();o.writeInt(bytes.length);o.write(bytes);
        }
    }
    static void fortressspawn(DataOutputStream o,long seed)throws Exception {
        World w=new World(new Oracle.NullSaveHandler(seed),"oracle",Oracle.settings(seed),WorldProvider.getProviderForDimension(-1));
        w.rand.setSeed(300);OracleHooks.reset(500);
        java.lang.reflect.Field next=Entity.class.getDeclaredField("nextEntityID");next.setAccessible(true);next.setInt(null,100);
        Object generator=Mode_structures10.field(w.chunkProvider,"chunkProvider");MapGenNetherBridge bridge=((ChunkProviderHell)generator).genNetherBridge;
        bridge.worldObj=w;int bx=0,bz=0,best=Integer.MAX_VALUE;
        for(int cx=-40;cx<40;cx++)for(int cz=-40;cz<40;cz++)if(Math.abs(cx)+Math.abs(cz)<best&&bridge.canSpawnStructureAtCoords(cx,cz)){best=Math.abs(cx)+Math.abs(cz);bx=cx;bz=cz;}
        o.writeInt(bx);o.writeInt(bz);
        for(int x=bx-3;x<=bx+3;x++)for(int z=bz-3;z<=bz+3;z++)w.getChunkFromChunkCoords(x,z);
        Random random=(Random)Mode_structures10.field(generator,"hellRNG");
        o.writeLong(EntityOracle.rngState(random));o.writeLong(EntityOracle.rngState(w.rand));o.writeLong(OracleHooks.nextSeed());o.writeInt(next.getInt(null));
        int hits=0;
        for(int x=bx*16-96;x<=bx*16+112;x+=5)for(int z=bz*16-96;z<=bz*16+112;z+=5)for(int y:new int[]{34,40,48,56,64,72,80,90}) {
            List list=w.chunkProvider.func_40377_a(EnumCreatureType.monster,x,y,z);
            boolean inside=list==bridge.func_40485_b();if(inside)hits++;o.writeBoolean(inside);o.writeInt(list.size());
            for(Object obj:list){SpawnListEntry entry=(SpawnListEntry)obj;o.writeUTF(entry.entityClass.getSimpleName().substring(6));o.writeInt(entry.itemWeight);o.writeInt(entry.field_35591_b);o.writeInt(entry.field_35592_c);}
        }
        o.writeInt(hits);System.out.println("fortress chunk "+bx+","+bz+": "+hits+" query points inside");
        for(int x=bx-3;x<=bx+3;x++)for(int z=bz-3;z<=bz+3;z++){Chunk c=w.getChunkFromChunkCoords(x,z);o.writeBoolean(c.isTerrainPopulated);o.write(c.blocks);o.write(c.data.data);}
    }
    /** Region files with the wall-clock timestamp table cleared. */
    static void regionFiles(DataOutputStream o,File dir,int dimension)throws Exception {
        File region=new File(dir,dimension==0?"region":dimension==-1?"DIM-1/region":"DIM1/region");
        String[] names=region.list();Arrays.sort(names);o.writeInt(names.length);
        for(String name:names) {
            byte[] bytes=java.nio.file.Files.readAllBytes(new File(region,name).toPath());
            for(int i=4096;i<8192&&i<bytes.length;i++)bytes[i]=0;
            o.writeUTF(name);o.writeInt(bytes.length);o.write(bytes);
        }
    }
    static void saveformat(DataOutputStream o,long seed)throws Exception {
        File root=new File("world4-saveformat-java").getAbsoluteFile();Mode_wbsavelife.clean(root);
        try {
            SaveFormatOld f=new SaveFormatOld(root);Random r=new Random(seed);
            for(int n=1;n<=6;n++) {
                File dir=new File(root,"World"+n);dir.mkdirs();NBTTagCompound data=new NBTTagCompound();
                data.setLong("RandomSeed",r.nextLong());data.setInteger("GameType",r.nextInt(3)-1);data.setLong("LastPlayed",r.nextInt(4)*1000L);data.setLong("SizeOnDisk",r.nextInt(1<<20));
                if(n%3!=0)data.setBoolean("hardcore",n%3==1);data.setString("LevelName","nameé"+n);
                NBTTagCompound tag=new NBTTagCompound();tag.setCompoundTag("Data",data);
                CompressedStreamTools.writeGzippedCompoundToOutputStream(tag,new FileOutputStream(new File(dir,"level.dat")));
            }
            List worlds=f.getSaveList();Collections.sort(worlds);o.writeInt(worlds.size());
            for(Object object:worlds){SaveFormatComparator w=(SaveFormatComparator)object;o.writeUTF(w.getFileName());o.writeUTF(w.getDisplayName());o.writeLong(w.getLastTimePlayed());o.writeInt(w.getGameType());o.writeBoolean(w.requiresConversion());o.writeBoolean(w.isHardcoreModeEnabled());}
        } finally {Mode_wbsavelife.clean(root);}
    }
    static void regionbytes(DataOutputStream o,long seed,int dimension)throws Exception {
        File dir=new File("world4-regionbytes-java").getAbsoluteFile();Mode_wbsavelife.clean(dir);
        try {
            World w=new World(Oracle.presetSaveHandler(dir.getPath(),seed),"oracle",Oracle.settings(seed),WorldProvider.getProviderForDimension(dimension));
            recordTickIdentities(w);
            w.rand.setSeed(300);OracleHooks.reset(500);
            java.lang.reflect.Field next=Entity.class.getDeclaredField("nextEntityID");next.setAccessible(true);next.setInt(null,100);
            for(int x=-2;x<=2;x++)for(int z=-2;z<=2;z++)w.getChunkFromChunkCoords(x,z);
            w.saveWorld(true,null);ThreadedFileIOBase.threadedIOInstance.waitForFinish();regionFiles(o,dir,dimension);
            Random r=new Random(seed);
            for(int pass=0;pass<3;pass++) {
                for(int x=-2;x<=2;x++)for(int z=-2;z<=2;z++) {
                    int kind=r.nextInt(4);if(kind==0)continue;
                    Chunk c=w.getChunkFromChunkCoords(x,z);
                    for(int i=0;i<c.blocks.length;i++) {
                        if(kind==1)c.blocks[i]=(byte)(r.nextInt(6)==0?1+r.nextInt(90):c.blocks[i]);
                        else if(kind==2)c.blocks[i]=0;
                        else if(r.nextInt(3)==0)c.blocks[i]=(byte)r.nextInt(100);
                    }
                    c.isModified=true;
                }
                w.saveWorld(true,null);ThreadedFileIOBase.threadedIOInstance.waitForFinish();regionFiles(o,dir,dimension);
            }
            RegionFileCache.clearRegionFileReferences();
            regionFiles(o,dir,dimension);
        } finally {RegionFileCache.clearRegionFileReferences();Mode_wbsavelife.clean(dir);}
    }
    public static void main(String[] a)throws Exception {
        String mode=a[1];long seed=Long.parseLong(a[2]);
        ByteArrayOutputStream body=new ByteArrayOutputStream();
        try(DataOutputStream o=new DataOutputStream(body)) {
            if(mode.equals("nbt"))nbt(o,seed);
            else if(mode.equals("colors"))colors(o,seed);
            else if(mode.equals("nearest"))nearest(o,seed);
            else if(mode.equals("intcache"))intcache(o);
            else if(mode.equals("population"))population(o,seed,0);
            else if(mode.equals("hellpopulation"))population(o,seed,-1);
            else if(mode.equals("waterlily"))waterlily(o,seed);
            else if(mode.equals("ticks"))ticks(o,seed,0);
            else if(mode.equals("hellticks"))ticks(o,seed,-1);
            else if(mode.equals("misc"))misc(o);
            else if(mode.equals("providerapi"))providerapi(o,seed);
            else if(mode.equals("worldinfo"))worldinfo(o,seed);
            else if(mode.equals("savefolders"))savefolders(o,seed);
            else if(mode.equals("notify"))notify(o);
            else if(mode.equals("endteleport"))endteleport(o);
            else if(mode.equals("gzipbytes"))gzipbytes(o,seed);
            else if(mode.equals("fortressspawn"))fortressspawn(o,seed);
            else if(mode.equals("regionbytes"))regionbytes(o,seed,0);
            else if(mode.equals("saveformat"))saveformat(o,seed);
            else if(mode.equals("hellregionbytes"))regionbytes(o,seed,-1);
            else throw new IllegalArgumentException(mode);
        }
        try(DataOutputStream f=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[3])))) {
            if(identities!=null){f.writeInt(identities.size());for(int v:identities)f.writeInt(v);}
            f.write(body.toByteArray());
        }
    }
}
