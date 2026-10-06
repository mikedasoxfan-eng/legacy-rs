package net.minecraft.src;
import java.io.*;
import java.util.*;
import java.lang.reflect.*;

/** Original TextureOffset and TexturedQuad, including flipped draw normals. */
public final class Mode_r10world4geometry {
    public static void main(String[] a)throws Exception {
        Random random=new Random(42);
        Field invert=TexturedQuad.class.getDeclaredField("invertNormal");invert.setAccessible(true);
        Field normal=Tessellator.class.getDeclaredField("normal");normal.setAccessible(true);
        try(DataOutputStream o=new DataOutputStream(new FileOutputStream(a[1]))) {
            for(int n=0;n<64;n++) {
                int u=random.nextInt(),v=random.nextInt();TextureOffset offset=new TextureOffset(u,v);o.writeInt(offset.field_40734_a);o.writeInt(offset.field_40733_b);
                for(float[] size:new float[][]{{64,32},{256,256},{127,93}})for(boolean flip:new boolean[]{false,true})for(boolean reverse:new boolean[]{false,true}) {
                    PositionTextureVertex[] vertices={new PositionTextureVertex(1.25F,2.5F,-3.5F,0,0),new PositionTextureVertex(-1,0,4,0,0),new PositionTextureVertex(0.5F,3,2,0,0),new PositionTextureVertex(7,-4,0,0,0)};
                    TexturedQuad q=new TexturedQuad(vertices,u,v,u+31,v+17,size[0],size[1]);if(flip)q.flipFace();invert.setBoolean(q,reverse);
                    for(PositionTextureVertex p:q.vertexPositions){o.writeDouble(p.vector3D.xCoord);o.writeDouble(p.vector3D.yCoord);o.writeDouble(p.vector3D.zCoord);o.writeFloat(p.texturePositionX);o.writeFloat(p.texturePositionY);}
                    q.draw(Tessellator.instance,0.0625F);o.writeInt(normal.getInt(Tessellator.instance));
                }
            }
        }
    }
}
