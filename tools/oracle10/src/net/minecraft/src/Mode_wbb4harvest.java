package net.minecraft.src;
import java.io.*;
import java.lang.reflect.*;

/** Block.harvestBlock (release Silk Touch / Fortune dispatch) and its overrides, for every block and metadata. */
public final class Mode_wbb4harvest {
    public static void main(String[] a)throws Exception {
        int init=Block.stone.blockID;
        Field exhaustion=FoodStats.class.getDeclaredField("foodExhaustionLevel");exhaustion.setAccessible(true);
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            for(int id=1;id<=122;id++)if(Block.blocksList[id]!=null)for(int meta:new int[]{0,1,2,3,5,8,15})for(int held=0;held<6;held++)for(boolean remote:new boolean[]{false,true}) {
                World w=Mode_worldblocks10.fixture(true);EntityPlayer p=Blocks4Oracle.newPlayer(w);Blocks4Oracle.place(p,8.5,66,6.5,0F,40F);
                if((meta&1)!=0)Mode_worldblocks10.raw(w,8,63,8,0,0);
                Mode_worldblocks10.raw(w,8,64,8,id,meta);
                ItemStack s=held==0?null:new ItemStack(new int[]{0,278,278,278,359,285}[held],1,0);
                if(held==2)s.addEnchantment(Enchantment.enchantmentsList[33],1);
                if(held==3)s.addEnchantment(Enchantment.enchantmentsList[35],3);
                if(held==5)s.addEnchantment(Enchantment.enchantmentsList[35],1);
                p.inventory.mainInventory[0]=s;w.multiplayerWorld=remote;Blocks4Oracle.seeds(w);
                Block.blocksList[id].harvestBlock(w,p,8,64,8,meta);
                Mode_wbplacement4.cells(o,w,7,9,63,65);Blocks4Oracle.entities(o,w);Blocks4Oracle.rngs(o,w);WorldEventOracle.snapshot(w,o);
                EntityOracle.f(o,Trace10.namedFloat("exhaustion",exhaustion.getFloat(p.getFoodStats())));
            }
            // BlockPistonMoving drops its stored block with fortune forced to zero.
            for(int stored:new int[]{89,73,18,1,59})for(int held=0;held<6;held++)for(boolean remote:new boolean[]{false,true}) {
                World w=Mode_worldblocks10.fixture(true);EntityPlayer p=Blocks4Oracle.newPlayer(w);Blocks4Oracle.place(p,8.5,66,6.5,0F,40F);
                Mode_worldblocks10.raw(w,8,64,8,36,3);w.setBlockTileEntity(8,64,8,new TileEntityPiston(stored,stored==59?7:2,3,true,false));
                ItemStack s=held==0?null:new ItemStack(new int[]{0,278,278,278,359,285}[held],1,0);
                if(held==2)s.addEnchantment(Enchantment.enchantmentsList[33],1);
                if(held==3)s.addEnchantment(Enchantment.enchantmentsList[35],3);
                if(held==5)s.addEnchantment(Enchantment.enchantmentsList[35],1);
                p.inventory.mainInventory[0]=s;w.multiplayerWorld=remote;Blocks4Oracle.seeds(w);
                Block.blocksList[36].harvestBlock(w,p,8,64,8,3);
                Mode_wbplacement4.cells(o,w,7,9,63,65);Blocks4Oracle.entities(o,w);Blocks4Oracle.rngs(o,w);WorldEventOracle.snapshot(w,o);
            }
        }
    }
}
