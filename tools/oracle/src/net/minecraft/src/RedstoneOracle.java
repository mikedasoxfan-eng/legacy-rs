package net.minecraft.src;

import java.io.*;
import java.lang.reflect.Field;
import java.util.*;

/** Scripted block worlds for redstone, rails, note blocks and pistons: a flat stone world
 *  (chunks -3..3, floor at y 63), commands from a text file, the original World.tick in
 *  between. Commands: place x y z id meta | remove x y z | use x y z | tick n
 *  (`#` starts a comment). The final world and a state line are written for comparison. */
class RedstoneOracle {
    static void run(String script, String out) throws Exception {
        World w = Oracle.newWorld(0);
        w.chunkProvider = new EntityOracle.FlatProvider(w);
        w.difficultySetting = 2;
        w.setAllowedMobSpawns(false, false);
        Field ids = Entity.class.getDeclaredField("nextEntityID"); ids.setAccessible(true); ids.setInt(null, 1000);
        Item.itemRand.setSeed(600); OracleHooks.reset(100); EntityOracle.mathRandom().setSeed(200);
        Field chest = BlockChest.class.getDeclaredField("random"); chest.setAccessible(true); ((Random) chest.get(Block.chest)).setSeed(500);
        w.rand.setSeed(300);
        w.field_9437_g = 0;
        Field sound = World.class.getDeclaredField("soundCounter"); sound.setAccessible(true); sound.setInt(w, 0);
        PlayerOracle.HeadlessPlayer p = PlayerOracle.player(w);
        p.setLocationAndAngles(0.5, 64, 0.5, 0, 0);
        int ticks = 0;
        StringBuilder steps = new StringBuilder();
        try (BufferedReader r = new BufferedReader(new FileReader(script))) {
            for (String line; (line = r.readLine()) != null; ) {
                line = line.split("#", 2)[0].trim();
                if (line.isEmpty()) continue;
                String[] a = line.split("\\s+");
                int[] v = new int[a.length];
                for (int i = 1; i < a.length; i++) v[i] = Integer.parseInt(a[i]);
                switch (a[0]) {
                    case "place": w.setBlockAndMetadataWithNotify(v[1], v[2], v[3], v[4], v[5]); break;
                    case "remove": w.setBlockWithNotify(v[1], v[2], v[3], 0); break;
                    case "use": {
                        int id = w.getBlockId(v[1], v[2], v[3]);
                        if (id > 0) Block.blocksList[id].blockActivated(w, v[1], v[2], v[3], p);
                        break;
                    }
                    case "tick":
                        for (int t = 0; t < v[1]; t++) { w.tick(); Oracle.drainLighting(w); ticks++; }
                        break;
                    case "check": {
                        // check x0 y0 z0 x1 y1 z1: ids and metadata of a box, every tick for n ticks
                        int n = a.length > 7 ? v[7] : 0;
                        for (int t = 0; t <= n; t++) {
                            if (t > 0) { w.tick(); Oracle.drainLighting(w); ticks++; }
                            steps.append(ticks).append(':');
                            for (int y = v[2]; y <= v[5]; y++) for (int z = v[3]; z <= v[6]; z++) for (int x = v[1]; x <= v[4]; x++)
                                steps.append(' ').append(w.getBlockId(x, y, z)).append('/').append(w.getBlockMetadata(x, y, z));
                            steps.append('\n');
                        }
                        break;
                    }
                    default: throw new IllegalArgumentException(line);
                }
                Oracle.drainLighting(w);
            }
        }
        Oracle.dumpWorld(w, out, 0, 64, 0);
        try (PrintWriter pw = new PrintWriter(out + ".txt")) {
            pw.println("ticks " + ticks + " time " + w.getWorldTime() + " entities " + w.loadedEntityList.size());
            pw.println("rand " + w.rand.nextLong());
            pw.print(steps);
        }
    }
}
