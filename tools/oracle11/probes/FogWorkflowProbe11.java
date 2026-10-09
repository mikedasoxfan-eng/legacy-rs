package net.minecraft.src; import java.io.*;import java.lang.reflect.*;import org.lwjgl.opengl.GL11;import static net.minecraft.src.Mode_render_models10.*;public final class FogWorkflowProbe11 {
static void commands()throws Exception{Mode_render_round4_10.commands();}
    static void fog()throws Exception{
        i(1536);
        for(int type=0;type<2;type++)for(int creative=0;creative<2;creative++)for(int c=0;c<384;c++){
            World w=FirstPersonOracle.world();PlayerOracle.HeadlessPlayer p=FirstPersonOracle.player(w,c);FirstPersonOracle.setup(p);EntityRenderer er=p.mc.entityRenderer; EnumWorldType wt=type==0?EnumWorldType.DEFAULT:EnumWorldType.FLAT; Field typ=WorldProvider.class.getDeclaredField("field_46067_b");typ.setAccessible(true);typ.set(w.worldProvider,wt);p.capabilities.depleteBuckets=creative!=0;
            boolean nether=(c&8)!=0,noSky=nether||c%5==0;w.worldProvider.isAlternateDimension=nether;w.worldProvider.hasNoSky=noSky;
            w.setWorldTime(c*1379);w.rainingStrength=w.prevRainingStrength=(c%5)*.17F;w.thunderingStrength=w.prevThunderingStrength=(c%7)*.11F;
            p.mc.gameSettings.renderDistance=c%4;p.mc.gameSettings.anaglyph=(c&4)!=0;
            int blind=c%4==1?new int[]{1,5,19,20,200,0}[(c/4)%6]:-1;if(blind>=0)p.addPotionEffect(new PotionEffect(Potion.blindness.id,blind,0));
            boolean breathing=c%6==2;if(breathing)p.addPotionEffect(new PotionEffect(Potion.waterBreathing.id,100,0));
            boolean cloud=c%7==3;Oracle.setField(er,"cloudFog",cloud);
            int id=c%3==1?9:c%3==2?11:0;EntityOracle.rawBlock(w,0,65,-3,id,c%16);EntityOracle.rawBlock(w,0,66,-3,c%9==0?8:0,c%16);
            ActiveRenderInfo.objectX=(c%5-2)*.03F;ActiveRenderInfo.objectY=(c%7-3)*.11F;ActiveRenderInfo.objectZ=(c%3-1)*.05F;
            float t=(c%11)*.1F,old=.3F,newer=.85F,far=(float)(256>>(c%4));int mode=c%3-1;
            if(c%3==0){p.posY=-9.0+(c%41)*.75;p.lastTickPosY=p.posY-.4;}
            Oracle.setField(er,"fogColor2",old);Oracle.setField(er,"fogColor1",newer);Oracle.setField(er,"farPlaneDistance",far);
            for(double v:new double[]{p.posX,p.posY,p.posZ,p.prevPosX,p.prevPosY,p.prevPosZ,p.lastTickPosY})d(v);
            f(p.rotationYaw);f(p.prevRotationYaw);f(p.rotationPitch);f(p.prevRotationPitch);
            f(t);i(c%4);i((c&4)!=0?1:0);i(nether?1:0);i(noSky?1:0);i(mode);f(far);f(old);f(newer);i(blind);i(breathing?1:0);i(cloud?1:0);
            f(ActiveRenderInfo.objectX);f(ActiveRenderInfo.objectY);f(ActiveRenderInfo.objectZ);i(id);i(c%16);i(c%9==0?8:0);
            Vec3D sky=w.getSkyColor(p,t),fog=w.getFogColor(t),clouds=w.drawClouds(t);for(double v:new double[]{sky.xCoord,sky.yCoord,sky.zCoord,fog.xCoord,fog.yCoord,fog.zCoord,clouds.xCoord,clouds.yCoord,clouds.zCoord})f((float)v);
            f(w.getRainStrength(t));f(w.getWeightedThunderStrength(t));f(w.getCelestialAngle(t));i(p.getEntityBrightnessForRender(t));i(type);i(creative);
            GlLog10.reset();GlLog10.active=true;
            Method u=EntityRenderer.class.getDeclaredMethod("updateFogColor",float.class);u.setAccessible(true);u.invoke(er,t);
            Method s=EntityRenderer.class.getDeclaredMethod("setupFog",int.class,float.class);s.setAccessible(true);s.invoke(er,mode,t);
            GlLog10.active=false;commands();
        }
    }


public static void main(String[] a)throws Exception{Block.stone.getClass();out=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[0])));i(0x34524452);i("fog11".hashCode());fog();out.close();System.out.println("FogWorkflow11:1536 original color/setup frames");}}