package net.minecraft.src;
import java.io.*;
import java.util.*;
import java.lang.reflect.*;
import java.nio.ByteBuffer;
import net.minecraft.client.Minecraft;
import org.lwjgl.input.Mouse;
/** Original stats list constructors, row callbacks, header geometry and full sorting cycles. */
public final class Mode_stats8_10 {
    static class Writer extends StatFileWriter {
        int scenario;
        Writer(){super(null,null);}
        public int writeStat(StatBase s){switch(scenario%8){case 0:return Integer.MIN_VALUE;case 1:return Integer.MAX_VALUE;case 2:return -1;case 3:return 0;case 4:return 1;default:return ((s.statId^scenario)*1664525+1013904223)%999-200;}}
    }
    static class Sounds extends SoundManager {int clicks;public void playSoundFX(String s,float v,float p){if(!s.equals("random.click")||v!=1||p!=1)throw new AssertionError();clicks++;}}
    static class Stats extends GuiStats {
        DataOutputStream out,tooltip;
        Stats(Writer writer){super(null,writer);}
        public void drawString(FontRenderer font,String text,int x,int y,int color){try{WorldEventOracle.text(out,text);for(int n:new int[]{Trace10.namedInt("x", x),Trace10.namedInt("y", y),Trace10.namedInt("color", color)})EntityOracle.i(out,n);}catch(Exception e){throw new RuntimeException(e);}}
        protected void drawGradientRect(int x,int y,int x1,int y1,int color,int color1){try{for(int n:new int[]{Trace10.namedInt("x", x),Trace10.namedInt("y", y),Trace10.namedInt("x1", x1),Trace10.namedInt("y1", y1),Trace10.namedInt("color", color),Trace10.namedInt("color1", color1)})EntityOracle.i(tooltip,n);}catch(Exception e){throw new RuntimeException(e);}}
    }
    static class Font extends FontRenderer {
        DataOutputStream out;
        Font(){super(new GameSettings(),"/font/default.png",new Mode_guiutility10.Textures());}
        public void drawStringWithShadow(String text,int x,int y,int color){try{WorldEventOracle.text(out,text);for(int n:new int[]{Trace10.namedInt("x", x),Trace10.namedInt("y", y),Trace10.namedInt("color", color)})EntityOracle.i(out,n);}catch(Exception e){throw new RuntimeException(e);}}
    }
    static class Items extends RenderItem {
        DataOutputStream out;
        public void drawItemIntoGui(FontRenderer f,RenderEngine e,int id,int damage,int icon,int x,int y){try{for(int n:new int[]{Trace10.namedInt("id", id),Trace10.namedInt("x", x),Trace10.namedInt("y", y)})EntityOracle.i(out,n);}catch(Exception t){throw new RuntimeException(t);}}
    }
    public static void main(String[] a)throws Exception {
        Mode_guislot8_10.field(Mouse.class,"created").setBoolean(null,true);Mode_guislot8_10.field(Mouse.class,"buttonCount").setInt(null,1);ByteBuffer buttons=ByteBuffer.allocate(1);Mode_guislot8_10.field(Mouse.class,"buttons").set(null,buttons);
        Writer writer=(Writer)Oracle.allocate(Writer.class);Stats g=new Stats(writer);g.mc=(PlayerOracle.HeadlessMinecraft)Oracle.allocate(PlayerOracle.HeadlessMinecraft.class);g.width=427;g.height=240;
        g.mc.renderEngine=(Mode_guislot8_10.Textures)Oracle.allocate(Mode_guislot8_10.Textures.class);Font font=new Font();g.fontRenderer=font;
        Sounds sounds=(Sounds)Oracle.allocate(Sounds.class);g.mc.sndManager=sounds;
        Items items=new Items();Field renderItem=GuiStats.class.getDeclaredField("renderItem");renderItem.setAccessible(true);renderItem.set(null,items);
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            g.out=o;items.out=o;
            for(int c=0;c<128;c++) {
                writer.scenario=c;
                for(GuiSlotStats list:new GuiSlotStats[]{new GuiSlotStatsBlock(g),new GuiSlotStatsItem(g)}) {
                    EntityOracle.i(o,Trace10.namedInt("list.getSize()", list.getSize()));EntityOracle.i(o,Trace10.namedInt("list.getContentHeight()", list.getContentHeight()));Trace10.writeBoolean(o, "list.isSelected(-1)", list.isSelected(-1));
                    for(Object entry:list.field_27273_c){StatCrafting s=(StatCrafting)entry;EntityOracle.i(o,Trace10.namedInt("s.statId", s.statId));EntityOracle.i(o,Trace10.namedInt("s.func_25072_b()", s.func_25072_b()));}
                    for(int column=-1;column<5;column++)WorldEventOracle.text(o,list.func_27263_a(column));
                    for(int i=0;i<list.getSize();i++)list.drawSlot(i,105+c,60+i*20-c,16,Tessellator.instance);
                    Oracle.weatherVertices.clear();Oracle.weatherLights.clear();
                    for(int x:new int[]{78,79,114,115,128,129,164,165,178,179,214,215,79,79,79,129,129,129,179,179,179}) {
                        list.func_27255_a(x,0);for(int n:new int[]{Trace10.namedInt("list.field_27268_b", list.field_27268_b),Trace10.namedInt("list.field_27271_e", list.field_27271_e),Trace10.namedInt("list.field_27270_f", list.field_27270_f)})EntityOracle.i(o,n);
                        EntityOracle.i(o,Trace10.namedInt("sounds.clicks", sounds.clicks));sounds.clicks=0;
                        for(Object entry:list.field_27273_c){StatCrafting s=(StatCrafting)entry;EntityOracle.i(o,Trace10.namedInt("s.statId", s.statId));EntityOracle.i(o,Trace10.namedInt("s.func_25072_b()", s.func_25072_b()));}
                        buttons.put(0,(byte)(c%2==0?1:0));list.func_27260_a(105,36,Tessellator.instance);EntityOracle.i(o,Trace10.namedInt("list.field_27268_b", list.field_27268_b));
                        EntityOracle.i(o,Trace10.namedInt("Oracle.weatherVertices.size()", Oracle.weatherVertices.size()));for(float[] v:Oracle.weatherVertices)for(int index=1;index<10;index++)EntityOracle.f(o,Trace10.namedFloat("v[index]", v[index]));Oracle.weatherVertices.clear();Oracle.weatherLights.clear();
                    }
                    for(int my:new int[]{31,32,36,51,52,56,60,75,175,176,177})for(int mx:new int[]{104,105,144,145,165,166,201,202,220,221,251,252,270,271,301,302,320,321,324,325,326}) {
                        ByteArrayOutputStream calls=new ByteArrayOutputStream();g.tooltip=new DataOutputStream(calls);font.out=g.tooltip;
                        list.func_27257_b(mx,my);EntityOracle.i(o,Trace10.namedInt("calls.size()", calls.size()));Trace10.writeBytes(o, "calls.toByteArray()", calls.toByteArray());
                    }
                }
            }
        }
    }
}
