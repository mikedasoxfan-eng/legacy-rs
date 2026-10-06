package org.lwjgl.opengl;
import java.util.*;
/** Graphics endpoint recorder. The model and renderer classes remain original bytecode. */
public final class GL11 {
    public static final int GL_COMPILE=4864, GL_CULL_FACE=2884, GL_FRONT=1028, GL_BACK=1029;
    public static final int GL_LIGHTING=2896,GL_LIGHT0=16384,GL_LIGHT1=16385,GL_COLOR_MATERIAL=2903,
        GL_FRONT_AND_BACK=1032,GL_AMBIENT_AND_DIFFUSE=5634,GL_POSITION=4611,GL_DIFFUSE=4609,
        GL_AMBIENT=4608,GL_SPECULAR=4610,GL_FLAT=7424,GL_SMOOTH=7425,GL_LIGHT_MODEL_AMBIENT=2899,
        GL_TEXTURE_2D=3553,GL_BLEND=3042,GL_ALPHA_TEST=3008,GL_SRC_ALPHA=770,GL_ONE=1,GL_ONE_MINUS_SRC_ALPHA=771,
        GL_DEPTH_TEST=2929,GL_EQUAL=514,GL_LEQUAL=515,GL_GREATER=516,GL_NORMALIZE=2977;
    public static final class Op {public int code;public float[] args;Op(int c,float... a){code=c;args=a;}}
    public static final class Face {public ArrayList<float[]> vertices;public ArrayList<Integer> colors;public int normal,flags,mode;public Face(ArrayList<float[]> v,int n,ArrayList<Integer> colors,int flags,int mode){vertices=v;normal=n;this.colors=colors;this.flags=flags;this.mode=mode;}}
    public static final class Draw {public ArrayList<Op> ops;public Face face;public int cull,flags,texture,src,dst,depth;public float[] material;public float alpha;Draw(Face f){cull=culling?cullFace:0;face=f;ops=new ArrayList<Op>(GL11.ops);flags=GL11.flags|f.flags;texture=GL11.texture;src=GL11.src;dst=GL11.dst;depth=GL11.depth;material=GL11.material.clone();alpha=GL11.alpha;}}
    public static ArrayList<Op> ops=new ArrayList<Op>();
    public static ArrayList<Draw> draws=new ArrayList<Draw>();
    static Stack<ArrayList<Op>> stack=new Stack<ArrayList<Op>>();
    static Map<Integer,ArrayList<Face>> lists=new HashMap<Integer,ArrayList<Face>>();
    static int next=1,compiling=0;
    static boolean culling=false;static int cullFace=2;
    public static int flags,texture,src,dst,depth,activeTexture;public static float[] material;public static float alpha;
    public static void reset(){ops.clear();draws.clear();stack.clear();culling=false;cullFace=2;flags=1|8|32|256;texture=0;src=1;dst=0;depth=515;material=new float[]{1,1,1,1};alpha=.1F;activeTexture=33984;}
    public static void glPushMatrix(){stack.push(new ArrayList<Op>(ops));}
    public static void glPopMatrix(){ops=stack.pop();}
    public static void glTranslatef(float x,float y,float z){ops.add(new Op(0,x,y,z,0));}
    public static void glScalef(float x,float y,float z){ops.add(new Op(1,x,y,z,0));}
    public static void glRotatef(float a,float x,float y,float z){ops.add(new Op(2,a,x,y,z));}
    public static int glGenLists(int count){int id=next;next+=count;return id;}
    public static void glNewList(int id,int mode){compiling=id;lists.put(id,new ArrayList<Face>());}
    public static void glEndList(){compiling=0;}
    public static void submit(Face f){if(compiling!=0)lists.get(compiling).add(f);else draws.add(new Draw(f));}
    public static void glCallList(int id){for(Face f:lists.get(id))draws.add(new Draw(f));}
    static int flag(int x){return x==2896?1:x==32826?2:x==2977?4:x==3553&&activeTexture==33984?8:x==3042?16:x==2929?256:0;}
    public static void glEnable(int x){if(x==2884)culling=true;flags|=flag(x);}
    public static void glDisable(int x){if(x==2884)culling=false;flags&=~flag(x);}
    public static void glCullFace(int x){cullFace=x==1028?1:2;}
    public static void glDeleteLists(int id,int count){lists.remove(id);}
    public static void glColor3f(float r,float g,float b){glColor4f(r,g,b,1);}
    public static void glBindTexture(int target,int id){texture=id;}
    public static void glColor4f(float r,float g,float b,float a){material=new float[]{r,g,b,a};}
    public static void glBlendFunc(int s,int d){src=s;dst=d;}
    public static void glDepthFunc(int d){depth=d;}
    public static void glDepthMask(boolean write){flags=write?flags|32:flags&~32;}
    public static void glAlphaFunc(int f,float a){alpha=a;}
    public static void glShadeModel(int mode){}
    public static void glColorMaterial(int face,int mode){}
    public static void glLight(int light,int name,java.nio.FloatBuffer values){}
    public static void glLightModel(int name,java.nio.FloatBuffer values){}
}
