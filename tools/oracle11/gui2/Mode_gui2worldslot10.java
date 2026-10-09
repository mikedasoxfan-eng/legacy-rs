package net.minecraft.src;
import java.io.*;import java.util.*;import java.text.*;import net.minecraft.client.Minecraft;
public final class Mode_gui2worldslot10 {
    static class Parent extends GuiSelectWorld {
        ArrayList<String> texts=new ArrayList<String>();ArrayList<int[]> labels=new ArrayList<int[]>();
        Parent(){super(new GuiScreen());}
        public void drawString(FontRenderer font,String s,int x,int y,int color){texts.add(s);labels.add(new int[]{x,y,color});}
    }
    public static void main(String[] a)throws Exception {
        Parent p=new Parent();p.mc=(Minecraft)Oracle.allocate(PlayerOracle.HeadlessMinecraft.class);p.width=854;p.height=480;
        Gui2Fixture10.field(GuiSelectWorld.class,"dateFormatter").set(p,new SimpleDateFormat("M/d/yy h:mm a",Locale.US));
        Gui2Fixture10.field(GuiSelectWorld.class,"localizedWorldText").set(p,StatCollector.translateToLocal("selectWorld.world"));Gui2Fixture10.field(GuiSelectWorld.class,"localizedMustConvertText").set(p,StatCollector.translateToLocal("selectWorld.conversion"));
        Gui2Fixture10.field(GuiSelectWorld.class,"localizedGameModeText").set(p,new String[]{StatCollector.translateToLocal("gameMode.survival"),StatCollector.translateToLocal("gameMode.creative")});
        Random r=new Random(11617);GuiWorldSlot slot=new GuiWorldSlot(p);
        try(DataOutputStream o=Gui2Fixture10.out(a[0])) {
            for(int c=0;c<1000;c++) {
                long time=r.nextInt(2000000000);long size=r.nextLong();int x=r.nextInt(1800)-200,y=r.nextInt(1200)-100;
                ArrayList saves=new ArrayList();for(int n=0;n<4;n++)saves.add(new SaveFormatComparator("World_"+n,n%2==0?"":"Display "+n,time,size,n%2,c%3==0,c%2==0));Gui2Fixture10.field(GuiSelectWorld.class,"saveList").set(p,saves);
                p.texts.clear();p.labels.clear();slot.drawSlot(c%4,x,y,36,Tessellator.instance);Gui2Fixture10.i(o,p.texts.size());
                for(int n=0;n<p.texts.size();n++){Gui2Fixture10.s(o,p.texts.get(n));for(int v:p.labels.get(n))Gui2Fixture10.i(o,v);}
            }
        }
    }
}
