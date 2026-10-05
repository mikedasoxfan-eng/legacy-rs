package net.minecraft.src;
import java.io.*;import java.util.*;
public final class Mode_modelregistry181 {
 static ModelBase create(int kind) {
  switch(kind) {
   case 0:return new ModelBiped();case 1:return new ModelZombie();case 2:return new ModelSkeleton();case 3:return new ModelQuadruped(12,0);
   case 4:return new ModelPig();case 5:return new ModelCow();case 6:return new ModelSheep1();case 7:return new ModelSheep2();case 8:return new ModelChicken();
   case 9:return new ModelSpider();case 10:return new ModelCreeper();case 11:return new ModelSlime(0);case 12:return new ModelSlime(16);case 13:return new ModelSquid();
   case 14:return new ModelWolf();case 15:return new ModelGhast();case 16:return new ModelEnderman();case 17:return new ModelSilverfish();case 18:return new ModelChest();default:return new ModelLargeChest();
  }
 }
 static void part(ModelRenderer p,DataOutputStream o)throws Exception{
  for(float f:new float[]{p.field_35977_i,p.field_35975_j,p.field_35976_k,p.field_35973_l,p.field_35974_m,p.field_35972_n,p.rotationPointX,p.rotationPointY,p.rotationPointZ,p.rotateAngleX,p.rotateAngleY,p.rotateAngleZ,p.field_35971_a,p.field_35970_b})EntityOracle.f(o,f);
 }
 public static void main(String[] a)throws Exception {
  try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
   EntityOracle.i(o,20);
   for(int kind=0;kind<20;kind++) {
    ModelBase m=create(kind);EntityOracle.i(o,kind);EntityOracle.i(o,m.field_35394_j.size());for(Object p:m.field_35394_j)part((ModelRenderer)p,o);
    Random r=new Random(42);for(int i=0;i<256;i++) {part(m.func_35393_a(r),o);EntityOracle.l(o,EntityOracle.rngState(r));}
   }
  }
 }
}
