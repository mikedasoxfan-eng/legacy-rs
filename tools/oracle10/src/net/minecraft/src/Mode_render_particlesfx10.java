package net.minecraft.src;
import java.io.*;
import java.util.*;
import java.lang.reflect.*;
/** Runs the original EntityFX classes and reads original Tessellator buffers. */
public final class Mode_render_particlesfx10 {
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
        case 11:return new EntityHeartFX(w,x,y,z,vx,vy,vz);case 12:return new EntityLavaFX(w,x,y,z);case 13:return new EntityBreakingFX(w,x,y,z,Item.snowball);case 14:return new EntityBreakingFX(w,x,y,z,Item.slimeBall);
        case 15:return new EntitySnowShovelFX(w,x,y,z,vx,vy,vz);case 16:return new EntityDiggingFX(w,x,y,z,vx,vy,vz,Block.blocksList[block],2,meta);case 17:return new EntityFootStepFX(engine,w,x,y,z);case 18:return new EntityCritFX(w,x,y,z,vx,vy,vz);case 19:{EntityCritFX p=new EntityCritFX(w,x,y,z,vx,vy,vz);p.particleRed*=0.3F;p.particleGreen*=0.8F;Harness10.setInt(p,"particleTextureIndex",Harness10.getInt(p,"particleTextureIndex")+1);return p;}case 20:return new EntityCloudFX(w,x,y,z,vx,vy,vz);case 21:return new EntityAuraFX(w,x,y,z,vx,vy,vz);case 22:return new EntitySuspendFX(w,x,y,z,vx,vy,vz);case 24:return new EntityDropParticleFX(w,x,y,z,Material.water);case 25:return new EntityDropParticleFX(w,x,y,z,Material.lava);
        case 26:return new EntityEnchantmentTableParticleFX(w,x,y,z,vx,vy,vz);
        case 27:case 28:case 29:{EntitySpellParticleFX p=new EntitySpellParticleFX(w,x,y,z,kind==29?0:vx,kind==29?0:vy,kind==29?0:vz);if(kind==28)p.func_40110_b(144);if(kind==29)p.func_40097_b((float)vx,(float)vy,(float)vz);return p;}
        case 30:return new EntityBreakingFX(w,x,y,z,Item.itemsList[block]);
        case 31:return new EntityBreakingFX(w,x,y,z,vx,vy,vz,Item.itemsList[block]);
        case 23:return new EntityLargeExplodeFX(engine,w,x,y,z,vx,vy,vz);default:throw new AssertionError(kind);
        }
    }
    static void state(EntityFX p,DataOutputStream o)throws Exception {
        for(double v:new double[]{p.posX,p.posY,p.posZ,p.prevPosX,p.prevPosY,p.prevPosZ,p.motionX,p.motionY,p.motionZ,p.boundingBox.minX,p.boundingBox.minY,p.boundingBox.minZ,p.boundingBox.maxX,p.boundingBox.maxY,p.boundingBox.maxZ})EntityOracle.d(o,v);
        for(float v:new float[]{p.particleRed,p.particleGreen,p.particleBlue,p.particleScale,p.particleGravity,p.particleTextureJitterX,p.particleTextureJitterY,p.width,p.height,p.yOffset})EntityOracle.f(o,v);
        int timer=p instanceof EntityDropParticleFX?EntityOracle.intField(EntityDropParticleFX.class,"field_40104_aw",p):p instanceof EntitySpellParticleFX?EntityOracle.intField(EntitySpellParticleFX.class,"field_40111_a",p):p instanceof EntityFootStepFX?EntityOracle.intField(EntityFootStepFX.class,"field_27018_a",p):p instanceof EntityLargeExplodeFX?EntityOracle.intField(EntityLargeExplodeFX.class,"field_35130_a",p):0;
        for(int v:new int[]{p.func_40100_q(),p.particleAge,p.particleMaxAge,p.getFXLayer(),timer})EntityOracle.i(o,v);
        o.writeBoolean(p.isDead);o.writeBoolean(p.onGround);o.writeBoolean(p.noClip);o.writeBoolean(p.isInWeb);
        EntityOracle.l(o,EntityOracle.rngState(p.rand));EntityOracle.l(o,EntityOracle.rngState(EntityOracle.mathRandom()));
    }
    public static void run(int selected,String path)throws Exception {
        World w=Oracle.newWorld(0);w.chunkProvider=new EntityOracle.FlatProvider(w);w.rand.setSeed(300);w.skylightSubtracted=0;
        for(Chunk c:((EntityOracle.FlatProvider)w.chunkProvider).chunks.values()) {Arrays.fill(c.skylightMap.data,(byte)255);Arrays.fill(c.heightMap,(byte)64);}
        for(int x=-4;x<=4;x++)for(int z=-4;z<=4;z++) {
            if(x==2)for(int y=64;y<68;y++)EntityOracle.rawBlock(w,x,y,z,1,0);
            if(x==-2)EntityOracle.rawBlock(w,x,64,z,53,z&3);
            if(x==0&&z<0)EntityOracle.rawBlock(w,x,64,z,9,3);
            if(x==-1&&z<0)EntityOracle.rawBlock(w,x,64,z,30,0);
            if(x==1&&z<0)EntityOracle.rawBlock(w,x,63,z,88,0);
            // 1.0 collision shapes: fences, gates, panes, new stairs, brewing stand,
            // cauldron, end portal frame, lily pad, enchantment table, dragon egg.
            // The arrangement rotates with the selected kind, under the odd variants' paths (z=1.5, drifting +z).
            if(z>=1&&x!=-2&&x!=2){int[] ids={85,113,107,101,102,108,109,114,117,118,120,111,116,122,107,85};int i=((x+4)*5+z+selected*3)%ids.length;
                EntityOracle.rawBlock(w,x,(z&1)==1?63:64,z,ids[i],ids[i]==107?(x+z+selected)&7:ids[i]==120?((z+selected)&4)|(x&3):(x+z+selected)&3);}
        }
        for(int x=-20;x<=20;x++)for(int z=-20;z<=20;z++)for(int y=0;y<128;y++) {
            Chunk c=w.getChunkFromChunkCoords(x>>4,z>>4);c.skylightMap.setNibble(x&15,y,z&15,(x+y+z)&15);c.blocklightMap.setNibble(x&15,y,z&15,(x*3+y*7+z*5)&15);
        }
        Mode_entities10.Player player=new Mode_entities10.Player(w);player.setPosition(.5,64.1,1.5);w.playerEntities.add(player);
        RenderEngine engine=(RenderEngine)Oracle.allocate(RenderEngine.class);HashMap<String,Integer> textures=new HashMap<>();textures.put("/misc/footprint.png",3);textures.put("/misc/explosion.png",4);Oracle.setField(engine,"textureMap",textures);Oracle.setField(engine,"texturePack",Oracle.allocate(TexturePackList.class));
        Field convert=Tessellator.class.getDeclaredField("convertQuadsToTriangles");convert.setAccessible(true);convert.setBoolean(null,false);
        Field raw=Tessellator.class.getDeclaredField("rawBuffer");raw.setAccessible(true);Field lit=Tessellator.class.getDeclaredField("hasBrightness");lit.setAccessible(true);
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(path)))) {
            o.writeBytes("R10PRT03");EntityOracle.i(o,41);
            for(int x=-20;x<=20;x++)for(int z=-20;z<=20;z++) {
                EntityOracle.i(o,x);EntityOracle.i(o,z);EntityOracle.i(o,Trace10.namedInt("w.func_35461_e(x,z)", w.func_35461_e(x,z)));
                for(int y=0;y<128;y++)Trace10.writeByte(o, "w.getBlockId(x,y,z)", w.getBlockId(x,y,z));for(int y=0;y<128;y++)Trace10.writeByte(o, "w.getBlockMetadata(x,y,z)", w.getBlockMetadata(x,y,z));for(int y=0;y<=128;y++)EntityOracle.f(o,Trace10.namedFloat("w.getLightBrightness(x,y,z)", w.getLightBrightness(x,y,z)));for(int y=0;y<=128;y++)EntityOracle.i(o,Trace10.namedInt("w.getLightBrightnessForSkyBlocks(x,y,z,0)", w.getLightBrightnessForSkyBlocks(x,y,z,0)));
            }
            EntityOracle.i(o,Trace10.namedInt("32", 32));
            for(int kind=selected;kind<=selected;kind++)for(int variant=0;variant<32;variant++) {
                long seed=100+variant;long mathseed=200+variant;OracleHooks.reset(seed);EntityOracle.mathRandom().setSeed(mathseed);
                double x=-3.5+(variant%8)*0.75,y=variant%8==7?66.1:64.1+(variant%8)*0.03125,z=variant%2==0?-1.5:1.5;
                double vx=variant==0?0.0:(variant%8-4)*0.125,vy=variant%3==0?0.0:-0.1,vz=variant%4==0?0:0.2;
                int block=new int[]{1,2,18,35,12,79,53,88}[variant%8],meta=variant&15;if(kind>=30)block=new int[]{332,341,373,381,264,368,280,265}[variant%8];
                EntityOracle.i(o,kind);EntityOracle.l(o,seed);EntityOracle.l(o,mathseed);for(double v:new double[]{Trace10.namedDouble("x", x),Trace10.namedDouble("y", y),Trace10.namedDouble("z", z),Trace10.namedDouble("vx", vx),Trace10.namedDouble("vy", vy),Trace10.namedDouble("vz", vz)})EntityOracle.d(o,v);EntityOracle.i(o,block);EntityOracle.i(o,meta);EntityOracle.i(o,340);
                EntityFX p=create(kind,w,engine,x,y,z,vx,vy,vz,block,meta);
                if(kind==16&&variant==7){p.multiplyVelocity(0.2F);p.func_405_d(0.6F);}
                for(int tick=0;tick<340;tick++) {
                    state(p,o);
                    for(float partial:new float[]{0.0F,0.25F,0.75F,1.0F}) {
                        EntityFX.interpPosX=0.125;EntityFX.interpPosY=65.0;EntityFX.interpPosZ=-0.25;
                        EntityOracle.i(o,Trace10.namedInt("p.getEntityBrightnessForRender(partial)", p.getEntityBrightnessForRender(partial)));EntityOracle.f(o,p.getEntityBrightness(partial));Tessellator t=Tessellator.instance;t.startDrawingQuads();t.setBrightness(p.getEntityBrightnessForRender(partial));
                        float yaw=37.25F,pitch=-21.5F;float cy=MathHelper.cos(yaw*(float)Math.PI/180.0F),sy=MathHelper.sin(yaw*(float)Math.PI/180.0F),sp=MathHelper.sin(pitch*(float)Math.PI/180.0F),cp=MathHelper.cos(pitch*(float)Math.PI/180.0F);
                        if(kind==17||kind==23){t.draw();p.renderParticle(t,partial,cy,cp,sy,-sy*sp,cy*sp);}else p.renderParticle(t,partial,cy,cp,sy,-sy*sp,cy*sp);
                        int count=kind==23&&((int)((EntityOracle.intField(EntityLargeExplodeFX.class,"field_35130_a",p)+partial)*15.0F/EntityOracle.intField(EntityLargeExplodeFX.class,"field_35129_ay",p)))>15?0:4;EntityOracle.i(o,count);int[] data=(int[])raw.get(t);for(int i=0;i<count;i++){for(int j=0;j<5;j++)EntityOracle.i(o,Trace10.namedInt("data[i*8+j]", data[i*8+j]));EntityOracle.i(o,Trace10.namedInt("data[i*8+5]", data[i*8+5]));EntityOracle.i(o,lit.getBoolean(t)?data[i*8+7]:-1);}
                        if(kind!=17&&kind!=23)t.draw();
                    }
                    if(!p.isDead)p.onUpdate();
                }
            }
            EntityOracle.i(o,0);
        }
    }
 public static void main(String[] a)throws Exception{run(Integer.parseInt(a[0]),a[1]);}
}