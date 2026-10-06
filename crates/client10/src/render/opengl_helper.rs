//! Release 1.0 `OpenGlHelper` and `OpenGlCapsChecker`: the lightmap texture units and the GL
//! calls the renderers issue through them. The wgpu backend samples the lightmap directly; these
//! values are what the original selects and records.

/// `OpenGlHelper.lightmapDisabled`: GL_TEXTURE0 (0x84C0) on both the ARB and core paths.
pub const LIGHTMAP_DISABLED: i32 = 0x84c0;
/// `OpenGlHelper.lightmapEnabled`: GL_TEXTURE1 (0x84C1) on both paths.
pub const LIGHTMAP_ENABLED: i32 = 0x84c1;

/// One recorded GL call: (code, arguments) in the oracle's GlLog10 numbering.
pub type Call = (i32, [f32; 4]);

/// `initializeTextures`: the ARB entry points are used only without OpenGL 1.3.
pub fn use_multitexture_arb(arb_multitexture: bool, opengl13: bool) -> bool {
    arb_multitexture && !opengl13
}
/// `OpenGlCapsChecker.checkARBOcclusion` (static in 1.0).
pub fn check_arb_occlusion(occlusion_query: bool) -> bool {
    occlusion_query
}
/// `setActiveTexture` on the GL 1.3 path (glActiveTexture).
pub fn set_active_texture(unit: i32) -> Call {
    (15, [unit as f32, 0.0, 0.0, 0.0])
}
/// `setClientActiveTexture` on the GL 1.3 path (glClientActiveTexture).
pub fn set_client_active_texture(unit: i32) -> Call {
    (16, [unit as f32, 0.0, 0.0, 0.0])
}
/// `setLightmapTextureCoords` on the GL 1.3 path (glMultiTexCoord2f).
pub fn set_lightmap_texture_coords(unit: i32, s: f32, t: f32) -> Call {
    (17, [unit as f32, s, t, 0.0])
}
/// The lightmap coordinates a packed `getBrightnessForRender` value selects
/// (`brightness % 65536`, `brightness / 65536`, each divided by 1.0F).
pub fn lightmap_coords(packed: i32) -> (f32, f32) {
    ((packed % 65536) as f32 / 1.0, (packed / 65536) as f32 / 1.0)
}
