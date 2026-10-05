package net.minecraft.src;
import java.io.*;import java.util.*;
public final class Mode_options181 {
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
    if(op==0){s.keyBindings[index].keyCode=code;KeyBinding.func_35961_b();}
    else if(op==1)KeyBinding.func_35960_a(code);
    else if(op==2)KeyBinding.func_35963_a(code,down);
    else if(op==3)o.println(s.keyBindings[index].func_35962_c());
    else KeyBinding.func_35959_a();
    for(KeyBinding k:s.keyBindings)o.print(k.keyCode+","+k.field_35965_e+","+k.field_35964_f+";");o.println();
   }
  }
 }
}
