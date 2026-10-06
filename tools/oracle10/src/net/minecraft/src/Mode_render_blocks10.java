package net.minecraft.src;

/** Original RenderBlocks + original Tessellator, through the harness's world mesh recorder. */
public final class Mode_render_blocks10 {
    public static void main(String[] args) throws Exception {
        ShowcaseOracle.run(args[0], args[1], !args[2].equals("fast"),
            !args[3].equals("off"), args[4].equals("anaglyph"));
    }
}
