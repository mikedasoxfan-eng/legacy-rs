package net.minecraft.src;
import java.awt.image.BufferedImage;
import org.lwjgl.opengl.GL11;
import org.lwjgl.opengl.GL13;

/** Controlled texture-unit-1 input for original GuiContainer pixel tests. */
public final class GuiLightmapFixture {
    static int texture=-1;
    static int color(int pattern,int x,int y) {
        int r,g,b;
        if(pattern==0){r=x*17;g=y*17;b=(x^y)*17;}
        else if(pattern==1){r=255-x*13;g=31+y*11;b=17+x*7+y*3;}
        else if(pattern==2){r=128;g=64;b=192;}
        else {r=x+y*16;g=255-r;b=(x*19+y*23)&255;}
        return 0xff000000|(r<<16)|(g<<8)|b;
    }
    static void install(RenderEngine engine,int pattern) {
        GL13.glActiveTexture(GL13.GL_TEXTURE0);
        if(texture!=-1)engine.deleteTexture(texture);
        BufferedImage image=new BufferedImage(16,16,BufferedImage.TYPE_INT_RGB);
        for(int y=0;y<16;y++)for(int x=0;x<16;x++)image.setRGB(x,y,color(pattern,x,y));
        texture=engine.allocateAndSetupTexture(image);
        GL13.glActiveTexture(GL13.GL_TEXTURE1);
        GL11.glMatrixMode(GL11.GL_TEXTURE);GL11.glLoadIdentity();GL11.glScalef(1f/256f,1f/256f,1f/256f);GL11.glTranslatef(8,8,8);
        GL11.glMatrixMode(GL11.GL_MODELVIEW);GL11.glBindTexture(GL11.GL_TEXTURE_2D,texture);
        GL11.glTexParameteri(GL11.GL_TEXTURE_2D,GL11.GL_TEXTURE_MIN_FILTER,GL11.GL_LINEAR);
        GL11.glTexParameteri(GL11.GL_TEXTURE_2D,GL11.GL_TEXTURE_MAG_FILTER,GL11.GL_LINEAR);
        GL11.glTexParameteri(GL11.GL_TEXTURE_2D,GL11.GL_TEXTURE_WRAP_S,org.lwjgl.opengl.GL12.GL_CLAMP_TO_EDGE);
        GL11.glTexParameteri(GL11.GL_TEXTURE_2D,GL11.GL_TEXTURE_WRAP_T,org.lwjgl.opengl.GL12.GL_CLAMP_TO_EDGE);
        GL11.glDisable(GL11.GL_TEXTURE_2D);
        GL13.glActiveTexture(GL13.GL_TEXTURE0);
    }
}
