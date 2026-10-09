package net.minecraft.src;
import java.util.*;
import org.lwjgl.opengl.GL11;
/** Captures TexturedQuad draw arguments before the graphics backend. */
public final class Tessellator {
    public static final Tessellator instance=new Tessellator();
    ArrayList<float[]> vertices;ArrayList<Integer> colors;int normal,color,flags,mode;
    public void startDrawingQuads(){startDrawing(7);}
    public void startDrawing(int mode){vertices=new ArrayList<float[]>();colors=new ArrayList<Integer>();normal=0;color=0;flags=0;this.mode=mode;}
    public void setNormal(float x,float y,float z){flags|=128;normal=(byte)(int)(x*127)|((byte)(int)(y*127)<<8)|((byte)(int)(z*127)<<16);}
    public void setColorRGBA_I(int rgb,int alpha){flags|=64;color=(rgb>>16&255)|(rgb&65280)|((rgb&255)<<16)|((Math.max(0,Math.min(255,alpha)))<<24);}
    public void setColorOpaque_I(int rgb){setColorRGBA_I(rgb,255);}
    public void addVertex(double x,double y,double z){vertices.add(new float[]{(float)x,(float)y,(float)z,0,0});colors.add(color);}
    public void addVertexWithUV(double x,double y,double z,double u,double v){flags|=512;vertices.add(new float[]{(float)x,(float)y,(float)z,(float)u,(float)v});colors.add(color);}
    public int draw(){GL11.submit(new GL11.Face(vertices,normal,colors,flags,mode));return vertices.size()*32;}
}
