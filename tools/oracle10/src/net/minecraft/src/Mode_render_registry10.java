package net.minecraft.src;
import java.io.*;
import java.lang.reflect.*;
import java.util.*;
import static net.minecraft.src.Mode_render_models10.*;
/** Constructor registration and inherited lookups on original managers. */
public final class Mode_render_registry10 {
    public static void main(String[] args)throws Exception{
        Class.forName("net.minecraft.src.Block");out=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(args[0])));i(0x30315252);i(1);
        Field field=RenderManager.class.getDeclaredField("entityRenderMap");field.setAccessible(true);Map<Class,Render> map=(Map<Class,Render>)field.get(RenderManager.instance);
        ArrayList<Class> keys=new ArrayList<Class>(map.keySet());Collections.sort(keys,new Comparator<Class>(){public int compare(Class a,Class b){return a.getSimpleName().compareTo(b.getSimpleName());}});
        i(keys.size());for(Class key:keys){Render r=map.get(key);s(key.getSimpleName());s(r.getClass().getSimpleName());f(r.shadowSize);f(r.field_194_c);}
        s(RenderManager.instance.getEntityClassRenderObject(EntityPigZombie.class).getClass().getSimpleName());
        field=TileEntityRenderer.class.getDeclaredField("specialRendererMap");field.setAccessible(true);Map<Class,TileEntitySpecialRenderer> tiles=(Map<Class,TileEntitySpecialRenderer>)field.get(TileEntityRenderer.instance);
        keys=new ArrayList<Class>(tiles.keySet());Collections.sort(keys,new Comparator<Class>(){public int compare(Class a,Class b){return a.getSimpleName().compareTo(b.getSimpleName());}});
        i(keys.size());for(Class key:keys){s(key.getSimpleName());s(tiles.get(key).getClass().getSimpleName());}
        out.close();System.out.println("Original RenderManager and TileEntityRenderer registrations and PigZombie fallback");
    }
}
