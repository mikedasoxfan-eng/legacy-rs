package net.minecraft.src;
import java.io.*;import java.util.*;import net.minecraft.client.Minecraft;
public final class Mode_gui3hud10 {
 static class Player extends EntityPlayerSP {
  int armor,air;boolean water,poison,hunger,regen;FoodStats food;
  Player(){super(null,null,null,0);}
  public int getPlayerArmorValue(){return armor;}
  public int func_41001_Z(){return air;}
  public boolean isInsideOfMaterial(Material m){return water;}
  public boolean isPotionActive(Potion p){return p==Potion.potionPoison?poison:p==Potion.potionHunger?hunger:p==Potion.potionRegeneration?regen:false;}
  public FoodStats getFoodStats(){return food;}
 }
 static class Controller extends PlayerControllerSP {
  boolean survival;Controller(){super(null);}public boolean shouldDrawHUD(){return survival;}public boolean func_35642_f(){return survival;}
 }
 static class Renderer extends EntityRenderer {Renderer(){super(null);}public void setupOverlayRendering(){}}
 static class Screen extends GuiIngame {
  Screen(Minecraft m){super(m);}
  public void drawTexturedModalRect(int x,int y,int u,int v,int w,int h){Gui3Draw10.rect(0,new int[]{x,y,u,v,w,h},zLevel);}
 }
 public static void main(String[] a)throws Exception {
  Block.stone.getClass();Minecraft mc=(Minecraft)Oracle.allocate(PlayerOracle.HeadlessMinecraft.class);mc.gameSettings=new GameSettings();mc.gameSettings.guiScale=1;mc.gameSettings.fancyGraphics=false;mc.fontRenderer=new Gui3Draw10.Font();mc.renderEngine=(Gui2Fixture10.Engine)Oracle.allocate(Gui2Fixture10.Engine.class);mc.entityRenderer=(Renderer)Oracle.allocate(Renderer.class);Controller controller=(Controller)Oracle.allocate(Controller.class);mc.playerController=controller;
  World world=(World)Oracle.allocate(World.class);mc.theWorld=world;Player p=(Player)Oracle.allocate(Player.class);mc.thePlayer=p;p.worldObj=world;p.inventory=new InventoryPlayer(p);p.food=new FoodStats();Screen g=new Screen(mc);Random input=new Random(30038);
  EntityDragon dragon=(EntityDragon)Oracle.allocate(EntityDragon.class);dragon.field_40157_aB=200;dragon.dataWatcher=new DataWatcher();dragon.dataWatcher.addObject(16,Integer.valueOf(200));
  try(DataOutputStream o=Gui2Fixture10.out(a[0])){for(int n=0;n<1000;n++){
   mc.displayWidth=320+input.nextInt(1000);mc.displayHeight=240+input.nextInt(600);Gui2Fixture10.field(GuiIngame.class,"updateCounter").setInt(g,input.nextInt());p.health=input.nextInt(25)-2;p.prevHealth=input.nextInt(25)-2;p.heartsLife=input.nextInt(40);p.armor=input.nextInt(23);p.food.setFoodLevel(input.nextInt(23));p.food.setFoodSaturationLevel(input.nextBoolean()?0:5);p.playerLevel=input.nextInt(60)-5;p.currentXP=input.nextFloat()*1.5F-0.2F;p.inventory.currentItem=input.nextInt(9);p.air=input.nextInt(340)-20;p.water=input.nextBoolean();p.poison=input.nextBoolean();p.hunger=input.nextBoolean();p.regen=input.nextBoolean();controller.survival=n%4!=0;Gui2Fixture10.field(World.class,"worldInfo").set(world,new WorldInfo(new WorldSettings(42,0,true,n%2==0),"fixture"));
   int health=input.nextInt(251)-25;dragon.dataWatcher.updateObject(16,Integer.valueOf(health));RenderDragon.field_41038_a=n%3==0?dragon:null;
   Gui3Draw10.begin();g.renderGameOverlay(0.5F,false,0,0);Gui3Draw10.write(o);Random rng=(Random)Gui2Fixture10.field(GuiIngame.class,"rand").get(g);Gui2Fixture10.l(o,rng.nextLong());o.writeBoolean(RenderDragon.field_41038_a==null);
  }}
 }
}
