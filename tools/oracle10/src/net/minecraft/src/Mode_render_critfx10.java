package net.minecraft.src;
import java.io.*;
/** EntityCrit2FX through both 1.0 constructors: code 1 = (world, target) and
 *  codes 2/3 = (world, target, "magicCrit"/"crit"). Same record as Mode_emitters10. */
public final class Mode_render_critfx10 {
 public static void main(String[] a)throws Exception {
  try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[0])))) {
   EntityOracle.i(o,96);
   for(int code=1;code<=3;code++)for(int variant=0;variant<32;variant++) {
    World w=Mode_entities10.world();EntityPig target=new EntityPig(w);target.setPosition(-3.5,64.1,1.5);target.motionX=.25;target.motionY=-.1;target.motionZ=.5;
    OracleHooks.reset(100+variant);EntityOracle.mathRandom().setSeed(200+variant);
    EntityOracle.i(o,code);EntityOracle.l(o,100+variant);EntityOracle.l(o,200+variant);
    EntityFX p=code==1?new EntityCrit2FX(w,target):new EntityCrit2FX(w,target,code==2?"magicCrit":"crit");
    for(int tick=0;tick<10;tick++) {
     Mode_particles2_10.state(p,o);WorldEventOracle.snapshot(w,o);
     target.setPosition(target.posX+.125,target.posY-.03125,target.posZ+.25);
     if(!p.isDead)p.onUpdate();
    }
   }
  }
 }
}
