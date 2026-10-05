package net.minecraft.src;
import java.io.*;import java.util.*;
/** RenderPlayer body/armor/items/arrows; label is suppressed for the viewing player. */
public final class Mode_playerliving6_181 {
 public static void main(String[] a)throws Exception{
  World w=Mode_entities181.world();RenderManager manager=RendererOracle.manager(w);
  net.minecraft.client.Minecraft mc=(net.minecraft.client.Minecraft)Oracle.allocate(MinecraftImpl.class);mc.renderEngine=manager.renderEngine;manager.itemRenderer=(ItemRenderer)Oracle.allocate(ItemRenderer.class);Oracle.setField(manager.itemRenderer,"mc",mc);Oracle.setField(manager.itemRenderer,"renderBlocksInstance",new RenderBlocks());
  Map textures=(Map)Oracle.getField(manager.renderEngine,"textureMap");textures.put("/mob/char.png",9);
  String[] armor={"cloth","chain","iron","diamond","gold"};for(int j=0;j<5;j++)for(int layer=1;layer<=2;layer++)textures.put("/armor/"+armor[j]+"_"+layer+".png",20+j*2+layer-1);
  Map renderers=new HashMap();RenderArrow arrow=new RenderArrow();arrow.setRenderManager(manager);renderers.put(EntityArrow.class,arrow);Oracle.setField(manager,"entityRenderMap",renderers);
  RendererOracle.field("convertQuadsToTriangles").setBoolean(null,false);RenderPlayer renderer=new RenderPlayer();renderer.setRenderManager(manager);
  Oracle.DataOut out=new Oracle.DataOut();out.i32(0x18160006);out.i32(8*2*2*5*2);
  for(int c=0;c<8;c++)for(int material:new int[]{2,5})for(boolean local:new boolean[]{false,true})for(int bed=-1;bed<4;bed++)for(boolean sleep:new boolean[]{false,true}){
   int id=267;int arrows=3;boolean using=true;EntityPlayer e=local?new EntityPlayerSP(mc,w,new Session("Player",""),0):new Mode_entities181.Player(w);manager.livingPlayer=e;e.username="Player";e.entityId=c*337;e.field_35172_bP=arrows;e.setPosition(-.25,10.5,3.75);e.lastTickPosX=e.posX;e.lastTickPosY=e.posY;e.lastTickPosZ=e.posZ;
   e.health=c%4==2?0:7;java.lang.reflect.Field sl=EntityPlayer.class.getDeclaredField("sleeping");sl.setAccessible(true);sl.setBoolean(e,sleep);java.lang.reflect.Field bf=EntityPlayer.class.getDeclaredField("bedChunkCoordinates");bf.setAccessible(true);if(bed>=0){bf.set(e,new ChunkCoordinates(0,64,0));w.getChunkFromChunkCoords(0,0).setBlockMetadata(0,64,0,bed);}e.field_22063_x=.1F;e.field_22062_y=-.2F;e.field_22061_z=.3F;e.prevRenderYawOffset=-33;e.renderYawOffset=17;e.prevRotationYaw=-75;e.rotationYaw=35;e.prevRotationPitch=25;e.rotationPitch=-12;
   e.field_705_Q=.2F;e.field_704_R=.9F;e.field_703_S=3.75F;e.ticksExisted=c*127;e.prevSwingProgress=.15F;e.swingProgress=.65F;e.hurtTime=c%4==1?3:0;e.deathTime=c%4==2?c:0;
   e.dataWatcher.updateObject(0,Byte.valueOf((byte)(c%3==0?2:0)));if(local){((EntityPlayerSP)e).movementInput=new MovementInput();((EntityPlayerSP)e).movementInput.sneak=c%3==0;}if(c%2!=0)e.ridingEntity=new EntityPig(w);
   e.inventory.mainInventory[0]=new ItemStack(id,1,0);for(int j=0;j<4;j++)e.inventory.armorInventory[j]=material==5?null:new ItemStack(298+material*4+3-j,1,0);
   if(using){java.lang.reflect.Field used=EntityPlayer.class.getDeclaredField("field_34906_e");used.setAccessible(true);used.setInt(e,20);}float partial=new float[]{0,.25F,.75F,1}[c%4];float brightness=.37F;
   // Entity brightness at this position is recorded separately from white GL primary color.
   brightness=e.getEntityBrightness(partial);out.i32(c);out.i32(material);out.i32(id);out.i32(arrows);out.i32(using?1:0);Oracle.modelFloat(out,partial);Oracle.modelFloat(out,brightness);out.i32(local?1:0);out.i32(bed);out.i32(sleep?1:0);out.i32(e.health);
   RendererOracle.reset(1);RendererOracle.active=true;renderer.doRender(e,-.25,10.5,3.75,0,partial);RendererOracle.active=false;out.i32(RendererOracle.count);out.bytes(RendererOracle.draws.buf.toByteArray());
  }
  try(OutputStream f=new FileOutputStream(a[1])){out.writeTo(f);}
 }
}
