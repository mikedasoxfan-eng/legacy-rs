package net.minecraft.src;

import java.util.Random;

/** The only override in the original Entity is its Random field initializer. */
public final class OracleHooks {
    private static boolean deterministic;
    private static long nextSeed;
    private static long explosionSeed;
    public static void reset(long seed) { nextSeed=seed; explosionSeed=seed+300; deterministic=true; }
    public static Random newEntityRandom() { return deterministic ? new Random(nextSeed++) : new Random(); }
    public static Random newExplosionRandom(){return deterministic?new Random(explosionSeed++):new Random();}
    public static long explosionSeed(){return explosionSeed;}
    public static long nextSeed() { return nextSeed; }
}
