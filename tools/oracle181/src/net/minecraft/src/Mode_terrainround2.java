package net.minecraft.src;
import java.io.*;
import java.util.*;
/** Sky population, cross-world biome generator lifetime, sky colours and dungeon tiles. */
public final class Mode_terrainround2 {
    public static void main(String[] a) throws Exception {
        String mode=a[1];long seed=Long.parseLong(a[2]);String file=a[3];
        if(mode.equals("lifetime")) {
            try(OutputStream out=new BufferedOutputStream(new FileOutputStream(file))) {
                for(long s:new long[]{seed,3,4,5}) {String p=file+".part";Mode_features181.main(new String[]{"features181","decorate4",Long.toString(s),"0",p});try(InputStream in=new FileInputStream(p)){byte[] b=new byte[8192];int n;while((n=in.read(b))>=0)out.write(b,0,n);}new File(p).delete();}
            }return;
        }
        if(mode.equals("dungeon")) {
            // A dedicated copy of the fixture includes all original tile contents.
            World w=flat(0,false);for(int x=10;x<=13;x++)for(int y=32;y<=33;y++)EntityOracle.rawBlock(w,x,y,8,0,0);
            Random r=new Random(seed);w.rand.setSeed(300);BlockSand.fallInstantly=true;boolean result=new WorldGenDungeons().generate(w,r,8,32,8);
            Mode_features181.dump(w,r,result,file);try(DataOutputStream out=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(file,true)))){Mode_structures181.tiles(out,w,-48,63);}return;
        }
        if(mode.equals("sky")) {
            World w=flat(seed,true);ChunkProviderSky gen=new ChunkProviderSky(w,seed);w.rand.setSeed(300);gen.populate(w.chunkProvider,0,0);
            java.lang.reflect.Field f=ChunkProviderSky.class.getDeclaredField("random");f.setAccessible(true);
            Mode_features181.dump(w,(Random)f.get(gen),true,file);return;
        }
        if(mode.equals("locations")) {
            World w=Oracle.newWorld(seed);MapGenStronghold g=new MapGenStronghold();g.field_35625_d=w;g.func_35628_a(0,0);
            ChunkCoordIntPair[] ps=(ChunkCoordIntPair[])Mode_structures181.field(g,"field_35633_g");
            try(DataOutputStream out=new DataOutputStream(new FileOutputStream(file))){for(ChunkCoordIntPair p:ps){out.writeInt(p.chunkXPos);out.writeInt(p.chunkZPos);}out.writeLong(EntityOracle.rngState(g.rand));}return;
        }
        if(mode.equals("skygrid")) {
            World w=new World(Oracle.newWorld(seed),new WorldProviderSky());w.rand.setSeed(300);
            for(int x=-2;x<=2;x++)for(int z=-2;z<=2;z++)w.getChunkFromChunkCoords(x,z);
            java.lang.reflect.Field f=ChunkProvider.class.getDeclaredField("chunkList");f.setAccessible(true);ArrayList<Chunk> cs=new ArrayList<Chunk>((Collection<Chunk>)f.get(w.chunkProvider));
            cs.sort((p,q)->p.xPosition!=q.xPosition?Integer.compare(p.xPosition,q.xPosition):Integer.compare(p.zPosition,q.zPosition));
            f=ChunkProvider.class.getDeclaredField("chunkProvider");f.setAccessible(true);ChunkProviderSky gen=(ChunkProviderSky)f.get(w.chunkProvider);f=ChunkProviderSky.class.getDeclaredField("random");f.setAccessible(true);
            try(DataOutputStream out=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(file)))){out.writeLong(EntityOracle.rngState((Random)f.get(gen)));out.writeLong(EntityOracle.rngState(w.rand));out.writeInt(cs.size());for(Chunk c:cs){out.writeInt(c.xPosition);out.writeInt(c.zPosition);out.writeBoolean(c.isTerrainPopulated);out.write(c.blocks);out.write(c.data.data);out.write(c.skylightMap.data);out.write(c.blocklightMap.data);out.write(c.heightMap);}}
            return;
        }
        if(mode.equals("colors")) {
            try(DataOutputStream out=new DataOutputStream(new FileOutputStream(file))){for(BiomeGenBase b:BiomeGenBase.field_35486_a){if(b==null)continue;for(float t:new float[]{-100,-3,-1,0,0.5F,1,3,100})out.writeInt(b.getSkyColorByTemp(t));}WorldProvider p=new WorldProviderSky();for(long time:new long[]{-1,0,6000,12000,18000,24000})for(float partial:new float[]{0,0.5F,1})out.writeFloat(p.calculateCelestialAngle(time,partial));}return;
        }
        throw new IllegalArgumentException(mode);
    }
    static World flat(long seed,boolean sky) {
        World w=sky?new World(Oracle.newWorld(seed),new WorldProviderSky()):Oracle.newWorld(seed);w.chunkProvider=new EntityOracle.FlatProvider(w);
        for(int cx=-3;cx<=3;cx++)for(int cz=-3;cz<=3;cz++){Chunk c=w.getChunkFromChunkCoords(cx,cz);Arrays.fill(c.blocks,(byte)0);for(int x=0;x<16;x++)for(int z=0;z<16;z++)for(int y=0;y<=63;y++)c.blocks[(x<<11)|(z<<7)|y]=(byte)(y<61?1:y<63?3:2);c.generateSkylightMap();}return w;
    }
}
