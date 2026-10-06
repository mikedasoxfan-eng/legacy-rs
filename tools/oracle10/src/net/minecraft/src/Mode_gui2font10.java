package net.minecraft.src;
import java.io.*;
import java.util.*;
import org.lwjgl.opengl.GL11;
public final class Mode_gui2font10 {
    static final String[] TEXT={""," ","   ","Minecraft 1.0.0","one two three four five","one\n\ntwo\n","\n","\n\n","longunbrokentextwithoutspaces","\u00a7aGreen \u00a7rReset","\u00a7kObfuscated \u00a7bblue \u00a7kagain","\u00a7KUPPER \u00a7zbad","1234567890 !?","\u00e9\u00df\u00f1\u00e5","  leading trailing  ","\u00a7a\u00a7kXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX"};
    static class Spy extends FontRenderer {
        ArrayList<String> text=new ArrayList<String>();ArrayList<Integer> y=new ArrayList<Integer>();ArrayList<Boolean> shadow=new ArrayList<Boolean>();
        Spy()throws Exception{super(new GameSettings(),"/font/default.png",(Gui2Fixture10.Engine)Oracle.allocate(Gui2Fixture10.Engine.class));}
        public void renderString(String s,int x,int y,int color,boolean shadow){text.add(s);this.y.add(y);this.shadow.add(shadow);}
    }
    public static void main(String[] a)throws Exception {
        FontRenderer font=Gui2Fixture10.font();int base=Gui2Fixture10.field(FontRenderer.class,"fontDisplayLists").getInt(font);Spy spy=new Spy();
        try(DataOutputStream o=Gui2Fixture10.out(a[0])){
            for(int height:new int[]{8,12,1,19})for(String text:TEXT)for(int width:new int[]{10,11,20,40,80,104,274})for(boolean shadow:new boolean[]{false,true}){
                spy.field_41063_b=height;spy.text.clear();spy.y.clear();spy.shadow.clear();spy.func_40609_a(text,7,13,width,0x123456,shadow);
                Gui2Fixture10.i(o,spy.getStringWidth(text));Gui2Fixture10.i(o,spy.splitStringWidth(text,width));Gui2Fixture10.i(o,spy.text.size());
                for(int n=0;n<spy.text.size();n++){Gui2Fixture10.s(o,spy.text.get(n));Gui2Fixture10.i(o,spy.y.get(n));o.writeBoolean(spy.shadow.get(n));}
            }
            Random r=new Random(10102);
            for(int n=0;n<2000;n++){
                String text=TEXT[n%TEXT.length];long seed=r.nextLong();boolean shadow=r.nextBoolean();font.field_41064_c.setSeed(seed);
                GL11.gui2Lists.clear();font.renderString(text,0,0,-1,shadow);
                Gui2Fixture10.i(o,GL11.gui2Lists.size());for(int id:GL11.gui2Lists)Gui2Fixture10.i(o,id-base);Gui2Fixture10.l(o,font.field_41064_c.nextLong());
            }
        }
    }
}
