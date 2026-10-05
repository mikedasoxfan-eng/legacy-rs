package net.minecraft.src;
import java.io.*;
import java.util.*;
public final class Mode_component181 extends StructureComponent {
    Mode_component181(int direction){super(7);field_35025_h=direction;field_35024_g=new StructureBoundingBox(0,64,0,10,80,10);}
    public boolean func_35023_a(World w,Random r,StructureBoundingBox b){return true;}
    public static void main(String[] a)throws Exception {
        Random r=new Random(Long.parseLong(a[1]));
        try(DataOutputStream out=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[2])))) {
            for(int d=-1;d<=4;d++) {
                Mode_component181 p=new Mode_component181(d);
                for(int x=-5;x<=5;x++)for(int z=-5;z<=5;z++){out.writeInt(p.func_35017_a(x,z));out.writeInt(p.func_35008_a(x));out.writeInt(p.func_35006_b(x,z));}
                for(int id=0;id<110;id++)for(int meta=-1;meta<17;meta++)out.writeInt(p.func_35009_c(id,meta));
                World w=Mode_terrainround2.flat(0,false);w.rand.setSeed(300);BlockSand.fallInstantly=true;
                StructureBoundingBox clip=new StructureBoundingBox(-4,0,-4,14,127,14);
                out.writeBoolean(p.func_35013_a(w,clip));EntityOracle.rawBlock(w,0,63,0,9,0);out.writeBoolean(p.func_35013_a(w,clip));EntityOracle.rawBlock(w,0,63,0,2,0);
                p.func_35011_a(w,clip,0,0,0,10,8,10,1,0,false);
                java.lang.reflect.Constructor<StructureStrongholdStones> ctor=StructureStrongholdStones.class.getDeclaredConstructor();ctor.setAccessible(true);
                p.func_35022_a(w,clip,0,0,0,10,6,10,true,r,ctor.newInstance());
                p.func_35010_a(w,clip,r,0.7F,0,1,0,10,4,10,5,0,false);
                p.func_35014_a(w,clip,r,1.0F,3,3,3,5,2);
                p.func_35015_a(w,clip,0,1,0,10,5,10,0,false);
                p.func_35016_b(w,2,0,2,clip);p.func_35005_b(w,4,0,3,-1,3,clip);
                p.func_35011_a(w,clip,5,-1,5,7,-1,7,1,1,false);
                p.func_35002_a(w,clip,r,6,0,6,p.func_35009_c(64,1));
                p.func_35003_a(w,clip,r,8,0,8,new StructurePieceTreasure[]{new StructurePieceTreasure(257,0,2,4,1),new StructurePieceTreasure(265,2,3,8,3)},12);
                out.writeInt(p.func_35007_a(w,6,0,6,clip));out.writeInt(p.func_35007_a(w,99,0,99,clip));
                out.writeLong(EntityOracle.rngState(r));out.writeLong(EntityOracle.rngState(w.rand));
                for(int cx=-3;cx<=3;cx++)for(int cz=-3;cz<=3;cz++){Chunk c=w.getChunkFromChunkCoords(cx,cz);out.write(c.blocks);out.write(c.data.data);}
                Mode_structures181.tiles(out,w,-48,63);
            }
        }
    }
}
