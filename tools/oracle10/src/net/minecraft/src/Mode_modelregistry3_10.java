package net.minecraft.src;
import java.io.*;import java.util.*;
/** Original constructor registration, including vehicle and sign models. */
public final class Mode_modelregistry3_10 {
 public static void main(String[] a)throws Exception{
  try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))){
   EntityOracle.i(o,23);
   for(int kind=0;kind<23;kind++){
    ModelBase m=kind<20?Mode_modelregistry10.create(kind):kind==20?new ModelMinecart():kind==21?new ModelBoat():new SignModel();
    EntityOracle.i(o,kind);EntityOracle.i(o,Trace10.namedInt("m.boxList.size()", m.boxList.size()));for(Object p:m.boxList)Mode_modelregistry10.part((ModelRenderer)p,o);
    Random r=new Random(42);for(int j=0;j<256;j++){Mode_modelregistry10.part(Harness10.removedRandomModel(m,r),o);EntityOracle.l(o,Trace10.namedLong("EntityOracle.rngState(r)", EntityOracle.rngState(r)));}
   }
  }
 }
}
