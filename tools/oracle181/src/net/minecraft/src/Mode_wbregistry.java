package net.minecraft.src;

/** Initialize Block before Item (the original client startup order). */
public final class Mode_wbregistry {
    public static void main(String[] a) throws Exception {
        int initialized = Block.stone.blockID;
        PlayerOracle.registry(a[1]);
    }
}
