package net.minecraft.src;
import java.io.*;import java.util.*;
/** ItemVine's captured Block and independent subtype flag, including custom callbacks. */
public final class Mode_itemvine7_181 {
 static final class Probe extends Block {
  final ArrayList<int[]> calls=new ArrayList<int[]>();int color;
  Probe(int id){super(id,1,Material.plants);}
  public int getBlockTextureFromSideAndMetadata(int side,int damage){calls.add(new int[]{blockID,side,damage});return blockID*1009+side*31+damage*17;}
  public int func_35274_i(){calls.add(new int[]{blockID,-1,0});return color;}
 }
 public static void main(String[] a)throws Exception {int init=Block.stone.blockID;Probe first=new Probe(253),second=new Probe(254);
  try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[3])))) {
   for(Probe block:new Probe[]{first,second})for(boolean subtypes:new boolean[]{false,true})for(int color:new int[]{Integer.MIN_VALUE,-1,0,4764952,Integer.MAX_VALUE})for(int damage:new int[]{Integer.MIN_VALUE,-65537,-32768,-17,-1,0,1,2,3,15,16,17,32767,65536,Integer.MAX_VALUE}) {
    block.color=color;Item item=new ItemVine(block.blockID-256,subtypes);block.calls.clear();EntityOracle.i(o,item.getItemStackLimit());EntityOracle.i(o,item.getMaxDamage());o.writeBoolean(item.getHasSubtypes());EntityOracle.i(o,item.getPlacedBlockMetadata(damage));
    EntityOracle.i(o,item.getIconFromDamage(damage));EntityOracle.i(o,item.getColorFromDamage(damage));EntityOracle.i(o,block.calls.size());for(int[] call:block.calls)for(int v:call)EntityOracle.i(o,v);
   }
  }
 }
}
