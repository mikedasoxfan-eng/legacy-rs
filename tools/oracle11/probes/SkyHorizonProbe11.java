package net.minecraft.src;
import java.io.*;
import java.lang.reflect.*;
import net.minecraft.client.Minecraft;
import static net.minecraft.src.Mode_render_models10.*;

/** All dimension sky branches. World values are inputs, draw/state bytecode is original. */
public final class SkyHorizonProbe11 {
    static class SkyWorld extends WorldClient {
        float angle,rain,stars;int phase;float[] rgb;
        SkyWorld(){super(null,null,0,0);}
        public Vec3D getSkyColor(Entity entity,float partial){return Vec3D.createVector(rgb[0],rgb[1],rgb[2]);}
        public float getCelestialAngle(float partial){return angle;}
        public float getCelestialAngleRadians(float partial){return angle*(float)Math.PI*2.0F;}
        public float getRainStrength(float partial){return rain;}
        public float getStarBrightness(float partial){return stars;}
        public int func_40475_d(float partial){return phase;}
    }
    static class Engine extends RenderEngine {
        Engine(){super(null,null);}
        public int getTexture(String path){
            if(path.equals("/misc/tunnel.png"))return 80;
            if(path.equals("/terrain/sun.png"))return 81;
            if(path.equals("/terrain/moon_phases.png"))return 82;
            throw new AssertionError(path);
        }
    }
    public static void main(String[] args)throws Exception {
        Block.stone.getClass();Field field=sun.misc.Unsafe.class.getDeclaredField("theUnsafe");field.setAccessible(true);sun.misc.Unsafe u=(sun.misc.Unsafe)field.get(null);
        Minecraft mc=(Minecraft)u.allocateInstance(MinecraftImpl.class);
        mc.gameSettings=(GameSettings)u.allocateInstance(GameSettings.class);
        mc.thePlayer=(EntityPlayerSP)u.allocateInstance(EntityPlayerSP.class);mc.renderViewEntity=mc.thePlayer;
        SkyWorld world=(SkyWorld)u.allocateInstance(SkyWorld.class);mc.theWorld=world;
        Engine engine=(Engine)u.allocateInstance(Engine.class);mc.renderEngine=engine;
        RenderGlobal global=new RenderGlobal(mc,engine);
        Oracle.setField(global,"worldObj",world);
        out=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(args[0])));i(0x30314752);i(2);i(128);
        for(int type=0;type<2;type++)for(int c=0;c<64;c++) { Field info=World.class.getDeclaredField("worldInfo");info.setAccessible(true);info.set(world,new WorldInfo(new WorldSettings(0,0,true,false,type==0?EnumWorldType.DEFAULT:EnumWorldType.FLAT),"probe"));
            int dimension=c<48?0:c<56?1:-1;boolean anaglyph=(c&1)!=0;
            Field provider=World.class.getDeclaredField("worldProvider");provider.setAccessible(true);
            provider.set(world,dimension==1?new WorldProviderEnd():dimension==-1?new WorldProviderHell():new WorldProviderSurface());
            world.worldProvider.worldType=dimension;
            world.worldProvider.isAlternateDimension=dimension==-1;
            world.angle=new float[]{0,.125F,.25F,.375F,.5F,.625F,.75F,.875F}[c%8];
            world.rain=c%3*.25F;world.stars=c%4*.2F;world.phase=c%16-7;world.rgb=new float[]{.37F,.51F,.83F};
            double height=new double[]{-1,0,62.999,63,64,80,130,30000000}[c%8];
            mc.thePlayer.posY=mc.thePlayer.prevPosY=height;mc.gameSettings.anaglyph=anaglyph;
            i(dimension);i(anaglyph?1:0);f(world.angle);f(world.rain);f(world.stars);i(world.phase);d(height);for(float v:world.rgb)f(v);d(world.func_46068_G());
            RendererOracle.reset10();global.renderSky(.5F);Mode_render_effects10.draws();
        }
        out.close();System.out.println("Original RenderGlobal sky: 128 default/flat dimension, phase, rain, star, sunrise, anaglyph and below-horizon cases");
    }
}
