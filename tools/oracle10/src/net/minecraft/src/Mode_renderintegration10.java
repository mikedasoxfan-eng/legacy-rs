package net.minecraft.src;
/** Supply the colorizer images Minecraft loads before these renderer calls. */
public final class Mode_renderintegration10 {
    static boolean captureInputs;
    static void manager(RenderManager m) throws Exception {
        if (!captureInputs) return;
        // Minecraft initializes the tile dispatcher with the same RenderEngine.
        // Without it bindTextureByName silently retains the terrain texture.
        java.util.Map textures = (java.util.Map)Oracle.getField(m.renderEngine, "textureMap");
        textures.put("/item/chest.png", 31); textures.put("/item/largechest.png", 32);
        TileEntityRenderer.instance.renderEngine = m.renderEngine;
        TileEntityRenderer.instance.worldObj = m.worldObj;
    }
    public static void main(String[] a) throws Exception {
        ColorizerGrass.setGrassBiomeColorizer(Oracle.jarImage("misc/grasscolor.png"));
        ColorizerFoliage.getFoilageBiomeColorizer(Oracle.jarImage("misc/foliagecolor.png"));
        captureInputs = true;
        try {
            if (a[1].equals("renderers")) RendererOracle.run(a[2]);
            else if (a[1].equals("pickuprenderers")) ParticleOracle.pickupDraws(a[2], true);
            else if (a[1].equals("pickuprenderersfast")) ParticleOracle.pickupDraws(a[2], false);
            else if (a[1].equals("vehiclerenderers")) RendererOracle.vehicles(a[2]);
            else throw new IllegalArgumentException("renderers|pickuprenderers|pickuprenderersfast|vehiclerenderers");
        } finally { captureInputs = false; }
    }
}
