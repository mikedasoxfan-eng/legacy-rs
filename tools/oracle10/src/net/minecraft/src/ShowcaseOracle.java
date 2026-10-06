package net.minecraft.src;

import java.io.*;
import java.util.*;

/** Mesh tools/oracle/showcase.txt (see make_showcase.py) with the original RenderBlocks. */
class ShowcaseOracle {
    static int[] build(World w, String layout) throws IOException {
        List<String> lines = new ArrayList<>();
        try (BufferedReader r = new BufferedReader(new FileReader(layout))) {
            for (String l; (l = r.readLine()) != null; ) if (!l.trim().isEmpty()) lines.add(l.trim());
        }
        String[] head = lines.get(0).split(" ");
        int x0 = Integer.parseInt(head[1]), z0 = Integer.parseInt(head[2]);
        int x1 = Integer.parseInt(head[3]), z1 = Integer.parseInt(head[4]);
        EntityOracle.FlatProvider f = new EntityOracle.FlatProvider(w);
        f.chunks.clear();
        w.chunkProvider = f;
        for (int cx = x0 - 1; cx <= x1 + 1; cx++) {
            for (int cz = z0 - 1; cz <= z1 + 1; cz++) {
                byte[] blocks = new byte[32768];
                for (int bx = 0; bx < 16; bx++) for (int bz = 0; bz < 16; bz++) blocks[(bx << 11) | (bz << 7) | 63] = 1;
                Chunk c = new Chunk(w, blocks, cx, cz);
                c.isTerrainPopulated = true;
                c.isChunkLoaded = true;
                f.chunks.put(f.key(cx, cz), c);
            }
        }
        for (int i = 1; i < lines.size(); i++) {
            String[] p = lines.get(i).split(" ");
            EntityOracle.rawBlock(w, Integer.parseInt(p[0]), Integer.parseInt(p[1]), Integer.parseInt(p[2]),
                Integer.parseInt(p[3]), Integer.parseInt(p[4]));
        }
        for (int cx = x0 - 1; cx <= x1 + 1; cx++) {
            for (int cz = z0 - 1; cz <= z1 + 1; cz++) {
                Chunk c = f.provideChunk(cx, cz);
                for (int x = 0; x < 16; x++) for (int y = 0; y < 128; y++) for (int z = 0; z < 16; z++) {
                    int wx = cx * 16 + x, wz = cz * 16 + z;
                    c.skylightMap.setNibble(x, y, z, (wx * 7 + y * 3 + wz * 5) & 15);
                    c.blocklightMap.setNibble(x, y, z, (wx * 3 + y * 11 + wz) & 15);
                }
            }
        }
        int[] cs = new int[(x1 - x0 + 1) * (z1 - z0 + 1) * 2];
        int k = 0;
        for (int cx = x0; cx <= x1; cx++) for (int cz = z0; cz <= z1; cz++) { cs[k++] = cx; cs[k++] = cz; }
        return cs;
    }

    static void run(String layout, String out, boolean fancy, boolean ao, boolean anaglyph) throws Exception {
        Oracle.meshSetup(fancy, ao);
        EntityRenderer.anaglyphEnable = anaglyph;
        World w = Oracle.newWorld(0);
        int[] cs = build(w, layout);
        Block.leaves.setGraphicsLevel(fancy);
        RenderBlocks.fancyGrass = fancy;
        ColorizerGrass.setGrassBiomeColorizer(Oracle.jarImage("misc/grasscolor.png"));
        ColorizerFoliage.getFoilageBiomeColorizer(Oracle.jarImage("misc/foliagecolor.png"));
        Oracle.dumpMeshes(w, cs, out);
    }
}
