package net.minecraft.src;
import java.io.*;import java.util.*;import java.lang.reflect.*;
public final class Mode_entityapi10 {
 static Object value(int kind,int n) {
  switch(kind) {
   case 0:return Byte.valueOf((byte)(n*17));case 1:return Short.valueOf((short)(n*1009));case 2:return Integer.valueOf(n*123456789);
   case 3:return Float.valueOf(new float[]{0F,-0F,Float.NaN,Float.POSITIVE_INFINITY,-1.25F}[Math.floorMod(n,5)]);
   case 4:return "A\u0000\ud83d\ude00\ud800"+n;case 5:return new ItemStack(35,n*17,(short)(n*1009));default:return new ChunkCoordinates(n*123456789,n-17,-n*1009);
  }
 }
 static byte[] wire(DataWatcher w)throws Exception{ByteArrayOutputStream b=new ByteArrayOutputStream();w.writeWatchableObjects(new DataOutputStream(b));return b.toByteArray();}
 static void snapshot(DataWatcher w,DataOutputStream o)throws Exception{
  byte[] b=wire(w);EntityOracle.i(o,Trace10.namedInt("b.length", b.length));Trace10.writeBytes(o, "b", b);
  Field f=DataWatcher.class.getDeclaredField("objectChanged");f.setAccessible(true);Trace10.writeBoolean(o, "f.getBoolean(w)", f.getBoolean(w));
  f=DataWatcher.class.getDeclaredField("watchedObjects");f.setAccessible(true);Map map=(Map)f.get(w);EntityOracle.i(o,Trace10.namedInt("map.size()", map.size()));
  Field watching=WatchableObject.class.getDeclaredField("isWatching");watching.setAccessible(true);
  for(Object obj:map.values()){WatchableObject v=(WatchableObject)obj;EntityOracle.i(o,Trace10.namedInt("v.getDataValueId()", v.getDataValueId()));EntityOracle.i(o,Trace10.namedInt("v.getObjectType()", v.getObjectType()));Trace10.writeBoolean(o, "watching.getBoolean(v)", watching.getBoolean(v));}
 }
 public static void main(String[] a)throws Exception {
  Mode_entities10.world();
  try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[3])))) {
   if(a[1].equals("registry")) {
    Field f=EntityList.class.getDeclaredField("IDtoClassMapping");f.setAccessible(true);Map ids=(Map)f.get(null);
    f=EntityList.class.getDeclaredField("classToStringMapping");f.setAccessible(true);Map names=(Map)f.get(null);
    for(int id=-10;id<110;id++) {Class c=(Class)ids.get(id);WorldEventOracle.text(o,c==null?null:c.getSimpleName());WorldEventOracle.text(o,c==null?null:(String)names.get(c));}
    f=EntityList.class.getDeclaredField("stringToClassMapping");f.setAccessible(true);Map strings=(Map)f.get(null);
    for(String name:new String[]{"Mob","Monster","Player","Fireball","ThrownEgg","FishingHook","LightningBolt","Enderman","Silverfish","CaveSpider","XPOrb","unknown","Zombie"}) {Class c=(Class)strings.get(name);WorldEventOracle.text(o,c==null?null:c.getSimpleName());}
   } else if(a[1].equals("light")) {
    World w=Mode_entities10.world();for(int x=-20;x<=40;x++)for(int z=-20;z<=40;z++)for(int y=0;y<128;y++){Chunk c=w.getChunkFromChunkCoords(x>>4,z>>4);c.skylightMap.setNibble(x&15,y,z&15,(x+y+z)&15);c.blocklightMap.setNibble(x&15,y,z&15,(x*3+y*7+z*5)&15);}
    for(String name:new String[]{"XPOrb","Pig","Enderman","Ghast","Slime"}) {
     Entity e=EntityList.createEntityInWorld(name,w);
     for(double x:new double[]{-17.01,-.1,0,15.99,33.5,30000001})for(double y:new double[]{-4,.25,63.5,127.99,256})for(float offset:new float[]{0,-.75F,1.62F})for(float partial:new float[]{0,.25F,.75F,1}) {
      e.yOffset=offset;e.setPosition(x,y,-.1);EntityOracle.i(o,Trace10.namedInt("e.getEntityBrightnessForRender(partial)", e.getEntityBrightnessForRender(partial)));EntityOracle.f(o,Trace10.namedFloat("e.getEntityBrightness(partial)", e.getEntityBrightness(partial)));
     }
    }
   } else if(a[1].equals("watchers")) {
    for(int variant=0;variant<32;variant++) {
     DataWatcher w=new DataWatcher();for(int j=0;j<31;j++){int id=(j+variant)%31;w.addObject(id,value(id%7,id+variant));}w.addObject(-1,Byte.valueOf((byte)-128));snapshot(w,o);
     byte[] first=wire(w);
     for(int id=0;id<31;id++)w.updateObject(id,value(id%7,id+variant));snapshot(w,o);
     for(int id=0;id<31;id++)w.updateObject(id,value(id%7,id+variant+1));snapshot(w,o);
     List list=DataWatcher.readWatchableObjects(new DataInputStream(new ByteArrayInputStream(first)));w.updateWatchedObjectsFromList(list);snapshot(w,o);
     for(int id:new int[]{32,100,0})try{w.addObject(id,Byte.valueOf((byte)0));}catch(IllegalArgumentException e){WorldEventOracle.text(o,e.getMessage());}
    }
    ByteArrayOutputStream b=new ByteArrayOutputStream();DataWatcher.writeObjectsInListToStream(null,new DataOutputStream(b));EntityOracle.i(o,Trace10.namedInt("b.size()", b.size()));Trace10.writeBytes(o, "b.toByteArray()", b.toByteArray());
    Trace10.writeBoolean(o, "DataWatcher.readWatchableObjects(new DataInputStream(new ByteArrayInputStream(new byte[]{127})))==null", DataWatcher.readWatchableObjects(new DataInputStream(new ByteArrayInputStream(new byte[]{127})))==null);
    List invalid=DataWatcher.readWatchableObjects(new DataInputStream(new ByteArrayInputStream(new byte[]{(byte)224,127})));EntityOracle.i(o,Trace10.namedInt("invalid.size()", invalid.size()));Trace10.writeBoolean(o, "invalid.get(0)==null", invalid.get(0)==null);
   } else if(a[1].equals("coordinates")) {
    Random r=new Random(42);for(int i=0;i<10000;i++) {
     ChunkCoordinates p=new ChunkCoordinates(r.nextInt(),r.nextInt(),r.nextInt()),q=new ChunkCoordinates(r.nextInt(),r.nextInt(),r.nextInt());
     EntityOracle.i(o,Trace10.namedInt("p.hashCode()", p.hashCode()));EntityOracle.i(o,Trace10.namedInt("p.compareChunkCoordinate(q)", p.compareChunkCoordinate(q)));EntityOracle.d(o,Trace10.namedDouble("p.getSqDistanceTo(q.posX,q.posY,q.posZ)", p.getSqDistanceTo(q.posX,q.posY,q.posZ)));Trace10.writeBoolean(o, "p.equals(q)", p.equals(q));Trace10.writeBoolean(o, "p.equals(new ChunkCoordinates(p))", p.equals(new ChunkCoordinates(p)));
    }
   }
  }
 }
}
