package org.lwjgl.opengl;
import java.util.*;
public final class GL11 {
 public static final int GL_TRIANGLE_STRIP=5,GL_TEXTURE_2D=3553;
 public static final class Vertex { public int texture; public float[] values; Vertex(int t,float[] f){texture=t;values=f;} }
 public static final List<Vertex> vertices=new ArrayList<Vertex>();
 public static float[] color={1,1,1,1}; static float u,v; static int texture;
 public static void glBindTexture(int target,int name){texture=name;}
 public static void glBegin(int mode){if(mode!=5)throw new AssertionError(mode);}
 public static void glEnd(){}
 public static void glTexCoord2f(float a,float b){u=a;v=b;}
 public static void glVertex3f(float x,float y,float z){vertices.add(new Vertex(texture,new float[]{x,y,z,u,v,color[0],color[1],color[2],color[3]}));}
 public static void glColor3f(float r,float g,float b){color=new float[]{r,g,b,1};}
 public static void glColor4f(float r,float g,float b,float a){color=new float[]{r,g,b,a};}
}
