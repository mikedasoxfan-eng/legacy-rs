package org.lwjgl.opengl;
import java.nio.*;
import net.minecraft.src.Oracle;
import net.minecraft.src.RendererOracle;
/** No GL context: record the original Tessellator immediately before draw resets it. */
public final class GL11 {
	public static void glPixelStorei(int a,int b) {}
	/** Screenshot oracle: a fixed byte pattern stands in for the framebuffer. */
	public static void glReadPixels(int x,int y,int w,int h,int format,int type,ByteBuffer b) { for(int i=0;i<w*h*3;i++)b.put(i,(byte)(i*37+11)); }
    public static final int GL_DST_ALPHA=772, GL_NORMALIZE=2977, GL_EQUAL=514, GL_LEQUAL=515, GL_COMPILE=4864, GL_QUADS=7;
    public static final int GL_LIGHT0=16384, GL_LIGHT1=16385, GL_FRONT_AND_BACK=1032, GL_AMBIENT_AND_DIFFUSE=5634, GL_POSITION=4611, GL_DIFFUSE=4609, GL_SPECULAR=4610, GL_LIGHT_MODEL_AMBIENT=2899, GL_POLYGON_OFFSET_FILL=32823, GL_DST_COLOR=774, GL_SRC_COLOR=768;
    public static final int GL_GREATER = 516;
    public static final int GL_TEXTURE_MIN_FILTER=10241,GL_TEXTURE_MAG_FILTER=10240,GL_TEXTURE_WRAP_S=10242,GL_TEXTURE_WRAP_T=10243,GL_NEAREST=9728,GL_NEAREST_MIPMAP_LINEAR=9986,GL_CLAMP=10496,GL_REPEAT=10497,GL_RGBA=6408;
    public static final int GL_DEPTH_BUFFER_BIT = 256;
    public static final int GL_COLOR_BUFFER_BIT = 16384;
    public static final int GL_POINTS = 0;
    public static final int GL_TRIANGLES = 4;
    public static final int GL_ONE = 1;
    public static final int GL_SRC_ALPHA = 770;
    public static final int GL_ONE_MINUS_SRC_ALPHA = 771;
    public static final int GL_BYTE = 5120;
    public static final int GL_UNSIGNED_BYTE = 5121;
    public static final int GL_FLOAT = 5126;
    public static final int GL_FRONT = 1028;
    public static final int GL_EXP = 2048;
    public static final int GL_CULL_FACE = 2884;
    public static final int GL_LIGHTING = 2896;
    public static final int GL_COLOR_MATERIAL = 2903;
    public static final int GL_FOG = 2912;
    public static final int GL_FOG_DENSITY = 2914;
    public static final int GL_FOG_START = 2915;
    public static final int GL_FOG_END = 2916;
    public static final int GL_FOG_MODE = 2917;
    public static final int GL_FOG_COLOR = 2918;
    public static final int GL_DEPTH_TEST = 2929;
    public static final int GL_ALPHA_TEST = 3008;
    public static final int GL_BLEND = 3042;
    public static final int GL_TEXTURE_2D = 3553;
    public static final int GL_AMBIENT = 4608;
    public static final int GL_MODELVIEW = 5888;
    public static final int GL_PROJECTION = 5889;
    public static final int GL_FLAT = 7424;
    public static final int GL_SMOOTH = 7425;
    public static final int GL_LINEAR = 9729;
    public static final int GL_VERTEX_ARRAY = 32884;
    public static final int GL_NORMAL_ARRAY = 32885;
    public static final int GL_COLOR_ARRAY = 32886;
    public static final int GL_TEXTURE_COORD_ARRAY = 32888;
    // Standard GL constants 1.0 references.
    public static final int GL_BACK = 1029;
    public static final int GL_GEQUAL = 518;
    public static final int GL_MODELVIEW_MATRIX = 2982;
    public static final int GL_ONE_MINUS_DST_COLOR = 775;
    public static final int GL_ONE_MINUS_SRC_COLOR = 769;
    public static final int GL_PACK_ALIGNMENT = 3333;
    public static final int GL_PROJECTION_MATRIX = 2983;
    public static final int GL_RENDERER = 7937;
    public static final int GL_RGB = 6407;
    public static final int GL_SHORT = 5122;
    public static final int GL_TEXTURE = 5890;
    public static final int GL_UNPACK_ALIGNMENT = 3317;
    public static final int GL_VENDOR = 7936;
    public static final int GL_VERSION = 7938;
    public static final int GL_ZERO = 0;
    private static int cullFace10=2;
    public static void glCullFace(int face){if(RendererOracle.capture10){cullFace10=face==GL_FRONT?1:2;if(RendererOracle.cull10!=0)RendererOracle.cull10=cullFace10;}}
    public static void glDeleteLists(int id,int count) { RendererOracle.listsDelete10(id,count); }
    public static int matrixMode=5888;
    public static float[] color = {1,1,1,1};
    public static int texture;
    public static float fogStart, fogEnd, fogDensity;
    public static int fogMode;
    public static float[] clearColor={0,0,0,0};
    public static byte[] uploaded;
    private static int textureName10=1;
    public static void glGenTextures(IntBuffer names){for(int i=names.position();i<names.limit();i++)names.put(i,textureName10++);}
    public static void glCallLists(IntBuffer list){for(int i=list.position();i<list.limit();i++)RendererOracle.callList(list.get(i));}
    public static void glColor3f(float r,float g,float b){Oracle.recordSkyColor(r,g,b);RendererOracle.color3(r,g,b);}
    public static void glTexParameteri(int target,int pname,int value){}
    public static void glTexImage2D(int target,int level,int internal,int width,int height,int border,int format,int type,ByteBuffer bytes){uploaded=new byte[bytes.remaining()];bytes.duplicate().get(uploaded);}
    public static void glDeleteTextures(IntBuffer names){}
    public static void glTexSubImage2D(int target,int level,int x,int y,int width,int height,int format,int type,ByteBuffer bytes){uploaded=new byte[bytes.remaining()];bytes.duplicate().get(uploaded);}
    public static int glGenLists(int n){return RendererOracle.genLists(n);}
    public static void glNewList(int id,int mode){RendererOracle.newList(id);}
    public static void glEndList(){RendererOracle.endList();}
    public static void glCallList(int id){RendererOracle.callList(id);}
    public static void glPushMatrix() { RendererOracle.push(); }
    public static void glPopMatrix() { RendererOracle.pop(); }
    public static void glTranslatef(float x,float y,float z) { FirstPersonOp(0,x,y,z,0); }
    public static void glScalef(float x,float y,float z) { FirstPersonOp(1,x,y,z,0); }
    public static void glRotatef(float a,float x,float y,float z) { FirstPersonOp(2,a,x,y,z); }
    // 1.0: the texture matrix (lightmap scale/translate) is not part of what is recorded.
    private static void FirstPersonOp(int c,float a,float b,float d,float e) { if(matrixMode==5890)return; if(matrixMode==5889)net.minecraft.src.FirstPersonOracle.projectionOp(c,a,b,d,e);else RendererOracle.op(c,a,b,d,e); }
    public static void glMatrixMode(int mode){matrixMode=mode;}
    public static void glLoadIdentity(){if(matrixMode==5890)return; if(matrixMode==5889)net.minecraft.src.FirstPersonOracle.projectionReset();else RendererOracle.identity();}
    public static void glMultMatrix(FloatBuffer matrix){net.minecraft.src.FirstPersonOracle.perspective(matrix);}
    public static void glScaled(double x,double y,double z){FirstPersonOp(1,(float)x,(float)y,(float)z,0);}
    public static void glLight(int light,int pname,FloatBuffer values){}
    public static void glLightModel(int pname,FloatBuffer values){}
    public static void glShadeModel(int mode){}
    public static void glLineWidth(float width){}
    public static void glPolygonOffset(float factor,float units){}
    public static void glDepthMask(boolean b) { if(b)RendererOracle.flags|=32;else RendererOracle.flags&=~32; }
    public static void glDepthFunc(int f) { RendererOracle.depth=f; }
    public static void glFog(int name, FloatBuffer values) {}
    public static void glFogf(int name, float value) {
        if(name==GL_FOG_START) fogStart=value;
        if(name==GL_FOG_END) fogEnd=value;
        if(name==GL_FOG_DENSITY) fogDensity=value;
    }
    public static void glFogi(int name,int value) {if(name==GL_FOG_MODE)fogMode=value;}
    public static void glColorMaterial(int face,int mode) {}
    // Texturing switches on the lightmap unit (1.0 texture unit 1) are not the model's texture.
    public static void glDisable(int x) { if(RendererOracle.capture10&&x==GL_CULL_FACE)RendererOracle.cull10=0; if(x==GL_TEXTURE_2D&&GL13.activeTexture!=0)return; RendererOracle.enable(x,false); }
    public static void glEnable(int x) { if(RendererOracle.capture10&&x==GL_CULL_FACE&&RendererOracle.cull10==0)RendererOracle.cull10=cullFace10; if(x==GL_TEXTURE_2D&&GL13.activeTexture!=0)return; RendererOracle.enable(x,true); }
    public static void glBlendFunc(int x,int y) { RendererOracle.src=x;RendererOracle.dst=y; }
    public static void glAlphaFunc(int x,float y) { if(RendererOracle.capture10)RendererOracle.alpha10=y; }
    public static void glColorMask(boolean r,boolean g,boolean b,boolean a) {}
    public static void glNormal3f(float x,float y,float z) { RendererOracle.normal=new float[]{x,y,z}; }
    public static void glBindTexture(int x,int y) { if(GL13.activeTexture!=0)return; texture=y; }
    public static void glColor4f(float r,float g,float b,float a) { color=new float[]{r,g,b,a}; }
    public static void glClearColor(float r,float g,float b,float a) {clearColor=new float[]{r,g,b,a};}
    public static void glTexCoordPointer(int a,int b,FloatBuffer c) {}
    public static void glTexCoordPointer(int a,int b,java.nio.ShortBuffer c) {} // 1.0 lightmap coordinates
    public static void glTexCoordPointer(int a,int b,int c,long d) { throw new AssertionError("VBO"); }
    public static void glColorPointer(int a,boolean b,int c,ByteBuffer d) {}
    public static void glColorPointer(int a,int b,int c,long d) { throw new AssertionError("VBO"); }
    public static void glNormalPointer(int a,ByteBuffer b) {}
    public static void glNormalPointer(int a,int b,long c) { throw new AssertionError("VBO"); }
    public static void glVertexPointer(int a,int b,FloatBuffer c) {}
    public static void glVertexPointer(int a,int b,int c,long d) { throw new AssertionError("VBO"); }
    public static void glEnableClientState(int x) {}
    public static void glDisableClientState(int x) {}
    public static void glDrawArrays(int mode,int first,int count) { net.minecraft.src.Mode_drawintegration10.record(mode,first,count); if(RendererOracle.active||RendererOracle.isCompiling()||RendererOracle.pistonCapture) RendererOracle.record(mode,count); else Oracle.recordWeatherDraw(mode,count,color,texture); }
}
