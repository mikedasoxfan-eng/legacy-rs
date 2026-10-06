package net.minecraft.src;
import java.io.*;
import java.util.*;
import org.lwjgl.opengl.GL11;
import static net.minecraft.src.Mode_render_models10.*;
/** Full original RenderLiving delegation for three new renderers, without resource or world dependencies. */
public final class Mode_render_living10 {
    static int texture(String path){
        if(path.equals("/mob/fire.png"))return 62;if(path.equals("/mob/lava.png"))return 63;
        if(path.equals("/mob/redcow.png"))return 60;if(path.equals("/mob/snowman.png"))return 61;
        String[] names={"villager","farmer","librarian","priest","smith","butcher"};
        for(int n=0;n<names.length;n++)if(path.equals("/mob/villager/"+names[n]+".png"))return 64+n;
        throw new AssertionError(path);
    }
    static class Blaze extends RenderBlaze {
        protected boolean loadDownloadableImageTexture(String url,String name){GL11.texture=texture(name);return true;}
        protected void passSpecialRender(EntityLiving e,double x,double y,double z){}
    }
    static class Magma extends RenderMagmaCube {
        protected boolean loadDownloadableImageTexture(String url,String name){GL11.texture=texture(name);return true;}
        protected void passSpecialRender(EntityLiving e,double x,double y,double z){}
    }
    static class Villager extends RenderVillager {
        protected boolean loadDownloadableImageTexture(String url,String name){GL11.texture=texture(name);return true;}
    }
    static class Mushroom extends RenderMooshroom {
        Mushroom(){super(new ModelCow(),.7F);}
        protected boolean loadDownloadableImageTexture(String url,String name){GL11.texture=texture(name);return true;}
        protected void loadTexture(String name){if(!name.equals("/terrain.png"))throw new AssertionError(name);GL11.texture=1;}
        protected void passSpecialRender(EntityLiving e,double x,double y,double z){}
    }
    static class Snow extends RenderSnowMan {
        protected boolean loadDownloadableImageTexture(String url,String name){GL11.texture=texture(name);return true;}
        protected void passSpecialRender(EntityLiving e,double x,double y,double z){}
    }
    /** Only the pumpkin inventory endpoint, whose geometry is original RenderBlocks bytecode. */
    static class PumpkinEndpoint extends ItemRenderer {
        PumpkinEndpoint(){super(null);}
        public void renderItem(EntityLiving entity,ItemStack item,int pass){
            if(item.itemID!=86||pass!=0||item.stackSize!=1||item.getItemDamage()!=0)throw new AssertionError("pumpkin arguments");
            GL11.glPushMatrix();GL11.texture=1;new RenderBlocks().renderBlockOnInventory(Block.pumpkin,0,1);GL11.glPopMatrix();
        }
    }
    public static void main(String[] args)throws Exception{
        int kind=Integer.parseInt(args[0]);Class.forName("net.minecraft.src.Block");
        OpenGlHelper.lightmapDisabled=33984;OpenGlHelper.lightmapEnabled=33985;
        out=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(args[1])));i(0x30314c52);i(1);i(kind);i(64);
        RenderLiving renderer=kind==0?new Blaze():kind==1?new Magma():kind==2?new Villager():kind==3?new Mushroom():new Snow();Random random=new Random(403001+kind);
        EntityLiving entity=kind==0?new EntityBlaze(null){public float getEntityBrightness(float t){return .73F;}}:
            kind==1?new EntityMagmaCube(null){public float getEntityBrightness(float t){return .73F;}}:
            kind==2?new EntityVillager(null){public float getEntityBrightness(float t){return .73F;}}:
            kind==3?new EntityMooshroom(null){public float getEntityBrightness(float t){return .73F;}}:
            new EntitySnowman(null){public float getEntityBrightness(float t){return .73F;}};
        if(kind==4){java.lang.reflect.Field field=sun.misc.Unsafe.class.getDeclaredField("theUnsafe");field.setAccessible(true);sun.misc.Unsafe unsafe=(sun.misc.Unsafe)field.get(null);RenderManager.instance.itemRenderer=(ItemRenderer)unsafe.allocateInstance(PumpkinEndpoint.class);renderer.setRenderManager(RenderManager.instance);}
        f(renderer.shadowSize);
        for(int c=0;c<64;c++){
            double[] position={(random.nextDouble()-.5)*128,(random.nextDouble()-.5)*128,(random.nextDouble()-.5)*128};
            float partial=c%4*.25F,body=random.nextFloat()*720-360,yaw=random.nextFloat()*720-360,pitch=random.nextFloat()*180-90;
            float amount=random.nextFloat()*1.5F,swing=random.nextFloat()*100,attack=random.nextFloat(),squish0=random.nextFloat()*2-1,squish1=random.nextFloat()*2-1;
            int ticks=random.nextInt(10000),hurt=c%5==0?3:0,death=c%7==0?c:0,size=1<<(c%4),profession=c%7-1;boolean child=kind==3&&c%2==0;
            if(kind==2){entity=new EntityVillager(null,profession){public float getEntityBrightness(float t){return .73F;}};}
            for(double v:position)d(v);for(float v:new float[]{partial,body,yaw,pitch,amount,swing,attack,squish0,squish1})f(v);
            for(int v:new int[]{ticks,hurt,death,size,profession,child?1:0})i(v);
            entity.prevRenderYawOffset=entity.renderYawOffset=body;entity.prevRotationYaw=entity.rotationYaw=yaw;entity.prevRotationPitch=entity.rotationPitch=pitch;
            entity.field_705_Q=entity.field_704_R=amount;entity.field_703_S=swing;entity.prevSwingProgress=entity.swingProgress=attack;entity.ticksExisted=ticks;entity.hurtTime=hurt;entity.deathTime=death;
            if(kind==1){EntityMagmaCube e=(EntityMagmaCube)entity;e.setSlimeSize(size);e.field_767_b=squish0;e.field_768_a=squish1;}
            if(kind==3)((EntityMooshroom)entity).func_40142_a_(child?-100:0);
            GL11.reset();renderer.doRender(entity,position[0],position[1],position[2],0,partial);Mode_render_effects10.draws();
        }
        out.close();System.out.println("Original 1.0 living renderer "+kind+": 64 full delegated cases");
    }
}
