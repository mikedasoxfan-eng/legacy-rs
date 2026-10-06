package net.minecraft.src;
import java.io.*;import java.util.*;import java.lang.reflect.*;import net.minecraft.client.Minecraft;
public final class Mode_gui2backgrounds10 {
    static ArrayList<int[]> rectangles=new ArrayList<int[]>();
    static void rect(int... r){rectangles.add(r);}
    static class Furnace extends GuiFurnace {Furnace(){super(null,null);}public void drawTexturedModalRect(int x,int y,int u,int v,int w,int h){rect(x,y,u,v,w,h);}}
    static class Craft extends GuiCrafting {Craft(){super(null,null,0,0,0);}public void drawTexturedModalRect(int x,int y,int u,int v,int w,int h){rect(x,y,u,v,w,h);}}
    static class Dispenser extends GuiDispenser {Dispenser(){super(null,null);}public void drawTexturedModalRect(int x,int y,int u,int v,int w,int h){rect(x,y,u,v,w,h);}}
    static class Chest extends GuiChest {Chest(){super(null,null);}public void drawTexturedModalRect(int x,int y,int u,int v,int w,int h){rect(x,y,u,v,w,h);}}
    static class Brewing extends GuiBrewingStand {Brewing(){super(null,null);}public void drawTexturedModalRect(int x,int y,int u,int v,int w,int h){rect(x,y,u,v,w,h);}}
    static class Font extends FontRenderer {
        ArrayList<String> text=new ArrayList<String>();ArrayList<int[]> labels=new ArrayList<int[]>();
        Font()throws Exception{super(new GameSettings(),"/font/default.png",(Gui2Fixture10.Engine)Oracle.allocate(Gui2Fixture10.Engine.class));}
        public void drawString(String s,int x,int y,int color){text.add(s);labels.add(new int[]{x,y,color});}
    }
    public static void main(String[] a)throws Exception {
        Block.stone.getClass();Minecraft mc=(Minecraft)Oracle.allocate(PlayerOracle.HeadlessMinecraft.class);mc.renderEngine=(Gui2Fixture10.Engine)Oracle.allocate(Gui2Fixture10.Engine.class);Font font=new Font();Random r=new Random(10607);
        Class<?>[] types={Furnace.class,Craft.class,Dispenser.class,Chest.class,Brewing.class};
        try(DataOutputStream o=Gui2Fixture10.out(a[0])) {
            for(int c=0;c<2000;c++) {
                int kind=c%5,rows=1+r.nextInt(9),width=r.nextInt(1800),height=r.nextInt(1200),cook=r.nextInt(400)-50,burn=r.nextInt(500)-50,fuel=r.nextInt(2000);
                GuiContainer g=(GuiContainer)Oracle.allocate(types[kind]);g.xSize=176;g.ySize=kind==3?114+rows*18:166;g.width=width;g.height=height;g.mc=mc;g.fontRenderer=font;
                if(kind==0){TileEntityFurnace tile=new TileEntityFurnace();tile.furnaceCookTime=cook;tile.furnaceBurnTime=burn;tile.currentItemBurnTime=fuel;Gui2Fixture10.field(GuiFurnace.class,"furnaceInventory").set(g,tile);}
                if(kind==3){Gui2Fixture10.field(GuiChest.class,"inventoryRows").setInt(g,rows);Gui2Fixture10.field(GuiChest.class,"upperChestInventory").set(g,new InventoryBasic("Inventory",36));Gui2Fixture10.field(GuiChest.class,"lowerChestInventory").set(g,new InventoryBasic(rows==3?"Chest":"Large chest",rows*9));}
                if(kind==4){Mode_gui2brewing10.Tile tile=new Mode_gui2brewing10.Tile();tile.time=cook;Gui2Fixture10.field(GuiBrewingStand.class,"field_40217_h").set(g,tile);}
                rectangles.clear();font.text.clear();font.labels.clear();g.drawGuiContainerBackgroundLayer(0.5F,c%widthSafe(width),c%widthSafe(height));g.drawGuiContainerForegroundLayer();
                Gui2Fixture10.i(o,rectangles.size());for(int[] rect:rectangles)for(int n:rect)Gui2Fixture10.i(o,n);
                Gui2Fixture10.i(o,font.text.size());for(int n=0;n<font.text.size();n++){Gui2Fixture10.s(o,font.text.get(n));for(int v:font.labels.get(n))Gui2Fixture10.i(o,v);}
            }
        }
    }
    static int widthSafe(int n){return Math.max(1,n);}
}
