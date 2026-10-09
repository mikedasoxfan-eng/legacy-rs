package net.minecraft.src;
import java.io.*;
import java.lang.reflect.*;
import java.util.*;
import net.minecraft.client.Minecraft;
import org.lwjgl.opengl.GL11;
import static net.minecraft.src.Mode_render_models10.*;
/** Original GuiIngame boss callback. Font metrics and resource binding are external fixture inputs. */
public final class Mode_render_boss10 {
    static ArrayList<int[]> rects=new ArrayList<int[]>();static ArrayList<String> strings=new ArrayList<String>();static ArrayList<int[]> texts=new ArrayList<int[]>();
    static class Hud extends GuiIngame {
        Hud(Minecraft minecraft){super(minecraft);}
        public void drawTexturedModalRect(int x,int y,int u,int v,int w,int h){rects.add(new int[]{x,y,u,v,w,h});}
    }
    static class Font extends FontRenderer {
        int width;Font(){super(null,null,null,false);}
        public int getStringWidth(String name){if(!name.equals("Boss health"))throw new AssertionError(name);return width;}
        public void drawStringWithShadow(String name,int x,int y,int color){strings.add(name);texts.add(new int[]{x,y,color});}
    }
    static class Engine extends RenderEngine {
        Engine(){super(null,null);}
        public int getTexture(String name){if(!name.equals("/gui/icons.png"))throw new AssertionError(name);return 0;}
    }
    public static void main(String[] args)throws Exception{
        Class.forName("net.minecraft.src.Block");Field field=sun.misc.Unsafe.class.getDeclaredField("theUnsafe");field.setAccessible(true);sun.misc.Unsafe unsafe=(sun.misc.Unsafe)field.get(null);
        Minecraft minecraft=(Minecraft)unsafe.allocateInstance(MinecraftImpl.class);minecraft.gameSettings=(GameSettings)unsafe.allocateInstance(GameSettings.class);
        Font font=(Font)unsafe.allocateInstance(Font.class);minecraft.fontRenderer=font;minecraft.renderEngine=(RenderEngine)unsafe.allocateInstance(Engine.class);
        Hud hud=new Hud(minecraft);Method method=GuiIngame.class.getDeclaredMethod("renderBossHealth");method.setAccessible(true);
        EntityDragon dragon=new EntityDragon(null);
        out=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(args[0])));i(0x30314842);i(1);i(64);
        Random random=new Random(410039);
        for(int c=0;c<64;c++){
            minecraft.displayWidth=320+random.nextInt(3000);minecraft.displayHeight=240+random.nextInt(1200);minecraft.gameSettings.guiScale=c%4;font.width=30+random.nextInt(150);
            int health=c<8?new int[]{0,1,100,199,200,201,-1,Integer.MAX_VALUE}[c]:random.nextInt(241)-20;
            dragon.dataWatcher.updateObject(16,Integer.valueOf(health));RenderDragon.entityDragon=c%7==0?null:dragon;
            i(new ScaledResolution(minecraft.gameSettings,minecraft.displayWidth,minecraft.displayHeight).getScaledWidth());i(font.width);i(health);i(RenderDragon.entityDragon==null?0:1);
            RendererOracle.reset10();rects.clear();strings.clear();texts.clear();method.invoke(hud);method.invoke(hud);
            i(rects.size());for(int[] rectangle:rects)for(int v:rectangle)i(v);
            i(strings.size());for(int n=0;n<strings.size();n++){s(strings.get(n));for(int v:texts.get(n))i(v);}
            i(RenderDragon.entityDragon==null?1:0);
        }
        out.close();System.out.println("Original boss-health callback: 64 cases, one-shot consumption, duplicate background and font requests");
    }
}
