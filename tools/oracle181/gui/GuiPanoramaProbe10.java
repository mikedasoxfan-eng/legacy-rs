package net.minecraft.src;
import java.io.*;
import java.lang.reflect.*;
import java.nio.*;
import org.lwjgl.BufferUtils;
import org.lwjgl.opengl.GL11;
/** Optional observations between actual original cube and blur methods. */
final class GuiPanoramaProbe10 {
    static void draw(GuiShot.Capture capture,String name)throws Exception {
        File out=GuiShot.out;
        Method cube=GuiMainMenu.class.getDeclaredMethod("func_35355_b",int.class,int.class,float.class);
        Method blur=GuiMainMenu.class.getDeclaredMethod("func_35354_a",float.class);
        cube.setAccessible(true);blur.setAccessible(true);
        Field ticks=GuiMainMenu.class.getDeclaredField("field_35357_f");ticks.setAccessible(true);
        float time=(float)ticks.getInt(capture.inner);

        java.nio.FloatBuffer matrix=BufferUtils.createFloatBuffer(16);
        try(DataOutputStream data=new DataOutputStream(new FileOutputStream(new File(out,name+"_matrices.bin")))) {
            GL11.glMatrixMode(GL11.GL_MODELVIEW);GL11.glPushMatrix();
            for(float angle:new float[]{0,20,90,180,-90,1,45,120}) {
                GL11.glLoadIdentity();GL11.glRotatef(angle,1,0,0);matrix.clear();GL11.glGetFloat(GL11.GL_MODELVIEW_MATRIX,matrix);
                System.out.println("rotation "+angle+" "+matrix.get(5)+" "+matrix.get(6));
            }
            GL11.glLoadIdentity();GL11.glRotatef(180,1,0,0);
            for(int sample=0;sample<64;sample++) {
                GL11.glPushMatrix();GL11.glTranslatef((sample%8/8.0F-0.5F)/64.0F,(sample/8/8.0F-0.5F)/64.0F,0);
                GL11.glRotatef(MathHelper.sin(time/400.0F)*25.0F+20.0F,1,0,0);GL11.glRotatef(-time*0.1F,0,1,0);
                for(int face=0;face<6;face++) {
                    GL11.glPushMatrix();
                    if(face==1)GL11.glRotatef(90,0,1,0);if(face==2)GL11.glRotatef(180,0,1,0);
                    if(face==3)GL11.glRotatef(-90,0,1,0);if(face==4)GL11.glRotatef(90,1,0,0);if(face==5)GL11.glRotatef(-90,1,0,0);
                    matrix.clear();GL11.glGetFloat(GL11.GL_MODELVIEW_MATRIX,matrix);
                    for(int k=0;k<16;k++)data.writeFloat(matrix.get(k));GL11.glPopMatrix();
                }
                GL11.glPopMatrix();
            }
            GL11.glPopMatrix();
        }
        GL11.glViewport(0,0,256,256);cube.invoke(capture.inner,capture.mx,capture.my,0.0F);
        capture.save(name+"_cube",org.lwjgl.opengl.EXTFramebufferObject.GL_COLOR_ATTACHMENT0_EXT);
        for(int pass=0;pass<8;pass++) {
            blur.invoke(capture.inner,0.0F);
            capture.save(name+"_blur"+pass,org.lwjgl.opengl.EXTFramebufferObject.GL_COLOR_ATTACHMENT0_EXT);
        }
        GL11.glViewport(0,0,capture.mc.displayWidth,capture.mc.displayHeight);
    }
}
