package net.minecraft.src;

import java.lang.reflect.Field;
import java.util.List;

/** Fixture access for 1.0 state that is no longer public in the original jar. */
final class Harness10 {
    static Field field(Object object, String name) {
        if (name.equals("damageRemainder")) name = "carryoverDamage";
        for (Class<?> type = object.getClass(); type != null; type = type.getSuperclass()) {
            try {
                Field field = type.getDeclaredField(name);
                field.setAccessible(true);
                return field;
            } catch (NoSuchFieldException missing) { }
        }
        throw new IllegalArgumentException(object.getClass().getName() + "." + name);
    }
    static int getInt(Object object, String name) {
        try { return field(object, name).getInt(object); }
        catch (IllegalAccessException error) { throw new RuntimeException(error); }
    }
    static void setInt(Object object, String name, int value) {
        try { field(object, name).setInt(object, value); }
        catch (IllegalAccessException error) { throw new RuntimeException(error); }
    }
    static float modelBound(ModelRenderer model, int index) {
        if (model.cubeList.isEmpty()) return 0;
        ModelBox box = (ModelBox) model.cubeList.get(0);
        return new float[] {box.field_40678_a, box.field_40676_b, box.field_40677_c,
                            box.field_40674_d, box.field_40675_e, box.field_40672_f}[index];
    }
    static PositionTextureVertex[] modelCorners(ModelRenderer model) throws IllegalAccessException {
        java.util.ArrayList<PositionTextureVertex> result = new java.util.ArrayList<PositionTextureVertex>();
        for (Object box : model.cubeList) {
            PositionTextureVertex[] corners = (PositionTextureVertex[]) field(box, "field_40679_h").get(box);
            java.util.Collections.addAll(result, corners);
        }
        return result.toArray(new PositionTextureVertex[result.size()]);
    }
    static TexturedQuad[] modelFaces(ModelRenderer model) throws IllegalAccessException {
        java.util.ArrayList<TexturedQuad> result = new java.util.ArrayList<TexturedQuad>();
        for (Object box : model.cubeList) {
            TexturedQuad[] faces = (TexturedQuad[]) field(box, "field_40680_i").get(box);
            java.util.Collections.addAll(result, faces);
        }
        return result.toArray(new TexturedQuad[result.size()]);
    }
    static boolean removedSleepSpawning(World world, List players) {
        // SpawnerAnimals.performSleepSpawning was removed in Minecraft 1.0: sleeping players
        // never cause a spawn, so the inherited call site spawns nothing and reports false.
        return false;
    }
    static float removedOrbPhase(EntityXPOrb orb) {
        // Reserved slot for the removed Beta orbital phase, to retain dump offsets.
        return 0;
    }
    static ModelRenderer removedRandomModel(ModelBase model, java.util.Random random) {
        throw new UnsupportedOperationException("ModelBase random-part selection was removed in Minecraft 1.0");
    }
    static int regionSizeDelta(java.io.File directory, int x, int z) {
        return regionSizeDelta(RegionFileCache.createOrLoadRegionFile(directory, x, z));
    }
    static int regionSizeDelta(RegionFile region) {
        // The public read-and-reset method was removed, but the original counter remains.
        int size = getInt(region, "sizeDelta");
        setInt(region, "sizeDelta", 0);
        return size;
    }
}
