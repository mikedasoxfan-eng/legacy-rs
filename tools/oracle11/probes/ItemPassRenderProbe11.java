package net.minecraft.src;
import java.io.*;
import java.lang.reflect.*;
import java.util.*;
import org.lwjgl.opengl.GL11;
/** Execute original RenderItem only; standard RendererOracle raw draw dialect. */
public final class ItemPassRenderProbe11 {
    static Field random;
    static {try {random=RenderItem.class.getDeclaredField("random");random.setAccessible(true);}catch(Exception e){throw new RuntimeException(e);}}
    static void f(Oracle.DataOut o,float x){Oracle.modelFloat(o,x);}
    static int emit(Oracle.DataOut o,RenderManager manager,int kind,int id,int damage,int number,boolean tint,int serial)throws Exception {
        int age=serial%4*317;float hover=serial%4*.73F,partial=new float[]{0,.25F,.75F,1}[serial%4];
        float[] base={.25F,.5F,.75F,.625F};float zLevel=(serial%3-1)*50.125F;int x=12+(serial%3-1)*17,y=9+(serial%3-1)*11;
        double[] pos={-.25,10.5,3.75};
        o.i32(kind);o.i32(id);o.i32(damage);o.i32(number);o.i32(tint?1:0);o.i32(age);f(o,hover);f(o,partial);f(o,manager.playerViewY);f(o,manager.playerViewX);
        for(double d:pos)o.i64(Double.doubleToRawLongBits(d));for(float c:base)f(o,c);f(o,zLevel);o.i32(x);o.i32(y);
        RenderItem renderer=new RenderItem();renderer.setRenderManager(manager);renderer.field_27004_a=tint;renderer.zLevel=zLevel;
        Random rng=(Random)random.get(renderer);rng.setSeed(777L);
        RendererOracle.capture10=false;RendererOracle.reset(1);GL11.color=base.clone();RendererOracle.active=true;
        if(kind==0) {
            EntityItem e=(EntityItem)Oracle.allocate(EntityItem.class);e.item=new ItemStack(id,number,damage);e.age=age;e.field_804_d=hover;
            renderer.doRender(e,pos[0],pos[1],pos[2],0,partial);
        } else renderer.drawItemIntoGui(null,manager.renderEngine,id,damage,Item.itemsList[id].getIconFromDamage(damage),x,y);
        RendererOracle.active=false;o.i32(RendererOracle.count);o.bytes(RendererOracle.draws.buf.toByteArray());
        o.i64(EntityOracle.rngState(rng));o.i64(rng.nextLong());return RendererOracle.count;
    }
    public static void main(String[] args)throws Exception {
        Block.stone.getClass();RenderManager manager=RendererOracle.manager(null);RendererOracle.field("convertQuadsToTriangles").setBoolean(null,false);
        TreeSet<Integer> eggs=new TreeSet<Integer>();for(Object id:EntityList.field_44041_a.keySet())eggs.add((Integer)id);
        if(eggs.size()!=20)throw new AssertionError("Expected20original eggs");
        for(int damage:new int[]{-1,0,1,49,53,63,119,121,32768,Integer.MIN_VALUE,Integer.MAX_VALUE})eggs.add(damage);
        Oracle.DataOut cases=new Oracle.DataOut();int serial=0,draws=0;
        for(int kind=0;kind<2;kind++)for(boolean tint:new boolean[]{false,true})for(int number:new int[]{1,2,6,21}) {
            for(int damage:eggs)draws+=emit(cases,manager,kind,383,damage,number,tint,serial++);
            for(int damage:new int[]{-1,0,1,15,16,8193,16385,32767,Integer.MIN_VALUE,Integer.MAX_VALUE})draws+=emit(cases,manager,kind,373,damage,number,tint,serial++);
            for(int damage:new int[]{-1,0,1,15,Integer.MIN_VALUE,Integer.MAX_VALUE})draws+=emit(cases,manager,kind,332,damage,number,tint,serial++);
        }
        Oracle.DataOut out=new Oracle.DataOut();out.i32(0x50313152);out.i32(1);out.i32(serial);out.bytes(cases.buf.toByteArray());
        try(OutputStream stream=new FileOutputStream(args[0])){out.writeTo(stream);}
        System.out.println("ItemPassRender11: "+serial+" cases, "+draws+" standard raw draws");
    }
}
