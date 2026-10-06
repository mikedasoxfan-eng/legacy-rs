package net.minecraft.src;
import java.io.*;import java.util.*;
public final class Mode_terrainutils10 {
 static void box(DataOutputStream o,StructureBoundingBox b)throws Exception {for(int v:new int[]{b.minX,b.minY,b.minZ,b.maxX,b.maxY,b.maxZ})Trace10.writeInt(o, "v", v);}
 public static void main(String[] a)throws Exception {
  Random r=new Random(Long.parseLong(a[1]));try(DataOutputStream o=new DataOutputStream(new FileOutputStream(a[2]))) {
   for(int k=0;k<512;k++) {
    int[] v=new int[10];for(int i=0;i<10;i++)v[i]=r.nextInt();v[9]=k%6-1;
    StructureBoundingBox b=StructureBoundingBox.getComponentToAddBoundingBox(v[0],v[1],v[2],v[3],v[4],v[5],v[6],v[7],v[8],v[9]);box(o,b);
    StructureBoundingBox c=new StructureBoundingBox(v[0],v[1],v[2],v[3],v[4],v[5]);
    Trace10.writeBoolean(o, "b.intersectsWith(c)", b.intersectsWith(c));Trace10.writeBoolean(o, "b.isInsideStructureBB(v[0],v[2],v[3],v[5])", b.isInsideStructureBB(v[0],v[2],v[3],v[5]));Trace10.writeBoolean(o, "b.isVecInside(v[6],v[7],v[8])", b.isVecInside(v[6],v[7],v[8]));
    b.resizeBoundingBoxTo(c);b.offset(v[6],v[7],v[8]);box(o,b);Trace10.writeInt(o, "b.getXSize()", b.getXSize());Trace10.writeInt(o, "b.getYSize()", b.getYSize());Trace10.writeInt(o, "b.getZSize()", b.getZSize());
   }
   box(o,StructureBoundingBox.getNewBoundingBox());box(o,new StructureBoundingBox(-13,31,44,55));
   for(int id=0;id<10;id++) {BiomeGenBase b=BiomeGenBase.biomeList[id];o.writeUTF(b.biomeName);Trace10.writeInt(o, "b.color", b.color);Trace10.writeInt(o, "b.field_6502_q", b.field_6502_q);Trace10.writeByte(o, "b.topBlock", b.topBlock);Trace10.writeByte(o, "b.fillerBlock", b.fillerBlock);Trace10.writeFloat(o, "b.minHeight", b.minHeight);Trace10.writeFloat(o, "b.maxHeight", b.maxHeight);Trace10.writeFloat(o, "b.temperature", b.temperature);Trace10.writeFloat(o, "b.rainfall", b.rainfall);Trace10.writeBoolean(o, "b.getEnableSnow()", b.getEnableSnow());Trace10.writeBoolean(o, "b.canSpawnLightningBolt()", b.canSpawnLightningBolt());
    for(EnumCreatureType type:new EnumCreatureType[]{EnumCreatureType.creature,EnumCreatureType.monster,EnumCreatureType.waterCreature}) {List<SpawnListEntry> list=b.getSpawnableList(type);Trace10.writeInt(o, "list.size()", list.size());for(SpawnListEntry e:list){o.writeUTF(e.entityClass.getSimpleName().substring(6));Trace10.writeInt(o, "e.field_35590_d", e.field_35590_d);Trace10.writeInt(o, "e.field_35591_b", e.field_35591_b);Trace10.writeInt(o, "e.field_35592_c", e.field_35592_c);}}}
   int[][] sets={{12,10,10,8},{0,1,0,9},{-3,8,2},{Integer.MAX_VALUE,Integer.MAX_VALUE,5}};
   for(int[] weights:sets) {WeightedRandomChoice[] list=new WeightedRandomChoice[weights.length];for(int i=0;i<list.length;i++)list[i]=new WeightedRandomChoice(weights[i]);ArrayList<WeightedRandomChoice> collection=new ArrayList<>(Arrays.asList(list));Trace10.writeInt(o, "WeightedRandom.func_35737_a(list)", WeightedRandom.func_35737_a(list));Trace10.writeInt(o, "WeightedRandom.func_35736_a(collection)", WeightedRandom.func_35736_a(collection));
    for(int k=0;k<64;k++){WeightedRandomChoice chosen=WeightedRandom.func_35732_a(r,list,100);Trace10.writeInt(o, "Arrays.asList(list).indexOf(chosen)", Arrays.asList(list).indexOf(chosen));chosen=WeightedRandom.func_35734_a(r,collection,100);Trace10.writeInt(o, "collection.indexOf(chosen)", collection.indexOf(chosen));}}
   Trace10.writeLong(o, "EntityOracle.rngState(r)", EntityOracle.rngState(r));
   java.lang.reflect.Constructor ctor=StructureStrongholdStones.class.getDeclaredConstructor();ctor.setAccessible(true);StructureStrongholdStones stones=(StructureStrongholdStones)ctor.newInstance();
   for(int k=0;k<256;k++){int x=r.nextInt(),z=r.nextInt();ChunkCoordIntPair p=new ChunkCoordIntPair(x,z);Trace10.writeLong(o, "ChunkCoordIntPair.chunkXZ2Int(x,z)", ChunkCoordIntPair.chunkXZ2Int(x,z));Trace10.writeInt(o, "p.hashCode()", p.hashCode());Trace10.writeBoolean(o, "p.equals(new ChunkCoordIntPair(x,z))", p.equals(new ChunkCoordIntPair(x,z)));stones.selectBlocks(r,x,k,z,k%3!=0);Trace10.writeInt(o, "stones.getSelectedBlockId()", stones.getSelectedBlockId());Trace10.writeInt(o, "stones.getSelectedBlockMetaData()", stones.getSelectedBlockMetaData());
    int weight=r.nextInt(),limit=r.nextInt(5),spawned=r.nextInt(8);StructureStrongholdPieceWeight s=new StructureStrongholdPieceWeight(ComponentStrongholdLibrary.class,weight,limit);StructureVillagePieceWeight v=new StructureVillagePieceWeight(ComponentVillageHouse1.class,weight,limit);StructureStrongholdPieceWeight2 l=new StructureStrongholdPieceWeight2(ComponentStrongholdLibrary.class,weight,limit);s.instancesSpawned=spawned;v.villagePiecesSpawned=spawned;l.instancesSpawned=spawned;int depth=k%10;Trace10.writeBoolean(o, "s.canSpawnMoreStructuresOfType(depth)", s.canSpawnMoreStructuresOfType(depth));Trace10.writeBoolean(o, "s.canSpawnMoreStructures()", s.canSpawnMoreStructures());Trace10.writeBoolean(o, "v.canSpawnMoreVillagePiecesOfType(depth)", v.canSpawnMoreVillagePiecesOfType(depth));Trace10.writeBoolean(o, "v.canSpawnMoreVillagePieces()", v.canSpawnMoreVillagePieces());Trace10.writeBoolean(o, "l.canSpawnMoreStructuresOfType(depth)", l.canSpawnMoreStructuresOfType(depth));
    StructurePieceTreasure t=new StructurePieceTreasure(x,z,k,limit,weight);for(int n:new int[]{t.field_35596_a,t.field_35594_b,t.field_35595_c,t.field_35593_e,t.field_35590_d})Trace10.writeInt(o, "n", n);
   }
   java.lang.reflect.Field doorMap=EnumDoorHelper.class.getDeclaredField("doorEnum");doorMap.setAccessible(true);for(EnumDoor d:EnumDoor.values()){Trace10.writeInt(o, "d.ordinal()", d.ordinal());Trace10.writeInt(o, "((int[])doorMap.get(null))[d.ordinal()]", ((int[])doorMap.get(null))[d.ordinal()]);}Trace10.writeLong(o, "EntityOracle.rngState(r)", EntityOracle.rngState(r));
  }
 }
}
