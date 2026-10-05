package net.minecraft.src;
import java.io.*;
/** Execute the original base-class methods with observable resource callbacks. */
public final class Mode_tilespecial9_181 {
 static DataOutputStream out;
 static void i(int v)throws IOException{EntityOracle.i(out,v);}
 static void text(String s)throws IOException{if(s==null){i(-1);return;}byte[] b=s.getBytes("UTF-8");i(b.length);out.write(b);}
 static final class Engine extends RenderEngine {
  int id,texture,fail;
  Engine(){super(null,null);}
  public int getTexture(String name){try{i(0);i(id);text(name);}catch(IOException e){throw new RuntimeException(e);}if(fail==2)throw new IllegalStateException();return texture;}
  public void bindTexture(int texture){try{i(1);i(id);i(texture);}catch(IOException e){throw new RuntimeException(e);}if(fail==3)throw new IllegalStateException();}
 }
 static final class Renderer extends TileEntitySpecialRenderer {
  public void renderTileEntityAt(TileEntity tile,double x,double y,double z,float partial){}
  void bind(String name){this.bindTextureByName(name);}
 }
 static int error(Runnable r){try{r.run();return 0;}catch(NullPointerException e){return 1;}catch(RuntimeException e){return 2;}}
 public static void main(String[] a)throws Exception{
  Mode_entities181.world();
  try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))){
   out=o;int cases=0;
   for(int manager=0;manager<3;manager++)for(int engine=0;engine<4;engine++)for(boolean hasFont:new boolean[]{false,true})
    for(String name:new String[]{null,"","/item/chest.png","/item/largechest.png","a\u0000b","%blur%/terrain.png"})
     for(int texture:new int[]{Integer.MIN_VALUE,-1,0,1,31,Integer.MAX_VALUE}){
      Renderer r=new Renderer();TileEntityRenderer m1=(TileEntityRenderer)Oracle.allocate(TileEntityRenderer.class),m2=(TileEntityRenderer)Oracle.allocate(TileEntityRenderer.class);
      FontRenderer f1=(FontRenderer)Oracle.allocate(FontRenderer.class),f2=(FontRenderer)Oracle.allocate(FontRenderer.class);
      Oracle.setField(m1,"fontRenderer",hasFont?f1:null);Oracle.setField(m2,"fontRenderer",hasFont?f2:null);
      Engine e1=(Engine)Oracle.allocate(Engine.class),e2=(Engine)Oracle.allocate(Engine.class);e1.id=1;e2.id=2;
      e1.texture=e2.texture=texture;e1.fail=e2.fail=engine;m1.renderEngine=engine==0?null:e1;m2.renderEngine=engine==0?null:e2;
      r.setTileEntityRenderer(manager==0?null:manager==1?m1:m2);
      i(error(new Runnable(){public void run(){r.bind(name);}}));
      int status=0,font=0;try{FontRenderer f=r.getFontRenderer();font=f==null?0:f==f1?1:2;}catch(NullPointerException ex){status=1;}i(status);i(font);
      // Empty callback must not inspect the renderer or the world.
      r.func_31069_a(null);r.func_31069_a((World)Oracle.allocate(World.class));
      r.setTileEntityRenderer(null);i(error(new Runnable(){public void run(){r.bind(name);}}));i(error(new Runnable(){public void run(){r.getFontRenderer();}}));
      r.setTileEntityRenderer(m2);i(error(new Runnable(){public void run(){r.bind(name);}}));
      i(r.getFontRenderer()==null?0:2);cases++;
     }
   System.out.println("tilespecial9: "+cases+" original-bytecode nullable/context/texture/font cases");
  }
 }
}
