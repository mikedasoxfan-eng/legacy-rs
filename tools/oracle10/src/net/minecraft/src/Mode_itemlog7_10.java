package net.minecraft.src;
import java.io.*;import java.util.*;
/** ItemLog's stored constructor Block, independent of its inherited placement Block. */
public final class Mode_itemlog7_10 {
 static final class Probe extends Block {
  final ArrayList<int[]> calls=new ArrayList<int[]>();
  Probe(int id){super(id,1,Material.rock);}
  public int getBlockTextureFromSideAndMetadata(int side,int damage){calls.add(new int[]{blockID,side,damage});return blockID*1009+side*31+damage*17;}
 }
 public static void main(String[] a)throws Exception {int init=Block.stone.blockID;Probe first=new Probe(253),second=new Probe(254);
  try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[3])))) {
   for(int placement:new int[]{1,17,31,98})for(Block texture:new Block[]{first,second,null})for(int damage:new int[]{Integer.MIN_VALUE,-65537,-32768,-17,-1,0,1,2,3,15,16,17,32767,65536,Integer.MAX_VALUE}) {
    first.calls.clear();second.calls.clear();Item item=new ItemMetadata(placement-256,texture);EntityOracle.i(o,Trace10.namedInt("item.getItemStackLimit()", item.getItemStackLimit()));EntityOracle.i(o,Trace10.namedInt("item.getMaxDamage()", item.getMaxDamage()));Trace10.writeBoolean(o, "item.getHasSubtypes()", item.getHasSubtypes());EntityOracle.i(o,Trace10.namedInt("item.getPlacedBlockMetadata(damage)", item.getPlacedBlockMetadata(damage)));
    String error="";int icon=0;try{icon=item.getIconFromDamage(damage);}catch(RuntimeException e){error=e.getClass().getSimpleName();}EntityOracle.i(o,icon);WorldEventOracle.text(o,error);
    ArrayList<int[]> calls=texture==first?first.calls:second.calls;EntityOracle.i(o,Trace10.namedInt("calls.size()", calls.size()));for(int[] call:calls)for(int v:call)EntityOracle.i(o,v);
   }
  }
 }
}
