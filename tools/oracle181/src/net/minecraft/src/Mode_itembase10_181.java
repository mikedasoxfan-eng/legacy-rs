package net.minecraft.src;
import java.io.*;
import java.util.*;
/** Actual Item bytecode, including constructor edits and inherited defaults. */
public final class Mode_itembase10_181 {
 static final IdentityHashMap<Item,Integer> handles=new IdentityHashMap<Item,Integer>();
 static void i(DataOutputStream o,int v)throws Exception{EntityOracle.i(o,v);}
 static void text(DataOutputStream o,String s)throws Exception{WorldEventOracle.text(o,s);}
 static void nullable(DataOutputStream o,String s)throws Exception{o.writeBoolean(s!=null);if(s!=null)text(o,s);}
 static void state(DataOutputStream o,Item p)throws Exception {
  i(o,p.shiftedIndex);i(o,p.getItemStackLimit());i(o,p.getMaxDamage());o.writeBoolean(p.getHasSubtypes());o.writeBoolean(p.isDamageable());o.writeBoolean(p.isFull3D());o.writeBoolean(p.shouldRotateAroundWhenRendering());
  i(o,p.getIconFromDamage(Integer.MIN_VALUE));i(o,p.getPlacedBlockMetadata(Integer.MAX_VALUE));i(o,p.getColorFromDamage(-1));nullable(o,p.getItemName());nullable(o,p.getItemNameIS(null));
  o.writeBoolean(p.hasContainerItem());i(o,p.getContainerItem()==null?-1:handles.get(p.getContainerItem()));text(o,p.getStatName());
 }
 static final class Icon extends Item {
  int calls,damage;boolean fail;Icon(){super(31001);}
  public int getIconFromDamage(int d){calls++;damage=d;if(fail)throw new IllegalArgumentException("icon");return d*17+31;}
 }
 static final class Name extends Item {
  int calls;String value;Name(){super(31002);}
  public String getItemName(){calls++;return value;}
 }
 public static void main(String[] a)throws Exception {
  int init=Block.stone.blockID;
  try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[3])))) {
   // Isolated high slots start empty; low occupied slots are cleared only in this JVM.
   for(int index:new int[]{Integer.MIN_VALUE,-257,-256,-255,30744,30744,31743,31744,Integer.MAX_VALUE,Integer.MAX_VALUE-255}) {
    int shifted=256+index;if(shifted>=0&&shifted<32000&&index!=30744)Item.itemsList[shifted]=null;
    PrintStream saved=System.out;ByteArrayOutputStream b=new ByteArrayOutputStream();String error="";Item p=null;
    try{System.setOut(new PrintStream(b));p=new Item(index);}catch(RuntimeException e){error=e.getClass().getSimpleName();}finally{System.setOut(saved);}
    text(o,error);text(o,b.toString("UTF-8").replace("\r\n","\n"));o.writeBoolean(p!=null);
    if(p!=null){handles.put(p,handles.size());o.writeBoolean(Item.itemsList[shifted]==p);state(o,p);}
   }
   Item p=new Item(30745),container=new Item(30746);handles.put(p,handles.size());handles.put(container,handles.size());
   int[] ints={Integer.MIN_VALUE,-17,-1,0,1,2,64,32767,Integer.MAX_VALUE};
   for(int limit:ints)for(int damage:ints)for(boolean sub:new boolean[]{false,true}) {
    o.writeBoolean(p.setMaxStackSize(limit)==p);o.writeBoolean(p.setMaxDamage(damage)==p);o.writeBoolean(p.setHasSubtypes(sub)==p);
    for(Item c:new Item[]{container,null,p}) {String error="";try{o.writeBoolean(p.setContainerItem(c)==p);}catch(RuntimeException e){o.writeBoolean(false);error=e.getClass().getSimpleName();}text(o,error);state(o,p);}
   }
   for(int x:ints)for(int y:ints){o.writeBoolean(p.setIconCoord(x,y)==p);state(o,p);o.writeBoolean(p.setIconIndex(y)==p);state(o,p);}
   for(String n:new String[]{null,"","stone","a.b","A\u0000B","\u03bb"}){o.writeBoolean(p.setItemName(n)==p);state(o,p);}
   o.writeBoolean(p.setFull3D()==p);state(o,p);o.writeBoolean(p.setFull3D()==p);state(o,p);
   for(int damage:ints)for(boolean absent:new boolean[]{false,true}) {
    ItemStack s=absent?null:new ItemStack(31001,-7,damage);if(s!=null)s.animationsToGo=91;
    String error="";int icon=0;try{icon=p.getIconIndex(s);}catch(RuntimeException e){error=e.getClass().getSimpleName();}i(o,icon);text(o,error);
    o.writeBoolean(p.onItemUse(s,null,null,-1,Integer.MAX_VALUE,2,-17));EntityOracle.f(o,p.getStrVsBlock(s,null));o.writeBoolean(p.onItemRightClick(s,null,null)==s);o.writeBoolean(p.func_35413_b(s,null,null)==s);
    o.writeBoolean(p.hitEntity(s,null,null));o.writeBoolean(p.onBlockDestroyed(s,-1,2,3,4,null));i(o,p.getDamageVsEntity(null));o.writeBoolean(p.canHarvestBlock(null));
    p.saddleEntity(s,null);p.onUpdate(s,null,null,Integer.MIN_VALUE,true);p.onCreated(s,null,null);p.func_35414_a(s,null,null,-1);i(o,p.func_35412_b(s).ordinal());i(o,p.func_35411_c(s));
    if(s!=null){i(o,s.itemID);i(o,s.stackSize);i(o,s.getItemDamage());i(o,s.animationsToGo);}
   }
   Icon icon=new Icon();for(int d:ints)for(boolean absent:new boolean[]{false,true})for(boolean fail:new boolean[]{false,true}) {
    icon.calls=0;icon.damage=0;icon.fail=fail;String error="";int value=0;try{value=icon.getIconIndex(absent?null:new ItemStack(1,1,d));}catch(RuntimeException e){error=e.getClass().getSimpleName();}i(o,value);text(o,error);i(o,icon.calls);i(o,icon.damage);
   }
   Name name=new Name();for(String n:new String[]{null,"","unregistered","item.stone","A\u0000B"}){name.calls=0;name.value=n;text(o,name.getStatName());i(o,name.calls);}
   // Replacing a registry slot preserves references held by container aliases.
   Item alias=new Item(30747);handles.put(alias,handles.size());p.setMaxStackSize(1).setContainerItem(alias);Item replacement=new Item(30747);handles.put(replacement,handles.size());
   o.writeBoolean(Item.itemsList[31003]==replacement);o.writeBoolean(p.getContainerItem()==alias);alias.setItemName("old");replacement.setItemName("new");state(o,p.getContainerItem());state(o,Item.itemsList[31003]);
  }
 }
}
