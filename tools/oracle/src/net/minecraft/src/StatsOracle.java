package net.minecraft.src;

import java.io.PrintWriter;
import java.lang.reflect.Field;

/** StatList as the original registers it: the general/block/item lists the statistics
 *  screen shows, and which stat each block/item index adds to (after replaceSimilarBlocks). */
class StatsOracle {
    static String type(StatBase s) throws Exception {
        Field f = StatBase.class.getDeclaredField("field_26902_a");
        f.setAccessible(true);
        Object t = f.get(s);
        return t == StatBase.field_27086_j ? "time" : t == StatBase.field_27085_k ? "distance" : "simple";
    }

    static void run(String out) throws Exception {
        Block.stone.getClass();
        Item.shovelSteel.getClass();
        try (PrintWriter pw = new PrintWriter(out, "UTF-8")) {
            for (Object o : StatList.field_25187_b) {
                StatBase s = (StatBase) o;
                pw.println("general\t" + s.statId + "\t" + type(s) + "\t" + s.statName);
            }
            for (Object o : StatList.field_25185_d) {
                StatCrafting s = (StatCrafting) o;
                pw.println("block\t" + s.statId + "\t" + s.func_25072_b() + "\t" + s.statName);
            }
            for (Object o : StatList.field_25186_c) {
                StatCrafting s = (StatCrafting) o;
                pw.println("item\t" + s.statId + "\t" + s.func_25072_b() + "\t" + s.statName);
            }
            String[] names = {"mine", "craft", "use", "break"};
            StatBase[][] arrays = {StatList.mineBlockStatArray, StatList.field_25158_z, StatList.field_25172_A, StatList.field_25170_B};
            for (int a = 0; a < 4; a++) {
                for (int i = 0; i < arrays[a].length; i++) {
                    if (arrays[a][i] != null) pw.println(names[a] + "\t" + i + "\t" + arrays[a][i].statId);
                }
            }
        }
    }
}
