package net.minecraft.src;

import java.awt.BorderLayout;
import java.awt.Canvas;
import java.awt.Dimension;
import java.awt.Frame;
import java.awt.image.BufferedImage;
import java.io.BufferedReader;
import java.io.File;
import java.io.InputStreamReader;
import java.io.FileInputStream;
import java.io.DataOutputStream;
import java.io.BufferedOutputStream;
import java.io.FileOutputStream;
import java.lang.reflect.Field;
import java.lang.reflect.Method;
import java.nio.ByteBuffer;
import java.util.ArrayList;
import java.util.List;
import javax.imageio.ImageIO;
import net.minecraft.client.Minecraft;
import org.lwjgl.BufferUtils;
import org.lwjgl.input.Keyboard;
import org.lwjgl.input.Mouse;
import org.lwjgl.opengl.GL11;

/**
 * Runs the original b1.7.3 client (real window, real OpenGL) through a scene script and
 * saves screenshots, for pixel comparison with the Rust port's `beta --guishot`.
 * See crates/client/src/gui/shot.rs for the script format.
 *
 *   GuiShot <scenes.txt> <outdir> <minecraft data dir>
 *
 * The data dir replaces .minecraft (options, saves list), so the player's real files are
 * never touched. GuiMainMenu is recompiled with its clock replaced by GuiShot.millis.
 */
public class GuiShot {
    static TileEntityChest chestFixture(World world) { TileEntityChest chest=new TileEntityChest(); chest.worldObj=world; return chest; }

    public static long millis = 0L;

    static List<String> script = new ArrayList<>();
    static int pc = 0;
    static File out;
    static File scriptDir;

    /** Minecraft with the screen it shows always wrapped by the capturing screen. */
    static class ShotMinecraft extends Minecraft {
        Capture capture;

        ShotMinecraft(Frame f, Canvas c) {
            super(f, c, null, 854, 480, false);
        }

        public void displayUnexpectedThrowable(UnexpectedThrowable t) {
            System.err.println("crashed:");
            System.err.println(t.description);
            t.exception.printStackTrace();
            Runtime.getRuntime().halt(3);
        }

        public void displayGuiScreen(GuiScreen s) {
            super.displayGuiScreen(s);
            if (this.currentScreen == null && capture != null) {
                // Keep running the script after a world starts (it displays no screen).
                this.currentScreen = capture;
                return;
            }
            if (this.currentScreen != null && this.currentScreen != capture) {
                if (capture == null) {
                    // Started: from now on there is no session, so Multiplayer is disabled
                    // (as in the singleplayer-only port).
                    this.session = null;
                    capture = new Capture();
                    capture.setWorldAndResolution(this, this.currentScreen.width, this.currentScreen.height);
                }
                capture.inner = this.currentScreen;
                this.currentScreen = capture;
            }
        }
    }

    /** Runs the script one `shot` per frame, drawing the wrapped screen. */
    static class Capture extends GuiScreen {
        GuiScreen inner;
        int mx = -1, my = -1;
        boolean down = false;
        String splash = "missingno";
        String mode = "screen";
        String loadingTitle = "", loadingText = "";
        int loadingProgress = -1;
        TileEntityFurnace furnace;

        public void onGuiClosed() {
            if(inner!=null)inner.onGuiClosed();
        }

        public void handleInput() {
            while (Mouse.next()) {
            }
            while (Keyboard.next()) {
            }
        }

        public void drawScreen(int ignoredX, int ignoredY, float partial) {
            try {
                step();
            } catch (Exception e) {
                e.printStackTrace();
                Runtime.getRuntime().halt(2);
            }
        }

        void setMouseDown(boolean b) throws Exception {
            Field f = Mouse.class.getDeclaredField("buttons");
            f.setAccessible(true);
            ByteBuffer buf = (ByteBuffer) f.get(null);
            buf.put(0, (byte) (b ? 1 : 0));
        }

        boolean reported = false;

        void step() throws Exception {
            if (!reported) {
                reported = true;
                System.out.println("java: GL " + GL11.glGetString(GL11.GL_RENDERER) + ", sample buffers "
                    + GL11.glGetInteger(org.lwjgl.opengl.GL13.GL_SAMPLE_BUFFERS) + ", samples "
                    + GL11.glGetInteger(org.lwjgl.opengl.GL13.GL_SAMPLES));
            }
            while (pc < script.size()) {
                String line = script.get(pc++);
                int hash = line.indexOf('#');
                if (hash >= 0) line = line.substring(0, hash);
                line = line.trim();
                if (line.isEmpty()) continue;
                String cmd = line.contains(" ") ? line.substring(0, line.indexOf(' ')) : line;
                String rest = line.contains(" ") ? line.substring(line.indexOf(' ') + 1) : "";
                String[] a = rest.isEmpty() ? new String[0] : rest.trim().split("\\s+");
                if (cmd.equals("saves")) {
                    // The client lists <data dir>/saves; the script's folder is copied there.
                } else if (cmd.equals("packs")) {
                    mc.texturePackList = new TexturePackList(mc, new File(scriptDir, rest));
                    Field packs = RenderEngine.class.getDeclaredField("texturePack");
                    packs.setAccessible(true); packs.set(mc.renderEngine, mc.texturePackList);
                } else if (cmd.equals("pack")) {
                    mc.texturePackList.updateAvaliableTexturePacks();
                    boolean found = false;
                    for (Object obj : mc.texturePackList.availableTexturePacks()) {
                        TexturePackBase pack = (TexturePackBase)obj;
                        if (pack.texturePackFileName.equals(rest)) { mc.texturePackList.setTexturePack(pack); found = true; break; }
                    }
                    if (!found) throw new IllegalArgumentException("pack missing " + rest);
                    mc.renderEngine.refreshTextures();
                } else if (cmd.equals("repeatstate")) {
                    if(Keyboard.areRepeatEventsEnabled()!=rest.equals("on"))throw new AssertionError("keyboard repeat state differs");
                } else if (cmd.equals("fontstart")) {
                    mc.gameSettings.anaglyph=rest.equals("on");
                    mc.fontRenderer=new FontRenderer(mc.gameSettings,"/font/default.png",mc.renderEngine);
                    mc.renderEngine.refreshTextures();
                } else if (cmd.equals("message")) {
                    mc.ingameGUI.addChatMessage(rest);
                } else if (cmd.equals("anaglyph")) {
                    mc.gameSettings.anaglyph = rest.equals("on");
                    mc.renderEngine.refreshTextures();
                } else if (cmd.equals("texturecheck")) {
                    textureCheck(rest);
                } else if (cmd.equals("sessionless")) {
                    mc.session = null;
                } else if (cmd.equals("millis")) {
                    millis = Long.parseLong(rest.trim());
                } else if (cmd.equals("splash")) {
                    splash = rest;
                } else if (cmd.equals("mojang")) {
                    mode = "mojang";
                } else if (cmd.equals("loading")) {
                    mode = "loading";
                    loadingProgress = Integer.parseInt(a[0]);
                    String t = rest.substring(rest.indexOf(' ') + 1);
                    int bar = t.indexOf('|');
                    loadingTitle = t.substring(0, bar);
                    loadingText = t.substring(bar + 1);
                } else if (cmd.equals("open")) {
                    mode = "screen";
                    GuiMainMenu title = new GuiMainMenu();
                    setSplash(title);
                    GuiScreen s;
                    String which = a[0];
                    if (which.equals("chat")) s = new GuiChat();
                    else if (which.equals("title")) s = title;
                    else if (which.equals("options")) s = new GuiOptions(title, mc.gameSettings);
                    else if (which.equals("video")) s = new GuiVideoSettings(title, mc.gameSettings);
                    else if (which.equals("controls")) s = new GuiControls(title, mc.gameSettings);
                    else if (which.equals("select")) s = new GuiSelectWorld(title);
                    else if (which.equals("create")) s = new GuiCreateWorld(title);
                    else if (which.equals("rename")) s = new GuiRenameWorld(title, a[1]);
                    else if (which.equals("texturepacks")) s = new GuiTexturePacks(title);
                    else if (which.equals("achievements")) s = new GuiAchievements(mc.statFileWriter);
                    else if (which.equals("gameover")) s = new GuiGameOver();
                    else if (which.equals("stats")) s = new GuiStats(title, mc.statFileWriter);
                    else if (which.equals("sign")) {
                        TileEntitySign sign=new TileEntitySign();sign.worldObj=mc.theWorld;sign.xCoord=0;sign.yCoord=64;sign.zCoord=0;
                        mc.theWorld.setBlockAndMetadata(0,64,0,Integer.parseInt(a[1]),Integer.parseInt(a[2]));
                        TileEntityRenderer.instance.cacheActiveRenderInfo(mc.theWorld,mc.renderEngine,mc.fontRenderer,mc.thePlayer,0);
                        s=new GuiEditSign(sign);
                    }
                    else if (which.equals("creative") || which.equals("inventory") || which.equals("crafting") || which.equals("chest") || which.equals("largechest") || which.equals("furnace") || which.equals("dispenser")) {
                        EntityPlayerSP p = mc.thePlayer;
                        p.inventory = new InventoryPlayer(p); p.inventorySlots = new ContainerPlayer(p.inventory);
                        p.ticksExisted=0;p.field_703_S=p.field_704_R=p.field_705_Q=0;p.swingProgress=p.prevSwingProgress=0;
                        p.deathTime=p.hurtTime=0;p.movementInput.sneak=false;
                        RenderManager.instance.cacheActiveRenderInfo(mc.theWorld,mc.renderEngine,mc.fontRenderer,p,mc.gameSettings,1.0F);
                        if(which.equals("creative")){mc.playerController=new PlayerControllerTest(mc);PlayerControllerTest.func_35646_d(p);s=new GuiContainerCreative(p);}
                        else if(which.equals("inventory")){mc.playerController=new PlayerControllerSP(mc);s=new GuiInventory(p);}
                        else if(which.equals("crafting"))s=new GuiCrafting(p.inventory,mc.theWorld,0,64,0);
                        else if(which.equals("chest"))s=new GuiChest(p.inventory,chestFixture(mc.theWorld));
                        else if(which.equals("largechest"))s=new GuiChest(p.inventory,new InventoryLargeChest("Large chest",chestFixture(mc.theWorld),chestFixture(mc.theWorld)));
                        else if(which.equals("furnace")){furnace=new TileEntityFurnace();s=new GuiFurnace(p.inventory,furnace);}
                        else s=new GuiDispenser(p.inventory,new TileEntityDispenser());
                    }
                    else throw new IllegalArgumentException("unknown screen " + which);
                    mc.displayGuiScreen(s);
                } else if (cmd.equals("world")) {
                    mc.session = new Session("Player", "-"); // EntityPlayerSP reads the username
                    mc.playerController = new PlayerControllerSP(mc);
                    mc.startWorld("guishot_world", "guishot", new WorldSettings(Long.parseLong(a[0]),0,false));
                    mc.currentScreen = this;
                    inner = null;
                } else if (cmd.equals("pose")) {
                    EntityPlayerSP p=mc.thePlayer;p.ticksExisted=(int)Float.parseFloat(a[0])-1;
                    p.field_703_S=Float.parseFloat(a[1]);p.field_704_R=p.field_705_Q=Float.parseFloat(a[2]);
                    p.swingProgress=p.prevSwingProgress=Float.parseFloat(a[3]);
                    p.ridingEntity=a[4].equals("on")?new EntityPig(mc.theWorld):null;p.movementInput.sneak=a[5].equals("on");
                    p.hurtTime=a.length>6?Integer.parseInt(a[6]):0;
                } else if (cmd.equals("hud")) {
                    mode = "hud";
                    Boolean water=null;
                    EntityPlayerSP p = mc.thePlayer;
                    for (String kv : a) {
                        String[] e = kv.split("=");
                        int v = e.length > 1 && !e[0].equals("pos") && !e[0].equals("brightness") && !e[0].equals("saturation") ? Integer.parseInt(e[1]) : 0;
                        if (e[0].equals("health")) p.health = v;
                        else if (e[0].equals("food")) p.func_35191_at().func_35764_a(v);
                        else if (e[0].equals("saturation")) p.func_35191_at().func_35767_b(Float.parseFloat(e[1]));
                        else if (e[0].equals("xp")) p.field_35211_aX=v;
                        else if (e[0].equals("level")) p.field_35210_aY=v;
                        else if (e[0].equals("air")) p.air=v;
                        else if (e[0].equals("water")) water=v!=0;
                        else if (e[0].equals("creative")) mc.playerController=v!=0?new PlayerControllerTest(mc):new PlayerControllerSP(mc);
                        else if (e[0].equals("armor")) { for(int slot=0;slot<4;slot++)p.inventory.armorInventory[slot]=v==20?new ItemStack(313-slot,1,0):null; }
                        else if (e[0].equals("seed")) { }
                        else if (e[0].equals("poison") || e[0].equals("hunger") || e[0].equals("regen")) {
                            int effect=e[0].equals("poison")?19:e[0].equals("hunger")?17:10;
                            if(v!=0)p.func_35165_a(new PotionEffect(effect,10000,0));
                        }
                        else if (e[0].equals("prev")) p.prevHealth = v;
                        else if (e[0].equals("hearts_life")) p.heartsLife = v;
                        else if (e[0].equals("item")) p.inventory.currentItem = v;
                        else if (e[0].equals("helmet")) p.inventory.armorInventory[3] = v < 0 ? null : new ItemStack(v, 1, 0);
                        else if (e[0].equals("pos")) {
                            String[] c = e[1].split(",");
                            p.setPosition(Double.parseDouble(c[0]), Double.parseDouble(c[1]), Double.parseDouble(c[2]));
                        }
                    }
                    if(water!=null){int waterY=MathHelper.floor_double(p.posY+p.getEyeHeight());
                    mc.theWorld.setBlockWithNotify(MathHelper.floor_double(p.posX),waterY,MathHelper.floor_double(p.posZ),water?Block.waterStill.blockID:0);}
                    System.out.println("java: player brightness " + p.getEntityBrightness(0.0F));
                } else if(cmd.equals("stack")) {
                    int id=Integer.parseInt(a[1]);
                    ((GuiContainer)inner).inventorySlots.putStackInSlot(Integer.parseInt(a[0]),id<0?null:new ItemStack(id,Integer.parseInt(a[2]),Integer.parseInt(a[3])));
                } else if(cmd.equals("cursor")) {
                    int id=Integer.parseInt(a[0]);mc.thePlayer.inventory.setItemStack(id<0?null:new ItemStack(id,Integer.parseInt(a[1]),Integer.parseInt(a[2])));
                } else if(cmd.equals("furnace")) {
                    furnace.furnaceCookTime=Integer.parseInt(a[0]);furnace.furnaceBurnTime=Integer.parseInt(a[1]);furnace.currentItemBurnTime=Integer.parseInt(a[2]);
                } else if (cmd.equals("statsreset")) {
                    for (String fn : new String[] {"field_25102_a", "field_25101_b"}) {
                        Field f = StatFileWriter.class.getDeclaredField(fn);
                        f.setAccessible(true);
                        ((java.util.Map) f.get(mc.statFileWriter)).clear();
                    }
                } else if (cmd.equals("stat")) {
                    mc.statFileWriter.readStat(StatList.func_27361_a(Integer.parseInt(a[0])), Integer.parseInt(a[1]));
                } else if (cmd.equals("clear")) {
                    clear = Integer.parseInt(a[0]) / 255.0F;
                } else if (cmd.equals("normalprobe")) {
                    mode="normalprobe";
                } else if (cmd.equals("white")) {
                    GL11.glBindTexture(GL11.GL_TEXTURE_2D,mc.renderEngine.getTexture("/"+a[0]));
                    ByteBuffer pixels=BufferUtils.createByteBuffer(256*256*4);while(pixels.hasRemaining())pixels.put((byte)255);pixels.flip();
                    GL11.glTexImage2D(GL11.GL_TEXTURE_2D,0,GL11.GL_RGBA,256,256,0,GL11.GL_RGBA,GL11.GL_UNSIGNED_BYTE,pixels);
                } else if (cmd.equals("counter")) {
                    Field f = GuiIngame.class.getDeclaredField("updateCounter");
                    f.setAccessible(true);
                    f.setInt(mc.ingameGUI, Integer.parseInt(a[0]));
                } else if (cmd.equals("mouse")) {
                    mx = Integer.parseInt(a[0]);
                    my = Integer.parseInt(a[1]);
                } else if (cmd.equals("down")) {
                    down = true;
                } else if (cmd.equals("up")) {
                    down = false;
                } else if (cmd.equals("click")) {
                    call(inner, "mouseClicked", mx, my, a.length==0?0:Integer.parseInt(a[0]));
                } else if (cmd.equals("wheel")) {
                    GuiContainerCreative g=(GuiContainerCreative)inner;Field f=GuiContainerCreative.class.getDeclaredField("field_35312_g");f.setAccessible(true);
                    int rows=((ContainerCreative)g.inventorySlots).field_35375_a.size()/8-8+1;
                    float position=(float)((double)f.getFloat(g)-(double)Integer.signum(Integer.parseInt(rest))/(double)rows);
                    position=Math.max(0,Math.min(1,position));f.setFloat(g,position);((ContainerCreative)g.inventorySlots).func_35374_a(position);
                } else if (cmd.equals("shift")) {
                    Field keys=Keyboard.class.getDeclaredField("keyDownBuffer");keys.setAccessible(true);
                    ((ByteBuffer)keys.get(null)).put(42,(byte)(rest.equals("on")?1:0));
                } else if (cmd.equals("release")) {
                    call(inner, "mouseMovedOrUp", mx, my, 0);
                } else if (cmd.equals("key")) {
                    Method m = GuiScreen.class.getDeclaredMethod("keyTyped", char.class, int.class);
                    m.setAccessible(true);
                    m.invoke(inner, (char) Integer.parseInt(a[1]), Integer.parseInt(a[0]));
                } else if (cmd.equals("update")) {
                    int n = a.length > 0 ? Integer.parseInt(a[0]) : 1;
                    for (int i = 0; i < n; i++) inner.updateScreen();
                } else if (cmd.equals("shot")) {
                    shot(rest.trim());
                    return;
                } else {
                    throw new IllegalArgumentException("unknown script command " + cmd);
                }
            }
            System.out.println("java: done");
            Runtime.getRuntime().halt(0);
        }

        void textureCheck(String name) throws Exception {
            boolean old = mc.gameSettings.anaglyph;
            mc.gameSettings.anaglyph = true;
            BufferedImage image = new BufferedImage(256,256,BufferedImage.TYPE_INT_ARGB);
            int tex = mc.renderEngine.allocateAndSetupTexture(image);
            ByteBuffer pixels = BufferUtils.createByteBuffer(256*256*4);
            byte[] bytes = new byte[256*256*4];
            DataOutputStream data = new DataOutputStream(new BufferedOutputStream(new FileOutputStream(new File(out,name))));
            data.writeInt(0x54504658);
            for(int r=0;r<256;r++) {
                for(int g=0;g<256;g++)for(int b=0;b<256;b++) image.setRGB(b,g,((r^g^b)<<24)|(r<<16)|(g<<8)|b);
                mc.renderEngine.setupTexture(image,tex);
                GL11.glBindTexture(GL11.GL_TEXTURE_2D,tex);
                pixels.clear(); GL11.glGetTexImage(GL11.GL_TEXTURE_2D,0,GL11.GL_RGBA,GL11.GL_UNSIGNED_BYTE,pixels);
                pixels.get(bytes); data.write(bytes);
            }
            mc.renderEngine.deleteTexture(tex);
            // The two deterministic item animations: real constructors and onTick bytecode.
            World world = mc.theWorld; EntityPlayerSP player = mc.thePlayer;
            mc.theWorld=null;mc.thePlayer=null;
            TextureCompassFX compass=new TextureCompassFX(mc);TextureWatchFX watch=new TextureWatchFX(mc);
            compass.onTick();watch.onTick();
            mc.theWorld=world;mc.thePlayer=player;
            ChunkCoordinates spawn=world.getSpawnPoint();
            data.writeInt(spawn.posX);data.writeInt(spawn.posY);data.writeInt(spawn.posZ);
            Field randomField=Class.forName("java.lang.Math$RandomNumberGeneratorHolder").getDeclaredField("randomNumberGenerator");
            randomField.setAccessible(true);((java.util.Random)randomField.get(null)).setSeed(888L);
            for(int i=0;i<1000;i++) {
                boolean ana=i%3==0;
                double x=i*1.125-600.0,z=(i%79)*3.75-150.0;
                float yaw=(float)(i*37.125-720.0);
                world.setWorldTime(i*93L);player.posX=x;player.posZ=z;player.rotationYaw=yaw;
                data.writeDouble(x);data.writeDouble(z);data.writeFloat(yaw);data.writeFloat(world.getCelestialAngle(1.0F));data.writeBoolean(ana);int mode=i%5;data.writeInt(mode);
                world.worldProvider.isNether=mode==1;
                mc.theWorld=mode==2||mode==4?null:world;mc.thePlayer=mode==3||mode==4?null:player;
                compass.anaglyphEnabled=watch.anaglyphEnabled=ana;compass.onTick();watch.onTick();
                data.write(compass.imageData);data.write(watch.imageData);
            }
            mc.theWorld=world;mc.thePlayer=player;world.worldProvider.isNether=false;
            ((java.util.Random)randomField.get(null)).setSeed(12345L);
            TextureFX[] effects={new TextureLavaFX(),new TextureWaterFX(),new TexturePortalFX(),new TextureWaterFlowFX(),new TextureLavaFlowFX(),new TextureFlamesFX(0),new TextureFlamesFX(1)};
            for(int tick=0;tick<400;tick++)for(TextureFX effect:effects) {
                effect.anaglyphEnabled=tick%3==0;effect.onTick();data.write(effect.imageData);
            }
            // Actual GL color masking, masked clears and destination-alpha blending.
            bindFbo();GL11.glViewport(0,0,mc.displayWidth,mc.displayHeight);
            GL11.glMatrixMode(GL11.GL_PROJECTION);GL11.glLoadIdentity();GL11.glMatrixMode(GL11.GL_MODELVIEW);GL11.glLoadIdentity();
            GL11.glDisable(GL11.GL_TEXTURE_2D);GL11.glDisable(GL11.GL_LIGHTING);GL11.glDisable(GL11.GL_FOG);GL11.glDisable(GL11.GL_DEPTH_TEST);GL11.glDisable(GL11.GL_CULL_FACE);
            ByteBuffer pixel=BufferUtils.createByteBuffer(4);byte[] rgba=new byte[4];
            data.writeInt(80);
            for(int c=0;c<80;c++) {
                int mask=c%5;float alpha=(c/5)%2,sourceAlpha=(c/10)%2==0?0.5F:1.0F;
                float r=(c/20)%2,g=(c/40)%2,b=1.0F-r;
                GL11.glColorMask(true,true,true,true);GL11.glClearColor(0,1,1,alpha);GL11.glClear(GL11.GL_COLOR_BUFFER_BIT);
                GL11.glColorMask(mask==0||mask==2||mask==4,mask==0||mask==1||mask==3,mask==0||mask==1||mask==3,mask==0||mask>=3);
                GL11.glClearColor(r,g,b,1);GL11.glClear(GL11.GL_COLOR_BUFFER_BIT);
                GL11.glEnable(GL11.GL_BLEND);GL11.glBlendFunc(GL11.GL_SRC_ALPHA,GL11.GL_DST_ALPHA);GL11.glColor4f(1,0,1,sourceAlpha);
                GL11.glBegin(GL11.GL_QUADS);GL11.glVertex3f(-1,-1,0);GL11.glVertex3f(1,-1,0);GL11.glVertex3f(1,1,0);GL11.glVertex3f(-1,1,0);GL11.glEnd();
                pixel.clear();GL11.glReadPixels(mc.displayWidth/2,mc.displayHeight/2,1,1,GL11.GL_RGBA,GL11.GL_UNSIGNED_BYTE,pixel);pixel.get(rgba);data.write(rgba);
            }
            GL11.glColorMask(true,true,true,true);GL11.glDisable(GL11.GL_BLEND);unbindFbo();
            data.close();mc.gameSettings.anaglyph=old;
        }

        void setSplash(GuiMainMenu m) throws Exception {
            Field f = GuiMainMenu.class.getDeclaredField("splashText");
            f.setAccessible(true);
            f.set(m, splash);
        }

        void call(GuiScreen s, String name, int x, int y, int b) throws Exception {
            Method m = GuiScreen.class.getDeclaredMethod(name, int.class, int.class, int.class);
            m.setAccessible(true);
            m.invoke(s, x, y, b);
        }

        int fbo = -1;
        float clear = 0.0F;

        /** Draw into a plain (single-sample) framebuffer: drivers may force MSAA on the window. */
        void bindFbo() {
            int w = mc.displayWidth, h = mc.displayHeight;
            if (fbo < 0) {
                fbo = org.lwjgl.opengl.EXTFramebufferObject.glGenFramebuffersEXT();
                org.lwjgl.opengl.EXTFramebufferObject.glBindFramebufferEXT(org.lwjgl.opengl.EXTFramebufferObject.GL_FRAMEBUFFER_EXT, fbo);
                int color = org.lwjgl.opengl.EXTFramebufferObject.glGenRenderbuffersEXT();
                org.lwjgl.opengl.EXTFramebufferObject.glBindRenderbufferEXT(org.lwjgl.opengl.EXTFramebufferObject.GL_RENDERBUFFER_EXT, color);
                org.lwjgl.opengl.EXTFramebufferObject.glRenderbufferStorageEXT(org.lwjgl.opengl.EXTFramebufferObject.GL_RENDERBUFFER_EXT, System.getenv("GUI_FLOAT")==null?GL11.GL_RGBA8:34836, w, h);
                org.lwjgl.opengl.EXTFramebufferObject.glFramebufferRenderbufferEXT(org.lwjgl.opengl.EXTFramebufferObject.GL_FRAMEBUFFER_EXT, org.lwjgl.opengl.EXTFramebufferObject.GL_COLOR_ATTACHMENT0_EXT, org.lwjgl.opengl.EXTFramebufferObject.GL_RENDERBUFFER_EXT, color);
                int depth = org.lwjgl.opengl.EXTFramebufferObject.glGenRenderbuffersEXT();
                org.lwjgl.opengl.EXTFramebufferObject.glBindRenderbufferEXT(org.lwjgl.opengl.EXTFramebufferObject.GL_RENDERBUFFER_EXT, depth);
                org.lwjgl.opengl.EXTFramebufferObject.glRenderbufferStorageEXT(org.lwjgl.opengl.EXTFramebufferObject.GL_RENDERBUFFER_EXT, GL11.GL_DEPTH_COMPONENT, w, h);
                System.out.println("java: depth bits "+org.lwjgl.opengl.EXTFramebufferObject.glGetRenderbufferParameterEXT(org.lwjgl.opengl.EXTFramebufferObject.GL_RENDERBUFFER_EXT,org.lwjgl.opengl.EXTFramebufferObject.GL_RENDERBUFFER_DEPTH_SIZE_EXT));
                org.lwjgl.opengl.EXTFramebufferObject.glFramebufferRenderbufferEXT(org.lwjgl.opengl.EXTFramebufferObject.GL_FRAMEBUFFER_EXT, org.lwjgl.opengl.EXTFramebufferObject.GL_DEPTH_ATTACHMENT_EXT, org.lwjgl.opengl.EXTFramebufferObject.GL_RENDERBUFFER_EXT, depth);
                int st = org.lwjgl.opengl.EXTFramebufferObject.glCheckFramebufferStatusEXT(org.lwjgl.opengl.EXTFramebufferObject.GL_FRAMEBUFFER_EXT);
                System.out.println("java: capture framebuffer status 0x" + Integer.toHexString(st) + ", samples "
                    + GL11.glGetInteger(org.lwjgl.opengl.GL13.GL_SAMPLES));
            }
            org.lwjgl.opengl.EXTFramebufferObject.glBindFramebufferEXT(org.lwjgl.opengl.EXTFramebufferObject.GL_FRAMEBUFFER_EXT, fbo);
            GL11.glClearColor(clear, clear, clear, 0.0F);
            GL11.glClear(GL11.GL_COLOR_BUFFER_BIT | GL11.GL_DEPTH_BUFFER_BIT);
        }

        void unbindFbo() {
            org.lwjgl.opengl.EXTFramebufferObject.glBindFramebufferEXT(org.lwjgl.opengl.EXTFramebufferObject.GL_FRAMEBUFFER_EXT, 0);
        }

        void shot(String name) throws Exception {
            bindFbo();
            try {
                shotInto(name);
            } finally {
                unbindFbo();
            }
        }

        void shotInto(String name) throws Exception {
            if (mode.equals("mojang")) {
                Method m = Minecraft.class.getDeclaredMethod("loadScreen");
                m.setAccessible(true);
                m.invoke(mc);
                save(name, org.lwjgl.opengl.EXTFramebufferObject.GL_COLOR_ATTACHMENT0_EXT);
            } else if (mode.equals("hud")) {
                Field v = GuiIngame.class.getDeclaredField("prevVignetteBrightness");
                v.setAccessible(true);
                v.setFloat(mc.ingameGUI, 1.0F);
                mc.ingameGUI.renderGameOverlay(0.0F, false, mx, my);
                save(name, org.lwjgl.opengl.EXTFramebufferObject.GL_COLOR_ATTACHMENT0_EXT);
            } else if (mode.equals("loading")) {
                LoadingScreenRenderer l = new LoadingScreenRenderer(mc);
                l.printText(loadingTitle);
                l.displayLoadingString(loadingText);
                if (loadingProgress >= 0) {
                    Field t = LoadingScreenRenderer.class.getDeclaredField("field_1006_d");
                    t.setAccessible(true);
                    t.setLong(l, 0L);
                    l.setLoadingProgress(loadingProgress);
                }
                save(name, org.lwjgl.opengl.EXTFramebufferObject.GL_COLOR_ATTACHMENT0_EXT);
            } else if(mode.equals("normalprobe")) {
                GL11.glDisable(GL11.GL_TEXTURE_2D);GL11.glDisable(GL11.GL_DEPTH_TEST);
                GL11.glEnable(org.lwjgl.opengl.GL12.GL_RESCALE_NORMAL);GL11.glDisable(GL11.GL_NORMALIZE);
                GL11.glColor4f(1,1,1,1);RenderHelper.enableStandardItemLighting();
                int[] values={-128,-127,-64,-1,0,1,64,127};
                for(int axis=0;axis<3;axis++)for(int j=0;j<values.length;j++) {
                    int n=values[j];GL11.glNormal3b((byte)(axis==0?n:0),(byte)(axis==1?n:0),(byte)(axis==2?n:0));
                    GL11.glBegin(GL11.GL_QUADS);int x=10+j*25,y=10+axis*30;
                    GL11.glVertex3f(x,y,0);GL11.glVertex3f(x,y+20,0);GL11.glVertex3f(x+20,y+20,0);GL11.glVertex3f(x+20,y,0);GL11.glEnd();
                    java.nio.FloatBuffer vertices=BufferUtils.createFloatBuffer(12);vertices.put(new float[]{x,y+110,0,x,y+130,0,x+20,y+130,0,x+20,y+110,0}).flip();
                    ByteBuffer normals=BufferUtils.createByteBuffer(12);for(int k=0;k<4;k++){normals.put((byte)(axis==0?n:0));normals.put((byte)(axis==1?n:0));normals.put((byte)(axis==2?n:0));}normals.flip();
                    GL11.glVertexPointer(3,0,vertices);GL11.glNormalPointer(0,normals);GL11.glEnableClientState(GL11.GL_VERTEX_ARRAY);GL11.glEnableClientState(GL11.GL_NORMAL_ARRAY);GL11.glDrawArrays(GL11.GL_QUADS,0,4);GL11.glDisableClientState(GL11.GL_VERTEX_ARRAY);GL11.glDisableClientState(GL11.GL_NORMAL_ARRAY);
                }
                for(int mode=0;mode<3;mode++)for(int j=0;j<values.length;j++) {
                    int n=values[j];float b=mode==2?1f/255f:0f;float v=mode==0?n/127f:mode==1?n/128f:(2f*n+1f)/255f;
                    GL11.glNormal3f(v,b,b);GL11.glBegin(GL11.GL_QUADS);int x=220+j*25,y=10+mode*30;
                    GL11.glVertex3f(x,y,0);GL11.glVertex3f(x,y+20,0);GL11.glVertex3f(x+20,y+20,0);GL11.glVertex3f(x+20,y,0);GL11.glEnd();
                }
                RenderHelper.disableStandardItemLighting();GL11.glEnable(GL11.GL_TEXTURE_2D);
                GL11.glReadBuffer(org.lwjgl.opengl.EXTFramebufferObject.GL_COLOR_ATTACHMENT0_EXT);
                for(int row=0;row<3;row++)for(int j=0;j<values.length;j++) {
                    java.nio.FloatBuffer f=BufferUtils.createFloatBuffer(4);GL11.glReadPixels(2*(20+j*25),mc.displayHeight-2*(130+row*30),1,1,GL11.GL_RGBA,GL11.GL_FLOAT,f);
                    System.out.println("normal "+row+" "+values[j]+" "+f.get(0));
                }
                save(name,org.lwjgl.opengl.EXTFramebufferObject.GL_COLOR_ATTACHMENT0_EXT);
            } else {
                setMouseDown(down);
                inner.drawScreen(mx, my, 0.0F);
                save(name, org.lwjgl.opengl.EXTFramebufferObject.GL_COLOR_ATTACHMENT0_EXT);
            }
        }

        void save(String name, int buffer) throws Exception {
            int w = mc.displayWidth, h = mc.displayHeight;
            if(System.getenv("GUI_FLOAT")!=null) {
                java.nio.FloatBuffer f=BufferUtils.createFloatBuffer(w*h*4);GL11.glReadBuffer(buffer);GL11.glReadPixels(0,0,w,h,GL11.GL_RGBA,GL11.GL_FLOAT,f);
                DataOutputStream floats=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(new File(out,name+".float"))));
                while(f.hasRemaining())floats.writeFloat(f.get());floats.close();
            }
            ByteBuffer px = BufferUtils.createByteBuffer(w * h * 4);
            GL11.glReadBuffer(buffer);
            GL11.glReadPixels(0, 0, w, h, GL11.GL_RGBA, GL11.GL_UNSIGNED_BYTE, px);
            BufferedImage img = new BufferedImage(w, h, BufferedImage.TYPE_INT_RGB);
            for (int y = 0; y < h; y++) {
                for (int x = 0; x < w; x++) {
                    int i = ((h - 1 - y) * w + x) * 4;
                    int r = px.get(i) & 255, g = px.get(i + 1) & 255, b = px.get(i + 2) & 255;
                    img.setRGB(x, y, r << 16 | g << 8 | b);
                }
            }
            File f = new File(out, name + ".png");
            ImageIO.write(img, "png", f);
            System.out.println("java: " + f.getPath());
        }
    }

    public static void main(String[] args) throws Exception {
        File scriptFile = new File(args[0]);
        scriptDir = scriptFile.getAbsoluteFile().getParentFile();
        out = new File(args[1]);
        out.mkdirs();
        try (BufferedReader r = new BufferedReader(new InputStreamReader(new FileInputStream(scriptFile), "UTF-8"))) {
            for (String l; (l = r.readLine()) != null; ) script.add(l);
        }
        Field dir = Minecraft.class.getDeclaredField("minecraftDir");
        dir.setAccessible(true);
        dir.set(null, new File(args[2]).getAbsoluteFile());

        Frame frame = new Frame("Minecraft (oracle GUI capture)");
        Canvas canvas = new Canvas();
        frame.setLayout(new BorderLayout());
        frame.add(canvas, "Center");
        canvas.setPreferredSize(new Dimension(854, 480));
        frame.pack();
        frame.setLocationRelativeTo(null);
        ShotMinecraft mc = new ShotMinecraft(frame, canvas);
        mc.minecraftUri = "www.minecraft.net";
        mc.session = new Session("Player", "-");
        frame.setVisible(true);
        Thread t = new Thread(mc, "Minecraft main thread");
        t.start();
        Thread watchdog = new Thread(() -> {
            try {
                Thread.sleep(120000L);
            } catch (InterruptedException e) {
            }
            System.err.println("GuiShot: timed out at script line " + pc);
            Runtime.getRuntime().halt(4);
        });
        watchdog.setDaemon(true);
        watchdog.start();
    }
}
