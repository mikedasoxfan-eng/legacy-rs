package net.minecraft.src;
import java.io.*;import java.util.*;import java.lang.reflect.*;
public final class Mode_newliving10 {
 static final class LightRecorder extends Oracle.DataOut {
  ArrayList<Integer> lights=new ArrayList<Integer>();int stage=0,remaining=0;
  void i32(int v){super.i32(v);if(stage==0){lights.add((int)org.lwjgl.opengl.GL13.lightmapU|((int)org.lwjgl.opengl.GL13.lightmapV<<16));stage=1;remaining=11;}else if(stage==1){if(--remaining==0)stage=2;}else if(stage==2){remaining=v*5;stage=remaining==0?4:3;}else if(stage==3){if(--remaining==0)stage=4;}else if(stage==4){remaining=v*7;stage=remaining==0?0:5;}else if(--remaining==0)stage=0;}
 }
 public static void main(String[] a)throws Exception {
  World w=Mode_entities10.world();RenderManager manager=RendererOracle.manager(w);
  Map textures=(Map)Mode_chests10.Mode_entities10Field.get(manager.renderEngine,"textureMap");
  String[] names={"enderman","enderman_eyes","silverfish","spider","spider_eyes","cavespider"};for(int i=0;i<names.length;i++)textures.put("/mob/"+names[i]+".png",40+i);
  Map renderers=new HashMap();RenderArrow arrow=new RenderArrow();arrow.setRenderManager(manager);renderers.put(EntityArrow.class,arrow);Oracle.setField(manager,"entityRenderMap",renderers);
  RendererOracle.field("convertQuadsToTriangles").setBoolean(null,false);
  Oracle.DataOut out=new Oracle.DataOut();out.i32(4*16*3);
  for(int kind=0;kind<4;kind++)for(int c=0;c<16;c++)for(int arrows:new int[]{0,1,5}) {
   EntityLiving e;Render renderer;
   if(kind==0){e=new EntityEnderman(w);renderer=new RenderEnderman();((EntityEnderman)e).setCarried(c%3==0?0:c%3==1?1:35);((EntityEnderman)e).setCarryingData(c%16);((EntityEnderman)e).isAttacking=(c&1)!=0;Oracle.setField(renderer,"rnd",new Random(42+c));}
   else if(kind==1){e=new EntitySilverfish(w);renderer=new RenderSilverfish();}else {e=kind==2?new EntitySpider(w):new EntityCaveSpider(w);renderer=new RenderSpider();}
   renderer.setRenderManager(manager);e.entityId=c*337;e.arrowHitTempCounter=arrows;e.setPosition(.25,70.5,-2.75);
   e.prevRenderYawOffset=-33;e.renderYawOffset=17;e.prevRotationYaw=-75;e.rotationYaw=35;e.prevRotationPitch=25;e.rotationPitch=-12;
   e.field_705_Q=.2F;e.field_704_R=.9F;e.field_703_S=3.75F;e.ticksExisted=c*127;e.prevSwingProgress=.15F;e.swingProgress=.65F;
   e.hurtTime=c%4==1?3:0;e.deathTime=c%4==2?c:0;
   float partial=new float[]{0,.25F,.75F,1}[c%4],brightness=e.getEntityBrightness(partial);
   out.i32(kind);out.i32(c);out.i32(arrows);Oracle.modelFloat(out,partial);Oracle.modelFloat(out,brightness);out.i32(e.getEntityBrightnessForRender(partial));
   RendererOracle.reset(1);LightRecorder light=new LightRecorder();RendererOracle.draws=light;int packed=e.getEntityBrightnessForRender(partial);org.lwjgl.opengl.GL13.glMultiTexCoord2f(33985,packed%65536,packed/65536);if(c>=8)org.lwjgl.opengl.GL11.glEnable(2977);RendererOracle.active=true;renderer.doRender(e,-.25,10.5,3.75,0,partial);RendererOracle.active=false;
   out.i32(RendererOracle.count);out.bytes(RendererOracle.draws.buf.toByteArray());out.i32(light.lights.size());for(int value:light.lights)out.i32(value);
  }
  try(OutputStream f=new FileOutputStream(a[1])){out.writeTo(f);}
 }
}
