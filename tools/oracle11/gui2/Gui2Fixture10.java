package net.minecraft.src;
import java.io.*;
import java.lang.reflect.*;
import java.awt.image.BufferedImage;
/** Fixture plumbing only. Every behavior under test executes the original jar. */
final class Gui2Fixture10 {
    static void i(DataOutputStream o,int n)throws IOException {o.writeInt(Integer.reverseBytes(n));}
    static void f(DataOutputStream o,float n)throws IOException {i(o,Float.floatToRawIntBits(n));}
    static void l(DataOutputStream o,long n)throws IOException {o.writeLong(Long.reverseBytes(n));}
    static void s(DataOutputStream o,String s)throws IOException {i(o,s.length());for(int n=0;n<s.length();n++)o.writeShort(Short.reverseBytes((short)s.charAt(n)));}
    static Field field(Class<?> c,String name)throws Exception {Field f=c.getDeclaredField(name);f.setAccessible(true);return f;}
    static DataOutputStream out(String path)throws Exception {return new DataOutputStream(new BufferedOutputStream(new FileOutputStream(path)));}
    static class Engine extends RenderEngine {
        Engine(){super(null,null);}
        public int allocateAndSetupTexture(BufferedImage image){return 0;}
        public int getTexture(String name){return 0;}
        public void bindTexture(int id){}
    }
    static FontRenderer font()throws Exception {return new FontRenderer(new GameSettings(),"/font/default.png",(Engine)Oracle.allocate(Engine.class), false);}
    static void buttons(DataOutputStream o,GuiScreen g)throws Exception {
        i(o,g.controlList.size());
        for(Object value:g.controlList){GuiButton b=(GuiButton)value;for(int n:new int[]{b.id,b.xPosition,b.yPosition,b.width,b.height})i(o,n);o.writeBoolean(b.enabled);o.writeBoolean(b.drawButton);s(o,b.displayString);}
    }
}
