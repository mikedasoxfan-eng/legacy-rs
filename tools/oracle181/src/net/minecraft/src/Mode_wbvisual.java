package net.minecraft.src;
import java.io.*;
public final class Mode_wbvisual {
    public static void main(String[] a)throws Exception {
        int init=Block.stone.blockID;
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            for(int fancy=0;fancy<2;fancy++) {
                ((BlockLeaves)Block.leaves).setGraphicsLevel(fancy!=0);
                for(int id=0;id<256;id++) {
                    Block b=Block.blocksList[id];EntityOracle.i(o,b==null?-1:b.getRenderType());
                    for(int side=0;side<6;side++)EntityOracle.i(o,b==null?0:b.getBlockTextureFromSide(side));
                    for(int meta=0;meta<16;meta++) {
                        EntityOracle.i(o,b==null?0:b.getRenderColor(meta));
                        for(int side=0;side<6;side++){int v=0;try{v=b==null?0:b.getBlockTextureFromSideAndMetadata(side,meta);}catch(ArrayIndexOutOfBoundsException e){v=Integer.MIN_VALUE;}EntityOracle.i(o,v);}
                    }
                }
            }
            World w=Mode_worldblocks181.fixture(true);EntityPlayer p=PlayerOracle.player(w);ContainerCreative c=new ContainerCreative(p);
            EntityOracle.i(o,c.field_35375_a.size());for(Object v:c.field_35375_a){ItemStack s=(ItemStack)v;EntityOracle.i(o,s.itemID);EntityOracle.i(o,s.stackSize);EntityOracle.i(o,s.getItemDamage());}
        }
    }
}
