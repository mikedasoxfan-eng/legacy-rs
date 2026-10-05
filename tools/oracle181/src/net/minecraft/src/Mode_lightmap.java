package net.minecraft.src;

import java.lang.reflect.Field;
import java.util.Random;

/**
 * oracle.sh lightmap out.bin: the original EntityRenderer.func_35808_e (b1.8.1 lightmap) over
 * times of day, rain, thunder, lightning flashes, torch flicker values, brightness settings and
 * both dimensions. Per case: the 16 brightness-table floats, sky factor, flicker, flash, gamma,
 * then celestial angle, rain, thunder and World.func_35464_b(1) for the sky-factor check, then
 * the 256 texels.
 */
public class Mode_lightmap {
    /** Captures the texels instead of uploading them. */
    static class CaptureEngine extends RenderEngine {
        int[] last;
        CaptureEngine() { super(null, null); }
        public void createTextureFromBytes(int[] a, int w, int h, int id) { last = a.clone(); }
    }

    static Object unsafeAlloc(Class<?> c) throws Exception {
        Field f = sun.misc.Unsafe.class.getDeclaredField("theUnsafe");
        f.setAccessible(true);
        return ((sun.misc.Unsafe) f.get(null)).allocateInstance(c);
    }

    public static void main(String[] a) throws Exception {
        Oracle.DataOut o = new Oracle.DataOut();
        Random r = new Random(181);
        int cases = 120;
        o.i32(cases);
        net.minecraft.client.Minecraft mc = (net.minecraft.client.Minecraft) unsafeAlloc(MinecraftImpl.class);
        mc.gameSettings = new GameSettings();
        CaptureEngine eng = (CaptureEngine) unsafeAlloc(CaptureEngine.class);
        mc.renderEngine = eng;
        EntityRenderer er = (EntityRenderer) unsafeAlloc(EntityRenderer.class);
        Field fmc = EntityRenderer.class.getDeclaredField("mc"); fmc.setAccessible(true); fmc.set(er, mc);
        Field fl = EntityRenderer.class.getDeclaredField("field_35811_L"); fl.setAccessible(true); fl.set(er, new int[256]);
        Field fe = EntityRenderer.class.getDeclaredField("field_35819_e"); fe.setAccessible(true);
        java.lang.reflect.Method upd = EntityRenderer.class.getDeclaredMethod("func_35808_e"); upd.setAccessible(true);
        World over = Oracle.newWorld(0), nether = Oracle.newNetherWorld(0);
        for (int c = 0; c < cases; c++) {
            World w = c % 5 == 4 ? nether : over;
            mc.theWorld = w;
            w.worldInfo.setWorldTime(r.nextInt(24000) + (c < 24 ? 0 : 24000L * r.nextInt(3)));
            float rain = c % 3 == 0 ? 0.0F : r.nextFloat(), thunder = c % 4 == 0 ? 0.0F : r.nextFloat();
            w.prevRainingStrength = w.rainingStrength = rain;
            w.prevThunderingStrength = w.thunderingStrength = thunder;
            w.field_27172_i = c % 7 == 3 ? 2 : 0;
            float flicker = c % 6 == 0 ? 0.0F : (r.nextFloat() - 0.5F) * 0.8F;
            fe.setFloat(er, flicker);
            float gamma = c % 4 == 1 ? 0.0F : c % 4 == 2 ? 1.0F : r.nextFloat();
            mc.gameSettings.field_35380_M = gamma;
            upd.invoke(er);
            for (float t : w.worldProvider.lightBrightnessTable) o.i32(Float.floatToRawIntBits(t));
            float sky = w.func_35464_b(1.0F);
            o.i32(Float.floatToRawIntBits(sky));
            o.i32(Float.floatToRawIntBits(flicker));
            o.i32(w.field_27172_i > 0 ? 1 : 0);
            o.i32(Float.floatToRawIntBits(gamma));
            o.i32(Float.floatToRawIntBits(w.getCelestialAngle(1.0F)));
            o.i32(Float.floatToRawIntBits(w.getRainStrength(1.0F)));
            o.i32(Float.floatToRawIntBits(w.getWeightedThunderStrength(1.0F)));
            o.i32(Float.floatToRawIntBits(sky));
            for (int v : eng.last) o.i32(v);
        }
        try (java.io.OutputStream os = new java.io.FileOutputStream(a[1])) { o.writeTo(os); }
    }
}
