package net.minecraft.src;
import java.io.*;import java.util.*;
/** Calls the original ItemPotion display-name and tooltip methods; no game rewrites. */
public final class PotionNames11 {
 static void text(DataOutputStream o,String s)throws Exception{o.writeInt(s.length());for(int i=0;i<s.length();i++)o.writeChar(s.charAt(i));}
 public static void main(String[] args)throws Exception{
  Block.stone.getClass();Item.potion.getClass();
  ArrayList<String> codes=new ArrayList<String>(StringTranslate.getInstance().func_44022_b().keySet());codes.add("en_US");
  try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(args[0])))){
   o.writeInt(0x504e3131);o.writeInt(codes.size());
   for(String code:codes){StringTranslate.getInstance().func_44023_a(code);text(o,code);
    // Export the active table so Rust calls the same installed resource translations.
    TreeSet<String> keys=new TreeSet<String>();keys.add("item.emptyPotion.name");keys.add("potion.prefix.grenade");keys.add("potion.empty");
    for(int id=0;id<65536;id++)keys.add(PotionHelper.func_40359_b(id));
    for(Potion p:Potion.potionTypes)if(p!=null){keys.add(p.getName());keys.add(p.getName()+".postfix");}
    for(int a=1;a<8;a++)keys.add("potion.potency."+a);
    o.writeInt(keys.size());for(String k:keys){text(o,k);text(o,StatCollector.translateToLocal(k));}
    text(o,Item.potion.getItemName());text(o,StringTranslate.getInstance().translateNamedKey(Item.potion.getItemName()));
    int count=code.equals("en_US")?65536:516;o.writeInt(count);
    for(int i=0;i<count;i++){int damage=code.equals("en_US")?i:(i<512?(i*127)&65535:new int[]{-1,Integer.MIN_VALUE,Integer.MAX_VALUE,16384}[i-512]);
     ItemStack s=new ItemStack(Item.potion,1,damage);o.writeInt(damage);text(o,Item.potion.getItemDisplayName(s));
     ArrayList<String> lines=new ArrayList<String>();Item.potion.addInformation(s,lines);o.writeInt(lines.size());for(String line:lines)text(o,line);
    }
   }
  }
 }
}
