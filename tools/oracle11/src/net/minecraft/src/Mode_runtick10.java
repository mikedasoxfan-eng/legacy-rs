package net.minecraft.src;
import java.io.*;
import java.util.*;
import java.lang.reflect.*;
import net.minecraft.client.Minecraft;
import org.lwjgl.input.Mouse;
import org.lwjgl.input.Keyboard;

/** Calls the unmodified original Minecraft.runTick. Only platform/render/world
 * tick callbacks are replaced; inventory, item use, clicks and controllers run
 * the original bytecode. The fixture carries its event inputs with each result. */
public final class Mode_runtick10 {
    static Field field(Class<?> c,String n)throws Exception {Field f=c.getDeclaredField(n);f.setAccessible(true);return f;}
    static final class Settings extends GameSettings {Client client;public void saveOptions(){client.actions.add(6);}}
    static final class Stats extends StatFileWriter {
        Stats(){super(null,null);}public void func_27178_d(){}public void syncStats(){}
    }
    static final class Hud extends GuiIngame {Hud(){super(null);}public void updateTick(){}}
    static final class Textures extends RenderEngine {
        Textures(){super(null,null);}public int getTexture(String s){return 0;}public void updateDynamicTextures(){}
    }
    static final class Renderer extends EntityRenderer {
        Renderer(){super(null);}public void getMouseOver(float p){}public void updateRenderer(){}
    }
    static final class Global extends RenderGlobal {Global(){super(null,null);}public void updateClouds(){}}
    static final class Effects extends EffectRenderer {
        Effects(World w){super(w,null);}public void addBlockHitEffects(int x,int y,int z,int side){WorldEventOracle.hit(worldObj,x,y,z,side);}public void updateEffects(){}
    }
    static final class Cursor extends MouseHelper {
        Cursor(){super(null);}public void grabMouseCursor(){}public void ungrabMouseCursor(){}
    }
    static final class Screen extends GuiScreen {
        int mouseCalls,keyCalls;
        public void handleInput(){super.handleInput();}public void handleMouseInput(){mouseCalls++;}
        public void handleKeyboardInput(){keyCalls++;}public void updateScreen(){}
    }
    static final class WorldProbe extends World {
        int ticks,updates,joins,display;boolean mobs,animals;
        WorldProbe(){super((ISaveHandler)null,"",new WorldSettings(0,0,true, false, EnumWorldType.DEFAULT));}
        public void tick(){ticks++;}public void updateEntities(){updates++;}
        public void joinEntityInSurroundings(Entity e){joins++;}
        public void randomDisplayUpdates(int x,int y,int z){display++;}
        public void setAllowedMobSpawns(boolean m,boolean a){mobs=m;animals=a;}
    }
    static final class Player extends EntityPlayerSP {
        int drops,swings;
        Player(Minecraft mc,World w){super(mc,w,new Session("Player",""),0);}
        public void addStat(StatBase s,int n){}
        public void dropCurrentItem(){drops++;super.dropCurrentItem();}
        public void swingItem(){swings++;super.swingItem();}
    }
    static final class Survival extends PlayerControllerSP {
        int[] clicks=new int[5];Survival(Minecraft mc){super(mc);}
        public void clickBlock(int x,int y,int z,int side){clicks[0]++;super.clickBlock(x,y,z,side);}
        public void attackEntity(EntityPlayer p,Entity e){clicks[1]++;super.attackEntity(p,e);}
        public void interactWithEntity(EntityPlayer p,Entity e){clicks[2]++;super.interactWithEntity(p,e);}
        public boolean onPlayerRightClick(EntityPlayer p,World w,ItemStack s,int x,int y,int z,int side){clicks[3]++;return super.onPlayerRightClick(p,w,s,x,y,z,side);}
        public boolean sendUseItem(EntityPlayer p,World w,ItemStack s){clicks[4]++;return super.sendUseItem(p,w,s);}
    }
    static final class Creative extends PlayerControllerCreative {
        int[] clicks=new int[5];Creative(Minecraft mc){super(mc);}
        public void clickBlock(int x,int y,int z,int side){clicks[0]++;super.clickBlock(x,y,z,side);}
        public void attackEntity(EntityPlayer p,Entity e){clicks[1]++;super.attackEntity(p,e);}
        public void interactWithEntity(EntityPlayer p,Entity e){clicks[2]++;super.interactWithEntity(p,e);}
        public boolean onPlayerRightClick(EntityPlayer p,World w,ItemStack s,int x,int y,int z,int side){clicks[3]++;return super.onPlayerRightClick(p,w,s,x,y,z,side);}
        public boolean sendUseItem(EntityPlayer p,World w,ItemStack s){clicks[4]++;return super.sendUseItem(p,w,s);}
    }
    static final class Client extends Minecraft {
        ArrayList<Integer> actions=new ArrayList<>();
        Client(){super(null,null,null,1,1,false);}
        public void displayUnexpectedThrowable(UnexpectedThrowable t){throw new RuntimeException();}
        public void displayGuiScreen(GuiScreen s){
            if(s==null){currentScreen=null;return;}
            actions.add(s instanceof GuiInventory?1:s instanceof GuiChat?2:7);
            Screen probe=new Screen();probe.allowUserInput=false;probe.guiParticles=new GuiParticle(this);
            currentScreen=probe;setIngameNotInFocus();
        }
        public void displayInGameMenu(){
            actions.add(3);Screen probe=new Screen();probe.guiParticles=new GuiParticle(this);currentScreen=probe;setIngameNotInFocus();
        }
        public void setIngameFocus(){if(!inGameHasFocus)actions.add(0);super.setIngameFocus();}
        public void toggleFullscreen(){actions.add(4);}
    }
    static void i(DataOutputStream o,int n)throws Exception {EntityOracle.i(o,n);}
    static WorldProbe world()throws Exception {
        World source=Mode_entities10.world();WorldProbe w=(WorldProbe)Oracle.allocate(WorldProbe.class);
        for(Field f:World.class.getDeclaredFields())if(!Modifier.isStatic(f.getModifiers())){f.setAccessible(true);f.set(w,f.get(source));}
        return w;
    }
    public static void main(String[] a)throws Exception {
        try(DataOutputStream out=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            i(out,26); // 2 controllers x 13 independent scenarios
            for(int mode=0;mode<2;mode++)for(int scenario=0;scenario<13;scenario++) {
                Mouse.reset();Keyboard.reset();WorldProbe w=world();WorldEventOracle.attach(w);Item.itemRand.setSeed(605);
                Client mc=(Client)Oracle.allocate(Client.class);mc.actions=new ArrayList<>();mc.theWorld=w;
                Player p=new Player(mc,w);mc.thePlayer=p;p.setLocationAndAngles(.5,64,.5,0,0);p.onGround=true;
                mc.gameSettings=new Settings();((Settings)mc.gameSettings).client=mc;KeyBinding.unPressAllKeys();
                mc.gameSettings.noclip=scenario%2==0;
                mc.statFileWriter=(Stats)Oracle.allocate(Stats.class);mc.ingameGUI=(Hud)Oracle.allocate(Hud.class);
                mc.renderEngine=(Textures)Oracle.allocate(Textures.class);mc.entityRenderer=(Renderer)Oracle.allocate(Renderer.class);
                mc.entityRenderer.itemRenderer=(ItemRenderer)Oracle.allocate(ItemRenderer.class);mc.renderGlobal=(Global)Oracle.allocate(Global.class);
                mc.effectRenderer=new Effects(w);mc.mouseHelper=(Cursor)Oracle.allocate(Cursor.class);
                mc.sndManager=(WorldEventOracle.Sounds)Oracle.allocate(WorldEventOracle.Sounds.class);((WorldEventOracle.Sounds)mc.sndManager).world=w;
                mc.inGameHasFocus=scenario!=4;
                mc.playerController=mode==0?new Survival(mc):new Creative(mc);
                if(mode==1)PlayerControllerCreative.enableAbilities(p);
                p.inventory.mainInventory=new ItemStack[36];p.inventory.armorInventory=new ItemStack[4];
                for(int slot=0;slot<9;slot++)p.inventory.mainInventory[slot]=new ItemStack(scenario==1?267:scenario==9?260:scenario==10?261:278,8,0);
                p.inventory.mainInventory[8]=new ItemStack(262,64,0);p.getFoodStats().setFoodLevel(10);
                if(scenario==6){mc.gameSettings.keyBindAttack.keyCode=33;mc.gameSettings.keyBindUseItem.keyCode=2;mc.gameSettings.keyBindPickBlock.keyCode=16;KeyBinding.resetKeyBindingArrayAndHash();}
                i(out,mode);i(out,scenario);i(out,80);
                for(int t=0;t<80;t++) {
                    int phase=t%20;
                    boolean close=phase==12&&(scenario==5||scenario==7||scenario==11);
                    if(close){mc.currentScreen=null;mc.setIngameFocus();mc.actions.clear();}
                    if(scenario==7&&phase==0){Screen s=new Screen();s.allowUserInput=true;s.guiParticles=new GuiParticle(mc);mc.currentScreen=s;mc.setIngameNotInFocus();}
                    boolean fresh=scenario!=4||phase>=4;
                    boolean paused=t%3!=0;field(Minecraft.class,"isGamePaused").setBoolean(mc,paused);
                    w.multiplayerWorld=scenario==8;mc.gameSettings.difficulty=t%4;w.lightningFlash=phase%5;
                    boolean target=phase<16;mc.objectMouseOver=target?new MovingObjectPosition(4,64,4,1,Vec3D.createVector(4,65,4)):null;
                    EntityOracle.rawBlock(w,4,64,4,scenario==12?58:1,0);EntityOracle.rawBlock(w,4,65,4,0,0);
                    ArrayList<int[]> mouse=new ArrayList<>(),keys=new ArrayList<>();
                    if(phase==0)mouse.add(new int[]{0,1,0});if(phase==3)mouse.add(new int[]{0,1,0});if(phase==6)mouse.add(new int[]{0,0,0});
                    if(phase==2||phase==4)mouse.add(new int[]{1,1,0});if(phase==9)mouse.add(new int[]{1,0,0});
                    if(phase==5)mouse.add(new int[]{2,1,0});if(phase==6)mouse.add(new int[]{2,0,0});
                    if(scenario==2||scenario==4){mouse.add(new int[]{-1,0,phase%2==0?120:-240});mouse.add(new int[]{-1,0,120});if(phase%3==0)keys.add(new int[]{2+t%9,1});}
                    if(scenario==3){if(phase==0)keys.add(new int[]{61,1});if(phase==3)keys.add(new int[]{42,1});if(phase==4)keys.add(new int[]{54,1});if(phase==5)keys.add(new int[]{42,0});if(phase==7)keys.add(new int[]{54,0});if(phase<10)keys.add(new int[]{33,1});if(phase==10)keys.add(new int[]{61,0});if(phase%3==0)keys.add(new int[]{59,1});if(phase%4==0)keys.add(new int[]{63,1});if(phase==15)keys.add(new int[]{66,1});if(phase==16)keys.add(new int[]{87,1});}
                    if(scenario==5&&phase==7){keys.add(new int[]{16,1});keys.add(new int[]{18,1});keys.add(new int[]{18,1});keys.add(new int[]{20,1});}
                    if(scenario==6){keys.add(new int[]{phase%2==0?33:2,phase%3==0||mode==1&&phase==4?0:1});if(phase==4)keys.add(new int[]{16,1});} // creative: no attack press with the pick press (1.0 picks the broken air block: Item.itemsList[0] NPE)
                    if(scenario==8&&phase==7)keys.add(new int[]{20,1});
                    if(scenario==11&&phase==7){keys.add(new int[]{1,1});keys.add(new int[]{5,1});}
                    if(scenario==0&&phase==8){keys.add(new int[]{16,1});keys.add(new int[]{16,1});keys.add(new int[]{16,0});}
                    i(out,Trace10.namedInt("close?1:0", close?1:0));i(out,Trace10.namedInt("scenario==7&&phase==0?1:0", scenario==7&&phase==0?1:0));Trace10.writeBoolean(out, "fresh", fresh);Trace10.writeBoolean(out, "paused", paused);Trace10.writeBoolean(out, "target", target);i(out,Trace10.namedInt("mc.gameSettings.difficulty", mc.gameSettings.difficulty));i(out,Trace10.namedInt("w.lightningFlash", w.lightningFlash));
                    i(out,Trace10.namedInt("mouse.size()", mouse.size()));for(int[] event:mouse){for(int n:event)i(out,n);Mouse.queue(event[0],event[1]!=0,event[2]);}
                    i(out,Trace10.namedInt("keys.size()", keys.size()));for(int[] event:keys){for(int n:event)i(out,n);Keyboard.queue(event[0],event[1]!=0);}
                    ArrayList<Integer> down=new ArrayList<>();for(int k=0;k<256;k++)if(Keyboard.isKeyDown(k))down.add(k);i(out,Trace10.namedInt("down.size()", down.size()));for(int k:down)i(out,k);
                    field(Minecraft.class,"systemTime").setLong(mc,System.currentTimeMillis()-(fresh?0:1000));
                    mc.runTick();
                    ByteArrayOutputStream bytes=new ByteArrayOutputStream();DataOutputStream o=new DataOutputStream(bytes);
                    i(o,Trace10.namedInt("mc.currentScreen==null?0:1", mc.currentScreen==null?0:1));Trace10.writeBoolean(o, "mc.inGameHasFocus", mc.inGameHasFocus);i(o,Trace10.namedInt("field(Minecraft.class,\"leftClickCounter\").getInt(mc)", field(Minecraft.class,"leftClickCounter").getInt(mc)));i(o,Trace10.namedInt("field(Minecraft.class,\"field_35001_ab\").getInt(mc)", field(Minecraft.class,"rightClickDelayTimer").getInt(mc)));
                    i(o,Trace10.namedInt("p.inventory.currentItem", p.inventory.currentItem));i(o,Trace10.namedInt("p.drops", p.drops));i(o,Trace10.namedInt("p.swingProgressInt", p.swingProgressInt));Trace10.writeBoolean(o, "p.isSwinging", p.isSwinging);Trace10.writeBoolean(o, "p.isUsingItem()", p.isUsingItem());i(o,Trace10.namedInt("p.func_35205_Y()", p.func_35205_Y()));
                    for(ItemStack stack:p.inventory.mainInventory)PlayerOracle.stack(o,stack);
                    int[] clicks=mode==0?((Survival)mc.playerController).clicks:((Creative)mc.playerController).clicks;
                    for(int n:clicks)i(o,n);i(o,Trace10.namedInt("p.swings", p.swings));
                    for(KeyBinding k:mc.gameSettings.keyBindings){Trace10.writeBoolean(o, "k.pressed", k.pressed);i(o,Trace10.namedInt("k.pressTime", k.pressTime));}
                    Trace10.writeBoolean(o, "mc.gameSettings.hideGUI", mc.gameSettings.hideGUI);Trace10.writeBoolean(o, "mc.gameSettings.showDebugInfo", mc.gameSettings.showDebugInfo);Trace10.writeBoolean(o, "mc.gameSettings.thirdPersonView!=0", mc.gameSettings.thirdPersonView!=0);i(o,Trace10.namedInt("mc.gameSettings.renderDistance", mc.gameSettings.renderDistance));EntityOracle.f(o,Trace10.namedFloat("mc.gameSettings.noclipRate", mc.gameSettings.noclipRate));
                    i(o,Trace10.namedInt("mc.actions.size()", mc.actions.size()));for(int action:mc.actions)i(o,action);mc.actions.clear();
                    i(o,Trace10.namedInt("w.difficultySetting", w.difficultySetting));i(o,Trace10.namedInt("w.lightningFlash", w.lightningFlash));i(o,Trace10.namedInt("w.ticks", w.ticks));i(o,Trace10.namedInt("w.updates", w.updates));i(o,Trace10.namedInt("w.joins", w.joins));i(o,Trace10.namedInt("w.display", w.display));Trace10.writeBoolean(o, "w.mobs", w.mobs);Trace10.writeBoolean(o, "w.animals", w.animals);
                    if(mode==0){PlayerControllerSP c=(PlayerControllerSP)mc.playerController;for(String n:new String[]{"curBlockDamage","prevBlockDamage","blockDestroySoundCounter"})EntityOracle.f(o,Trace10.namedFloat("field(PlayerControllerSP.class,n).getFloat(c)", field(PlayerControllerSP.class,n).getFloat(c)));for(String n:new String[]{"curBlockX","curBlockY","curBlockZ","blockHitWait"})i(o,Trace10.namedInt("field(PlayerControllerSP.class,n).getInt(c)", field(PlayerControllerSP.class,n).getInt(c)));}
                    else i(o,Trace10.namedInt("field(PlayerControllerCreative.class,\"field_35647_c\").getInt(mc.playerController)", field(PlayerControllerCreative.class,"field_35647_c").getInt(mc.playerController)));
                    i(out,Trace10.namedInt("bytes.size()", bytes.size()));Trace10.writeBytes(out, "bytes.toByteArray()", bytes.toByteArray());
                }
            }
        }
    }
}
