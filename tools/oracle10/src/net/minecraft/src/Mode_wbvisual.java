package net.minecraft.src;
import java.io.*;
public final class Mode_wbvisual {
    public static void main(String[] a)throws Exception {
        int init=Block.stone.blockID;
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            for(int fancy=0;fancy<2;fancy++) {
                ((BlockLeaves)Block.leaves).setGraphicsLevel(fancy!=0);
                for(int id=0;id<256;id++) {
                    Block b=Block.blocksList[id];EntityOracle.i(o,Trace10.namedInt("b==null?-1:b.getRenderType()", b==null?-1:b.getRenderType()));
                    for(int side=0;side<6;side++)EntityOracle.i(o,Trace10.namedInt("b==null?0:b.getBlockTextureFromSide(side)", b==null?0:b.getBlockTextureFromSide(side)));
                    for(int meta=0;meta<16;meta++) {
                        EntityOracle.i(o,Trace10.namedInt("b==null?0:b.getRenderColor(meta)", b==null?0:b.getRenderColor(meta)));
                        for(int side=0;side<6;side++){int v=0;try{v=b==null?0:b.getBlockTextureFromSideAndMetadata(side,meta);}catch(ArrayIndexOutOfBoundsException e){v=Integer.MIN_VALUE;}EntityOracle.i(o,v);}
                    }
                }
            }
            World w=Mode_worldblocks10.fixture(true);EntityPlayer p=PlayerOracle.player(w);ContainerCreative c=new ContainerCreative(p);
            EntityOracle.i(o,Trace10.namedInt("c.itemList.size()", c.itemList.size()));for(Object v:c.itemList){ItemStack s=(ItemStack)v;EntityOracle.i(o,Trace10.namedInt("s.itemID", s.itemID));EntityOracle.i(o,Trace10.namedInt("s.stackSize", s.stackSize));EntityOracle.i(o,Trace10.namedInt("s.getItemDamage()", s.getItemDamage()));}
        }
    }
}
