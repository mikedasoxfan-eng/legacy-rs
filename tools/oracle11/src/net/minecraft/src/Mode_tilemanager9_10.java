package net.minecraft.src;
import java.io.*;import java.lang.reflect.*;import java.util.*;
import org.lwjgl.opengl.GL11;import org.lwjgl.opengl.GL13;
/** Original tile manager bytecode, including HashMap iteration with real Class hashes. */
public final class Mode_tilemanager9_10 {
 static DataOutputStream out;static int worldFail=-1,drawFail=-1;
 static final Class[] classes={TileEntity.class,Object.class,TileEntitySign.class,TileEntityMobSpawner.class,TileEntityPiston.class,TileEntityChest.class,
  TileEntityFurnace.class,TileEntityDispenser.class,TileEntityNote.class,TileEntityRecordPlayer.class,ChildSign.class,GrandSign.class,ChildPiston.class,String.class,Runnable.class,ProbeTile.class,Unknown.class};
 static void i(int v)throws IOException{EntityOracle.i(out,v);}static void f(float v)throws IOException{EntityOracle.f(out,v);}static void d(double v)throws IOException{EntityOracle.d(out,v);}
 static final class ChildSign extends TileEntitySign {}static final class GrandSign extends SignBase {}static class SignBase extends TileEntitySign {}static final class ChildPiston extends TileEntityPiston {}
 // Include the intermediate class in the captured hash inputs as well.
 static Class[] all(){Class[] a=Arrays.copyOf(classes,classes.length+3);a[a.length-3]=SignBase.class;a[a.length-2]=TileEntityEnchantmentTable.class;a[a.length-1]=TileEntityEndPortal.class;return a;}
 static Map map(TileEntityRenderer m)throws Exception{return (Map)Oracle.getField(m,"specialRendererMap");}
 static TileEntityRenderer manager()throws Exception{Constructor<TileEntityRenderer> c=TileEntityRenderer.class.getDeclaredConstructor();c.setAccessible(true);return c.newInstance();}
 static int error(Runnable r){try{r.run();return 0;}catch(NullPointerException e){return 1;}catch(RuntimeException e){return 2;}}
 static int slot(TileEntitySpecialRenderer r,TileEntitySpecialRenderer[] slots){if(r==null)return -1;for(int j=0;j<slots.length;j++)if(r==slots[j])return j;throw new AssertionError();}
 static void lookup(TileEntityRenderer m,Class c,TileEntitySpecialRenderer[] slots)throws Exception{
  i(map(m).size());i(map(m).containsKey(c)?1:0);int error=0,id=-1;
  try{id=slot(m.getSpecialRendererForClass(c),slots);}catch(NullPointerException e){error=1;}
  i(error);i(id);i(map(m).size());i(map(m).containsKey(c)?1:0);
 }
 static void registry()throws Exception{
  TileEntityRenderer m=manager();TileEntitySpecialRenderer[] slots=new TileEntitySpecialRenderer[4];i(map(m).size());
  for(int j=0;j<4;j++){TileEntitySpecialRenderer r=m.getSpecialRendererForClass(classes[j+2]);slots[j]=r;byte[] name=r.getClass().getSimpleName().getBytes("UTF-8");i(name.length);Trace10.writeBytes(out, "name", name);
   Field field=TileEntitySpecialRenderer.class.getDeclaredField("tileEntityRenderer");field.setAccessible(true);i(field.get(r)==m?1:0);for(int k=0;k<j;k++)i(r==slots[k]?1:0);}
  for(Class c:new Class[]{TileEntity.class,TileEntityFurnace.class,TileEntityDispenser.class,TileEntityNote.class,TileEntityRecordPlayer.class,ChildSign.class,GrandSign.class,ChildPiston.class,Object.class,String.class,Runnable.class,null}){lookup(m,c,slots);lookup(m,c,slots);}
  i(m.getSpecialRendererForEntity(null)==null?1:0);i(m.hasSpecialRenderer(null)?1:0);
  TileEntitySign t=new TileEntitySign();for(double x:new double[]{-64,-.5,0,.5,63.5,64}){t.xCoord=-17;t.yCoord=127;t.zCoord=16;d(t.getDistanceFrom(x,-.25,16.5));}
 }
 static final class WorldProbe extends World {
  int id,light,fail;WorldProbe(){super((ISaveHandler)null,"unused",(WorldProvider)null,(WorldSettings)null);}
  public int getLightBrightnessForSkyBlocks(int x,int y,int z,int min){try{i(41);i(id);i(x);i(y);i(z);i(min);}catch(IOException e){throw new RuntimeException(e);}if(fail==1)throw new IllegalStateException();return light;}
 }
 static void gl()throws IOException{f(GL13.lightmapU);f(GL13.lightmapV);for(float c:GL11.color)f(c);}
 static final class Renderer extends TileEntitySpecialRenderer {
  int id;Renderer(int id){this.id=id;}
  public void func_31069_a(World w){try{i(30);i(id);i(w==null?0:((WorldProbe)w).id);}catch(IOException e){throw new RuntimeException(e);}if(worldFail==id)throw new IllegalStateException();}
  public void renderTileEntityAt(TileEntity t,double x,double y,double z,float partial){try{i(42);i(id);d(x);d(y);d(z);f(partial);gl();}catch(IOException e){throw new RuntimeException(e);}if(drawFail==id)throw new IllegalStateException();}
 }
 static TileEntityRenderer probes()throws Exception{
  TileEntityRenderer m=manager();for(int j=0;j<4;j++){Renderer r=new Renderer(j);r.setTileEntityRenderer(m);map(m).put(classes[j+2],r);}
  m.getSpecialRendererForClass(ChildSign.class);m.getSpecialRendererForClass(GrandSign.class);m.getSpecialRendererForClass(ChildPiston.class);m.getSpecialRendererForClass(TileEntityFurnace.class);return m;
 }
 static void state(TileEntityRenderer m,WorldProbe w1)throws Exception{
  i(m.worldObj==null?0:m.worldObj==w1?1:2);i(m.renderEngine==null?0:1);i(m.getFontRenderer()==null?0:1);i(m.entityLivingPlayer==null?0:1);
  f(m.playerYaw);f(m.playerPitch);d(m.playerX);d(m.playerY);d(m.playerZ);
 }
 static void cache()throws Exception{
  TileEntityRenderer m=probes();WorldProbe w1=(WorldProbe)Oracle.allocate(WorldProbe.class),w2=(WorldProbe)Oracle.allocate(WorldProbe.class);w1.id=1;w2.id=2;
  RenderEngine e=(RenderEngine)Oracle.allocate(RenderEngine.class);FontRenderer font=(FontRenderer)Oracle.allocate(FontRenderer.class);
  Mode_rendermanager9_10.Camera camera=new Mode_rendermanager9_10.Camera(null);
  for(int scene=0;scene<3;scene++)for(int resources=0;resources<4;resources++)for(int player=0;player<3;player++)for(float t:new float[]{-.25F,0,.375F,1,1.25F})for(int fault=-1;fault<2;fault++){
   final World w=scene==0?null:scene==1?w1:w2;final RenderEngine engine=(resources&1)!=0?e:null;final FontRenderer ft=(resources&2)!=0?font:null;
   Mode_rendermanager9_10.pose(camera,resources);camera.sleeping=player==1;final EntityLiving p=player==2?null:camera;worldFail=fault;
   i(error(new Runnable(){public void run(){m.cacheActiveRenderInfo(w,engine,ft,p,t);}}));state(m,w1);
   final World next=resources%2==0?w2:null;i(error(new Runnable(){public void run(){m.func_31072_a(next);}}));state(m,w1);
  }
  worldFail=-1;
 }
 static class ProbeTile extends TileEntitySign {
  double distance;
  public double getDistanceFrom(double x,double y,double z){try{i(40);d(x);d(y);d(z);}catch(IOException e){throw new RuntimeException(e);}return distance;}
 }
 static final class Unknown extends ProbeTile {}
 static void resetGl(){GL13.lightmapU=123.25F;GL13.lightmapV=-456.5F;GL11.color=new float[]{.25F,.5F,.75F,.125F};}
 static void dispatch()throws Exception{
  for(int tileKind=0;tileKind<3;tileKind++)for(boolean world:new boolean[]{false,true})for(int fault=0;fault<3;fault++)
   for(double distance:new double[]{-1,0,4095.999,4096,Double.NaN,Double.POSITIVE_INFINITY})for(int light:new int[]{0,-1,Integer.MIN_VALUE,Integer.MAX_VALUE,15728880}){
    TileEntityRenderer m=probes();WorldProbe w=(WorldProbe)Oracle.allocate(WorldProbe.class);w.id=1;w.light=light;w.fail=fault;m.worldObj=world?w:null;
    m.playerX=-3.5;m.playerY=68.0;m.playerZ=4.5;TileEntityRenderer.staticPlayerX=19.125;TileEntityRenderer.staticPlayerY=-3.25;TileEntityRenderer.staticPlayerZ=7.75;
    ProbeTile t=new ProbeTile();t.distance=distance;t.xCoord=-17;t.yCoord=127;t.zCoord=16;final TileEntity tile=tileKind==0?t:tileKind==1?new Unknown():null;
    if(tile instanceof Unknown){ProbeTile u=(ProbeTile)tile;u.distance=distance;u.xCoord=-17;u.yCoord=127;u.zCoord=16;map(m).put(Unknown.class,null);map(m).put(ProbeTile.class,null);map(m).put(TileEntitySign.class,null);}
    drawFail=fault==2?0:-1;resetGl();i(error(new Runnable(){public void run(){m.renderTileEntity(tile,.375F);}}));gl();
    resetGl();i(error(new Runnable(){public void run(){m.renderTileEntityAt(tile,-.25,10.5,3.75,.375F);}}));gl();
   }
  drawFail=-1;
 }
 public static void main(String[] a)throws Exception{
  Mode_entities10.world();OpenGlHelper.initializeTextures();GL13.activeTexture=0;try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))){
   out=o;i(0x39383154);i(all().length);for(Class c:all())i(c.hashCode());registry();cache();dispatch();
  }System.out.println("tilemanager9: 4 registrations, recursive/null caches, 540 active caches and 1080 range/dispatch calls (real JVM Class hashes)");
 }
}
