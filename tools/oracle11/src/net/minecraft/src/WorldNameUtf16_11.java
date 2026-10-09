package net.minecraft.src;
import java.io.*;import java.util.*;
/** WorldInfo's actual LevelName tag and original NBT modified-UTF8 save/read. */
public final class WorldNameUtf16_11 {
 static String hex(byte[] a){StringBuilder s=new StringBuilder();for(byte b:a)s.append(String.format("%02x",b&255));return s.toString();}
 static String units(String a){StringBuilder s=new StringBuilder();for(int i=0;i<a.length();i++)s.append(String.format("%04x",(int)a.charAt(i)));return s.length()==0?"-":s.toString();}
 static void emit(PrintWriter out,char[] units)throws Exception{
  String name=new String(units);WorldInfo info=new WorldInfo(new WorldSettings(42L,0,true,false,EnumWorldType.DEFAULT),name);
  NBTBase tag=info.getNBTTagCompound().getTag("LevelName");ByteArrayOutputStream bytes=new ByteArrayOutputStream();NBTBase.writeTag(tag,new DataOutputStream(bytes));
  NBTTagString read=(NBTTagString)NBTBase.readTag(new DataInputStream(new ByteArrayInputStream(bytes.toByteArray())));
  NBTTagCompound data=new NBTTagCompound();data.setString("LevelName",read.stringValue);WorldInfo reopened=new WorldInfo(data);
  if(!reopened.getWorldName().equals(name))throw new AssertionError("WorldInfo changed units");out.println(units(name)+" "+hex(bytes.toByteArray())+" "+units(reopened.getWorldName()));
 }
 public static void main(String[]args)throws Exception{
  PrintWriter out=new PrintWriter(new File(args[0],"worldname_utf16_11.txt"));
  for(int u=0;u<65536;u++)emit(out,new char[]{(char)u});
  for(char[] u:new char[][]{{},{'a',0,'b'},{0xd800,0xdc00},{0xdbff,0xdfff},{0xd800,0xd800},{0xdc00,0xdc00},{0xdc00,0xd800},{'x',0xd800,'y'},{0xd800,'x',0xdc00},{0xd800,0xdc00,0xd800},{0xdc00,0xd800,0xdc00},{0xdfff,0xdbff,0xdfff},{0xfffd},{0xfffe,0xffff}})emit(out,u);
  out.close();
 }
}
