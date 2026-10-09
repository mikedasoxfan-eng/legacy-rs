package net.minecraft.src;
import java.io.*;
import java.lang.reflect.*;
import java.util.*;
import org.lwjgl.opengl.GL11;
import org.lwjgl.opengl.GL13;

/** Original RenderManager bytecode. Virtual probes observe its dependencies,
 * without replacing the class or recording any renderer's drawing algorithm. */
public final class Mode_rendermanager9_10 {
 static DataOutputStream out;
 static final Class[] registered = {
  EntitySpider.class,EntityCaveSpider.class,EntityPig.class,EntitySheep.class,
  EntityCow.class,EntityWolf.class,EntityChicken.class,EntitySilverfish.class,
  EntityCreeper.class,EntityEnderman.class,EntitySkeleton.class,EntityZombie.class,
  EntitySlime.class,EntityPlayer.class,EntityGiantZombie.class,EntityGhast.class,
  EntitySquid.class,EntityLiving.class,Entity.class,EntityPainting.class,EntityArrow.class,
  EntitySnowball.class,EntityEgg.class,EntityFireball.class,EntityItem.class,EntityXPOrb.class,
  EntityTNTPrimed.class,EntityFallingSand.class,EntityMinecart.class,EntityBoat.class,
  EntityFishHook.class,EntityLightningBolt.class,
  // 1.0 registrations, in RenderManager's constructor order.
  EntityMooshroom.class,EntityVillager.class,EntitySnowman.class,EntityBlaze.class,EntityMagmaCube.class,EntityDragon.class,
  EntityEnderCrystal.class,EntityEnderPearl.class,EntityEnderEye.class,EntityPotion.class,EntitySmallFireball.class
 };
 static void i(int v)throws IOException{EntityOracle.i(out,v);}
 static void f(float v)throws IOException{EntityOracle.f(out,v);}
 static void d(double v)throws IOException{EntityOracle.d(out,v);}
 static void text(String s)throws IOException{byte[] b=s.getBytes("UTF-8");i(b.length);Trace10.writeBytes(out, "b", b);}
 static RenderManager manager()throws Exception{
  Constructor<RenderManager> c=RenderManager.class.getDeclaredConstructor();c.setAccessible(true);return c.newInstance();
 }
 static Object field(Object object,String name)throws Exception{
  for(Class c=object.getClass();c!=null;c=c.getSuperclass())try{Field f=c.getDeclaredField(name);f.setAccessible(true);return f.get(object);}catch(NoSuchFieldException ignored){}
  throw new NoSuchFieldException(name);
 }
 static Map map(RenderManager m)throws Exception{return (Map)Oracle.getField(m,"entityRenderMap");}
 static int error(Runnable r){try{r.run();return 0;}catch(NullPointerException e){return 1;}catch(RuntimeException e){return 2;}}
 static final class Child extends EntityPig {Child(World w){super(w);}}
 static final class Grandchild extends ChildBase {Grandchild(World w){super(w);}}
 static class ChildBase extends EntityPig {ChildBase(World w){super(w);}}
 static int slot(Render r,Render[] slots){if(r==null)return -1;for(int j=0;j<slots.length;j++)if(r==slots[j])return j;throw new AssertionError();}
 static void lookup(RenderManager m,Class c,Render[] slots)throws Exception{
  i(map(m).size());i(map(m).containsKey(c)?1:0);
  int status=0,id=-1;try{id=slot(m.getEntityClassRenderObject(c),slots);}catch(NullPointerException e){status=1;}
  i(status);i(id);i(map(m).size());i(map(m).containsKey(c)?1:0);
 }
 static void registry()throws Exception{
  RenderManager m=manager();Render[] slots=new Render[registered.length];i(map(m).size());
  for(int j=0;j<registered.length;j++){
   Render r=m.getEntityClassRenderObject(registered[j]);slots[j]=r;
   text(r.getClass().getSimpleName());f(((Float)field(r,"shadowSize")).floatValue());
   f(((Float)field(r,"shadowOpaque")).floatValue());
   i(field(r,"renderManager")==m?1:0);
   for(int k=0;k<j;k++)i(r==slots[k]?1:0);
  }
  Class[] queries={EntityPigZombie.class,EntityMob.class,EntityAnimal.class,EntityWaterMob.class,
   EntityFlying.class,EntityWeatherEffect.class,EntityPlayerSP.class,EntityOtherPlayerMP.class,
   EntityFX.class,EntityPickupFX.class,Child.class,Grandchild.class,ChildBase.class,
   Entity.class,Object.class,String.class,Runnable.class,null};
  for(Class c:queries){lookup(m,c,slots);lookup(m,c,slots);}
  i(m.getEntityRenderObject(new Child(null))==slots[2]?1:0);
  i(error(new Runnable(){public void run(){m.getEntityRenderObject(null);}}));
  // Cached subclasses keep their old renderer when a parent's mapping changes.
  map(m).put(EntityPig.class,slots[4]);lookup(m,Child.class,slots);lookup(m,Grandchild.class,slots);
  // Null entries are not a terminal cached hit. The Entity root is terminal.
  map(m).clear();map(m).put(Entity.class,null);
  lookup(m,Grandchild.class,slots);lookup(m,Grandchild.class,slots);
  map(m).put(Entity.class,slots[18]);lookup(m,Grandchild.class,slots);
 }
 static final class ProbeWorld extends World {
  int block,metadata;
  ProbeWorld(){super((ISaveHandler)null,"unused",(WorldProvider)null,(WorldSettings)null);}
  public int getBlockId(int x,int y,int z){try{i(10);i(x);i(y);i(z);}catch(IOException e){throw new RuntimeException(e);}return block;}
  public int getBlockMetadata(int x,int y,int z){try{i(11);i(x);i(y);i(z);}catch(IOException e){throw new RuntimeException(e);}return metadata;}
 }
 static final class Camera extends EntityPig {
  boolean sleeping,brightnessThrows;int light;
  Camera(World w){super(w);}
  public boolean isPlayerSleeping(){return sleeping;}
  public int getEntityBrightnessForRender(float partial){
   try{i(20);f(partial);gl();}catch(IOException e){throw new RuntimeException(e);}
   if(brightnessThrows)throw new IllegalStateException();return light;
  }
 }
 static void pose(Camera e,int c){
  double[][] last={{-1e8,85.1,-3.75},{-.125,-.25,7.1},{1e16,-17.75,2.1},{0,0,0}};
  double[][] now={{.125,80.7,7.1},{-17.01,63.5,-.1},{-1e16,128.1,-2.1},{1,-1,3}};
  float[][] prev={{-33.7F,11.3F},{1e10F,-1e10F},{-0.0F,0},{179,-90}};
  float[][] rot={{123.5F,-26.75F},{-1e10F,1e10F},{0,-0.0F},{-179,90}};
  e.lastTickPosX=last[c][0];e.lastTickPosY=last[c][1];e.lastTickPosZ=last[c][2];
  e.posX=now[c][0];e.posY=now[c][1];e.posZ=now[c][2];
  e.prevRotationYaw=prev[c][0];e.prevRotationPitch=prev[c][1];e.rotationYaw=rot[c][0];e.rotationPitch=rot[c][1];
 }
 static void cache()throws Exception{
  RenderManager m=manager();m.playerViewY=37.25F;m.playerViewX=-21.5F;
  m.field_1222_l=19.125;m.field_1221_m=-3.25;m.field_1220_n=7.75;
  ProbeWorld w1=(ProbeWorld)Oracle.allocate(ProbeWorld.class),w2=(ProbeWorld)Oracle.allocate(ProbeWorld.class);
  RenderEngine engine=(RenderEngine)Oracle.allocate(RenderEngine.class);
  FontRenderer font=(FontRenderer)Oracle.allocate(FontRenderer.class);
  GameSettings options=(GameSettings)Oracle.allocate(GameSettings.class);Camera player=new Camera(null);
  for(int scene=0;scene<5;scene++)for(int sleep=0;sleep<3;sleep++)for(int resources=0;resources<8;resources++)
   for(float partial:new float[]{-.25F,0,.375F,1,1.25F})for(int c=0;c<4;c++){
    ProbeWorld w=scene==0?null:scene%2==1?w1:w2;
    if(w!=null){w.block=scene==1?0:26;w.metadata=new int[]{0,0,0,2,15}[scene];}
    pose(player,c);player.sleeping=sleep==1;final Camera p=sleep==2?null:player;
    final RenderEngine r=(resources&1)!=0?engine:null;final FontRenderer ft=(resources&2)!=0?font:null;
    final GameSettings o=(resources&4)!=0?options:null;final World world=w;
    i(error(new Runnable(){public void run(){m.cacheActiveRenderInfo(world,r,ft,p,o,partial);}}));
    i(m.worldObj==null?0:m.worldObj==w1?1:2);i(m.renderEngine==null?0:1);
    i(m.getFontRenderer()==null?0:1);i(m.livingPlayer==null?0:1);i(m.options==null?0:1);
    f(m.playerViewY);f(m.playerViewX);d(m.field_1222_l);d(m.field_1221_m);d(m.field_1220_n);
    d(m.getDistanceToCamera(-3.5,68.0,4.5));
    m.set(c%2==0?null:w2);i(m.worldObj==null?0:2);
    f(m.playerViewY);f(m.playerViewX);d(m.field_1222_l);d(m.field_1221_m);d(m.field_1220_n);
   }
 }
 static void gl()throws IOException{f(GL13.lightmapU);f(GL13.lightmapV);for(float c:GL11.color)f(c);}
 static final class ProbeRender extends Render {
  final int id;boolean throwsBody;
  ProbeRender(int id){this.id=id;}
  void record(int stage,double x,double y,double z,float yaw,float partial){
   try{i(stage);i(id);d(x);d(y);d(z);f(yaw);f(partial);gl();}catch(IOException e){throw new RuntimeException(e);}
  }
  public void doRender(Entity e,double x,double y,double z,float yaw,float partial){
   record(21,x,y,z,yaw,partial);GL13.lightmapU=-23.5F;GL13.lightmapV=9.25F;
   GL11.color=new float[]{.1F,.2F,.3F,.4F};if(throwsBody)throw new IllegalStateException();
  }
  public void doRenderShadowAndFire(Entity e,double x,double y,double z,float yaw,float partial){record(22,x,y,z,yaw,partial);}
 }
 static void resetGl(){GL13.lightmapU=123.25F;GL13.lightmapV=-456.5F;GL11.color=new float[]{.25F,.5F,.75F,.125F};}
 static void dispatch()throws Exception{
  for(int mode=0;mode<6;mode++)for(float partial:new float[]{-.25F,0,.375F,1,1.25F})
   for(int light:new int[]{0,1,65535,65536,15728880,-1,Integer.MIN_VALUE,Integer.MAX_VALUE}){
    RenderManager m=manager();Map map=map(m);map.clear();ProbeRender r0=new ProbeRender(0),r1=new ProbeRender(1);
    map.put(Entity.class,mode==2?null:r0);if(mode!=1&&mode!=2)map.put(EntityPig.class,r1);
    r1.throwsBody=mode==3;Camera e=new Camera(null);pose(e,0);e.light=light;e.brightnessThrows=mode==4;
    final Entity entity=mode==5?null:e;RenderManager.renderPosX=19.125;RenderManager.renderPosY=-3.25;RenderManager.renderPosZ=7.75;
    resetGl();i(error(new Runnable(){public void run(){m.renderEntity(entity,partial);}}));gl();
    resetGl();i(error(new Runnable(){public void run(){m.renderEntityWithPosYaw(entity,-.25,10.5,3.75,-75.125F,partial);}}));gl();
   }
 }
 public static void main(String[] a)throws Exception{
  Mode_entities10.world();OpenGlHelper.initializeTextures();GL13.activeTexture=0;
  try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))){
   out=o;registry();cache();dispatch();
  }
  System.out.println("rendermanager9: 32 registrations, recursive/null cache transitions, 2400 camera caches, 480 dispatches");
 }
}
