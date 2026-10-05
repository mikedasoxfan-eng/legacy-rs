package net.minecraft.src;
import java.io.*;
import java.lang.reflect.*;
import java.util.*;
/** Record the actual VM identity input before the original HashSet inserts it.
 * Separate streams preserve the independent active and spawning set lifetimes. */
public final class Mode_replayidentities181 implements AutoCloseable {
    private final DataOutputStream active, spawning;
    private final Field spawnField;
    private final Object previousSpawning;
    private static HashSet<ChunkCoordIntPair> recording(final DataOutputStream out) {
        return new HashSet<ChunkCoordIntPair>() {
            public boolean add(ChunkCoordIntPair p) {
                try { out.writeInt(System.identityHashCode(p)); }
                catch (IOException e) { throw new UncheckedIOException(e); }
                return super.add(p);
            }
        };
    }
    public Mode_replayidentities181(World w, String path) throws Exception {
        active = new DataOutputStream(new BufferedOutputStream(new FileOutputStream(path + ".identities")));
        spawning = new DataOutputStream(new BufferedOutputStream(new FileOutputStream(path + ".spawnidentities")));
        Field positions = World.class.getDeclaredField("positionsToUpdate");
        positions.setAccessible(true); positions.set(w, recording(active));
        spawnField = SpawnerAnimals.class.getDeclaredField("eligibleChunksForSpawning");
        spawnField.setAccessible(true); previousSpawning = spawnField.get(null);
        spawnField.set(null, recording(spawning));
    }
    public void close() throws Exception {
        spawnField.set(null, previousSpawning);
        active.close(); spawning.close();
    }
    public static void main(String[] a) { throw new IllegalArgumentException("helper, use an integration replay"); }
}
