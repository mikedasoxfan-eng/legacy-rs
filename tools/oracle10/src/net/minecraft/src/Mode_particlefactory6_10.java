package net.minecraft.src;
import java.io.*;import java.util.*;
/** Calls original RenderGlobal.spawnParticle; constructors and factory guard/order remain original. */
public final class Mode_particlefactory6_10 {
 public static void main(String[] a)throws Exception {
  ArrayList<String> list=new ArrayList<String>();
  for(int c=0;c<=65535;c++)if(Character.digit((char)c,10)==0){
   String digits=""+(char)(c+2)+(char)(c+6)+(char)c;
   list.add("iconcrack_"+digits);list.add("iconcrack_+"+digits);list.add("iconcrack_-"+digits);
   list.add("tilecrack_"+(char)(c+3)+(char)(c+5));
  }
  Collections.addAll(list,"iconcrack_2147483647","iconcrack_2147483648","iconcrack_-2147483648","iconcrack_-2147483649","iconcrack_","iconcrack_+","iconcrack_\u00b2","iconcrack_\uff12\u0666\u0030","iconcrack_\ud835\udfd0\ud835\udfd4\ud835\udfce","iconcrack_ 260","iconcrack_260 ","tilecrack_\u22121","tilecrack_00035");
  String[] names=list.toArray(new String[0]);
  try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
   EntityOracle.i(o,0x18160007);EntityOracle.i(o,Trace10.namedInt("names.length", names.length));for(String name:names)WorldEventOracle.text(o,name);
   for(String name:names)for(double offset:new double[]{0,16,16.0001,1000,Double.NaN}){
    World w=Mode_entities10.world();EntityPig view=new EntityPig(w);view.setPosition(-3.5+offset,85.25,4.5);RenderManager manager=RendererOracle.manager(w);EffectRenderer effects=new EffectRenderer(w,manager.renderEngine);Map textures=(Map)Oracle.getField(manager.renderEngine,"textureMap");textures.put("/misc/explosion.png",60);textures.put("/misc/footprint.png",61);
    net.minecraft.client.Minecraft mc=(net.minecraft.client.Minecraft)Oracle.allocate(MinecraftImpl.class);mc.gameSettings=(GameSettings)Oracle.allocate(GameSettings.class);mc.renderViewEntity=view;mc.effectRenderer=effects;RenderGlobal global=(RenderGlobal)Oracle.allocate(RenderGlobal.class);Oracle.setField(global,"mc",mc);Oracle.setField(global,"worldObj",w);Oracle.setField(global,"renderEngine",manager.renderEngine);
    OracleHooks.reset(100);EntityOracle.mathRandom().setSeed(200);String error="";
    try {global.spawnParticle(name,-3.5,85.25,4.5,.125,-.1,.2);}catch(RuntimeException e){error=e.getClass().getSimpleName();}
    WorldEventOracle.text(o,error);for(List layer:Mode_effectlifecycle4_10.layers(effects)){EntityOracle.i(o,Trace10.namedInt("layer.size()", layer.size()));for(Object q:layer)Mode_particles10.state((EntityFX)q,o);}
    EntityOracle.l(o,Trace10.namedLong("EntityOracle.rngState(EntityOracle.mathRandom())", EntityOracle.rngState(EntityOracle.mathRandom())));EntityOracle.i(o,Trace10.namedInt("Integer.parseInt(effects.getStatistics())", Integer.parseInt(effects.getStatistics())));
   }
  }
 }
}
