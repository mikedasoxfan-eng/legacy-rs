package net.minecraft.src;
import java.io.*;import java.util.*;import java.lang.reflect.*;import net.minecraft.client.Minecraft;
public final class Mode_gui2inventory10 {
    static class Player extends EntityPlayerSP {
        Collection effects;
        Player(){super(null,null,new Session("Player",""),0);}
        public Collection func_40118_aO(){return effects;}
    }
    static class Spy extends GuiInventory {
        ArrayList<int[]> rectangles;
        Spy(){super(null);}
        public void drawTexturedModalRect(int x,int y,int u,int v,int w,int h){rectangles.add(new int[]{x,y,u,v,w,h});}
    }
    static class Font extends FontRenderer {
        ArrayList<String> text;ArrayList<int[]> labels;
        Font()throws Exception{super(new GameSettings(),"/font/default.png",(Gui2Fixture10.Engine)Oracle.allocate(Gui2Fixture10.Engine.class));}
        public void drawStringWithShadow(String s,int x,int y,int color){text.add(s);labels.add(new int[]{x,y,color});}
    }
    public static void main(String[] a)throws Exception {
        Block.stone.getClass();Spy g=(Spy)Oracle.allocate(Spy.class);g.rectangles=new ArrayList<int[]>();g.xSize=176;g.ySize=166;g.inventorySlots=(Container)Oracle.allocate(ContainerPlayer.class);g.controlList=new ArrayList();
        Minecraft mc=(Minecraft)Oracle.allocate(PlayerOracle.HeadlessMinecraft.class);g.mc=mc;mc.gameSettings=new GameSettings();mc.renderEngine=(Gui2Fixture10.Engine)Oracle.allocate(Gui2Fixture10.Engine.class);mc.playerController=new PlayerControllerSP(mc);
        Player p=(Player)Oracle.allocate(Player.class);p.effects=new ArrayList();mc.thePlayer=p;
        Font font=new Font();font.text=new ArrayList<String>();font.labels=new ArrayList<int[]>();g.fontRenderer=font;
        Method draw=GuiInventory.class.getDeclaredMethod("func_40218_g");draw.setAccessible(true);Random r=new Random(10405);
        try(DataOutputStream o=Gui2Fixture10.out(a[0])) {
            for(int c=0;c<1000;c++) {
                g.width=320+r.nextInt(1000);g.height=240+r.nextInt(1000);p.effects.clear();int count=c%20;
                for(int n=0;n<count;n++)p.effects.add(new PotionEffect(n+1,r.nextInt(144001)-1200,r.nextInt(6)-1));
                g.initGui();Gui2Fixture10.i(o,g.guiLeft);Gui2Fixture10.i(o,g.guiTop);
                g.rectangles.clear();font.text.clear();font.labels.clear();draw.invoke(g);
                Gui2Fixture10.i(o,g.rectangles.size());for(int[] rect:g.rectangles)for(int n:rect)Gui2Fixture10.i(o,n);
                Gui2Fixture10.i(o,font.text.size());for(int n=0;n<font.text.size();n++){Gui2Fixture10.s(o,font.text.get(n));for(int v:font.labels.get(n))Gui2Fixture10.i(o,v);}
            }
        }
    }
}
