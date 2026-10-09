package net.minecraft.src;
import java.io.*;import java.lang.reflect.*;import java.text.Bidi;import java.util.Random;
public final class BidiProbe11 {
 static FontRenderer font;static Method reorder;
 static void text(DataOutputStream o,String s)throws Exception{o.writeInt(s.length());for(int i=0;i<s.length();i++)o.writeChar(s.charAt(i));}
 static void run(DataOutputStream o,String s)throws Exception{
  text(o,s);o.writeBoolean(Bidi.requiresBidi(s.toCharArray(),0,s.length()));Bidi b=new Bidi(s,-2);
  o.writeInt(b.getBaseLevel());for(int i=0;i<s.length();i++)o.writeByte(b.getLevelAt(i));
  o.writeInt(b.getRunCount());for(int i=0;i<b.getRunCount();i++){o.writeInt(b.getRunStart(i));o.writeInt(b.getRunLimit(i));o.writeByte(b.getRunLevel(i));}
  text(o,(String)reorder.invoke(font,s));
 }
 public static void main(String[] args)throws Exception{
  font=new FontRenderer(new GameSettings(),"/font/default.png",new FontDraw11.Textures(),false);
  reorder=FontRenderer.class.getDeclaredMethod("func_46121_b",String.class);reorder.setAccessible(true);
  try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(args[0])))){
   o.writeInt(0x42444931);o.writeInt(85569);
   for(String s:new String[]{"","abc","עברית","العربية","abc אבג 123","אבג (abc) דהו","ا(A)א","א א A א א","a\nא\nb","\u202aא\u202c","\u202bA\u202c","\u202dא\u202c","\u202eA\u202c","א\ud83d\ude00א","\ud800א\udc00","ا١٢٣ ABC","\u2066א\u2069A"})run(o,s);
   for(int c=0;c<65536;c++)run(o,"ا"+(char)c+"A");
   Random random=new Random(41811);char[] pool="Aa01 +-()[]{}§\n\tאבגדاأة١٢٣\u202a\u202b\u202c\u202d\u202e\u2066\u2067\u2068\u2069\u200e\u200f\ud800\udc00".toCharArray();
   for(int n=0;n<20000;n++){char[] s=new char[random.nextInt(96)];for(int i=0;i<s.length;i++)s[i]=random.nextInt(8)==0?(char)random.nextInt(65536):pool[random.nextInt(pool.length)];run(o,new String(s));}
   for(int depth=0;depth<16;depth++){StringBuilder s=new StringBuilder();for(int i=0;i<depth*10;i++)s.append(i%2==0?'\u202b':'\u202a');s.append("Aא1");for(int i=0;i<depth*10;i++)s.append('\u202c');run(o,s.toString());}
  }
  try(OutputStream o=new BufferedOutputStream(new FileOutputStream(args[1]))){Class<?> cls=Class.forName("sun.text.normalizer.UBiDiProps");Object data=cls.getMethod("getSingleton").invoke(null);Method get=cls.getMethod("getClass",int.class);for(int c=0;c<=0x10ffff;c++)o.write((Integer)get.invoke(data,c));}
 }
}
