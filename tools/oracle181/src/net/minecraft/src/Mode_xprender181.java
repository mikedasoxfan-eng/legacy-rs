package net.minecraft.src;
import java.io.*;import java.util.*;
public final class Mode_xprender181 {
 public static void main(String[] args)throws Exception {
  World w=Mode_entities181.world();RenderManager m=RendererOracle.manager(w);
  java.lang.reflect.Field field=RenderEngine.class.getDeclaredField("textureMap");field.setAccessible(true);Map textures=(Map)field.get(m.renderEngine);textures.put("/item/xporb.png",30);
  RendererOracle.field("convertQuadsToTriangles").setBoolean(null,false);
  Oracle.DataOut out=new Oracle.DataOut();out.i32(0x33373152);out.i32(1);out.i32(11*8*4);
  for(int value:new int[]{1,3,7,17,37,73,149,307,617,1237,2477})for(int age:new int[]{0,1,7,25,100,6000,-1,Integer.MAX_VALUE})for(float partial:new float[]{0,.25F,.75F,1}) {
   EntityXPOrb e=new EntityXPOrb(w,0,0,0,value);e.field_35127_a=age;
   RendererOracle.writeCase(out,8,e,new RenderXPOrb(),m,value,1,0,age,0,0,0,partial);
  }
  try(OutputStream f=new FileOutputStream(args[1])){out.writeTo(f);}
 }
}
