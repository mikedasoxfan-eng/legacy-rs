package net.minecraft.src;
import java.io.*;
import java.lang.reflect.*;
import java.util.*;

/** Uses the original, remapped release jar. No model class is recompiled. */
public final class Mode_render_models10 {
    static DataOutputStream out;
    static void i(int x)throws Exception{out.writeInt(Integer.reverseBytes(x));}
    static void f(float x)throws Exception{i(Float.floatToRawIntBits(x));}
    static void d(double x)throws Exception{out.writeLong(Long.reverseBytes(Double.doubleToRawLongBits(x)));}
    static void s(String x)throws Exception{byte[] b=(x==null?"":x).getBytes("UTF-8");i(b.length);out.write(b);}
    static Object field(Object x,String n)throws Exception{Field f=x.getClass().getDeclaredField(n);f.setAccessible(true);return f.get(x);}
    static void wolfField(EntityWolf x,String n,float v)throws Exception{Field f=EntityWolf.class.getDeclaredField(n);f.setAccessible(true);f.setFloat(x,v);}
    static ModelBase model(int k,float a,float b,int legs) {
        switch(k){
        case 0:return new ModelBiped(a,b);case 1:return new ModelZombie();case 2:return new ModelSkeleton();
        case 3:return new ModelQuadruped(legs,a);case 4:return new ModelPig(a);case 5:return new ModelCow();
        case 6:return new ModelChicken();case 7:return new ModelWolf();case 8:return new ModelDragon(a);
        case 9:return new ModelVillager(a,b);case 10:return new ModelSnowMan();case 11:return new ModelBlaze();
        case 12:return new ModelMagmaCube();case 13:return new ModelBook();case 14:return new ModelEnderCrystal(a);
        default:throw new AssertionError(k);
        }
    }
    static void vertex(PositionTextureVertex v)throws Exception{
        d(v.vector3D.xCoord);d(v.vector3D.yCoord);d(v.vector3D.zCoord);f(v.texturePositionX);f(v.texturePositionY);
    }
    static void part(ModelRenderer p)throws Exception{
        f(p.textureWidth);f(p.textureHeight);s(p.boxName);
        f(p.rotationPointX);f(p.rotationPointY);f(p.rotationPointZ);f(p.rotateAngleX);f(p.rotateAngleY);f(p.rotateAngleZ);
        i((p.mirror?1:0)|(p.showModel?2:0)|(p.isHidden?4:0));
        i(p.cubeList.size());
        for(Object o:p.cubeList){ModelBox c=(ModelBox)o;
            f(c.field_40678_a);f(c.field_40676_b);f(c.field_40677_c);f(c.field_40674_d);f(c.field_40675_e);f(c.field_40672_f);s(c.field_40673_g);
            for(PositionTextureVertex v:(PositionTextureVertex[])field(c,"field_40679_h"))vertex(v);
            for(TexturedQuad q:(TexturedQuad[])field(c,"field_40680_i"))for(PositionTextureVertex v:q.vertexPositions)vertex(v);
        }
    }
    public static void main(String[] args)throws Exception{
        int kind=Integer.parseInt(args[0]);
        out=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(args[1])));
        i(0x30314d52);i(2);i(5*16);
        Random r=new Random(301001L+kind);
        for(int group=0;group<5;group++){
            float a=new float[]{0,.35F,-.25F,.5F,2}[group],b=new float[]{0,1.25F,-2,4,-14}[group];int legs=6+group*3;
            ModelBase m=model(kind,a,b,legs);
            EntityWolf wolf=kind==7?new EntityWolf(null):null;
            EntityMagmaCube magma=kind==12?new EntityMagmaCube(null):null;
            for(int c=0;c<16;c++){
                float[] p={r.nextFloat()*2000-1000,r.nextFloat()*2,r.nextFloat()*20000-10000,r.nextFloat()*720-360,r.nextFloat()*180-90,.0625F};
                float attack=c==0?-10000:r.nextFloat();int flags=c;float partial=(c%5)*.25F;
                int left=c%4,right=(c/4)%4;boolean bow=(c&4)!=0;
                float[] interest={r.nextFloat(),r.nextFloat()},shake={r.nextFloat()*2,r.nextFloat()*2};
                m.onGround=attack;m.isRiding=(flags&1)!=0;m.field_40301_k=(flags&8)!=0;
                if(m instanceof ModelBiped){ModelBiped v=(ModelBiped)m;v.isSneak=(flags&2)!=0;v.heldItemLeft=left;v.heldItemRight=right;v.aimedBow=bow;}
                if(wolf!=null){wolf.setAngry((flags&2)!=0);wolf.setIsSitting((flags&4)!=0);wolfField(wolf,"field_25054_c",interest[0]);wolfField(wolf,"field_25048_b",interest[1]);wolfField(wolf,"prevTimeWolfIsShaking",shake[0]);wolfField(wolf,"timeWolfIsShaking",shake[1]);m.setLivingAnimations(wolf,p[0],p[1],partial);}
                if(magma!=null){magma.field_767_b=shake[0]-.7F;magma.field_768_a=shake[1]-.7F;m.setLivingAnimations(magma,p[0],p[1],partial);}
                m.setRotationAngles(p[0],p[1],p[2],p[3],p[4],p[5]);
                i(group);i(kind);f(a);f(b);i(legs);for(float v:p)f(v);f(attack);i(flags);f(partial);i(left);i(right);i(bow?1:0);for(float v:interest)f(v);for(float v:shake)f(v);
                i(m.textureWidth);i(m.textureHeight);i(m instanceof ModelBlaze?((ModelBlaze)m).func_40321_a():m instanceof ModelMagmaCube?((ModelMagmaCube)m).func_40343_a():0);i(m.boxList.size());
                for(Object box:m.boxList)part((ModelRenderer)box);
                i(m.boxList.size());
                for(Object box:m.boxList){ModelRenderer v=(ModelRenderer)box;i(v.childModels==null?0:v.childModels.size());if(v.childModels!=null)for(Object child:v.childModels)i(m.boxList.indexOf(child));}
            }
        }
        out.close();System.out.println("Original 1.0 model bytecode: "+kind+", 80 cases");
    }
}
