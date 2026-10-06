package net.minecraft.src;
import java.io.*;
import java.lang.reflect.*;
import java.util.*;
/** Record the actual VM identity input before the original HashSet inserts it.
 * Separate streams preserve the independent active and spawning set lifetimes. */
public final class Mode_replayidentities10 implements AutoCloseable {
    private final DataOutputStream active, spawning;
    private final Field spawnField;
    private final Object previousSpawning;
    private static HashSet<ChunkCoordIntPair> recording(final DataOutputStream out) {
        return new HashSet<ChunkCoordIntPair>() {
            public boolean add(ChunkCoordIntPair p) {
                try { Trace10.writeInt(out, "System.identityHashCode(p)", System.identityHashCode(p)); }
                catch (IOException e) { throw new UncheckedIOException(e); }
                return super.add(p);
            }
        };
    }
    private static HashMap<ChunkCoordIntPair,Boolean> recordingMap(final DataOutputStream out) {
        return new HashMap<ChunkCoordIntPair,Boolean>() {
            public Boolean put(ChunkCoordIntPair p, Boolean boundary) {
                try { Trace10.writeInt(out, "System.identityHashCode(p)", System.identityHashCode(p)); }
                catch (IOException e) { throw new UncheckedIOException(e); }
                return super.put(p, boundary);
            }
        };
    }
    public Mode_replayidentities10(World w, String path) throws Exception {
        active = new DataOutputStream(new BufferedOutputStream(new FileOutputStream(path + ".identities")));
        spawning = new DataOutputStream(new BufferedOutputStream(new FileOutputStream(path + ".spawnidentities")));
        Field positions = World.class.getDeclaredField("positionsToUpdate");
        positions.setAccessible(true); positions.set(w, recording(active));
        spawnField = SpawnerAnimals.class.getDeclaredField("eligibleChunksForSpawning");
        spawnField.setAccessible(true); previousSpawning = spawnField.get(null);
        // Release spawning records border flags in a HashMap, rather than the
        // Beta HashSet. Record inputs while preserving the original map calls.
        spawnField.set(null, recordingMap(spawning));
    }
    public void close() throws Exception {
        spawnField.set(null, previousSpawning);
        active.close(); spawning.close();
    }
    public static void main(String[] a) { throw new IllegalArgumentException("helper, use an integration replay"); }
}
