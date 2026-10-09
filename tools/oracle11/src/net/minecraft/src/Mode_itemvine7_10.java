package net.minecraft.src;
import java.io.*;import java.util.*;
/** ItemColored (1.8.1 ItemVine): captured Block and subtype flag, including custom callbacks. */
public final class Mode_itemvine7_10 {
 static final class Probe extends Block {
  final ArrayList<int[]> calls=new ArrayList<int[]>();int color;
  Probe(int id){super(id,1,Material.plants);}
  public int getBlockTextureFromSideAndMetadata(int side,int damage){calls.add(new int[]{blockID,side,damage});return blockID*1009+side*31+damage*17;}
  public int getRenderColor(int damage){calls.add(new int[]{blockID,-1,damage});return color;}
 }
 public static void main(String[] a)throws Exception {int init=Block.stone.blockID;Probe first=new Probe(253),second=new Probe(254);
  try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[3])))) {
   for(Probe block:new Probe[]{first,second})for(boolean subtypes:new boolean[]{false,true})for(int color:new int[]{Integer.MIN_VALUE,-1,0,4764952,Integer.MAX_VALUE})for(int damage:new int[]{Integer.MIN_VALUE,-65537,-32768,-17,-1,0,1,2,3,15,16,17,32767,65536,Integer.MAX_VALUE}) {
    block.color=color;Item item=new ItemColored(block.blockID-256,subtypes);block.calls.clear();EntityOracle.i(o,Trace10.namedInt("item.getItemStackLimit()", item.getItemStackLimit()));EntityOracle.i(o,Trace10.namedInt("item.getMaxDamage()", item.getMaxDamage()));Trace10.writeBoolean(o, "item.getHasSubtypes()", item.getHasSubtypes());EntityOracle.i(o,Trace10.namedInt("item.getPlacedBlockMetadata(damage)", item.getMetadata(damage)));
    EntityOracle.i(o,Trace10.namedInt("item.getIconFromDamage(damage)", item.getIconFromDamage(damage)));EntityOracle.i(o,Trace10.namedInt("item.getColorFromDamage(damage, 0)", item.getColorFromDamage(damage, 0)));EntityOracle.i(o,Trace10.namedInt("block.calls.size()", block.calls.size()));for(int[] call:block.calls)for(int v:call)EntityOracle.i(o,v);
   }
  }
 }
}
