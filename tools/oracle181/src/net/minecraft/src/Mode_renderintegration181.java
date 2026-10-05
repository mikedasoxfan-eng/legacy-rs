package net.minecraft.src;
/** Supply the colorizer images Minecraft loads before these renderer calls. */
public final class Mode_renderintegration181 {
    public static void main(String[] a) throws Exception {
        ColorizerGrass.func_28181_a(Oracle.jarImage("misc/grasscolor.png"));
        ColorizerFoliage.func_28152_a(Oracle.jarImage("misc/foliagecolor.png"));
        if (a[1].equals("renderers")) RendererOracle.run(a[2]);
        else if (a[1].equals("pickuprenderers")) ParticleOracle.pickupDraws(a[2], true);
        else if (a[1].equals("pickuprenderersfast")) ParticleOracle.pickupDraws(a[2], false);
        else throw new IllegalArgumentException("renderers|pickuprenderers|pickuprenderersfast");
    }
}
