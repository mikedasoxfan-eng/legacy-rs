package net.minecraft.src;
import java.io.*;import java.util.*;
/** Original ItemStack's virtual compound calls and retained alias on failure. */
public final class Mode_stacknbt11_181 {
 static ItemStack alias;static int scene,fail;static ArrayList<long[]> events=new ArrayList<long[]>();
 static final class Probe extends NBTTagCompound {
  boolean[] present=new boolean[3];int[] written=new int[3];
  void call(int op,int value){events.add(new long[]{op,value,alias.itemID,alias.stackSize,alias.getItemDamage(),alias.animationsToGo});if(op==fail)throw new IllegalStateException("before");
   if(scene==1){alias.itemID=-17;alias.stackSize=Integer.MIN_VALUE;alias.setItemDamage(Integer.MAX_VALUE);alias.animationsToGo=-33;}
   if(scene==2){alias.itemID++;alias.stackSize+=7;alias.setItemDamage(alias.getItemDamage()+3);alias.animationsToGo++;}
   if(scene==3){alias.stackSize=-91;throw new IllegalStateException("after");}
  }
  public short getShort(String key){int op=key.equals("id")?0:2;int value=op==0?267:99;call(op,value);return(short)value;}
  public byte getByte(String key){call(1,-17);return -17;}
  public void setShort(String key,short value){int op=key.equals("id")?3:5;call(op,value);super.setShort(key,value);present[op==3?0:2]=true;written[op==3?0:2]=value;}
  public void setByte(String key,byte value){call(4,value);super.setByte(key,value);present[1]=true;written[1]=value;}
 }
 public static void main(String[] a)throws Exception {
  try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[3])))) {
   for(int op=0;op<2;op++)for(int scenario=0;scenario<4;scenario++)for(int failure=-1;failure<6;failure++)for(int value:new int[]{Integer.MIN_VALUE,-1,0,1,Integer.MAX_VALUE}) {
    scene=scenario;fail=failure;events.clear();alias=new ItemStack(value,value,value);alias.animationsToGo=17;Probe n=new Probe();boolean returned=false;String error="";
    try{if(op==0){alias.readFromNBT(n);returned=true;}else{returned=alias.writeToNBT(n)==n;}}catch(RuntimeException e){error=e.getClass().getSimpleName();}
    WorldEventOracle.text(o,error);o.writeBoolean(returned);Mode_itemstack11_181.state(o,alias);
    for(int k=0;k<3;k++){o.writeBoolean(n.present[k]);EntityOracle.i(o,n.written[k]);}
    EntityOracle.i(o,events.size());for(long[] row:events)for(long v:row)EntityOracle.l(o,v);
   }
  }
 }
}
