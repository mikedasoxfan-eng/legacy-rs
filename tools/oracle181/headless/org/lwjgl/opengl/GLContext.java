package org.lwjgl.opengl;
/** Capabilities queried by the original renderer; no native context exists. */
public final class GLContext {
    private static final ContextCapabilities capabilities = new ContextCapabilities();
    public static ContextCapabilities getCapabilities() { return capabilities; }
}
