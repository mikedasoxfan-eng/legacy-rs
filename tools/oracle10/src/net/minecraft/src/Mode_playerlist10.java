package net.minecraft.src;
import java.io.*;import java.lang.reflect.*;import java.util.*;
/** Execute original PlayerList bytecode, including duplicate insertion and table order. */
public final class Mode_playerlist10 {
 static void i(DataOutputStream out,int value)throws Exception{EntityOracle.i(out,value);}
 static void value(DataOutputStream out,Object value)throws Exception{i(out,Trace10.namedInt("value==null?Integer.MIN_VALUE:(Integer)value", value==null?Integer.MIN_VALUE:(Integer)value));}
 public static void main(String[] args)throws Exception {
  LongHashMap list=new LongHashMap();Random random=new Random(181);
  Field buckets=LongHashMap.class.getDeclaredField("playerListEntries");buckets.setAccessible(true);
  try(DataOutputStream out=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(args[1])))) {
   for(int tick=0;tick<3000;tick++) {
    int op=random.nextInt(5);long key=(long)(random.nextInt(128)-64)|((long)(random.nextInt(5)-2)<<32);
    Integer v=tick%7==0?null:Integer.valueOf(random.nextInt());
    if(op<2)list.add(key,v);else if(op==2)value(out,list.remove(key));else if(op==3)value(out,list.getValueByKey(key));else Trace10.writeBoolean(out, "list.func_35575_b(key)", list.func_35575_b(key));
    LongHashMapEntry[] table=(LongHashMapEntry[])buckets.get(list);
    i(out,Trace10.namedInt("list.getNumHashElements()", list.getNumHashElements()));i(out,Trace10.namedInt("EntityOracle.intField(LongHashMap.class,\"capacity\",list)", EntityOracle.intField(LongHashMap.class,"capacity",list)));i(out,Trace10.namedInt("EntityOracle.intField(LongHashMap.class,\"field_35581_e\",list)", EntityOracle.intField(LongHashMap.class,"field_35581_e",list)));i(out,Trace10.namedInt("table.length", table.length));
    for(LongHashMapEntry head:table){int count=0;for(LongHashMapEntry entry=head;entry!=null;entry=entry.field_35833_c)count++;i(out,count);for(LongHashMapEntry entry=head;entry!=null;entry=entry.field_35833_c){EntityOracle.l(out,Trace10.namedLong("entry.func_35830_a()", entry.func_35830_a()));value(out,entry.func_35829_b());i(out,Trace10.namedInt("entry.hashCode()", entry.hashCode()));}}
   }
   for(int c=0;c<10000;c++) {
    long key=random.nextLong();Integer v=c%7==0?null:Integer.valueOf(random.nextInt());
    LongHashMapEntry first=new LongHashMapEntry(c,key,v,null);
    LongHashMapEntry second=new LongHashMapEntry(-c,c%3==0?key:key+1,c%2==0?v:Integer.valueOf(c),first);
    Trace10.writeBoolean(out, "first.equals(first)", first.equals(first));Trace10.writeBoolean(out, "first.equals(second)", first.equals(second));
    i(out,Trace10.namedInt("first.hashCode()", first.hashCode()));WorldEventOracle.text(out,first.toString());
   }
  }
 }
}
