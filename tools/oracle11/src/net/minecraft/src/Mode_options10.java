package net.minecraft.src;
import java.io.*;import java.util.*;
public final class Mode_options10 {
 static class Sound extends SoundManager {public void onSoundOptionsChanged(){}}
 public static void main(String[] a)throws Exception {
  GameSettings s=new GameSettings();s.mc=(net.minecraft.client.Minecraft)Oracle.allocate(PlayerOracle.HeadlessMinecraft.class);s.mc.sndManager=new Sound();
  try(PrintWriter o=new PrintWriter(new OutputStreamWriter(new FileOutputStream(a[1]),"UTF-8"))) {
   for(EnumOptions e:EnumOptions.values()) o.println(e.ordinal()+" "+e.getEnumString()+" "+e.getEnumFloat()+" "+e.getEnumBoolean()+" "+s.getKeyBinding(e));
   for(EnumOptions e:new EnumOptions[]{EnumOptions.FOV,EnumOptions.GAMMA,EnumOptions.SENSITIVITY,EnumOptions.MUSIC,EnumOptions.SOUND})for(float f:new float[]{-1,0,0.01F,0.333F,0.5F,0.99F,1,2}){s.setOptionFloatValue(e,f);o.println(s.getKeyBinding(e));}
   for(KeyBinding k:s.keyBindings)o.println(k.keyDescription+" "+k.keyCode);
   Random r=new Random(181);
   for(int i=0;i<10000;i++) {
    int op=r.nextInt(5),index=r.nextInt(s.keyBindings.length),code=r.nextInt(220)-105;boolean down=r.nextBoolean();
    if(op==0){s.keyBindings[index].keyCode=code;KeyBinding.resetKeyBindingArrayAndHash();}
    else if(op==1)KeyBinding.onTick(code);
    else if(op==2)KeyBinding.setKeyBindState(code,down);
    else if(op==3)o.println(s.keyBindings[index].isPressed());
    else KeyBinding.unPressAllKeys();
    for(KeyBinding k:s.keyBindings)o.print(k.keyCode+","+k.pressed+","+k.pressTime+";");o.println();
   }
   Block.stone.getClass();Item.shovelSteel.getClass();
   for(Object object:StatList.generalStats){StatBase stat=(StatBase)object;o.println("general\t"+stat.statId+"\t"+StatsOracle.type(stat)+"\t"+((String)R11Access.get(stat,"statName")));}
   for(Object object:StatList.objectMineStats){StatCrafting stat=(StatCrafting)object;o.println("block\t"+stat.statId+"\t"+stat.func_25072_b());}
   for(Object object:StatList.itemStats){StatCrafting stat=(StatCrafting)object;o.println("item\t"+stat.statId+"\t"+stat.func_25072_b());}
   String[] names={"mine","craft","use","break"};StatBase[][] arrays={StatList.mineBlockStatArray,StatList.objectCraftStats,StatList.objectUseStats,StatList.objectBreakStats};
   for(int table=0;table<4;table++)for(int index=0;index<arrays[table].length;index++)if(arrays[table][index]!=null)o.println(names[table]+"\t"+index+"\t"+arrays[table][index].statId);
   for(Object object:AchievementList.achievementList){Achievement achievement=(Achievement)object;o.println("achievement\t"+achievement.statId+"\t"+achievement.displayColumn+"\t"+achievement.displayRow+"\t"+achievement.theItemStack.itemID+"\t"+(achievement.parentAchievement==null?-1:achievement.parentAchievement.statId)+"\t"+achievement.getSpecial()+"\t"+((String)R11Access.get(achievement,"statName"))+"\t"+achievement.getDescription());}
   o.println("bounds\t"+AchievementList.minDisplayColumn+"\t"+AchievementList.minDisplayRow+"\t"+AchievementList.maxDisplayColumn+"\t"+AchievementList.maxDisplayRow);
   IStatType[] types={new StatTypeSimple(),new StatTypeTime(),new StatTypeDistance()};
   for(IStatType type:types)for(int v:new int[]{Integer.MIN_VALUE,-1,0,1,9,10,49,50,51,599,600,601,35999,36000,36001,863999,864000,864001,50000,50001,315359999,315360000,315360001,Integer.MAX_VALUE})o.println("format\t"+v+"\t"+type.format(v));
   r=new Random(181);for(int i=0;i<5000;i++){int v=r.nextInt();for(IStatType type:types)o.println("format\t"+v+"\t"+type.format(v));}
  }
 }
}
