package net.minecraft.src;
import java.io.*;
import java.lang.reflect.*;
import java.util.*;

/** Original live sheep getters, both model callbacks and the complete RenderSheep draw. */
public final class SheepRender11 {
    public static void main(String[] args)throws Exception {
        World world=Mode_entities10.world();RenderManager manager=RendererOracle.manager(world);
        Map textures=(Map)Oracle.getField(manager.renderEngine,"textureMap");
        textures.put("/mob/sheep.png",50);textures.put("/mob/sheep_fur.png",51);
        RendererOracle.field("convertQuadsToTriangles").setBoolean(null,false);
        Field timer=EntitySheep.class.getDeclaredField("field_44004_b");timer.setAccessible(true);
        int[] times={-1,0,1,2,3,4,5,20,35,36,37,38,39,40,41};
        float[] partials={0,.25F,.75F,1};
        ModelSheep2 body=new ModelSheep2();ModelSheep1 wool=new ModelSheep1();
        RenderSheep renderer=new RenderSheep(new ModelSheep2(),new ModelSheep1(),.7F);renderer.setRenderManager(manager);
        Oracle.DataOut out=new Oracle.DataOut();out.i32(0x534d3131);out.i32(times.length*partials.length*8);
        int index=0;
        for(int time:times)for(float partial:partials)for(boolean child:new boolean[]{false,true})for(boolean sheared:new boolean[]{false,true})for(boolean hurt:new boolean[]{false,true}) {
            EntitySheep e=new EntitySheep(world);e.setDelay(child?-24000:0);e.setSheared(sheared);e.setFleeceColor(index%16);
            timer.setInt(e,time);e.entityId=337*(index++);e.setPosition(.25,70.5,-2.75);
            e.lastTickPosX=.1;e.lastTickPosY=70.4;e.lastTickPosZ=-2.9;
            e.prevRenderYawOffset=-33;e.renderYawOffset=17;e.prevRotationYaw=-75;e.rotationYaw=35;e.prevRotationPitch=-12.25F;e.rotationPitch=37.5F;
            e.field_705_Q=.2F;e.field_704_R=.9F;e.field_703_S=3.75F;e.ticksExisted=31;e.prevSwingProgress=.15F;e.swingProgress=.65F;e.hurtTime=hurt?3:0;
            out.i32(time);Oracle.modelFloat(out,partial);out.i32(child?1:0);out.i32(sheared?1:0);out.i32(hurt?1:0);out.i32(e.getFleeceColor());
            Oracle.modelFloat(out,e.getEntityBrightness(partial));int packed=e.getEntityBrightnessForRender(partial);out.i32(packed);
            Oracle.modelFloat(out,e.func_44003_c(partial));Oracle.modelFloat(out,e.func_44002_d(partial));
            float amount=.2F+(.9F-.2F)*partial,swing=3.75F-.9F*(1-partial);if(child)swing*=3;
            for(ModelQuadruped model:new ModelQuadruped[]{body,wool}) {
                model.setLivingAnimations(e,swing,amount,partial);
                model.setRotationAngles(swing,amount,31+partial,60*partial-42,-12.25F+(37.5F+12.25F)*partial,.0625F);
                Oracle.modelFloat(out,model.head.rotationPointY);Oracle.modelFloat(out,model.head.rotateAngleX);Oracle.modelFloat(out,model.head.rotateAngleY);
            }
            RendererOracle.reset(1);Mode_generaliving4_10.LightRecorder light=new Mode_generaliving4_10.LightRecorder();RendererOracle.draws=light;
            org.lwjgl.opengl.GL13.glMultiTexCoord2f(33985,packed%65536,packed/65536);
            RendererOracle.active=true;
            renderer.doRender(e,.1+(.25-.1)*partial,70.4+(70.5-70.4)*partial-64,-2.9+(-2.75+2.9)*partial,0,partial);
            RendererOracle.active=false;out.i32(RendererOracle.count);out.bytes(RendererOracle.draws.buf.toByteArray());
            out.i32(light.lights.size());for(int value:light.lights)out.i32(value);
        }
        try(OutputStream file=new FileOutputStream(args[0])){out.writeTo(file);}
        System.out.println("SHEEP RENDER ORIGINAL: "+index+" live getter/model/renderer cases");
    }
}
