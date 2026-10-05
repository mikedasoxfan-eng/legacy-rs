package net.minecraft.src;
import java.io.*;import java.util.*;
public final class Mode_terrainutils181 {
 static void box(DataOutputStream o,StructureBoundingBox b)throws Exception {for(int v:new int[]{b.field_35753_a,b.field_35751_b,b.field_35752_c,b.field_35749_d,b.field_35750_e,b.field_35748_f})o.writeInt(v);}
 public static void main(String[] a)throws Exception {
  Random r=new Random(Long.parseLong(a[1]));try(DataOutputStream o=new DataOutputStream(new FileOutputStream(a[2]))) {
   for(int k=0;k<512;k++) {
    int[] v=new int[10];for(int i=0;i<10;i++)v[i]=r.nextInt();v[9]=k%6-1;
    StructureBoundingBox b=StructureBoundingBox.func_35747_a(v[0],v[1],v[2],v[3],v[4],v[5],v[6],v[7],v[8],v[9]);box(o,b);
    StructureBoundingBox c=new StructureBoundingBox(v[0],v[1],v[2],v[3],v[4],v[5]);
    o.writeBoolean(b.func_35740_a(c));o.writeBoolean(b.func_35746_a(v[0],v[2],v[3],v[5]));o.writeBoolean(b.func_35742_b(v[6],v[7],v[8]));
    b.func_35738_b(c);b.func_35745_a(v[6],v[7],v[8]);box(o,b);o.writeInt(b.func_35744_b());o.writeInt(b.func_35743_c());o.writeInt(b.func_35739_d());
   }
   box(o,StructureBoundingBox.func_35741_a());box(o,new StructureBoundingBox(-13,31,44,55));
   for(int id=0;id<10;id++) {BiomeGenBase b=BiomeGenBase.field_35486_a[id];o.writeUTF(b.biomeName);o.writeInt(b.color);o.writeInt(b.field_6502_q);o.writeByte(b.topBlock);o.writeByte(b.fillerBlock);o.writeFloat(b.field_35492_q);o.writeFloat(b.field_35491_r);o.writeFloat(b.field_35490_s);o.writeFloat(b.field_35489_t);o.writeBoolean(b.getEnableSnow());o.writeBoolean(b.canSpawnLightningBolt());
    for(EnumCreatureType type:new EnumCreatureType[]{EnumCreatureType.creature,EnumCreatureType.monster,EnumCreatureType.waterCreature}) {List<SpawnListEntry> list=b.getSpawnableList(type);o.writeInt(list.size());for(SpawnListEntry e:list){o.writeUTF(e.entityClass.getSimpleName().substring(6));o.writeInt(e.field_35590_d);o.writeInt(e.field_35591_b);o.writeInt(e.field_35592_c);}}}
   int[][] sets={{12,10,10,8},{0,1,0,9},{-3,8,2},{Integer.MAX_VALUE,Integer.MAX_VALUE,5}};
   for(int[] weights:sets) {WeightedRandomChoice[] list=new WeightedRandomChoice[weights.length];for(int i=0;i<list.length;i++)list[i]=new WeightedRandomChoice(weights[i]);ArrayList<WeightedRandomChoice> collection=new ArrayList<>(Arrays.asList(list));o.writeInt(WeightedRandom.func_35737_a(list));o.writeInt(WeightedRandom.func_35736_a(collection));
    for(int k=0;k<64;k++){WeightedRandomChoice chosen=WeightedRandom.func_35732_a(r,list,100);o.writeInt(Arrays.asList(list).indexOf(chosen));chosen=WeightedRandom.func_35734_a(r,collection,100);o.writeInt(collection.indexOf(chosen));}}
   o.writeLong(EntityOracle.rngState(r));
   StructureStrongholdStones stones=new StructureStrongholdStones(null);
   for(int k=0;k<256;k++){int x=r.nextInt(),z=r.nextInt();ChunkCoordIntPair p=new ChunkCoordIntPair(x,z);o.writeLong(ChunkCoordIntPair.chunkXZ2Int(x,z));o.writeInt(p.hashCode());o.writeBoolean(p.equals(new ChunkCoordIntPair(x,z)));stones.func_35706_a(r,x,k,z,k%3!=0);o.writeInt(stones.func_35707_a());o.writeInt(stones.func_35708_b());
    int weight=r.nextInt(),limit=r.nextInt(5),spawned=r.nextInt(8);StructureStrongholdPieceWeight s=new StructureStrongholdPieceWeight(ComponentStrongholdLibrary.class,weight,limit);StructureVillagePieceWeight v=new StructureVillagePieceWeight(ComponentVillageHouse1.class,weight,limit);StructureStrongholdPieceWeight2 l=new StructureStrongholdPieceWeight2(ComponentStrongholdLibrary.class,weight,limit);s.field_35617_c=spawned;v.field_35606_c=spawned;l.field_35617_c=spawned;int depth=k%10;o.writeBoolean(s.func_35613_a(depth));o.writeBoolean(s.func_35614_a());o.writeBoolean(v.func_35602_a(depth));o.writeBoolean(v.func_35603_a());o.writeBoolean(l.func_35613_a(depth));
    StructurePieceTreasure t=new StructurePieceTreasure(x,z,k,limit,weight);for(int n:new int[]{t.field_35596_a,t.field_35594_b,t.field_35595_c,t.field_35593_e,t.field_35590_d})o.writeInt(n);
   }
   for(EnumDoor d:EnumDoor.values()){o.writeInt(d.ordinal());o.writeInt(EnumDoorHelper.field_35619_a[d.ordinal()]);}o.writeLong(EntityOracle.rngState(r));
  }
 }
}
