package net.minecraft.src;
import java.io.*;import java.util.*;
/** Original EffectRenderer layer caps, same-tick pruning and clear/reuse. */
public final class Mode_effectlifecycle4_10 {
 static List[] layers(EffectRenderer r)throws Exception{return (List[])Oracle.getField(r,"fxLayers");}
 static void snapshot(EffectRenderer r,DataOutputStream o)throws Exception{
  for(List list:layers(r)){EntityOracle.i(o,Trace10.namedInt("list.size()", list.size()));for(Object q:list){EntityFX p=(EntityFX)q;EntityOracle.d(o,Trace10.namedDouble("p.posX", p.posX));EntityOracle.i(o,Trace10.namedInt("p.particleAge", p.particleAge));Trace10.writeBoolean(o, "p.isDead", p.isDead);EntityOracle.l(o,Trace10.namedLong("EntityOracle.rngState(p.rand)", EntityOracle.rngState(p.rand)));}}
  EntityOracle.i(o,Trace10.namedInt("Integer.parseInt(r.getStatistics())", Integer.parseInt(r.getStatistics())));EntityOracle.l(o,Trace10.namedLong("EntityOracle.rngState(EntityOracle.mathRandom())", EntityOracle.rngState(EntityOracle.mathRandom())));
 }
 public static void main(String[] a)throws Exception{
  World w=Mode_entities10.world();EffectRenderer r=new EffectRenderer(w,null);OracleHooks.reset(100);EntityOracle.mathRandom().setSeed(200);
  try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))){
   for(int cycle=0;cycle<3;cycle++){
    for(int layer=0;layer<4;layer++)for(int j=0;j<4003;j++){
     EntityFX p=layer==0?new EntityFX(w,0,85,0,0,0,0):layer==1?new EntityDiggingFX(w,0,85,0,0,0,0,Block.stone,j%6,0):layer==2?new EntityBreakingFX(w,0,85,0,Item.snowball):new EntityFootStepFX(null,w,0,85,0);
     p.posX=j*.001;p.particleMaxAge=j%4;p.isDead=j%7==0;r.addEffect(p);
    }
    snapshot(r,o);for(int tick=0;tick<5;tick++){r.updateEffects();snapshot(r,o);}
    r.clearEffects(cycle==1?null:w);snapshot(r,o);
   }
  }
 }
}
