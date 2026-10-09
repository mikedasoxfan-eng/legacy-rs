package net.minecraft.src;
import java.io.*;
import java.util.*;
import org.lwjgl.opengl.GL11;
/** Original ItemRenderer.renderItem primitive, not an invented full held workflow. */
public final class HeldItemPassProbe11 {
    public static void main(String[] args)throws Exception {
        Block.stone.getClass();RenderManager manager=RendererOracle.manager(null);
        net.minecraft.client.Minecraft mc=(net.minecraft.client.Minecraft)Oracle.allocate(MinecraftImpl.class);mc.renderEngine=manager.renderEngine;
        ItemRenderer renderer=(ItemRenderer)Oracle.allocate(ItemRenderer.class);Oracle.setField(renderer,"mc",mc);
        EntityPig actor=(EntityPig)Oracle.allocate(EntityPig.class);
        RendererOracle.field("convertQuadsToTriangles").setBoolean(null,false);
        TreeSet<Integer> eggs=new TreeSet<Integer>();for(Object id:EntityList.field_44041_a.keySet())eggs.add((Integer)id);
        for(int damage:new int[]{-1,0,1,49,53,63,119,121,32768,Integer.MIN_VALUE,Integer.MAX_VALUE})eggs.add(damage);
        Oracle.DataOut cases=new Oracle.DataOut();int count=0,draws=0;
        for(int id:new int[]{383,373,332}) {
            Iterable<Integer> values=id==383?eggs:id==373?Arrays.asList(-1,0,1,15,16,8193,16385,32767,Integer.MIN_VALUE,Integer.MAX_VALUE):Arrays.asList(-1,0,1,15,Integer.MIN_VALUE,Integer.MAX_VALUE);
            for(int damage:values)for(int pass:new int[]{-1,0,1,2}) {
                ItemStack stack=new ItemStack(id,1,damage);Item item=Item.itemsList[id];float[] base={.25F,.5F,.75F,.625F};
                // Leave original clock-dependent glint cases out of this finite deterministic corpus.
                if(pass==0&&stack.func_40713_r())continue;
                cases.i32(id);cases.i32(damage);cases.i32(pass);cases.i32(actor.getItemIcon(stack,pass));cases.i32(item.getColorFromDamage(damage,pass));cases.i32(item.func_46058_c()?1:0);
                for(float c:base)Oracle.modelFloat(cases,c);
                RendererOracle.capture10=false;RendererOracle.reset(1);GL11.color=base.clone();RendererOracle.active=true;
                renderer.renderItem(actor,stack,pass);RendererOracle.active=false;
                cases.i32(RendererOracle.count);cases.bytes(RendererOracle.draws.buf.toByteArray());draws+=RendererOracle.count;count++;
            }
        }
        Oracle.DataOut out=new Oracle.DataOut();out.i32(0x48313152);out.i32(1);out.i32(count);out.bytes(cases.buf.toByteArray());
        try(OutputStream stream=new FileOutputStream(args[0])){out.writeTo(stream);}
        System.out.println("HeldItemPass11: "+count+" cases, "+draws+" standard raw draws");
    }
}
