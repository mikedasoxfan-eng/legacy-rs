package org.lwjgl.opengl;

/** Headless GL13 for 1.0's lightmap: tracks the active texture unit and the lightmap
 * coordinates (texture unit 1) so recorders can read them; no GL context needed. */
public final class GL13 {
    public static final int GL_TEXTURE0 = 33984;
    public static final int GL_TEXTURE1 = 33985;
    /** Index of the active texture unit (0 or 1). */
    public static int activeTexture;
    /** Last glMultiTexCoord2f on unit 1: the lightmap coordinates. */
    public static float lightmapU, lightmapV;
    public static void glActiveTexture(int unit) { activeTexture = unit - GL_TEXTURE0; }
    public static void glClientActiveTexture(int unit) {}
    public static void glMultiTexCoord2f(int unit, float s, float t) {
        if (unit == GL_TEXTURE1) { lightmapU = s; lightmapV = t; }
    }
}
