package net.minecraft.src;
import java.io.*;
import java.util.*;
import net.minecraft.client.Minecraft;
/** Actual confirmation callbacks with arbitrary parents, signed world IDs and button IDs. */
public final class Mode_yesno8_10 {
    static class Client extends Minecraft {
        ArrayList<Boolean> displays=new ArrayList<>();
        Client(){super(null,null,null,1,1,false);}
        public void displayUnexpectedThrowable(UnexpectedThrowable t){throw new RuntimeException();}
        public void displayGuiScreen(GuiScreen g){displays.add(g!=null);}
        public void setIngameFocus(){}
    }
    static class Parent extends GuiScreen {
        boolean returns;ArrayList<int[]> calls=new ArrayList<>();Client client;
        public void deleteWorld(boolean yes,int n){calls.add(new int[]{yes?1:0,n});if(returns)client.displayGuiScreen(this);}
    }
    static void buttons(DataOutputStream o,GuiScreen g)throws Exception {
        EntityOracle.i(o,Trace10.namedInt("g.controlList.size()", g.controlList.size()));
        for(Object value:g.controlList){GuiButton b=(GuiButton)value;for(int n:new int[]{Trace10.namedInt("b.id", b.id),Trace10.namedInt("b.xPosition", b.xPosition),Trace10.namedInt("b.yPosition", b.yPosition),Trace10.namedInt("b.width", b.width),Trace10.namedInt("b.height", b.height)})EntityOracle.i(o,n);Trace10.writeBoolean(o, "b.enabled", b.enabled);Trace10.writeBoolean(o, "b.drawButton", b.drawButton);Mode_worldcallbacks10.str(o,b.displayString);}
    }
    static void displays(DataOutputStream o,Client mc)throws Exception{EntityOracle.i(o,Trace10.namedInt("mc.displays.size()", mc.displays.size()));for(boolean b:mc.displays)Trace10.writeBoolean(o, "b", b);mc.displays.clear();}
    public static void main(String[] a)throws Exception {
        Random r=new Random(8182);
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            for(int c=0;c<4096;c++) {
                int w=r.nextInt(1800),h=r.nextInt(1200),number=r.nextInt(),id=c%3==0?0:c%3==1?1:r.nextInt();
                Client mc=(Client)Oracle.allocate(Client.class);mc.displays=new ArrayList<>();
                Parent parent=new Parent();parent.client=mc;parent.returns=c%2==0;
                GuiYesNo g=new GuiYesNo(parent,"question","warning","Yes \u00e9","No",number);g.mc=mc;g.width=w;g.height=h;g.initGui();if(c%4==0)g.initGui();
                buttons(o,g);g.actionPerformed(new GuiButton(id,0,0,""));EntityOracle.i(o,Trace10.namedInt("parent.calls.size()", parent.calls.size()));for(int[] call:parent.calls){Trace10.writeBoolean(o, "call[0]!=0", call[0]!=0);EntityOracle.i(o,Trace10.namedInt("call[1]", call[1]));}displays(o,mc);
                g.keyTyped((char)0,c%2==0?1:28);displays(o,mc);
            }
        }
    }
}
