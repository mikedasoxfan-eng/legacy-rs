package net.minecraft.src;
import java.io.*;
import java.util.*;
import java.lang.reflect.*;
import java.nio.*;
import net.minecraft.client.Minecraft;
import org.lwjgl.opengl.GL11;

/** Original b1.7.3 camera, ItemRenderer, RenderPlayer and targeting bytecode. */
public final class FirstPersonOracle {
    public static boolean recording=false;
    static class FontEngine extends RenderEngine {
        FontEngine(){super(null,null);}
        public int allocateAndSetupTexture(java.awt.image.BufferedImage image){return 14;}
    }
    static void mapSetup(PlayerOracle.HeadlessPlayer p,int damage,int c)throws Exception {
        MapData data=new MapData("map_"+damage);
        for(int i=0;i<16384;i++)data.colors[i]=(byte)((i+c)%56);
        for(int i=0;i<c;i++)data.playersVisibleOnMap.add(new MapCoord(data,(byte)(i%16),(byte)(i*43-127),(byte)(127-i*37),(byte)(i*5)));
        p.worldObj.setItemData("map_"+damage,data);
        MapItemRenderer renderer=(MapItemRenderer)Oracle.allocate(MapItemRenderer.class);
        Oracle.setField(renderer,"intArray",new int[16384]);Oracle.setField(renderer,"bufferedImage",12);Oracle.setField(renderer,"gameSettings",p.mc.gameSettings);
        Oracle.setField(renderer,"fontRenderer",new FontRenderer(p.mc.gameSettings,"/font/default.png",new FontEngine()));
        Oracle.setField(p.mc.renderEngine,"imageData",ByteBuffer.allocate(65536));Oracle.setField(p.mc.renderEngine,"options",p.mc.gameSettings);
        HashMap<String,Integer> textures=(HashMap<String,Integer>)Oracle.getField(p.mc.renderEngine,"textureMap");textures.put("/misc/mapicons.png",13);
        Oracle.setField(p.mc.entityRenderer.itemRenderer,"mapItemRenderer",renderer);
    }
    static ArrayList<Integer> modes=new ArrayList<Integer>();
    static ArrayList<float[]> matrices=new ArrayList<float[]>();
    static ArrayList<float[]> projectionOps=new ArrayList<float[]>();
    static float[] projection=new float[16];
    public static void recordMode(int mode){if(recording)modes.add(mode);}
    // Portable float matrix stack for the recorded GL inputs. Driver-specific
    // rasterization is deliberately outside this reference arithmetic boundary.
    static float[] identity(){float[] m=new float[16];m[0]=m[5]=m[10]=m[15]=1;return m;}
    static float[] multiply(float[] a,float[] b){float[] m=new float[16];for(int c=0;c<4;c++)for(int r=0;r<4;r++)m[c*4+r]=((a[r]*b[c*4]+a[4+r]*b[c*4+1])+a[8+r]*b[c*4+2])+a[12+r]*b[c*4+3];return m;}
    static float[] matrix(ArrayList<float[]> ops){float[] m=identity();for(float[] o:ops){float[] q=identity();if(o[0]==0){q[12]=o[1];q[13]=o[2];q[14]=o[3];}else if(o[0]==1){q[0]=o[1];q[5]=o[2];q[10]=o[3];}else{double angle=(double)o[1]*Math.PI/180.0,s=Math.sin(angle),c=Math.cos(angle),t=1.0-c,len=Math.sqrt((double)o[2]*o[2]+(double)o[3]*o[3]+(double)o[4]*o[4]),x=o[2]/len,y=o[3]/len,z=o[4]/len;q=new float[]{(float)(x*x*t+c),(float)(y*x*t+z*s),(float)(z*x*t-y*s),0,(float)(x*y*t-z*s),(float)(y*y*t+c),(float)(z*y*t+x*s),0,(float)(x*z*t+y*s),(float)(y*z*t-x*s),(float)(z*z*t+c),0,0,0,0,1};}m=multiply(m,q);}return m;}
    public static void recordMatrix(){if(recording)matrices.add(matrix(RendererOracle.ops));}
    public static void projectionOp(int c,float a,float b,float d,float e){projectionOps.add(new float[]{c,a,b,d,e});}
    public static void projectionReset(){projectionOps.clear();Arrays.fill(projection,0);}
    public static void perspective(FloatBuffer b){for(int i=0;i<16;i++)projection[i]=b.get(b.position()+i);}
    static void f(Oracle.DataOut o,float v){Oracle.modelFloat(o,v);}
    static void d(Oracle.DataOut o,double v){o.i64(Double.doubleToRawLongBits(v));}
    static Object call(Object target,String name,Class[] types,Object... args)throws Exception {
        Method m=target.getClass().getDeclaredMethod(name,types);m.setAccessible(true);return m.invoke(target,args);
    }
    static void state(Oracle.DataOut o,EntityPlayer p) {
        for(int v:new int[]{p.health,p.hurtTime,p.maxHurtTime,p.deathTime,p.ticksExisted,Harness10.getInt(p,"fire"),p.isPlayerSleeping()?1:0,p.isSneaking()?1:0})o.i32(v);
        for(float v:new float[]{Trace10.namedFloat("p.rotationYaw", p.rotationYaw),Trace10.namedFloat("p.prevRotationYaw", p.prevRotationYaw),Trace10.namedFloat("p.rotationPitch", p.rotationPitch),Trace10.namedFloat("p.prevRotationPitch", p.prevRotationPitch),Trace10.namedFloat("p.yOffset", p.yOffset),Trace10.namedFloat("p.width", p.width),Trace10.namedFloat("p.height", p.height),Trace10.namedFloat("p.distanceWalkedModified", p.distanceWalkedModified),Trace10.namedFloat("p.prevDistanceWalkedModified", p.prevDistanceWalkedModified),Trace10.namedFloat("p.swingProgress", p.swingProgress),Trace10.namedFloat("p.prevSwingProgress", p.prevSwingProgress),Trace10.namedFloat("p.renderYawOffset", p.renderYawOffset),Trace10.namedFloat("p.prevRenderYawOffset", p.prevRenderYawOffset),Trace10.namedFloat("p.field_704_R", p.field_704_R),Trace10.namedFloat("p.field_705_Q", p.field_705_Q),Trace10.namedFloat("p.field_703_S", p.field_703_S),Trace10.namedFloat("p.cameraYaw", p.cameraYaw),Trace10.namedFloat("p.prevCameraYaw", p.prevCameraYaw),Trace10.namedFloat("p.cameraPitch", p.cameraPitch),Trace10.namedFloat("p.prevCameraPitch", p.prevCameraPitch),Trace10.namedFloat("p.timeInPortal", p.timeInPortal),Trace10.namedFloat("p.prevTimeInPortal", p.prevTimeInPortal),Trace10.namedFloat("p.field_22063_x", p.field_22063_x),Trace10.namedFloat("p.field_22062_y", p.field_22062_y),Trace10.namedFloat("p.field_22061_z", p.field_22061_z),Trace10.namedFloat("p.attackedAtYaw", p.attackedAtYaw)})f(o,v);
        for(double v:new double[]{Trace10.namedDouble("p.posX", p.posX),Trace10.namedDouble("p.posY", p.posY),Trace10.namedDouble("p.posZ", p.posZ),Trace10.namedDouble("p.prevPosX", p.prevPosX),Trace10.namedDouble("p.prevPosY", p.prevPosY),Trace10.namedDouble("p.prevPosZ", p.prevPosZ),Trace10.namedDouble("p.lastTickPosX", p.lastTickPosX),Trace10.namedDouble("p.lastTickPosY", p.lastTickPosY),Trace10.namedDouble("p.lastTickPosZ", p.lastTickPosZ)})d(o,v);
    }
    static void ops(Oracle.DataOut o,ArrayList<float[]> list){o.i32(list.size());for(float[] v:list){o.i32((int)v[0]);for(int i=1;i<5;i++)f(o,v[i]);}}
    static void draws(Oracle.DataOut o){o.i32(RendererOracle.count);o.bytes(RendererOracle.draws.buf.toByteArray());o.i32(modes.size());for(int m:modes)o.i32(m);for(float[] matrix:matrices)for(float v:matrix)f(o,v);}
    static void start(float brightness){RendererOracle.reset(brightness);modes.clear();matrices.clear();recording=true;RendererOracle.active=true;}
    static void stop(){RendererOracle.active=false;recording=false;}
    static World world(){World w=Oracle.newWorld(0);w.chunkProvider=new EntityOracle.FlatProvider(w);w.setWorldTime(6000);w.skylightSubtracted=0;return w;}
    static PlayerOracle.HeadlessPlayer player(World w,int c)throws Exception {
        PlayerOracle.HeadlessPlayer p=PlayerOracle.player(w);
        p.setPosition(.25,65.5,-2.75);p.prevPosX=.1;p.prevPosY=65.25;p.prevPosZ=-2.9;
        p.lastTickPosX=p.prevPosX;p.lastTickPosY=p.prevPosY;p.lastTickPosZ=p.prevPosZ;
        p.rotationYaw=c*17.3F-361;p.prevRotationYaw=p.rotationYaw-21.5F;
        p.rotationPitch=c%7*23.75F-83;p.prevRotationPitch=p.rotationPitch+13.25F;
        p.distanceWalkedModified=c*.137F;p.prevDistanceWalkedModified=p.distanceWalkedModified-.17F;
        p.cameraYaw=c%4*.027F;p.prevCameraYaw=.011F;p.prevCameraPitch=-.75F;p.cameraPitch=c%3*.35F;
        p.swingProgress=c%8*.13F;p.prevSwingProgress=c%5*.17F;
        p.renderYawOffset=p.rotationYaw-27;p.prevRenderYawOffset=p.prevRotationYaw-31;
        p.field_704_R=c%6*.21F;p.field_705_Q=.23F;p.field_703_S=c*.19F;
        p.ticksExisted=c*19;p.maxHurtTime=10;p.hurtTime=c%5==0?c%11:0;p.attackedAtYaw=c*13.75F;
        return p;
    }
    static RenderManager setup(PlayerOracle.HeadlessPlayer p)throws Exception {
        OpenGlHelper.initializeTextures();org.lwjgl.opengl.GL13.activeTexture=0;
        // The client loads the biome colour maps at startup; 1.0 grass item tops read them.
        ColorizerGrass.setGrassBiomeColorizer(Oracle.jarImage("misc/grasscolor.png"));ColorizerFoliage.getFoilageBiomeColorizer(Oracle.jarImage("misc/foliagecolor.png"));
        RenderManager m=RendererOracle.manager(p.worldObj);m.livingPlayer=p;RenderManager.instance=m;
        HashMap<Class,Render> renderers=new HashMap<Class,Render>();RenderPlayer rp=new RenderPlayer();rp.setRenderManager(m);renderers.put(EntityPlayer.class,rp);Oracle.setField(m,"entityRenderMap",renderers);
        Map textures=(Map)Oracle.getField(m.renderEngine,"textureMap");textures.put("/mob/char.png",9);textures.put("/misc/water.png",10);textures.put("/misc/mapbg.png",11);textures.put("/item/chest.png",31);TileEntityRenderer.instance.renderEngine=m.renderEngine;
        String[] names={"cloth","chain","iron","diamond","gold"};for(int a=0;a<5;a++)for(int b=1;b<=2;b++)textures.put("/armor/"+names[a]+"_"+b+".png",20+a*2+b-1);
        Minecraft mc=p.mc;mc.renderViewEntity=p;mc.thePlayer=p;if(mc.playerController==null)mc.playerController=(PlayerController)Oracle.allocate(PlayerControllerSP.class);mc.renderEngine=m.renderEngine;mc.gameSettings=m.options;mc.displayWidth=853;mc.displayHeight=480;
        RenderGlobal global=(RenderGlobal)Oracle.allocate(RenderGlobal.class);Oracle.setField(global,"worldObj",p.worldObj);Oracle.setField(global,"renderEngine",m.renderEngine);Oracle.setField(global,"mc",mc);Oracle.setField(global,"globalRenderBlocks",new RenderBlocks(p.worldObj));mc.renderGlobal=global;
        ItemRenderer ir=(ItemRenderer)Oracle.allocate(ItemRenderer.class);Oracle.setField(ir,"mc",mc);Oracle.setField(ir,"renderBlocksInstance",new RenderBlocks());Oracle.setField(ir,"field_20099_f",-1);m.itemRenderer=ir;
        EntityRenderer er=(EntityRenderer)Oracle.allocate(EntityRenderer.class);Oracle.setField(er,"mc",mc);er.itemRenderer=ir;Oracle.setField(er,"cameraZoom",1.0);Oracle.setField(er,"thirdPersonDistanceTemp",4.0F);Oracle.setField(er,"thirdPersonDistance",4.0F);Oracle.setField(er,"fogColorBuffer",FloatBuffer.allocate(16));mc.entityRenderer=er;return m;
    }
    static void block(Oracle.DataOut o,World w,int x,int y,int z,int id,int meta){o.i32(x);o.i32(y);o.i32(z);o.i32(id);o.i32(meta);EntityOracle.rawBlock(w,x,y,z,id,meta);}
    static void settings(Oracle.DataOut o,PlayerOracle.HeadlessPlayer p,int c)throws Exception {
        GameSettings s=p.mc.gameSettings;s.viewBobbing=(c&1)!=0;s.thirdPersonView=((c&2)!=0?1:0);s.anaglyph=(c&4)!=0;s.debugCamEnable=(c&8)!=0;s.renderDistance=c%4;s.hideGUI=(c&16)!=0;
        o.i32((s.viewBobbing?1:0)|((s.thirdPersonView!=0)?2:0)|(s.anaglyph?4:0)|(s.debugCamEnable?8:0)|(s.hideGUI?16:0));o.i32(c%2);o.i32(c*17);o.i32(s.renderDistance);
        EntityRenderer er=p.mc.entityRenderer;Oracle.setField(er,"rendererUpdateCount",c*17);
        float[] extras={-.75F,.5F,1.25F,-.35F,-31.25F,12.5F,20.75F,-17.25F};
        String[] fields={"prevCamRoll","camRoll","prevDebugCamFOV","debugCamFOV","prevDebugCamYaw","debugCamYaw","prevDebugCamPitch","debugCamPitch"};
        for(int i=0;i<fields.length;i++){f(o,extras[i]);Oracle.setField(er,fields[i],extras[i]);}
        // 1.0 FOV: the FOV option and the sprint/bow zoom (field_35813_N previous, field_35812_M current).
        float option=(c%5)*.25F,zp=1F+(c%3)*.05F,zc=1F+(c%7)*.03F;s.fovSetting=option;
        Oracle.setField(er,"fovModifierHandPrev",zp);Oracle.setField(er,"fovModifierHand",zc);f(o,option);f(o,zp);f(o,zc);
        double zoom=c%3==0?1.35:1.0;double yaw=.125,pitch=-.0625;d(o,zoom);d(o,yaw);d(o,pitch);Oracle.setField(er,"cameraZoom",zoom);Oracle.setField(er,"cameraYaw",yaw);Oracle.setField(er,"cameraPitch",pitch);
    }
    public static void run(String path)throws Exception {
        RendererOracle.field("convertQuadsToTriangles").setBoolean(null,false);
        Oracle.DataOut out=new Oracle.DataOut();int n=0;
        // Camera + projection + FOV, all combinations of debug, bob, stereo,
        // third person, death, materials, portal, bed directions and fractions.
        for(int c=0;c<320;c++) {
            World w=world();PlayerOracle.HeadlessPlayer p=player(w,c);setup(p);
            int medium=c%3;p.health=c%7==0?0:20;p.deathTime=c%47;p.sleeping=c%11==0;
            p.prevTimeInPortal=c%6*.13F;p.timeInPortal=c%5*.19F;
            out.i32(0);state(out,p);float t=new float[]{0,.125F,.5F,.9F,1}[c%5];f(out,t);settings(out,p,c);
            out.i32(c<256?2:10);block(out,w,0,65,-3,p.sleeping?26:medium==1?9:medium==2?11:0,c%4);block(out,w,0,65,-6,1,0);
            if(c>=256){double range=.5+(c%7)*.5;double dx=-MathHelper.sin(p.rotationYaw/180F*(float)Math.PI)*MathHelper.cos(p.rotationPitch/180F*(float)Math.PI),dz=MathHelper.cos(p.rotationYaw/180F*(float)Math.PI)*MathHelper.cos(p.rotationPitch/180F*(float)Math.PI),dy=-MathHelper.sin(p.rotationPitch/180F*(float)Math.PI);for(int i=0;i<8;i++)block(out,w,MathHelper.floor_double(p.posX-dx*range)+i%2,MathHelper.floor_double(p.posY-dy*range)+i/2%2,MathHelper.floor_double(p.posZ-dz*range)+i/4%2,1,0);}
            start(1);call(p.mc.entityRenderer,"setupCameraTransform",new Class[]{float.class,int.class},t,c%2);stop();
            f(out,(Float)call(p.mc.entityRenderer,"getFOVModifier",new Class[]{float.class,boolean.class},t,true));ops(out,projectionOps);for(float v:projection)f(out,v);ops(out,RendererOracle.ops);
            for(float v:matrix(RendererOracle.ops))f(out,v);for(float v:multiply(matrix(projectionOps),projection))f(out,v);n++;
        }
        // First person: every item, four swing/equip/partial states, plus empty hand.
        for(int id=0;id<Item.itemsList.length;id++)if(id==0||Item.itemsList[id]!=null) {
            for(int c=0;c<4;c++) {
                World w=world();PlayerOracle.HeadlessPlayer p=player(w,c+1);setup(p);p.hurtTime=c==2?7:0;
                if(id==346&&c>=2)p.fishEntity=new EntityFishHook(w);out.i32(p.fishEntity==null?1:10);state(out,p);float t=new float[]{0,.25F,.75F,1}[c];f(out,t);settings(out,p,1);
                // Render hand itself (normal non-stereo first person).
                p.mc.gameSettings.anaglyph=false;p.mc.gameSettings.thirdPersonView=0;p.mc.gameSettings.debugCamEnable=false;
                ItemStack item=id==0?null:new ItemStack(id,1,new int[]{0,1,3,15}[c]);out.i32(id);out.i32(item==null?0:item.getItemDamage());f(out,c*.23F);f(out,1-c*.19F);
                if(id==358)mapSetup(p,item.getItemDamage(),c);
                float brightness=w.getLightBrightness(MathHelper.floor_double(p.posX),MathHelper.floor_double(p.posY),MathHelper.floor_double(p.posZ));f(out,brightness);f(out,p.getEntityBrightness(1));
                Oracle.setField(p.mc.entityRenderer.itemRenderer,"itemToRender",item);Oracle.setField(p.mc.entityRenderer.itemRenderer,"prevEquippedProgress",c*.23F);Oracle.setField(p.mc.entityRenderer.itemRenderer,"equippedProgress",1-c*.19F);
                start(brightness);call(p.mc.entityRenderer,"renderHand",new Class[]{float.class,int.class},t,0);stop();draws(out);n++;
                if(id==358){int[] pixels=(int[])Oracle.getField(Oracle.getField(p.mc.entityRenderer.itemRenderer,"mapItemRenderer"),"intArray");for(int v:pixels)out.i32(v);out.bytes(GL11.uploaded);}
            }
        }
        // Overlay branch combinations include the vanilla double opaque draw.
        for(int c=0;c<32;c++) {
            World w=world();PlayerOracle.HeadlessPlayer p=player(w,c);setup(p);Harness10.setInt(p,"fire",(c&1)!=0?20:0);p.mc.gameSettings.thirdPersonView=0;p.mc.gameSettings.anaglyph=false;
            out.i32(2);state(out,p);float t=.35F;f(out,t);settings(out,p,0);
            out.i32(2);block(out,w,0,65,-3,(c&2)!=0?1:(c&4)!=0?9:0,0);block(out,w,-1,65,-3,(c&8)!=0?3:0,0);
            f(out,p.getEntityBrightness(t));out.i32(p.isInsideOfMaterial(Material.water)?1:0);
            start(1);GL11.glDisable(GL11.GL_LIGHTING);p.mc.entityRenderer.itemRenderer.renderOverlays(t);stop();draws(out);n++;
        }
        // Player model + all armor materials, sneaking, death and sleeping.
        RenderPlayer persistent=new RenderPlayer();
        for(int c=0;c<160;c++) {
            World w=world();PlayerOracle.HeadlessPlayer p=player(w,c);RenderManager m=setup(p);p.movementInput.sneak=c%3==0;p.health=c%9==0?0:20;p.deathTime=c%9==0?c%25:0;p.sleeping=c%7==0;
            if(c>=80){persistent.setRenderManager(m);((HashMap<Class,Render>)Oracle.getField(m,"entityRenderMap")).put(EntityPlayer.class,persistent);}
            if(c%4==0)p.ridingEntity=new EntityPig(w);
            p.bedChunkCoordinates=new ChunkCoordinates(0,65,-3);p.field_22063_x=.1F;p.field_22062_y=.2F;p.field_22061_z=-.15F;EntityOracle.rawBlock(w,0,65,-3,26,c%4);
            int[] ids={0,1,35,44,53,267,346,260,280,351};int id=ids[c%ids.length];p.inventory.mainInventory[0]=id==0?null:new ItemStack(id,1,c%16);
            for(int a=0;a<4;a++)p.inventory.armorInventory[a]=c%6==5?null:new ItemStack(298+c%5*4+3-a,1,0);
            // Cancel large doubles before subtracting yOffset, as RenderManager
            // followed by RenderPlayer does. Reordering these loses low bits.
            if(c>=144){p.setPosition(p.posX,1099511627776.5,p.posZ);p.prevPosY=p.lastTickPosY=p.posY;}
            if(c>=80&&id==346)p.fishEntity=new EntityFishHook(w);out.i32(p.fishEntity==null?3:11);state(out,p);out.i32(p.ridingEntity==null?0:1);out.i32(c>=80?1:0);float t=new float[]{0,.25F,.75F,1}[c%4];f(out,t);out.i32(id);out.i32(c%16);for(ItemStack item:p.inventory.armorInventory)out.i32(item==null?0:item.itemID);out.i32(c%4);f(out,p.getEntityBrightness(t));
            double[] camera={-.5,c>=144?p.posY-(double)p.yOffset+.375:61.25,-3.75};for(double v:camera)d(out,v);
            start(p.getEntityBrightness(t));m.getEntityRenderObject(p).doRender(p,p.lastTickPosX+(p.posX-p.lastTickPosX)*t-camera[0],p.lastTickPosY+(p.posY-p.lastTickPosY)*t-camera[1],p.lastTickPosZ+(p.posZ-p.lastTickPosZ)*t-camera[2],0,t);stop();draws(out);n++;
        }
        // Every legal block selection state, including float-coordinate overrides.
        for(int id=1;id<256;id++)if(Block.blocksList[id]!=null)for(int meta=0;meta<16;meta++) {
            World w=world();PlayerOracle.HeadlessPlayer p=player(w,1);setup(p);int x=meta%3==0?30000001:0,y=65,z=-3;
            // Far coordinates get a fixture chunk, never generated terrain.
            if(x!=0){EntityOracle.FlatProvider provider=(EntityOracle.FlatProvider)w.chunkProvider;Chunk chunk=new Chunk(w,new byte[32768],x>>4,z>>4);provider.chunks.put(provider.key(x>>4,z>>4),chunk);}
            EntityOracle.rawBlock(w,x,y,z,id,meta);
            out.i32(4);out.i32(id);out.i32(meta);out.i32(x);out.i32(y);out.i32(z);
            Block base=Block.blocksList[id];for(double v:new double[]{Trace10.namedDouble("base.minX", base.minX),Trace10.namedDouble("base.minY", base.minY),Trace10.namedDouble("base.minZ", base.minZ),Trace10.namedDouble("base.maxX", base.maxX),Trace10.namedDouble("base.maxY", base.maxY),Trace10.namedDouble("base.maxZ", base.maxZ)})d(out,v);
            double[] camera={x+.125,61.25,-3.75};for(double v:camera)d(out,v);p.lastTickPosX=p.posX=camera[0];p.lastTickPosY=p.posY=camera[1];p.lastTickPosZ=p.posZ=camera[2];
            start(1);GL11.glDisable(GL11.GL_LIGHTING);p.mc.renderGlobal.drawSelectionBox(p,new MovingObjectPosition(x,y,z,1,Vec3D.createVector(0,0,0)),0,null,.5F);stop();draws(out);n++;
        }
        // Destruction geometry, side occlusion, all metadata and ten stages.
        for(int id=1;id<256;id++)if(Block.blocksList[id]!=null && new HashSet<Integer>(Arrays.asList(0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17)).contains(Block.blocksList[id].getRenderType()))for(int c=0;c<160;c++) {
            World w=world();PlayerOracle.HeadlessPlayer p=player(w,c);setup(p);int x=0,y=65,z=-3;EntityOracle.rawBlock(w,x,y,z,id,c%16);
            if(c%2==0)EntityOracle.rawBlock(w,1,65,-3,1,0);
            out.i32(5);out.i32(id);out.i32(c%16);out.i32(c%2==0?1:0);f(out,(c/16)*.1F+.01F);
            Block base=Block.blocksList[id];for(double v:new double[]{Trace10.namedDouble("base.minX", base.minX),Trace10.namedDouble("base.minY", base.minY),Trace10.namedDouble("base.minZ", base.minZ),Trace10.namedDouble("base.maxX", base.maxX),Trace10.namedDouble("base.maxY", base.maxY),Trace10.namedDouble("base.maxZ", base.maxZ)})d(out,v);
            double[] camera={.125,61.25,-3.75};for(double v:camera)d(out,v);p.lastTickPosX=p.posX=camera[0];p.lastTickPosY=p.posY=camera[1];p.lastTickPosZ=p.posZ=camera[2];
            p.mc.gameSettings.ambientOcclusion=true;Oracle.setField(p.mc.renderGlobal,"damagePartialTime",(c/16)*.1F+.01F);RenderBlocks.fancyGrass=true;
            start(1);GL11.glDisable(GL11.GL_LIGHTING);p.mc.renderGlobal.drawBlockBreaking(p,new MovingObjectPosition(x,y,z,1,Vec3D.createVector(0,0,0)),0,null,.5F);stop();draws(out);n++;
        }
        for(int c=0;c<192;c++) {
            World w=world();PlayerOracle.HeadlessPlayer p=player(w,c);setup(p);EntityRenderer er=p.mc.entityRenderer;
            w.worldProvider.isNether=(c&8)!=0;w.setWorldTime(c*1379);w.rainingStrength=w.prevRainingStrength=(c%5)*.17F;w.thunderingStrength=w.prevThunderingStrength=(c%7)*.11F;
            p.mc.gameSettings.renderDistance=c%4;p.mc.gameSettings.anaglyph=(c&4)!=0;
            EntityOracle.rawBlock(w,0,65,-3,c%3==1?9:c%3==2?11:0,c%16);
            float t=(c%11)*.1F,old=.3F,newer=.85F,far=(float)(256>>(c%4));int mode=c%3-1;
            // Deep players reach the 1.0 void fog (sky light / 16 + (y + 4) / 32 below 1).
            if(c%3==0){p.posY=-9.0+(c%41)*.75;p.lastTickPosY=p.posY-.4;}
            Oracle.setField(er,"fogColor2",old);Oracle.setField(er,"fogColor1",newer);Oracle.setField(er,"farPlaneDistance",far);
            out.i32(6);state(out,p);f(out,t);out.i32(c%4);out.i32((c&4)!=0?1:0);out.i32((c&8)!=0?1:0);out.i32(mode);f(out,far);f(out,old);f(out,newer);
            Vec3D sky=w.getSkyColor(p,t),fog=w.getFogColor(t);for(double v:new double[]{sky.xCoord,sky.yCoord,sky.zCoord,fog.xCoord,fog.yCoord,fog.zCoord})f(out,(float)v);
            f(out,w.getRainStrength(t));f(out,w.getWeightedThunderStrength(t));int active=ActiveRenderInfo.func_41066_a(w,p,t);out.i32(active!=0&&Block.blocksList[active].blockMaterial==Material.water?1:active!=0&&Block.blocksList[active].blockMaterial==Material.lava?2:0);f(out,w.getCelestialAngle(t));out.i32(p.getEntityBrightnessForRender(t));
            start(1);call(er,"updateFogColor",new Class[]{float.class},t);call(er,"setupFog",new Class[]{int.class,float.class},mode,t);stop();
            for(int i=0;i<3;i++)f(out,GL11.clearColor[i]);out.i32(GL11.fogMode);if(GL11.fogMode==GL11.GL_EXP){f(out,GL11.fogDensity);}else{f(out,GL11.fogStart);f(out,GL11.fogEnd);}n++;
        }
        World ew=world();PlayerOracle.HeadlessPlayer ep=player(ew,1);setup(ep);ItemRenderer eq=ep.mc.entityRenderer.itemRenderer;
        ItemStack[] stacks={new ItemStack(267,1,0),new ItemStack(1,12,0),new ItemStack(267,1,0),new ItemStack(267,1,7),new ItemStack(260,1,0)};
        for(int c=0;c<240;c++) {
            int index=new int[]{-1,0,0,0,0,1,1,1,1,2,2,2,2,3,3,3,3,4,-1,-1}[c%20],slot=(c/20)%4;
            ItemStack current=index<0?null:stacks[index];if(current!=null && c%20==3)current.setItemDamage((c/20)*3);
            boolean identity=current==Oracle.getField(eq,"itemToRender");ep.inventory.currentItem=slot;ep.inventory.mainInventory[slot]=current;
            out.i32(7);out.i32(current==null?0:current.itemID);out.i32(current==null?0:current.getItemDamage());out.i32(slot);out.i32(identity?1:0);
            if(identity && current!=null)out.i32(current.getItemDamage());else out.i32(-1);
            eq.updateEquippedItem();f(out,(Float)Oracle.getField(eq,"prevEquippedProgress"));f(out,(Float)Oracle.getField(eq,"equippedProgress"));out.i32((Integer)Oracle.getField(eq,"field_20099_f"));ItemStack shown=(ItemStack)Oracle.getField(eq,"itemToRender");out.i32(shown==null?0:shown.itemID);out.i32(shown==null?0:shown.getItemDamage());n++;
        }
        for(int c=0;c<192;c++) {
            World w=world();PlayerOracle.HeadlessPlayer p=player(w,c);setup(p);p.mc.playerController=new PlayerControllerSP(p.mc);
            p.rotationYaw=(c%8)*45;p.rotationPitch=(c%5)*25-50;p.prevRotationYaw=p.rotationYaw-30;p.prevRotationPitch=p.rotationPitch+10;
            out.i32(8);out.i32(p.entityId);state(out,p);float t=(c%5)*.25F;f(out,t);
            out.i32(4);for(int b=0;b<4;b++)block(out,w,b%2,64+b/2,-1,c%3==0?1:c%3==1?44:53,c%4);
            out.i32(3);for(int i=0;i<3;i++){EntityPig pig=new EntityPig(w);pig.setPosition(c%2==0?.25:.25+i*.6,64.5+i*.15,-2.75+(c%4)*.75);w.entityJoinedWorld(pig);out.i32(pig.entityId);for(double v:new double[]{Trace10.namedDouble("pig.posX", pig.posX),Trace10.namedDouble("pig.posY", pig.posY),Trace10.namedDouble("pig.posZ", pig.posZ)})d(out,v);}
            p.mc.entityRenderer.getMouseOver(t);MovingObjectPosition hit=p.mc.objectMouseOver;out.i32(hit==null?-1:hit.typeOfHit==EnumMovingObjectType.ENTITY?1:0);if(hit!=null){if(hit.typeOfHit==EnumMovingObjectType.ENTITY)out.i32(hit.entityHit.entityId);else{out.i32(hit.blockX);out.i32(hit.blockY);out.i32(hit.blockZ);out.i32(hit.sideHit);for(double v:new double[]{Trace10.namedDouble("hit.hitVec.xCoord", hit.hitVec.xCoord),Trace10.namedDouble("hit.hitVec.yCoord", hit.hitVec.yCoord),Trace10.namedDouble("hit.hitVec.zCoord", hit.hitVec.zCoord)})d(out,v);}}n++;
        }
        for(int c=0;c<32;c++) {
            World w=world();PlayerOracle.HeadlessPlayer p=player(w,c);setup(p);p.mc.gameSettings.anaglyph=(c&1)!=0;mapSetup(p,c,c%17);
            out.i32(9);out.i32(c);out.i32((c&1)!=0?1:0);out.i32(c%17);
            MapItemRenderer mr=(MapItemRenderer)Oracle.getField(p.mc.entityRenderer.itemRenderer,"mapItemRenderer");MapData data=(MapData)w.loadItemData(MapData.class,"map_"+c);
            start(1);mr.renderMap(p,p.mc.renderEngine,data);stop();draws(out);for(int v:(int[])Oracle.getField(mr,"intArray"))out.i32(v);out.bytes(GL11.uploaded);n++;
        }
        Oracle.DataOut file=new Oracle.DataOut();file.bytes(new byte[]{'F','1','7','3'});file.i32(1);file.i32(n);file.bytes(out.buf.toByteArray());try(OutputStream stream=new BufferedOutputStream(new FileOutputStream(path))){file.writeTo(stream);}
        System.out.println("first-person oracle: "+n+" original-bytecode cases");
    }
}


