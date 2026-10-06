package net.minecraft.src;
import java.io.*;
import java.util.Random;
import org.lwjgl.opengl.GL11;
import static net.minecraft.src.Mode_render_models10.*;
/** Calls the original renderer and model bytecode. Only texture binding and GL are endpoints. */
public final class Mode_render_tiles10 {
    static String texture;
    static final class Book extends RenderEnchantmentTable {
        protected void bindTextureByName(String name){texture=name;}
    }
    static final class Crystal extends RenderEnderCrystal {
        protected void loadTexture(String name){texture=name;}
    }
    public static void main(String[] args)throws Exception{
        int kind=Integer.parseInt(args[0]);out=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(args[1])));
        i(0x30315452);i(1);i(kind);i(64);
        Random random=new Random(401006+kind);
        Book book=kind==0?new Book():null;Crystal crystal=kind==1?new Crystal():null;
        TileEntityEnchantmentTable tile=kind==0?new TileEntityEnchantmentTable():null;
        EntityEnderCrystal entity=kind==1?new EntityEnderCrystal(null):null;
        for(int c=0;c<64;c++){
            double x=(random.nextDouble()-.5)*128,y=(random.nextDouble()-.5)*128,z=(random.nextDouble()-.5)*128;
            float partial=c%4*.25F;int ticks=c==63?16777219:random.nextInt(200000)-100000;
            d(x);d(y);d(z);f(partial);i(ticks);GL11.reset();texture=null;
            if(kind==0){
                float[] values={random.nextFloat()*40-20,random.nextFloat()*40-20,random.nextFloat()*20-10,random.nextFloat()*20-10,random.nextFloat()*2-1,random.nextFloat()*2-1};
                if(c<4){values[0]=0;values[1]=new float[]{(float)Math.PI,-(float)Math.PI,(float)Math.PI*2,-(float)Math.PI*2}[c];}
                for(float v:values)f(v);
                tile.field_40068_a=ticks;tile.field_40067_p=values[0];tile.field_40069_h=values[1];tile.field_40065_c=values[2];tile.field_40063_b=values[3];tile.field_40060_g=values[4];tile.field_40059_f=values[5];
                book.renderTileEntityAt(tile,x,y,z,partial);
            }else {entity.field_41032_a=ticks;crystal.doRender(entity,x,y,z,0,partial);}
            s(texture);Mode_render_modeldraw10.draws();
        }
        out.close();System.out.println("Original 1.0 renderer bytecode: "+(kind==0?"RenderEnchantmentTable":"RenderEnderCrystal")+", 64 cases");
    }
}
