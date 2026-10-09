package net.minecraft.src;
import java.io.*;import java.util.*;
/** RenderPlayer body/armor/items/arrows; label is suppressed for the viewing player. */
public final class Mode_playerliving4_10 {
 public static void main(String[] a)throws Exception{
  World w=Mode_entities10.world();RenderManager manager=RendererOracle.manager(w);
  net.minecraft.client.Minecraft mc=(net.minecraft.client.Minecraft)Oracle.allocate(MinecraftImpl.class);mc.renderEngine=manager.renderEngine;manager.itemRenderer=(ItemRenderer)Oracle.allocate(ItemRenderer.class);Oracle.setField(manager.itemRenderer,"mc",mc);Oracle.setField(manager.itemRenderer,"renderBlocksInstance",new RenderBlocks());
  Map textures=(Map)Oracle.getField(manager.renderEngine,"textureMap");textures.put("/mob/char.png",9);
  String[] armor={"cloth","chain","iron","diamond","gold"};for(int j=0;j<5;j++)for(int layer=1;layer<=2;layer++)textures.put("/armor/"+armor[j]+"_"+layer+".png",20+j*2+layer-1);
  Map renderers=new HashMap();RenderArrow arrow=new RenderArrow();arrow.setRenderManager(manager);renderers.put(EntityArrow.class,arrow);Oracle.setField(manager,"entityRenderMap",renderers);
  RendererOracle.field("convertQuadsToTriangles").setBoolean(null,false);RenderPlayer renderer=new RenderPlayer();renderer.setRenderManager(manager);
  Oracle.DataOut out=new Oracle.DataOut();out.i32(8*6*4*2*2);
  for(int c=0;c<8;c++)for(int material=0;material<6;material++)for(int id:new int[]{1,267,280,260})for(int arrows:new int[]{0,3})for(boolean using:new boolean[]{false,true}){
   EntityPlayer e=new Mode_entities10.Player(w);manager.livingPlayer=e;e.username="Player";e.entityId=c*337;e.arrowHitTempCounter=arrows;e.setPosition(-.25,10.5,3.75);e.lastTickPosX=e.posX;e.lastTickPosY=e.posY;e.lastTickPosZ=e.posZ;
   e.prevRenderYawOffset=-33;e.renderYawOffset=17;e.prevRotationYaw=-75;e.rotationYaw=35;e.prevRotationPitch=25;e.rotationPitch=-12;
   e.field_705_Q=.2F;e.field_704_R=.9F;e.field_703_S=3.75F;e.ticksExisted=c*127;e.prevSwingProgress=.15F;e.swingProgress=.65F;e.hurtTime=c%4==1?3:0;e.deathTime=c%4==2?c:0;
   e.dataWatcher.updateObject(0,Byte.valueOf((byte)(c%3==0?2:0)));if(c%2!=0)e.ridingEntity=new EntityPig(w);
   e.inventory.mainInventory[0]=new ItemStack(id,1,0);for(int j=0;j<4;j++)e.inventory.armorInventory[j]=material==5?null:new ItemStack(298+material*4+3-j,1,0);
   if(using){java.lang.reflect.Field used=EntityPlayer.class.getDeclaredField("itemInUseCount");used.setAccessible(true);used.setInt(e,20);}float partial=new float[]{0,.25F,.75F,1}[c%4];float brightness=.37F;
   // Entity brightness at this position is recorded separately from white GL primary color.
   brightness=e.getEntityBrightness(partial);out.i32(c);out.i32(material);out.i32(id);out.i32(arrows);out.i32(using?1:0);Oracle.modelFloat(out,partial);Oracle.modelFloat(out,brightness);
   RendererOracle.reset(1);RendererOracle.active=true;renderer.doRender(e,-.25,10.5,3.75,0,partial);RendererOracle.active=false;out.i32(RendererOracle.count);out.bytes(RendererOracle.draws.buf.toByteArray());
  }
  try(OutputStream f=new FileOutputStream(a[1])){out.writeTo(f);}
 }
}
