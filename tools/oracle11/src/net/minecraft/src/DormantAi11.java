package net.minecraft.src;
import java.io.*;import java.lang.reflect.*;import java.util.*;import java.util.concurrent.atomic.AtomicLong;
/** Audit-only original graph. No substitute AI methods or patched helper classes. */
public final class DormantAi11 {
 static final String[] ROOTS={"field_46017_d","field_46018_e","field_46021_am","field_46022_an","field_46019_bU"};
 static Field field(Class c,String n)throws Exception{Field f=c.getDeclaredField(n);f.setAccessible(true);return f;}
 static Method resolve(Class c,String n)throws Exception{while(c!=null){try{Method m=c.getDeclaredMethod(n);m.setAccessible(true);return m;}catch(NoSuchMethodException ex){c=c.getSuperclass();}}throw new AssertionError(n);}
 static long rng(Random r)throws Exception{return((AtomicLong)field(Random.class,"seed").get(r)).get();}
 static String graph(Object x,Entity e,World w,Set<Object> seen)throws Exception{
  if(x==null)return "null";if(x==e)return "ENTITY";if(x==w)return "WORLD";
  if(x instanceof Float)return "f"+Float.floatToRawIntBits((Float)x);if(x instanceof Double)return "d"+Double.doubleToRawLongBits((Double)x);
  if(x instanceof Number||x instanceof Boolean||x instanceof String)return x.toString();
  if(!seen.add(x))return "REF:"+x.getClass().getSimpleName();StringBuilder s=new StringBuilder(x.getClass().getSimpleName()+"{");
  if(x instanceof List){for(Object v:(List)x)s.append(graph(v,e,w,seen)).append(';');}
  else{ArrayList<Field> fs=new ArrayList<Field>();for(Class c=x.getClass();c!=Object.class;c=c.getSuperclass())for(Field f:c.getDeclaredFields())if(!Modifier.isStatic(f.getModifiers()))fs.add(f);Collections.sort(fs,new Comparator<Field>(){public int compare(Field a,Field b){return(a.getDeclaringClass().getName()+a.getName()).compareTo(b.getDeclaringClass().getName()+b.getName());}});for(Field f:fs){f.setAccessible(true);s.append(f.getName()).append('=').append(graph(f.get(x),e,w,seen)).append(';');}}
  return s.append('}').toString();
 }
 static String roots(EntityLiving e,World w)throws Exception{StringBuilder s=new StringBuilder();for(String n:ROOTS)s.append(n).append('=').append(graph(field(EntityLiving.class,n).get(e),e,w,Collections.newSetFromMap(new IdentityHashMap<Object,Boolean>()))).append('|');return s.toString();}
 static byte[] nbt(Entity e)throws Exception{NBTTagCompound n=new NBTTagCompound();e.addEntityID(n);ByteArrayOutputStream b=new ByteArrayOutputStream();NBTBase.writeTag(n,new DataOutputStream(b));return b.toByteArray();}
 static void constructDetached(EntityLiving e,World w){new EntityLookHelper(e);new EntityMoveHelper(e,e.moveSpeed);new EntityJumpHelper(e);new PathNavigate(e,w);EntityAITasks ts=new EntityAITasks();if(e instanceof EntityZombie){ts.func_46118_a(1,new EntityAISwimming(e));ts.func_46118_a(2,new EntityAIAttackOnCollide((EntityMob)e,w,16F));ts.func_46118_a(3,new EntityAIWander((EntityCreature)e));ts.func_46118_a(4,new EntityAIWatchClosest(e,w,8F));ts.func_46118_a(4,new EntityAILookIdle(e));}}
 public static void main(String[] args)throws Exception{
  File dir=new File(args[0]);dir.mkdirs();PrintWriter o=new PrintWriter(new File(dir,"ai_dormant11.txt"));int cases=0;
  for(int joined=0;joined<2;joined++)for(long seed:new long[]{0,1,42,100,-1,341873128712L})for(int id:EggState11.IDS){
   World w=Mode_entities10.world();OracleHooks.reset(seed);EntityLiving e=(EntityLiving)EntityList.createEntity(id,w);e.setLocationAndAngles(.5,65,.5,0,0);
   Method guard=resolve(e.getClass(),"func_46006_aR");if((Boolean)guard.invoke(e))throw new AssertionError("Reachable new AI "+id);
   String before=roots(e,w);long er=rng(e.rand),wr=rng(w.rand);int next=field(Entity.class,"nextEntityID").getInt(null),loaded=w.loadedEntityList.size();byte[] n=nbt(e);
   constructDetached(e,w);
   if(er!=rng(e.rand)||wr!=rng(w.rand)||next!=field(Entity.class,"nextEntityID").getInt(null)||loaded!=w.loadedEntityList.size()||!Arrays.equals(n,nbt(e))||!before.equals(roots(e,w)))throw new AssertionError("Constructor side effect "+id);
   if(joined!=0)w.spawnEntityInWorld(e);for(int t=0;t<20;t++){if(joined==0)e.onUpdate();else w.updateEntityWithOptionalForce(e,true);}
   if(!before.equals(roots(e,w)))throw new AssertionError("AI helper changed in ordinary tick "+id);
   NBTTagCompound saved=new NBTTagCompound();boolean saveOk=e.addEntityID(saved);if(saveOk){EntityLiving loadedE=(EntityLiving)EntityList.createEntityFromNBT(saved,w);String fresh=roots(loadedE,w);for(int t=0;t<5;t++)loadedE.onUpdate();if(!fresh.equals(roots(loadedE,w)))throw new AssertionError("AI helper changed after load "+id);}
   o.println(id+" "+seed+" "+joined+" guard="+guard.getDeclaringClass().getSimpleName()+":false constructors=no-observable-side-effects ticks20=unchanged reloadTicks5="+(saveOk?"unchanged":"original-save-rejected")+" "+before);cases++;
  }o.close();System.out.println("ORIGINAL AI GRAPH EXACT: "+cases+" constructor/20tick/save/reload5tick cases across20mob types; virtual guards allfalse");
 }
}
