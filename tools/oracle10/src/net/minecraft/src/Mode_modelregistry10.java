package net.minecraft.src;
import java.io.*;import java.util.*;
public final class Mode_modelregistry10 {
 static ModelBase create(int kind) {
  switch(kind) {
   case 0:return new ModelBiped();case 1:return new ModelZombie();case 2:return new ModelSkeleton();case 3:return new ModelQuadruped(12,0);
   case 4:return new ModelPig();case 5:return new ModelCow();case 6:return new ModelSheep1();case 7:return new ModelSheep2();case 8:return new ModelChicken();
   case 9:return new ModelSpider();case 10:return new ModelCreeper();case 11:return new ModelSlime(0);case 12:return new ModelSlime(16);case 13:return new ModelSquid();
   case 14:return new ModelWolf();case 15:return new ModelGhast();case 16:return new ModelEnderman();case 17:return new ModelSilverfish();case 18:return new ModelChest();default:return new ModelLargeChest();
  }
 }
 static void part(ModelRenderer p,DataOutputStream o)throws Exception{
  for(float f:new float[]{Trace10.namedFloat("Harness10.modelBound(p,0)", Harness10.modelBound(p,0)),Trace10.namedFloat("Harness10.modelBound(p,1)", Harness10.modelBound(p,1)),Trace10.namedFloat("Harness10.modelBound(p,2)", Harness10.modelBound(p,2)),Trace10.namedFloat("Harness10.modelBound(p,3)", Harness10.modelBound(p,3)),Trace10.namedFloat("Harness10.modelBound(p,4)", Harness10.modelBound(p,4)),Trace10.namedFloat("Harness10.modelBound(p,5)", Harness10.modelBound(p,5)),Trace10.namedFloat("p.rotationPointX", p.rotationPointX),Trace10.namedFloat("p.rotationPointY", p.rotationPointY),Trace10.namedFloat("p.rotationPointZ", p.rotationPointZ),Trace10.namedFloat("p.rotateAngleX", p.rotateAngleX),Trace10.namedFloat("p.rotateAngleY", p.rotateAngleY),Trace10.namedFloat("p.rotateAngleZ", p.rotateAngleZ),Trace10.namedFloat("p.textureWidth", p.textureWidth),Trace10.namedFloat("p.textureHeight", p.textureHeight)})EntityOracle.f(o,f);
 }
 public static void main(String[] a)throws Exception {
  try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
   EntityOracle.i(o,20);
   for(int kind=0;kind<20;kind++) {
    ModelBase m=create(kind);EntityOracle.i(o,kind);EntityOracle.i(o,Trace10.namedInt("m.boxList.size()", m.boxList.size()));for(Object p:m.boxList)part((ModelRenderer)p,o);
    Random r=new Random(42);for(int i=0;i<256;i++) {part(Harness10.removedRandomModel(m,r),o);EntityOracle.l(o,Trace10.namedLong("EntityOracle.rngState(r)", EntityOracle.rngState(r)));}
   }
  }
 }
}
