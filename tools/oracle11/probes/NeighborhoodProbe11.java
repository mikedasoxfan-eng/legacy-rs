package net.minecraft.src;
import java.io.*;
import java.lang.reflect.*;
/** Arbitrary parent arrays, executed by the original three getInts methods. */
public final class NeighborhoodProbe11 {
    static final Field CHUNK, WORLD;
    static {try {CHUNK=GenLayer.class.getDeclaredField("chunkSeed");WORLD=GenLayer.class.getDeclaredField("worldGenSeed");CHUNK.setAccessible(true);WORLD.setAccessible(true);}catch(Exception e){throw new RuntimeException(e);}}
    static final int[][] COORD={{0,0},{-1,-1},{-19,31},{512,-1024},{Integer.MAX_VALUE,Integer.MIN_VALUE},{Integer.MIN_VALUE,Integer.MAX_VALUE},{Integer.MAX_VALUE-1,Integer.MAX_VALUE-1},{Integer.MIN_VALUE+1,Integer.MIN_VALUE+1}};
    static final class Parent extends GenLayer {
        final int[] values;final int x,z,w,h;
        Parent(int[] a,int x,int z,int w,int h){super(0L);values=a;this.x=x;this.z=z;this.w=w;this.h=h;}
        public int[] getInts(int x,int z,int w,int h){if(x!=this.x||z!=this.z||w!=this.w||h!=this.h)throw new AssertionError("parent rectangle");return values.clone();}
    }
    static void emit(DataOutputStream out,int kind,long salt,long seed,int x,int z,int w,int h,int[] p)throws Exception {
        Parent parent=new Parent(p,x-1,z-1,w+2,h+2);
        GenLayer l=kind==0?new GenLayerHills(salt,parent):kind==1?new GenLayerShore(salt,parent):new GenLayerSwampRivers(salt,parent);
        l.initWorldGenSeed(seed);IntCache.resetIntCache();int[] values=l.getInts(x,z,w,h);
        for(int i=0;i<w*h;i++)out.writeInt(values[i]);out.writeLong(CHUNK.getLong(l));out.writeLong(WORLD.getLong(l));
    }
    public static void main(String[] a)throws Exception {
        long seed=Long.parseLong(a[0]);
        try(DataOutputStream out=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            for(int kind=0;kind<3;kind++)for(long salt:new long[]{0,-1,1000})for(int[] q:COORD) {
                for(int ci=-1;ci<=21;ci++) {int center=ci==21?255:ci;
                    for(int pattern=0;pattern<32;pattern++) {
                        int[] p=new int[9];java.util.Arrays.fill(p,(pattern&1)==0?0:20);p[4]=center;
                        for(int side=0;side<4;side++)p[new int[]{1,5,3,7}[side]]=(pattern&(1<<side))==0?center:pattern<16?0:center+1;
                        emit(out,kind,salt,seed,q[0],q[1],1,1,p);
                    }
                }
                for(int[] size:new int[][]{{3,2},{17,13},{2,3}})for(int pattern=0;pattern<3;pattern++) {
                    int w=size[0],h=size[1];int[] p=new int[(w+2)*(h+2)];
                    for(int j=0;j<h+2;j++)for(int i=0;i<w+2;i++){int n=(i*13+j*7+pattern)%24;p[i+j*(w+2)]=n==21?-1:n==22?255:n==23?6:n;}
                    emit(out,kind,salt,seed,q[0],q[1],w,h,p);
                }
            }
        }
    }
}
