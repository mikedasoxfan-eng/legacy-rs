package net.minecraft.src;
import java.io.*;
import java.util.*;

/** Isolated feature fixtures; all generators below come from the original jar. */
public final class Mode_features181 {
    public static void main(String[] a) throws Exception {
        String name=a[1];long seed=Long.parseLong(a[2]);int fixture=Integer.parseInt(a[3]);
        World w=Oracle.newWorld(0);w.chunkProvider=new EntityOracle.FlatProvider(w);
        for(int cx=-3;cx<=3;cx++)for(int cz=-3;cz<=3;cz++) {
            Chunk c=w.getChunkFromChunkCoords(cx,cz);Arrays.fill(c.blocks,(byte)0);
            for(int x=0;x<16;x++)for(int z=0;z<16;z++)for(int y=0;y<=63;y++) c.blocks[(x<<11)|(z<<7)|y]=(byte)(y<61?1:y<63?3:2);
        }
        if(fixture==1)for(int x=3;x<=13;x++)for(int z=3;z<=13;z++) EntityOracle.rawBlock(w,x,64,z,9,0);
        if(fixture==2) EntityOracle.rawBlock(w,8,66,8,1,0);
        if(fixture==4)for(int x=10;x<=13;x++)for(int y=32;y<=33;y++)EntityOracle.rawBlock(w,x,y,8,0,0);
        for(int cx=-3;cx<=3;cx++)for(int cz=-3;cz<=3;cz++)w.getChunkFromChunkCoords(cx,cz).generateSkylightMap();
        w.rand.setSeed(300);Random r=new Random(seed);BlockSand.fallInstantly=true;
        if(name.startsWith("animals")) {
            SpawnerAnimals.func_35957_a(w,BiomeGenBase.field_35486_a[Integer.parseInt(name.substring(7))],8,8,16,16,r);
            dump(w,r,true,a[4]);return;
        }
        if(name.startsWith("decorate")) {
            // The decorator takes an external RNG, just as population does.
            BiomeGenBase.field_35486_a[Integer.parseInt(name.substring(8))].func_35477_a(w,r,0,0);
            dump(w,r,true,a[4]);return;
        }
        WorldGenerator feature;
        switch(name) {
            case "oak":feature=new WorldGenTrees();break;case "birch":feature=new WorldGenForest();break;
            case "taiga1":feature=new WorldGenTaiga1();break;case "taiga2":feature=new WorldGenTaiga2();break;
            case "big":feature=new WorldGenBigTree();feature.func_517_a(1,1,1);break;
            case "swamp":feature=new WorldGenSwamp();break;
            case "brown":feature=new WorldGenBigMushroom(0);break;case "red":feature=new WorldGenBigMushroom(1);break;
            case "clay":feature=new WorldGenClay(4);break;case "sand":feature=new WorldGenSand(7,12);break;case "gravel":feature=new WorldGenSand(6,13);break;
            case "ore":feature=new WorldGenMinable(14,8);break;
            case "lake":feature=new WorldGenLakes(9);break;case "lava":feature=new WorldGenLakes(11);break;case "dungeon":feature=new WorldGenDungeons();break;
            default:throw new IllegalArgumentException(name);
        }
        int y=name.equals("ore")||name.equals("dungeon")?32:fixture==3?123:64;
        boolean result=feature.generate(w,r,8,y,8);
        dump(w,r,result,a[4]);
    }
    static void dump(World w,Random r,boolean result,String path) throws Exception {
        try(DataOutputStream out=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(path)))) {
            out.writeBoolean(result);out.writeLong(EntityOracle.rngState(r));out.writeLong(EntityOracle.rngState(w.rand));
            for(int cx=-3;cx<=3;cx++)for(int cz=-3;cz<=3;cz++) {Chunk c=w.getChunkFromChunkCoords(cx,cz);out.write(c.blocks);out.write(c.data.data);}
            out.writeInt(w.loadedEntityList.size());for(Object obj:w.loadedEntityList) {Entity e=(Entity)obj;out.writeUTF(EntityList.getEntityString(e));out.writeDouble(e.posX);out.writeDouble(e.posY);out.writeDouble(e.posZ);out.writeFloat(e.rotationYaw);out.writeInt(e instanceof EntitySheep?((EntitySheep)e).getFleeceColor():-1);}
        }
    }
}
