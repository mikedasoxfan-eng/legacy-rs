package net.minecraft.src;
import java.io.*;import java.util.*;
/** Original constructor registration, including vehicle and sign models. */
public final class Mode_modelregistry3_181 {
 public static void main(String[] a)throws Exception{
  try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))){
   EntityOracle.i(o,23);
   for(int kind=0;kind<23;kind++){
    ModelBase m=kind<20?Mode_modelregistry181.create(kind):kind==20?new ModelMinecart():kind==21?new ModelBoat():new SignModel();
    EntityOracle.i(o,kind);EntityOracle.i(o,m.field_35394_j.size());for(Object p:m.field_35394_j)Mode_modelregistry181.part((ModelRenderer)p,o);
    Random r=new Random(42);for(int j=0;j<256;j++){Mode_modelregistry181.part(m.func_35393_a(r),o);EntityOracle.l(o,EntityOracle.rngState(r));}
   }
  }
 }
}
