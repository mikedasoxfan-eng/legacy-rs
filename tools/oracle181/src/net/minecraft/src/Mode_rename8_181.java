package net.minecraft.src;
import java.io.*;
import java.util.*;
import java.lang.reflect.*;
import net.minecraft.client.Minecraft;
import org.lwjgl.input.Keyboard;
/** Calls actual original rename init/input/save callbacks and missing-save failure. */
public final class Mode_rename8_181 {
    static class Client extends Minecraft {
        String name;ArrayList<Boolean> displays;
        Client(){super(null,null,null,1,1,false);}
        public void displayUnexpectedThrowable(UnexpectedThrowable t){throw new RuntimeException();}
        public ISaveFormat getSaveLoader(){return (ISaveFormat)Proxy.newProxyInstance(ISaveFormat.class.getClassLoader(),new Class[]{ISaveFormat.class},(p,m,a)->{
            if(m.getName().equals("getWorldInfo"))return a[0].equals("Missing")?null:new WorldInfo(new WorldSettings(0,0,true),name);
            if(m.getName().equals("func_22170_a")){if(!a[0].equals("Fixture"))throw new AssertionError();name=(String)a[1];return null;}
            throw new UnsupportedOperationException(m.getName());
        });}
        public void displayGuiScreen(GuiScreen g){displays.add(g!=null);}
    }
    public static void main(String[] a)throws Exception {
        String[] names={"World","","   ","  trim  ","\tWorld\r","\u00a0","\u2003","caf\u00e9","0123456789012345678901234567890123456789"};
        Field text=GuiRenameWorld.class.getDeclaredField("theGuiTextField");text.setAccessible(true);
        Field max=GuiTextField.class.getDeclaredField("maxStringLength");max.setAccessible(true);
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            Client mc=(Client)Oracle.allocate(Client.class);mc.displays=new ArrayList<>();
            for(int c=0;c<360;c++) {
                mc.name=names[c%names.length];GuiRenameWorld g=new GuiRenameWorld(new GuiScreen(),"Fixture");g.mc=mc;g.width=427;g.height=240;g.controlList.add(new GuiButton(17,0,0,"dirty"));g.initGui();
                Mode_yesno8_181.buttons(o,g);o.writeBoolean(Keyboard.areRepeatEventsEnabled());
                for(int t=0;t<c%30;t++)g.updateScreen();
                int x=c%4==0?112:113+c%200;g.mouseClicked(x,60,c%3==0?1:0);
                for(int[] key:new int[][]{{'x',45},{0,14},{9,15},{' ',57},{0,1}})g.keyTyped((char)key[0],key[1]);
                GuiTextField f=(GuiTextField)text.get(g);Mode_worldcallbacks181.str(o,f.getText());o.writeBoolean(f.isFocused);o.writeBoolean(((GuiButton)g.controlList.get(0)).enabled);EntityOracle.i(o,max.getInt(f));
                if(c%5==0)((GuiButton)g.controlList.get(0)).enabled=false;
                switch(c%4){case 0:g.keyTyped((char)13,28);break;case 1:g.actionPerformed((GuiButton)g.controlList.get(0));break;case 2:g.actionPerformed((GuiButton)g.controlList.get(1));break;default:g.actionPerformed(new GuiButton(17,0,0,""));}
                Mode_worldcallbacks181.str(o,mc.name);EntityOracle.i(o,mc.displays.size());for(boolean b:mc.displays)o.writeBoolean(b);mc.displays.clear();g.onGuiClosed();if(Keyboard.areRepeatEventsEnabled())throw new AssertionError();
            }
            GuiRenameWorld missing=new GuiRenameWorld(new GuiScreen(),"Missing");missing.mc=mc;
            boolean failed=false;try{missing.initGui();}catch(NullPointerException expected){failed=true;}o.writeBoolean(failed);Mode_yesno8_181.buttons(o,missing);
        }
    }
}
