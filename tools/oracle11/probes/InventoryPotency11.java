package net.minecraft.src;
import java.io.*;import java.lang.reflect.*;import java.util.*;import net.minecraft.client.Minecraft;
/** Private inventory panel entrypoint executes unchanged bytecode; font recorder captures actual call arguments. */
public final class InventoryPotency11 {
 static class Client extends Minecraft{Client(){super(null,null,null,1,1,false);}public void displayUnexpectedThrowable(UnexpectedThrowable t){throw new RuntimeException();}}
 static final ArrayList<String> labels=new ArrayList<String>();static final ArrayList<int[]> positions=new ArrayList<int[]>();
 static class Font extends FontRenderer {Font(GameSettings s,RenderEngine r){super(s,"/font/default.png",r,false);}public void drawStringWithShadow(String s,int x,int y,int c){labels.add(s);positions.add(new int[]{x,y,c});}}
 static void set(Object value,Class<?> owner,String name,Object v)throws Exception{Field f=owner.getDeclaredField(name);f.setAccessible(true);f.set(value,v);}
 static void text(DataOutputStream o,String s)throws Exception{o.writeInt(s.length());for(int i=0;i<s.length();i++)o.writeChar(s.charAt(i));}
 public static void main(String[] args)throws Exception{
  Locale.setDefault(Locale.US);Block.stone.getClass();RenderManager manager=RendererOracle.manager(null);RendererOracle.field("convertQuadsToTriangles").setBoolean(null,false);
  Client mc=(Client)Oracle.allocate(Client.class);mc.gameSettings=new GameSettings();mc.renderEngine=manager.renderEngine;mc.thePlayer=(EntityPlayerSP)Oracle.allocate(EntityPlayerSP.class);
  GuiInventory screen=(GuiInventory)Oracle.allocate(GuiInventory.class);set(screen,GuiScreen.class,"mc",mc);set(screen,GuiScreen.class,"fontRenderer",new Font(mc.gameSettings,mc.renderEngine));set(screen,GuiContainer.class,"ySize",166);
  Method draw=GuiInventory.class.getDeclaredMethod("func_40218_g");draw.setAccessible(true);
  String[] codes={"en_US","ar_SA","he_IL","ja_JP","el_GR","de_DE"};int[] amps={-1,0,1,2,3,4,127};
  try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(args[0])))){
   o.writeInt(0x49505031);o.writeInt(codes.length*19*amps.length*3);
   for(String code:codes){StringTranslate.getInstance().func_44023_a(code);for(int[] xy:new int[][]{{0,0},{197,-31},{-124,91}})for(int id=1;id<=19;id++)for(int amp:amps){
    set(screen,GuiContainer.class,"guiLeft",xy[0]+124);set(screen,GuiContainer.class,"guiTop",xy[1]);HashMap<Integer,PotionEffect> effects=new HashMap<Integer,PotionEffect>();effects.put(id,new PotionEffect(id,4181,amp));set(mc.thePlayer,EntityLiving.class,"activePotionsMap",effects);
    labels.clear();positions.clear();RendererOracle.reset(0);draw.invoke(screen);text(o,code);o.writeInt(xy[0]);o.writeInt(xy[1]);o.writeInt(id);o.writeInt(amp);o.writeInt(labels.size());for(int i=0;i<labels.size();i++){text(o,labels.get(i));for(int n:positions.get(i))o.writeInt(n);}
   }}
  }
 }
}
