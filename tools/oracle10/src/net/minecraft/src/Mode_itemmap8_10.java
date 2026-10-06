package net.minecraft.src;
import java.io.*;
import java.util.*;

/** ItemMap's static/stack lookups, creation and real onUpdate callbacks. */
public final class Mode_itemmap8_10 {
 static void reset(World w,int seed)throws Exception {
  w.mapStorage=new MapStorage(null);
  ((Map)Oracle.getField(w.mapStorage,"idCounts")).put("map",Short.valueOf((short)seed));
 }
 static void snapshot(Oracle.DataOut o,World w,ItemStack stack)throws Exception {
  Map data=(Map)Oracle.getField(w.mapStorage,"loadedDataMap");ArrayList<Integer> ids=new ArrayList<Integer>();
  for(Object key:data.keySet())ids.add(Integer.valueOf(((String)key).substring(4)));
  Collections.sort(ids);o.i32(stack.getItemDamage());o.i32(stack.stackSize);
  o.i32(((Short)((Map)Oracle.getField(w.mapStorage,"idCounts")).get("map")).intValue());o.i32(ids.size());
  for(int id:ids){o.i32(id);DecorationOracle.mapState(o,(MapData)data.get("map_"+id),stack);}
 }
 public static void main(String[] a)throws Exception {
  World w=DecorationOracle.mapWorld();w.getWorldInfo().setSpawn(57,64,-93);
  Oracle.DataOut o=new Oracle.DataOut();int cases=0;
  DecorationOracle.SignPlayer player=new DecorationOracle.SignPlayer((net.minecraft.client.Minecraft)Oracle.allocate(PlayerOracle.HeadlessMinecraft.class),w);
  EntityPig pig=new EntityPig(w);
  for(int seed:new int[]{-32768,-1,0,32766,32767})for(int input:new int[]{-32768,-1,0,1,32767,32768,Integer.MIN_VALUE,Integer.MAX_VALUE}) {
   reset(w,seed);ItemStack stack=new ItemStack(358,-7,input);
   MapData first=ItemMap.getMPMapData((short)input,w);o.i32(Integer.parseInt(first.mapName.substring(4)));snapshot(o,w,stack);
   MapData second=ItemMap.getMPMapData((short)input,w);o.i32(first==second?1:0);o.i32(Integer.parseInt(second.mapName.substring(4)));snapshot(o,w,stack);
   MapData third=Item.map.getMapData(stack,w);o.i32(third==first?1:0);snapshot(o,w,stack);
   MapData fourth=Item.map.getMapData(stack,w);o.i32(third==fourth?1:0);snapshot(o,w,stack);cases++;
  }
  for(int dimension:new int[]{0,-1})for(boolean remote:new boolean[]{false,true})
   for(int entityKind:new int[]{0,1,2})for(boolean selected:new boolean[]{false,true})
    for(int damage:new int[]{-32768,-1,32767,Integer.MAX_VALUE})for(boolean existing:new boolean[]{false,true}) {
     reset(w,32766);w.multiplayerWorld=remote;
     Oracle.setField(w,"worldProvider",dimension==0?new WorldProviderSurface():new WorldProviderHell());w.worldProvider.registerWorld(w);
     ItemStack stack=new ItemStack(358,-7,damage);player.isDead=false;player.dimension=dimension;player.inventory.mainInventory[0]=stack;
     player.inventory.currentItem=selected?0:1;player.setPosition(-13.25,70,19.75);pig.setPosition(-13.25,70,19.75);
     if(existing){MapData m=new MapData("map_"+damage);m.xCenter=-13;m.zCenter=19;m.scale=3;m.dimension=(byte)dimension;w.setItemData(m.mapName,m);}
     Entity entity=entityKind==0?player:entityKind==1?pig:null;
     String error="";try{Item.map.onUpdate(stack,w,entity,17,selected);}catch(RuntimeException e){error=e.getClass().getSimpleName();}
     byte[] bytes=error.getBytes("UTF-8");o.i32(bytes.length);o.bytes(bytes);snapshot(o,w,stack);cases++;
    }
  w.multiplayerWorld=false;
  for(int seed:new int[]{-32768,-1,0,32766,32767}){
   reset(w,seed);ItemStack stack=new ItemStack(358,-7,Integer.MAX_VALUE);
   String error="";try{Item.map.onCreated(stack,w,null);}catch(RuntimeException e){error=e.getClass().getSimpleName();}
   byte[] bytes=error.getBytes("UTF-8");o.i32(bytes.length);o.bytes(bytes);snapshot(o,w,stack);cases++;
  }
  for(int seed:new int[]{-32768,-1,0,32766,32767})for(int dimension:new int[]{0,-1})for(double x:new double[]{-13.25,0,19.75}){
   reset(w,seed);Oracle.setField(w,"worldProvider",dimension==0?new WorldProviderSurface():new WorldProviderHell());w.worldProvider.registerWorld(w);
   player.setPosition(x,70,-x);ItemStack stack=new ItemStack(358,-7,Integer.MAX_VALUE);
   Item.map.onCreated(stack,w,player);snapshot(o,w,stack);Item.map.onCreated(stack,w,player);snapshot(o,w,stack);cases++;
  }
  try(OutputStream f=new FileOutputStream(a[3])){o.writeTo(f);}
  System.out.println("itemmap8: "+cases+" lookup/update/creation cases");
 }
}
