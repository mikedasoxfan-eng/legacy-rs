package net.minecraft.src;
import java.io.*;
import java.util.*;
import java.lang.reflect.*;
/** Original sign, painting, map and fishing bytecode fixtures. */
public final class DecorationOracle {
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
