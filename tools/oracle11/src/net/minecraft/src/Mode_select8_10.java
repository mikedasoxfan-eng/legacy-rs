package net.minecraft.src;
import java.io.*;
import java.util.*;
import java.lang.reflect.*;
import net.minecraft.client.Minecraft;
/** Original saved-world controller policy, labels, selection latch and delete callbacks. */
public final class Mode_select8_10 {
    static class Client extends Minecraft {
        ArrayList<SaveFormatComparator> saves;DataOutputStream trace;int displays;
        Client(){super(null,null,null,1,1,false);}
        public void displayUnexpectedThrowable(UnexpectedThrowable t){throw new RuntimeException();}
        public ISaveFormat getSaveLoader(){return (ISaveFormat)Proxy.newProxyInstance(ISaveFormat.class.getClassLoader(),new Class[]{ISaveFormat.class},(p,m,a)->{
            if(m.getName().equals("getSaveList"))return new ArrayList(saves);
            if(m.getName().equals("flushCache"))return null;
            if(m.getName().equals("deleteWorldDirectory")){for(Iterator<SaveFormatComparator> it=saves.iterator();it.hasNext();)if(it.next().getFileName().equals(a[0]))it.remove();return null;}
            throw new UnsupportedOperationException(m.getName());
        });}
        public void displayGuiScreen(GuiScreen g){displays++;try{if(trace!=null)Trace10.writeByte(trace, "g==null?0:1", g==null?0:1);}catch(Exception e){throw new RuntimeException(e);}}
        public void startWorld(String folder,String name,WorldSettings settings){try{Trace10.writeByte(trace, "2", 2);Mode_worldcallbacks10.str(trace,folder);Mode_worldcallbacks10.str(trace,name);Trace10.writeBoolean(trace, "playerController instanceof PlayerControllerCreative", playerController instanceof PlayerControllerCreative);Trace10.writeBoolean(trace, "settings==null", settings==null);EntityPlayer p=playerController.createPlayer(theWorld);playerController.flipPlayer(p);EntityOracle.f(trace,Trace10.namedFloat("p.rotationYaw", p.rotationYaw));}catch(Exception e){throw new RuntimeException(e);}}
    }
    static Field field(String n)throws Exception {Field f=GuiSelectWorld.class.getDeclaredField(n);f.setAccessible(true);return f;}
    static ArrayList<SaveFormatComparator> fixture(int mode,boolean empty){ArrayList<SaveFormatComparator> result=new ArrayList<>();for(int i=0;i<3;i++){String name=""+(char)('A'+i);result.add(new SaveFormatComparator(name,empty?"":"Name "+name,2-i,0,mode,false, false));}return result;}
    public static void main(String[] a)throws Exception {
        Random r=new Random(8183);Client mc=(Client)Oracle.allocate(Client.class);
        mc.theWorld=Mode_entities10.world();mc.session=new Session("Player","local");
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            for(int c=0;c<2048;c++) {
                int mode=c%8==0?0:c%8==1?1:c%8==2?-1:c%8==3?2:c%8==4?Integer.MIN_VALUE:c%8==5?Integer.MAX_VALUE:r.nextInt();
                int index=c%5-1;mc.saves=fixture(mode,c%2==0);GuiSelectWorld g=new GuiSelectWorld(new GuiScreen());EntityOracle.i(o,Trace10.namedInt("field(\"selectedWorld\").getInt(g)", field("selectedWorld").getInt(g)));g.mc=mc;g.width=427;g.height=240;g.initGui();
                String[] modes=(String[])field("localizedGameModeText").get(g);
                for(String label:new String[]{(String)field("screenTitle").get(g),(String)field("localizedWorldText").get(g),(String)field("localizedMustConvertText").get(g),modes[0],modes[1]})Mode_worldcallbacks10.str(o,label);
                EntityOracle.i(o,Trace10.namedInt("field(\"selectedWorld\").getInt(g)", field("selectedWorld").getInt(g)));Mode_yesno8_10.buttons(o,g);
                ByteArrayOutputStream calls=new ByteArrayOutputStream();mc.trace=new DataOutputStream(calls);mc.displays=0;
                boolean failed=false;try{g.selectWorld(index);}catch(IndexOutOfBoundsException expected){failed=true;}
                Trace10.writeBoolean(o, "failed", failed);Trace10.writeBoolean(o, "field(\"selected\").getBoolean(g)", field("selected").getBoolean(g));EntityOracle.i(o,Trace10.namedInt("mc.displays+(failed?0:1)", mc.displays+(failed?0:1)));Trace10.writeBytes(o, "calls.toByteArray()", calls.toByteArray());
                calls.reset();mc.displays=0;g.selectWorld(-77);EntityOracle.i(o,Trace10.namedInt("mc.displays", mc.displays));Trace10.writeBytes(o, "calls.toByteArray()", calls.toByteArray());mc.trace=null;
            }
            for(int c=0;c<96;c++) {
                mc.saves=fixture(0,false);GuiSelectWorld g=new GuiSelectWorld(new GuiScreen());g.mc=mc;g.width=427;g.height=240;g.initGui();field("selectedWorld").setInt(g,1);field("deleting").setBoolean(g,c%3!=0);mc.displays=0;
                g.deleteWorld(c%2==0,1);EntityOracle.i(o,Trace10.namedInt("mc.displays", mc.displays));Trace10.writeBoolean(o, "field(\"deleting\").getBoolean(g)", field("deleting").getBoolean(g));EntityOracle.i(o,Trace10.namedInt("field(\"selectedWorld\").getInt(g)", field("selectedWorld").getInt(g)));java.util.List list=(java.util.List)field("saveList").get(g);EntityOracle.i(o,Trace10.namedInt("list.size()", list.size()));for(Object v:list)Mode_worldcallbacks10.str(o,((SaveFormatComparator)v).getFileName());
            }
        }
    }
}
