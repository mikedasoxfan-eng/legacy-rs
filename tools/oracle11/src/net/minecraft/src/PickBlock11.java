package net.minecraft.src;
import net.minecraft.client.Minecraft;import java.io.*;import java.lang.reflect.*;
/** Actual original Minecraft.clickMiddleMouseButton and InventoryPlayer.setCurrentItem. */
public final class PickBlock11 {
 static class C extends Minecraft {C(){super(null,null,null,1,1,false);}public void displayUnexpectedThrowable(UnexpectedThrowable t){throw new RuntimeException();}}
 static class W extends World {int id,meta;W(){super((ISaveHandler)null,"",new WorldSettings(0L,0,false,false,EnumWorldType.DEFAULT));}public int getBlockId(int x,int y,int z){return id;}public int getBlockMetadata(int x,int y,int z){return meta&15;}}
 static class I extends InventoryPlayer {int calls,id,meta,subtypes,creative;I(EntityPlayer p){super(p);}public void setCurrentItem(int id,int meta,boolean subtypes,boolean creative){calls++;this.id=id;this.meta=meta;this.subtypes=subtypes?1:0;this.creative=creative?1:0;super.setCurrentItem(id,meta,subtypes,creative);}}
 public static void main(String[] args)throws Exception{int boot=Block.stone.blockID;Method pick=Minecraft.class.getDeclaredMethod("clickMiddleMouseButton");pick.setAccessible(true);PrintWriter o=new PrintWriter(new File(args[0],"pickblock11.txt"));
 for(int id=0;id<256;id++)for(int meta:new int[]{-1,0,1,7,32767})for(int creative=0;creative<2;creative++)for(int hit=0;hit<3;hit++)for(int scene=0;scene<4;scene++){
  W w=(W)SheepSlice11.unsafe().allocateInstance(W.class);w.id=id;w.meta=meta;Minecraft mc=(Minecraft)SheepSlice11.unsafe().allocateInstance(C.class);EntityPlayerSP p=(EntityPlayerSP)SheepSlice11.unsafe().allocateInstance(EntityPlayerSP.class);I inv=new I(p);p.inventory=inv;inv.currentItem=3;int target=id==2?3:id==43?44:id==7?1:id;if(scene!=0){inv.mainInventory[scene==2?9:7]=new ItemStack(target,1,scene==3?meta+1:meta);}mc.thePlayer=p;mc.theWorld=w;mc.playerController=creative!=0?(PlayerControllerCreative)SheepSlice11.unsafe().allocateInstance(PlayerControllerCreative.class):null;mc.objectMouseOver=hit==0?null:hit==1?new MovingObjectPosition(1,64,1,0,Vec3D.createVector(0,0,0)):new MovingObjectPosition(new EntityPig(w));pick.invoke(mc);o.println(id+" "+meta+" "+creative+" "+hit+" "+scene+" "+inv.currentItem+" "+inv.calls+" "+inv.id+" "+inv.meta+" "+inv.subtypes+" "+inv.creative);
 }o.close();
 }
}
