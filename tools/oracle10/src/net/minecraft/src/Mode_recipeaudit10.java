package net.minecraft.src;

/** Initialize the original registries in client order before inspecting recipes. */
public final class Mode_recipeaudit10 {
    public static void main(String[] args) throws Exception {
        Block.stone.getClass();Item.shovelSteel.getClass();
        ContainerOracle.recipes(args[1]);
    }
}
