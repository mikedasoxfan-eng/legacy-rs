package net.minecraft.src;
import java.io.*;
import org.lwjgl.opengl.GL11;
import static net.minecraft.src.Mode_render_models10.*;
public final class Mode_render_modeldraw10 {
    static void draws()throws Exception{
        i(GL11.draws.size());
        for(GL11.Draw d:GL11.draws){i(d.ops.size());for(GL11.Op o:d.ops){i(o.code);for(float a:o.args)f(a);}i(d.cull);i(d.face.normal);i(d.face.vertices.size());for(float[] v:d.face.vertices)for(float a:v)f(a);}
        i(GL11.ops.size());for(GL11.Op o:GL11.ops){i(o.code);for(float a:o.args)f(a);}
    }
    public static void main(String[] args)throws Exception{
        int kind=Integer.parseInt(args[0]);out=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(args[1])));
        i(0x3031444d);i(2);i(32);
        EntityDragon dragon=kind==8?new EntityDragon(null):null;
        if(dragon!=null){dragon.health=200;dragon.field_40164_e=37;for(int j=0;j<64;j++){dragon.field_40162_d[j][0]=(j%5==0?-179:179)+j*7.75;dragon.field_40162_d[j][1]=80+j*.125;dragon.field_40162_d[j][2]=j*.37;}dragon.field_40173_aw=.13F;dragon.field_40172_ax=.87F;}
        for(int group=0;group<4;group++){
            ModelBase m=model(kind,group*.25F,group-2,6+group*3);
            for(int c=0;c<8;c++){
                float[] p={1.25F*c,.13F*c,17.25F*c,-179+c*67,89-c*27,new float[]{.0625F,.125F,.03125F,.0625F}[c%4]};
                int flags=c;float partial=c*.125F;
                m.field_40301_k=(c&1)!=0;m.onGround=c==0?-10000:c*.125F;m.isRiding=(c&2)!=0;
                if(m instanceof ModelBiped){ModelBiped v=(ModelBiped)m;v.field_1279_h=c%4;v.field_1278_i=c/2;v.field_40333_u=(c&4)!=0;v.isSneak=(c&1)!=0;}
                if(dragon!=null)m.setLivingAnimations(dragon,p[0],p[1],partial);
                i(group);i(kind);for(float v:p)f(v);i(flags);f(partial);
                if(dragon!=null){f(dragon.field_40173_aw+(dragon.field_40172_ax-dragon.field_40173_aw)*partial);for(int j=0;j<24;j++)for(double v:dragon.func_40160_a(j,partial))d(v);}
                GL11.reset();
                m.render(dragon,p[0],p[1],p[2],p[3],p[4],p[5]);
                draws();
            }
        }
        out.close();System.out.println("Original 1.0 model render bytecode: "+kind+", 32 sequential draw cases");
    }
}
