package net.minecraft.src;
import java.io.*;
public final class Mode_bedmodel181 {
 public static void main(String[] a)throws Exception {
  Oracle.DataOut o=new Oracle.DataOut();for(int[] table:new int[][]{ModelBed.field_35871_a,ModelBed.field_35870_b,ModelBed.headInvisibleFace,ModelBed.field_35869_d,ModelBed.footInvisibleFaceRemap,ModelBed.field_35867_f,ModelBed.field_35868_g})for(int v:table)o.i32(v);for(int[] table:ModelBed.bedDirection)for(int v:table)o.i32(v);
  try(OutputStream f=new FileOutputStream(a[1])){o.writeTo(f);}
 }
}
