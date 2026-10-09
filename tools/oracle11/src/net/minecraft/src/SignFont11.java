package net.minecraft.src;
import java.io.*;
import java.lang.reflect.*;
import java.util.*;
import org.lwjgl.opengl.GL11;

/** Original sign renderer followed by a GUI draw on the very same FontRenderer. */
public final class SignFont11 {
    static class FontTextures extends RenderEngine {
        int next=1000;
        FontTextures(){super(null,new GameSettings());}
        public int allocateAndSetupTexture(java.awt.image.BufferedImage image){
            int id=next++;GL11.glBindTexture(3553,id);return id;
        }
    }
    static void text(Oracle.DataOut o,String s){o.i32(s.length());for(int i=0;i<s.length();i++)o.i32(s.charAt(i));}
    static void phase(Oracle.DataOut o,FontRenderer font,Field pages)throws Exception {
        int[] cache=(int[])pages.get(font);
        o.i32(font.fontTextureName);for(int id:cache)o.i32(id);
        o.i32(RendererOracle.count);o.bytes(RendererOracle.draws.buf.toByteArray());
    }
    public static void main(String[] args)throws Exception {
        RendererOracle.field("convertQuadsToTriangles").setBoolean(null,false);
        World world=Oracle.newWorld(0);world.chunkProvider=new EntityOracle.FlatProvider(world);
        RenderManager manager=RendererOracle.manager(world);
        ((Map)Oracle.getField(manager.renderEngine,"textureMap")).put("/item/sign.png",15);
        TileEntityRenderer dispatcher=TileEntityRenderer.instance;dispatcher.renderEngine=manager.renderEngine;
        TileEntitySignRenderer renderer=new TileEntitySignRenderer();renderer.setTileEntityRenderer(dispatcher);
        Field pages=FontRenderer.class.getDeclaredField("field_44034_f");pages.setAccessible(true);
        String[] strings={"Hello!", "\u05d0\u05d1 (ABC) \u05d2", "\u0627\u0661 ABC", "\u65e5\u672c\u8a9e", "\u00a7kABC\u00a7aDEF", "\u05d0\ud83d\ude00\u05d1", "\u202bABC\u202c"};
        Oracle.DataOut out=new Oracle.DataOut();out.i32(0x53463131);out.i32(4*2*4*3*strings.length);
        int index=0;
        for(boolean unicode:new boolean[]{false,true})for(boolean bidi:new boolean[]{false,true}) {
            FontRenderer font=new FontRenderer(new GameSettings(),"/font/default.png",new FontTextures(),unicode);
            font.func_46123_b(bidi);Oracle.setField(dispatcher,"fontRenderer",font);
            for(int block:new int[]{63,68})for(int metadata:new int[]{0,2,5,15})for(int edited:new int[]{-1,0,3})for(int choice=0;choice<strings.length;choice++) {
                long seed=4181L+(index++*813114L);
                out.i32(unicode?1:0);out.i32(bidi?1:0);out.i32(block);out.i32(metadata);out.i32(edited);out.i64(seed);
                EntityOracle.rawBlock(world,0,64,0,block,metadata);
                TileEntitySign sign=new TileEntitySign();sign.worldObj=world;sign.xCoord=0;sign.yCoord=64;sign.zCoord=0;sign.lineBeingEdited=edited;
                for(int line=0;line<4;line++){sign.signText[line]=strings[(choice+line)%strings.length];text(out,sign.signText[line]);}
                font.field_41064_c.setSeed(seed);
                RendererOracle.reset(.73F);RendererOracle.active=true;renderer.a(sign,-.25,10.5,3.75,0);RendererOracle.active=false;
                phase(out,font,pages);
                RendererOracle.reset(1);RendererOracle.active=true;font.drawString("\u00a7kABC \u05d0\u05d1",7,11,0x7fabcdef);RendererOracle.active=false;
                phase(out,font,pages);out.i64(font.field_41064_c.nextLong());
            }
        }
        try(OutputStream file=new FileOutputStream(args[0])){out.writeTo(file);}
        System.out.println("SIGN FONT ORIGINAL: "+index+" world/GUI sequences");
    }
}
