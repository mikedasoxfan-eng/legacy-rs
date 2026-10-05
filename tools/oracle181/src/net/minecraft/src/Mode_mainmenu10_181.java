package net.minecraft.src;
import java.io.*;
import java.util.*;
import java.lang.reflect.*;
import java.awt.image.BufferedImage;
import net.minecraft.client.Minecraft;
/** Original constructor RNG, complete singleplayer buttons and update/key/action callbacks. */
public final class Mode_mainmenu10_181 {
    static class Engine extends RenderEngine {
        Engine(){super(null,null);}
        public int allocateAndSetupTexture(BufferedImage image) {
            if(image.getWidth()!=256 || image.getHeight()!=256 || image.getType()!=2)throw new AssertionError();
            return 42;
        }
    }
    static class Client extends Minecraft {
        int displays,quits;
        Client(){super(null,null,null,1,1,false);}
        public void displayUnexpectedThrowable(UnexpectedThrowable t){throw new AssertionError();}
        public void displayGuiScreen(GuiScreen s){displays++;}
        public void shutdown(){quits++;}
    }
    static Field field(String name)throws Exception {Field f=GuiMainMenu.class.getDeclaredField(name);f.setAccessible(true);return f;}
    public static void main(String[] args)throws Exception {
        Random random=(Random)field("rand").get(null);random.setSeed(18110);
        Client mc=(Client)Oracle.allocate(Client.class);mc.renderEngine=(Engine)Oracle.allocate(Engine.class);mc.gameSettings=new GameSettings();
        Random r=new Random(18110);
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(args[1])))) {
            for(int c=0;c<1000;c++)Mode_worldcallbacks181.str(o,(String)field("splashText").get(new GuiMainMenu()));
            o.writeLong(random.nextLong());
            for(int c=0;c<10000;c++) {
                GuiMainMenu g=new GuiMainMenu();g.mc=mc;g.width=r.nextInt(1800);g.height=r.nextInt(1200);mc.hideQuitButton=r.nextBoolean();mc.session=null;
                field("splashText").set(g,"Ported to Rust!");g.initGui();Mode_yesno8_181.buttons(o,g);
                float counter=c%4==0?16777216F:c%4==1?-16777216F:r.nextInt();int ticks=r.nextInt();
                field("updateCounter").setFloat(g,counter);field("field_35357_f").setInt(g,ticks);
                for(int t=0;t<4;t++){g.updateScreen();EntityOracle.f(o,field("updateCounter").getFloat(g));EntityOracle.i(o,field("field_35357_f").getInt(g));}
                mc.displays=0;mc.quits=0;g.keyTyped((char)r.nextInt(65536),r.nextInt());EntityOracle.i(o,mc.displays);EntityOracle.i(o,mc.quits);o.writeBoolean(g.doesGuiPauseGame());
                for(int id:new int[]{-1,0,1,3,4,5}) {mc.displays=0;mc.quits=0;g.actionPerformed(new GuiButton(id,0,0,""));EntityOracle.i(o,mc.displays);EntityOracle.i(o,mc.quits);}
            }
        }
    }
}
