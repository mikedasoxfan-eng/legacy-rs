package net.minecraft.src;
import java.io.*;import java.util.*;
/** RenderList (1.0 keeps the camera offset in double): the glTranslatef each list emits,
 * for WorldRenderer origins near spawn, far out and at the world border. */
public final class Mode_renderlist10 {
 public static void main(String[] a)throws Exception {
  Random r=new Random(181);int n=20000;
  try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
   Trace10.writeInt(o, "n", n);
   for(int i=0;i<n;i++) {
    double range=new double[]{512,100000,8000000,30000000}[i%4];
    int x=(int)((r.nextDouble()*2-1)*range)&~15,y=r.nextInt(8)*16,z=(int)((r.nextDouble()*2-1)*range)&~15;
    double cx=x+(r.nextDouble()*2-1)*200,cy=r.nextDouble()*160-16,cz=z+(r.nextDouble()*2-1)*200;
    if(i%7==0){cx=Math.floor(cx)+r.nextInt(4)*0.25;cz=Math.floor(cz);}
    // WorldRenderer.setPosition: posXMinus = posX - (posX & 1023)
    int mx=x-(x&1023),my=y-(y&1023),mz=z-(z&1023);
    RenderList l=new RenderList();l.func_861_a(mx,my,mz,cx,cy,cz);l.func_858_a(1);
    RendererOracle.identity();ArrayList<float[]> ops=RendererOracle.ops;
    boolean active=RendererOracle.active;RendererOracle.active=false;l.func_860_a();RendererOracle.active=active;
    float[] t=ops.get(ops.size()-1);if(t[0]!=0)throw new AssertionError("no translate");
    Trace10.writeInt(o, "x", x);Trace10.writeInt(o, "y", y);Trace10.writeInt(o, "z", z);Trace10.writeDouble(o, "cx", cx);Trace10.writeDouble(o, "cy", cy);Trace10.writeDouble(o, "cz", cz);
    Trace10.writeFloat(o, "t[1]", t[1]);Trace10.writeFloat(o, "t[2]", t[2]);Trace10.writeFloat(o, "t[3]", t[3]);
   }
  }
 }
}
