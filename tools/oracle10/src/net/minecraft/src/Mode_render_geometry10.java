package net.minecraft.src;
import java.io.*;
import org.lwjgl.opengl.GL11;
import static net.minecraft.src.Mode_render_models10.*;
public final class Mode_render_geometry10 {
    public static void main(String[] args)throws Exception{
        out=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(args[0])));i(0x3031474d);i(2);
        ModelBase m=new ModelBase(){};m.textureWidth=128;m.textureHeight=64;
        m.setTextureOffset("named.first",-17,37);m.setTextureOffset("named.first",-56,88);
        TextureOffset uv=m.func_40297_a("named.first");i(uv.field_40734_a);i(uv.field_40733_b);i(m.func_40297_a("absent")==null?1:0);
        ModelRenderer p=new ModelRenderer(m,"named");p.addBox("first",-1.5F,-0.0F,2.25F,0,7,3);p.mirror=true;p.setTextureOffset(3,-7).addBox(-2.5F,-3.25F,-4.75F,9,0,5,-.35F);
        ModelRenderer a=new ModelRenderer(m,19,31);a.addBox(-3,2,-1,4,6,2,.5F);a.setRotationPoint(.25F,-.5F,2);
        ModelRenderer b=new ModelRenderer(m);b.setTextureSize(256,256);b.setTextureOffset(-56,144).addBox(-56,0,2,56,0,56);b.setRotationPoint(-2,3,4);b.rotateAngleY=.75F;
        p.addChild(a);p.addChild(b);
        for(Object x:m.boxList)part((ModelRenderer)x);
        i(24);
        for(int c=0;c<24;c++){
            p.showModel=c%7!=3;p.isHidden=c%7==4;
            p.setRotationPoint(c%4==0?0:-.25F*c,c%4==0?0:1.5F,c%4==0?0:-2);
            p.rotateAngleX=c%4==0?0:.13F*c;p.rotateAngleY=c%4==0?0:-.17F;p.rotateAngleZ=c%4==0?0:.31F;
            if(c==6){p.setTextureOffset(18,19).addBox(-1,-2,-3,1,2,3);a.addBox(0,1,2,0,2,3);}
            RendererOracle.reset10();float scale=new float[]{.125F,.0625F,.25F}[c%3];
            if(c%3==0)p.postRender(scale);else if(c%3==1)p.render(scale);else p.renderWithRotation(scale);
            Mode_render_modeldraw10.draws();
        }
        out.close();System.out.println("Original ModelBase/ModelBox/ModelRenderer: names, mirroring, children, visibility, cache and all three draw methods");
    }
}
