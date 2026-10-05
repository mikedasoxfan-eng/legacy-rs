package net.minecraft.src;
import java.io.*;
import java.awt.image.BufferedImage;
import java.util.Random;

/** Original scale rounding, button hit boundaries and font measurement/wrapping. */
public final class Mode_guiutility181 {
    static class Textures extends RenderEngine {
        Textures(){super(null,new GameSettings());}
        public int allocateAndSetupTexture(BufferedImage image){return 0;}
    }
    static final String[] TEXT={""," ","   ","Minecraft Beta 1.8.1","one two three four five","one\n\ntwo\n","\n","\n\n","longunbrokentextwithoutspaces","\u00a7aGreen \u00a7rReset","\u00a7","\u00a7zbad","1234567890 !?", "\u00e9\u00df\u00f1\u00e5", "  leading trailing  "};
    public static void main(String[] args)throws Exception {
        try(DataOutputStream o=new DataOutputStream(new FileOutputStream(args[1]))) {
            Random r=new Random(181);GameSettings settings=new GameSettings();
            for(int c=0;c<10000;c++) {
                settings.guiScale=r.nextInt(7)-1;int w=r.nextInt(8193),h=r.nextInt(4097);
                ScaledResolution s=new ScaledResolution(settings,w,h);
                EntityOracle.i(o,s.getScaledWidth());EntityOracle.i(o,s.getScaledHeight());EntityOracle.i(o,s.scaleFactor);EntityOracle.d(o,s.scaledWidthD);EntityOracle.d(o,s.scaledHeightD);
            }
            for(int c=0;c<10000;c++) {
                int x=r.nextInt(1000)-500,y=r.nextInt(1000)-500,w=r.nextInt(250),h=r.nextInt(50),mx=x+r.nextInt(260)-5,my=y+r.nextInt(60)-5;
                GuiButton b=new GuiButton(c,x,y,w,h,"label");b.enabled=r.nextBoolean();b.enabled2=r.nextBoolean();
                o.writeBoolean(b.mousePressed(null,mx,my));EntityOracle.i(o,b.getHoverState(mx>=x&&my>=y&&mx<x+w&&my<y+h));
            }
            FontRenderer font=new FontRenderer(new GameSettings(),"/font/default.png",new Textures());
            for(String text:TEXT){EntityOracle.i(o,font.getStringWidth(text));for(int w:new int[]{10,11,20,40,80,120,200})EntityOracle.i(o,font.splitStringWidth(text,w));}
            for(int c=0;c<256;c++)EntityOracle.i(o,font.getStringWidth(String.valueOf((char)c)));
        }
    }
}
