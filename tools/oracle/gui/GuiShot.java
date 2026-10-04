package net.minecraft.src;

import java.awt.BorderLayout;
import java.awt.Canvas;
import java.awt.Dimension;
import java.awt.Frame;
import java.awt.image.BufferedImage;
import java.io.BufferedReader;
import java.io.File;
import java.io.FileReader;
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
                    if (which.equals("title")) s = title;
                    else if (which.equals("options")) s = new GuiOptions(title, mc.gameSettings);
                    else if (which.equals("video")) s = new GuiVideoSettings(title, mc.gameSettings);
                    else if (which.equals("controls")) s = new GuiControls(title, mc.gameSettings);
                    else if (which.equals("select")) s = new GuiSelectWorld(title);
                    else if (which.equals("create")) s = new GuiCreateWorld(title);
                    else if (which.equals("rename")) s = new GuiRenameWorld(title, a[1]);
                    else if (which.equals("texturepacks")) s = new GuiTexturePacks(title);
                    else throw new IllegalArgumentException("unknown screen " + which);
                    mc.displayGuiScreen(s);
                } else if (cmd.equals("world")) {
                    mc.session = new Session("Player", "-"); // EntityPlayerSP reads the username
                    mc.playerController = new PlayerControllerSP(mc);
                    mc.startWorld("guishot_world", "guishot", Long.parseLong(a[0]));
                    mc.currentScreen = this;
                    inner = null;
                } else if (cmd.equals("hud")) {
                    mode = "hud";
                    EntityPlayerSP p = mc.thePlayer;
                    for (String kv : a) {
                        String[] e = kv.split("=");
                        int v = e.length > 1 && !e[0].equals("pos") && !e[0].equals("brightness") ? Integer.parseInt(e[1]) : 0;
                        if (e[0].equals("health")) p.health = v;
                        else if (e[0].equals("prev")) p.prevHealth = v;
                        else if (e[0].equals("hearts_life")) p.heartsLife = v;
                        else if (e[0].equals("item")) p.inventory.currentItem = v;
                        else if (e[0].equals("pos")) {
                            String[] c = e[1].split(",");
                            p.setPosition(Double.parseDouble(c[0]), Double.parseDouble(c[1]), Double.parseDouble(c[2]));
                        }
                    }
                    System.out.println("java: player brightness " + p.getEntityBrightness(0.0F));
                } else if (cmd.equals("clear")) {
                    clear = Integer.parseInt(a[0]) / 255.0F;
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
                    call(inner, "mouseClicked", mx, my, 0);
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
                org.lwjgl.opengl.EXTFramebufferObject.glRenderbufferStorageEXT(org.lwjgl.opengl.EXTFramebufferObject.GL_RENDERBUFFER_EXT, GL11.GL_RGBA8, w, h);
                org.lwjgl.opengl.EXTFramebufferObject.glFramebufferRenderbufferEXT(org.lwjgl.opengl.EXTFramebufferObject.GL_FRAMEBUFFER_EXT, org.lwjgl.opengl.EXTFramebufferObject.GL_COLOR_ATTACHMENT0_EXT, org.lwjgl.opengl.EXTFramebufferObject.GL_RENDERBUFFER_EXT, color);
                int depth = org.lwjgl.opengl.EXTFramebufferObject.glGenRenderbuffersEXT();
                org.lwjgl.opengl.EXTFramebufferObject.glBindRenderbufferEXT(org.lwjgl.opengl.EXTFramebufferObject.GL_RENDERBUFFER_EXT, depth);
                org.lwjgl.opengl.EXTFramebufferObject.glRenderbufferStorageEXT(org.lwjgl.opengl.EXTFramebufferObject.GL_RENDERBUFFER_EXT, GL11.GL_DEPTH_COMPONENT, w, h);
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
            } else {
                setMouseDown(down);
                inner.drawScreen(mx, my, 0.0F);
                save(name, org.lwjgl.opengl.EXTFramebufferObject.GL_COLOR_ATTACHMENT0_EXT);
            }
        }

        void save(String name, int buffer) throws Exception {
            int w = mc.displayWidth, h = mc.displayHeight;
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
        try (BufferedReader r = new BufferedReader(new FileReader(scriptFile))) {
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
