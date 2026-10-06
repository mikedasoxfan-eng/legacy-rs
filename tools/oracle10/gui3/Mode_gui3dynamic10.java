package net.minecraft.src;
import java.io.*;import java.util.*;import java.nio.*;import org.lwjgl.opengl.GL11;
public final class Mode_gui3dynamic10 {
 static ArrayList<int[]> calls=new ArrayList<int[]>();
 static class FX extends TextureFX {
  int id,frame;
  FX(int id,int icon,int tile){super(icon);this.id=id;tileSize=tile;textureId=90+id;}
  public void onTick(){calls.add(new int[]{0,id,anaglyphEnabled?1:0});frame++;for(int n=0;n<imageData.length;n++)imageData[n]=(byte)(n*37+id*13+frame*7+(anaglyphEnabled?91:0));}
  public void bindImage(RenderEngine e){calls.add(new int[]{1,id,iconIndex});GL11.glBindTexture(3553,17+id);}
 }
 public static void main(String[] a)throws Exception {
  Block.stone.getClass();RenderEngine engine=new RenderEngine(null,new GameSettings());Random r=new Random(30036);
  try(DataOutputStream o=Gui2Fixture10.out(a[0])) {for(int n=0;n<200;n++){
   ArrayList<TextureFX> effects=new ArrayList<TextureFX>();for(int k=0;k<8;k++)effects.add(new FX(k,k<3?19:k==3?-1:r.nextInt(256),r.nextInt(4)));Gui2Fixture10.field(RenderEngine.class,"textureList").set(engine,effects);GameSettings options=(GameSettings)Gui2Fixture10.field(RenderEngine.class,"options").get(engine);options.anaglyph=n%2==0;RenderEngine.useMipmaps=n%3==0;
   for(int frame=0;frame<3;frame++){
    calls.clear();GL11.gui3Uploads.clear();GL11.gui3Bytes.clear();engine.updateDynamicTextures();
    Gui3Fixture10.i(o,calls.size());for(int[] call:calls)for(int v:call)Gui3Fixture10.i(o,v);
    Gui3Fixture10.i(o,GL11.gui3Uploads.size());for(int k=0;k<GL11.gui3Uploads.size();k++){for(int v:GL11.gui3Uploads.get(k))Gui3Fixture10.i(o,v);Gui3Fixture10.i(o,GL11.gui3Bytes.get(k).length);o.write(GL11.gui3Bytes.get(k));}
    ByteBuffer buffer=(ByteBuffer)Gui2Fixture10.field(RenderEngine.class,"imageData").get(engine);Gui3Fixture10.i(o,buffer.position());Gui3Fixture10.i(o,buffer.limit());
   }
  }}
 }
}
