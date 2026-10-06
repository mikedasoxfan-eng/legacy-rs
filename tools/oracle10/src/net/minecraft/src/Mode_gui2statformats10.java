package net.minecraft.src;
import java.io.*;import java.util.*;
public final class Mode_gui2statformats10 {
    public static void main(String[] a)throws Exception {
        IStatType[] types={new StatTypeSimple(),new StatTypeTime(),new StatTypeDistance()};Random r=new Random(10809);
        try(DataOutputStream o=Gui2Fixture10.out(a[0])) {
            for(int c=0;c<5000;c++){int value=c==0?Integer.MIN_VALUE:c==1?Integer.MAX_VALUE:c==2?0:r.nextInt();for(IStatType type:types)Gui2Fixture10.s(o,type.format(value));}
        }
    }
}
