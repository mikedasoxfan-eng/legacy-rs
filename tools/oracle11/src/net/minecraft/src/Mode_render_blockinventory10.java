package net.minecraft.src;
import java.io.*;
import static net.minecraft.src.Mode_render_models10.*;

/** Both changed inventory branches, original RenderBlocks/Tessellator, per-vertex normals. */
public final class Mode_render_blockinventory10 {
    public static void main(String[] args)throws Exception {
        Class.forName("net.minecraft.src.Block");
        out=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(args[0])));
        i(0x30314942);i(1);i(128);
        RenderBlocks renderer=new RenderBlocks();
        for(int id:new int[]{111,122})for(int metadata=0;metadata<16;metadata++)for(int variant=0;variant<4;variant++) {
            float brightness=new float[]{0,.73F,1,1.5F}[variant];boolean tint=(variant&1)!=0;
            i(id);i(metadata);f(brightness);i(tint?1:0);
            RendererOracle.reset10();renderer.useInventoryTint=tint;
            renderer.renderBlockAsItem(Block.blocksList[id],metadata,brightness);
            Mode_render_effects10.draws();
        }
        out.close();System.out.println("Original release RenderBlocks inventory: 128 cases, normals and final state");
    }
}
