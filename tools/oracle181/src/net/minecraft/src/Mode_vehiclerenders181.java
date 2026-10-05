package net.minecraft.src;
import java.io.*;import java.util.*;
public final class Mode_vehiclerenders181 {
    static void run(String path) throws Exception {
        World w=VehicleOracle.fixture();RenderManager manager=RendererOracle.manager(w);Map textures=(Map)Mode_chests181.Mode_entities181Field.get(manager.renderEngine,"textureMap");textures.put("/item/chest.png",31);textures.put("/item/largechest.png",32);
        TileEntityRenderer.instance.renderEngine=manager.renderEngine;
        RendererOracle.field("convertQuadsToTriangles").setBoolean(null,false);
        Oracle.DataOut cases=new Oracle.DataOut();int n=0;
        for(int type=-1;type<=2;type++)for(int meta=-1;meta<10;meta++)for(float partial:new float[]{0,.25F,.75F,1})for(int hurt=0;hurt<3;hurt++) {
            if(type==-1&&meta!=-1)continue;
            VehicleOracle.raw(w,0,70,-3,meta<0?0:66,meta<0?0:meta);
            Entity e=type<0?new EntityBoat(w):new EntityMinecart(w,.25,70.5,-2.75,type);
            e.setPosition(.25,70.5,-2.75);e.lastTickPosX=.1;e.lastTickPosY=70.4;e.lastTickPosZ=-2.9;
            e.prevRotationYaw=-33.7F;e.rotationYaw=123.5F;e.prevRotationPitch=11.3F;e.rotationPitch=-26.75F;
            int hit=hurt==0?0:hurt==1?10:1,damage=hurt==0?0:hurt==1?37:0,direction=hurt==1?-1:1;
            if(type<0){EntityBoat b=(EntityBoat)e;b.timeSinceHit=hit;b.damageTaken=damage;b.forwardDirection=direction;}
            else {EntityMinecart c=(EntityMinecart)e;c.minecartTimeSinceHit=hit;c.minecartCurrentDamage=damage;c.minecartRockDirection=direction;}
            float brightness=e.getEntityBrightness(partial);
            cases.i32(type);cases.i32(meta);cases.i32(hit);cases.i32(damage);cases.i32(direction);
            for(float v:new float[]{partial,brightness,e.prevRotationYaw,e.rotationYaw,e.prevRotationPitch,e.rotationPitch})Oracle.modelFloat(cases,v);
            for(double v:new double[]{e.lastTickPosX,e.lastTickPosY,e.lastTickPosZ,e.posX,e.posY,e.posZ,-.25,10.5,3.75})cases.i64(Double.doubleToRawLongBits(v));
            Render r=type<0?new RenderBoat():new RenderMinecart();r.setRenderManager(manager);
            RendererOracle.reset(brightness);RendererOracle.active=true;r.doRender(e,-.25,10.5,3.75,e.prevRotationYaw+(e.rotationYaw-e.prevRotationYaw)*partial,partial);RendererOracle.active=false;
            cases.i32(RendererOracle.count);cases.bytes(RendererOracle.draws.buf.toByteArray());n++;
        }
        Oracle.DataOut out=new Oracle.DataOut();out.i32(0x33373156);out.i32(1);out.i32(n);out.bytes(cases.buf.toByteArray());
        try(OutputStream os=new FileOutputStream(path)){out.writeTo(os);}System.out.println("vehicles: "+n+" renderer cases");
    }
 public static void main(String[] a)throws Exception{run(a[1]);}
}
