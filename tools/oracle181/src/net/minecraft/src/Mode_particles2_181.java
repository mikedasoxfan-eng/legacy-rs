package net.minecraft.src;
import java.io.*;
import java.util.*;
import java.lang.reflect.*;
/** Runs the original EntityFX classes and reads original Tessellator buffers. */
public final class Mode_particles2_181 {
    static final class PickupRender extends Render {
        double[] position;float yaw,partial;
        public void doRender(Entity e,double x,double y,double z,float yaw,float partial){position=new double[]{x,y,z};this.yaw=yaw;this.partial=partial;}
        public void doRenderShadowAndFire(Entity e,double x,double y,double z,float yaw,float partial) {}
    }

    static EntityFX create(int kind,World w,RenderEngine engine,double x,double y,double z,double vx,double vy,double vz,int block,int meta) {
        switch(kind) {
        case 0:return new EntityFX(w,x,y,z,vx,vy,vz);case 1:return new EntitySmokeFX(w,x,y,z,vx,vy,vz);case 2:return new EntitySmokeFX(w,x,y,z,vx,vy,vz,2.5F);
        case 3:return new EntityFlameFX(w,x,y,z,vx,vy,vz);case 4:return new EntityExplodeFX(w,x,y,z,vx,vy,vz);case 5:return new EntitySplashFX(w,x,y,z,vx,vy,vz);case 6:return new EntityRainFX(w,x,y,z);
        case 7:return new EntityBubbleFX(w,x,y,z,vx,vy,vz);case 8:return new EntityPortalFX(w,x,y,z,vx,vy,vz);case 9:return new EntityNoteFX(w,x,y,z,vx,vy,vz);case 10:return new EntityReddustFX(w,x,y,z,(float)vx,(float)vy,(float)vz);
        case 11:return new EntityHeartFX(w,x,y,z,vx,vy,vz);case 12:return new EntityLavaFX(w,x,y,z);case 13:return new EntitySlimeFX(w,x,y,z,Item.snowball);case 14:return new EntitySlimeFX(w,x,y,z,Item.slimeBall);
        case 15:return new EntitySnowShovelFX(w,x,y,z,vx,vy,vz);case 16:return new EntityDiggingFX(w,x,y,z,vx,vy,vz,Block.blocksList[block],2,meta);case 17:return new EntityFootStepFX(engine,w,x,y,z);case 18:return new EntityCritFX(w,x,y,z,vx,vy,vz);case 19:{EntityCritFX p=new EntityCritFX(w,x,y,z,vx,vy,vz);p.particleRed*=0.3F;p.particleGreen*=0.8F;p.particleTextureIndex++;return p;}case 20:return new EntityCloudFX(w,x,y,z,vx,vy,vz);case 21:return new EntityAuraFX(w,x,y,z,vx,vy,vz);case 22:return new EntitySuspendFX(w,x,y,z,vx,vy,vz);case 23:return new EntityLargeExplodeFX(engine,w,x,y,z,vx,vy,vz);default:throw new AssertionError(kind);
        }
    }
    static void state(EntityFX p,DataOutputStream o)throws Exception {
        for(double n:new double[]{p.posX,p.posY,p.posZ,p.prevPosX,p.prevPosY,p.prevPosZ,p.motionX,p.motionY,p.motionZ,p.boundingBox.minX,p.boundingBox.minY,p.boundingBox.minZ,p.boundingBox.maxX,p.boundingBox.maxY,p.boundingBox.maxZ})EntityOracle.d(o,n);
        for(float n:new float[]{p.particleRed,p.particleGreen,p.particleBlue,p.particleScale,p.particleGravity,p.particleTextureJitterX,p.particleTextureJitterY,p.width,p.height,p.yOffset})EntityOracle.f(o,n);
        for(int n:new int[]{p.particleTextureIndex,p.particleAge,p.particleMaxAge,p.getFXLayer(),p instanceof EntityFootStepFX?EntityOracle.intField(EntityFootStepFX.class,"field_27018_a",p):p instanceof EntityPickupFX?EntityOracle.intField(EntityPickupFX.class,"age",p):p instanceof EntityLargeExplodeFX?EntityOracle.intField(EntityLargeExplodeFX.class,"field_35130_a",p):p instanceof EntityHugeExplodeFX?EntityOracle.intField(EntityHugeExplodeFX.class,"field_35139_a",p):p instanceof EntityCrit2FX?EntityOracle.intField(EntityCrit2FX.class,"field_35133_ay",p):0})EntityOracle.i(o,n);
        o.writeBoolean(p.isDead);o.writeBoolean(p.onGround);o.writeBoolean(p.noClip);o.writeBoolean(p.isInWeb);EntityOracle.l(o,EntityOracle.rngState(p.rand));EntityOracle.l(o,EntityOracle.rngState(EntityOracle.mathRandom()));
    }
    public static void run(String path)throws Exception {
        World w=Oracle.newWorld(0);w.chunkProvider=new EntityOracle.FlatProvider(w);w.rand.setSeed(300);w.skylightSubtracted=0;
        for(Chunk c:((EntityOracle.FlatProvider)w.chunkProvider).chunks.values()) {Arrays.fill(c.skylightMap.data,(byte)255);Arrays.fill(c.heightMap,(byte)64);}
        for(int x=-4;x<=4;x++)for(int z=-4;z<=4;z++) {
            if(x==2)for(int y=64;y<68;y++)EntityOracle.rawBlock(w,x,y,z,1,0);
            if(x==-2)EntityOracle.rawBlock(w,x,64,z,53,z&3);
            if(x==0&&z<0)EntityOracle.rawBlock(w,x,64,z,9,3);
            if(x==-1&&z<0)EntityOracle.rawBlock(w,x,64,z,30,0);
            if(x==1&&z<0)EntityOracle.rawBlock(w,x,63,z,88,0);
        }
        for(int x=-20;x<=20;x++)for(int z=-20;z<=20;z++)for(int y=0;y<128;y++) {
            Chunk c=w.getChunkFromChunkCoords(x>>4,z>>4);c.skylightMap.setNibble(x&15,y,z&15,(x+y+z)&15);c.blocklightMap.setNibble(x&15,y,z&15,(x*3+y*7+z*5)&15);
        }
        Mode_entities181.Player player=new Mode_entities181.Player(w);player.setPosition(.5,64.1,1.5);w.playerEntities.add(player);
        RenderEngine engine=(RenderEngine)Oracle.allocate(RenderEngine.class);HashMap<String,Integer> textures=new HashMap<>();textures.put("/misc/footprint.png",3);textures.put("/misc/explosion.png",4);Oracle.setField(engine,"textureMap",textures);Oracle.setField(engine,"texturePack",Oracle.allocate(TexturePackList.class));
        Field convert=Tessellator.class.getDeclaredField("convertQuadsToTriangles");convert.setAccessible(true);convert.setBoolean(null,false);
        Field raw=Tessellator.class.getDeclaredField("rawBuffer");raw.setAccessible(true);
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(path)))) {
            o.writeBytes("B181PRT2");EntityOracle.i(o,41);
            for(int x=-20;x<=20;x++)for(int z=-20;z<=20;z++) {
                EntityOracle.i(o,x);EntityOracle.i(o,z);EntityOracle.i(o,w.func_35461_e(x,z));
                for(int y=0;y<128;y++)o.writeByte(w.getBlockId(x,y,z));for(int y=0;y<128;y++)o.writeByte(w.getBlockMetadata(x,y,z));for(int y=0;y<=128;y++)EntityOracle.f(o,w.getLightBrightness(x,y,z));for(int y=0;y<=128;y++)EntityOracle.i(o,w.func_35451_b(x,y,z,0));
            }
            EntityOracle.i(o,24*8);
            for(int kind=0;kind<24;kind++)for(int variant=0;variant<8;variant++) {
                long seed=100+variant;long mathseed=200+variant;OracleHooks.reset(seed);EntityOracle.mathRandom().setSeed(mathseed);
                double x=-3.5+variant*0.75,y=variant==7?66.1:64.1+variant*0.03125,z=variant%2==0?-1.5:1.5;
                double vx=variant==0?0.0:(variant-4)*0.125,vy=variant%3==0?0.0:-0.1,vz=0.2;
                int block=new int[]{1,2,18,35,12,79,53,88}[variant],meta=variant&3;
                EntityOracle.i(o,kind);EntityOracle.l(o,seed);EntityOracle.l(o,mathseed);for(double v:new double[]{x,y,z,vx,vy,vz})EntityOracle.d(o,v);EntityOracle.i(o,block);EntityOracle.i(o,meta);EntityOracle.i(o,202);
                EntityFX p=create(kind,w,engine,x,y,z,vx,vy,vz,block,meta);
                if(kind==16&&variant==7){p.func_407_b(0.2F);p.func_405_d(0.6F);}
                for(int tick=0;tick<202;tick++) {
                    state(p,o);
                    for(float partial:new float[]{0.0F,0.25F,0.75F,1.0F}) {
                        EntityFX.interpPosX=0.125;EntityFX.interpPosY=65.0;EntityFX.interpPosZ=-0.25;
                        EntityOracle.i(o,p.func_35115_a(partial));Tessellator t=Tessellator.instance;t.startDrawingQuads();
                        float yaw=37.25F,pitch=-21.5F;float cy=MathHelper.cos(yaw*(float)Math.PI/180.0F),sy=MathHelper.sin(yaw*(float)Math.PI/180.0F),sp=MathHelper.sin(pitch*(float)Math.PI/180.0F),cp=MathHelper.cos(pitch*(float)Math.PI/180.0F);
                        if(kind==17||kind==23){t.draw();p.renderParticle(t,partial,cy,cp,sy,-sy*sp,cy*sp);}else p.renderParticle(t,partial,cy,cp,sy,-sy*sp,cy*sp);
                        int count=kind==23&&((int)((EntityOracle.intField(EntityLargeExplodeFX.class,"field_35130_a",p)+partial)*15.0F/EntityOracle.intField(EntityLargeExplodeFX.class,"field_35129_ay",p)))>15?0:4;EntityOracle.i(o,count);int[] data=(int[])raw.get(t);for(int i=0;i<count;i++){for(int j=0;j<5;j++)EntityOracle.i(o,data[i*8+j]);EntityOracle.i(o,data[i*8+5]);}
                        if(kind!=17&&kind!=23)t.draw();
                    }
                    if(!p.isDead)p.onUpdate();
                }
            }
            EntityOracle.i(o,32);
            RenderManager manager=RendererOracle.manager(w);PickupRender renderer=new PickupRender();renderer.setRenderManager(manager);Map renders=new HashMap();renders.put(EntityPig.class,renderer);Oracle.setField(manager,"entityRenderMap",renders);RenderManager.instance=manager;
            for(int variant=0;variant<32;variant++) {
             EntityPig source=new EntityPig(w),target=new EntityPig(w);source.setPosition(-3.5+variant*.125,64.1,1.5);source.motionX=.25;source.motionY=-.1;source.motionZ=.5;target.lastTickPosX=-1;target.lastTickPosY=65;target.lastTickPosZ=2;target.setPosition(3,66,-2);
             long seed=500+variant,mathseed=600+variant;OracleHooks.reset(seed);EntityOracle.mathRandom().setSeed(mathseed);
             EntityOracle.l(o,seed);EntityOracle.l(o,mathseed);for(double v:new double[]{source.posX,source.posY,source.posZ,source.motionX,source.motionY,source.motionZ,target.lastTickPosX,target.lastTickPosY,target.lastTickPosZ,target.posX,target.posY,target.posZ})EntityOracle.d(o,v);
             EntityPickupFX p=new EntityPickupFX(w,source,target,-.5F);
             for(int tick=0;tick<4;tick++) {state(p,o);for(float partial:new float[]{0,.25F,.75F,1}) {
              EntityFX.interpPosX=.125;EntityFX.interpPosY=65;EntityFX.interpPosZ=-.25;p.renderParticle(Tessellator.instance,partial,0,0,0,0,0);for(double v:renderer.position)EntityOracle.d(o,v);EntityOracle.f(o,org.lwjgl.opengl.GL11.color[0]);EntityOracle.i(o,p.func_35115_a(partial));
             }if(!p.isDead)p.onUpdate();}
            }
        }
    }
 public static void main(String[] a)throws Exception{run(a[1]);}
}