package net.minecraft.src;
import java.io.*;
public final class Mode_arrows10 {
 public static void main(String[] a)throws Exception {
  World w=Mode_entities10.world();RenderManager manager=RendererOracle.manager(w);RendererOracle.field("convertQuadsToTriangles").setBoolean(null,false);RenderArrow r=new RenderArrow();r.setRenderManager(manager);Oracle.DataOut out=new Oracle.DataOut();out.i32(32*4*4*2);
  for(int c=0;c<32;c++)for(int shake:new int[]{0,1,7,30})for(float partial:new float[]{0,.25F,.75F,1})for(boolean crit:new boolean[]{false,true}) {
   EntityArrow e=new EntityArrow(w);e.prevRotationYaw=c==0?0:c*13.75F-100;e.rotationYaw=c==0?0:e.prevRotationYaw+57;e.prevRotationPitch=c==0?0:c*3.5F-40;e.rotationPitch=c==0?0:e.prevRotationPitch-13;e.arrowShake=shake;e.arrowCritical=crit;
   Oracle.modelFloat(out,e.prevRotationYaw+(e.rotationYaw-e.prevRotationYaw)*partial);Oracle.modelFloat(out,e.prevRotationPitch+(e.rotationPitch-e.prevRotationPitch)*partial);out.i32(shake);Oracle.modelFloat(out,partial);
   RendererOracle.reset(1);RendererOracle.active=true;r.doRender(e,-.25,10.5,3.75,0,partial);RendererOracle.active=false;out.i32(RendererOracle.count);out.bytes(RendererOracle.draws.buf.toByteArray());
  }
  try(OutputStream f=new FileOutputStream(a[1])){out.writeTo(f);}
 }
}
