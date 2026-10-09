package net.minecraft.src;
import java.io.*;import java.lang.reflect.*;import java.util.Random;
public final class BidiBoundaryProbe11 {
 public static void main(String[] args)throws Exception{
  BidiProbe11.font=new FontRenderer(new GameSettings(),"/font/default.png",new FontDraw11.Textures(),false);
  BidiProbe11.reorder=FontRenderer.class.getDeclaredMethod("func_46121_b",String.class);BidiProbe11.reorder.setAccessible(true);
  char[] small={'A','\u05d0','\u0627','1','\u0661',' ','\n','\t','\u0300','\u202a','\u202b','\u202c','\u202d','\u202e'};
  char[] pool="A01 +-()[]{}\r\n\t\u0085\u2028\u2029\u05d0\u05d1\u0627\u0623\u0661\u0662\u0300\u1bed\u1033\u202a\u202b\u202c\u202d\u202e\u2066\u2067\u2068\u2069\u200e\u200f\ud800\udc00".toCharArray();
  try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(args[0])))){
   o.writeInt(0x42444931);o.writeInt(38416+100000+10000);
   for(int code=0;code<38416;code++){int t=code;char[] s=new char[4];for(int i=0;i<4;i++){s[i]=small[t%14];t/=14;}BidiProbe11.run(o,new String(s));}
   Random r=new Random(9813411);
   for(int n=0;n<100000;n++){char[] s=new char[r.nextInt(256)];for(int i=0;i<s.length;i++)s[i]=r.nextInt(5)==0?(char)r.nextInt(65536):pool[r.nextInt(pool.length)];BidiProbe11.run(o,new String(s));}
   for(int n=0;n<10000;n++){StringBuilder s=new StringBuilder();int depth=r.nextInt(180);for(int i=0;i<depth;i++)s.append("\u202a\u202b\u202d\u202e".charAt(r.nextInt(4)));s.append("A\u05d0\u0661\u0300");for(int i=0;i<depth;i++)s.append(r.nextInt(3)==0?'\n':'\u202c');s.append("\r\n\u05d0\u202cA");BidiProbe11.run(o,s.toString());}
  }
 }
}
