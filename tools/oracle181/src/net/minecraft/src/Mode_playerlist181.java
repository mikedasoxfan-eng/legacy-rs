package net.minecraft.src;
import java.io.*;import java.lang.reflect.*;import java.util.*;
/** Execute original PlayerList bytecode, including duplicate insertion and table order. */
public final class Mode_playerlist181 {
 static void i(DataOutputStream out,int value)throws Exception{EntityOracle.i(out,value);}
 static void value(DataOutputStream out,Object value)throws Exception{i(out,value==null?Integer.MIN_VALUE:(Integer)value);}
 public static void main(String[] args)throws Exception {
  PlayerList list=new PlayerList();Random random=new Random(181);
  Field buckets=PlayerList.class.getDeclaredField("field_35584_a");buckets.setAccessible(true);
  try(DataOutputStream out=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(args[1])))) {
   for(int tick=0;tick<3000;tick++) {
    int op=random.nextInt(5);long key=(long)(random.nextInt(128)-64)|((long)(random.nextInt(5)-2)<<32);
    Integer v=tick%7==0?null:Integer.valueOf(random.nextInt());
    if(op<2)list.func_35577_a(key,v);else if(op==2)value(out,list.func_35574_d(key));else if(op==3)value(out,list.func_35578_a(key));else out.writeBoolean(list.func_35575_b(key));
    PlayerListEntry[] table=(PlayerListEntry[])buckets.get(list);
    i(out,list.func_35576_a());i(out,EntityOracle.intField(PlayerList.class,"field_35583_c",list));i(out,EntityOracle.intField(PlayerList.class,"field_35581_e",list));i(out,table.length);
    for(PlayerListEntry head:table){int count=0;for(PlayerListEntry entry=head;entry!=null;entry=entry.field_35833_c)count++;i(out,count);for(PlayerListEntry entry=head;entry!=null;entry=entry.field_35833_c){EntityOracle.l(out,entry.func_35830_a());value(out,entry.func_35829_b());i(out,entry.hashCode());}}
   }
  }
 }
}
