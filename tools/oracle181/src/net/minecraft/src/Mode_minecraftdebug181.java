package net.minecraft.src;
import java.io.*;
import java.util.*;
import java.lang.reflect.*;
import net.minecraft.client.Minecraft;
/** Actual Minecraft.debugInfoEntities -> EffectRenderer + World delegation. */
public final class Mode_minecraftdebug181 {
    public static void main(String[] a)throws Exception {
        Random r=new Random(18109);
        Minecraft mc=(Minecraft)Oracle.allocate(PlayerOracle.HeadlessMinecraft.class);
        mc.effectRenderer=(EffectRenderer)Oracle.allocate(EffectRenderer.class);
        mc.theWorld=(World)Oracle.allocate(World.class);
        Field layers=EffectRenderer.class.getDeclaredField("fxLayers");layers.setAccessible(true);
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            for(int c=0;c<10000;c++) {
                List[] ls=new List[4];for(int j=0;j<4;j++){int n=r.nextInt(1025);EntityOracle.i(o,n);ls[j]=new ArrayList(Collections.nCopies(n,null));}
                layers.set(mc.effectRenderer,ls);int n=r.nextInt(1025);EntityOracle.i(o,n);
                mc.theWorld.loadedEntityList=new ArrayList(Collections.nCopies(n,null));
                WorldEventOracle.text(o,mc.debugInfoEntities());
            }
        }
    }
}
