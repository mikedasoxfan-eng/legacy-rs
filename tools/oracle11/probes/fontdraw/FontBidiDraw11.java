package net.minecraft.src;
import java.io.*;import java.lang.reflect.*;import java.util.*;import org.lwjgl.opengl.GL11;
public final class FontBidiDraw11 {
 static void writeText(DataOutputStream o,String s)throws Exception{o.writeInt(s.length());for(int i=0;i<s.length();i++)o.writeChar(s.charAt(i));}
 public static void main(String[] args)throws Exception{
  Field pages=FontRenderer.class.getDeclaredField("field_44034_f");pages.setAccessible(true);Field pos=FontRenderer.class.getDeclaredField("field_46126_j");pos.setAccessible(true);
  List<String> strings=new ArrayList<String>(Arrays.asList("","ABC","\u05d0\u05d1 (ABC) \u05d2","\u0627\u0661\u0662 ABC","\u00a7kABC \u05d0\u00a7aDEF","\u05d0\ud83d\ude00\u05d0","\ud800\u05d0\udc00","\n\u202d\t\u05d2\u05d1","\u05d0\u05d0 A \u05d0\u05d0","\u202bA\u202c","\u05d0\u00a7zA\u00a7kB\u00a7","\u0627\u1bed\u1033\u0300A"));
  Random r=new Random(813114);char[] pool="ABC01 ()\u00a7akz\u05d0\u05d1\u0627\u0661\u0300\u202a\u202b\u202c\u202d\u202e\n\t\ud800\udc00".toCharArray();
  for(int n=0;n<212;n++){char[] text=new char[r.nextInt(48)];for(int i=0;i<text.length;i++)text[i]=pool[r.nextInt(pool.length)];strings.add(new String(text));}
  try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(args[0])))){
   o.writeInt(0x42464431);o.writeInt(strings.size()*8);
   for(boolean unicode:new boolean[]{false,true})for(boolean bidi:new boolean[]{false,true}){
    FontRenderer f=new FontRenderer(new GameSettings(),"/font/default.png",new FontDraw11.Textures(),unicode);f.func_46123_b(bidi);
    for(String s:strings)for(boolean shadow:new boolean[]{false,true}){
     int x=r.nextBoolean()?Integer.MAX_VALUE:-123,y=r.nextBoolean()?Integer.MIN_VALUE:37,color=r.nextInt();long seed=r.nextLong();
     o.writeBoolean(unicode);o.writeBoolean(bidi);o.writeBoolean(shadow);writeText(o,s);o.writeInt(x);o.writeInt(y);o.writeInt(color);o.writeLong(seed);
     f.field_41064_c.setSeed(seed);GL11.vertices.clear();if(shadow)f.drawStringWithShadow(s,x,y,color);else f.drawString(s,x,y,color);
     int[] cache=(int[])pages.get(f);o.writeInt(GL11.vertices.size());
     for(GL11.Vertex v:GL11.vertices){int texture=256;if(v.texture!=f.fontTextureName){texture=-1;for(int p=0;p<256;p++)if(cache[p]==v.texture){texture=p;break;}if(texture<0)throw new AssertionError(v.texture);}o.writeInt(texture);for(float a:v.values)o.writeFloat(a);}
     for(float c:GL11.color)o.writeFloat(c);o.writeFloat(pos.getFloat(f));o.writeLong(f.field_41064_c.nextLong());
    }
   }
  }
 }
}
