package net.minecraft.src;
import java.io.*;import java.util.*;
/** Arbitrary held stacks through original RenderBiped and ItemRenderer. */
public final class Mode_bipedequipment5_181 {
 static final class HoldingZombie extends EntityZombie {ItemStack held;HoldingZombie(World w){super(w);}public ItemStack getHeldItem(){return held;}}
 public static void main(String[] a)throws Exception {
  World w=Mode_entities181.world();RenderManager manager=RendererOracle.manager(w);
  net.minecraft.client.Minecraft mc=(net.minecraft.client.Minecraft)Oracle.allocate(MinecraftImpl.class);mc.renderEngine=manager.renderEngine;manager.itemRenderer=(ItemRenderer)Oracle.allocate(ItemRenderer.class);Oracle.setField(manager.itemRenderer,"mc",mc);Oracle.setField(manager.itemRenderer,"renderBlocksInstance",new RenderBlocks());
  Map textures=(Map)Mode_chests181.Mode_entities181Field.get(manager.renderEngine,"textureMap");textures.put("/mob/zombie.png",7);textures.put("/item/chest.png",31);textures.put("/item/largechest.png",32);TileEntityRenderer.instance.renderEngine=manager.renderEngine;
  RendererOracle.field("convertQuadsToTriangles").setBoolean(null,false);
  int[] items={1,5,17,18,20,35,44,54,58,86,98,109,261,267,280,260,352,351,358};int[] cases={0,1,8,10};int[] damage={0,1,3,15};
  Oracle.DataOut out=new Oracle.DataOut();out.i32(items.length*cases.length*damage.length);
  for(int id:items)for(int meta:damage)for(int c:cases) {
   HoldingZombie e=new HoldingZombie(w);e.held=new ItemStack(id,1,meta);RenderBiped renderer=new RenderBiped(new ModelZombie(),.5F);renderer.setRenderManager(manager);e.entityId=c*337;e.setPosition(.25,70.5,-2.75);
   e.prevRenderYawOffset=-33;e.renderYawOffset=17;e.prevRotationYaw=-75;e.rotationYaw=35;e.prevRotationPitch=25;e.rotationPitch=-12;e.field_705_Q=.2F;e.field_704_R=.9F;e.field_703_S=3.75F;e.ticksExisted=c*127;e.prevSwingProgress=.15F;e.swingProgress=.65F;e.hurtTime=c%4==1?3:0;e.deathTime=c%4==2?c:0;
   float partial=new float[]{0,.25F,.75F,1}[c%4],brightness=e.getEntityBrightness(partial);int packed=e.func_35115_a(partial);
   out.i32(17);out.i32(c);out.i32(0);Oracle.modelFloat(out,partial);Oracle.modelFloat(out,brightness);out.i32(packed);out.i32(id);out.i32(meta);
   RendererOracle.reset(1);Mode_generaliving4_181.LightRecorder light=new Mode_generaliving4_181.LightRecorder();RendererOracle.draws=light;org.lwjgl.opengl.GL13.glMultiTexCoord2f(33985,packed%65536,packed/65536);if(c>=8)org.lwjgl.opengl.GL11.glEnable(2977);RendererOracle.active=true;renderer.doRender(e,-.25,10.5,3.75,0,partial);RendererOracle.active=false;
   out.i32(RendererOracle.count);out.bytes(RendererOracle.draws.buf.toByteArray());out.i32(light.lights.size());for(int value:light.lights)out.i32(value);
  }
  try(OutputStream f=new FileOutputStream(a[1])){out.writeTo(f);}
 }
}
