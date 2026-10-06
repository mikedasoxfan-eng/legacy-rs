package net.minecraft.src;
import java.io.*;
public final class Mode_bedmodel10 {
 public static void main(String[] a)throws Exception {
  Oracle.DataOut o=new Oracle.DataOut();for(int[] table:new int[][]{Direction.field_35871_a,Direction.field_35870_b,Direction.headInvisibleFace,Direction.field_35869_d,Direction.footInvisibleFaceRemap,Direction.field_35867_f,Direction.field_35868_g})for(int v:table)o.i32(v);for(int[] table:Direction.bedDirection)for(int v:table)o.i32(v);
  try(OutputStream f=new FileOutputStream(a[1])){o.writeTo(f);}
 }
}
