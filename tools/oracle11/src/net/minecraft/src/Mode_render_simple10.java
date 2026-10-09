package net.minecraft.src;
import java.io.*;
import java.lang.reflect.Field;
import java.util.*;
import org.lwjgl.opengl.GL11;
import org.lwjgl.opengl.GL13;
import static net.minecraft.src.Mode_render_models10.*;
/** The simple 1.0 entity renderers through their original bytecode: the mob
 *  renderers' pass callbacks (int results), eye/carried-block lightmap
 *  coordinates, RenderXPOrb, RenderPainting and RenderFallingSand.
 *  kind 0..7: Sheep, Pig, Creeper, Spider, CaveSpider, Slime, Enderman, Silverfish;
 *  8: XP orb; 9: painting; 10: falling sand/gravel/dragon egg (raw Tessellator words). */
public final class Mode_render_simple10 {
    static int texture(String path){
        String[] names={"/mob/sheep.png","/mob/sheep_fur.png","/mob/pig.png","/mob/saddle.png","/mob/creeper.png","/armor/power.png",
            "/mob/spider.png","/mob/spider_eyes.png","/mob/cavespider.png","/mob/slime.png","/mob/enderman.png","/mob/enderman_eyes.png",
            "/terrain.png","/mob/silverfish.png","/item/xporb.png","/art/kz.png"};
        int[] ids={50,51,47,48,57,58,43,44,45,59,40,41,1,42,30,16};
        for(int n=0;n<names.length;n++)if(path.equals(names[n]))return ids[n];
        throw new AssertionError(path);
    }
    static void tex(String name){GL11.texture=texture(name);}
    /** Draw10 records plus the lightmap coordinates in force at each draw. */
    static void draws()throws Exception{
        i(RendererOracle.draws10.size());
        for(RendererOracle.Draw10 v:RendererOracle.draws10){Mode_render_effects10.ops(v.ops);i(v.cull);i(v.flags);i(v.texture);i(v.src);i(v.dst);i(v.depth);i(v.face.mode);f(v.alpha);for(float a:v.material)f(a);i(v.face.vertices.size());
            for(int n=0;n<v.face.vertices.size();n++){i(v.face.normals.get(n));i(v.face.colors.get(n));for(float a:v.face.vertices.get(n))f(a);}
            f(v.lightmapU);f(v.lightmapV);}
        Mode_render_effects10.ops(RendererOracle.operations10());i(RendererOracle.flags);i(RendererOracle.src);i(RendererOracle.dst);i(RendererOracle.depth);f(RendererOracle.alpha10);for(float a:GL11.color)f(a);
        f(GL13.lightmapU);f(GL13.lightmapV);
    }
    static RenderLiving mobRenderer(int kind){
        switch(kind){
        case 0:return new RenderSheep(new ModelSheep2(),new ModelSheep1(),.7F){protected void loadTexture(String n){tex(n);}protected boolean loadDownloadableImageTexture(String u,String n){tex(n);return true;}protected void passSpecialRender(EntityLiving e,double x,double y,double z){}};
        case 1:return new RenderPig(new ModelPig(),new ModelPig(.5F),.7F){protected void loadTexture(String n){tex(n);}protected boolean loadDownloadableImageTexture(String u,String n){tex(n);return true;}protected void passSpecialRender(EntityLiving e,double x,double y,double z){}};
        case 2:return new RenderCreeper(){protected void loadTexture(String n){tex(n);}protected boolean loadDownloadableImageTexture(String u,String n){tex(n);return true;}protected void passSpecialRender(EntityLiving e,double x,double y,double z){}};
        case 3:case 4:return new RenderSpider(){protected void loadTexture(String n){tex(n);}protected boolean loadDownloadableImageTexture(String u,String n){tex(n);return true;}protected void passSpecialRender(EntityLiving e,double x,double y,double z){}};
        case 5:return new RenderSlime(new ModelSlime(16),new ModelSlime(0),.25F){protected void loadTexture(String n){tex(n);}protected boolean loadDownloadableImageTexture(String u,String n){tex(n);return true;}protected void passSpecialRender(EntityLiving e,double x,double y,double z){}};
        case 6:return new RenderEnderman(){protected void loadTexture(String n){tex(n);}protected boolean loadDownloadableImageTexture(String u,String n){tex(n);return true;}protected void passSpecialRender(EntityLiving e,double x,double y,double z){}};
        default:return new RenderSilverfish(){protected void loadTexture(String n){tex(n);}protected boolean loadDownloadableImageTexture(String u,String n){tex(n);return true;}protected void passSpecialRender(EntityLiving e,double x,double y,double z){}};
        }
    }
    static int light;
    static EntityLiving mob(int kind){
        switch(kind){
        case 0:return new EntitySheep(null){public float getEntityBrightness(float t){return .73F;}public int getEntityBrightnessForRender(float t){return light;}};
        case 1:return new EntityPig(null){public float getEntityBrightness(float t){return .73F;}public int getEntityBrightnessForRender(float t){return light;}};
        case 2:return new EntityCreeper(null){public float getEntityBrightness(float t){return .73F;}public int getEntityBrightnessForRender(float t){return light;}};
        case 3:return new EntitySpider(null){public float getEntityBrightness(float t){return .73F;}public int getEntityBrightnessForRender(float t){return light;}};
        case 4:return new EntityCaveSpider(null){public float getEntityBrightness(float t){return .73F;}public int getEntityBrightnessForRender(float t){return light;}};
        case 5:return new EntitySlime(null){public float getEntityBrightness(float t){return .73F;}public int getEntityBrightnessForRender(float t){return light;}};
        case 6:return new EntityEnderman(null){public float getEntityBrightness(float t){return .73F;}public int getEntityBrightnessForRender(float t){return light;}};
        default:return new EntitySilverfish(null){public float getEntityBrightness(float t){return .73F;}public int getEntityBrightnessForRender(float t){return light;}};
        }
    }
    static void mobs(int kind,Random random)throws Exception{
        RenderLiving renderer=mobRenderer(kind);renderer.setRenderManager(RenderManager.instance);
        f(renderer.shadowSize);
        Field rnd=null;if(kind==6){rnd=RenderEnderman.class.getDeclaredField("rnd");rnd.setAccessible(true);}
        for(int c=0;c<64;c++){
            EntityLiving entity=mob(kind);
            double[] position={(random.nextDouble()-.5)*128,(random.nextDouble()-.5)*128,(random.nextDouble()-.5)*128};
            float partial=c%4*.25F,body=random.nextFloat()*720-360,yaw=random.nextFloat()*720-360,pitch=random.nextFloat()*180-90;
            float amount=random.nextFloat()*1.5F,swing=random.nextFloat()*100,attack=random.nextFloat();
            int ticks=random.nextInt(10000),hurt=c%5==0?3:0,death=c%7==0?c:0;
            light=(random.nextInt(16)<<20)|(random.nextInt(16)<<4);
            int start=c%3==0?light:(random.nextInt(16)<<20)|(random.nextInt(16)<<4);
            // kind-specific state: a = fleece/saddle/powered/size/carried, b = sheared/child/ignition/squish/data, e = scream seed
            int a=0,b=0,child=0,attacking=0;long seed=0;float flash=0,squish0=0,squish1=0;
            if(kind==0){a=random.nextInt(16);b=random.nextInt(2);child=random.nextInt(2);EntitySheep s=(EntitySheep)entity;s.setFleeceColor(a);s.setSheared(b!=0);}
            if(kind==1){a=random.nextInt(2);child=random.nextInt(2);((EntityPig)entity).setSaddled(a!=0);}
            if(kind==0||kind==1)((EntityAnimal)entity).setDelay(child!=0?-100-random.nextInt(1000):0);
            if(kind==2){EntityCreeper e=(EntityCreeper)entity;a=random.nextInt(2);if(a!=0)e.dataWatcher.updateObject(17,Byte.valueOf((byte)1));
                e.lastActiveTime=random.nextInt(31);e.timeSinceIgnited=random.nextInt(31);flash=e.setCreeperFlashTime(partial);}
            if(kind==5){EntitySlime e=(EntitySlime)entity;a=1<<random.nextInt(3);e.setSlimeSize(a);squish0=random.nextFloat()*2-1;squish1=random.nextFloat()*2-1;e.field_767_b=squish0;e.field_768_a=squish1;}
            if(kind==6){EntityEnderman e=(EntityEnderman)entity;a=c%4==0?0:new int[]{1,2,3,4,12,13,18,20,35,46,81,103}[random.nextInt(12)];b=random.nextInt(16);e.setCarried(a);e.setCarryingData(b);
                attacking=random.nextInt(2);e.isAttacking=attacking!=0;seed=random.nextLong();rnd.set(renderer,new Random(seed));}
            for(double v:position)d(v);for(float v:new float[]{partial,body,yaw,pitch,amount,swing,attack,flash,squish0,squish1})f(v);
            for(int v:new int[]{ticks,hurt,death,light,start,a,b,child,attacking})i(v);out.writeLong(Long.reverseBytes(seed));
            entity.prevRenderYawOffset=entity.renderYawOffset=body;entity.prevRotationYaw=entity.rotationYaw=yaw;entity.prevRotationPitch=entity.rotationPitch=pitch;
            entity.field_705_Q=entity.field_704_R=amount;entity.field_703_S=swing;entity.prevSwingProgress=entity.swingProgress=attack;entity.ticksExisted=ticks;entity.hurtTime=hurt;entity.deathTime=death;
            RendererOracle.reset10();GL13.lightmapU=start%65536;GL13.lightmapV=start/65536;
            renderer.doRender(entity,position[0],position[1],position[2],0,partial);draws();
        }
    }
    static void orbs(Random random)throws Exception{
        RenderXPOrb renderer=new RenderXPOrb(){protected void loadTexture(String n){tex(n);}};renderer.setRenderManager(RenderManager.instance);
        for(int c=0;c<64;c++){
            int value=c<16?new int[]{1,2,3,6,7,16,17,36,37,72,73,148,149,306,307,2477}[c]:random.nextInt(3000);
            final int packed=(random.nextInt(16)<<20)|(random.nextInt(16)<<4);
            EntityXPOrb orb=new EntityXPOrb(null,0,0,0,value){public int getEntityBrightnessForRender(float t){return packed;}public float getEntityBrightness(float t){return .37F;}};
            orb.xpColor=random.nextInt(100000);
            double x=(random.nextDouble()-.5)*64,y=(random.nextDouble()-.5)*64,z=(random.nextDouble()-.5)*64;
            float yaw=random.nextFloat()*1080-540,pitch=random.nextFloat()*360-180,partial=c%4*.25F;
            RenderManager.instance.playerViewY=yaw;RenderManager.instance.playerViewX=pitch;
            i(value);i(orb.xpColor);i(packed);d(x);d(y);d(z);f(yaw);f(pitch);f(partial);
            RendererOracle.reset10();GL13.lightmapU=-1;GL13.lightmapV=-1;
            renderer.doRender(orb,x,y,z,0,partial);draws();
        }
    }
    static void paintings(Random random)throws Exception{
        World w=Oracle.newWorld(0);w.chunkProvider=new EntityOracle.FlatProvider(w);
        RenderManager m=RendererOracle.manager(w);
        RenderPainting renderer=new RenderPainting(){protected void loadTexture(String n){tex(n);}};renderer.setRenderManager(m);
        i(EnumArt.values().length*4*3);
        for(EnumArt art:EnumArt.values())for(int dir=0;dir<4;dir++)for(int scene=0;scene<3;scene++){
            EntityPainting p=new EntityPainting(w,0,67,0,dir,art.title);
            i(art.ordinal());i(dir);
            ArrayList<Integer> lights=new ArrayList<Integer>();
            for(int tx=0;tx<art.sizeX/16;tx++)for(int ty=0;ty<art.sizeY/16;ty++){
                int[] xyz=DecorationOracle.paintingLight(p,-art.sizeX/2.0F+tx*16+8,-art.sizeY/2.0F+ty*16+8);
                Chunk c=w.getChunkFromBlockCoords(xyz[0],xyz[2]);
                c.blocklightMap.setNibble(xyz[0]&15,xyz[1],xyz[2]&15,scene==0?0:random.nextInt(16));
                c.skylightMap.setNibble(xyz[0]&15,xyz[1],xyz[2]&15,scene==1?15:random.nextInt(16));
            }
            for(int tx=0;tx<art.sizeX/16;tx++)for(int ty=0;ty<art.sizeY/16;ty++){
                int[] xyz=DecorationOracle.paintingLight(p,-art.sizeX/2.0F+tx*16+8,-art.sizeY/2.0F+ty*16+8);
                i(w.getLightBrightnessForSkyBlocks(xyz[0],xyz[1],xyz[2],0));
            }
            float yaw=dir*90;double x=-.25,y=10.5,z=3.75;
            RendererOracle.reset10();GL13.lightmapU=-1;GL13.lightmapV=-1;
            renderer.doRender(p,x,y,z,yaw,0);draws();
        }
    }
    static void falling(Random random)throws Exception{
        Oracle.DataOut cases=new Oracle.DataOut();int count=0;
        for(boolean ao:new boolean[]{false,true}){
            Oracle.meshSetup(true,ao);
            World w=Oracle.newWorld(0);w.chunkProvider=new EntityOracle.FlatProvider(w);w.skylightSubtracted=3;
            for(int cx=-2;cx<=1;cx++)for(int cz=-2;cz<=1;cz++){
                Chunk c=w.getChunkFromChunkCoords(cx,cz);
                for(int x=0;x<16;x++)for(int y=0;y<128;y++)for(int z=0;z<16;z++){
                    int wx=cx*16+x,wz=cz*16+z;
                    c.skylightMap.setNibble(x,y,z,(wx*7+y*3+wz*5)&15);
                    c.blocklightMap.setNibble(x,y,z,(wx*3+y*11+wz)&15);
                }
            }
            for(int x=-3;x<=3;x++)for(int z=-3;z<=3;z++)if((x*5+z*3&3)==0)EntityOracle.rawBlock(w,x,70+((x+z)&1),z,new int[]{1,20,12,49}[(x+z+8)%4],0);
            RenderManager m=RendererOracle.manager(w);
            RenderFallingSand renderer=new RenderFallingSand(){protected void loadTexture(String n){tex(n);}};renderer.setRenderManager(m);
            for(int id:new int[]{12,13,122})for(int c=0;c<24;c++){
                double px=(random.nextInt(13)-6)+random.nextInt(8)/8.0,py=69+random.nextInt(4)+random.nextInt(8)/8.0,pz=(random.nextInt(13)-6)+random.nextInt(8)/8.0;
                EntityFallingSand e=new EntityFallingSand(w,px,py,pz,id);
                double[] rel={(random.nextDouble()-.5)*32,(random.nextDouble()-.5)*32,(random.nextDouble()-.5)*32};
                for(int v:new int[]{id,ao?1:0})cases.i32(v);for(double v:new double[]{e.posX,e.posY,e.posZ,rel[0],rel[1],rel[2]})cases.i64(Double.doubleToRawLongBits(v));
                RendererOracle.reset(1);RendererOracle.active=true;
                RendererOracle.pistonCapture=true;RendererOracle.pistonRaw=null;
                renderer.doRender(e,rel[0],rel[1],rel[2],0,.5F);
                RendererOracle.pistonCapture=false;RendererOracle.active=false;
                RendererOracle.Raw raw=RendererOracle.pistonRaw;
                cases.i32(raw==null?0:raw.n);cases.i32(raw!=null&&raw.packedLight?1:0);
                if(raw!=null)for(int v=0;v<raw.n;v++){for(int f=0;f<6;f++)cases.i32(raw.data[v*8+f]);cases.i32(raw.data[v*8+7]);}
                count++;
            }
        }
        i(count);out.flush();cases.writeTo(out);
    }
    public static void main(String[] args)throws Exception{
        int kind=Integer.parseInt(args[0]);Class.forName("net.minecraft.src.Block");
        ColorizerGrass.setGrassBiomeColorizer(Oracle.jarImage("misc/grasscolor.png"));
        ColorizerFoliage.getFoilageBiomeColorizer(Oracle.jarImage("misc/foliagecolor.png"));
        OpenGlHelper.lightmapDisabled=33984;OpenGlHelper.lightmapEnabled=33985;
        RendererOracle.field("convertQuadsToTriangles").setBoolean(null,false);
        out=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(args[1])));i(0x58465231);i(1);i(kind);
        Random random=new Random(1009001+kind);
        if(kind<8)mobs(kind,random);else if(kind==8)orbs(random);else if(kind==9)paintings(random);else falling(random);
        out.close();System.out.println("Original 1.0 simple renderer kind "+kind);
    }
}
