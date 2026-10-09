package net.minecraft.src;

import java.awt.BorderLayout;
import java.awt.Canvas;
import java.awt.Dimension;
import java.awt.Frame;
import java.awt.image.BufferedImage;
import java.io.File;
import java.nio.ByteBuffer;
import javax.imageio.ImageIO;
import net.minecraft.client.Minecraft;
import org.lwjgl.BufferUtils;
import org.lwjgl.input.Keyboard;
import org.lwjgl.input.Mouse;
import org.lwjgl.opengl.GL11;

/**
 * The original Release 1.0 client in the End, for comparing the port's End lighting, fog and sky
 * (QA journey, Part C). Starts a world with the given seed, goes to the End with the real
 * Minecraft.usePortal(1) (Teleporter builds the obsidian platform at 100, 0), turns the player and
 * saves what the game rendered for each view, after letting the chunk renderers catch up.
 *
 *   EndShot seed outdir minecraftDataDir name:yaw:pitch...
 *
 * The window shows a pass-through screen (no pausing, input drained) whose drawScreen reads the
 * back buffer right after EntityRenderer has drawn the world and GuiIngame its overlay.
 */
public class EndShot {
    static String[] views;
    static long seed;
    static File out;

    static class ShotMinecraft extends Minecraft {
        Grab grab;

        ShotMinecraft(Frame f, Canvas c) {
            super(f, c, null, 854, 480, false);
        }

        public void displayUnexpectedThrowable(UnexpectedThrowable t) {
            System.err.println("crashed: " + t.description);
            t.exception.printStackTrace();
            Runtime.getRuntime().halt(3);
        }

        public void displayGuiScreen(GuiScreen s) {
            super.displayGuiScreen(s);
            if (grab == null) {
                grab = new Grab();
                int w = this.currentScreen != null ? this.currentScreen.width : 427;
                int h = this.currentScreen != null ? this.currentScreen.height : 240;
                grab.setWorldAndResolution(this, w, h);
            }
            this.currentScreen = grab;
        }
    }

    static class Grab extends GuiScreen {
        int frame = 0, view = 0, settle = 0;
        boolean inEnd = false;

        public boolean doesGuiPauseGame() {
            return false;
        }

        public void handleInput() {
            while (Mouse.next()) {
            }
            while (Keyboard.next()) {
            }
        }

        public void drawScreen(int x, int y, float partial) {
            try {
                step();
            } catch (Exception e) {
                e.printStackTrace();
                Runtime.getRuntime().halt(2);
            }
        }

        void step() throws Exception {
            frame++;
            if (mc.theWorld == null) {
                if (frame < 5) return;
                mc.session = new Session("Player", "-");
                mc.playerController = new PlayerControllerSP(mc);
                mc.startWorld("journey_endshot", "journey_endshot", new WorldSettings(seed, 0, true, false, EnumWorldType.DEFAULT));
                mc.currentScreen = this;
                return;
            }
            if (!inEnd) {
                inEnd = true;
                mc.usePortal(1);
                mc.currentScreen = this;
                System.out.println("java: in dimension " + mc.thePlayer.dimension + " at " + mc.thePlayer.posX + " " + mc.thePlayer.posY + " " + mc.thePlayer.posZ);
                settle = 0;
                return;
            }
            String[] v = views[view].split(":");
            mc.thePlayer.rotationYaw = mc.thePlayer.prevRotationYaw = Float.parseFloat(v[1]);
            mc.thePlayer.rotationPitch = mc.thePlayer.prevRotationPitch = Float.parseFloat(v[2]);
            if (++settle < (view == 0 ? 400 : 60)) return;
            int w = mc.displayWidth, h = mc.displayHeight;
            ByteBuffer px = BufferUtils.createByteBuffer(w * h * 4);
            GL11.glReadBuffer(GL11.GL_BACK);
            GL11.glReadPixels(0, 0, w, h, GL11.GL_RGBA, GL11.GL_UNSIGNED_BYTE, px);
            BufferedImage img = new BufferedImage(w, h, BufferedImage.TYPE_INT_RGB);
            for (int yy = 0; yy < h; yy++) {
                for (int xx = 0; xx < w; xx++) {
                    int i = ((h - 1 - yy) * w + xx) * 4;
                    img.setRGB(xx, yy, (px.get(i) & 255) << 16 | (px.get(i + 1) & 255) << 8 | (px.get(i + 2) & 255));
                }
            }
            File f = new File(out, v[0] + ".png");
            ImageIO.write(img, "png", f);
            EntityRenderer r = mc.entityRenderer;
            java.lang.reflect.Field fr = EntityRenderer.class.getDeclaredField("fogColorRed");
            java.lang.reflect.Field fg = EntityRenderer.class.getDeclaredField("fogColorGreen");
            java.lang.reflect.Field fb = EntityRenderer.class.getDeclaredField("fogColorBlue");
            fr.setAccessible(true);
            fg.setAccessible(true);
            fb.setAccessible(true);
            System.out.println("java: " + f.getPath() + " pos " + mc.thePlayer.posX + " " + mc.thePlayer.posY + " " + mc.thePlayer.posZ
                + " fog " + fr.getFloat(r) + " " + fg.getFloat(r) + " " + fb.getFloat(r)
                + " celestial " + mc.theWorld.getCelestialAngle(1.0F) + " gamma " + mc.gameSettings.gammaSetting
                + " fancy " + mc.gameSettings.fancyGraphics + " ao " + mc.gameSettings.ambientOcclusion
                + " distance " + mc.gameSettings.renderDistance);
            view++;
            settle = 0;
            if (view >= views.length) {
                System.out.println("java: done");
                Runtime.getRuntime().halt(0);
            }
        }
    }

    public static void main(String[] args) throws Exception {
        seed = Long.parseLong(args[0]);
        out = new File(args[1]);
        out.mkdirs();
        views = java.util.Arrays.copyOfRange(args, 3, args.length);
        java.lang.reflect.Field dir = Minecraft.class.getDeclaredField("minecraftDir");
        dir.setAccessible(true);
        dir.set(null, new File(args[2]).getAbsoluteFile());
        Frame frame = new Frame("Minecraft (oracle End capture)");
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
        new Thread(mc, "Minecraft main thread").start();
        Thread watchdog = new Thread(new Runnable() {
            public void run() {
                try {
                    Thread.sleep(180000L);
                } catch (InterruptedException e) {
                }
                System.err.println("EndShot: timed out");
                Runtime.getRuntime().halt(4);
            }
        });
        watchdog.setDaemon(true);
        watchdog.start();
    }
}
