package net.minecraft.src;
import java.io.*;
import java.util.*;

/** Probe only EffectRenderer's original bytecode, with observable virtual FX callbacks. */
public final class Mode_effectdispatch8_10 {
 static DataOutputStream out;
 static final class Probe extends EntityFX {
  final int id, layer, light;
  Probe(World w,int id,int layer) {
   super(w,.1,id+.2,.1,0,0,0);this.id=id;this.layer=layer;
   light=((id&15)*16<<16)|((15-(id&15))*16);
  }
  public int getFXLayer(){return layer;}
  public int getEntityBrightnessForRender(float partial){
   try{EntityOracle.i(out,0);EntityOracle.i(out,id);EntityOracle.f(out,partial);}
   catch(IOException e){throw new RuntimeException(e);}return light;
  }
  public void renderParticle(Tessellator t,float partial,float a,float b,float c,float d,float e){
   try{
    EntityOracle.i(out,1);EntityOracle.i(out,id);EntityOracle.i(out,layer);
    for(float f:new float[]{Trace10.namedFloat("partial", partial),Trace10.namedFloat("a", a),Trace10.namedFloat("b", b),Trace10.namedFloat("c", c),Trace10.namedFloat("d", d),Trace10.namedFloat("e", e)})EntityOracle.f(out,f);
    for(double f:new double[]{Trace10.namedDouble("interpPosX", interpPosX),Trace10.namedDouble("interpPosY", interpPosY),Trace10.namedDouble("interpPosZ", interpPosZ)})EntityOracle.d(out,f);
    EntityOracle.i(out,Trace10.namedInt("((Integer)Oracle.getField(t,\"brightness\")).intValue()", ((Integer)Oracle.getField(t,"brightness")).intValue()));
    for(float f:org.lwjgl.opengl.GL11.color)EntityOracle.f(out,f);
    // Subsequent callbacks in this layer must observe the mutation. Only
    // the next nonempty regular layer resets GL color to white.
    if(id%8==0)org.lwjgl.opengl.GL11.color=new float[]{.1F,.2F,.3F,.4F};
   }catch(Exception ex){throw new RuntimeException(ex);}
  }
 }
 /** 1.0 EffectRenderer.renderParticles reads ActiveRenderInfo's statics, which
  * EntityRenderer sets once per frame from thePlayer (mirrored in front view). */
 static void active(float yaw,float pitch,boolean mirror){
  int m=mirror?1:0;
  ActiveRenderInfo.field_41070_d=MathHelper.cos(yaw*(float)Math.PI/180.0F)*(float)(1-m*2);
  ActiveRenderInfo.field_41068_f=MathHelper.sin(yaw*(float)Math.PI/180.0F)*(float)(1-m*2);
  ActiveRenderInfo.field_41069_g=-ActiveRenderInfo.field_41068_f*MathHelper.sin(pitch*(float)Math.PI/180.0F)*(float)(1-m*2);
  ActiveRenderInfo.field_41078_h=ActiveRenderInfo.field_41070_d*MathHelper.sin(pitch*(float)Math.PI/180.0F)*(float)(1-m*2);
  ActiveRenderInfo.field_41071_e=MathHelper.cos(pitch*(float)Math.PI/180.0F);
 }
 static void fill(EffectRenderer r,World w,int mask,int count){
  // Deliberately interleave admission; renderer must regroup ordinary layers,
  // while retaining insertion order within each layer (including layer 3).
  for(int j=0;j<count;j++)for(int layer=3;layer>=0;layer--)
   if((mask&(1<<layer))!=0)r.addEffect(new Probe(w,layer*8+j,layer));
 }
 public static void main(String[] a)throws Exception {
  World w=Mode_entities10.world();RenderEngine engine=RendererOracle.manager(w).renderEngine;
  ((Map)Oracle.getField(engine,"textureMap")).put("/particles.png",40);
  EntityPig view=new EntityPig(w);
  try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))){
   out=o;int cases=0;
   for(int mask:new int[]{0,1,8,10,15})for(int count:new int[]{1,6})
    for(float yaw:new float[]{0,37.25F,-280.125F,1.0e10F})for(float pitch:new float[]{0,-21.5F,90})
     for(float partial:new float[]{-.25F,0,.375F,1,1.25F}){
      EffectRenderer r=new EffectRenderer(w,engine);fill(r,w,mask,count);
      EntityFX.interpPosX=19.125;EntityFX.interpPosY=-3.25;EntityFX.interpPosZ=7.75;
      org.lwjgl.opengl.GL11.color=new float[]{.25F,.5F,.75F,.125F};
      view.rotationYaw=yaw;view.rotationPitch=pitch;
      view.lastTickPosX=-1e8;view.posX=.125;view.lastTickPosY=85.1;view.posY=80.7;view.lastTickPosZ=-3.75;view.posZ=7.1;
      r.func_1187_b(view,partial);active(yaw,pitch,count==6);r.renderParticles(view,partial);r.func_1187_b(view,partial);
      r.clearEffects(null);r.func_1187_b(view,partial);
      fill(r,w,mask,count);r.func_1187_b(view,partial);
      view.rotationYaw=-yaw;view.rotationPitch=-pitch;
      active(-yaw,-pitch,count==6);r.renderParticles(view,1-partial);r.func_1187_b(view,1-partial);
      cases++;
     }
   System.out.println("effectdispatch8: "+cases+" original-bytecode pass sequences");
  }
 }
}
