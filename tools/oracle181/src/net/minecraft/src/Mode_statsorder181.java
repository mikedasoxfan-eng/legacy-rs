package net.minecraft.src;
import java.io.*;
import java.util.*;
/** Calls the original comparators, including missing-stat precedence and signed overflow. */
public final class Mode_statsorder181 {
    static class Writer extends StatFileWriter {
        int salt;
        Writer(){super(null,null);}
        public int writeStat(StatBase s){return salt%5==0?0:(s.statId^salt)*1664525+1013904223;}
    }
    public static void main(String[] args)throws Exception {
        Block.stone.getClass();Item.shovelSteel.getClass();
        Writer writer=(Writer)Oracle.allocate(Writer.class);GuiStats gui=new GuiStats(null,writer);
        GuiSlotStatsItem items=new GuiSlotStatsItem(gui);GuiSlotStatsBlock blocks=new GuiSlotStatsBlock(gui);
        SorterStatsItem ic=new SorterStatsItem(items,gui);SorterStatsBlock bc=new SorterStatsBlock(blocks,gui);
        Random r=new Random(4181);
        try(DataOutputStream o=new DataOutputStream(new FileOutputStream(args[1]))) {
            for(int t=0;t<20000;t++) {
                boolean block=r.nextBoolean();int column=r.nextInt(5)-1,dir=r.nextInt(3)-1,ia=r.nextInt(block?256:2258),ib=r.nextInt(block?256:2258);writer.salt=r.nextInt();
                GuiSlotStats slot=block?blocks:items;slot.field_27271_e=column;slot.field_27270_f=dir;
                EntityOracle.i(o,block?bc.compare(new StatCrafting(0,"a",ia),new StatCrafting(0,"b",ib)):ic.compare(new StatCrafting(0,"a",ia),new StatCrafting(0,"b",ib)));
            }
        }
    }
}
