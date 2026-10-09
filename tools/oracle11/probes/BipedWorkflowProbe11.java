package net.minecraft.src;
import java.io.*;
import java.util.*;

/** Original 1.1 RenderBiped.renderEquippedItems: the potion second pass and held-item glint
 * (ItemRenderer.renderItem with a non-player entity), with the texture matrix of every draw. */
public final class BipedWorkflowProbe11 {
    static final class HoldingZombie extends EntityZombie {ItemStack held;HoldingZombie(World w){super(w);}public ItemStack getHeldItem(){return held;}}
    public static void main(String[] a)throws Exception {
        Block.stone.getClass(); World w=Mode_entities10.world();RenderManager manager=RendererOracle.manager(w);
        net.minecraft.client.Minecraft mc=(net.minecraft.client.Minecraft)Oracle.allocate(MinecraftImpl.class);mc.renderEngine=manager.renderEngine;
        manager.itemRenderer=(ItemRenderer)Oracle.allocate(ItemRenderer.class);Oracle.setField(manager.itemRenderer,"mc",mc);Oracle.setField(manager.itemRenderer,"renderBlocksInstance",new RenderBlocks());
        Map textures=(Map)Oracle.getField(manager.renderEngine,"textureMap");textures.put("/mob/zombie.png",7);TileEntityRenderer.instance.renderEngine=manager.renderEngine;
        RendererOracle.field("convertQuadsToTriangles").setBoolean(null,false);
        int[][] items={{373,0},{373,16},{373,8193},{373,16385},{373,8226},{373,32767},{322,0},{267,-1},{261,-1},{276,0},{1,0},{50,0}};
        ArrayList<int[]> expanded=new ArrayList<int[]>(); for(int[] item:items)expanded.add(item); TreeSet<Integer> eggs=new TreeSet<Integer>(); for(Object id:EntityList.field_44041_a.keySet())eggs.add((Integer)id); for(int meta:new int[]{0,1,49,53,63,119,121,32768,Integer.MAX_VALUE})eggs.add(meta); for(int meta:eggs)expanded.add(new int[]{383,meta}); items=expanded.toArray(new int[expanded.size()][]); int[] cases={0,1,8,10};
        Oracle.DataOut out=new Oracle.DataOut();out.i32(0x34425052);out.i32(items.length*cases.length);
        for(int[] item:items)for(int c:cases) {
            HoldingZombie e=new HoldingZombie(w);int id=item[0],meta=Math.max(item[1],0);e.held=new ItemStack(id,1,meta);if(item[1]<0)e.held.addEnchantment(Enchantment.sharpness,2);
            RenderBiped renderer=new RenderBiped(new ModelZombie(),.5F);renderer.setRenderManager(manager);e.entityId=c*337;e.setPosition(.25,70.5,-2.75);
            e.prevRenderYawOffset=-33;e.renderYawOffset=17;e.prevRotationYaw=-75;e.rotationYaw=35;e.prevRotationPitch=25;e.rotationPitch=-12;e.field_705_Q=.2F;e.field_704_R=.9F;e.field_703_S=3.75F;e.ticksExisted=c*127;e.prevSwingProgress=.15F;e.swingProgress=.65F;e.hurtTime=c%4==1?3:0;e.deathTime=c%4==2?c:0;
            float partial=new float[]{0,.25F,.75F,1}[c%4],brightness=e.getEntityBrightness(partial);int packed=e.getEntityBrightnessForRender(partial);
            out.i32(c);Oracle.modelFloat(out,partial);Oracle.modelFloat(out,brightness);out.i32(packed);out.i32(id);out.i32(meta);out.i32(item[1]<0?1:0);
            RendererOracle.reset(1);Mode_generaliving4_10.LightRecorder light=new Mode_generaliving4_10.LightRecorder();RendererOracle.draws=light;
            org.lwjgl.opengl.GL13.glMultiTexCoord2f(33985,packed%65536,packed/65536);if(c>=8)org.lwjgl.opengl.GL11.glEnable(2977);
            GlLog10.reset();GlLog10.active=true;RendererOracle.active=true;renderer.doRender(e,-.25,10.5,3.75,0,partial);RendererOracle.active=false;GlLog10.active=false;
            out.i32(GlLog10.drawTexture.size());for(ArrayList<float[]> list:GlLog10.drawTexture){out.i32(list.size());for(float[] o:list){out.i32((int)o[0]);for(int k=1;k<5;k++)Oracle.modelFloat(out,o[k]);}}
            out.i32(RendererOracle.count);out.bytes(RendererOracle.draws.buf.toByteArray());out.i32(light.lights.size());for(int value:light.lights)out.i32(value);
        }
        try(OutputStream f=new FileOutputStream(a[0])){out.writeTo(f);}
        System.out.println("Original 1.1 RenderBiped held items: "+items.length*cases.length+" cases");
    }
}
