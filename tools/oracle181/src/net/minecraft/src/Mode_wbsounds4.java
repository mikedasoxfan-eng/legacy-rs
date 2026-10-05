package net.minecraft.src;
import java.io.*;
public final class Mode_wbsounds4 {
    public static void main(String[] a)throws Exception {
        int init=Block.stone.blockID;
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            for(int id=1;id<=109;id++)if(Block.blocksList[id]!=null) {
                EntityOracle.i(o,id);StepSound s=Block.blocksList[id].stepSound;
                for(String name:new String[]{s.stepSoundDir2(),s.stepSoundDir()}) {
                    byte[] bytes=name.getBytes("UTF-8");EntityOracle.i(o,bytes.length);o.write(bytes);EntityOracle.f(o,s.getVolume());EntityOracle.f(o,s.getPitch());
                }
            }
        }
    }
}
