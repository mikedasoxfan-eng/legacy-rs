package net.minecraft.src;
import java.io.*;
import java.util.*;

public final class Mode_r10world3guard {
    static String rejected(BiomeDecorator d,World w,Random r){try{d.decorate(w,r,0,0);throw new AssertionError("duplicate decoration");}catch(RuntimeException e){return e.getMessage();}}
    public static void main(String[] a)throws Exception {
        final World w=Mode_r10world3.flat(0),other=Mode_r10world3.flat(0);
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            for(int id=0;id<16;id++) {
                final BiomeDecorator d=BiomeGenBase.biomeList[id].biomeDecorator;final Random r=new Random(42);
                d.currentWorld=w;d.decoRNG=r;o.writeUTF(rejected(d,other,r));
                final String[] result=new String[1];Thread t=new Thread(new Runnable(){public void run(){result[0]=rejected(d,other,r);}});t.start();t.join();o.writeUTF(result[0]);
                o.writeBoolean(d.currentWorld==w);o.writeLong(EntityOracle.rngState(r));o.writeUTF(rejected(d,other,r));
                d.currentWorld=null;d.decoRNG=null;o.writeBoolean(d.currentWorld!=null);
            }
        }
    }
}
