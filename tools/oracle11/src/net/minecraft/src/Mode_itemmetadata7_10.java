package net.minecraft.src;
import java.io.*;
/** All signed-short icon/color/name/placement inputs plus full int boundaries. */
public final class Mode_itemmetadata7_10 {
 static void row(DataOutputStream o,Item item,int d)throws Exception{
  EntityOracle.i(o,Trace10.namedInt("item.getIconFromDamage(d)", item.getIconFromDamage(d)));EntityOracle.i(o,Trace10.namedInt("item.getColorFromDamage(d, 0)", item.getColorFromDamage(d, 0)));EntityOracle.i(o,Trace10.namedInt("item.getPlacedBlockMetadata(d)", item.getMetadata(d)));
  if(item instanceof ItemSlab||item instanceof ItemCloth)WorldEventOracle.text(o,item.getItemNameIS(new ItemStack(item,1,d)));
 }
 public static void main(String[] a)throws Exception{int init=Block.stone.blockID;ColorizerGrass.setGrassBiomeColorizer(Oracle.jarImage("misc/grasscolor.png"));
  try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[3])))){
   for(int id:new int[]{44,35,18,17,98,31,106}) {
    Item item=Item.itemsList[id];EntityOracle.i(o,Trace10.namedInt("item.getItemStackLimit()", item.getItemStackLimit()));EntityOracle.i(o,Trace10.namedInt("item.getMaxDamage()", item.getMaxDamage()));Trace10.writeBoolean(o, "item.getHasSubtypes()", item.getHasSubtypes());
    for(boolean fancy:new boolean[]{false,true}){Block.leaves.setGraphicsLevel(fancy);
     for(int d=-32768;d<=32767;d++)row(o,item,d);
     for(int d:new int[]{Integer.MIN_VALUE,-65537,-65536,-32769,32768,65535,65536,65537,Integer.MAX_VALUE})row(o,item,d);
    }
   }
  }
 }
}
