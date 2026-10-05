package net.minecraft.src;
import java.io.*;
public final class Mode_emitters181 {
 public static void main(String[] a)throws Exception {
  try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
   EntityOracle.i(o,64);
   for(int kind=0;kind<2;kind++)for(int variant=0;variant<32;variant++) {
    World w=Mode_entities181.world();EntityPig target=new EntityPig(w);target.setPosition(-3.5,64.1,1.5);target.motionX=.25;target.motionY=-.1;target.motionZ=.5;
    OracleHooks.reset(100+variant);EntityOracle.mathRandom().setSeed(200+variant);
    EntityOracle.i(o,kind);EntityOracle.l(o,100+variant);EntityOracle.l(o,200+variant);
    EntityFX p=kind==0?new EntityHugeExplodeFX(w,-3.5,64.1,1.5,0,0,0):new EntityCrit2FX(w,target);
    for(int tick=0;tick<10;tick++) {
     Mode_particles2_181.state(p,o);WorldEventOracle.snapshot(w,o);
     target.setPosition(target.posX+.125,target.posY-.03125,target.posZ+.25);
     if(!p.isDead)p.onUpdate();
    }
   }
  }
 }
}
