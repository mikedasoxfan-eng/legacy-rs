package net.minecraft.src;
import java.io.*;import java.util.*;
/** Original charAt-domain acceptance and supplementary boundaries; no clipboard API read. */
public final class UnicodeInput11 {
 static void text(DataOutputStream o,String s)throws Exception{o.writeInt(s.length());for(int i=0;i<s.length();i++)o.writeChar(s.charAt(i));}
 static String repeat(int n){char[] c=new char[n];Arrays.fill(c,'A');return new String(c);}
 public static void main(String[] args)throws Exception{
  try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(args[0])))){
   o.writeInt(0x55494e31);o.writeInt(65536);
   for(int unit=0;unit<65536;unit++){GuiChat c=new GuiChat();c.keyTyped((char)unit,0);text(o,c.message);GuiTextField f=new GuiTextField(new GuiScreen(),null,0,0,200,20,"");f.isFocused=true;f.setMaxStringLength(32);if(unit==22)o.writeInt(-1);else{f.textboxKeyTyped((char)unit,0);text(o,f.getText());}}
   o.writeInt(96);
   for(int initial:new int[]{0,99,100})for(int unit:new int[]{0,32,127,167,256,0xD83D,0xDE00,0xFFFF})for(int key:new int[]{0,14})for(boolean field:new boolean[]{false,true}){
    o.writeBoolean(field);o.writeInt(initial);o.writeInt(unit);o.writeInt(key);GuiChat c=new GuiChat();c.message=repeat(initial);GuiTextField f=new GuiTextField(new GuiScreen(),null,0,0,200,20,repeat(initial));f.isFocused=true;f.setMaxStringLength(100);if(field){f.textboxKeyTyped((char)unit,key);text(o,f.getText());}else{c.keyTyped((char)unit,key);text(o,c.message);}
   }
   int[][] sequence={{0xD83D,0},{0xDE00,0},{0,14},{0,14},{0xD83D,0},{0xDE00,0},{167,0},{0x100,0},{0,14}};
   o.writeInt(18);for(boolean field:new boolean[]{false,true}){GuiChat c=new GuiChat();GuiTextField f=new GuiTextField(new GuiScreen(),null,0,0,200,20,"");f.isFocused=true;f.setMaxStringLength(100);for(int[] event:sequence){o.writeBoolean(field);o.writeInt(event[0]);o.writeInt(event[1]);if(field){f.textboxKeyTyped((char)event[0],event[1]);text(o,f.getText());}else{c.keyTyped((char)event[0],event[1]);text(o,c.message);}}}
  }
 }
}
