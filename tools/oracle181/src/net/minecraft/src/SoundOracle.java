package net.minecraft.src;
import java.io.*;
import java.util.Random;
/** Exercises Mojang's original MusInputStream bytecode, with no audio device. */
public final class SoundOracle {
    static void bytes(DataOutputStream out,byte[] b)throws Exception {EntityOracle.i(out,b.length);out.write(b);}
    static void run(String path)throws Exception {
        String[] names={"13.mus","cat.mus","calm1.mus","where are we now.mus","#percent%question?.mus","[]<>\"^`{|}.mus","\u00e9\u96ea\ud834\udd1e.mus","tab\tfile.mus"};
        Random random=new Random(4815162342L);
        try(DataOutputStream out=new DataOutputStream(new FileOutputStream(path))) {
            out.write("B173MUS1".getBytes("US-ASCII"));EntityOracle.i(out,names.length);
            for(String name:names) {
                byte[] cipher=new byte[65537];random.nextBytes(cipher);
                MusInputStream stream=new MusInputStream(null,new File(name).toURI().toURL(),new ByteArrayInputStream(cipher));
                ByteArrayOutputStream plain=new ByteArrayOutputStream();byte[] block=new byte[257];
                int n;while((n=stream.read(block,0,block.length))>=0)plain.write(block,0,n);
                bytes(out,name.getBytes("UTF-8"));bytes(out,cipher);bytes(out,plain.toByteArray());
            }
        }
    }
}
