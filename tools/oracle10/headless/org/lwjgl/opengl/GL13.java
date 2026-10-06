package org.lwjgl.opengl;
/** Texture-unit endpoint; primary texturing is recorded separately from the lightmap unit. */
public final class GL13 {
    public static void glActiveTexture(int unit){GL11.activeTexture=unit;}
    public static void glClientActiveTexture(int unit){}
    public static void glMultiTexCoord2f(int unit,float x,float y){}
}
