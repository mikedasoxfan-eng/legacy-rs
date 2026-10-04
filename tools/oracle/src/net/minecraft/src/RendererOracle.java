package net.minecraft.src;
import java.io.*;
import java.lang.reflect.*;
import java.util.*;
import org.lwjgl.opengl.GL11;
/** Execute original renderer bytecode and record GL inputs + original Tessellator bytes. */
public final class RendererOracle {
    public static boolean active=false;
    public static int flags=297,src=1,dst=0,depth=515;
    public static float[] normal={0,0,1};
    static ArrayList<float[]> ops=new ArrayList<float[]>();
    static ArrayList<ArrayList<float[]>> stack=new ArrayList<ArrayList<float[]>>();
    static Oracle.DataOut draws;
    static int count;
    public static void push(){stack.add(new ArrayList<float[]>(ops));}
    public static void pop(){ops=stack.remove(stack.size()-1);}
    public static void op(int c,float a,float b,float d,float e){ops.add(new float[]{c,a,b,d,e});}
    public static void enable(int c,boolean on) {
        int bit=c==2896?1:c==32826?2:c==2977?4:c==3553?8:c==3042?16:c==2929?256:0;
        if(on)flags|=bit;else flags&=~bit;
    }
    static Field field(String n) throws Exception {Field f=Tessellator.class.getDeclaredField(n);f.setAccessible(true);return f;}
    public static void record(int mode,int n) {
        if(!active)return;
        try {
            if(mode!=7)throw new AssertionError("expected quads, got "+mode);
            Tessellator t=Tessellator.instance;
            boolean color=field("hasColor").getBoolean(t),normalArray=field("hasNormals").getBoolean(t),uv=field("hasTexture").getBoolean(t);
            draws.i32(GL11.texture);for(float f:GL11.color)Oracle.modelFloat(draws,f);
            for(float f:normal)Oracle.modelFloat(draws,f);
            draws.i32(flags|(color?64:0)|(normalArray?128:0)|(uv?512:0));draws.i32(src);draws.i32(dst);draws.i32(depth);
            draws.i32(ops.size());for(float[] o:ops){draws.i32((int)o[0]);for(int i=1;i<5;i++)Oracle.modelFloat(draws,o[i]);}
            draws.i32(n);int[] raw=(int[])field("rawBuffer").get(t);
            for(int i=0;i<n;i++) {for(int j=0;j<5;j++)draws.i32(j>=3&&!uv?0:raw[i*8+j]);draws.i32(color?raw[i*8+5]:0);draws.i32(normalArray?raw[i*8+6]:0);}
            count++;
        }catch(Exception e){throw new RuntimeException(e);}
    }
    static void reset(float brightness) {
        ops.clear();stack.clear();flags=297;src=1;dst=0;depth=515;normal=new float[]{0,0,1};
        GL11.color=new float[]{brightness,brightness,brightness,1};GL11.texture=0;
        draws=new Oracle.DataOut();count=0;
    }
    static RenderManager manager(World w) throws Exception {
        RenderManager m=(RenderManager)Oracle.allocate(RenderManager.class);
        m.worldObj=w;m.playerViewY=37.25F;m.playerViewX=-21.5F;
        m.options=(GameSettings)Oracle.allocate(GameSettings.class);m.options.fancyGraphics=true;
        m.renderEngine=(RenderEngine)Oracle.allocate(RenderEngine.class);
        HashMap<String,Integer> textures=new HashMap<String,Integer>();
        textures.put("/terrain.png",1);textures.put("/gui/items.png",2);textures.put("/item/arrows.png",3);textures.put("%clamp%/misc/shadow.png",4);
        Oracle.setField(m.renderEngine,"textureMap",textures);Oracle.setField(m.renderEngine,"texturePack",Oracle.allocate(TexturePackList.class));
        return m;
    }
    static void writeCase(Oracle.DataOut out,int kind,Entity e,Render renderer,RenderManager manager,int id,int number,int damage,int age,float hover,int fuse,int shake,float partial) throws Exception {
        e.setPosition(.25,70.5,-2.75);
        float brightness=e.getEntityBrightness(partial);
        out.i32(kind);out.i32(id);out.i32(number);out.i32(damage);out.i32(age);Oracle.modelFloat(out,hover);out.i32(fuse);out.i32(shake);Oracle.modelFloat(out,partial);Oracle.modelFloat(out,brightness);
        Oracle.modelFloat(out,37.25F);Oracle.modelFloat(out,-21.5F);
        Oracle.modelFloat(out,13.5F);Oracle.modelFloat(out,-26.75F);
        double[] rel={-.25,10.5,3.75};for(double x:rel)out.i64(Double.doubleToRawLongBits(x));
        float[] light=new float[7];int[][] ds={{0,0,0},{0,-1,0},{0,1,0},{0,0,-1},{0,0,1},{-1,0,0},{1,0,0}};
        for(int i=0;i<7;i++){light[i]=Block.blocksList[12].getBlockBrightness(manager.worldObj,0+ds[i][0],70+ds[i][1],-3+ds[i][2]);Oracle.modelFloat(out,light[i]);}
        reset(brightness);renderer.setRenderManager(manager);active=true;
        renderer.doRender(e,rel[0],rel[1],rel[2],0,partial);active=false;
        out.i32(count);out.bytes(draws.buf.toByteArray());
    }
    public static void run(String path) throws Exception {
        World w=Oracle.newWorld(0);w.setWorldTime(6000);w.skylightSubtracted=w.calculateSkylightSubtracted(1);
        for(int x=-1;x<=0;x++)for(int z=-1;z<=0;z++){w.getBlockId(x*16,64,z*16);Oracle.drainLighting(w);}
        RenderManager m=manager(w);Field convert=field("convertQuadsToTriangles");convert.setBoolean(null,false);
        Oracle.DataOut cases=new Oracle.DataOut();int n=0;
        int[] nums={1,2,6,21};float[] parts={0,.25F,.75F,1};
        for(int id=1;id<Item.itemsList.length;id++)if(Item.itemsList[id]!=null)for(int c=0;c<4;c++) {
            int damage=new int[]{0,1,3,15}[c];
            EntityItem e=new EntityItem(w,.25,70.5,-2.75,new ItemStack(id,nums[c],damage));e.age=c*317;e.field_804_d=c*.73F;
            writeCase(cases,0,e,new RenderItem(),m,id,nums[c],damage,e.age,e.field_804_d,0,0,parts[c]);n++;
        }
        for(int id:new int[]{12,13})for(int c=0;c<4;c++) {
            EntityFallingSand e=new EntityFallingSand(w,.25,70.5,-2.75,id);
            writeCase(cases,1,e,new RenderFallingSand(),m,id,1,0,0,0,0,0,parts[c]);n++;
        }
        for(int fuse:new int[]{80,79,11,10,9,5,4,1,0,-1})for(float partial:parts) {
            EntityTNTPrimed e=new EntityTNTPrimed(w,.25,70.5,-2.75);e.fuse=fuse;
            writeCase(cases,2,e,new RenderTNTPrimed(),m,46,1,0,0,0,fuse,0,partial);n++;
        }
        for(int shake:new int[]{0,1,7})for(float partial:parts) {
            EntityArrow e=new EntityArrow(w);e.prevRotationYaw=-17;e.rotationYaw=105;e.prevRotationPitch=23;e.rotationPitch=-72;
            // Chosen interpolation inputs in the dump are the interpolated results.
            e.prevRotationYaw=e.rotationYaw=13.5F;e.prevRotationPitch=e.rotationPitch=-26.75F;e.arrowShake=shake;
            writeCase(cases,3,e,new RenderArrow(),m,0,1,0,0,0,0,shake,partial);n++;
        }
        for(float partial:parts) {
            writeCase(cases,4,new EntityFireball(w),new RenderFireball(),m,332,1,0,0,0,0,0,partial);n++;
            writeCase(cases,5,new EntitySnowball(w),new RenderSnowball(14),m,332,1,0,0,0,0,0,partial);n++;
        }
        for(int id=1;id<Item.itemsList.length;id++)if(Item.itemsList[id]!=null)for(int damage:new int[]{0,1,3,15}) {
            cases.i32(6);cases.i32(id);cases.i32(1);cases.i32(damage);cases.i32(0);Oracle.modelFloat(cases,0);cases.i32(0);cases.i32(0);Oracle.modelFloat(cases,0);Oracle.modelFloat(cases,1);
            for(int i=0;i<4;i++)Oracle.modelFloat(cases,0);for(int i=0;i<3;i++)cases.i64(0);for(int i=0;i<7;i++)Oracle.modelFloat(cases,0);
            reset(1);active=true;new RenderItem().drawItemIntoGui(null,m.renderEngine,id,damage,Item.itemsList[id].getIconFromDamage(damage),12,9);active=false;
            cases.i32(count);cases.bytes(draws.buf.toByteArray());n++;
        }
        for(int id=1;id<Item.itemsList.length;id++)if(Item.itemsList[id]!=null && Item.itemsList[id].getMaxDamage()>0)for(int damage:new int[]{1,Item.itemsList[id].getMaxDamage()/2,Item.itemsList[id].getMaxDamage()}) {
            cases.i32(7);cases.i32(id);cases.i32(1);cases.i32(damage);cases.i32(0);Oracle.modelFloat(cases,0);cases.i32(0);cases.i32(0);Oracle.modelFloat(cases,0);Oracle.modelFloat(cases,1);
            for(int i=0;i<4;i++)Oracle.modelFloat(cases,0);for(int i=0;i<3;i++)cases.i64(0);for(int i=0;i<7;i++)Oracle.modelFloat(cases,0);
            reset(1);active=true;new RenderItem().renderItemOverlayIntoGUI(null,m.renderEngine,new ItemStack(id,1,damage),12,9);active=false;
            cases.i32(count);cases.bytes(draws.buf.toByteArray());n++;
        }
        Oracle.DataOut o=new Oracle.DataOut();o.i32(0x33373152);o.i32(1);o.i32(n);o.bytes(cases.buf.toByteArray());
        try(OutputStream os=new FileOutputStream(path)){o.writeTo(os);}System.out.println("renderers: "+n+" original-bytecode cases");
    }
    public static void effects(String path) throws Exception {
        World w=Oracle.newWorld(0);w.setWorldTime(6000);w.skylightSubtracted=0;
        for(int x=-2;x<=1;x++)for(int z=-2;z<=1;z++){w.getBlockId(x*16,64,z*16);Oracle.drainLighting(w);}
        for(int x=-5;x<=5;x++)for(int z=-5;z<=5;z++) {
            w.setBlock(x,68+(Math.abs(x+z)%3),z,new int[]{1,20,44,53,18}[Math.abs(x+z)%5]);
            for(int y=71;y<=90;y++)w.setBlock(x,y,z,0);
        }
        Oracle.drainLighting(w);RenderManager m=manager(w);
        field("convertQuadsToTriangles").setBoolean(null,false);
        Method shadow=Render.class.getDeclaredMethod("renderShadow",Entity.class,double.class,double.class,double.class,float.class,float.class);shadow.setAccessible(true);
        Method fire=Render.class.getDeclaredMethod("renderEntityOnFire",Entity.class,double.class,double.class,double.class,float.class);fire.setAccessible(true);
        Oracle.DataOut o=new Oracle.DataOut();o.i32(0x33373145);o.i32(1);o.i32(48);
        for(int k=0;k<6;k++)for(float partial:new float[]{0,.25F,.75F,1})for(int effect=0;effect<2;effect++) {
            Entity e=new EntityItem(w,.25,70.5,-.75,new ItemStack(1,1,0));e.width=new float[]{.25F,.6F,.9F,2,4,3.6F}[k];e.height=new float[]{.25F,1.8F,1.3F,.6F,4,10.8F}[k];
            e.lastTickPosX=-.125;e.lastTickPosY=69.875;e.lastTickPosZ=-1.25;
            double yoff=k*.03125;e.boundingBox.setBounds(e.posX-.5,e.posY-yoff,e.posZ-.5,e.posX+.5,e.posY-yoff+e.height,e.posZ+.5);
            float radius=new float[]{.15F,.5F,.7F,1,.5F,3}[k];float factor=k==0?.75F:1;
            Render renderer=new RenderItem();Field rs=Render.class.getDeclaredField("shadowSize");rs.setAccessible(true);rs.setFloat(renderer,radius);renderer.setRenderManager(m);
            o.i32(k);o.i32(effect);Oracle.modelFloat(o,partial);Oracle.modelFloat(o,e.width);Oracle.modelFloat(o,e.height);o.i64(Double.doubleToRawLongBits(e.boundingBox.minY));
            double[] pos={e.posX,e.posY,e.posZ},prev={e.lastTickPosX,e.lastTickPosY,e.lastTickPosZ},cam={0,69,4};
            for(double d:pos)o.i64(Double.doubleToRawLongBits(d));for(double d:prev)o.i64(Double.doubleToRawLongBits(d));for(double d:cam)o.i64(Double.doubleToRawLongBits(d));Oracle.modelFloat(o,m.playerViewY);
            double px=prev[0]+(pos[0]-prev[0])*partial,py=prev[1]+(pos[1]-prev[1])*partial,pz=prev[2]+(pos[2]-prev[2])*partial;
            Oracle.DataOut surfaces=new Oracle.DataOut();int ns=0;
            for(int x=MathHelper.floor_double(px-radius);x<=MathHelper.floor_double(px+radius);x++)for(int y=MathHelper.floor_double(py+e.getShadowSize()-radius);y<=MathHelper.floor_double(py+e.getShadowSize());y++)for(int z=MathHelper.floor_double(pz-radius);z<=MathHelper.floor_double(pz+radius);z++) {
                int id=w.getBlockId(x,y-1,z);if(id>0 && w.getBlockLightValue(x,y,z)>3) {surfaces.i32(x);surfaces.i32(y);surfaces.i32(z);surfaces.i32(id);Oracle.modelFloat(surfaces,w.getLightBrightness(x,y,z));ns++;}
            }
            o.i32(ns);o.bytes(surfaces.buf.toByteArray());
            float alpha=(float)((1-m.func_851_a(e.posX,e.posY,e.posZ)/256.0)*factor);
            // RenderManager camera distance is independent of the rendering origin.
            m.field_1222_l=cam[0];m.field_1221_m=cam[1];m.field_1220_n=cam[2];alpha=(float)((1-m.func_851_a(e.posX,e.posY,e.posZ)/256.0)*factor);
            reset(1);active=true;
            if(effect==0)shadow.invoke(renderer,e,px-cam[0],py-cam[1],pz-cam[2],alpha,partial);else fire.invoke(renderer,e,px-cam[0],py-cam[1],pz-cam[2],partial);
            active=false;o.i32(count);o.bytes(draws.buf.toByteArray());
        }
        try(OutputStream os=new FileOutputStream(path)){o.writeTo(os);}System.out.println("effects: 48 shadow/fire cases");
    }
    public static void itemTable(String path) throws Exception {
        StringBuilder s=new StringBuilder("//! Generated from original Beta Item methods by oracle itemtable.\n");
        s.append("pub fn properties(id:i16,damage:i16)->(i32,u32,i32) {\n let d=damage as i32;\n match id {\n");
        for(int id=1;id<Item.itemsList.length;id++)if(Item.itemsList[id]!=null) {
            Item item=Item.itemsList[id];s.append(" "+id+" => { let row = [");
            for(int d=0;d<16;d++)s.append("("+item.getIconFromDamage(d)+","+item.getColorFromDamage(d)+"u32,"+item.getMaxDamage()+"),");
            s.append("][(d&15) as usize]; ");
            if(id==351)s.append("(78+d%8*16+d/8,row.1,row.2)");else s.append("row");
            s.append(" },\n");
        }
        s.append(" _ => (0,0xffffff,0),\n }\n}\n");
        try(Writer writer=new OutputStreamWriter(new FileOutputStream(path),"UTF-8")){writer.write(s.toString());}
    }
}
