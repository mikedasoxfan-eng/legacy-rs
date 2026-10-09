package net.minecraft.src;
import java.io.*;import java.util.*;import org.lwjgl.opengl.GL13;
public final class BurningLightProbe11 {
 static int light,queries;
 static class Pig extends EntityPig {Pig(World w){super(w);}public int getEntityBrightnessForRender(float p){queries++;return light;}}
 static class Blaze extends EntityBlaze {Blaze(World w){super(w);}public int getEntityBrightnessForRender(float p){queries++;return light;}}
 static class Magma extends EntityMagmaCube {Magma(World w){super(w);}public int getEntityBrightnessForRender(float p){queries++;return light;}}
 static class Empty extends Render {public void doRender(Entity e,double x,double y,double z,float yaw,float p){}public void doRenderShadowAndFire(Entity e,double x,double y,double z,float yaw,float p){}}
 public static void main(String[] args)throws Exception {
  World w=Mode_entities10.world();OpenGlHelper.initializeTextures();RenderManager manager=Mode_rendermanager9_10.manager();Map map=(Map)Oracle.getField(manager,"entityRenderMap");map.clear();map.put(Entity.class,new Empty());Oracle.DataOut out=new Oracle.DataOut();out.i32(0x42313152);out.i32(1);out.i32(720);
  for(int kind=0;kind<3;kind++)for(int fire:new int[]{-1,0,1})for(int flag=0;flag<2;flag++)for(int value:new int[]{0,1,65535,65536,15728880,-1,Integer.MIN_VALUE,Integer.MAX_VALUE})for(float partial:new float[]{-.25F,0,.375F,1,1.25F}) {
   Entity e=kind==0?new Pig(w):kind==1?new Blaze(w):new Magma(w);java.lang.reflect.Field field=Entity.class.getDeclaredField("fire");field.setAccessible(true);field.setInt(e,fire);e.dataWatcher.updateObject(0,Byte.valueOf((byte)flag));if(kind==1)e.dataWatcher.updateObject(16,Byte.valueOf((byte)flag));light=value;queries=0;manager.renderEntity(e,partial);
   out.i32(kind);out.i32(fire);out.i32(flag);out.i32(value);Oracle.modelFloat(out,partial);out.i32(e.isBurning()?1:0);Oracle.modelFloat(out,GL13.lightmapU);Oracle.modelFloat(out,GL13.lightmapV);out.i32(queries);
  }
  try(OutputStream f=new FileOutputStream(args[0])){out.writeTo(f);}System.out.println("BurningLight11:720 original dispatch cases");
 }
}
