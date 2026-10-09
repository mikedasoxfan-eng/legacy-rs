package net.minecraft.src;
import java.io.*;
import java.lang.reflect.*;
import net.minecraft.client.Minecraft;

/** Executes the original create-world input, button and launch callbacks with a recording client. */
public final class Mode_worldcallbacks10 {
    static final class Client extends Minecraft {
        DataOutputStream trace;
        Client(){super(null,null,null,1,1,false);}
        public void displayUnexpectedThrowable(UnexpectedThrowable t){throw new RuntimeException();}
        public ISaveFormat getSaveLoader(){return (ISaveFormat)Proxy.newProxyInstance(ISaveFormat.class.getClassLoader(),new Class[]{ISaveFormat.class},(p,m,a)->{
            if(m.getName().equals("getWorldInfo")) {String s=(String)a[0];return s.equals("Taken")||s.equals("Taken-")||s.equals("Taken--")?new WorldInfo(new WorldSettings(1,0,true, false, EnumWorldType.DEFAULT),s):null;}
            throw new UnsupportedOperationException(m.getName());
        });}
        public void displayGuiScreen(GuiScreen g){try{Trace10.writeByte(trace, "g==null?0:1", g==null?0:1);}catch(IOException e){throw new RuntimeException(e);}}
        public void startWorld(String folder,String name,WorldSettings settings){try{Trace10.writeByte(trace, "2", 2);str(trace,folder);str(trace,name);EntityOracle.l(trace,Trace10.namedLong("settings.getSeed()", settings.getSeed()));EntityOracle.i(trace,Trace10.namedInt("settings.getGameType()", settings.getGameType()));Trace10.writeBoolean(trace, "settings.isMapFeaturesEnabled()", settings.isMapFeaturesEnabled());EntityOracle.i(trace,settings.func_46107_e().ordinal());}catch(Exception e){throw new RuntimeException(e);}}
    }
    static Field field(String name)throws Exception {Field f=GuiCreateWorld.class.getDeclaredField(name);f.setAccessible(true);return f;}
    static void set(GuiCreateWorld g,String name,Object v)throws Exception {field(name).set(g,v);}
    static void str(DataOutputStream o,String s)throws Exception {EntityOracle.i(o,Trace10.namedInt("s.length()", s.length()));for(int c=0;c<s.length();c++){int ch=s.charAt(c);Trace10.writeByte(o, "ch", ch);Trace10.writeByte(o, "ch>>8", ch>>8);}}
    static void state(DataOutputStream o,GuiCreateWorld g)throws Exception {
        str(o,((GuiTextField)field("textboxWorldName").get(g)).getText());str(o,((GuiTextField)field("textboxSeed").get(g)).getText());str(o,(String)field("folderName").get(g));
        for(String n:new String[]{"field_35365_g","createClicked","moreOptions"})Trace10.writeBoolean(o, "field(n).getBoolean(g)", field(n).getBoolean(g));
        Trace10.writeBoolean(o, "field(\"field_35364_f\").get(g).equals(\"creative\")", field("gameMode").get(g).equals("creative"));
        Trace10.writeBoolean(o, "((GuiTextField)field(\"textboxWorldName\").get(g)).isFocused", ((GuiTextField)field("textboxWorldName").get(g)).isFocused);Trace10.writeBoolean(o, "((GuiTextField)field(\"textboxSeed\").get(g)).isFocused", ((GuiTextField)field("textboxSeed").get(g)).isFocused);
        for(Object b:g.controlList){GuiButton v=(GuiButton)b;EntityOracle.i(o,Trace10.namedInt("v.id", v.id));Trace10.writeBoolean(o, "v.enabled", v.enabled);Trace10.writeBoolean(o, "v.drawButton", v.drawButton);str(o,v.displayString);}
    }
    public static void main(String[] args)throws Exception {
        String[] names={"Name","Taken","Taken-","Empty","","   ","  trim  ","A/B\\C:D*?<>|\"`","World\nName","\tWorld\r","\u00a0","\u2003","\u00e9","\ud83d\ude00"};
        String[] seeds={"+123","-123","9223372036854775807","-9223372036854775808","9223372036854775808","hello"};
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(args[1])))) {
            for(int c=0;c<336;c++) {
                ByteArrayOutputStream calls=new ByteArrayOutputStream();Client mc=(Client)Oracle.allocate(Client.class);mc.trace=new DataOutputStream(calls);
                GuiCreateWorld g=new GuiCreateWorld(new GuiScreen());g.mc=mc;g.width=427;g.height=240;
                GuiTextField name=new GuiTextField(g,null,113,60,200,20,names[c%names.length]);name.isFocused=true;name.setMaxStringLength(32);
                GuiTextField seed=new GuiTextField(g,null,113,60,200,20,seeds[c%seeds.length]);set(g,"textboxWorldName",name);set(g,"textboxSeed",seed);
                for(int id=0;id<6;id++)g.controlList.add(new GuiButton(id,0,0,""));
                ((GuiButton)g.controlList.get(0)).displayString=StatCollector.translateToLocal("selectWorld.create");((GuiButton)g.controlList.get(1)).displayString=StatCollector.translateToLocal("gui.cancel");
                set(g,"gameModeButton",g.controlList.get(2));set(g,"moreWorldOptions",g.controlList.get(3));set(g,"generateStructuresButton",g.controlList.get(4));set(g,"worldTypeButton",g.controlList.get(5));
                ((GuiButton)g.controlList.get(3)).displayString=StatCollector.translateToLocal("selectWorld.moreWorldOptions");((GuiButton)g.controlList.get(4)).drawButton=false;((GuiButton)g.controlList.get(5)).drawButton=false;
                Method labels=GuiCreateWorld.class.getDeclaredMethod("func_35363_g");labels.setAccessible(true);labels.invoke(g);
                g.keyTyped((char)0,0);state(o,g);
                if((c&1)!=0)g.actionPerformed((GuiButton)g.controlList.get(2));
                if((c&2)!=0)g.actionPerformed((GuiButton)g.controlList.get(4));
                g.actionPerformed((GuiButton)g.controlList.get(3));for(int t=0;t<c%3;t++)g.actionPerformed((GuiButton)g.controlList.get(5));g.keyTyped((char)9,15);state(o,g);
                g.keyTyped((char)9,15);g.actionPerformed((GuiButton)g.controlList.get(3));state(o,g);
                g.actionPerformed((GuiButton)g.controlList.get(0));g.actionPerformed((GuiButton)g.controlList.get(0));g.actionPerformed((GuiButton)g.controlList.get(1));
                state(o,g);EntityOracle.i(o,Trace10.namedInt("calls.size()", calls.size()));Trace10.writeBytes(o, "calls.toByteArray()", calls.toByteArray());
            }
        }
    }
}
