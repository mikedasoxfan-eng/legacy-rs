package net.minecraft.src;
import java.io.*;import java.awt.image.*;import javax.imageio.ImageIO;import java.util.*;import java.lang.reflect.*;import java.nio.*;
public final class Mode_renderengine11_10 {
 static StringBuilder events;
 static String flag(boolean b){return b?"1":"0";}
 static BufferedImage image(int w,int h,int seed){BufferedImage i=new BufferedImage(w,h,2);for(int y=0;y<h;y++)for(int x=0;x<w;x++)i.setRGB(x,y,0xff000000|((seed+x+y*w)*3571&0xffffff));return i;}
 static int hash(BufferedImage i){return Arrays.hashCode(i.getRGB(0,0,i.getWidth(),i.getHeight(),null,0,i.getWidth()));}
 static class Pack extends TexturePackBase{
  Engine engine;int behavior,width=32,height=3,seed=7;
  public InputStream getResourceAsStream(String path){events.append("R:").append(path).append(':').append(flag(engine.blurTexture)).append(flag(engine.clampTexture)).append(';');
   if(behavior==1)return null;if(behavior==4)throw new IllegalStateException();
   if(behavior==2)return new InputStream(){public int read()throws IOException{throw new IOException();}};
   if(behavior==3)return new ByteArrayInputStream(new byte[]{1,2,3});
   try{ByteArrayOutputStream out=new ByteArrayOutputStream();ImageIO.write(image(width,height,seed),"png",out);if(behavior==5)return new ByteArrayInputStream(out.toByteArray()){public void close()throws IOException{throw new IOException();}};return new ByteArrayInputStream(out.toByteArray());}catch(IOException e){throw new RuntimeException(e);}
  }
 }
 static class Engine extends RenderEngine{
  int fail;
  Engine(TexturePackList p){super(p,new GameSettings());}
  public void setupTexture(BufferedImage i,int id){
   if(i==null)throw new NullPointerException();events.append("U:").append(id).append(':').append(i.getWidth()).append('x').append(i.getHeight()).append(':').append(hash(i)).append(':').append(flag(blurTexture)).append(flag(clampTexture)).append(';');
   if(fail>0){fail--;throw new IllegalStateException();}
  }
 }
 static void resetNames()throws Exception{setNames(1);}
 static void setNames(int value)throws Exception{Field f=org.lwjgl.opengl.GL11.class.getDeclaredField("textureName10");f.setAccessible(true);f.setInt(null,value);}
 static Engine engine(Pack p)throws Exception{resetNames();TexturePackList list=(TexturePackList)Oracle.allocate(TexturePackList.class);list.selectedTexturePack=p;Engine e=new Engine(list);p.engine=e;Field f=RenderEngine.class.getDeclaredField("missingTextureImage");f.setAccessible(true);f.set(e,image(2,2,99));return e;}
 static String outcome(Engine e,Pack p,String name,boolean contents){String result;try{
  if(contents){int[] first=e.getTextureContents(name);first[0]=123;int[] again=e.getTextureContents(name);result="C:"+(first==again)+":"+Arrays.hashCode(first);}
  else{int id=e.getTexture(name);int again=e.getTexture(name);result="T:"+id+":"+(id==again);}
 }catch(Exception ex){result="E:"+ex.getClass().getSimpleName();}return events+result+":"+flag(e.blurTexture)+flag(e.clampTexture);}
 static void text(DataOutputStream o,String s)throws Exception{byte[] b=s.getBytes("UTF-8");Trace10.writeInt(o, "b.length", b.length);Trace10.writeBytes(o, "b", b);}
 public static void main(String[] a)throws Exception{
  String[] prefixes={"","##","%clamp%","%blur%","%blurclamp%","%blur%%clamp%","%clamp%%blur%"};
  try(DataOutputStream o=new DataOutputStream(new FileOutputStream(a[1]))){
   Trace10.writeInt(o, "672", 672);
   for(int c=0;c<672;c++){events=new StringBuilder();Pack p=new Pack();Engine e=engine(p);p.behavior=c%6;e.blurTexture=(c/6%2)!=0;e.clampTexture=(c/12%2)!=0;boolean contents=c/24%2!=0;String name=prefixes[c/48%7]+"/fixture.png";e.fail=c/336;
    Trace10.writeInt(o, "p.behavior", p.behavior);Trace10.writeBoolean(o, "e.blurTexture", e.blurTexture);Trace10.writeBoolean(o, "e.clampTexture", e.clampTexture);Trace10.writeBoolean(o, "contents", contents);text(o,name);Trace10.writeInt(o, "e.fail", e.fail);text(o,outcome(e,p,name,contents));
   }
   // Dynamic image identity/mutation/deletion and Java key-set refresh ordering.
   events=new StringBuilder();Pack p=new Pack();Engine e=engine(p);ArrayList<BufferedImage> images=new ArrayList<>();ArrayList<Integer> ids=new ArrayList<>();
   for(int i=0;i<180;i++){BufferedImage im=image(2,2,i);images.add(im);ids.add(e.allocateAndSetupTexture(im));}
   for(int i=0;i<180;i+=3)e.deleteTexture(ids.get(i));for(int i=0;i<180;i++)images.get(i).setRGB(0,0,0xffabcdef+i);
   e.blurTexture=true;e.clampTexture=true;e.refreshTextures();text(o,events.toString());
   // Nonsequential/reused GL names force HashSet tree bins and deletion balancing.
   events=new StringBuilder();p=new Pack();e=engine(p);int[] keys=new int[64];
   for(int i=0;i<64;i++){int h=(i+1)*1024;keys[i]=h^(h>>>16);setNames(keys[i]);e.allocateAndSetupTexture(image(2,2,i));}
   for(int t=0;t<64;t++){int i=t*37%64;e.deleteTexture(keys[i]);if(t%5==0){setNames(keys[i]);e.allocateAndSetupTexture(image(2,2,1000+t));}e.blurTexture=t%2!=0;e.clampTexture=t%3==0;e.refreshTextures();}
   text(o,events.toString());
   // Contents refresh retains the same array, its stale tail, and partial writes.
   Trace10.writeInt(o, "20", 20);for(int c=0;c<20;c++){events=new StringBuilder();p=new Pack();e=engine(p);String name=prefixes[c%7]+"/cached.png";int[] cache=e.getTextureContents(name);cache[cache.length-1]=123;p.width=c%4==0?16:c%4==1?32:48;p.height=c%4==2?4:2;p.seed=81;p.behavior=c/4%5;String status="ok";try{e.refreshTextures();}catch(Exception ex){status=ex.getClass().getSimpleName();}Trace10.writeInt(o, "c", c);text(o,events+status+":"+Arrays.hashCode(cache)+":"+flag(e.blurTexture)+flag(e.clampTexture));}
   // Unified refresh: Java string-map order, retained flags, resource failures,
   // contents identity and deleteTexture retaining the named cache entry.
   Trace10.writeInt(o, "96", 96);for(int c=0;c<96;c++){
    events=new StringBuilder();p=new Pack();e=engine(p);ArrayList<int[]> caches=new ArrayList<>();ArrayList<Integer> named=new ArrayList<>();
    for(int i=0;i<7;i++){String name=prefixes[(i+c)%7]+"/mixed"+i+".png";named.add(e.getTexture(name));caches.add(e.getTextureContents(name));}
    for(int i=0;i<3;i++)e.allocateAndSetupTexture(image(2,2,i));
    e.deleteTexture(named.get(0));events=new StringBuilder();e.blurTexture=c%2!=0;e.clampTexture=c%4>=2;p.behavior=c/4%6;p.seed=87;p.width=c/24%2==0?16:32;p.height=2;e.fail=c/48;
    String status="ok";try{e.refreshTextures();}catch(Exception ex){status=ex.getClass().getSimpleName();}
    StringBuilder state=new StringBuilder();for(int[] cache:caches)state.append(Arrays.hashCode(cache)).append(',');
    int cached=e.getTexture(prefixes[c%7]+"/mixed0.png");Trace10.writeInt(o, "c", c);text(o,events+status+":"+flag(e.blurTexture)+flag(e.clampTexture)+":"+state+":"+(cached==named.get(0)));
   }
   // String.compareTo, rather than identity order, breaks equal-hash tree ties.
   events=new StringBuilder();p=new Pack();e=engine(p);for(int i=0;i<32;i++){StringBuilder name=new StringBuilder("/collision/");for(int bit=0;bit<5;bit++)name.append((i&(1<<bit))==0?"Aa":"BB");name.append(".png");p.seed=i;e.getTexture(name.toString());e.getTextureContents(name.toString());}events=new StringBuilder();p.seed=200;e.refreshTextures();text(o,events.toString());
   // Actual unoverridden byte-buffer paths, including capacity and prior state.
   TexturePackList list=(TexturePackList)Oracle.allocate(TexturePackList.class);list.selectedTexturePack=new Pack();RenderEngine base=new RenderEngine(list,new GameSettings());
   Trace10.writeInt(o, "12", 12);for(int c=0;c<12;c++){int w=c%6==0?2048:c%6==1?2049:c%6==2?1024:c%6==3?0:c%6==4?-1:4;int h=c%6<3?2048:4;int n=c%6==5?17:0;String status="ok";
    try{if(c<6){if(w<=0)base.setupTexture(null,900);else base.setupTexture(image(w,h,3),900);}else base.createTextureFromBytes(new int[n],w,h,900);}catch(Exception ex){status=ex.getClass().getSimpleName();}
    ByteBuffer buffer=(ByteBuffer)Mode_statsync10.field(base,"imageData");Trace10.writeInt(o, "c", c);text(o,status+":"+buffer.position()+":"+buffer.limit());
   }
  }
  System.out.println("RENDERENGINE11 original cache/failure/dynamic/buffer trace");
 }
}
