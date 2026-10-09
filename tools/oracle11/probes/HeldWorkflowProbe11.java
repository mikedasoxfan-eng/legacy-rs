package net.minecraft.src;
import java.io.*;import java.util.*;
public final class HeldWorkflowProbe11 {
 public static void main(String[] args)throws Exception {
  Block.stone.getClass();RendererOracle.field("convertQuadsToTriangles").setBoolean(null,false);
  TreeSet<Integer> eggs=new TreeSet<Integer>();for(Object id:EntityList.field_44041_a.keySet())eggs.add((Integer)id);for(int d:new int[]{-1,0,1,49,53,63,119,121,32768,Integer.MIN_VALUE,Integer.MAX_VALUE})eggs.add(d);
  Oracle.DataOut out=new Oracle.DataOut();int n=0;RenderPlayer persistent=new RenderPlayer();
  for(int id:new int[]{383,373,332})for(int damage:id==383?eggs:id==373?Arrays.asList(-1,0,1,15,16,8193,16385,32767,Integer.MIN_VALUE,Integer.MAX_VALUE):Arrays.asList(-1,0,1,15,Integer.MIN_VALUE,Integer.MAX_VALUE)) {
   if(new ItemStack(id,1,damage).func_40713_r())continue;
   for(int c=0;c<4;c++) {
    World w=FirstPersonOracle.world();PlayerOracle.HeadlessPlayer p=FirstPersonOracle.player(w,c+1);FirstPersonOracle.setup(p);p.hurtTime=c==2?7:0;
    out.i32(1);FirstPersonOracle.state(out,p);float t=new float[]{0,.25F,.75F,1}[c];FirstPersonOracle.f(out,t);FirstPersonOracle.settings(out,p,1);
    p.mc.gameSettings.anaglyph=false;p.mc.gameSettings.thirdPersonView=0;p.mc.gameSettings.debugCamEnable=false;
    ItemStack stack=new ItemStack(id,1,damage);out.i32(id);out.i32(damage);FirstPersonOracle.f(out,c*.23F);FirstPersonOracle.f(out,1-c*.19F);
    float brightness=w.getLightBrightness(MathHelper.floor_double(p.posX),MathHelper.floor_double(p.posY),MathHelper.floor_double(p.posZ));FirstPersonOracle.f(out,brightness);FirstPersonOracle.f(out,p.getEntityBrightness(1));
    Oracle.setField(p.mc.entityRenderer.itemRenderer,"itemToRender",stack);Oracle.setField(p.mc.entityRenderer.itemRenderer,"prevEquippedProgress",c*.23F);Oracle.setField(p.mc.entityRenderer.itemRenderer,"equippedProgress",1-c*.19F);
    FirstPersonOracle.start(brightness);FirstPersonOracle.call(p.mc.entityRenderer,"renderHand",new Class[]{float.class,int.class},t,0);FirstPersonOracle.stop();FirstPersonOracle.draws(out);n++;
    w=FirstPersonOracle.world();p=FirstPersonOracle.player(w,c+1);RenderManager manager=FirstPersonOracle.setup(p);p.movementInput.sneak=c%2==0;p.hurtTime=c==2?7:0;
    if(c%2==1)p.ridingEntity=new EntityPig(w);p.inventory.mainInventory[0]=new ItemStack(id,1,damage);persistent.setRenderManager(manager);((Map)Oracle.getField(manager,"entityRenderMap")).put(EntityPlayer.class,persistent);
    out.i32(3);FirstPersonOracle.state(out,p);out.i32(p.ridingEntity==null?0:1);out.i32(1);FirstPersonOracle.f(out,t);out.i32(id);out.i32(damage);for(int a=0;a<4;a++)out.i32(0);out.i32(0);FirstPersonOracle.f(out,p.getEntityBrightness(t));double[] camera={-.5,61.25,-3.75};for(double d:camera)FirstPersonOracle.d(out,d);
    FirstPersonOracle.start(p.getEntityBrightness(t));persistent.doRender(p,p.lastTickPosX+(p.posX-p.lastTickPosX)*t-camera[0],p.lastTickPosY+(p.posY-p.lastTickPosY)*t-camera[1],p.lastTickPosZ+(p.posZ-p.lastTickPosZ)*t-camera[2],0,t);FirstPersonOracle.stop();FirstPersonOracle.draws(out);n++;
   }
  }
  Oracle.DataOut file=new Oracle.DataOut();file.bytes(new byte[]{'F','1','7','3'});file.i32(1);file.i32(n);file.bytes(out.buf.toByteArray());try(OutputStream stream=new FileOutputStream(args[0])){file.writeTo(stream);}System.out.println("HeldWorkflow11:"+n+" original first/third-person frame cases");
 }
}

