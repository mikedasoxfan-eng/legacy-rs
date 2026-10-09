package net.minecraft.src;
import java.io.*;import java.lang.reflect.*;import java.awt.image.BufferedImage;import org.lwjgl.opengl.GL11;
public final class FontDraw11 {
 static class Textures extends RenderEngine {
  int next=1000; Textures(){super(null,new GameSettings());}
  public int allocateAndSetupTexture(BufferedImage image){int id=next++;GL11.glBindTexture(3553,id);return id;}
 }
 static Method render; static Field pages,pos;
 static void run(DataOutputStream o,FontRenderer f,String s,int x,int y,int color,boolean shadow,long seed)throws Exception{
  f.field_41064_c.setSeed(seed);GL11.vertices.clear();render.invoke(f,s,x,y,color,shadow);
  int[] cache=(int[])pages.get(f);o.writeInt(GL11.vertices.size());
  for(GL11.Vertex v:GL11.vertices){int texture=256;if(v.texture!=f.fontTextureName){texture=-1;for(int p=0;p<256;p++)if(cache[p]==v.texture){texture=p;break;}if(texture<0)throw new AssertionError(v.texture);}o.writeInt(texture);for(float a:v.values)o.writeFloat(a);}
  for(float c:GL11.color)o.writeFloat(c);o.writeFloat(pos.getFloat(f));o.writeLong(f.field_41064_c.nextLong());
 }
 public static void main(String[] args)throws Exception{
  render=FontRenderer.class.getDeclaredMethod("renderString",String.class,int.class,int.class,int.class,boolean.class);render.setAccessible(true);
  pages=FontRenderer.class.getDeclaredField("field_44034_f");pages.setAccessible(true);pos=FontRenderer.class.getDeclaredField("field_46126_j");pos.setAccessible(true);
  try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(args[0])))){
   for(boolean unicode:new boolean[]{false,true}){
    FontRenderer f=new FontRenderer(new GameSettings(),"/font/default.png",new Textures(),unicode);
    for(int c=0;c<65536;c++)run(o,f,String.valueOf((char)c),-123,37,0x02abcdef,false,c);
    for(String s:new String[]{""," ","ABC xyz","\u00a7kHELLO WORLD!","\u00a7aA\u00a7kBC\u00a7zD\u00a7","\u00a7KK\u00a7rE","\u0627\u05d0\u65e5\ud83d\ude00","\u0000\u00a7\u0000"})
     for(int color:new int[]{0,0x01abcdef,0x02abcdef,0x03abcdef,0x04abcdef,0x7f102030,0xffabcdef})
      for(boolean shadow:new boolean[]{false,true})run(o,f,s,Integer.MAX_VALUE, Integer.MIN_VALUE,color,shadow,4181);
   }
  }
 }
}
