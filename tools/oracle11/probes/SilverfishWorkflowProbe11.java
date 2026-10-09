package net.minecraft.src;
import java.io.*;
import java.util.*;
/** Reuses one original renderer across 64 frames and captures its stdout. */
public final class SilverfishWorkflowProbe11 {
 public static void main(String[] args)throws Exception {
  Block.stone.getClass();RendererOracle.field("convertQuadsToTriangles").setBoolean(null,false);
  OpenGlHelper.lightmapDisabled=33984;OpenGlHelper.lightmapEnabled=33985;
  ByteArrayOutputStream diagnostics=new ByteArrayOutputStream();PrintStream saved=System.out;
  Mode_render_models10.out=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(args[0])));
  Mode_render_models10.i(0x58465231);Mode_render_models10.i(1);Mode_render_models10.i(7);
  try {System.setOut(new PrintStream(diagnostics));Mode_render_simple10.mobs(7,new Random(1009008));}
  finally {System.setOut(saved);Mode_render_models10.out.close();}
  try(OutputStream f=new FileOutputStream(args[0]+".stdout")){f.write(diagnostics.toByteArray());}
  if(diagnostics.size()!=0)throw new AssertionError("unexpected original silverfish stdout");
  saved.println("SilverfishWorkflow11:64 reused-renderer original frames; empty stdout");
 }
}
