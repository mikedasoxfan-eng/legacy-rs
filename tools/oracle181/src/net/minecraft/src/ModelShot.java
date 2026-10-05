package net.minecraft.src;

import java.io.*;
import java.nio.*;
import java.util.*;
import java.util.zip.ZipFile;
import java.awt.image.BufferedImage;
import javax.imageio.ImageIO;
import org.lwjgl.BufferUtils;
import org.lwjgl.opengl.*;
import org.lwjgl.util.glu.GLU;

/** Optional visual oracle. Actual Mojang RenderLiving subclasses and model display
 * lists draw into a GL pbuffer. The only stubs supply textures, brightness and render
 * state, replacing the world/AI. models mode in Oracle.java remains entirely GL-free. */
public class ModelShot {
    static boolean variants;
    static float brightness = 1.0F;
    static class JarTextures extends RenderEngine {
        final Map<String,Integer> ids = new HashMap<>();
        final ZipFile jar;
        JarTextures() throws Exception {
            super(null,null);
            String path = System.getenv("BETA_JAR");
            if (path == null) path = System.getenv("APPDATA") + "/.minecraft/versions/b1.7.3/b1.7.3.jar";
            jar = new ZipFile(path);
        }
        public int getTexture(String name) {
            if (ids.containsKey(name)) return ids.get(name);
            try {
                BufferedImage img = ImageIO.read(jar.getInputStream(jar.getEntry(name.startsWith("/") ? name.substring(1) : name)));
                ByteBuffer data = BufferUtils.createByteBuffer(img.getWidth()*img.getHeight()*4);
                for (int y=0;y<img.getHeight();y++) for (int x=0;x<img.getWidth();x++) {
                    int c = img.getRGB(x,y);
                    data.put((byte)(c>>16)).put((byte)(c>>8)).put((byte)c).put((byte)(c>>24));
                }
                data.flip(); int id = GL11.glGenTextures(); GL11.glBindTexture(GL11.GL_TEXTURE_2D,id);
                GL11.glTexParameteri(GL11.GL_TEXTURE_2D,GL11.GL_TEXTURE_MIN_FILTER,GL11.GL_NEAREST);
                GL11.glTexParameteri(GL11.GL_TEXTURE_2D,GL11.GL_TEXTURE_MAG_FILTER,GL11.GL_NEAREST);
                GL11.glTexImage2D(GL11.GL_TEXTURE_2D,0,GL11.GL_RGBA8,img.getWidth(),img.getHeight(),0,GL11.GL_RGBA,GL11.GL_UNSIGNED_BYTE,data);
                ids.put(name,id); return id;
            } catch (Exception e) { throw new RuntimeException(e); }
        }
    }
    static void wolfField(EntityWolf wolf,String name,float v) throws Exception {
        java.lang.reflect.Field f = EntityWolf.class.getDeclaredField(name); f.setAccessible(true); f.setFloat(wolf,v);
    }
    static EntityLiving entity(int i) throws Exception {
        EntityLiving e;
        switch(i) {
        case 0: e = new EntityZombie(null) { public float getEntityBrightness(float p) { return brightness; } public String getEntityTexture() { return "/mob/char.png"; } }; break;
        case 1: e = new EntityZombie(null) { public float getEntityBrightness(float p) { return brightness; } }; break;
        case 2: e = new EntitySkeleton(null) { public float getEntityBrightness(float p) { return brightness; } }; break;
        case 3: e = new EntityPig(null) { public float getEntityBrightness(float p) { return brightness; } }; ((EntityPig)e).setSaddled(variants); break;
        case 4: e = new EntityCow(null) { public float getEntityBrightness(float p) { return brightness; } }; break;
        case 5: e = new EntitySheep(null) { public float getEntityBrightness(float p) { return brightness; } }; ((EntitySheep)e).setFleeceColor(variants ? 14 : 0); break;
        case 6:
            EntityChicken chicken = new EntityChicken(null) { public float getEntityBrightness(float p) { return brightness; } };
            chicken.field_756_e = chicken.field_752_b = variants ? 1.25F : 0;
            chicken.field_757_d = chicken.destPos = variants ? .6F : 0;
            e=chicken; break;
        case 7: e = new EntitySpider(null) { public float getEntityBrightness(float p) { return brightness; } }; break;
        case 8: e = new EntityCreeper(null) { public float getEntityBrightness(float p) { return brightness; } public float setCreeperFlashTime(float p) { return variants ? .75F : 0; } public boolean getPowered() { return variants; } }; break;
        case 9:
            EntitySlime slime = new EntitySlime(null) { public float getEntityBrightness(float p) { return brightness; } };
            slime.setSlimeSize(2); slime.field_767_b = slime.field_768_a = variants ? .4F : 0;
            e=slime; break;
        case 10:
            EntitySquid squid = new EntitySquid(null) { public float getEntityBrightness(float p) { return brightness; } };
            squid.field_21088_b = squid.field_21089_a = variants ? 20 : 0;
            squid.field_21086_f = squid.field_21087_c = variants ? 15 : 0;
            squid.field_21082_j = squid.field_21083_i = variants ? .8F : .25F;
            e=squid; break;
        default:
            EntityWolf wolf = new EntityWolf(null) { public float getEntityBrightness(float p) { return brightness; } };
            wolf.setIsSitting(variants); wolf.setIsTamed(variants);
            wolf.dataWatcher.updateObject(18,Integer.valueOf(8));
            if (variants) { wolfField(wolf,"field_25054_c",.5F); wolfField(wolf,"field_25048_b",1.0F); }
            e=wolf;
        }
        e.prevRotationYaw = e.rotationYaw = e.prevRenderYawOffset = e.renderYawOffset = -25;
        e.ticksExisted = 4;
        if (variants) { if (i==1) e.hurtTime=5; if(i==2) e.deathTime=8; }
        return e;
    }
    static RenderLiving renderer(int i) {
        switch(i) {
        case 0:
            ModelBiped b = new ModelBiped(); b.isSneak=variants; b.field_1278_i=variants;
            // RenderLiving reads attack from the entity; override only that state input.
            return new RenderBiped(b,.5F) { protected float func_167_c(EntityLiving e,float p) { return variants ? .35F : 0; } };
        case 1: return new RenderBiped(new ModelZombie(),.5F);
        case 2: return new RenderBiped(new ModelSkeleton(),.5F);
        case 3: return new RenderPig(new ModelPig(),new ModelPig(.5F),.7F);
        case 4: return new RenderLiving(new ModelCow(),.7F);
        case 5: return new RenderSheep(new ModelSheep2(),new ModelSheep1(),.7F);
        case 6: return new RenderChicken(new ModelChicken(),.3F);
        case 7: return new RenderSpider();
        case 8: return new RenderCreeper();
        case 9: return new RenderSlime(new ModelSlime(16),new ModelSlime(0),.25F);
        case 10: return new RenderSquid(new ModelSquid(),.7F);
        default: return new RenderWolf(new ModelWolf(),.5F);
        }
    }
    public static void main(String[] a) throws Exception {
        Class.forName("net.minecraft.src.BiomeGenBase");
        Scanner scene = new Scanner(new File(a[0])).useLocale(Locale.ROOT); variants = scene.nextInt()!=0;
        Pbuffer pb = new Pbuffer(2560,720,new PixelFormat(8,24,0),null,null);
        pb.makeCurrent();
        GL11.glViewport(0,0,2560,720); GL11.glClearColor(.69F,.79F,.91F,1);
        GL11.glClear(GL11.GL_COLOR_BUFFER_BIT | GL11.GL_DEPTH_BUFFER_BIT);
        GL11.glEnable(GL11.GL_DEPTH_TEST); GL11.glDepthFunc(GL11.GL_LEQUAL);
        GL11.glEnable(GL11.GL_TEXTURE_2D); GL11.glEnable(GL11.GL_ALPHA_TEST); GL11.glAlphaFunc(GL11.GL_GREATER,.1F);
        GL11.glMatrixMode(GL11.GL_PROJECTION); GL11.glLoadIdentity(); GLU.gluPerspective(40,2560F/720F,.05F,256);
        GL11.glMatrixMode(GL11.GL_MODELVIEW); GL11.glLoadIdentity(); GL11.glRotatef(14,1,0,0); GL11.glRotatef(360,0,1,0);
        // Enable lights with only the camera rotation in the modelview, as EntityRenderer does.
        RenderHelper.enableStandardItemLighting();
        RenderManager.instance.renderEngine = new JarTextures();
        // Supply only Minecraft.renderEngine to the real ItemRenderer's sprite path.
        java.lang.reflect.Field uf = sun.misc.Unsafe.class.getDeclaredField("theUnsafe"); uf.setAccessible(true);
        sun.misc.Unsafe unsafe = (sun.misc.Unsafe)uf.get(null);
        net.minecraft.client.Minecraft mc = (net.minecraft.client.Minecraft)unsafe.allocateInstance(MinecraftImpl.class);
        mc.renderEngine = RenderManager.instance.renderEngine;
        ItemRenderer items = (ItemRenderer)unsafe.allocateInstance(ItemRenderer.class);
        java.lang.reflect.Field imc = ItemRenderer.class.getDeclaredField("mc"); imc.setAccessible(true); imc.set(items,mc);
        RenderManager.instance.itemRenderer = items;
        RenderHelper.disableStandardItemLighting();
        GL11.glBindTexture(GL11.GL_TEXTURE_2D,RenderManager.instance.renderEngine.getTexture("/terrain.png"));
        GL11.glColor4f(1,1,1,1);
        GL11.glPushMatrix(); GL11.glTranslatef(0,-4.6F,-13.5F);
        GL11.glBegin(GL11.GL_QUADS);
        for(int x=-20;x<20;x++) for(int z=-5;z<5;z++) {
            GL11.glTexCoord2f(1F/16,0); GL11.glVertex3f(x,0,z);
            GL11.glTexCoord2f(1F/16,1F/16); GL11.glVertex3f(x,0,z+1);
            GL11.glTexCoord2f(2F/16,1F/16); GL11.glVertex3f(x+1,0,z+1);
            GL11.glTexCoord2f(2F/16,0); GL11.glVertex3f(x+1,0,z);
        }
        GL11.glEnd(); GL11.glPopMatrix();
        RenderHelper.enableStandardItemLighting();
        for(int i=0;i<12;i++) {
            double x=scene.nextDouble(),y=scene.nextDouble(),z=scene.nextDouble();
            brightness = variants && i==7 ? .1F : 1;
            EntityLiving e=entity(i); RenderLiving r=renderer(i); r.setRenderManager(RenderManager.instance);
            GL11.glColor3f(brightness,brightness,brightness);
            r.doRenderLiving(e,x,y-4.6,z-13.5,-25,.5F);
        }
        ByteBuffer pixels=BufferUtils.createByteBuffer(2560*720*4);
        GL11.glReadPixels(0,0,2560,720,GL11.GL_RGBA,GL11.GL_UNSIGNED_BYTE,pixels);
        BufferedImage img = new BufferedImage(2560,720,BufferedImage.TYPE_INT_ARGB);
        for(int y=0;y<720;y++) for(int x=0;x<2560;x++) {
            int at=((719-y)*2560+x)*4;
            int c=0xff000000 | (pixels.get(at)&255)<<16 | (pixels.get(at+1)&255)<<8 | (pixels.get(at+2)&255);
            img.setRGB(x,y,c);
        }
        ImageIO.write(img,"png",new File(a[1])); pb.destroy();
        System.out.println("original RenderLiving showcase: " + a[1]);
    }
}
