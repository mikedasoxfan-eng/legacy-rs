package net.minecraft.src;
import java.io.*;
import java.lang.reflect.*;
import org.lwjgl.opengl.GL11;
/** Companion capture of the actual 1.8.1 Tessellator GL submissions. The legacy
 * fixtures stay readable; this sidecar retains primitive mode and all eight
 * words, including packed light. It is a fixture, not an exactness assertion. */
public final class Mode_drawintegration10 {
    private static Oracle.DataOut draws;
    private static int count;
    public static void record(int mode, int first, int n) {
        if (draws == null) return;
        try {
            Tessellator t = Tessellator.instance;
            int mask = (RendererOracle.field("hasTexture").getBoolean(t) ? 1 : 0)
                | (RendererOracle.field("hasColor").getBoolean(t) ? 2 : 0)
                | (RendererOracle.field("hasNormals").getBoolean(t) ? 4 : 0)
                | (RendererOracle.field("hasBrightness").getBoolean(t) ? 8 : 0);
            int[] raw = (int[]) RendererOracle.field("rawBuffer").get(t);
            draws.i32(mode); draws.i32(first); draws.i32(mask); draws.i32(n);
            draws.i32(GL11.texture);
            for (float v : GL11.color) Oracle.modelFloat(draws, v);
            for (int i = 0; i < n; i++) for (int word = 0; word < 8; word++) {
                boolean enabled = word < 3 || (word < 5 ? (mask & 1) != 0
                    : word == 5 ? (mask & 2) != 0 : word == 6 ? (mask & 4) != 0 : (mask & 8) != 0);
                draws.i32(enabled ? raw[(first + i) * 8 + word] : 0);
            }
            count++;
        } catch (Exception e) { throw new RuntimeException(e); }
    }
    public static void main(String[] a) throws Exception {
        String kind = a[1], path = a[2];
        ColorizerGrass.setGrassBiomeColorizer(Oracle.jarImage("misc/grasscolor.png"));
        ColorizerFoliage.getFoilageBiomeColorizer(Oracle.jarImage("misc/foliagecolor.png"));
        draws = new Oracle.DataOut(); count = 0;
        try {
            if (kind.equals("objects")) RendererOracle.run(path);
            else if (kind.equals("pickup")) ParticleOracle.pickupDraws(path, true);
            else if (kind.equals("pickupfast")) ParticleOracle.pickupDraws(path, false);
            else if (kind.equals("effects")) ParticleOracle.effects(path);
            else throw new IllegalArgumentException("objects|pickup|pickupfast|effects");
            Oracle.DataOut out = new Oracle.DataOut();
            out.bytes("B181DRW1".getBytes("US-ASCII")); out.i32(count);
            out.bytes(draws.buf.toByteArray());
            try (OutputStream stream = new FileOutputStream(path + ".draws181")) { out.writeTo(stream); }
            System.out.println("1.8.1 full Tessellator capture: " + count + " GL submissions");
        } finally { draws = null; }
    }
}
