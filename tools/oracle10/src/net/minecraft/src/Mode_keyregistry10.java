package net.minecraft.src;
import java.io.*;
import java.util.*;
/** Global KeyBinding registry, including stale lookup until reset and retained old settings. */
public final class Mode_keyregistry10 {
    public static void main(String[] args)throws Exception {
        ArrayList<KeyBinding> keys=new ArrayList<KeyBinding>();
        for(int g=0;g<4;g++)Collections.addAll(keys,new GameSettings().keyBindings);
        for(int c:new int[]{0,-100,-99,-98,17,17,Integer.MIN_VALUE,Integer.MAX_VALUE})keys.add(new KeyBinding("custom",c));
        Random r=new Random(4181);
        try(PrintWriter o=new PrintWriter(new OutputStreamWriter(new FileOutputStream(args[1]),"UTF-8"))) {
            for(int t=0;t<20000;t++) {
                int op=r.nextInt(7),index=r.nextInt(keys.size()),code=r.nextInt(220)-105;boolean pressed=r.nextBoolean();
                switch(op) {
                    case 0:keys.get(index).keyCode=code;break;
                    case 1:KeyBinding.onTick(code);break;
                    case 2:KeyBinding.setKeyBindState(code,pressed);break;
                    case 3:o.println(keys.get(index).isPressed());break;
                    case 4:KeyBinding.unPressAllKeys();break;
                    case 5:KeyBinding.resetKeyBindingArrayAndHash();break;
                    case 6:KeyBinding.onTick(keys.get(index).keyCode);KeyBinding.setKeyBindState(keys.get(index).keyCode,pressed);break;
                }
                for(KeyBinding k:keys)o.print(k.keyCode+","+k.pressed+","+k.pressTime+";");o.println();
            }
            keys.get(0).pressTime=Integer.MAX_VALUE;keys.get(0).keyCode=12345;KeyBinding.resetKeyBindingArrayAndHash();KeyBinding.onTick(12345);
            o.println(keys.get(0).pressTime);o.println(keys.get(0).isPressed());o.println(keys.get(0).pressTime);
        }
    }
}
