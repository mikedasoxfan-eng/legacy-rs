package net.minecraft.src;
import java.util.*;

/** Opt-in log of GL calls the shared recorder does not keep (texgen, texture matrix,
 * lights, fog, GL13 texture units), plus the texture matrix in effect at each draw. */
public final class GlLog10 {
    public static boolean active;
    public static final ArrayList<float[]> commands=new ArrayList<float[]>();
    static ArrayList<float[]> texture=new ArrayList<float[]>();
    static final ArrayList<ArrayList<float[]>> textureStack=new ArrayList<ArrayList<float[]>>();
    /** Texture-matrix operations in effect for each draw emitted while active. */
    public static final ArrayList<ArrayList<float[]>> drawTexture=new ArrayList<ArrayList<float[]>>();
    public static void command(int code,float a,float b,float c,float d){if(active)commands.add(new float[]{code,a,b,c,d});}
    public static void values(java.nio.FloatBuffer v){command(22,v.get(v.position()),v.get(v.position()+1),v.get(v.position()+2),v.get(v.position()+3));}
    public static void textureOp(int c,float a,float b,float d,float e){texture.add(new float[]{c,a,b,d,e});}
    public static void texturePush(){textureStack.add(new ArrayList<float[]>(texture));}
    public static void texturePop(){texture=textureStack.remove(textureStack.size()-1);}
    public static void textureIdentity(){texture.clear();}
    public static void drawn(){if(active)drawTexture.add(new ArrayList<float[]>(texture));}
    public static void reset(){commands.clear();texture=new ArrayList<float[]>();textureStack.clear();drawTexture.clear();}
}
