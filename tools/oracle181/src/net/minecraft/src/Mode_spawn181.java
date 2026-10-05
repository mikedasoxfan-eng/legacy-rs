package net.minecraft.src;
import java.io.*;
public final class Mode_spawn181 {
    public static void main(String[] a) throws Exception {
        World w=Oracle.newWorld(Long.parseLong(a[1]));
        w.rand.setSeed(1);w.getInitialSpawnLocation();
        ChunkCoordinates pos=w.getSpawnPoint();
        try(DataOutputStream out=new DataOutputStream(new FileOutputStream(a[2]))) {out.writeInt(pos.posX);out.writeInt(pos.posY);out.writeInt(pos.posZ);}
    }
}
