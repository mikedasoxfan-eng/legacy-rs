package net.minecraft.src;
import java.io.*;
import java.util.*;
import org.lwjgl.opengl.GL11;
import static net.minecraft.src.Mode_render_models10.*;
/** Full original RenderDragon/RenderLiving model, eye, death and hurt pass ordering. */
public final class Mode_render_dragon10 {
    static final class Dragon extends RenderDragon {
        protected boolean loadDownloadableImageTexture(String url,String name){loadTexture(name);return true;}
        protected void loadTexture(String name){
            String[] paths={"ender","ender_eyes","shuffle"};for(int j=0;j<3;j++)if(name.equals("/mob/enderdragon/"+paths[j]+".png")){GL11.texture=70+j;return;}
            throw new AssertionError(name);
        }
        protected void passSpecialRender(EntityLiving e,double x,double y,double z){}
    }
    public static void main(String[] args)throws Exception{
        OpenGlHelper.lightmapDisabled=33984;OpenGlHelper.lightmapEnabled=33985;
        out=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(args[0])));i(0x30314452);i(1);i(32);
        Dragon renderer=new Dragon();EntityDragon entity=new EntityDragon(null){public float getEntityBrightness(float partial){return .73F;}};Random random=new Random(402081);
        for(int c=0;c<32;c++){
            double[] position={(random.nextDouble()-.5)*128,(random.nextDouble()-.5)*128,(random.nextDouble()-.5)*128};for(double p:position)d(p);
            float partial=c%4*.25F,previous=random.nextFloat()*4,current=previous+random.nextFloat();f(partial);f(previous);f(current);
            int health=c%8==0?0:200,death=c%7==0?c:0,hurt=c%5==0?3:0,deathticks=c%4==0?c*6:0;
            for(int v:new int[]{health,death,hurt,deathticks})i(v);
            entity.health=health;entity.deathTime=death;entity.hurtTime=hurt;entity.field_40178_aA=deathticks;entity.field_40173_aw=previous;entity.field_40172_ax=current;entity.field_40164_e=37;
            for(int j=0;j<64;j++)for(int k=0;k<3;k++){double value=k==0?(j%5==0?-179:179)+j*7.75+c:k==1?80+j*.125+c*.01:j*.37;entity.field_40162_d[j][k]=value;d(value);}
            RendererOracle.reset10();renderer.doRender(entity,position[0],position[1],position[2],0,partial);Mode_render_effects10.draws();
        }
        out.close();System.out.println("Original RenderDragon/RenderLiving: 32 full pass cases");
    }
}
