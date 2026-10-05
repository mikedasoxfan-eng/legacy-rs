package net.minecraft.src;
import java.io.*;
import java.lang.reflect.*;
/** Real registered renderers, including the newly routed thrown-item sprites. */
public final class Mode_managervisual9_181 {
 public static void main(String[] a)throws Exception{
  World w=Mode_entities181.world();RenderManager m=Mode_rendermanager9_181.manager();
  m.worldObj=w;m.renderEngine=RendererOracle.manager(w).renderEngine;
  m.options=(GameSettings)Oracle.allocate(GameSettings.class);m.playerViewY=37.25F;m.playerViewX=-21.5F;
  RendererOracle.field("convertQuadsToTriangles").setBoolean(null,false);
  Oracle.DataOut out=new Oracle.DataOut();out.i32(0x3938314d);out.i32(1);out.i32(5*4*4*2);
  Field radius=Render.class.getDeclaredField("shadowSize");radius.setAccessible(true);
  for(int kind=0;kind<5;kind++)for(float t:new float[]{0,.25F,.75F,1})for(int camera=0;camera<4;camera++)for(boolean fancy:new boolean[]{false,true}){
   Entity e=kind==0?new EntityCaveSpider(w):kind==1?new EntitySilverfish(w):kind==2?new EntityXPOrb(w):kind==3?new EntitySnowball(w):new EntityEgg(w);
   e.setPosition(.25,64,-.75);e.lastTickPosX=-.125;e.lastTickPosY=63.875;e.lastTickPosZ=-1.25;
   Render r=m.getEntityRenderObject(e);m.options.fancyGraphics=fancy;
   double[] origin={0,63,4};double[][] views={{0,63,4},{17,64,-.75},{16.25,64,-.75},{0,70,4}};
   double[] view=views[camera];m.field_1222_l=view[0];m.field_1221_m=view[1];m.field_1220_n=view[2];
   out.i32(kind);Oracle.modelFloat(out,t);out.i32(fancy?1:0);
   Oracle.modelFloat(out,e.width);Oracle.modelFloat(out,e.height);out.i64(Double.doubleToRawLongBits(e.boundingBox.minY));
   double[] pos={e.posX,e.posY,e.posZ},last={e.lastTickPosX,e.lastTickPosY,e.lastTickPosZ};
   for(double v:pos)out.i64(Double.doubleToRawLongBits(v));for(double v:last)out.i64(Double.doubleToRawLongBits(v));
   for(double v:origin)out.i64(Double.doubleToRawLongBits(v));for(double v:view)out.i64(Double.doubleToRawLongBits(v));
   Oracle.modelFloat(out,m.playerViewY);Oracle.modelFloat(out,m.playerViewX);
   double px=last[0]+(pos[0]-last[0])*t,py=last[1]+(pos[1]-last[1])*t,pz=last[2]+(pos[2]-last[2])*t;
   float size=radius.getFloat(r);Oracle.DataOut surfaces=new Oracle.DataOut();int ns=0;
   for(int x=MathHelper.floor_double(px-size);x<=MathHelper.floor_double(px+size);x++)for(int y=MathHelper.floor_double(py+e.getShadowSize()-size);y<=MathHelper.floor_double(py+e.getShadowSize());y++)for(int z=MathHelper.floor_double(pz-size);z<=MathHelper.floor_double(pz+size);z++){
    int id=w.getBlockId(x,y-1,z);if(id>0&&w.getBlockLightValue(x,y,z)>3){surfaces.i32(x);surfaces.i32(y);surfaces.i32(z);surfaces.i32(id);Oracle.modelFloat(surfaces,w.getLightBrightness(x,y,z));ns++;}
   }
   out.i32(ns);out.bytes(surfaces.buf.toByteArray());RendererOracle.reset(1);RendererOracle.active=true;
   if(kind<3)r.doRenderShadowAndFire(e,px-origin[0],py-origin[1],pz-origin[2],0,t);
   else r.doRender(e,px-origin[0],py-origin[1],pz-origin[2],0,t);
   RendererOracle.active=false;out.i32(RendererOracle.count);out.bytes(RendererOracle.draws.buf.toByteArray());
  }
  try(OutputStream f=new FileOutputStream(a[1])){out.writeTo(f);}
  System.out.println("managervisual9: 160 actual CaveSpider/Silverfish/XPOrb shadows and Snowball/Egg sprites");
 }
}
