package net.minecraft.src;
import java.io.*;
// World.randomDisplayUpdates in the flat entity fixture: 1.0 "depthsuspend" specks near the
// bottom of the world and the World.rand calls they make. Usage: displayloop10 <out>
public final class Mode_displayloop10 {
    public static void main(String[] a)throws Exception {
        World w=Mode_entities10.world();
        int[][] centers={{0,64,0},{0,4,0},{0,1,0},{-9,10,7},{5,0,-5},{0,-3,0}};
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            for(int[] c:centers)for(int rep=0;rep<3;rep++) {
                w.randomDisplayUpdates(c[0],c[1],c[2]);
                WorldEventOracle.snapshot(w,o);EntityOracle.l(o,Trace10.namedLong("w.rand.nextLong()", w.rand.nextLong()));
            }
        }
    }
}
