package net.minecraft.src;
import java.io.*;
import java.util.*;
import java.lang.reflect.*;
/** Original sign, painting, map and fishing bytecode fixtures. */
public final class DecorationOracle {
    static void mapState(Oracle.DataOut o,MapData m,ItemStack stack) {
        for(int v:new int[]{stack.getItemDamage(),m.field_28180_b,m.field_28179_c,m.field_28178_d,m.field_28177_e,m.field_28175_g,m.isDirty()?1:0})o.i32(v);
        o.bytes(m.field_28176_f);o.i32(m.field_28173_i.size());
        for(Object value:m.field_28173_i){MapCoord c=(MapCoord)value;o.bytes(new byte[]{c.field_28217_a,c.field_28216_b,c.field_28220_c,c.field_28219_d});}
        o.i32(m.field_28174_h.size());for(Object value:m.field_28174_h){MapInfo info=(MapInfo)value;for(int v:info.field_28119_b)o.i32(v);for(int v:info.field_28124_c)o.i32(v);}
    }
    static World mapWorld()throws Exception {
        World w=Oracle.newWorld(0);EntityOracle.FlatProvider provider=new EntityOracle.FlatProvider(w);w.chunkProvider=provider;
        int[] blocks={1,2,3,12,9,11,18,20,35,41,42,79,80,82,17,78};
        for(int cx=-10;cx<=10;cx++)for(int cz=-10;cz<=10;cz++) {
            Chunk c=new Chunk(w,new byte[32768],cx,cz);c.isChunkLoaded=true;c.isTerrainPopulated=true;
            for(int x=0;x<16;x++)for(int z=0;z<16;z++) {
                int wx=cx*16+x,wz=cz*16+z,h=60+((wx*3+wz*5)&7),id=blocks[(wx/8+wz/8)&15];
                c.blocks[(x<<11)|(z<<7)|55]=1;
                for(int y=56;y<h;y++)c.blocks[(x<<11)|(z<<7)|y]=3;
                c.blocks[(x<<11)|(z<<7)|h]=(byte)id;
                if(id==9||id==11)for(int y=h-((wx+wz)&3);y<h;y++)c.blocks[(x<<11)|(z<<7)|y]=(byte)id;
                if((wx+wz&7)==0)c.blocks[(x<<11)|(z<<7)|(h+1)]=20;
                c.heightMap[z<<4|x]=(byte)(h+2);
            }provider.chunks.put(provider.key(cx,cz),c);
        }return w;
    }
    public static void maps(String path)throws Exception {
        Oracle.DataOut o=new Oracle.DataOut();for(int id=0;id<256;id++)o.i32(Block.blocksList[id]==null?0:Block.blocksList[id].blockMaterial.materialMapColor.colorIndex);
        World w=mapWorld();SignPlayer p=new SignPlayer((net.minecraft.client.Minecraft)Oracle.allocate(PlayerOracle.HeadlessMinecraft.class),w);p.mc.theWorld=w;
        for(int dimension:new int[]{0,-1})for(int scale=0;scale<5;scale++) {
            Oracle.setField(w,"worldProvider",dimension==0?new WorldProviderSurface():new WorldProviderHell());w.worldProvider.registerWorld(w);p.dimension=dimension;
            ItemStack stack=new ItemStack(358,1,100+scale);p.inventory.mainInventory[0]=stack;p.setPosition(-13.25,70,19.75);
            MapData m=new MapData("map_"+stack.getItemDamage());m.field_28180_b=-13;m.field_28179_c=19;m.field_28178_d=(byte)dimension;m.field_28177_e=(byte)scale;w.setItemData(m.field_28168_a,m);
            for(int tick=0;tick<48;tick++) {
                p.rotationYaw=tick*37.25F-400;p.setPosition(-13.25+(tick%9-4)*2.125,70,19.75+(tick%7-3)*3.125);
                Item.map.onUpdate(stack,w,p,0,tick%11!=0);mapState(o,m,stack);
            }
            NBTTagCompound n=new NBTTagCompound();m.writeToNBT(n);MapData copy=new MapData("copy");copy.readFromNBT(n);mapState(o,copy,stack);
        }
        for(int size:new int[]{16,64,128})for(int scale:new int[]{-1,0,3,5}) {
            NBTTagCompound n=new NBTTagCompound();n.setByte("dimension",(byte)-1);n.setInteger("xCenter",-123);n.setInteger("zCenter",256);n.setByte("scale",(byte)scale);n.setShort("width",(short)size);n.setShort("height",(short)size);byte[] colors=new byte[size*size];for(int i=0;i<colors.length;i++)colors[i]=(byte)(i%56);n.setByteArray("colors",colors);
            MapData m=new MapData("test");m.readFromNBT(n);mapState(o,m,new ItemStack(358,1,0));
        }
        MapData markers=new MapData("map_100");markers.field_28177_e=0;ItemStack held=new ItemStack(358,1,100);p.inventory.mainInventory[0]=held;p.dimension=0;
        SignPlayer second=new SignPlayer((net.minecraft.client.Minecraft)Oracle.allocate(PlayerOracle.HeadlessMinecraft.class),w);second.inventory.armorInventory[0]=new ItemStack(358,1,100);second.setPosition(-63.75,70,63.75);second.dimension=0;second.rotationYaw=179;
        for(int t=0;t<12;t++){p.setPosition(t==6?64.01:-64+t*.25,70,-63.75);p.rotationYaw=-370+t*31.25F;markers.func_28169_a(p,held);markers.func_28169_a(second,held);mapState(o,markers,held);if(t==7)p.isDead=true;if(t==9)second.inventory.armorInventory[0]=null;}p.isDead=false;
        World created=Oracle.newWorld(0);((Map)Oracle.getField(created.mapStorage,"idCounts")).put("map",Short.valueOf((short)32766));created.chunkProvider=new EntityOracle.FlatProvider(created);SignPlayer player=new SignPlayer((net.minecraft.client.Minecraft)Oracle.allocate(PlayerOracle.HeadlessMinecraft.class),created);player.setPosition(-13.25,70,19.75);
        for(int c=0;c<4;c++){ItemStack stack=new ItemStack(358,1,32767);if(c%2==0)Item.map.onCreated(stack,created,player);else Item.map.func_28012_a(stack,created);mapState(o,(MapData)created.loadItemData(MapData.class,"map_"+stack.getItemDamage()),stack);}
        try(OutputStream f=new FileOutputStream(path)){o.writeTo(f);}System.out.println("maps: palette, 480 terrain/marker frames, NBT sizes/scales and creation");
    }
    static class Files extends Oracle.NullSaveHandler {
        File dir;Files(String path){super(0);dir=new File(path,"data");dir.mkdirs();}
        public File func_28113_a(String name){return new File(dir,name+".dat");}
    }
    public static void mapFiles(String out)throws Exception {
        Files files=new Files(out+".save");MapStorage storage=new MapStorage(files);
        for(int id:new int[]{0,7,32767}) {
            MapData m=new MapData("map_"+id);m.field_28180_b=-123+id;m.field_28179_c=256-id;m.field_28178_d=(byte)(id==7?-1:0);m.field_28177_e=(byte)(id%5);
            for(int i=0;i<16384;i++)m.field_28176_f[i]=(byte)((i*17+id)%56);m.markDirty();storage.setData(m.field_28168_a,m);
        }
        ((Map)Oracle.getField(storage,"idCounts")).put("map",Short.valueOf((short)32766));storage.getUniqueDataId("map");storage.saveAllData();mapRead(out+".save",out);
    }
    public static void mapRead(String dir,String out)throws Exception {
        MapStorage storage=new MapStorage(new Files(dir));Oracle.DataOut o=new Oracle.DataOut();
        for(int id:new int[]{0,7,32767})mapState(o,(MapData)storage.loadData(MapData.class,"map_"+id),new ItemStack(358,1,id));
        o.i32(storage.getUniqueDataId("map"));try(OutputStream f=new FileOutputStream(out)){o.writeTo(f);}System.out.println("map files: gzip map data, uncompressed idcounts and signed-short rollover");
    }
    static Object itemRandom()throws Exception {Field f=Item.class.getDeclaredField("itemRand");f.setAccessible(true);return f.get(null);}
    static void fishState(DataOutputStream o,EntityFish f,SignPlayer p,ItemStack rod,World w)throws Exception {
        for(double v:new double[]{f.posX,f.posY,f.posZ,f.prevPosX,f.prevPosY,f.prevPosZ,f.motionX,f.motionY,f.motionZ,f.boundingBox.minX,f.boundingBox.minY,f.boundingBox.minZ,f.boundingBox.maxX,f.boundingBox.maxY,f.boundingBox.maxZ})EntityOracle.d(o,v);
        for(float v:new float[]{f.rotationYaw,f.rotationPitch,f.prevRotationYaw,f.prevRotationPitch,f.fallDistance})EntityOracle.f(o,v);
        for(int v:new int[]{f.isDead?1:0,f.onGround?1:0,f.isCollidedHorizontally?1:0,f.inWater?1:0,f.ticksExisted,f.shake,EntityOracle.intField(EntityFish.class,"ticksInGround",f),EntityOracle.intField(EntityFish.class,"ticksInAir",f),EntityOracle.intField(EntityFish.class,"ticksCatchable",f),((Boolean)Oracle.getField(f,"inGround"))?1:0,f.bobber==null?0:1,p.fishEntity==null?0:1,rod.getItemDamage(),rod.stackSize,p.caught,p.broken})EntityOracle.i(o,v);
        EntityOracle.l(o,EntityOracle.rngState(f.rand));EntityOracle.l(o,EntityOracle.rngState(w.rand));EntityOracle.l(o,EntityOracle.rngState(EntityOracle.mathRandom()));
        ArrayList<Entity> other=new ArrayList<Entity>();for(Object value:w.loadedEntityList)if(value!=p&&value!=f)other.add((Entity)value);EntityOracle.i(o,other.size());
        for(Entity e:other){EntityOracle.i(o,e instanceof EntityItem?((EntityItem)e).item.itemID:0);for(double v:new double[]{e.posX,e.posY,e.posZ,e.motionX,e.motionY,e.motionZ})EntityOracle.d(o,v);EntityOracle.l(o,EntityOracle.rngState(e.rand));}
        WorldEventOracle.snapshot(w,o);
    }
    public static void fishing(String path)throws Exception {
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(path)))) {
        for(int scene=0;scene<10;scene++)for(int seed=0;seed<3;seed++) {
            World w=Oracle.newWorld(0);w.chunkProvider=new EntityOracle.FlatProvider(w);w.rand.setSeed(300);EntityOracle.mathRandom().setSeed(200);SignPlayer p=new SignPlayer((net.minecraft.client.Minecraft)Oracle.allocate(PlayerOracle.HeadlessMinecraft.class),w);p.mc.theWorld=w;
            p.setPosition(.5,66,.5);p.rotationYaw=scene<3?scene*73.25F:0;p.rotationPitch=scene<3?35:0;ItemStack rod=new ItemStack(346,1,scene==9?64:0);p.inventory.mainInventory[0]=rod;w.entityJoinedWorld(p);
            if(scene<3||scene==6||scene==7)for(int x=-18;x<=18;x++)for(int z=-18;z<=18;z++)for(int y=64;y<=65;y++)EntityOracle.rawBlock(w,x,y,z,9,0);
            if(scene==1){w.worldInfo.setIsRaining(true);w.rainingStrength=1;}
            OracleHooks.reset(100+seed);EntityOracle.mathRandom().setSeed(200);((Random)itemRandom()).setSeed(500);
            WorldEventOracle.attach(w);Item.fishingRod.onItemRightClick(rod,w,p);EntityFish f=p.fishEntity;
            if(scene==3||scene==4){EntityPig pig=new EntityPig(w);pig.setPosition(.5,65.75,3.5);w.entityJoinedWorld(pig);}
            if(scene==5||scene==9){Oracle.setField(f,"inGround",true);f.shake=7;}
            if(scene==8){f.setVelocity(.25,-.125,.5);f.setPositionAndRotation2(-4,68,7,400,-30,5);}
            fishState(o,f,p,rod,w);
            for(int tick=0;tick<1600;tick++) {
                AxisAlignedBB.clearBoundingBoxPool();Vec3D.initialize();
                if(scene==6&&tick==50)p.setPosition(40,66,.5);
                if(scene==7&&tick==50)p.inventory.currentItem=1;
                if(scene==4&&tick==10&&f.bobber!=null)f.bobber.setEntityDead();
                if(!f.isDead)f.onUpdate();
                if(!f.isDead&&((EntityOracle.intField(EntityFish.class,"ticksCatchable",f)>0&&tick>20)||(scene==3&&f.bobber!=null)||(scene==9&&tick==10)))Item.fishingRod.onItemRightClick(rod,w,p);
                if(scene==5&&tick==1200){w.loadedEntityList.remove(p);w.updateEntities();}
                if(scene==5&&tick==1250)Item.fishingRod.onItemRightClick(rod,w,p);
                if(scene==5&&tick==1251){Item.fishingRod.onItemRightClick(rod,w,p);f=p.fishEntity;}
                fishState(o,f,p,rod,w);
            }
        }}System.out.println("fishing: 30 casts, 48000 ticks, rain/bites/catches/mobs/ground/line breaks/interpolation/rod durability/events");
    }
    public static void fishRenderers(String path)throws Exception {
        RendererOracle.field("convertQuadsToTriangles").setBoolean(null,false);World w=Oracle.newWorld(0);w.chunkProvider=new EntityOracle.FlatProvider(w);RenderManager m=RendererOracle.manager(w);((Map)Oracle.getField(m.renderEngine,"textureMap")).put("/particles.png",17);RenderFish r=new RenderFish();r.setRenderManager(m);Oracle.DataOut o=new Oracle.DataOut();o.i32(2*4*5*3);
        SignPlayer p=new SignPlayer((net.minecraft.client.Minecraft)Oracle.allocate(PlayerOracle.HeadlessMinecraft.class),w);EntityFish f=new EntityFish(w);f.prevPosX=-1.5;f.prevPosY=65.25;f.prevPosZ=2.75;f.setPosition(.25,64.5,-.75);p.prevPosX=1.5;p.prevPosY=66.25;p.prevPosZ=3.75;p.setPosition(-.25,65.5,.75);
        for(int third=0;third<2;third++)for(float yaw:new float[]{-181,-30,0,217})for(float swing:new float[]{0,.125F,.5F,.875F,1})for(float partial:new float[]{0,.5F,1}) {
            p.prevRotationYaw=yaw-17;p.rotationYaw=yaw;p.prevRotationPitch=-23;p.rotationPitch=37;p.prevRenderYawOffset=yaw+11;p.renderYawOffset=yaw-7;p.prevSwingProgress=swing;p.swingProgress=(swing+.2F)%1;f.angler=p;m.options.thirdPersonView=third!=0;m.playerViewY=37;m.playerViewX=-17;
            o.i32(third);Oracle.modelFloat(o,yaw);Oracle.modelFloat(o,swing);Oracle.modelFloat(o,partial);RendererOracle.reset(.73F);RendererOracle.active=true;r.doRender(f,-.25,10.5,3.75,0,partial);RendererOracle.active=false;o.i32(RendererOracle.count);o.bytes(RendererOracle.draws.buf.toByteArray());
        }
        try(OutputStream out=new FileOutputStream(path)){o.writeTo(out);}System.out.println("fish renderers: 120 cases, first/third person, swing and partial ticks");
    }
    static void paintingState(Oracle.DataOut o,EntityPainting p,World w)throws Exception {
        o.i32(p.art.ordinal());o.i32(p.direction);for(int v:new int[]{p.xPosition,p.yPosition,p.zPosition,EntityOracle.intField(EntityPainting.class,"field_695_c",p)})o.i32(v);
        for(double v:new double[]{p.posX,p.posY,p.posZ,p.boundingBox.minX,p.boundingBox.minY,p.boundingBox.minZ,p.boundingBox.maxX,p.boundingBox.maxY,p.boundingBox.maxZ})o.i64(Double.doubleToLongBits(v));
        Oracle.modelFloat(o,p.rotationYaw);Oracle.modelFloat(o,p.prevRotationYaw);o.i32(p.isDead?1:0);o.i32(p.beenAttacked?1:0);o.i32(p.canStay()?1:0);o.i64(EntityOracle.rngState(p.rand));o.i32(w.loadedEntityList.size());
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
        for(EnumArt art:EnumArt.values())for(int dir=0;dir<4;dir++)for(int action=0;action<4;action++) {
            World w=Oracle.newWorld(0);w.chunkProvider=new EntityOracle.FlatProvider(w);w.rand.setSeed(300);OracleHooks.reset(100);EntityOracle.mathRandom().setSeed(200);
            EntityPainting p=new EntityPainting(w);NBTTagCompound n=new NBTTagCompound();n.setString("Motive",action==3?"invalid art":art.title);n.setByte("Dir",(byte)dir);n.setInteger("TileX",0);n.setInteger("TileY",67);n.setInteger("TileZ",0);p.readEntityFromNBT(n);paintingState(o,p,w);
            for(int repeat=0;repeat<2;repeat++){if(action==0)p.attackEntityFrom(null,0);else if(action==1)p.moveEntity(.125,0,-.25);else if(action==2)p.addVelocity(0,.25,0);else p.moveEntity(0,0,0);paintingState(o,p,w);o.i32(w.loadedEntityList.size());for(Object v:w.loadedEntityList){Entity e=(Entity)v;for(double x:new double[]{e.posX,e.posY,e.posZ,e.motionX,e.motionY,e.motionZ})o.i64(Double.doubleToLongBits(x));o.i64(EntityOracle.rngState(e.rand));}o.i64(EntityOracle.rngState(EntityOracle.mathRandom()));}
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
        int opened,caught,broken;
        SignPlayer(net.minecraft.client.Minecraft mc,World w){super(mc,w,new Session("Player",""),0);}
        public void addStat(StatBase s,int count){if(s==StatList.fishCaughtStat)caught+=count;if(s==StatList.objectBreakStats[346])broken+=count;}
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
