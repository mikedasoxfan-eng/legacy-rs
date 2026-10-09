package net.minecraft.src;
import java.io.*;
import java.util.*;
import org.lwjgl.opengl.GL11;
import static net.minecraft.src.Mode_render_models10.*;
/** Executes the original RenderDragon beam. The living-model callbacks are isolated out. */
public final class Mode_render_dragonbeam10 {
    static final class Beam extends RenderDragon {
        protected void a(EntityDragon entity,float a,float b,float c,float d,float e,float f){}
        protected void a(EntityDragon entity,float partial){}
        protected int a(EntityDragon entity,int pass,float partial){return -1;}
        protected void passSpecialRender(EntityLiving entity,double x,double y,double z){}
        protected void loadTexture(String name){if(!name.equals("/mob/enderdragon/beam.png"))throw new AssertionError(name);GL11.texture=73;}
    }
    public static void main(String[] args)throws Exception{
        OpenGlHelper.lightmapDisabled=33984;OpenGlHelper.lightmapEnabled=33985;
        out=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(args[0])));i(0x30314244);i(1);i(64);
        Beam renderer=new Beam();EntityDragon dragon=new EntityDragon(null){public float getEntityBrightness(float partial){return 1;}};
        EntityEnderCrystal crystal=new EntityEnderCrystal(null);dragon.healingEnderCrystal=crystal;Random random=new Random(410035);
        for(int c=0;c<64;c++){
            double[][] points=new double[4][3];for(double[] p:points)for(int j=0;j<3;j++)p[j]=(random.nextDouble()-.5)*128;
            if(c<4){points[1]=points[2].clone();points[3]=points[2].clone();points[3][1]+=1;}
            int rotation=random.nextInt(100000),ticks=random.nextInt(100000);float partial=c%4*.25F;
            for(double[] p:points)for(double v:p)d(v);i(rotation);i(ticks);f(partial);
            dragon.prevPosX=points[1][0];dragon.prevPosY=points[1][1];dragon.prevPosZ=points[1][2];
            dragon.posX=points[2][0];dragon.posY=points[2][1];dragon.posZ=points[2][2];dragon.ticksExisted=ticks;
            crystal.posX=points[3][0];crystal.posY=points[3][1];crystal.posZ=points[3][2];crystal.field_41032_a=rotation;
            RendererOracle.reset10();renderer.doRender(dragon,points[0][0],points[0][1],points[0][2],0,partial);Mode_render_effects10.draws();
        }
        out.close();System.out.println("Original RenderDragon crystal beam: 64 cases");
    }
}
