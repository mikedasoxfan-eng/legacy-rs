package net.minecraft.src;
import java.io.*;
import java.util.*;
import java.lang.reflect.Field;

/** Original general-stat list and virtual drawString arguments, without GPU rounding. */
public final class Mode_generalstats10 {
    static final class Stats extends GuiStats {
        DataOutputStream out;
        Stats(StatFileWriter writer){super(new GuiScreen(),writer);}
        public void drawString(FontRenderer font,String text,int x,int y,int color) {
            try{WorldEventOracle.text(out,text);EntityOracle.i(out,x);EntityOracle.i(out,y);EntityOracle.i(out,color);}catch(IOException e){throw new RuntimeException(e);}
        }
    }
    public static void main(String[] a)throws Exception {
        StatFileWriter writer=(StatFileWriter)Oracle.allocate(StatFileWriter.class);
        Field field=StatFileWriter.class.getDeclaredField("field_25102_a");field.setAccessible(true);
        Stats screen=new Stats(writer);screen.mc=(PlayerOracle.HeadlessMinecraft)Oracle.allocate(PlayerOracle.HeadlessMinecraft.class);
        screen.width=427;screen.height=240;screen.fontRenderer=new FontRenderer(new GameSettings(),"/font/default.png",new Mode_guiutility10.Textures(), false);
        GuiSlotStatsGeneral list=new GuiSlotStatsGeneral(screen);Random random=new Random(7281);
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            screen.out=o;
            for(int c=0;c<1024;c++) {
                Map<StatBase,Integer> values=new HashMap<StatBase,Integer>();
                for(Object entry:StatList.generalStats) {
                    StatBase stat=(StatBase)entry;int value=c%7==0?Integer.MIN_VALUE:c%7==1?Integer.MAX_VALUE:c%7==2?-1:c%7==3?0:stat.statId*1664525+c*1013904223;
                    values.put(stat,value);
                }
                field.set(writer,values);int x=random.nextInt(20001)-10000,y=random.nextInt(20001)-10000;
                list.elementClicked(c-512,(c&1)!=0);EntityOracle.i(o,Trace10.namedInt("list.getSize()", list.getSize()));EntityOracle.i(o,Trace10.namedInt("list.getContentHeight()", list.getContentHeight()));Trace10.writeBoolean(o, "list.isSelected(c-512)", list.isSelected(c-512));
                for(int index=0;index<list.getSize();index++)list.drawSlot(index,x,y,10,null);
            }
            StatBase distance=StatList.getOneShotStat(2000);
            for(int c=0;c<10000;c++) {
                int center=random.nextInt(2147483)*1000+500;
                for(int delta=-1;delta<=1;delta++)WorldEventOracle.text(o,distance.func_27084_a(center+delta));
            }
        }
    }
}
