package net.minecraft.src;
import java.io.*;
import java.util.*;

/** All Release biome singletons and generator selection on the original jar. */
public final class Mode_r10world3biomes {
    public static void main(String[] a)throws Exception {
        Random r=new Random(Long.parseLong(a[1]));
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[2])))) {
            for(int id=0;id<21;id++) {
                BiomeGenBase b=BiomeGenBase.biomeList[id];o.writeUTF(b.biomeName);o.writeInt(b.color);o.writeInt(b.field_6502_q);
                o.writeByte(b.topBlock);o.writeByte(b.fillerBlock);
                for(float f:new float[]{b.minHeight,b.maxHeight,b.temperature,b.rainfall})o.writeFloat(f);
                o.writeBoolean(b.getEnableSnow());o.writeBoolean(b.canSpawnLightningBolt());o.writeInt(b.waterColorMultiplier);
                o.writeInt(b.getIntTemperature());o.writeInt(b.getIntRainfall());o.writeFloat(b.getSpawningChance());
                BiomeDecorator d=b.biomeDecorator;
                for(int n:new int[]{d.treesPerChunk,d.flowersPerChunk,d.grassPerChunk,d.deadBushPerChunk,d.mushroomsPerChunk,
                    d.reedsPerChunk,d.cactiPerChunk,d.waterlilyPerChunk,d.bigMushroomsPerChunk,d.sandPerChunk,d.sandPerChunk2,d.clayPerChunk})o.writeInt(n);
                o.writeBoolean(d.generateLakes);
                for(EnumCreatureType t:new EnumCreatureType[]{EnumCreatureType.creature,EnumCreatureType.monster,EnumCreatureType.waterCreature}) {
                    List<SpawnListEntry> list=b.getSpawnableList(t);o.writeInt(list.size());
                    for(SpawnListEntry e:list){o.writeUTF(e.entityClass.getSimpleName().substring(6));o.writeInt(e.itemWeight);o.writeInt(e.field_35591_b);o.writeInt(e.field_35592_c);}
                }
                for(int n=0;n<128;n++){o.writeUTF(b.getRandomWorldGenForTrees(r).getClass().getSimpleName());o.writeLong(EntityOracle.rngState(r));}
            }
        }
    }
}
