package net.minecraft.src;

import java.io.*;
import java.lang.reflect.Field;
import java.lang.reflect.Modifier;
import java.nio.ByteBuffer;
import java.nio.ByteOrder;
import java.util.*;

/**
 * Ground-truth dumper. Runs Mojang's original b1.7.3 bytecode (deobfuscated.jar,
 * names remapped only) and writes data the Rust port is compared against.
 * Lives in net.minecraft.src so it can call protected World methods.
 *
 *   rawnether <seed> <x0> <z0> <x1> <z1> <out>  unpopulated Nether chunks
 *   gridnether <seed> <wrand> <x0> <z0> <x1> <z1> <each|end|none> <out>
 *   raw <seed> <x0> <z0> <x1> <z1> <out>     unpopulated chunks (terrain+surface+caves)
 *   sin <out>                                MathHelper sine table
 *   blocktable <out.rs>                      static block/material properties as Rust source
 *   grid <seed> <wrand> <x0> <z0> <x1> <z1> <light:each|end|none> <out>
 *                                            load chunks x-major through the real World, dump world
 *   preload <seed> <wrand> <out> [fancy]     vanilla new-world flow: spawn search + "Building terrain"
 *   rendertable <out.rs>                     per-block render data (textures by meta/side, render type...)
 *   rendermesh <seed> <wrand> <out> cx cz [cx cz ...]
 *                                            after the vanilla preload, RenderBlocks output for chunks
 *   savepreload <seed> <wrand> <dir> <out>   preload with a real save directory, saveWorld(true), dump
 *   readsave <dir> <out>                     open a save made elsewhere, load every chunk stored in
 *                                            its region files with the game's own loader, dump
 *   mathcheck <out>                          Math.sin/cos/pow results BigTree can observe
 *   ticks <seed> <wrand> <lcg> <sound> <n> <clear|rain|thunder> <dx> <dz> <out> [fires]
 *                                            vanilla preload, a player at spawn+(dx,dz), then n
 *                                            World.tick() calls (lighting drained after each);
 *                                            dump plus <out>.txt with the world's tick state;
 *                                            fires "dx:dz,..." lights fire on top of those columns
 *   models <out>                             every mob part, box and pose, without OpenGL
 *
 * <wrand> seeds World.rand, which vanilla leaves unseeded (it drives spawn search,
 * lava flow during generation and item-drop jitter).
 */
public class Oracle {
    static final BiomeGenBase[] BIOMES = {
        BiomeGenBase.rainforest, BiomeGenBase.swampland, BiomeGenBase.seasonalForest,
        BiomeGenBase.forest, BiomeGenBase.savanna, BiomeGenBase.shrubland, BiomeGenBase.taiga,
        BiomeGenBase.desert, BiomeGenBase.plains, BiomeGenBase.iceDesert, BiomeGenBase.tundra,
        BiomeGenBase.hell, BiomeGenBase.sky,
    };

    static int biomeId(BiomeGenBase b) {
        for (int i = 0; i < BIOMES.length; i++) if (BIOMES[i] == b) return i;
        throw new IllegalStateException("unknown biome " + b.biomeName);
    }

    /** Save handler with no disk I/O; hands the world a preset WorldInfo so no random spawn search runs. */
    static class NullSaveHandler implements ISaveHandler {
        final long seed;
        NullSaveHandler(long seed) { this.seed = seed; }
        public WorldInfo loadWorldInfo() { return new WorldInfo(seed, "oracle"); }
        public void func_22150_b() {}
        public IChunkLoader getChunkLoader(WorldProvider p) { return null; }
        public void saveWorldInfoAndPlayer(WorldInfo i, List l) {}
        public void saveWorldInfo(WorldInfo i) {}
        public File func_28113_a(String s) { return null; }
    }

    static World newWorld(long seed) {
        return new World(new NullSaveHandler(seed), "oracle", seed);
    }

    public static void main(String[] a) throws Exception {
        String mode = a.length > 0 ? a[0] : "";
        if(mode.equals("signs")){DecorationOracle.signs(a[1]);return;}
        if(mode.equals("paintings")){DecorationOracle.paintings(a[1]);return;}
        if(mode.equals("mapfiles")){DecorationOracle.mapFiles(a[1]);return;}
        if(mode.equals("mapread")){DecorationOracle.mapRead(a[1],a[2]);return;}
        if(mode.equals("fishrenderers")){DecorationOracle.fishRenderers(a[1]);return;}
        if(mode.equals("fishing")){DecorationOracle.fishing(a[1]);return;}
        if(mode.equals("maps")){DecorationOracle.maps(a[1]);return;}
        if(mode.equals("paintingrenderers")){DecorationOracle.paintingRenderers(a[1]);return;}
        if(mode.equals("signrenderers")){DecorationOracle.signRenderers(a[1]);return;}
        if (mode.equals("pickuprenderersfast")) { ParticleOracle.pickupDraws(a[1],false);
        } else if (mode.equals("musaudio")) { SoundOracle.run(a[1]);
        } else if (mode.equals("pickuprenderers")) { ParticleOracle.pickupDraws(a[1]);
        } else if (mode.equals("musicevents")) { ParticleOracle.music(a[1]);
        } else if (mode.equals("particleeffects")) { ParticleOracle.effects(a[1]);
        } else if (mode.equals("displayevents")) { ParticleOracle.display(a[1]);
        } else if (mode.equals("particles")) { ParticleOracle.run(a[1]);
        } else if (mode.equals("raw")) {
            raw(Long.parseLong(a[1]), Integer.parseInt(a[2]), Integer.parseInt(a[3]),
                Integer.parseInt(a[4]), Integer.parseInt(a[5]), a[6]);
        } else if (mode.equals("sin")) {
            sin(a[1]);
        } else if (mode.equals("containerrecipes")) {
            ContainerOracle.recipes(a[1]);
        } else if (mode.equals("containerreplay")) {
            ContainerOracle.replay(a[1], a[2]);
        } else if (mode.equals("containertiles")) {
            ContainerOracle.tiles(Integer.parseInt(a[1]), a[2]);
        } else if (mode.equals("player")) {
            PlayerOracle.simulation(a[1],Integer.parseInt(a[2]),a[3]);
        } else if (mode.equals("playerregistry")) {
            PlayerOracle.registry(a[1]);
        } else if (mode.equals("playerinventory")) {
            PlayerOracle.inventory(a[1]);
        } else if (mode.equals("vehicleinventory")) {
            VehicleOracle.inventory(a[1]);
        } else if (mode.equals("vehicles")) {
            VehicleOracle.replay(a[1],Integer.parseInt(a[2]),a[3]);
        } else if (mode.equals("entities")) {
            EntityOracle.objects(a[1],Integer.parseInt(a[2]),a[3]);
        } else if (mode.equals("entityblocks")) {
            EntityOracle.blockPhysics(a[1]);
        } else if (mode.equals("entitygeometry")) {
            EntityOracle.geometry(a[1]);
        } else if (mode.equals("collisiontable")) {
            EntityOracle.blockGeometry(a[1]);
        } else if (mode.equals("blocktable")) {
            blockTable(a[1]);
        } else if (mode.equals("grid")) {
            grid(Long.parseLong(a[1]), Long.parseLong(a[2]), Integer.parseInt(a[3]), Integer.parseInt(a[4]),
                 Integer.parseInt(a[5]), Integer.parseInt(a[6]), a[7], a[8]);
        } else if (mode.equals("preload")) {
            // optional 5th arg "fancy": leaves render translucent (RenderGlobal already ran)
            Block.leaves.setGraphicsLevel(a.length > 4 && a[4].equals("fancy"));
            preload(Long.parseLong(a[1]), Long.parseLong(a[2]), a[3]);
        } else if (mode.equals("rendermesh") || mode.equals("rendermeshnether")) {
            meshNether = mode.equals("rendermeshnether");
            // chunk list, or "auto R": every chunk within R of the spawn chunk
            int meshEnd=a.length;
            boolean ao=true,fancy=true;
            if(meshEnd>4 && (a[meshEnd-1].equals("on") || a[meshEnd-1].equals("off"))) ao=!a[--meshEnd].equals("off");
            if(meshEnd>4 && (a[meshEnd-1].equals("fancy") || a[meshEnd-1].equals("fast"))) fancy=!a[--meshEnd].equals("fast");
            int[] cs;
            if (a[4].equals("auto")) {
                cs = new int[] {Integer.MIN_VALUE, Integer.parseInt(a[5])};
            } else {
                cs = new int[meshEnd - 4];
                for (int i = 4; i < meshEnd; i++) cs[i - 4] = Integer.parseInt(a[i]);
            }
            renderMesh(Long.parseLong(a[1]), Long.parseLong(a[2]), a[3], cs, fancy, ao);
        } else if (mode.equals("savepreload")) {
            savePreload(Long.parseLong(a[1]), Long.parseLong(a[2]), a[3], a[4]);
        } else if (mode.equals("readsave")) {
            readSave(a[1], a[2]);
        } else if (mode.equals("skycolor") || mode.equals("skycolornether")) {
            World w = mode.equals("skycolornether") ? newNetherWorld(Long.parseLong(a[1])) : newWorld(Long.parseLong(a[1]));
            w.setWorldTime(Long.parseLong(a[4]));
            EntityItem e = new EntityItem(w, Double.parseDouble(a[2]), 64.0, Double.parseDouble(a[3]), new ItemStack(1, 1, 0));
            float partial = Float.parseFloat(a[5]);
            float rain = a.length > 6 ? Float.parseFloat(a[6]) : 0.0F;
            float thunder = a.length > 7 ? Float.parseFloat(a[7]) : 0.0F;
            setField(w, "rainingStrength", rain);
            setField(w, "prevRainingStrength", a.length > 9 ? Float.parseFloat(a[9]) : rain);
            setField(w, "thunderingStrength", thunder);
            setField(w, "prevThunderingStrength", a.length > 10 ? Float.parseFloat(a[10]) : thunder);
            setField(w, "field_27172_i", a.length > 8 ? Integer.parseInt(a[8]) : 0);
            EntityRenderer renderer = headlessRenderer(w, e);
            setField(renderer, "fogColor2", a.length > 11 ? Float.parseFloat(a[11]) : 1.0F);
            setField(renderer, "fogColor1", a.length > 12 ? Float.parseFloat(a[12]) : 1.0F);
            int distance = a.length > 13 ? Integer.parseInt(a[13]) : 0;
            Field mcField = EntityRenderer.class.getDeclaredField("mc");
            mcField.setAccessible(true);
            ((net.minecraft.client.Minecraft)mcField.get(renderer)).gameSettings.renderDistance = distance;
            java.lang.reflect.Method updateFog = EntityRenderer.class.getDeclaredMethod("updateFogColor", float.class);
            updateFog.setAccessible(true);
            updateFog.invoke(renderer, partial);
            setField(renderer, "farPlaneDistance", (float)(256 >> distance));
            renderer.fogColorBuffer = ByteBuffer.allocateDirect(64).order(ByteOrder.nativeOrder()).asFloatBuffer();
            java.lang.reflect.Method setupFog = EntityRenderer.class.getDeclaredMethod("setupFog", int.class, float.class);
            setupFog.setAccessible(true); setupFog.invoke(renderer, 0, partial);
            Vec3D sky = w.func_4079_a(e, partial);
            Vec3D fog = w.getFogColor(partial);
            Vec3D cloud = w.func_628_d(partial);
            System.out.println("angle " + Float.floatToIntBits(w.getCelestialAngle(partial)) + " sub " + w.calculateSkylightSubtracted(1.0F)
                + " sky " + Float.floatToIntBits((float) sky.xCoord) + " " + Float.floatToIntBits((float) sky.yCoord) + " " + Float.floatToIntBits((float) sky.zCoord)
                + " fog " + Float.floatToIntBits((float) fog.xCoord) + " " + Float.floatToIntBits((float) fog.yCoord) + " " + Float.floatToIntBits((float) fog.zCoord)
                + " cloud " + Float.floatToIntBits((float) cloud.xCoord) + " " + Float.floatToIntBits((float) cloud.yCoord) + " " + Float.floatToIntBits((float) cloud.zCoord)
                + " stars " + Float.floatToIntBits(w.getStarBrightness(partial))
                + " renderfog " + Float.floatToIntBits(renderer.fogColorRed) + " " + Float.floatToIntBits(renderer.fogColorGreen) + " " + Float.floatToIntBits(renderer.fogColorBlue)
                + " renderstars " + Float.floatToIntBits(w.getStarBrightness(partial) * (1.0F - w.func_27162_g(partial)))
                + " sunalpha " + Float.floatToIntBits(1.0F - w.func_27162_g(partial))
                + " fogrange " + Float.floatToIntBits(org.lwjgl.opengl.GL11.fogStart) + " " + Float.floatToIntBits(org.lwjgl.opengl.GL11.fogEnd));
        } else if (mode.equals("weathermesh")) {
            weatherMesh(a);
        } else if (mode.equals("skydraw")) {
            skyDraw(a[1]);
        } else if (mode.equals("cloudmesh")) {
            cloudMesh(a);
        } else if (mode.equals("videowindow")) {
            videoWindow(a);
        } else if (mode.equals("rendertable")) {
            renderTable(a[1]);
        } else if (mode.equals("useportal")) {
            TeleportOracle.usePortal(Long.parseLong(a[1]), Long.parseLong(a[2]), Double.parseDouble(a[3]),
                Double.parseDouble(a[4]), Double.parseDouble(a[5]), Long.parseLong(a[6]), Long.parseLong(a[7]), a[8]);
        } else if (mode.equals("respawn")) {
            TeleportOracle.respawn(Long.parseLong(a[1]), Long.parseLong(a[2]), a[3], a[4]);
        } else if (mode.equals("statlist")) {
            StatsOracle.run(a[1]);
        } else if (mode.equals("convertworld")) {
            // SaveConverterMcRegion on a pre-McRegion save: prints every progress step.
            SaveConverterMcRegion c = new SaveConverterMcRegion(new java.io.File(a[1]));
            System.out.println("old " + c.isOldMapFormat(a[2]));
            c.convertMapFormat(a[2], new IProgressUpdate() {
                public void func_594_b(String t) { System.out.println("title " + t); }
                public void displayLoadingString(String t) { System.out.println("text " + t); }
                public void setLoadingProgress(int n) { System.out.println("progress " + n); }
            });
            System.out.println("old " + c.isOldMapFormat(a[2]));
        } else if (mode.equals("statsfile")) {
            StatsOracle.file(a[1], a[2]);
        } else if (mode.equals("pistonrender")) {
            PistonRendererOracle.run(a[1]);
        } else if (mode.equals("redstone")) {
            RedstoneOracle.run(a[1], a[2]);
        } else if (mode.equals("rendershowcase")) {
            // layout out [fancy|fast] [on|off]
            ShowcaseOracle.run(a[1], a[2], a.length <= 3 || !a[3].equals("fast"), a.length <= 4 || !a[4].equals("off"), a.length > 5 && a[5].equals("anaglyph"));
        } else if (mode.equals("teleport")) {
            TeleportOracle.run(Long.parseLong(a[1]), Long.parseLong(a[2]), Integer.parseInt(a[3]),
                Double.parseDouble(a[4]), Double.parseDouble(a[5]), Double.parseDouble(a[6]), Long.parseLong(a[7]), a[8]);
        } else if (mode.equals("ticks")) {
            ticks(Long.parseLong(a[1]), Long.parseLong(a[2]), Integer.parseInt(a[3]), Integer.parseInt(a[4]),
                  Integer.parseInt(a[5]), a[6], Double.parseDouble(a[7]), Double.parseDouble(a[8]), a[9],
                  a.length > 10 ? a[10] : "");
        } else if (mode.equals("mathcheck")) {
            mathCheck(a[1]);
        } else if (mode.equals("rawnether")) {
            rawNether(Long.parseLong(a[1]), Integer.parseInt(a[2]), Integer.parseInt(a[3]),
                Integer.parseInt(a[4]), Integer.parseInt(a[5]), a[6]);
        } else if (mode.equals("gridnether")) {
            gridNether(Long.parseLong(a[1]), Long.parseLong(a[2]), Integer.parseInt(a[3]), Integer.parseInt(a[4]),
                Integer.parseInt(a[5]), Integer.parseInt(a[6]), a[7], a[8]);
        } else if (mode.equals("livingrenderers")) {
            RendererOracle.mobs(a[1]);
        } else if (mode.equals("rendereffects")) {
            RendererOracle.effects(a[1]);
        } else if (mode.equals("firstperson")) {
            FirstPersonOracle.run(a[1]);
        } else if (mode.equals("vehiclerenderers")) {
            RendererOracle.vehicles(a[1]);
        } else if (mode.equals("renderers")) {
            RendererOracle.run(a[1]);
        } else if (mode.equals("itemtable")) {
            RendererOracle.itemTable(a[1]);
        } else if (mode.equals("models")) {
            models(a[1]);
        } else {
            System.err.println("bad mode; see Oracle.java header");
            System.exit(2);
        }
    }

    // ---------------------------------------------------------------- mob models

    /** All inherited parts, including private fields and the squid's array, sorted by Java name. */
    static SortedMap<String, ModelRenderer> modelParts(ModelBase m) throws Exception {
        SortedMap<String, ModelRenderer> parts = new TreeMap<>();
        for (Class<?> c = m.getClass(); c != null; c = c.getSuperclass()) {
            for (Field f : c.getDeclaredFields()) {
                f.setAccessible(true);
                if (f.getType() == ModelRenderer.class) {
                    ModelRenderer p = (ModelRenderer) f.get(m);
                    if (p != null) parts.put(f.getName(), p);
                } else if (f.getType() == ModelRenderer[].class) {
                    ModelRenderer[] ps = (ModelRenderer[]) f.get(m);
                    for (int i = 0; i < ps.length; i++) parts.put(f.getName() + "[" + i + "]", ps[i]);
                }
            }
        }
        return parts;
    }

    static void modelFloat(DataOut o, float v) { o.i32(Float.floatToRawIntBits(v)); }
    static void modelString(DataOut o, String s) throws Exception {
        byte[] b = s.getBytes("UTF-8"); o.i32(b.length); o.bytes(b);
    }
    static void modelVertex(DataOut o, PositionTextureVertex v) {
        o.i64(Double.doubleToRawLongBits(v.vector3D.xCoord));
        o.i64(Double.doubleToRawLongBits(v.vector3D.yCoord));
        o.i64(Double.doubleToRawLongBits(v.vector3D.zCoord));
        modelFloat(o, v.texturePositionX); modelFloat(o, v.texturePositionY);
    }
    static void wolfField(EntityWolf wolf, String name, float v) throws Exception {
        Field f = EntityWolf.class.getDeclaredField(name); f.setAccessible(true); f.setFloat(wolf, v);
    }

    /** Instantiate the original bytecode and exercise successive poses on each instance.
     * No render()/display-list methods are called, so this runs without GL or natives. */
    static void models(String out) throws Exception {
        int[] kinds = {0,0,0,1,2,3,3,4,4,5,6,7,8,9,10,10,10,11,12,13,14,15};
        float[] inflate = {0,.35F,-.25F,0,0,0,.6F,0,.5F,0,0,0,0,0,0,2,-.1F,0,0,0,0,0};
        float[] offsets = {0,1.25F,-2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0};
        int[] legs = {12,12,12,12,12,12,6,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12};
        float[][] poses = {
            {0,0,0,0,0,.0625F}, {1.25F,.7F,4.5F,35,-20,.0625F},
            {-3.75F,1,17.25F,-179,89,.125F}, {12.345F,.13F,-2.5F,270,-67,.03125F},
            {0,0,0,0,0,.0625F}, {3.1415927F,.99F,.75F,-45,12,.0625F},
            {1024.5F,.4F,10000.25F,123.75F,-45.5F,.0625F}, {0,0,0,0,0,.0625F}
        };
        float[] attacks = {0,.35F,.8F,1,-10000,.01F,.6F,0};
        Field corners = ModelRenderer.class.getDeclaredField("corners"); corners.setAccessible(true);
        Field faces = ModelRenderer.class.getDeclaredField("faces"); faces.setAccessible(true);
        Field normal = Tessellator.class.getDeclaredField("normal"); normal.setAccessible(true);
        Field drawing = Tessellator.class.getDeclaredField("isDrawing"); drawing.setAccessible(true);
        DataOut o = new DataOut(); o.i32(0x3337314d); o.i32(2); o.i32(kinds.length * poses.length);
        for (int group = 0; group < kinds.length; group++) {
            ModelBase m;
            switch (kinds[group]) {
                case 0: m = new ModelBiped(inflate[group], offsets[group]); break;
                case 1: m = new ModelZombie(); break;
                case 2: m = new ModelSkeleton(); break;
                case 3: m = new ModelQuadruped(legs[group], inflate[group]); break;
                case 4: m = new ModelPig(inflate[group]); break;
                case 5: m = new ModelCow(); break;
                case 6: m = new ModelSheep1(); break;
                case 7: m = new ModelSheep2(); break;
                case 8: m = new ModelChicken(); break;
                case 9: m = new ModelSpider(); break;
                case 10: m = new ModelCreeper(inflate[group]); break;
                case 11: m = new ModelSlime(0); break;
                case 12: m = new ModelSlime(16); break;
                case 13: m = new ModelSquid(); break;
                case 14: m = new ModelWolf(); break;
                case 15: m = new ModelGhast(); break;
                default: throw new AssertionError();
            }
            EntityWolf wolf = kinds[group] == 14 ? new EntityWolf(null) : null;
            for (int c = 0; c < poses.length; c++) {
                float[] p = poses[c];
                int flags = c == 1 ? 1 : c == 2 ? 30 : c == 3 ? 63 : c == 5 ? 12 : 0;
                float partial = c * .125F;
                float[] interest = {.1F * c, .2F * c};
                float[] shake = {.15F * c, .25F * c};
                m.onGround = attacks[c]; m.isRiding = (flags & 1) != 0;
                if (m instanceof ModelBiped) {
                    ModelBiped b = (ModelBiped)m;
                    b.isSneak = (flags & 2) != 0;
                    b.field_1279_h = (flags & 4) != 0; b.field_1278_i = (flags & 8) != 0;
                }
                if (wolf != null) {
                    wolf.setWolfAngry((flags & 16) != 0); wolf.setWolfSitting((flags & 32) != 0);
                    wolfField(wolf, "field_25054_c", interest[0]); wolfField(wolf, "field_25048_b", interest[1]);
                    wolfField(wolf, "prevTimeWolfIsShaking", shake[0]); wolfField(wolf, "timeWolfIsShaking", shake[1]);
                    m.setLivingAnimations(wolf, p[0], p[1], partial);
                }
                m.setRotationAngles(p[0],p[1],p[2],p[3],p[4],p[5]);
                o.i32(group); o.i32(kinds[group]); modelFloat(o, inflate[group]); modelFloat(o, offsets[group]); o.i32(legs[group]);
                for (float v : p) modelFloat(o,v);
                modelFloat(o,attacks[c]); o.i32(flags); modelFloat(o,partial);
                for (float v : interest) modelFloat(o,v); for (float v : shake) modelFloat(o,v);
                SortedMap<String,ModelRenderer> ps = modelParts(m); o.i32(ps.size());
                for (Map.Entry<String,ModelRenderer> e : ps.entrySet()) {
                    ModelRenderer r = e.getValue(); modelString(o,e.getKey());
                    for (String f : new String[]{"rotationPointX","rotationPointY","rotationPointZ","rotateAngleX","rotateAngleY","rotateAngleZ"})
                        modelFloat(o, ModelRenderer.class.getField(f).getFloat(r));
                    o.i32((r.mirror ? 1 : 0) | (r.showModel ? 2 : 0) | (r.field_1402_i ? 4 : 0));
                    PositionTextureVertex[] vs = (PositionTextureVertex[])corners.get(r); o.i32(vs.length);
                    for (PositionTextureVertex v : vs) modelVertex(o,v);
                    TexturedQuad[] qs = (TexturedQuad[])faces.get(r); o.i32(qs.length);
                    for (TexturedQuad q : qs) {
                        o.i32(q.vertexPositions.length);
                        for (PositionTextureVertex v : q.vertexPositions) modelVertex(o,v);
                        // The same Vec3D calls TexturedQuad.draw uses, followed by
                        // the original Tessellator's BYTE packing. No draw/GL call.
                        Vec3D edge1 = q.vertexPositions[1].vector3D.subtract(q.vertexPositions[0].vector3D);
                        Vec3D edge2 = q.vertexPositions[1].vector3D.subtract(q.vertexPositions[2].vector3D);
                        Vec3D n = edge2.crossProduct(edge1).normalize();
                        modelFloat(o,(float)n.xCoord); modelFloat(o,(float)n.yCoord); modelFloat(o,(float)n.zCoord);
                        Tessellator.instance.startDrawingQuads();
                        Tessellator.instance.setNormal((float)n.xCoord,(float)n.yCoord,(float)n.zCoord);
                        o.i32(normal.getInt(Tessellator.instance)); drawing.setBoolean(Tessellator.instance,false);
                    }
                }
            }
        }
        try (OutputStream os = new BufferedOutputStream(new FileOutputStream(out))) { o.writeTo(os); }
        System.out.println("models: " + kinds.length + " constructors, " + poses.length + " successive poses each");
    }

    // ---------------------------------------------------------------- raw terrain

    static void raw(long seed, int x0, int z0, int x1, int z1, String out) throws IOException {
        World world = newWorld(seed);
        ChunkProviderGenerate gen = new ChunkProviderGenerate(world, seed);
        WorldChunkManager wcm = world.getWorldChunkManager();
        try (OutputStream os = new BufferedOutputStream(new FileOutputStream(out))) {
            os.write("B173RAW1".getBytes("US-ASCII"));
            ByteBuffer bb = ByteBuffer.allocate(8 + 32768 + 16384 + 256 + 256 * 16).order(ByteOrder.LITTLE_ENDIAN);
            for (int cx = x0; cx <= x1; cx++) {
                for (int cz = z0; cz <= z1; cz++) {
                    Chunk c = gen.provideChunk(cx, cz);
                    // provideChunk leaves the chunk's own 16x16 climate in the shared WCM arrays
                    BiomeGenBase[] biomes = wcm.loadBlockGeneratorData(null, cx * 16, cz * 16, 16, 16);
                    bb.clear();
                    bb.putInt(cx).putInt(cz);
                    bb.put(c.blocks);
                    bb.put(c.data.data);
                    for (int i = 0; i < 256; i++) bb.put((byte) biomeId(biomes[i]));
                    for (int i = 0; i < 256; i++) bb.putDouble(wcm.temperature[i]);
                    for (int i = 0; i < 256; i++) bb.putDouble(wcm.humidity[i]);
                    os.write(bb.array(), 0, bb.position());
                }
            }
        }
    }

    /** MathHelper.SIN_TABLE as little-endian float bits. */
    static void sin(String out) throws Exception {
        Field f = MathHelper.class.getDeclaredField("SIN_TABLE");
        f.setAccessible(true);
        float[] t = (float[]) f.get(null);
        ByteBuffer bb = ByteBuffer.allocate(t.length * 4).order(ByteOrder.LITTLE_ENDIAN);
        for (float v : t) bb.putInt(Float.floatToRawIntBits(v));
        try (OutputStream os = new FileOutputStream(out)) { os.write(bb.array()); }
    }

    /**
     * Every Math.sin/Math.cos input WorldGenBigTree can produce (nextFloat() * 2.0 * 3.14159
     * for all 2^24 floats), plus the Math.pow(x, 2.0) inputs it uses. Written as raw
     * little-endian double bits: Math.sin, Math.cos, StrictMath.sin, StrictMath.cos
     * (2^24 each), then pow pairs (x, result).
     */
    static void mathCheck(String out) throws IOException {
        int n = 1 << 24;
        try (OutputStream os = new BufferedOutputStream(new FileOutputStream(out), 1 << 20)) {
            ByteBuffer bb = ByteBuffer.allocate(8).order(ByteOrder.LITTLE_ENDIAN);
            for (int pass = 0; pass < 4; pass++) {
                for (int i = 0; i < n; i++) {
                    double a = (double) ((float) i / (float) (1 << 24)) * 2.0D * 3.14159D;
                    double v = pass == 0 ? Math.sin(a) : pass == 1 ? Math.cos(a)
                             : pass == 2 ? StrictMath.sin(a) : StrictMath.cos(a);
                    bb.clear(); bb.putLong(Double.doubleToRawLongBits(v)); os.write(bb.array());
                }
            }
            List<Double> xs = new ArrayList<>();
            for (int i = 0; i <= 64; i++) { xs.add((double) i); xs.add(i + 0.5D); xs.add((double) ((float) i / 2.0F)); }
            for (int h = 0; h <= 64; h++) xs.add(1.0D * (double) h / 13.0D);
            for (double x : xs) {
                bb.clear(); bb.putLong(Double.doubleToRawLongBits(x)); os.write(bb.array());
                bb.clear(); bb.putLong(Double.doubleToRawLongBits(Math.pow(x, 2.0D))); os.write(bb.array());
            }
        }
    }

    // ---------------------------------------------------------------- block table

    static void blockTable(String out) throws Exception {
        List<Material> mats = new ArrayList<>();
        List<String> matNames = new ArrayList<>();
        for (Field f : Material.class.getDeclaredFields()) {
            if (Modifier.isStatic(f.getModifiers()) && f.getType() == Material.class) {
                mats.add((Material) f.get(null));
                matNames.add(f.getName());
            }
        }
        StringBuilder s = new StringBuilder();
        s.append("// @generated by tools/oracle (Oracle blocktable) from Mojang's b1.7.3 classes. Do not edit.\n\n");
        s.append("use super::{BlockProps, MaterialProps};\n\n");
        s.append("pub const MATERIALS: [MaterialProps; ").append(mats.size()).append("] = [\n");
        for (int i = 0; i < mats.size(); i++) {
            Material m = mats.get(i);
            s.append(String.format(
                "    MaterialProps { name: \"%s\", is_solid: %b, blocks_movement: %b, is_liquid: %b, burns: %b, ground_cover: %b, translucent_solid: %b, blocks_grass: %b },\n",
                matNames.get(i), m.isSolid(), m.getIsSolid(), m.getIsLiquid(), m.getBurning(), m.getIsGroundCover(),
                m.getIsTranslucent(), m.getCanBlockGrass()));
        }
        s.append("];\n\n");
        for (int i = 0; i < mats.size(); i++) {
            s.append("pub const MAT_").append(matNames.get(i).toUpperCase()).append(": u8 = ").append(i).append(";\n");
        }
        s.append("\npub const BLOCKS: [BlockProps; 256] = [\n");
        for (int id = 0; id < 256; id++) {
            Block b = Block.blocksList[id];
            if (b == null) {
                s.append(String.format("    BlockProps::NONE, // %d\n", id));
                continue;
            }
            int mat = mats.indexOf(b.blockMaterial);
            s.append(String.format(
                "    BlockProps { exists: true, material: %d, light_opacity: %d, light_value: %d, opaque_cube: %b, normal_render: %b, container: %b, ticks: %b, tick_rate: %d, name: \"%s\" }, // %d\n",
                mat, Block.lightOpacity[id], Block.lightValue[id], Block.opaqueCubeLookup[id], b.renderAsNormalBlock(),
                Block.isBlockContainer[id], Block.tickOnLoad[id], b.tickRate(), b.getBlockName(), id));
        }
        s.append("];\n");
        try (Writer w = new OutputStreamWriter(new FileOutputStream(out), "UTF-8")) { w.write(s.toString()); }
    }

    /** Static render data, with Fancy graphics (the b1.7.3 default) applied to leaves. */
    static void renderTable(String out) throws Exception {
        Block.leaves.setGraphicsLevel(true);
        String nl = System.lineSeparator().equals("\r\n") ? "\n" : "\n";
        StringBuilder s = new StringBuilder();
        s.append("// @generated by tools/oracle (Oracle rendertable) from Mojang's b1.7.3 classes. Do not edit.").append(nl);
        s.append("// Fancy graphics. textures[meta][side] = getBlockTextureFromSideAndMetadata(side, meta).").append(nl).append(nl);
        s.append("use super::RenderProps;").append(nl).append(nl);
        s.append("pub static RENDER: [RenderProps; 256] = [").append(nl);
        for (int id = 0; id < 256; id++) {
            Block b = Block.blocksList[id];
            if (b == null) {
                // Block.canBlockGrass[0] is set true explicitly (air); keep it.
                s.append(Block.canBlockGrass[id] ? "    RenderProps { can_block_grass: true, ..RenderProps::NONE }, // " : "    RenderProps::NONE, // ")
                 .append(id).append(nl);
                continue;
            }
            s.append(String.format("    RenderProps { exists: true, render_type: %d, pass: %d, opaque: %b, normal: %b, can_block_grass: %b, tex_index: %d, bounds: [%sf32, %sf32, %sf32, %sf32, %sf32, %sf32], textures: [",
                b.getRenderType(), b.getRenderBlockPass(), b.isOpaqueCube(), b.renderAsNormalBlock(), Block.canBlockGrass[id], b.blockIndexInTexture,
                (float) b.minX, (float) b.minY, (float) b.minZ, (float) b.maxX, (float) b.maxY, (float) b.maxZ));
            for (int meta = 0; meta < 16; meta++) {
                s.append("[");
                for (int side = 0; side < 6; side++) {
                    int t;
                    try { t = b.getBlockTextureFromSideAndMetadata(side, meta); }
                    catch (RuntimeException e) { t = 0; } // metadata the block never takes
                    if (t < 0 || t > 255) t = 0;
                    s.append(t).append(side < 5 ? "," : "");
                }
                s.append(meta < 15 ? "]," : "]");
            }
            s.append("] }, // ").append(id).append(" ").append(b.getBlockName()).append(nl);
        }
        s.append("];").append(nl);
        try (Writer w = new OutputStreamWriter(new FileOutputStream(out), "UTF-8")) { w.write(s.toString()); }
    }

    /**
     * Mesh chunks with the original RenderBlocks, the way WorldRenderer does: per 16-high
     * section, per pass, a ChunkCache over the section +-1, blocks in y, z, x order.
     * Smooth lighting and Fancy graphics are selected by the caller (default: both on).
     * The Tessellator's raw buffer is
     * read back instead of drawn; quads were stored as triangles (v0 v1 v2 v0 v2 v3).
     */
    static boolean meshNether;

    static void renderMesh(long seed, long wrand, String out, int[] cs, boolean fancy, boolean ao) throws Exception {
        meshSetup(fancy, ao);
        // Generate first (leaves opaque, as for the first world of a session), then switch
        // to Fancy the way RenderGlobal does before building chunk renderers.
        Block.leaves.setGraphicsLevel(false);
        World w = meshNether ? newNetherWorld(seed) : newWorld(seed);
        w.rand.setSeed(wrand);
        w.getInitialSpawnLocation();
        ChunkCoordinates sp = w.getSpawnPoint();
        for (int dx = -128; dx <= 128; dx += 16) {
            for (int dz = -128; dz <= 128; dz += 16) {
                w.getBlockId(sp.x + dx, 64, sp.z + dz);
                drainLighting(w);
            }
        }

        if (cs.length == 2 && cs[0] == Integer.MIN_VALUE) {
            int r = cs[1], scx = sp.x >> 4, scz = sp.z >> 4, k = 0;
            cs = new int[(2 * r + 1) * (2 * r + 1) * 2];
            for (int cx = scx - r; cx <= scx + r; cx++)
                for (int cz = scz - r; cz <= scz + r; cz++) { cs[k++] = cx; cs[k++] = cz; }
        }
        Block.leaves.setGraphicsLevel(fancy);
        RenderBlocks.fancyGrass = fancy;
        // The client loads the biome color maps at startup (RenderEngine -> Colorizer*).
        ColorizerGrass.func_28181_a(jarImage("misc/grasscolor.png"));
        ColorizerFoliage.func_28152_a(jarImage("misc/foliagecolor.png"));
        dumpMeshes(w, cs, out);
    }

    /** A bare Minecraft with the graphics settings RenderBlocks reads. */
    static void meshSetup(boolean fancy, boolean ao) throws Exception {
        // Minecraft.isAmbientOcclusionEnabled() reads a Minecraft instance: make a bare one.
        Field uf = sun.misc.Unsafe.class.getDeclaredField("theUnsafe");
        uf.setAccessible(true);
        sun.misc.Unsafe unsafe = (sun.misc.Unsafe) uf.get(null);
        net.minecraft.client.Minecraft mc = (net.minecraft.client.Minecraft) unsafe.allocateInstance(MinecraftImpl.class);
        mc.gameSettings = new GameSettings();
        mc.gameSettings.ambientOcclusion = ao;
        mc.gameSettings.fancyGraphics = fancy;
        Field tm = net.minecraft.client.Minecraft.class.getDeclaredField("theMinecraft");
        tm.setAccessible(true);
        tm.set(null, mc);

    }

    /** Mesh chunks with the original RenderBlocks, section by section like WorldRenderer. */
    static void dumpMeshes(World w, int[] cs, String out) throws Exception {
        Tessellator t = Tessellator.instance;
        Field fRaw = Tessellator.class.getDeclaredField("rawBuffer");
        Field fIdx = Tessellator.class.getDeclaredField("rawBufferIndex");
        Field fDrawing = Tessellator.class.getDeclaredField("isDrawing");
        fRaw.setAccessible(true);
        fIdx.setAccessible(true);
        fDrawing.setAccessible(true);
        DataOut o = new DataOut();
        o.bytes("B173MSH1".getBytes("US-ASCII"));
        o.i32(w.skylightSubtracted);
        o.i32(cs.length / 2);
        for (int k = 0; k < cs.length; k += 2) {
            int cx = cs[k], cz = cs[k + 1];
            o.i32(cx); o.i32(cz);
            for (int pass = 0; pass < 2; pass++) {
                t.startDrawingQuads();
                t.setTranslationD((double) (-cx * 16), 0.0D, (double) (-cz * 16));
                for (int sy = 0; sy < 8; sy++) {
                    int x0 = cx * 16, y0 = sy * 16, z0 = cz * 16;
                    ChunkCache cache = new ChunkCache(w, x0 - 1, y0 - 1, z0 - 1, x0 + 16 + 1, y0 + 16 + 1, z0 + 16 + 1);
                    RenderBlocks rb = new RenderBlocks(cache);
                    for (int y = y0; y < y0 + 16; y++) {
                        for (int z = z0; z < z0 + 16; z++) {
                            for (int x = x0; x < x0 + 16; x++) {
                                int id = cache.getBlockId(x, y, z);
                                if (id > 0) {
                                    Block b = Block.blocksList[id];
                                    if (b.getRenderBlockPass() == pass) rb.renderBlockByRenderType(b, x, y, z);
                                }
                            }
                        }
                    }
                }
                int[] raw = (int[]) fRaw.get(t);
                int n = fIdx.getInt(t) / 8;
                int quads = n / 6;
                o.i32(quads * 4);
                int[] pick = {0, 1, 2, 5};
                for (int q = 0; q < quads; q++) {
                    for (int v : pick) {
                        int base = (q * 6 + v) * 8;
                        for (int f = 0; f < 6; f++) o.i32(raw[base + f]);
                    }
                }
                fDrawing.setBoolean(t, false);
            }
        }
        try (OutputStream os = new BufferedOutputStream(new FileOutputStream(out))) { o.writeTo(os); }
    }

    /** ARGB pixels of an image inside the player's b1.7.3 client jar. */
    static int[] jarImage(String name) throws IOException {
        String jarPath = System.getenv("BETA_JAR");
        if (jarPath == null) jarPath = new File(new File(new File(new File(System.getenv("APPDATA"), ".minecraft"), "versions"), "b1.7.3"), "b1.7.3.jar").getPath();
        try (java.util.zip.ZipFile z = new java.util.zip.ZipFile(jarPath)) {
            java.awt.image.BufferedImage img = javax.imageio.ImageIO.read(z.getInputStream(z.getEntry(name)));
            int w = img.getWidth(), h = img.getHeight();
            return img.getRGB(0, 0, w, h, null, 0, w);
        }
    }

    /** SaveHandler for an arbitrary world directory. */
    static SaveHandler saveHandler(String dir) {
        File d = new File(dir).getAbsoluteFile();
        return new SaveOldDir(d.getParentFile(), d.getName(), false);
    }

    /** McRegion save handler that reports a preset WorldInfo, so no unseeded spawn search runs. */
    static SaveHandler presetSaveHandler(String dir, final long seed) {
        File d = new File(dir).getAbsoluteFile();
        final String name = d.getName();
        return new SaveOldDir(d.getParentFile(), name, false) {
            public WorldInfo loadWorldInfo() { return new WorldInfo(seed, name); }
        };
    }

    static void savePreload(long seed, long wrand, String dir, String out) throws Exception {
        World w = new World(presetSaveHandler(dir, seed), new File(dir).getName(), seed);
        w.rand.setSeed(wrand);
        w.getInitialSpawnLocation();
        ChunkCoordinates sp = w.getSpawnPoint();
        for (int dx = -128; dx <= 128; dx += 16) {
            for (int dz = -128; dz <= 128; dz += 16) {
                w.getBlockId(sp.x + dx, 64, sp.z + dz);
                drainLighting(w);
            }
        }
        w.saveWorld(true, null);
        dumpWorld(w, out, sp.x, sp.y, sp.z);
    }

    static void readSave(String dir, String out) throws Exception {
        World w = new World(saveHandler(dir), new File(dir).getName(), 0L);
        File region = new File(dir, "region");
        java.util.List<int[]> chunks = new java.util.ArrayList<>();
        for (File f : region.listFiles()) {
            String[] parts = f.getName().split("\\.");
            if (parts.length != 4 || !parts[3].equals("mcr")) continue;
            int rx = Integer.parseInt(parts[1]), rz = Integer.parseInt(parts[2]);
            try (java.io.RandomAccessFile raf = new java.io.RandomAccessFile(f, "r")) {
                for (int i = 0; i < 1024; i++)
                    if (raf.readInt() != 0) chunks.add(new int[] {rx * 32 + (i & 31), rz * 32 + (i >> 5)});
            }
        }
        chunks.sort((p, q) -> p[0] != q[0] ? Integer.compare(p[0], q[0]) : Integer.compare(p[1], q[1]));
        for (int[] c : chunks) w.getChunkFromChunkCoords(c[0], c[1]);
        ChunkCoordinates sp = w.getSpawnPoint();
        dumpWorld(w, out, sp.x, sp.y, sp.z);
    }

    // ---------------------------------------------------------------- world scenarios

    static void drainLighting(World w) {
        while (w.updatingLighting()) {
        }
    }

    static void grid(long seed, long wrand, int x0, int z0, int x1, int z1, String light, String out) throws Exception {
        World w = newWorld(seed);
        w.rand.setSeed(wrand);
        for (int cx = x0; cx <= x1; cx++) {
            for (int cz = z0; cz <= z1; cz++) {
                w.getChunkFromChunkCoords(cx, cz);
                if (light.equals("each")) drainLighting(w);
            }
        }
        if (light.equals("end")) drainLighting(w);
        ChunkCoordinates sp = w.getSpawnPoint();
        dumpWorld(w, out, sp.x, sp.y, sp.z);
    }

    /** Minecraft.startWorld for a new world, through Minecraft.func_6255_d ("Building terrain"). */
    static void preload(long seed, long wrand, String out) throws Exception {
        World w = newWorld(seed);
        w.rand.setSeed(wrand);
        w.getInitialSpawnLocation();
        ChunkCoordinates sp = w.getSpawnPoint();
        short r = 128;
        for (int dx = -r; dx <= r; dx += 16) {
            for (int dz = -r; dz <= r; dz += 16) {
                w.getBlockId(sp.x + dx, 64, sp.z + dz);
                drainLighting(w);
            }
        }
        dumpWorld(w, out, sp.x, sp.y, sp.z);
    }

    static void ticks(long seed, long wrand, int lcg, int sound, int n, String weather,
                      double dx, double dz, String out, String fires) throws Exception {
        World w = newWorld(seed);
        w.rand.setSeed(wrand);
        w.getInitialSpawnLocation();
        ChunkCoordinates sp = w.getSpawnPoint();
        for (int x = -128; x <= 128; x += 16) {
            for (int z = -128; z <= 128; z += 16) {
                w.getBlockId(sp.x + x, 64, sp.z + z);
                drainLighting(w);
            }
        }
        w.field_9437_g = lcg;
        Field sc = World.class.getDeclaredField("soundCounter");
        sc.setAccessible(true);
        sc.setInt(w, sound);
        if (!weather.equals("clear")) {
            w.worldInfo.setRaining(true);
            w.worldInfo.setRainTime(1000000);
            w.rainingStrength = w.prevRainingStrength = 1.0F;
            if (weather.equals("thunder")) {
                w.worldInfo.setThundering(true);
                w.worldInfo.setThunderTime(1000000);
                w.thunderingStrength = w.prevThunderingStrength = 1.0F;
            }
        }
        w.skylightSubtracted = w.calculateSkylightSubtracted(1.0F);
        Field uf = sun.misc.Unsafe.class.getDeclaredField("theUnsafe");
        uf.setAccessible(true);
        sun.misc.Unsafe U = (sun.misc.Unsafe) uf.get(null);
        EntityPlayer pl = (EntityPlayer) U.allocateInstance(EntityPlayerSP.class);
        pl.posX = sp.x + 0.5 + dx;
        pl.posY = sp.y + 1.62;
        pl.posZ = sp.z + 0.5 + dz;
        w.playerEntities.add(pl);
        w.setAllowedMobSpawns(false, false);
        for (String f : fires.split(",")) {
            if (f.isEmpty()) continue;
            String[] xz = f.split(":");
            int fx = sp.x + Integer.parseInt(xz[0]), fz = sp.z + Integer.parseInt(xz[1]);
            if (xz.length >= 3) {
                // dx:dz:id[:meta] places that block on top instead of fire.
                w.setBlockAndMetadataWithNotify(fx, w.getHeightValue(fx, fz), fz, Integer.parseInt(xz[2]),
                    xz.length >= 4 ? Integer.parseInt(xz[3]) : 0);
            } else {
                w.setBlockWithNotify(fx, w.getHeightValue(fx, fz), fz, Block.fire.blockID);
            }
        }
        drainLighting(w);
        OracleHooks.reset(100);EntityOracle.mathRandom().setSeed(200);WorldEventOracle.attach(w);
        try(DataOutputStream events=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(out+".events")))) {
            for (int t = 0; t < n; t++) {
                w.tick();drainLighting(w);WorldEventOracle.snapshot(w,events);
            }
        }
        dumpWorld(w, out, sp.x, sp.y, sp.z);
        try (PrintWriter pw = new PrintWriter(out + ".txt")) {
            pw.println("time " + w.getWorldTime());
            pw.println("lcg " + w.field_9437_g);
            pw.println("sound " + sc.getInt(w));
            pw.println("rain " + w.worldInfo.getRaining() + " " + w.worldInfo.getRainTime() + " " + Float.floatToIntBits(w.rainingStrength));
            pw.println("thunder " + w.worldInfo.getThundering() + " " + w.worldInfo.getThunderTime() + " " + Float.floatToIntBits(w.thunderingStrength));
            pw.println("skylight " + w.skylightSubtracted);
            pw.println("flash " + w.field_27168_F + " bolts " + w.weatherEffects.size());
            pw.println("rand " + w.rand.nextLong());
        }
    }

    @SuppressWarnings("unchecked")
    static void dumpWorld(World w, String out, int sx, int sy, int sz) throws Exception {
        try (OutputStream os = new BufferedOutputStream(new FileOutputStream(out))) { worldBytes(w,sx,sy,sz).writeTo(os); }
    }
    static DataOut worldBytes(World w, int sx, int sy, int sz) throws Exception {
        List<Chunk> chunks;
        if(w.chunkProvider instanceof EntityOracle.FlatProvider)chunks=new ArrayList<>(((EntityOracle.FlatProvider)w.chunkProvider).chunks.values());
        else {Field cmF = ChunkProvider.class.getDeclaredField("chunkMap");cmF.setAccessible(true);Map<Integer,Chunk> map=(Map<Integer,Chunk>)cmF.get(w.chunkProvider);chunks=new ArrayList<>(map.values());}
        chunks.sort((p, q) -> p.xPosition != q.xPosition ? Integer.compare(p.xPosition, q.xPosition)
                                                         : Integer.compare(p.zPosition, q.zPosition));
        DataOut o = new DataOut();
        o.bytes("B173WLD1".getBytes("US-ASCII"));
        o.i32(sx); o.i32(sy); o.i32(sz);
        o.i32(chunks.size());
        for (Chunk c : chunks) {
            o.i32(c.xPosition); o.i32(c.zPosition);
            o.u8(c.isTerrainPopulated ? 1 : 0);
            o.bytes(c.blocks);
            o.bytes(c.data.data);
            o.bytes(c.skylightMap.data);
            o.bytes(c.blocklightMap.data);
            o.bytes(c.heightMap);
            List<TileEntity> tes = new ArrayList<>((Collection<TileEntity>) c.chunkTileEntityMap.values());
            tes.sort((p, q) -> p.xCoord != q.xCoord ? Integer.compare(p.xCoord, q.xCoord)
                             : p.yCoord != q.yCoord ? Integer.compare(p.yCoord, q.yCoord)
                             : Integer.compare(p.zCoord, q.zCoord));
            o.i32(tes.size());
            for (TileEntity te : tes) {
                o.i32(te.xCoord); o.i32(te.yCoord); o.i32(te.zCoord);
                if (te instanceof TileEntityChest) {
                    TileEntityChest ch = (TileEntityChest) te;
                    o.u8(1);
                    for (int i = 0; i < 27; i++) {
                        ItemStack st = ch.getStackInSlot(i);
                        if (st == null) { o.i16(-1); o.u8(0); o.i16(0); }
                        else { o.i16(st.itemID); o.u8(st.stackSize); o.i16(st.getItemDamage()); }
                    }
                } else if (te instanceof TileEntityMobSpawner) {
                    o.u8(2);
                    byte[] id = ((TileEntityMobSpawner) te).getMobID().getBytes("US-ASCII");
                    o.u8(id.length); o.bytes(id);
                } else if(te instanceof TileEntityNote) {
                    TileEntityNote n=(TileEntityNote)te;o.u8(5);o.u8(n.note);o.u8(n.previousRedstoneState?1:0);
                } else if(te instanceof TileEntityPiston) {
                    TileEntityPiston p=(TileEntityPiston)te;o.u8(6);o.i32(p.getStoredBlockID());o.i32(p.getBlockMetadata());o.i32(p.func_31009_d());
                    Field progress=TileEntityPiston.class.getDeclaredField("field_31022_k");progress.setAccessible(true);
                    o.i32(Float.floatToRawIntBits(progress.getFloat(p)));o.i32(Float.floatToRawIntBits(p.func_31008_a(0)));
                    o.u8(p.func_31015_b()?1:0);o.u8(p.func_31012_k()?1:0);
                } else if(te instanceof TileEntityFurnace||te instanceof TileEntityDispenser) {
                    boolean furnace=te instanceof TileEntityFurnace;o.u8(furnace?3:4);IInventory inv=(IInventory)te;
                    for(int i=0;i<inv.getSizeInventory();i++){ItemStack st=inv.getStackInSlot(i);o.i16(st==null?-1:st.itemID);o.u8(st==null?0:st.stackSize);o.i16(st==null?0:st.getItemDamage());}
                    if(furnace){TileEntityFurnace f=(TileEntityFurnace)te;o.i16(f.furnaceBurnTime);o.i16(f.furnaceCookTime);}
                } else {
                    throw new IllegalStateException("unexpected tile entity " + te);
                }
            }
        }
        // Pending (non-immediate) block ticks, in processing order (time, then entry id).
        Field tts = World.class.getDeclaredField("scheduledTickTreeSet");
        tts.setAccessible(true);
        List<NextTickListEntry> ticks = new ArrayList<>((Collection<NextTickListEntry>) tts.get(w));
        o.i32(ticks.size());
        for (NextTickListEntry t : ticks) {
            o.i32(t.xCoord); o.i32(t.yCoord); o.i32(t.zCoord); o.i32(t.blockID); o.i64(t.scheduledTime);
        }
        Field ltu = World.class.getDeclaredField("lightingToUpdate");
        ltu.setAccessible(true);
        o.i32(((List<?>) ltu.get(w)).size());
        return o;
    }

    static class DataOut {
        final ByteArrayOutputStream buf = new ByteArrayOutputStream();
        final ByteBuffer bb = ByteBuffer.allocate(8).order(ByteOrder.LITTLE_ENDIAN);
        void u8(int v) { buf.write(v); }
        void i16(int v) { bb.clear(); bb.putShort((short) v); buf.write(bb.array(), 0, 2); }
        void i32(int v) { bb.clear(); bb.putInt(v); buf.write(bb.array(), 0, 4); }
        void i64(long v) { bb.clear(); bb.putLong(v); buf.write(bb.array(), 0, 8); }
        void bytes(byte[] b) { buf.write(b, 0, b.length); }
        void writeTo(OutputStream os) throws IOException { buf.writeTo(os); }
    }
    // ---------------------------------------------------------------- Nether
    static World newNetherWorld(long seed) {
        return new World(new NullSaveHandler(seed), "oracle", seed, WorldProvider.getProviderForDimension(-1));
    }

    static void rawNether(long seed, int x0, int z0, int x1, int z1, String out) throws IOException {
        World world = newNetherWorld(seed);
        ChunkProviderHell gen = new ChunkProviderHell(world, seed);
        WorldChunkManager wcm = world.getWorldChunkManager();
        try (OutputStream os = new BufferedOutputStream(new FileOutputStream(out))) {
            os.write("B173RAW1".getBytes("US-ASCII"));
            ByteBuffer bb = ByteBuffer.allocate(8 + 32768 + 16384 + 256 + 256 * 16).order(ByteOrder.LITTLE_ENDIAN);
            for (int cx = x0; cx <= x1; cx++) {
                for (int cz = z0; cz <= z1; cz++) {
                    Chunk c = gen.provideChunk(cx, cz);
                    // WorldChunkManagerHell supplies constant biome/climate arrays.
                    BiomeGenBase[] biomes = wcm.loadBlockGeneratorData(null, cx * 16, cz * 16, 16, 16);
                    bb.clear();
                    bb.putInt(cx).putInt(cz);
                    bb.put(c.blocks);
                    bb.put(c.data.data);
                    for (int i = 0; i < 256; i++) bb.put((byte) biomeId(biomes[i]));
                    for (int i = 0; i < 256; i++) bb.putDouble(wcm.temperature[i]);
                    for (int i = 0; i < 256; i++) bb.putDouble(wcm.humidity[i]);
                    os.write(bb.array(), 0, bb.position());
                }
            }
        }
    }

    static void gridNether(long seed, long wrand, int x0, int z0, int x1, int z1, String light, String out) throws Exception {
        World w = newNetherWorld(seed);
        w.rand.setSeed(wrand);
        for (int cx = x0; cx <= x1; cx++) {
            for (int cz = z0; cz <= z1; cz++) {
                w.getChunkFromChunkCoords(cx, cz);
                if (light.equals("each")) drainLighting(w);
            }
        }
        if (light.equals("end")) drainLighting(w);
        ChunkCoordinates sp = w.getSpawnPoint();
        dumpWorld(w, out, sp.x, sp.y, sp.z);
    }


    static Object allocate(Class<?> cls) throws Exception {
        Field f = sun.misc.Unsafe.class.getDeclaredField("theUnsafe"); f.setAccessible(true);
        return ((sun.misc.Unsafe) f.get(null)).allocateInstance(cls);
    }
    static void setField(Object o, String name, Object v) throws Exception {
        Field f = o.getClass().getDeclaredField(name); f.setAccessible(true); f.set(o, v);
    }
    static EntityRenderer headlessRenderer(World w, Entity e) throws Exception {
        net.minecraft.client.Minecraft mc = (net.minecraft.client.Minecraft) allocate(MinecraftImpl.class);
        EntityLiving camera = (EntityLiving) allocate(EntityPlayerSP.class);
        camera.worldObj = w;
        camera.posX = camera.lastTickPosX = e.posX;
        camera.posY = camera.lastTickPosY = e.posY;
        camera.posZ = camera.lastTickPosZ = e.posZ;
        Field box = Entity.class.getDeclaredField("boundingBox"); box.setAccessible(true);
        box.set(camera, AxisAlignedBB.getBoundingBox(e.posX-.3, e.posY, e.posZ-.3, e.posX+.3, e.posY+1.8, e.posZ+.3));
        mc.theWorld = w; mc.renderViewEntity = camera;
        mc.gameSettings = (GameSettings) allocate(GameSettings.class);
        mc.gameSettings.fancyGraphics = true; mc.gameSettings.renderDistance = 0;
        mc.renderEngine = (RenderEngine) allocate(RenderEngine.class);
        HashMap<String,Integer> textures = new HashMap<String,Integer>();
        textures.put("/environment/snow.png", 0); textures.put("/environment/rain.png", 1);
        setField(mc.renderEngine, "textureMap", textures);
        setField(mc.renderEngine, "texturePack", allocate(TexturePackList.class));
        mc.effectRenderer = new EffectRenderer(w, mc.renderEngine);
        EntityRenderer renderer = (EntityRenderer) allocate(EntityRenderer.class);
        setField(renderer, "mc", mc); setField(renderer, "random", new Random(0));
        setField(renderer, "fogColor1", 1.0F); setField(renderer, "fogColor2", 1.0F);
        return renderer;
    }
    static ArrayList<float[]> skyColors;
    public static void recordSkyColor(float r,float g,float b) { if(skyColors!=null)skyColors.add(new float[]{r,g,b}); }
    static void skyDraw(String out) throws Exception {
        World w=newWorld(42L);
        Entity view=new EntityItem(w,0.5,70,0.5,new ItemStack(1,1,0));
        EntityRenderer renderer=headlessRenderer(w,view);
        net.minecraft.client.Minecraft mc=(net.minecraft.client.Minecraft)getField(renderer,"mc");
        ((HashMap)getField(mc.renderEngine,"textureMap")).put("/terrain/sun.png",2);
        ((HashMap)getField(mc.renderEngine,"textureMap")).put("/terrain/moon.png",3);
        RenderGlobal global=(RenderGlobal)allocate(RenderGlobal.class);
        setField(global,"mc",mc);setField(global,"worldObj",w);setField(global,"renderEngine",mc.renderEngine);
        Field convert=Tessellator.class.getDeclaredField("convertQuadsToTriangles");convert.setAccessible(true);convert.setBoolean(null,false);
        DataOut o=new DataOut();o.bytes("B173SKY1".getBytes("US-ASCII"));o.i32(512);
        for(int c=0;c<512;c++) {
            int time=(c/2)*97,flash=c%7==0?2:0;
            float partial=(c%4)*0.25F,rain=(c%5)*0.25F,thunder=(c%3)*0.5F;
            double x=c*31.125-8000,z=(c%23)*-73.75;
            w.setWorldTime(time);w.rainingStrength=w.prevRainingStrength=rain;w.thunderingStrength=w.prevThunderingStrength=thunder;w.field_27172_i=flash;
            mc.renderViewEntity.posX=x;mc.renderViewEntity.posZ=z;mc.gameSettings.anaglyph=c%2==0;
            o.i64(Double.doubleToLongBits(x));o.i64(Double.doubleToLongBits(z));o.i32(time);o.i32(flash);
            modelFloat(o,partial);modelFloat(o,rain);modelFloat(o,thunder);o.i32(mc.gameSettings.anaglyph?1:0);
            skyColors=new ArrayList<float[]>();weatherVertices.clear();
            global.renderSky(partial);
            o.i32(skyColors.size());for(float[] rgb:skyColors)for(float f:rgb)modelFloat(o,f);
            o.i32(weatherVertices.size());for(float[] vertex:weatherVertices)for(float f:vertex)modelFloat(o,f);
            skyColors=null;
        }
        try(OutputStream os=new FileOutputStream(out)){o.writeTo(os);}
    }
    static final ArrayList<float[]> weatherVertices = new ArrayList<float[]>();
    public static void recordWeatherDraw(int mode, int count, float[] color, int texture) {
        try {
            Tessellator t = Tessellator.instance;
            Field rawF = Tessellator.class.getDeclaredField("rawBuffer"); rawF.setAccessible(true);
            Field colF = Tessellator.class.getDeclaredField("hasColor"); colF.setAccessible(true);
            int[] raw = (int[]) rawF.get(t);
            ArrayList<float[]> strip = new ArrayList<float[]>();
            for (int i=0; i<count; i++) {
                int o=i*8;
                float[] v={texture,Float.intBitsToFloat(raw[o]),Float.intBitsToFloat(raw[o+1]),Float.intBitsToFloat(raw[o+2]),
                    mode==5?0:Float.intBitsToFloat(raw[o+3]),mode==5?0:Float.intBitsToFloat(raw[o+4]),color[0],color[1],color[2],color[3]};
                if (colF.getBoolean(t)) for (int c=0;c<4;c++) v[6+c]=(float)(raw[o+5] >>> (c*8) & 255)/255.0F;
                strip.add(v);
            }
            if(mode==5) for(int i=2;i<strip.size();i++) {
                weatherVertices.add(strip.get(i%2==0?i-2:i-1));
                weatherVertices.add(strip.get(i%2==0?i-1:i-2)); weatherVertices.add(strip.get(i));
            } else weatherVertices.addAll(strip);
        } catch(Exception e) { throw new RuntimeException(e); }
    }
    static final ArrayList<double[]> rainSounds = new ArrayList<double[]>();
    static void videoWindow(String[] a) throws Exception {
        Field convert=Tessellator.class.getDeclaredField("convertQuadsToTriangles");convert.setAccessible(true);convert.setBoolean(null,false);
        net.minecraft.client.Minecraft mc=(net.minecraft.client.Minecraft)allocate(MinecraftImpl.class);
        mc.gameSettings=new GameSettings();mc.gameSettings.renderDistance=Integer.parseInt(a[1]);
        RenderGlobal global=(RenderGlobal)allocate(RenderGlobal.class);
        setField(global,"mc",mc);setField(global,"worldRenderersToUpdate",new ArrayList());
        setField(global,"tileEntities",new ArrayList());
        // Record display-list compilation as lists, not ambient geometry draws.
        RendererOracle.active=true;
        global.loadRenderers();
        java.lang.reflect.Method move=RenderGlobal.class.getDeclaredMethod("markRenderersForNewPosition",int.class,int.class,int.class);
        move.setAccessible(true);int p=Integer.parseInt(a[2]);move.invoke(global,p,0,p);
        RendererOracle.active=false;
        int wide=(Integer)getField(global,"renderChunksWide");
        WorldRenderer[] renderers=(WorldRenderer[])getField(global,"worldRenderers");
        StringBuilder line=new StringBuilder("window "+wide);
        for(int i=0;i<wide;i++)line.append(" ").append(renderers[i].posX >> 4);
        System.out.println(line);
    }
    static void cloudMesh(String[] a) throws Exception {
        long seed=Long.parseLong(a[1]);
        double x=Double.parseDouble(a[2]),y=Double.parseDouble(a[3]),z=Double.parseDouble(a[4]);
        World w=newWorld(seed);w.setWorldTime(Long.parseLong(a[7]));
        w.rainingStrength=w.prevRainingStrength=Float.parseFloat(a[8]);
        w.thunderingStrength=w.prevThunderingStrength=Float.parseFloat(a[9]);
        EntityRenderer r=headlessRenderer(w,new EntityItem(w,x,y,z,new ItemStack(1,1,0)));
        Field mcF=EntityRenderer.class.getDeclaredField("mc");mcF.setAccessible(true);
        net.minecraft.client.Minecraft mc=(net.minecraft.client.Minecraft)mcF.get(r);
        mc.gameSettings.fancyGraphics=!a[10].equals("fast");
        mc.gameSettings.anaglyph=a.length>12 && a[12].equals("anaglyph");
        mc.renderViewEntity.prevPosX=x;mc.renderViewEntity.prevPosY=y;mc.renderViewEntity.prevPosZ=z;
        ((HashMap) getField(mc.renderEngine,"textureMap")).put("/environment/clouds.png",2);
        RenderGlobal global=(RenderGlobal)allocate(RenderGlobal.class);
        setField(global,"mc",mc);setField(global,"worldObj",w);setField(global,"renderEngine",mc.renderEngine);
        setField(global,"cloudOffsetX",Integer.parseInt(a[5]));
        Field convert=Tessellator.class.getDeclaredField("convertQuadsToTriangles");convert.setAccessible(true);convert.setBoolean(null,false);
        global.renderClouds(Float.parseFloat(a[6]));
        DataOut o=new DataOut();o.bytes("B173CLD1".getBytes("US-ASCII"));o.i32(weatherVertices.size());
        for(float[] v:weatherVertices)for(float f:v)o.i32(Float.floatToIntBits(f));
        try(OutputStream os=new FileOutputStream(a[11])) {o.writeTo(os);}
    }
    static Object getField(Object o,String name) throws Exception {
        Field f=o.getClass().getDeclaredField(name);f.setAccessible(true);return f.get(o);
    }
    static void weatherMesh(String[] a) throws Exception {
        long seed=Long.parseLong(a[1]);
        double x=Double.parseDouble(a[2]),y=Double.parseDouble(a[3]),z=Double.parseDouble(a[4]);
        int tick=Integer.parseInt(a[5]); float partial=Float.parseFloat(a[6]), rain=Float.parseFloat(a[7]);
        World w=newWorld(seed); w.setWorldTime(6000L); w.rainingStrength=w.prevRainingStrength=rain;
        w.skylightSubtracted=w.calculateSkylightSubtracted(1.0F);
        int cx=MathHelper.floor_double(x),cz=MathHelper.floor_double(z);
        // Explicit x-major loading makes lighting independent of renderer queries.
        for(int xx=(cx-11)>>4;xx<=(cx+11)>>4;xx++) for(int zz=(cz-11)>>4;zz<=(cz+11)>>4;zz++) {w.getBlockId(xx*16,64,zz*16);drainLighting(w);}
        String scene=a.length>9?a[9]:"natural";
        int soundCounter=a.length>10?Integer.parseInt(a[10]):0;
        if(!scene.equals("natural")) for(int xx=cx-10;xx<=cx+10;xx++) for(int zz=cz-10;zz<=cz+10;zz++) {
            int yy=MathHelper.floor_double(y)+(scene.equals("roof")?4:-1);
            w.setBlock(xx,yy,zz,scene.equals("lava") && ((xx+zz)&1)==0?11:1);
        }
        drainLighting(w);
        w.addWorldAccess((IWorldAccess)java.lang.reflect.Proxy.newProxyInstance(Oracle.class.getClassLoader(),new Class[]{IWorldAccess.class},
            (proxy,method,args)->{if(method.getName().equals("playSound"))rainSounds.add(new double[]{(Double)args[1],(Double)args[2],(Double)args[3],(Float)args[4],(Float)args[5]});return null;}));
        Entity e=new EntityItem(w,x,y,z,new ItemStack(1,1,0));
        EntityRenderer r=headlessRenderer(w,e);
        Field weatherMc=EntityRenderer.class.getDeclaredField("mc");weatherMc.setAccessible(true);
        ((net.minecraft.client.Minecraft)weatherMc.get(r)).gameSettings.fancyGraphics=!(a.length>11 && a[11].equals("fast"));
        setField(r,"rendererUpdateCount",tick);setField(r,"rainSoundCounter",soundCounter);
        DataOut o=new DataOut();o.bytes("B173WTH1".getBytes("US-ASCII"));o.i32(soundCounter);
        BiomeGenBase[] biomes=w.getWorldChunkManager().func_4069_a(cx-10,cz-10,21,21);int k=0;
        for(int xx=cx-10;xx<=cx+10;xx++) for(int zz=cz-10;zz<=cz+10;zz++) {
            o.i32(xx);o.i32(zz);o.i32(w.findTopSolidBlock(xx,zz));
            BiomeGenBase b=biomes[k++];o.u8(b.getEnableSnow()?0:b.canSpawnLightningBolt()?1:2);
            o.u8(w.getWorldChunkManager().getBiomeGenAt(xx,zz).canSpawnLightningBolt()?1:0);
            for(int yy=0;yy<128;yy++)o.u8(w.getBlockId(xx,yy,zz));
            for(int yy=0;yy<128;yy++)o.u8(w.getBlockMetadata(xx,yy,zz));
            for(int yy=0;yy<=128;yy++)o.i32(Float.floatToIntBits(w.getLightBrightness(xx,yy,zz)));
        }
        Field convert=Tessellator.class.getDeclaredField("convertQuadsToTriangles");convert.setAccessible(true);convert.setBoolean(null,false);
        r.renderRainSnow(partial);
        java.lang.reflect.Method particles=EntityRenderer.class.getDeclaredMethod("addRainParticles");particles.setAccessible(true);particles.invoke(r);
        Field mcF=EntityRenderer.class.getDeclaredField("mc");mcF.setAccessible(true);
        net.minecraft.client.Minecraft mc=(net.minecraft.client.Minecraft)mcF.get(r);
        Field layersF=EffectRenderer.class.getDeclaredField("fxLayers");layersF.setAccessible(true);
        List[] layers=(List[])layersF.get(mc.effectRenderer);
        o.i32(layers[0].size());
        for(Object obj:layers[0]) {
            EntityFX fx=(EntityFX)obj;o.u8(fx instanceof EntitySmokeFX?1:0);
            o.i64(Double.doubleToLongBits(fx.posX));o.i64(Double.doubleToLongBits(fx.posY));o.i64(Double.doubleToLongBits(fx.posZ));
        }
        Field randomF=EntityRenderer.class.getDeclaredField("random");randomF.setAccessible(true);
        o.i64(((Random)randomF.get(r)).nextLong());
        Field soundF=EntityRenderer.class.getDeclaredField("rainSoundCounter");soundF.setAccessible(true);o.i32(soundF.getInt(r));
        o.i32(rainSounds.size());
        for(double[] sound:rainSounds) {
            for(int i=0;i<3;i++)o.i64(Double.doubleToLongBits(sound[i]));
            o.i32(Float.floatToIntBits((float)sound[3]));o.i32(Float.floatToIntBits((float)sound[4]));
        }
        EntityLightningBolt bolt=(EntityLightningBolt)allocate(EntityLightningBolt.class);bolt.field_27029_a=seed;
        org.lwjgl.opengl.GL11.glBindTexture(3553,3);
        new RenderLightningBolt().doRender(bolt,0.25,10.5,-2.75,0.0F,partial);
        o.i32(weatherVertices.size());
        for(float[] v:weatherVertices)for(float f:v)o.i32(Float.floatToIntBits(f));
        Random gaussian=new Random(seed);
        for(int i=0;i<256;i++)o.i64(Double.doubleToLongBits(gaussian.nextGaussian()));
        gaussian.setSeed(seed);
        for(int i=0;i<3;i++)o.i64(Double.doubleToLongBits(gaussian.nextGaussian()));
        gaussian.setSeed(seed);
        o.i64(Double.doubleToLongBits(gaussian.nextGaussian()));
        try(OutputStream os=new FileOutputStream(a[8])) { o.writeTo(os); }
    }
}
