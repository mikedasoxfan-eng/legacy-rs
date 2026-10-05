package net.minecraft.src;
import java.io.*;import java.util.*;import java.lang.reflect.*;
public final class Mode_generaliving4_181 {
 static final class LightRecorder extends Oracle.DataOut {
  ArrayList<Integer> lights=new ArrayList<Integer>();int stage=0,remaining=0;
  void i32(int v){super.i32(v);if(stage==0){lights.add((int)org.lwjgl.opengl.GL13.lightmapU|((int)org.lwjgl.opengl.GL13.lightmapV<<16));stage=1;remaining=11;}else if(stage==1){if(--remaining==0)stage=2;}else if(stage==2){remaining=v*5;stage=remaining==0?4:3;}else if(stage==3){if(--remaining==0)stage=4;}else if(stage==4){remaining=v*7;stage=remaining==0?0:5;}else if(--remaining==0)stage=0;}
 }
 public static void main(String[] a)throws Exception {
  World w=Mode_entities181.world();RenderManager manager=RendererOracle.manager(w);
  net.minecraft.client.Minecraft mc=(net.minecraft.client.Minecraft)Oracle.allocate(MinecraftImpl.class);mc.renderEngine=manager.renderEngine;manager.itemRenderer=(ItemRenderer)Oracle.allocate(ItemRenderer.class);Oracle.setField(manager.itemRenderer,"mc",mc);
  Map textures=(Map)Mode_chests181.Mode_entities181Field.get(manager.renderEngine,"textureMap");
  String[] names={"skeleton","pig","saddle","cow","sheep","sheep_fur","chicken","squid","wolf","wolf_tame","wolf_angry","creeper"};for(int i=0;i<names.length;i++)textures.put("/mob/"+names[i]+".png",46+i);textures.put("/mob/zombie.png",7);textures.put("/armor/power.png",58);textures.put("/mob/ghast.png",5);textures.put("/mob/ghast_fire.png",6);textures.put("/mob/pigzombie.png",8);textures.put("/mob/slime.png",59);
  Map renderers=new HashMap();RenderArrow arrow=new RenderArrow();arrow.setRenderManager(manager);renderers.put(EntityArrow.class,arrow);Oracle.setField(manager,"entityRenderMap",renderers);
  RendererOracle.field("convertQuadsToTriangles").setBoolean(null,false);
  Oracle.DataOut out=new Oracle.DataOut();out.i32(13*16*3);
  for(int kind=4;kind<17;kind++)for(int c=0;c<16;c++)for(int arrows:new int[]{0,1,5}) {
   EntityLiving e;Render renderer;
   switch(kind){
    case 4:e=new EntityZombie(w);renderer=new RenderBiped(new ModelZombie(),.5F);break;
    case 5:e=new EntitySkeleton(w);renderer=new RenderBiped(new ModelSkeleton(),.5F);break;
    case 6:e=new EntityPig(w);renderer=new RenderPig(new ModelPig(),new ModelPig(.5F),.7F);((EntityPig)e).setSaddled((c&1)!=0);break;
    case 7:e=new EntityCow(w);renderer=new RenderCow(new ModelCow(),.7F);break;
    case 8:e=new EntitySheep(w);renderer=new RenderSheep(new ModelSheep2(),new ModelSheep1(),.7F);((EntitySheep)e).setSheared((c&1)!=0);((EntitySheep)e).setFleeceColor(c);break;
    case 9:e=new EntityChicken(w);renderer=new RenderChicken(new ModelChicken(),.3F);EntityChicken ch=(EntityChicken)e;ch.field_756_e=.2F;ch.field_752_b=1.9F;ch.field_757_d=.15F;ch.destPos=.8F;break;
    case 10:e=new EntitySquid(w);renderer=new RenderSquid(new ModelSquid(),.7F);EntitySquid sq=(EntitySquid)e;sq.field_21088_b=-17;sq.field_21089_a=35;sq.field_21086_f=12;sq.field_21087_c=-45;sq.field_21082_j=.3F;sq.field_21083_i=1.1F;break;
    case 11:e=new EntityWolf(w);renderer=new RenderWolf(new ModelWolf(),.5F);e.dataWatcher.updateObject(16,Byte.valueOf((byte)(c%8)));e.dataWatcher.updateObject(18,Integer.valueOf(c+4));Oracle.setField(e,"field_25054_c",Float.valueOf(.2F));Oracle.setField(e,"field_25048_b",Float.valueOf(.8F));Oracle.setField(e,"prevTimeWolfIsShaking",Float.valueOf(.1F));Oracle.setField(e,"timeWolfIsShaking",Float.valueOf(.6F));break;
    case 13:e=new EntityGhast(w);renderer=new RenderGhast();((EntityGhast)e).prevAttackCounter=c-5;((EntityGhast)e).attackCounter=c+5;e.texture=c%2!=0?"/mob/ghast_fire.png":"/mob/ghast.png";break;
    case 14:e=new EntityPigZombie(w);renderer=new RenderBiped(new ModelZombie(),.5F);break;
    case 15:e=new EntityGiantZombie(w);renderer=new RenderGiantZombie(new ModelZombie(),.5F,6);break;
    case 16:e=new EntitySlime(w);renderer=new RenderSlime(new ModelSlime(16),new ModelSlime(0),.25F);e.dataWatcher.updateObject(16,Byte.valueOf((byte)(1<<(c%3))));((EntitySlime)e).field_767_b=.2F;((EntitySlime)e).field_768_a=.8F;break;
    default:e=new EntityCreeper(w);renderer=new RenderCreeper();((EntityCreeper)e).lastActiveTime=c;((EntityCreeper)e).timeSinceIgnited=c+3;e.dataWatcher.updateObject(17,Byte.valueOf((byte)(c&1)));break;
   }
   renderer.setRenderManager(manager);e.entityId=c*337;e.field_35172_bP=arrows;e.setPosition(.25,70.5,-2.75);
   e.prevRenderYawOffset=-33;e.renderYawOffset=17;e.prevRotationYaw=-75;e.rotationYaw=35;e.prevRotationPitch=25;e.rotationPitch=-12;
   e.field_705_Q=.2F;e.field_704_R=.9F;e.field_703_S=3.75F;e.ticksExisted=c*127;e.prevSwingProgress=.15F;e.swingProgress=.65F;
   e.hurtTime=c%4==1?3:0;e.deathTime=c%4==2?c:0;
   float partial=new float[]{0,.25F,.75F,1}[c%4],brightness=e.getEntityBrightness(partial);
   out.i32(kind);out.i32(c);out.i32(arrows);Oracle.modelFloat(out,partial);Oracle.modelFloat(out,brightness);out.i32(e.func_35115_a(partial));
   RendererOracle.reset(1);LightRecorder light=new LightRecorder();RendererOracle.draws=light;int packed=e.func_35115_a(partial);org.lwjgl.opengl.GL13.glMultiTexCoord2f(33985,packed%65536,packed/65536);if(c>=8)org.lwjgl.opengl.GL11.glEnable(2977);RendererOracle.active=true;renderer.doRender(e,-.25,10.5,3.75,0,partial);RendererOracle.active=false;
   out.i32(RendererOracle.count);out.bytes(RendererOracle.draws.buf.toByteArray());out.i32(light.lights.size());for(int value:light.lights)out.i32(value);
  }
  try(OutputStream f=new FileOutputStream(a[1])){out.writeTo(f);}
 }
}
