package net.minecraft.src;
import java.io.*;
import java.lang.reflect.*;
import net.minecraft.client.Minecraft;

/** Executes the original create-world input, button and launch callbacks with a recording client. */
public final class Mode_worldcallbacks181 {
    static final class Client extends Minecraft {
        DataOutputStream trace;
        Client(){super(null,null,null,1,1,false);}
        public void displayUnexpectedThrowable(UnexpectedThrowable t){throw new RuntimeException();}
        public ISaveFormat getSaveLoader(){return (ISaveFormat)Proxy.newProxyInstance(ISaveFormat.class.getClassLoader(),new Class[]{ISaveFormat.class},(p,m,a)->{
            if(m.getName().equals("getWorldInfo")) {String s=(String)a[0];return s.equals("Taken")||s.equals("Taken-")||s.equals("Taken--")?new WorldInfo(new WorldSettings(1,0,true),s):null;}
            throw new UnsupportedOperationException(m.getName());
        });}
        public void displayGuiScreen(GuiScreen g){try{trace.writeByte(g==null?0:1);}catch(IOException e){throw new RuntimeException(e);}}
        public void startWorld(String folder,String name,WorldSettings settings){try{trace.writeByte(2);str(trace,folder);str(trace,name);EntityOracle.l(trace,settings.func_35518_a());EntityOracle.i(trace,settings.func_35519_b());trace.writeBoolean(settings.func_35520_c());}catch(Exception e){throw new RuntimeException(e);}}
    }
    static Field field(String name)throws Exception {Field f=GuiCreateWorld.class.getDeclaredField(name);f.setAccessible(true);return f;}
    static void set(GuiCreateWorld g,String name,Object v)throws Exception {field(name).set(g,v);}
    static void str(DataOutputStream o,String s)throws Exception {EntityOracle.i(o,s.length());for(int c=0;c<s.length();c++){int ch=s.charAt(c);o.writeByte(ch);o.writeByte(ch>>8);}}
    static void state(DataOutputStream o,GuiCreateWorld g)throws Exception {
        str(o,((GuiTextField)field("textboxWorldName").get(g)).getText());str(o,((GuiTextField)field("textboxSeed").get(g)).getText());str(o,(String)field("folderName").get(g));
        for(String n:new String[]{"field_35365_g","createClicked","field_35368_i"})o.writeBoolean(field(n).getBoolean(g));
        o.writeBoolean(field("field_35364_f").get(g).equals("creative"));
        o.writeBoolean(((GuiTextField)field("textboxWorldName").get(g)).isFocused);o.writeBoolean(((GuiTextField)field("textboxSeed").get(g)).isFocused);
        for(Object b:g.controlList){GuiButton v=(GuiButton)b;EntityOracle.i(o,v.id);o.writeBoolean(v.enabled);o.writeBoolean(v.enabled2);str(o,v.displayString);}
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
                set(g,"field_35366_j",g.controlList.get(2));set(g,"field_35367_k",g.controlList.get(3));set(g,"field_35372_s",g.controlList.get(4));set(g,"field_35371_t",g.controlList.get(5));
                ((GuiButton)g.controlList.get(3)).displayString=StatCollector.translateToLocal("selectWorld.moreWorldOptions");((GuiButton)g.controlList.get(4)).enabled2=false;((GuiButton)g.controlList.get(5)).enabled2=false;((GuiButton)g.controlList.get(5)).enabled=false;
                Method labels=GuiCreateWorld.class.getDeclaredMethod("func_35363_g");labels.setAccessible(true);labels.invoke(g);
                g.keyTyped((char)0,0);state(o,g);
                if((c&1)!=0)g.actionPerformed((GuiButton)g.controlList.get(2));
                if((c&2)!=0)g.actionPerformed((GuiButton)g.controlList.get(4));
                g.actionPerformed((GuiButton)g.controlList.get(3));g.keyTyped((char)9,15);state(o,g);
                g.keyTyped((char)9,15);g.actionPerformed((GuiButton)g.controlList.get(3));state(o,g);
                g.actionPerformed((GuiButton)g.controlList.get(0));g.actionPerformed((GuiButton)g.controlList.get(0));g.actionPerformed((GuiButton)g.controlList.get(1));
                state(o,g);EntityOracle.i(o,calls.size());o.write(calls.toByteArray());
            }
        }
    }
}
