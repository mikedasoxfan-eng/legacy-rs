package net.minecraft.src;

import java.io.BufferedReader;
import java.io.FileReader;
import java.io.PrintWriter;
import java.util.ArrayList;
import java.util.Collections;
import java.util.List;
import java.util.Map;
import java.lang.reflect.Field;

/** StatList as the original registers it: the general/block/item lists the statistics
 *  screen shows, and which stat each block/item index adds to (after replaceSimilarBlocks). */
class StatsOracle {
    static String type(StatBase s) throws Exception {
        Field f = StatBase.class.getDeclaredField("type");
        f.setAccessible(true);
        Object t = f.get(s);
        return t == StatBase.timeStatType ? "time" : t == StatBase.distanceStatType ? "distance" : "simple";
    }

    static void run(String out) throws Exception {
        Block.stone.getClass();
        Item.shovelSteel.getClass();
        try (PrintWriter pw = new PrintWriter(out, "UTF-8")) {
            for (Object o : StatList.field_25187_b) {
                StatBase s = (StatBase) o;
                pw.println("general\t" + s.statId + "\t" + type(s) + "\t" + s.statName);
            }
            for (Object o : StatList.objectMineStats) {
                StatCrafting s = (StatCrafting) o;
                pw.println("block\t" + s.statId + "\t" + s.func_25072_b() + "\t" + s.statName);
            }
            for (Object o : StatList.field_25186_c) {
                StatCrafting s = (StatCrafting) o;
                pw.println("item\t" + s.statId + "\t" + s.func_25072_b() + "\t" + s.statName);
            }
            String[] names = {"mine", "craft", "use", "break"};
            StatBase[][] arrays = {StatList.mineBlockStatArray, StatList.objectCraftStats, StatList.objectUseStats, StatList.objectBreakStats};
            for (int a = 0; a < 4; a++) {
                for (int i = 0; i < arrays[a].length; i++) {
                    if (arrays[a][i] != null) pw.println(names[a] + "\t" + i + "\t" + arrays[a][i].statId);
                }
            }
        }
    }

    /** Read a stats file the way StatsSyncher does (lines joined, checksum checked), print the
     *  stats sorted by id ("null" if rejected), then write the map back in the original format. */
    static void file(String in, String out) throws Exception {
        Block.stone.getClass();
        Item.shovelSteel.getClass();
        StringBuilder text = new StringBuilder();
        try (BufferedReader r = new BufferedReader(new FileReader(in))) {
            String line;
            while ((line = r.readLine()) != null) text.append(line);
        }
        Map map = StatFileWriter.func_27177_a(text.toString());
        if (map == null) {
            System.out.println("null");
            return;
        }
        List<String> rows = new ArrayList<>();
        for (Object k : map.keySet()) rows.add(((StatBase) k).statId + " " + map.get(k));
        Collections.sort(rows, (x, y) -> Integer.compare(Integer.parseInt(x.split(" ")[0]), Integer.parseInt(y.split(" ")[0])));
        for (String row : rows) System.out.println(row);
        try (PrintWriter pw = new PrintWriter(out)) {
            pw.print(StatFileWriter.func_27185_a("Player", "local", map));
        }
    }
}
