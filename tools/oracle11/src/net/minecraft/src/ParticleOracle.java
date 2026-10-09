package net.minecraft.src;
import java.io.*;
import java.util.*;
import java.lang.reflect.*;
/** Runs the original EntityFX classes and reads original Tessellator buffers. */
public final class ParticleOracle {
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
        case 15:return new EntitySnowShovelFX(w,x,y,z,vx,vy,vz);case 16:return new EntityDiggingFX(w,x,y,z,vx,vy,vz,Block.blocksList[block],2,meta);case 17:return new EntityFootStepFX(engine,w,x,y,z);default:throw new AssertionError(kind);
        }
    }
    static void state(EntityFX p,DataOutputStream o)throws Exception {
        for(double n:new double[]{Trace10.namedDouble("p.posX", p.posX),Trace10.namedDouble("p.posY", p.posY),Trace10.namedDouble("p.posZ", p.posZ),Trace10.namedDouble("p.prevPosX", p.prevPosX),Trace10.namedDouble("p.prevPosY", p.prevPosY),Trace10.namedDouble("p.prevPosZ", p.prevPosZ),Trace10.namedDouble("p.motionX", p.motionX),Trace10.namedDouble("p.motionY", p.motionY),Trace10.namedDouble("p.motionZ", p.motionZ),Trace10.namedDouble("p.boundingBox.minX", p.boundingBox.minX),Trace10.namedDouble("p.boundingBox.minY", p.boundingBox.minY),Trace10.namedDouble("p.boundingBox.minZ", p.boundingBox.minZ),Trace10.namedDouble("p.boundingBox.maxX", p.boundingBox.maxX),Trace10.namedDouble("p.boundingBox.maxY", p.boundingBox.maxY),Trace10.namedDouble("p.boundingBox.maxZ", p.boundingBox.maxZ)})EntityOracle.d(o,n);
        for(float n:new float[]{Trace10.namedFloat("p.particleRed", p.particleRed),Trace10.namedFloat("p.particleGreen", p.particleGreen),Trace10.namedFloat("p.particleBlue", p.particleBlue),Trace10.namedFloat("p.particleScale", p.particleScale),Trace10.namedFloat("p.particleGravity", p.particleGravity),Trace10.namedFloat("p.particleTextureJitterX", p.particleTextureJitterX),Trace10.namedFloat("p.particleTextureJitterY", p.particleTextureJitterY),Trace10.namedFloat("p.width", p.width),Trace10.namedFloat("p.height", p.height),Trace10.namedFloat("p.yOffset", p.yOffset)})EntityOracle.f(o,n);
        for(int n:new int[]{Trace10.namedInt("Harness10.getInt(p,\"particleTextureIndex\")", Harness10.getInt(p,"particleTextureIndex")),Trace10.namedInt("p.particleAge", p.particleAge),Trace10.namedInt("p.particleMaxAge", p.particleMaxAge),Trace10.namedInt("p.getFXLayer()", p.getFXLayer()),Trace10.namedInt("p instanceof EntityFootStepFX?EntityOracle.intField(EntityFootStepFX.class,\"field_27018_a\",p):p instanceof EntityPickupFX?EntityOracle.intField(EntityPickupFX.class,\"age\",p):0", p instanceof EntityFootStepFX?EntityOracle.intField(EntityFootStepFX.class,"field_27018_a",p):p instanceof EntityPickupFX?EntityOracle.intField(EntityPickupFX.class,"age",p):0)})EntityOracle.i(o,n);
        Trace10.writeBoolean(o, "p.isDead", p.isDead);Trace10.writeBoolean(o, "p.onGround", p.onGround);Trace10.writeBoolean(o, "p.noClip", p.noClip);Trace10.writeBoolean(o, "p.isInWeb", p.isInWeb);EntityOracle.l(o,EntityOracle.rngState(p.rand));EntityOracle.l(o,EntityOracle.rngState(EntityOracle.mathRandom()));
    }
    public static void pickupDraws(String path)throws Exception {pickupDraws(path,true);}
    public static void pickupDraws(String path,boolean fancy)throws Exception {
        Block.leaves.setGraphicsLevel(fancy);
        World w=Oracle.newWorld(0);w.chunkProvider=new EntityOracle.FlatProvider(w);w.skylightSubtracted=0;
        RenderManager boot=RendererOracle.manager(w),m=RenderManager.instance;m.worldObj=w;m.options=boot.options;m.options.fancyGraphics=false;m.renderEngine=boot.renderEngine;m.playerViewY=37.25F;m.playerViewX=-21.5F;
        Field convert=Tessellator.class.getDeclaredField("convertQuadsToTriangles");convert.setAccessible(true);convert.setBoolean(null,false);
        Oracle.DataOut out=new Oracle.DataOut();out.i32(0x33373152);out.i32(fancy?1:2);out.i32(14*4*3*4+12);
        for(int id:new int[]{1,2,18,35,44,53,67,81,85,96,263,331,2256,351})for(int variant=0;variant<4;variant++)for(int age=0;age<3;age++)for(float partial:new float[]{0,.25F,.75F,1}) {
            EntityItem source=new EntityItem(w,.25,64.5,-2.75,new ItemStack(id,new int[]{0,1,6,21}[variant],variant*5));source.age=variant*317;source.field_804_d=variant*.73F;
            Entity target=new EntityItem(w);target.lastTickPosX=-.125;target.lastTickPosY=65.875;target.lastTickPosZ=-1.25;target.posX=.5;target.posY=66.5;target.posZ=.75;
            EntityPickupFX fx=new EntityPickupFX(w,source,target,-.5F);for(int tick=0;tick<age;tick++)fx.onUpdate();EntityFX.interpPosX=.125;EntityFX.interpPosY=65;EntityFX.interpPosZ=-.25;
            float t=(age+partial)/3;t*=t;double[] pos={source.posX+(target.lastTickPosX+(target.posX-target.lastTickPosX)*partial-source.posX)*t-.125,source.posY+(target.lastTickPosY+(target.posY-target.lastTickPosY)*partial-.5-source.posY)*t-65,source.posZ+(target.lastTickPosZ+(target.posZ-target.lastTickPosZ)*partial-source.posZ)*t+.25};
            out.i32(0);out.i32(id);out.i32(source.item.stackSize);out.i32(source.item.getItemDamage());out.i32(source.age);Oracle.modelFloat(out,source.field_804_d);out.i32(0);out.i32(0);Oracle.modelFloat(out,partial);Oracle.modelFloat(out,source.getEntityBrightness(partial));for(float v:new float[]{m.playerViewY,m.playerViewX,13.5F,-26.75F})Oracle.modelFloat(out,v);for(double v:pos)out.i64(Double.doubleToRawLongBits((double)(float)v));for(int i=0;i<7;i++)Oracle.modelFloat(out,1);
            RendererOracle.reset(source.getEntityBrightness(partial));RendererOracle.active=true;fx.renderParticle(Tessellator.instance,partial,0,0,0,0,0);RendererOracle.active=false;out.i32(RendererOracle.count);out.bytes(RendererOracle.draws.buf.toByteArray());
        }
        for(int age=0;age<3;age++)for(float partial:new float[]{0,.25F,.75F,1}) {
            EntityArrow source=new EntityArrow(w);source.setPosition(.25,64.5,-2.75);source.prevRotationYaw=source.rotationYaw=13.5F;source.prevRotationPitch=source.rotationPitch=-26.75F;source.arrowShake=4;
            Entity target=new EntityItem(w);target.lastTickPosX=target.posX=.5;target.lastTickPosY=target.posY=65.62;target.lastTickPosZ=target.posZ=.75;
            EntityPickupFX fx=new EntityPickupFX(w,source,target,-.5F);for(int tick=0;tick<age;tick++)fx.onUpdate();EntityFX.interpPosX=.125;EntityFX.interpPosY=65;EntityFX.interpPosZ=-.25;
            float t=(age+partial)/3;t*=t;double[] world={.25+(.5-.25)*t,64.5+(65.62-.5-64.5)*t,-2.75+(.75+2.75)*t};float light=w.getLightBrightness(MathHelper.floor_double(world[0]),MathHelper.floor_double(world[1]+fx.yOffset/2),MathHelper.floor_double(world[2]));
            out.i32(3);out.i32(262);out.i32(1);out.i32(0);out.i32(0);Oracle.modelFloat(out,0);out.i32(0);out.i32(4);Oracle.modelFloat(out,partial);Oracle.modelFloat(out,light);for(float v:new float[]{m.playerViewY,m.playerViewX,13.5F,-26.75F})Oracle.modelFloat(out,v);double[] camera={.125,65,-.25};for(int i=0;i<3;i++)out.i64(Double.doubleToRawLongBits((double)(float)(world[i]-camera[i])));for(int i=0;i<7;i++)Oracle.modelFloat(out,1);
            RendererOracle.reset(light);RendererOracle.active=true;fx.renderParticle(Tessellator.instance,partial,0,0,0,0,0);RendererOracle.active=false;out.i32(RendererOracle.count);out.bytes(RendererOracle.draws.buf.toByteArray());
        }
        try(OutputStream f=new FileOutputStream(path)){out.writeTo(f);}
    }
    public static void music(String path)throws Exception {
        World w=Oracle.newWorld(0);w.chunkProvider=new EntityOracle.FlatProvider(w);WorldEventOracle.attach(w);OracleHooks.reset(100);EntityOracle.mathRandom().setSeed(200);w.rand.setSeed(300);
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(path)))) {
            for(int below:new int[]{0,1,12,20,5}) {w.removeBlockTileEntity(8,64,8);EntityOracle.rawBlock(w,8,64,8,25,0);EntityOracle.rawBlock(w,8,63,8,below,0);
                for(int action=0;action<30;action++) {EntityOracle.rawBlock(w,8,65,8,action==27?1:0,0);if(action==26)Block.music.onBlockClicked(w,8,64,8,null);else Block.music.blockActivated(w,8,64,8,null);WorldEventOracle.snapshot(w,o);EntityOracle.i(o,((TileEntityNote)w.getBlockTileEntity(8,64,8)).note);}
            }
            w.removeBlockTileEntity(8,64,8);EntityOracle.rawBlock(w,8,64,8,84,0);
            for(int record:new int[]{2256,2257}) {((BlockJukeBox)Block.jukebox).ejectRecord(w,8,64,8,record);w.playAuxSFXAtEntity(null,1005,8,64,8,record);WorldEventOracle.snapshot(w,o);EntityOracle.i(o,w.getBlockMetadata(8,64,8));EntityOracle.i(o,((TileEntityRecordPlayer)w.getBlockTileEntity(8,64,8)).record);
                Block.jukebox.blockActivated(w,8,64,8,null);WorldEventOracle.snapshot(w,o);EntityOracle.i(o,w.getBlockMetadata(8,64,8));EntityOracle.i(o,((TileEntityRecordPlayer)w.getBlockTileEntity(8,64,8)).record);EntityOracle.l(o,EntityOracle.rngState(w.rand));Entity item=(Entity)w.loadedEntityList.get(w.loadedEntityList.size()-1);for(double v:new double[]{Trace10.namedDouble("item.posX", item.posX),Trace10.namedDouble("item.posY", item.posY),Trace10.namedDouble("item.posZ", item.posZ),Trace10.namedDouble("item.motionX", item.motionX),Trace10.namedDouble("item.motionY", item.motionY),Trace10.namedDouble("item.motionZ", item.motionZ)})EntityOracle.d(o,v);EntityOracle.l(o,EntityOracle.rngState(item.rand));EntityItem drop=(EntityItem)item;EntityOracle.f(o,drop.field_804_d);EntityOracle.i(o,drop.delayBeforeCanPickup);EntityOracle.i(o,drop.item.itemID);EntityOracle.i(o,drop.item.stackSize);
            }
            w.removeBlockTileEntity(8,64,8);EntityOracle.rawBlock(w,8,64,8,25,0);EntityOracle.rawBlock(w,8,63,8,1,0);
            for(int action=0;action<16;action++) {
                EntityOracle.rawBlock(w,8,65,8,action>=12?1:0,0);EntityOracle.rawBlock(w,8,64,7,77,action%4<2?13:5);
                Block.music.onNeighborBlockChange(w,8,64,8,action==5?0:77);
                WorldEventOracle.snapshot(w,o);TileEntityNote note=(TileEntityNote)w.getBlockTileEntity(8,64,8);EntityOracle.i(o,note.note);Trace10.writeBoolean(o, "note.previousRedstoneState", note.previousRedstoneState);
            }
            for(int id=1;id<=96;id++){StepSound step=Block.blocksList[id].stepSound;byte[] a=step.stepSoundDir2().getBytes("UTF-8"),b=step.stepSoundDir().getBytes("UTF-8");EntityOracle.i(o,a.length);o.write(a);EntityOracle.i(o,b.length);o.write(b);EntityOracle.f(o,step.getVolume());EntityOracle.f(o,step.getPitch());}
        }
    }
    public static void effects(String path)throws Exception {
        World w=Oracle.newWorld(0);w.chunkProvider=new EntityOracle.FlatProvider(w);w.skylightSubtracted=0;
        ColorizerGrass.setGrassBiomeColorizer(Oracle.jarImage("misc/grasscolor.png"));ColorizerFoliage.getFoilageBiomeColorizer(Oracle.jarImage("misc/foliagecolor.png"));
        RenderEngine engine=(RenderEngine)Oracle.allocate(RenderEngine.class);
        Field layers=EffectRenderer.class.getDeclaredField("fxLayers");layers.setAccessible(true);Field random=EffectRenderer.class.getDeclaredField("rand");random.setAccessible(true);
        Field raw=Tessellator.class.getDeclaredField("rawBuffer");raw.setAccessible(true);Field convert=Tessellator.class.getDeclaredField("convertQuadsToTriangles");convert.setAccessible(true);convert.setBoolean(null,false);
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(path)))) {
            o.writeBytes("B173EFX1");EntityOracle.i(o,16*16);int scene=0;
            for(int id:new int[]{1,2,18,31,35,44,53,67,64,71,78,85,88,92,96,18})for(int meta=0;meta<16;meta++) {
                boolean fancy=scene++<240;Block.leaves.setGraphicsLevel(fancy);
                int x=8,y=64,z=8;EntityOracle.rawBlock(w,x,y,z,id,meta);Block b=Block.blocksList[id];b.setBlockBoundsBasedOnState(w,x,y,z);
                EntityOracle.i(o,id);EntityOracle.i(o,meta);EntityOracle.i(o,fancy?1:0);for(double v:new double[]{Trace10.namedDouble("b.minX", b.minX),Trace10.namedDouble("b.minY", b.minY),Trace10.namedDouble("b.minZ", b.minZ),Trace10.namedDouble("b.maxX", b.maxX),Trace10.namedDouble("b.maxY", b.maxY),Trace10.namedDouble("b.maxZ", b.maxZ)})EntityOracle.d(o,v);EntityOracle.i(o,id==2?0xffffff:b.colorMultiplier(w,x,y,z));
                EntityOracle.i(o,21);for(int xx=-2;xx<=18;xx++)for(int zz=-2;zz<=18;zz++){EntityOracle.i(o,xx);EntityOracle.i(o,zz);EntityOracle.i(o,w.getPrecipitationHeight(xx,zz));for(int yy=0;yy<128;yy++)Trace10.writeByte(o, "w.getBlockId(xx,yy,zz)", w.getBlockId(xx,yy,zz));for(int yy=0;yy<128;yy++)Trace10.writeByte(o, "w.getBlockMetadata(xx,yy,zz)", w.getBlockMetadata(xx,yy,zz));for(int yy=0;yy<=128;yy++)EntityOracle.f(o,w.getLightBrightness(xx,yy,zz));}
                OracleHooks.reset(800);EntityOracle.mathRandom().setSeed(900);EffectRenderer r=new EffectRenderer(w,engine);random.set(r,new Random(700));
                r.addBlockDestroyEffects(x,y,z,id,meta);for(int side=0;side<6;side++)r.addBlockHitEffects(x,y,z,side);
                for(int tick=0;tick<20;tick++) {EntityOracle.l(o,EntityOracle.rngState((Random)random.get(r)));List list=((List[])layers.get(r))[1];EntityOracle.i(o,list.size());for(Object object:list){EntityFX p=(EntityFX)object;state(p,o);Tessellator t=Tessellator.instance;t.startDrawingQuads();EntityFX.interpPosX=8;EntityFX.interpPosY=65;EntityFX.interpPosZ=8;p.renderParticle(t,0.5F,1,1,0,0,0);int[] data=(int[])raw.get(t);for(int i=0;i<4;i++)for(int j=0;j<6;j++)EntityOracle.i(o,data[i*8+j]);t.draw();}r.updateEffects();}
            }
            OracleHooks.reset(1100);EntityOracle.mathRandom().setSeed(1200);
            HashMap<String,Integer> textureMap=new HashMap<>();textureMap.put("/misc/footprint.png",3);Oracle.setField(engine,"textureMap",textureMap);Oracle.setField(engine,"texturePack",Oracle.allocate(TexturePackList.class));
            final EffectRenderer updates=new EffectRenderer(w,engine);
            w.addWorldAccess((IWorldAccess)Proxy.newProxyInstance(ParticleOracle.class.getClassLoader(),new Class[]{IWorldAccess.class},(proxy,method,args)->{if(method.getName().equals("spawnParticle")&&args[0].equals("smoke"))updates.addEffect(new EntitySmokeFX(w,(Double)args[1],(Double)args[2],(Double)args[3],(Double)args[4],(Double)args[5],(Double)args[6]));return null;}));
            for(int kind:new int[]{12,1,16,14,17,6})updates.addEffect(create(kind,w,engine,8.5,66.1,8.5,.1,-.1,.2,1,0));
            for(int tick=0;tick<30;tick++) {for(List list:(List[])layers.get(updates)){EntityOracle.i(o,list.size());for(Object p:list)state((EntityFX)p,o);}updates.updateEffects();}
            // Each layer evicts its oldest element independently at 4000.
            OracleHooks.reset(5000);EntityOracle.mathRandom().setSeed(6000);EffectRenderer cap=new EffectRenderer(w,engine);
            for(int i=0;i<4002;i++)cap.addEffect(new EntityFX(w,i,70,0,0,0,0));List list=((List[])layers.get(cap))[0];EntityOracle.i(o,list.size());EntityOracle.d(o,((EntityFX)list.get(0)).posX);EntityOracle.d(o,((EntityFX)list.get(list.size()-1)).posX);
            // Actual RenderGlobal name selection and its inclusive 16-block gate.
            net.minecraft.client.Minecraft mc=(net.minecraft.client.Minecraft)Oracle.allocate(PlayerOracle.HeadlessMinecraft.class);
            mc.gameSettings=(GameSettings)Oracle.allocate(GameSettings.class);
            mc.renderViewEntity=new EntityCow(w);mc.renderViewEntity.setPosition(8.5,65,8.5);
            RenderGlobal global=(RenderGlobal)Oracle.allocate(RenderGlobal.class);Oracle.setField(global,"mc",mc);Oracle.setField(global,"worldObj",w);Oracle.setField(global,"renderEngine",engine);
            String[] names={"bubble","smoke","note","portal","explode","flame","lava","footstep","splash","largesmoke","reddust","snowballpoof","snowshovel","slime","heart","unknown"};
            double[] distances={0,16,Math.nextAfter(16,Double.POSITIVE_INFINITY),-16,Double.NaN,Double.POSITIVE_INFINITY};
            EntityOracle.i(o,names.length*distances.length);
            for(String name:names)for(double distance:distances) {
                mc.effectRenderer=new EffectRenderer(w,engine);OracleHooks.reset(1000);EntityOracle.mathRandom().setSeed(2000);
                global.spawnParticle(name,8.5+distance,65,8.5,.1,-.1,.2);
                for(List effects:(List[])layers.get(mc.effectRenderer)) {EntityOracle.i(o,effects.size());for(Object fx:effects)state((EntityFX)fx,o);}
                EntityOracle.l(o,EntityOracle.rngState(EntityOracle.mathRandom()));
            }

        }
    }
    public static void display(String path)throws Exception {
        World w=Oracle.newWorld(0);w.chunkProvider=new EntityOracle.FlatProvider(w);WorldEventOracle.attach(w);
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(path)))) {
            for(int id:new int[]{8,9,10,11,50,51,55,62,74,75,76,90,93,94})for(int meta=0;meta<16;meta++)for(int scene=0;scene<4;scene++) {
                int x=scene==3?-16:8,y=scene==1?65:64,z=scene==3?-16:8;
                for(int[] d:new int[][]{{0,-1,0},{0,1,0},{-1,0,0},{1,0,0},{0,0,-1},{0,0,1}})EntityOracle.rawBlock(w,x+d[0],y+d[1],z+d[2],scene==2?17:scene==0&&d[1]<0?1:0,0);
                if(id==90&&scene==2)EntityOracle.rawBlock(w,x-1,y,z,90,0);
                EntityOracle.rawBlock(w,x,y,z,id,meta);Random display=new Random(1000+meta);w.rand.setSeed(2000+id);
                for(int t=0;t<100;t++) {Block.blocksList[id].randomDisplayTick(w,x,y,z,display);WorldEventOracle.snapshot(w,o);EntityOracle.l(o,EntityOracle.rngState(w.rand));EntityOracle.l(o,EntityOracle.rngState(display));}
            }
        }
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
        RenderEngine engine=(RenderEngine)Oracle.allocate(RenderEngine.class);HashMap<String,Integer> textures=new HashMap<>();textures.put("/misc/footprint.png",3);Oracle.setField(engine,"textureMap",textures);Oracle.setField(engine,"texturePack",Oracle.allocate(TexturePackList.class));
        Field convert=Tessellator.class.getDeclaredField("convertQuadsToTriangles");convert.setAccessible(true);convert.setBoolean(null,false);
        Field raw=Tessellator.class.getDeclaredField("rawBuffer");raw.setAccessible(true);
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(path)))) {
            o.writeBytes("B173PRT1");EntityOracle.i(o,41);
            for(int x=-20;x<=20;x++)for(int z=-20;z<=20;z++) {
                EntityOracle.i(o,x);EntityOracle.i(o,z);EntityOracle.i(o,w.getPrecipitationHeight(x,z));
                for(int y=0;y<128;y++)Trace10.writeByte(o, "w.getBlockId(x,y,z)", w.getBlockId(x,y,z));for(int y=0;y<128;y++)Trace10.writeByte(o, "w.getBlockMetadata(x,y,z)", w.getBlockMetadata(x,y,z));for(int y=0;y<=128;y++)EntityOracle.f(o,w.getLightBrightness(x,y,z));
            }
            EntityOracle.i(o,18*8);
            for(int kind=0;kind<18;kind++)for(int variant=0;variant<8;variant++) {
                long seed=100+variant;long mathseed=200+variant;OracleHooks.reset(seed);EntityOracle.mathRandom().setSeed(mathseed);
                double x=-3.5+variant*0.75,y=variant==7?66.1:64.1+variant*0.03125,z=variant%2==0?-1.5:1.5;
                double vx=variant==0?0.0:(variant-4)*0.125,vy=variant%3==0?0.0:-0.1,vz=0.2;
                int block=new int[]{1,2,18,35,12,79,53,88}[variant],meta=variant&3;
                EntityOracle.i(o,kind);EntityOracle.l(o,seed);EntityOracle.l(o,mathseed);for(double v:new double[]{Trace10.namedDouble("x", x),Trace10.namedDouble("y", y),Trace10.namedDouble("z", z),Trace10.namedDouble("vx", vx),Trace10.namedDouble("vy", vy),Trace10.namedDouble("vz", vz)})EntityOracle.d(o,v);EntityOracle.i(o,block);EntityOracle.i(o,meta);EntityOracle.i(o,202);
                EntityFX p=create(kind,w,engine,x,y,z,vx,vy,vz,block,meta);
                if(kind==16&&variant==7){p.multiplyVelocity(0.2F);p.func_405_d(0.6F);}
                for(int tick=0;tick<202;tick++) {
                    state(p,o);
                    for(float partial:new float[]{0.0F,0.25F,0.75F,1.0F}) {
                        EntityFX.interpPosX=0.125;EntityFX.interpPosY=65.0;EntityFX.interpPosZ=-0.25;
                        Tessellator t=Tessellator.instance;t.startDrawingQuads();
                        float yaw=37.25F,pitch=-21.5F;float cy=MathHelper.cos(yaw*(float)Math.PI/180.0F),sy=MathHelper.sin(yaw*(float)Math.PI/180.0F),sp=MathHelper.sin(pitch*(float)Math.PI/180.0F),cp=MathHelper.cos(pitch*(float)Math.PI/180.0F);
                        if(kind==17){t.draw();p.renderParticle(t,partial,cy,cp,sy,-sy*sp,cy*sp);}else p.renderParticle(t,partial,cy,cp,sy,-sy*sp,cy*sp);
                        int[] data=(int[])raw.get(t);for(int i=0;i<4;i++){for(int j=0;j<5;j++)EntityOracle.i(o,data[i*8+j]);EntityOracle.i(o,data[i*8+5]);}
                        if(kind!=17)t.draw();
                    }
                    if(!p.isDead)p.onUpdate();
                }
            }
            EntityOracle.i(o,64);
            PickupRender capture=new PickupRender();
            Field map=RenderManager.class.getDeclaredField("entityRenderMap");map.setAccessible(true);((Map)map.get(RenderManager.instance)).put(EntityItem.class,capture);
            for(int variant=0;variant<64;variant++) {
                double[] source={variant*0.03125-1.5,64.1+variant*0.125,variant*0.25-8.0};
                double[] motion={0.2,-0.1,0.3},last={0.25,65.62,-0.5},now={0.5+variant*0.1,65.62+variant*0.025,0.75-variant*0.01};
                EntityItem item=new EntityItem(w,source[0],source[1],source[2],new ItemStack(263,0,0));item.motionX=motion[0];item.motionY=motion[1];item.motionZ=motion[2];
                Entity target=new EntityItem(w);target.lastTickPosX=last[0];target.lastTickPosY=last[1];target.lastTickPosZ=last[2];target.posX=now[0];target.posY=now[1];target.posZ=now[2];
                long seed=500+variant,mathseed=600+variant;OracleHooks.reset(seed);EntityOracle.mathRandom().setSeed(mathseed);
                EntityOracle.l(o,seed);EntityOracle.l(o,mathseed);for(double[] row:new double[][]{source,motion,last,now})for(double v:row)EntityOracle.d(o,v);
                EntityPickupFX p=new EntityPickupFX(w,item,target,-0.5F);
                for(int tick=0;tick<4;tick++) {
                    state(p,o);
                    for(float partial:new float[]{0.0F,0.25F,0.75F,1.0F}) {
                        EntityFX.interpPosX=0.125;EntityFX.interpPosY=65.0;EntityFX.interpPosZ=-0.25;
                        p.renderParticle(Tessellator.instance,partial,0,0,0,0,0);
                        for(double v:capture.position)EntityOracle.d(o,v);EntityOracle.f(o,org.lwjgl.opengl.GL11.color[0]);
                    }
                    if(!p.isDead)p.onUpdate();
                }
            }
        }
    }
}
