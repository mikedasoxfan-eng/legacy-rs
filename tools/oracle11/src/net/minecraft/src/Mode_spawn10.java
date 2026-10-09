package net.minecraft.src;
import java.io.*;
public final class Mode_spawn10 {
    public static void main(String[] a) throws Exception {
        World w=Oracle.newWorld(Long.parseLong(a[1]));
        w.rand.setSeed(1);w.getInitialSpawnLocation();
        ChunkCoordinates pos=w.getSpawnPoint();
        try(DataOutputStream out=new DataOutputStream(new FileOutputStream(a[2]))) {Trace10.writeInt(out, "pos.posX", pos.posX);Trace10.writeInt(out, "pos.posY", pos.posY);Trace10.writeInt(out, "pos.posZ", pos.posZ);}
    }
}
