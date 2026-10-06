package net.minecraft.src;
import java.io.*;
import java.lang.reflect.*;
import java.util.*;
import org.lwjgl.opengl.GL11;
import org.lwjgl.opengl.GLContext;
import static net.minecraft.src.Mode_render_models10.*;

/** Release 1.0 rendering round 4: original RenderItem GUI passes, ActiveRenderInfo, EntityRenderer
 * fog and End lightmap, OpenGlHelper/OpenGlCapsChecker, RenderHelper, Render shadows and the
 * Tessellator. Only GL endpoints and fixtures are substituted. */
public final class Mode_render_round4_10 {
    static void commands()throws Exception{i(GlLog10.commands.size());for(float[] c:GlLog10.commands){i((int)c[0]);for(int k=1;k<5;k++)f(c[k]);}}
    static ItemStack stack(int id,int damage){ItemStack s=new ItemStack(id,1,Math.max(damage,0));if(damage<0)s.addEnchantment(Enchantment.sharpness,1);return s;}

    /** RenderItem.renderItemIntoGUI: potion passes, zLevel, untinted color and the GUI glint. */
    static void gui()throws Exception{
        // The client loads the biome colour maps at startup; grass item tops read them.
        ColorizerGrass.setGrassBiomeColorizer(Oracle.jarImage("misc/grasscolor.png"));ColorizerFoliage.getFoilageBiomeColorizer(Oracle.jarImage("misc/foliagecolor.png"));
        World w=Oracle.newWorld(0);RenderManager m=RendererOracle.manager(w);RendererOracle.field("convertQuadsToTriangles").setBoolean(null,false);
        int[][] stacks={{373,0},{373,16},{373,8193},{373,16385},{373,8226},{373,32767},{322,0},{267,-1},{261,-1},{278,-1},{1,0},{50,0},{35,14},{280,0},{331,0},{2,0},{18,1},{54,0}};
        Random r=new Random(40268);i(stacks.length*8);
        for(int[] s:stacks)for(int c=0;c<8;c++){
            float z=new float[]{0,100,200,300}[c%4];boolean tint=c<4||s[0]==373&&c==7;int x=r.nextInt(450)-50,y=r.nextInt(300)-50;
            float[] base={r.nextFloat(),r.nextFloat(),r.nextFloat(),1};
            RendererOracle.reset10();GL11.color=base.clone();
            RenderItem ri=new RenderItem();ri.field_40268_b=z;ri.field_27004_a=tint;
            ri.renderItemIntoGUI(null,m.renderEngine,stack(s[0],s[1]),x,y);
            // The glint's currentTimeMillis offset, recovered exactly from the last vertex's u (an exact power-of-two scale).
            float[] offsets=new float[2];int pass=0;
            for(RendererOracle.Draw10 d:RendererOracle.draws10)if(d.texture==76&&pass<2)offsets[pass++]=d.face.vertices.get(3)[3]*256.0F;
            i(s[0]);i(s[1]);i(x);i(y);f(z);i(tint?1:0);for(float v:base)f(v);for(float v:offsets)f(v);
            Mode_render_effects10.draws();
        }
    }

    /** ActiveRenderInfo: GLU unprojection of the viewport center, view vectors, eye-block lookup. */
    static void activeInfo()throws Exception{
        Random r=new Random(41067);i(256);
        World w=FirstPersonOracle.world();
        for(int c=0;c<256;c++){
            float[] mv=new float[16],pj=new float[16];
            float yaw=r.nextFloat()*720-360,pitch=r.nextFloat()*180-90;
            if(c%16==15){/* singular: gluUnProject fails and the previous result stays */}
            else {
                double a=Math.toRadians(pitch),b=Math.toRadians(yaw+180);
                float cp=(float)Math.cos(a),sp=(float)Math.sin(a),cy=(float)Math.cos(b),sy=(float)Math.sin(b);
                float tz=c%3==0?-.1F:-(r.nextFloat()*4);
                mv=new float[]{cy,sp*sy,-cp*sy,0, 0,cp,sp,0, sy,-sp*cy,cp*cy,0, r.nextFloat()*.2F-.1F,r.nextFloat()*.2F-.1F,tz,1};
                float fov=30+r.nextFloat()*80,aspect=.5F+r.nextFloat()*2,far=8+r.nextFloat()*500,cot=(float)(1/Math.tan(Math.toRadians(fov/2)));
                pj=new float[]{cot/aspect,0,0,0, 0,cot,0,0, 0,0,-(far+.05F)/(far-.05F),-1, 0,0,-2*.05F*far/(far-.05F),0};
                if(c%5==1){pj[12]=r.nextFloat()*.14F-.07F;}
            }
            int[] vp={c%4==0?r.nextInt(50):0,c%4==0?r.nextInt(50):0,1+r.nextInt(2000),1+r.nextInt(1200)};
            GL11.modelviewInput=mv;GL11.projectionInput=pj;GL11.viewportInput=vp;
            PlayerOracle.HeadlessPlayer p=FirstPersonOracle.player(w,c);p.rotationYaw=yaw;p.rotationPitch=pitch;boolean front=c%3==1;
            ActiveRenderInfo.func_41067_a(p,front);
            for(float v:mv)f(v);for(float v:pj)f(v);for(int v:vp)i(v);f(yaw);f(pitch);i(front?1:0);
            for(float v:new float[]{ActiveRenderInfo.field_41074_a,ActiveRenderInfo.field_41072_b,ActiveRenderInfo.field_41073_c,ActiveRenderInfo.field_41070_d,ActiveRenderInfo.field_41071_e,ActiveRenderInfo.field_41068_f,ActiveRenderInfo.field_41069_g,ActiveRenderInfo.field_41078_h})f(v);
            // Eye block: fluids of every height around the eye, offset by the unprojected near plane.
            World bw=FirstPersonOracle.world();PlayerOracle.HeadlessPlayer e=FirstPersonOracle.player(bw,c);
            int id=new int[]{8,9,10,11,1,0,8,10}[c%8],meta=c%16;
            for(int y=64;y<=67;y++)EntityOracle.rawBlock(bw,0,y,-3,y==66&&c%3==0?(c%2==0?9:3):id,meta);
            e.setPosition(.25,64.1+(c%37)*.05,-2.75);e.prevPosX=.1;e.prevPosY=e.posY-(c%5)*.07;e.prevPosZ=-2.9;
            float t=(c%5)*.25F;
            d(e.posX);d(e.posY);d(e.posZ);d(e.prevPosX);d(e.prevPosY);d(e.prevPosZ);f(t);i(id);i(meta);i(c%3==0?(c%2==0?9:3):id);
            Vec3D at=ActiveRenderInfo.func_41065_a(e,t);d(at.xCoord);d(at.yCoord);d(at.zCoord);i(ActiveRenderInfo.func_41066_a(bw,e,t));
        }
    }

    /** EntityRenderer.updateFogColor + setupFog: blindness, water breathing, hasNoSky, cloud fog. */
    static void fog()throws Exception{
        i(384);
        for(int c=0;c<384;c++){
            World w=FirstPersonOracle.world();PlayerOracle.HeadlessPlayer p=FirstPersonOracle.player(w,c);FirstPersonOracle.setup(p);EntityRenderer er=p.mc.entityRenderer;
            boolean nether=(c&8)!=0,noSky=nether||c%5==0;w.worldProvider.isNether=nether;w.worldProvider.hasNoSky=noSky;
            w.setWorldTime(c*1379);w.rainingStrength=w.prevRainingStrength=(c%5)*.17F;w.thunderingStrength=w.prevThunderingStrength=(c%7)*.11F;
            p.mc.gameSettings.renderDistance=c%4;p.mc.gameSettings.anaglyph=(c&4)!=0;
            int blind=c%4==1?new int[]{1,5,19,20,200,0}[(c/4)%6]:-1;if(blind>=0)p.addPotionEffect(new PotionEffect(Potion.potionBlindness.id,blind,0));
            boolean breathing=c%6==2;if(breathing)p.addPotionEffect(new PotionEffect(Potion.potionWaterBreathing.id,100,0));
            boolean cloud=c%7==3;Oracle.setField(er,"cloudFog",cloud);
            int id=c%3==1?9:c%3==2?11:0;EntityOracle.rawBlock(w,0,65,-3,id,c%16);EntityOracle.rawBlock(w,0,66,-3,c%9==0?8:0,c%16);
            ActiveRenderInfo.field_41074_a=(c%5-2)*.03F;ActiveRenderInfo.field_41072_b=(c%7-3)*.11F;ActiveRenderInfo.field_41073_c=(c%3-1)*.05F;
            float t=(c%11)*.1F,old=.3F,newer=.85F,far=(float)(256>>(c%4));int mode=c%3-1;
            if(c%3==0){p.posY=-9.0+(c%41)*.75;p.lastTickPosY=p.posY-.4;}
            Oracle.setField(er,"fogColor2",old);Oracle.setField(er,"fogColor1",newer);Oracle.setField(er,"farPlaneDistance",far);
            for(double v:new double[]{p.posX,p.posY,p.posZ,p.prevPosX,p.prevPosY,p.prevPosZ,p.lastTickPosY})d(v);
            f(p.rotationYaw);f(p.prevRotationYaw);f(p.rotationPitch);f(p.prevRotationPitch);
            f(t);i(c%4);i((c&4)!=0?1:0);i(nether?1:0);i(noSky?1:0);i(mode);f(far);f(old);f(newer);i(blind);i(breathing?1:0);i(cloud?1:0);
            f(ActiveRenderInfo.field_41074_a);f(ActiveRenderInfo.field_41072_b);f(ActiveRenderInfo.field_41073_c);i(id);i(c%16);i(c%9==0?8:0);
            Vec3D sky=w.getSkyColor(p,t),fog=w.getFogColor(t),clouds=w.drawClouds(t);for(double v:new double[]{sky.xCoord,sky.yCoord,sky.zCoord,fog.xCoord,fog.yCoord,fog.zCoord,clouds.xCoord,clouds.yCoord,clouds.zCoord})f((float)v);
            f(w.getRainStrength(t));f(w.getWeightedThunderStrength(t));f(w.getCelestialAngle(t));i(p.getEntityBrightnessForRender(t));
            GlLog10.reset();GlLog10.active=true;
            Method u=EntityRenderer.class.getDeclaredMethod("updateFogColor",float.class);u.setAccessible(true);u.invoke(er,t);
            Method s=EntityRenderer.class.getDeclaredMethod("setupFog",int.class,float.class);s.setAccessible(true);s.invoke(er,mode,t);
            GlLog10.active=false;commands();
        }
    }

    /** EntityRenderer.updateLightmap in the End (worldType 1) and the Overworld for contrast. */
    static void lightmapEnd()throws Exception{
        net.minecraft.client.Minecraft mc=(net.minecraft.client.Minecraft)Mode_lightmap.unsafeAlloc(MinecraftImpl.class);mc.gameSettings=new GameSettings();
        Mode_lightmap.CaptureEngine eng=(Mode_lightmap.CaptureEngine)Mode_lightmap.unsafeAlloc(Mode_lightmap.CaptureEngine.class);mc.renderEngine=eng;
        EntityRenderer er=(EntityRenderer)Mode_lightmap.unsafeAlloc(EntityRenderer.class);Oracle.setField(er,"mc",mc);Oracle.setField(er,"lightmapColors",new int[256]);
        Method upd=EntityRenderer.class.getDeclaredMethod("updateLightmap");upd.setAccessible(true);
        World end=new World(new Oracle.NullSaveHandler(0),"oracle",Oracle.settings(0),WorldProvider.getProviderForDimension(1)),over=Oracle.newWorld(0);
        Random r=new Random(1001);i(96);
        for(int c=0;c<96;c++){
            World w=c%4==3?over:end;mc.theWorld=w;w.worldInfo.setWorldTime(r.nextInt(24000));
            w.prevRainingStrength=w.rainingStrength=c%3==0?0:r.nextFloat();w.prevThunderingStrength=w.thunderingStrength=c%4==0?0:r.nextFloat();
            w.lightningFlash=c%7==3?2:0;float flicker=c%6==0?0:(r.nextFloat()-.5F)*.8F;Oracle.setField(er,"torchFlickerX",flicker);
            float gamma=c%4==1?0:c%4==2?1:r.nextFloat();mc.gameSettings.gammaSetting=gamma;
            upd.invoke(er);
            for(float t:w.worldProvider.lightBrightnessTable)f(t);f(w.func_35464_b(1.0F));f(flicker);i(w.lightningFlash>0?1:0);f(gamma);i(w.worldProvider.worldType);
            for(int v:eng.last)i(v);
        }
    }

    /** OpenGlHelper texture units under every capability combination, and OpenGlCapsChecker. */
    static void glHelper()throws Exception{
        Random r=new Random(84);i(64);
        Field arb=OpenGlHelper.class.getDeclaredField("useMultitextureARB");arb.setAccessible(true);
        for(int c=0;c<64;c++){
            boolean multitexture=(c&1)!=0,gl13=(c&2)!=0,occlusion=(c&4)!=0;
            GLContext.getCapabilities().GL_ARB_multitexture=multitexture;GLContext.getCapabilities().OpenGL13=gl13;GLContext.getCapabilities().GL_ARB_occlusion_query=occlusion;
            OpenGlHelper.lightmapDisabled=-1;OpenGlHelper.lightmapEnabled=-1;
            OpenGlHelper.initializeTextures();
            i(c);i(OpenGlHelper.lightmapDisabled);i(OpenGlHelper.lightmapEnabled);i(arb.getBoolean(null)?1:0);i(OpenGlCapsChecker.checkARBOcclusion()?1:0);
            float s=r.nextFloat()*240,t=r.nextFloat()*240;f(s);f(t);
            GlLog10.reset();
            // The ARB entry points need a live context; on 1.3+ drivers the GL13 branch is taken.
            if(!arb.getBoolean(null)){
                GlLog10.active=true;
                OpenGlHelper.setActiveTexture(OpenGlHelper.lightmapEnabled);OpenGlHelper.setLightmapTextureCoords(OpenGlHelper.lightmapEnabled,s,t);
                OpenGlHelper.setClientActiveTexture(OpenGlHelper.lightmapEnabled);OpenGlHelper.setClientActiveTexture(OpenGlHelper.lightmapDisabled);OpenGlHelper.setActiveTexture(OpenGlHelper.lightmapDisabled);
                GlLog10.active=false;
            }
            commands();
        }
        GLContext.getCapabilities().GL_ARB_multitexture=false;GLContext.getCapabilities().OpenGL13=true;GLContext.getCapabilities().GL_ARB_occlusion_query=false;
        OpenGlHelper.initializeTextures();org.lwjgl.opengl.GL13.activeTexture=0;
    }

    /** RenderHelper: standard item lighting, the 1.0 GUI variant (func_41089_c) and disabling. */
    static void renderHelper()throws Exception{
        i(3);
        for(int c=0;c<3;c++){
            GlLog10.reset();GlLog10.active=true;
            if(c==0)RenderHelper.enableStandardItemLighting();else if(c==1)RenderHelper.func_41089_c();else RenderHelper.disableStandardItemLighting();
            GlLog10.active=false;i(c);commands();
        }
    }

    /** Render.renderShadow: 1.0 halves the radius of baby animals. */
    static void shadow()throws Exception{
        World w=Oracle.newWorld(0);w.setWorldTime(6000);w.skylightSubtracted=0;
        for(int x=-2;x<=1;x++)for(int z=-2;z<=1;z++){w.getBlockId(x*16,64,z*16);Oracle.drainLighting(w);}
        for(int x=-5;x<=5;x++)for(int z=-5;z<=5;z++){w.setBlock(x,68+(Math.abs(x+z)%3),z,new int[]{1,20,44,53,18}[Math.abs(x+z)%5]);for(int y=71;y<=90;y++)w.setBlock(x,y,z,0);}
        Oracle.drainLighting(w);RenderManager rm=RendererOracle.manager(w);RendererOracle.field("convertQuadsToTriangles").setBoolean(null,false);
        // The constructed RenderManager registers every 1.0 entity renderer.
        RenderManager m=RenderManager.instance;m.worldObj=w;m.renderEngine=rm.renderEngine;m.options=rm.options;
        Method shadow=Render.class.getDeclaredMethod("renderShadow",Entity.class,double.class,double.class,double.class,float.class,float.class);shadow.setAccessible(true);
        Field shadowSize=Render.class.getDeclaredField("shadowSize");shadowSize.setAccessible(true);Field opaque=Render.class.getDeclaredField("field_194_c");opaque.setAccessible(true);
        String[] kinds={"Pig","Cow","Sheep","Chicken","Wolf","MushroomCow","Zombie","Villager"};
        i(kinds.length*2*4);
        for(int k=0;k<kinds.length;k++)for(int child=0;child<2;child++)for(float partial:new float[]{0,.25F,.75F,1}){
            EntityLiving e=(EntityLiving)EntityList.createEntityInWorld(kinds[k],w);e.setPosition(.25,70.5,-.75);e.lastTickPosX=-.125;e.lastTickPosY=69.875;e.lastTickPosZ=-1.25;
            if(child==1&&e instanceof EntityAnimal)((EntityAnimal)e).func_40142_a_(-24000);
            Render renderer=m.getEntityRenderObject(e);
            float radius=shadowSize.getFloat(renderer)*e.func_35159_aC();if(e instanceof EntityAnimal&&((EntityAnimal)e).func_40127_l())radius*=.5F;
            double[] cam={0,69,4};
            i(k);i(child);f(partial);for(double v:new double[]{e.posX,e.posY,e.posZ,e.lastTickPosX,e.lastTickPosY,e.lastTickPosZ})d(v);for(double v:cam)d(v);
            double px=e.lastTickPosX+(e.posX-e.lastTickPosX)*partial,py=e.lastTickPosY+(e.posY-e.lastTickPosY)*partial,pz=e.lastTickPosZ+(e.posZ-e.lastTickPosZ)*partial;
            ArrayList<double[]> surfaces=new ArrayList<double[]>();
            for(int x=MathHelper.floor_double(px-radius);x<=MathHelper.floor_double(px+radius);x++)for(int y=MathHelper.floor_double(py+e.getShadowSize()-radius);y<=MathHelper.floor_double(py+e.getShadowSize());y++)for(int z=MathHelper.floor_double(pz-radius);z<=MathHelper.floor_double(pz+radius);z++){
                int id=w.getBlockId(x,y-1,z);if(id>0&&w.getBlockLightValue(x,y,z)>3)surfaces.add(new double[]{x,y,z,id,w.getLightBrightness(x,y,z)});
            }
            i(surfaces.size());for(double[] s:surfaces){i((int)s[0]);i((int)s[1]);i((int)s[2]);i((int)s[3]);f((float)s[4]);}
            m.field_1222_l=cam[0];m.field_1221_m=cam[1];m.field_1220_n=cam[2];
            float alpha=(float)((1-m.getDistanceToCamera(e.posX,e.posY,e.posZ)/256.0)*opaque.getFloat(renderer));
            RendererOracle.reset10();shadow.invoke(renderer,e,px-cam[0],py-cam[1],pz-cam[2],alpha,partial);
            Mode_render_effects10.draws();
        }
    }

    /** Tessellator: 1.0 draw() returns the bytes drawn; brightness uses OpenGlHelper's client unit. */
    static void tessellator()throws Exception{
        OpenGlHelper.initializeTextures();
        Tessellator t=Tessellator.instance;RendererOracle.field("convertQuadsToTriangles").setBoolean(null,false);
        Field raw=Tessellator.class.getDeclaredField("rawBuffer");raw.setAccessible(true);
        Random r=new Random(353);i(128);
        for(int c=0;c<128;c++){
            int n=c%16==0?0:1+r.nextInt(40);boolean bright=(c&1)!=0;double[] translate={r.nextInt(64)-32,r.nextInt(128),r.nextInt(64)-32};
            i(n);i(bright?1:0);for(double v:translate)d(v);
            int[] snapshot=new int[n*8];
            t.startDrawingQuads();t.setTranslationD(-translate[0],-translate[1],-translate[2]);
            for(int v=0;v<n;v++){
                int color=r.nextInt();int light=r.nextInt(16)<<20|r.nextInt(16)<<4;double x=r.nextDouble()*20-4,y=r.nextDouble()*140,z=r.nextDouble()*20-4,u=r.nextDouble(),vv=r.nextDouble();
                i(color);i(light);d(x);d(y);d(z);d(u);d(vv);
                t.setColorOpaque_I(color);if(bright)t.setBrightness(light);t.addVertexWithUV(x,y,z,u,vv);
            }
            int[] buf=(int[])raw.get(t);for(int k=0;k<n*8;k++)snapshot[k]=buf[k];
            GlLog10.reset();GlLog10.active=true;RendererOracle.active=false;
            int bytes=t.draw();GlLog10.active=false;t.setTranslationD(0,0,0);
            i(bytes);for(int v=0;v<n;v++){for(int k=0;k<6;k++)i(snapshot[v*8+k]);i(bright?snapshot[v*8+7]:0);}
            commands();
        }
    }

    public static void main(String[] a)throws Exception{
        String kind=a[0];out=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])));i(0x34524452);i(kind.hashCode());
        if(kind.equals("gui"))gui();else if(kind.equals("activeinfo"))activeInfo();else if(kind.equals("fog"))fog();else if(kind.equals("lightmap"))lightmapEnd();
        else if(kind.equals("glhelper"))glHelper();else if(kind.equals("renderhelper"))renderHelper();else if(kind.equals("shadow"))shadow();else if(kind.equals("tessellator"))tessellator();
        else throw new IllegalArgumentException(kind);
        out.close();System.out.println("Original 1.0 rendering round 4: "+kind);
    }
}
