package net.minecraft.src;

import java.io.*;
import java.util.*;
import org.lwjgl.opengl.GL11;
import org.lwjgl.opengl.GL13;

/** Complete logical draws, including display-list playback. Unlike the legacy
 * seven-word format, this retains primitive mode and both lightmap inputs.
 * A companion legacy file supplies the original case inputs without changes. */
public final class Mode_drawtrace10 {
    private static Oracle.DataOut trace;
    private static int count;

    private static void pickupLight(String path) throws Exception {
        // Match Minecraft.startGame before exercising the original unit-1 callback.
        OpenGlHelper.initializeTextures();
        World w = Oracle.newWorld(0); w.chunkProvider = new EntityOracle.FlatProvider(w);
        RenderManager boot = RendererOracle.manager(w), m = RenderManager.instance;
        m.worldObj = w; m.options = boot.options; m.renderEngine = boot.renderEngine;
        m.playerViewY = 37.25F; m.playerViewX = -21.5F;
        Oracle.DataOut cases = new Oracle.DataOut(); int scenes = 0;
        for (int sky : new int[]{0, 5, 15}) for (int block : new int[]{0, 7, 15})
        for (int kind = 0; kind < 2; kind++) for (int age = 0; age < 3; age++)
        for (float partial : new float[]{0, .25F, .75F, 1}) {
            for (Chunk c : ((EntityOracle.FlatProvider)w.chunkProvider).chunks.values()) {
                Arrays.fill(c.skylightMap.data, (byte)(sky | sky << 4));
                Arrays.fill(c.blocklightMap.data, (byte)(block | block << 4));
            }
            w.skylightSubtracted = 0;
            OracleHooks.reset(100); EntityOracle.mathRandom().setSeed(200); w.rand.setSeed(300);
            Entity source = kind == 0 ? new EntityItem(w, 8.5, 65.1, 8.5, new ItemStack(1, 1, 0)) : new EntityArrow(w);
            source.setPosition(8.5, 65.1, 8.5);
            source.prevRotationYaw = source.rotationYaw = 13.5F;
            source.prevRotationPitch = source.rotationPitch = -26.75F;
            Entity target = new EntityCow(w); target.setPosition(9.5, 65.62, 9.5);
            target.lastTickPosX = 9.25; target.lastTickPosY = 65.5; target.lastTickPosZ = 9.75;
            EntityPickupFX p = new EntityPickupFX(w, source, target, -.5F);
            for (int tick = 0; tick < age; tick++) p.onUpdate();
            EntityFX.interpPosX = 8.25; EntityFX.interpPosY = 65; EntityFX.interpPosZ = 8.75;
            cases.i32(sky); cases.i32(block); cases.i32(kind); cases.i32(age); Oracle.modelFloat(cases, partial);
            RendererOracle.reset(1); GL13.lightmapU = -1; GL13.lightmapV = -1;
            RendererOracle.active = true; int before = count;
            try { p.renderParticle(Tessellator.instance, partial, 0, 0, 0, 0, 0); }
            finally { RendererOracle.active = false; }
            cases.i32(count - before); scenes++;
        }
        Oracle.DataOut out = new Oracle.DataOut();
        out.bytes("B181PKL1".getBytes("US-ASCII")); out.i32(scenes); out.bytes(cases.buf.toByteArray());
        try (OutputStream stream = new FileOutputStream(path)) { out.writeTo(stream); }
        System.out.println("pickup lightmap: " + scenes + " original EntityPickupFX scenes");
    }

    static void emitted(RendererOracle.Raw r) {
        if (trace == null) return;
        int mask = ((r.mask & 512) != 0 ? 1 : 0)
            | ((r.mask & 64) != 0 ? 2 : 0)
            | ((r.mask & 128) != 0 ? 4 : 0) | (r.packedLight ? 8 : 0);
        trace.i32(r.mode); trace.i32(mask); trace.i32(r.n);
        trace.i32(GL11.texture);
        for (float v : GL11.color) Oracle.modelFloat(trace, v);
        for (float v : RendererOracle.normal) Oracle.modelFloat(trace, v);
        trace.i32(RendererOracle.flags | r.mask);
        trace.i32(RendererOracle.src); trace.i32(RendererOracle.dst); trace.i32(RendererOracle.depth);
        Oracle.modelFloat(trace, GL13.lightmapU); Oracle.modelFloat(trace, GL13.lightmapV);
        trace.i32(RendererOracle.ops.size());
        for (float[] op : RendererOracle.ops) {
            trace.i32((int)op[0]);
            for (int i = 1; i < 5; i++) Oracle.modelFloat(trace, op[i]);
        }
        for (int i = 0; i < r.n; i++) for (int word = 0; word < 8; word++) {
            boolean enabled = word < 3 || (word < 5 ? (mask & 1) != 0
                : word == 5 ? (mask & 2) != 0 : word == 6 ? (mask & 4) != 0 : (mask & 8) != 0);
            trace.i32(enabled ? r.data[i * 8 + word] : 0);
        }
        count++;
    }

    /** Exercise the actual EffectRenderer batch, which the inherited effects
     * fixture bypasses. Each layer receives original per-particle packed light. */
    private static void particles(String path) throws Exception {
        World w = Oracle.newWorld(0);
        w.chunkProvider = new EntityOracle.FlatProvider(w);
        RenderEngine engine = RendererOracle.manager(w).renderEngine;
        Map<String,Integer> textures = new HashMap<String,Integer>();
        textures.put("/particles.png", 17); textures.put("/terrain.png", 1);
        textures.put("/gui/items.png", 2); textures.put("/misc/footprint.png", 3);
        Oracle.setField(engine, "textureMap", textures);
        Oracle.setField(engine, "texturePack", Oracle.allocate(TexturePackList.class));
        Entity camera = new EntityCow(w);
        camera.setPosition(8.5, 65, 8.5);
        camera.lastTickPosX = 8.25; camera.lastTickPosY = 64.75; camera.lastTickPosZ = 8.75;
        camera.rotationYaw = 37.25F; camera.rotationPitch = -21.5F;
        Oracle.DataOut cases = new Oracle.DataOut();
        int scenes = 0;
        for (int sky : new int[]{0, 5, 15}) for (int block : new int[]{0, 7, 15})
        for (float partial : new float[]{0, .25F, .75F, 1}) {
            for (Chunk c : ((EntityOracle.FlatProvider)w.chunkProvider).chunks.values()) {
                Arrays.fill(c.skylightMap.data, (byte)(sky | sky << 4));
                Arrays.fill(c.blocklightMap.data, (byte)(block | block << 4));
            }
            w.skylightSubtracted = 0;
            OracleHooks.reset(100); EntityOracle.mathRandom().setSeed(200); w.rand.setSeed(300);
            EffectRenderer effects = new EffectRenderer(w, engine);
            for (int kind = 0; kind < 18; kind++) {
                EntityFX p = ParticleOracle.create(kind, w, engine, 8.5, 65.1, 8.5,
                    .1, -.1, .2, 1, 0);
                effects.addEffect(p);
            }
            cases.i32(sky); cases.i32(block); Oracle.modelFloat(cases, partial);
            RendererOracle.reset(1); RendererOracle.active = true;
            int before = count;
            try { effects.renderParticles(camera, partial); effects.func_1187_b(camera, partial); }
            finally { RendererOracle.active = false; }
            cases.i32(count - before); scenes++;
        }
        Oracle.DataOut out = new Oracle.DataOut();
        out.bytes("B181PIP1".getBytes("US-ASCII")); out.i32(scenes); out.bytes(cases.buf.toByteArray());
        try (OutputStream stream = new FileOutputStream(path)) { out.writeTo(stream); }
        System.out.println("particle pipeline: " + scenes + " original EffectRenderer scenes");
    }

    public static void main(String[] a) throws Exception {
        String kind = a[1], path = a[2];
        ColorizerGrass.setGrassBiomeColorizer(Oracle.jarImage("misc/grasscolor.png"));
        ColorizerFoliage.getFoilageBiomeColorizer(Oracle.jarImage("misc/foliagecolor.png"));
        // A fresh JVM has these defaults; make them explicit for repeat callers.
        GL13.activeTexture = 0; GL13.lightmapU = 0; GL13.lightmapV = 0;
        RendererOracle.field("convertQuadsToTriangles").setBoolean(null, false);
        trace = new Oracle.DataOut(); count = 0; Mode_renderintegration10.captureInputs = true;
        try {
            if (kind.equals("objects")) RendererOracle.run(path);
            else if (kind.equals("pickup")) ParticleOracle.pickupDraws(path, true);
            else if (kind.equals("pickupfast")) ParticleOracle.pickupDraws(path, false);
            else if (kind.equals("particles")) particles(path);
            else if (kind.equals("pickuplight")) pickupLight(path);
            else throw new IllegalArgumentException("objects|pickup|pickupfast|particles|pickuplight");
            Oracle.DataOut out = new Oracle.DataOut();
            out.bytes("B181TRC1".getBytes("US-ASCII")); out.i32(count); out.bytes(trace.buf.toByteArray());
            try (OutputStream stream = new FileOutputStream(path + ".trace181")) { out.writeTo(stream); }
            System.out.println("complete 1.8.1 draw trace: " + count + " logical draws");
        } finally { trace = null; RendererOracle.active = false; Mode_renderintegration10.captureInputs = false; }
    }
}
