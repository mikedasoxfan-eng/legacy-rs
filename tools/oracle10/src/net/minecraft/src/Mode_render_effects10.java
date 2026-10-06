package net.minecraft.src;
import java.io.*;
import java.util.*;
import java.lang.reflect.Field;
import org.lwjgl.opengl.GL11;
import static net.minecraft.src.Mode_render_models10.*;
/** Original potion billboards and dragon death-ray callbacks, with graphics endpoints recorded. */
public final class Mode_render_effects10 {
    static final class Sprite extends RenderSnowball {
        Sprite(int icon){super(icon);setRenderManager(RenderManager.instance);}
        protected void loadTexture(String name){if(!name.equals("/gui/items.png"))throw new AssertionError(name);GL11.texture=2;}
    }
    static final class Fireball extends RenderFireball {
        Fireball(float scale){super(scale);setRenderManager(RenderManager.instance);}
        protected void loadTexture(String name){if(!name.equals("/gui/items.png"))throw new AssertionError(name);GL11.texture=2;}
    }
    static void ops(ArrayList<GL11.Op> ops)throws Exception{i(ops.size());for(GL11.Op o:ops){i(o.code);for(float a:o.args)f(a);}}
    static void draws()throws Exception{
        i(GL11.draws.size());
        for(GL11.Draw v:GL11.draws){ops(v.ops);i(v.cull);i(v.flags);i(v.texture);i(v.src);i(v.dst);i(v.depth);i(v.face.mode);f(v.alpha);for(float a:v.material)f(a);i(v.face.vertices.size());
            for(int n=0;n<v.face.vertices.size();n++){i(v.face.normal);i(v.face.colors.get(n));for(float a:v.face.vertices.get(n))f(a);}}
        ops(GL11.ops);i(GL11.flags);i(GL11.src);i(GL11.dst);i(GL11.depth);f(GL11.alpha);for(float a:GL11.material)f(a);
    }
    public static void main(String[] args)throws Exception{
        int kind=Integer.parseInt(args[0]);out=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(args[1])));i(0x30314552);i(1);i(kind);i(64);
        Random random=new Random(411001+kind);
        if(kind!=1)Class.forName("net.minecraft.src.Block");
        RenderDragon renderer=kind==1?new RenderDragon():null;EntityDragon dragon=kind==1?new EntityDragon(null):null;
        for(int c=0;c<64;c++){
            GL11.reset();
            if(kind!=1){
                int icon=kind==2?46:new int[]{154,107,155,14,12}[c%5],damage=c<16?new int[]{0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15}[c]:random.nextInt(32768);
                double x=(random.nextDouble()-.5)*64,y=(random.nextDouble()-.5)*64,z=(random.nextDouble()-.5)*64;
                float yaw=random.nextFloat()*1080-540,pitch=random.nextFloat()*360-180,partial=c%4*.25F;
                i(icon);i(damage);d(x);d(y);d(z);f(yaw);f(pitch);f(partial);
                float scale=c%2==0?.5F:2; if(kind==2)f(scale);
                RenderManager.instance.playerViewY=yaw;RenderManager.instance.playerViewX=pitch;
                EntityPotion entity=new EntityPotion(null);Field field=EntityPotion.class.getDeclaredField("field_40089_d");field.setAccessible(true);field.setInt(entity,damage);
                if(kind==2)new Fireball(scale).doRender(new EntityFireball(null),x,y,z,0,partial);else new Sprite(icon).doRender(entity,x,y,z,0,partial);
            }else {
                int ticks=c<8?new int[]{0,1,100,159,160,161,199,200}[c]:random.nextInt(201);float partial=c%4*.25F;i(ticks);f(partial);dragon.field_40178_aA=ticks;
                renderer.func_40279_a(dragon,partial);
            }
            draws();
        }
        out.close();System.out.println("Original 1.0 "+(kind==0?"RenderSnowball":kind==1?"RenderDragon death rays":"RenderFireball")+": 64 cases");
    }
}
