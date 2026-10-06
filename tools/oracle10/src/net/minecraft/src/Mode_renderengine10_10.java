package net.minecraft.src;
import java.io.*;import java.util.*;import java.awt.image.BufferedImage;import javax.imageio.ImageIO;
/** Exercise the actual getTexture branch order, new blurclamp prefix and flag reset.
 * Texture upload is the observing adapter; retained texturecheck covers conversion. */
public final class Mode_renderengine10_10 {
    static class Pack extends TexturePackBase {
        byte[] png;String requested;
        public InputStream getResourceAsStream(String path){requested=path;return new ByteArrayInputStream(png);}
    }
    static class Engine extends RenderEngine {
        boolean blur,clamp;int calls;
        Engine(TexturePackList packs){super(packs,new GameSettings());}
        public void setupTexture(BufferedImage image,int id){blur=blurTexture;clamp=clampTexture;calls++;}
    }
    public static void main(String[] a)throws Exception {
        Pack pack=new Pack();ByteArrayOutputStream bytes=new ByteArrayOutputStream();ImageIO.write(new BufferedImage(16,16,2),"png",bytes);pack.png=bytes.toByteArray();
        TexturePackList packs=(TexturePackList)Oracle.allocate(TexturePackList.class);packs.selectedTexturePack=pack;
        Engine engine=new Engine(packs);
        String[] prefixes={"","%clamp%","%blur%","%blurclamp%","%blur%%clamp%","%clamp%%blur%","%blurclamp%%blur%"};
        try(DataOutputStream out=new DataOutputStream(new FileOutputStream(a[1]))) {
            EntityOracle.i(out,2000);
            for(int c=0;c<2000;c++) {
                String name=prefixes[c%prefixes.length]+"/texture"+c+".png";
                int previous=engine.calls;int id=engine.getTexture(name);int again=engine.getTexture(name);
                Mode_worldcallbacks10.str(out,name);Mode_worldcallbacks10.str(out,pack.requested);
                Trace10.writeBoolean(out, "engine.blur", engine.blur);Trace10.writeBoolean(out, "engine.clamp", engine.clamp);Trace10.writeBoolean(out, "engine.blurTexture", engine.blurTexture);Trace10.writeBoolean(out, "engine.clampTexture", engine.clampTexture);
                Trace10.writeBoolean(out, "id==again", id==again);EntityOracle.i(out,Trace10.namedInt("engine.calls-previous", engine.calls-previous));
            }
        }
    }
}
