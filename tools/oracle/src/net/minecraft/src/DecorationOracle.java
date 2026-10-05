package net.minecraft.src;
import java.io.*;
import java.util.*;
import java.lang.reflect.*;
/** Original sign, painting, map and fishing bytecode fixtures. */
public final class DecorationOracle {
    static void paintingState(Oracle.DataOut o,EntityPainting p,World w)throws Exception {
        o.i32(p.art.ordinal());o.i32(p.direction);for(int v:new int[]{p.xPosition,p.yPosition,p.zPosition,EntityOracle.intField(EntityPainting.class,"field_695_c",p)})o.i32(v);
        for(double v:new double[]{p.posX,p.posY,p.posZ,p.boundingBox.minX,p.boundingBox.minY,p.boundingBox.minZ,p.boundingBox.maxX,p.boundingBox.maxY,p.boundingBox.maxZ})o.i64(Double.doubleToLongBits(v));
        Oracle.modelFloat(o,p.rotationYaw);Oracle.modelFloat(o,p.prevRotationYaw);o.i32(p.isDead?1:0);o.i32(p.beenAttacked?1:0);o.i32(p.func_410_i()?1:0);o.i64(EntityOracle.rngState(p.rand));o.i32(w.loadedEntityList.size());
    }
    public static void paintings(String path)throws Exception {
        Oracle.DataOut o=new Oracle.DataOut();
        int[] widths={8,1,2,1,2,4,4,4},heights={8,1,1,2,2,2,3,4};
        for(int scene=0;scene<12;scene++)for(int dir=0;dir<4;dir++)for(int seed=0;seed<8;seed++) {
            World w=Oracle.newWorld(0);w.chunkProvider=new EntityOracle.FlatProvider(w);w.rand.setSeed(300);OracleHooks.reset(100+seed);EntityOracle.mathRandom().setSeed(200);
            int width=widths[scene%8],height=heights[scene%8];int material=scene==8?20:scene==9?18:scene==10?44:scene==11?0:1;
            for(int a=0;a<width;a++)for(int b=0;b<height;b++) {
                int x=(dir==0||dir==2)?a-width/2:0,z=(dir==0||dir==2)?0:a-width/2;EntityOracle.rawBlock(w,x,67+b-height/2,z,material,0);
            }
            EntityPainting p=new EntityPainting(w,0,67,0,dir);paintingState(o,p,w);
            NBTTagCompound n=new NBTTagCompound();p.writeToNBT(n);EntityPainting q=new EntityPainting(w);q.readFromNBT(n);paintingState(o,q,w);
            if(scene%3==0) {w.entityJoinedWorld(p);EntityPainting overlap=new EntityPainting(w,0,67,0,dir);paintingState(o,overlap,w);}
            for(int y=64;y<73;y++)for(int a=-4;a<=4;a++){EntityOracle.rawBlock(w,(dir==0||dir==2)?a:0,y,(dir==0||dir==2)?0:a,0,0);}
            for(int tick=0;tick<103;tick++){p.onUpdate();paintingState(o,p,w);}
            p.attackEntityFrom(null,0);paintingState(o,p,w);
        }
        try(OutputStream f=new FileOutputStream(path)){o.writeTo(f);}System.out.println("paintings: 384 walls/seeds, persistence, overlap, support and damage replays");
    }
    static int[] paintingLight(EntityPainting e,float xx,float yy) {
        int x=MathHelper.floor_double(e.posX),y=MathHelper.floor_double(e.posY+(double)(yy/16.0F)),z=MathHelper.floor_double(e.posZ);
        if(e.direction==0)x=MathHelper.floor_double(e.posX+(double)(xx/16.0F));if(e.direction==1)z=MathHelper.floor_double(e.posZ-(double)(xx/16.0F));if(e.direction==2)x=MathHelper.floor_double(e.posX-(double)(xx/16.0F));if(e.direction==3)z=MathHelper.floor_double(e.posZ+(double)(xx/16.0F));return new int[]{x,y,z};
    }
    public static void paintingRenderers(String path)throws Exception {
        RendererOracle.field("convertQuadsToTriangles").setBoolean(null,false);World w=Oracle.newWorld(0);w.chunkProvider=new EntityOracle.FlatProvider(w);
        RenderManager m=RendererOracle.manager(w);((Map)Oracle.getField(m.renderEngine,"textureMap")).put("/art/kz.png",16);RenderPainting renderer=new RenderPainting();renderer.setRenderManager(m);Oracle.DataOut o=new Oracle.DataOut();o.i32(25*4*3);
        for(EnumArt art:EnumArt.values())for(int dir=0;dir<4;dir++)for(int scene=0;scene<3;scene++) {
            EntityPainting p=new EntityPainting(w,0,67,0,dir,art.title);o.i32(art.ordinal());o.i32(dir);
            for(int tx=0;tx<art.sizeX/16;tx++)for(int ty=0;ty<art.sizeY/16;ty++) {
                int[] xyz=paintingLight(p,-art.sizeX/2.0F+tx*16+8,-art.sizeY/2.0F+ty*16+8);
                Chunk c=w.getChunkFromBlockCoords(xyz[0],xyz[2]);c.blocklightMap.setNibble(xyz[0]&15,xyz[1],xyz[2]&15,scene==0?0:scene==1?15:(tx*7+ty*3)%16);
            }
            for(int tx=0;tx<art.sizeX/16;tx++)for(int ty=0;ty<art.sizeY/16;ty++) {int[] xyz=paintingLight(p,-art.sizeX/2.0F+tx*16+8,-art.sizeY/2.0F+ty*16+8);Oracle.modelFloat(o,w.getLightBrightness(xyz[0],xyz[1],xyz[2]));}
            RendererOracle.reset(.73F);RendererOracle.active=true;renderer.doRender(p,-.25,10.5,3.75,dir*90,0);RendererOracle.active=false;o.i32(RendererOracle.count);o.bytes(RendererOracle.draws.buf.toByteArray());
        }
        try(OutputStream f=new FileOutputStream(path)){o.writeTo(f);}System.out.println("painting renderers: 300 cases, every art and direction, varied lighting");
    }
    static class SignPlayer extends EntityPlayerSP {
        int opened;
        SignPlayer(net.minecraft.client.Minecraft mc,World w){super(mc,w,new Session("Player",""),0);}
        public void addStat(StatBase s,int count){}
        public void displayGUIEditSign(TileEntitySign s){opened++;}
    }
    static void text(Oracle.DataOut o,String s)throws Exception {byte[] b=s.getBytes("UTF-8");o.i32(b.length);o.bytes(b);}
    static void sign(Oracle.DataOut o,TileEntitySign s)throws Exception {
        o.i32(s==null?0:1);if(s!=null)for(String line:s.signText)text(o,line);
    }
    public static void signs(String path)throws Exception {
        World w=Oracle.newWorld(0);w.chunkProvider=new EntityOracle.FlatProvider(w);w.rand.setSeed(300);OracleHooks.reset(100);
        net.minecraft.client.Minecraft mc=(net.minecraft.client.Minecraft)Oracle.allocate(PlayerOracle.HeadlessMinecraft.class);
        mc.theWorld=w;SignPlayer p=new SignPlayer(mc,w);
        Oracle.DataOut o=new Oracle.DataOut();text(o,ChatAllowedCharacters.allowedCharacters);
        for(int support:new int[]{1,20,18,44,85,0,9})for(int side=0;side<6;side++)for(float yaw:new float[]{-720,-181,-180,-168.75F,-.1F,0,11.25F,179.99F,180,359.99F,720}) {
            EntityOracle.rawBlock(w,0,64,0,support,0);p.rotationYaw=yaw;p.opened=0;
            int[] dx={0,0,0,0,-1,1},dy={-1,1,0,0,0,0},dz={0,0,-1,1,0,0};int x=dx[side],y=64+dy[side],z=dz[side];
            if(y!=63)w.setBlock(x,y,z,0);
            ItemStack stack=new ItemStack(323,1,0);boolean used=Item.sign.onItemUse(stack,p,w,0,64,0,side);
            o.i32(used?1:0);o.i32(stack.stackSize);o.i32(p.opened);o.i32(w.getBlockId(x,y,z));o.i32(w.getBlockMetadata(x,y,z));
            TileEntitySign te=(w.getBlockId(x,y,z)==63||w.getBlockId(x,y,z)==68)?(TileEntitySign)w.getBlockTileEntity(x,y,z):null;sign(o,te);
            w.setBlockWithNotify(0,64,0,0);o.i32(w.getBlockId(x,y,z));o.i64(EntityOracle.rngState(w.rand));
            w.setBlock(x,y,z,0);if(y==63)EntityOracle.rawBlock(w,x,y,z,1,0);
        }
        TileEntitySign s=new TileEntitySign();GuiEditSign gui=new GuiEditSign(s);Random r=new Random(1729);
        for(int i=0;i<2000;i++) {
            int key=new int[]{200,208,28,14,1,0,0,0,0}[r.nextInt(9)];char c=(char)r.nextInt(256);gui.keyTyped(c,key);gui.updateScreen();
            o.i32(EntityOracle.intField(GuiEditSign.class,"editLine",gui));o.i32(EntityOracle.intField(GuiEditSign.class,"updateCounter",gui));sign(o,s);
        }
        NBTTagCompound n=new NBTTagCompound();for(int i=1;i<=4;i++)n.setString("Text"+i,"01234567890123456789012345");s.readFromNBT(n);sign(o,s);
        try(OutputStream f=new FileOutputStream(path)){o.writeTo(f);}System.out.println("signs: placement/support/RNG and 2000 edit operations");
    }
    public static void signRenderers(String path)throws Exception {
        RendererOracle.field("convertQuadsToTriangles").setBoolean(null,false);
        World w=Oracle.newWorld(0);w.chunkProvider=new EntityOracle.FlatProvider(w);
        RenderManager m=RendererOracle.manager(w);((Map)Oracle.getField(m.renderEngine,"textureMap")).put("/item/sign.png",15);
        TileEntityRenderer dispatcher=TileEntityRenderer.instance;dispatcher.renderEngine=m.renderEngine;
        Oracle.setField(dispatcher,"fontRenderer",new FontRenderer((GameSettings)Oracle.allocate(GameSettings.class),"/font/default.png",new FirstPersonOracle.FontEngine()));
        TileEntitySignRenderer renderer=new TileEntitySignRenderer();renderer.setTileEntityRenderer(dispatcher);Oracle.DataOut o=new Oracle.DataOut();o.i32(32*5);
        for(int block:new int[]{63,68})for(int metadata=0;metadata<16;metadata++)for(int edited=-1;edited<4;edited++) {
            EntityOracle.rawBlock(w,0,64,0,block,metadata);TileEntitySign s=new TileEntitySign();s.worldObj=w;s.xCoord=0;s.yCoord=64;s.zCoord=0;s.lineBeingEdited=edited;
            s.signText=new String[]{"Hello!","012345678901234","a W i",""};o.i32(block);o.i32(metadata);o.i32(edited);
            RendererOracle.reset(.73F);RendererOracle.active=true;renderer.renderTileEntitySignAt(s,-.25,10.5,3.75,0);RendererOracle.active=false;o.i32(RendererOracle.count);o.bytes(RendererOracle.draws.buf.toByteArray());
        }
        try(OutputStream f=new FileOutputStream(path)){o.writeTo(f);}System.out.println("sign renderers: 160 cases");
    }
}
