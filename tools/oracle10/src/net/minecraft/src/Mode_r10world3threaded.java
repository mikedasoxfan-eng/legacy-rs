package net.minecraft.src;
import java.io.*;
import java.util.*;
import java.lang.reflect.*;
import java.util.concurrent.CountDownLatch;

/** Original worker held at a barrier while the original loader replaces/drains its FIFO. */
public final class Mode_r10world3threaded {
    static long counter(ThreadedFileIOBase io,String name)throws Exception{return (Long)Mode_structures10.field(io,name);}
    static List<ThreadedChunkLoaderPending> entries(ThreadedChunkLoader loader)throws Exception {
        return (List<ThreadedChunkLoaderPending>)Mode_structures10.field(loader,"field_40556_a");
    }
    public static void main(String[] a)throws Exception {
        ThreadedFileIOBase io=ThreadedFileIOBase.field_40573_a;io.func_40566_a();
        long queued=counter(io,"field_40572_c"),complete=counter(io,"field_40569_d");
        final CountDownLatch started=new CountDownLatch(1),release=new CountDownLatch(1);
        IThreadedFileIO barrier=new IThreadedFileIO(){public boolean func_40550_A_(){started.countDown();try{release.await();}catch(InterruptedException e){throw new RuntimeException(e);}return false;}};
        io.func_40567_a(barrier);started.await();io.func_40567_a(barrier);
        File dir=new File("target/r10/world3/threaded-java");dir.mkdirs();ThreadedChunkLoader loader=new ThreadedChunkLoader(dir);
        Method enqueue=ThreadedChunkLoader.class.getDeclaredMethod("func_40552_a",ChunkCoordIntPair.class,NBTTagCompound.class);enqueue.setAccessible(true);
        int[][] coords={{1,0},{2,-1},{-1,Integer.MIN_VALUE}};
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            for(int[] q:new int[][]{{1,0,10},{2,-1,20},{1,0,30},{-1,Integer.MIN_VALUE,40},{2,-1,50}}) {
                NBTTagCompound root=new NBTTagCompound();root.setInteger("value",q[2]);enqueue.invoke(loader,new ChunkCoordIntPair(q[0],q[1]),root);
                List<ThreadedChunkLoaderPending> pending=entries(loader);o.writeInt(pending.size());
                for(ThreadedChunkLoaderPending p:pending){o.writeInt(p.field_40739_a.chunkXPos);o.writeInt(p.field_40739_a.chunkZPos);o.writeInt(p.field_40738_b.getInteger("value"));}
                for(int[] c:coords){NBTTagCompound found=null;for(ThreadedChunkLoaderPending p:pending)if(p.field_40739_a.equals(new ChunkCoordIntPair(c[0],c[1])))found=p.field_40738_b;o.writeBoolean(found!=null);if(found!=null)o.writeInt(found.getInteger("value"));}
                o.writeLong(counter(io,"field_40572_c")-queued);o.writeLong(counter(io,"field_40569_d")-complete);
            }
            for(int n=0;n<4;n++){o.writeBoolean(loader.func_40550_A_());o.writeInt(entries(loader).size());}
            release.countDown();io.func_40566_a();o.writeLong(counter(io,"field_40572_c")-queued);o.writeLong(counter(io,"field_40569_d")-complete);
            for(int[] c:coords){DataInputStream in=RegionFileCache.getChunkInputStream(dir,c[0],c[1]);NBTTagCompound root=CompressedStreamTools.func_1141_a(in);in.close();o.writeInt(root.getInteger("value"));o.writeBoolean(!entries(loader).isEmpty());}
        } finally {release.countDown();}
    }
}
