package net.minecraft.src;
import java.io.*;
import java.util.*;
import org.lwjgl.opengl.GL11;
import static net.minecraft.src.Mode_render_models10.*;

/** Original RenderEndPortal; only GL/resource endpoints are substituted. */
public final class Mode_render_portal10 {
    public static boolean active;
    static ArrayList<float[]> commands=new ArrayList<float[]>();
    public static void command(int code,float a,float b,float c,float d) {
        if(active)commands.add(new float[]{code,a,b,c,d});
    }
    static class Portal extends RenderEndPortal {
        protected void bindTextureByName(String name) {
            if(!name.equals("/misc/tunnel.png")&&!name.equals("/misc/particlefield.png"))throw new AssertionError(name);
            GL11.glBindTexture(GL11.GL_TEXTURE_2D,name.equals("/misc/tunnel.png")?80:83);
        }
    }
    public static void main(String[] args)throws Exception {
        out=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(args[0])));i(0x34315045);i(1);i(64);
        Portal renderer=new Portal();renderer.setTileEntityRenderer(TileEntityRenderer.instance);
        Random rng=new Random(31104);
        for(int c=0;c<64;c++) {
            double[] position={rng.nextDouble()*128-64,rng.nextDouble()*128-64,rng.nextDouble()*128-64};
            double[] player={rng.nextDouble()*60000000-30000000,rng.nextDouble()*256,rng.nextDouble()*60000000-30000000};
            float[] offset={rng.nextFloat()*.2F-.1F,rng.nextFloat()*.2F-.1F,rng.nextFloat()*.2F-.1F};
            if(c<8){position[1]=new double[]{0,1,-1,64,128,-.75,-.751,.25}[c];player[0]=.25;player[1]=65.5;player[2]=-2.75;}
            for(double v:position)d(v);for(double v:player)d(v);for(float v:offset)f(v);
            TileEntityRenderer.instance.playerX=player[0];TileEntityRenderer.instance.playerY=player[1];TileEntityRenderer.instance.playerZ=player[2];
            ActiveRenderInfo.field_41074_a=offset[0];ActiveRenderInfo.field_41072_b=offset[1];ActiveRenderInfo.field_41073_c=offset[2];
            RendererOracle.reset10();commands.clear();active=true;
            renderer.renderTileEntityAt(new TileEntityEndPortal(),position[0],position[1],position[2],.5F);
            active=false;
            i(commands.size());for(float[] command:commands){i((int)command[0]);for(int j=1;j<5;j++)f(command[j]);}
            Mode_render_effects10.draws();
        }
        out.close();System.out.println("Original RenderEndPortal: 64 cases, 16 layers, seeded colors, complete GL/texgen calls");
    }
}
