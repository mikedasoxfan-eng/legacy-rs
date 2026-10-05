package net.minecraft.src;
import java.io.*;import java.util.*;
/** Calls original RenderGlobal.spawnParticle; constructors and factory guard/order remain original. */
public final class Mode_particlefactory5_181 {
 public static void main(String[] a)throws Exception {
  String[] names={"bubble","suspended","depthsuspend","townaura","crit","magicCrit","smoke","note","portal","explode","flame","lava","footstep","splash","largesmoke","cloud","reddust","snowballpoof","snowshovel","slime","heart","largeexplode","hugeexplosion","iconcrack_260","iconcrack_+267","iconcrack_351","tilecrack_1","tilecrack_18","tilecrack_35","tilecrack_98","unknown","iconcrack_foo","tilecrack_","tilecrack_300","tilecrack_111","iconcrack_111"};
  try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
   for(String name:names)for(double offset:new double[]{0,16,16.0001,1000,Double.NaN}){
    World w=Mode_entities181.world();EntityPig view=new EntityPig(w);view.setPosition(-3.5+offset,85.25,4.5);RenderManager manager=RendererOracle.manager(w);EffectRenderer effects=new EffectRenderer(w,manager.renderEngine);Map textures=(Map)Oracle.getField(manager.renderEngine,"textureMap");textures.put("/misc/explosion.png",60);textures.put("/misc/footprint.png",61);
    net.minecraft.client.Minecraft mc=(net.minecraft.client.Minecraft)Oracle.allocate(MinecraftImpl.class);mc.renderViewEntity=view;mc.effectRenderer=effects;RenderGlobal global=(RenderGlobal)Oracle.allocate(RenderGlobal.class);Oracle.setField(global,"mc",mc);Oracle.setField(global,"worldObj",w);Oracle.setField(global,"renderEngine",manager.renderEngine);
    OracleHooks.reset(100);EntityOracle.mathRandom().setSeed(200);String error="";
    try {global.spawnParticle(name,-3.5,85.25,4.5,.125,-.1,.2);}catch(RuntimeException e){error=e.getClass().getSimpleName();}
    WorldEventOracle.text(o,error);for(List layer:Mode_effectlifecycle4_181.layers(effects)){EntityOracle.i(o,layer.size());for(Object q:layer)Mode_particles181.state((EntityFX)q,o);}
    EntityOracle.l(o,EntityOracle.rngState(EntityOracle.mathRandom()));EntityOracle.i(o,Integer.parseInt(effects.getStatistics()));
   }
  }
 }
}
