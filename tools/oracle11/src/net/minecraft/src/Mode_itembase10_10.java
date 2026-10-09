package net.minecraft.src;
import java.io.*;
import java.util.*;
/** Actual Item bytecode, including constructor edits and inherited defaults. */
public final class Mode_itembase10_10 {
 static final IdentityHashMap<Item,Integer> handles=new IdentityHashMap<Item,Integer>();
 static void i(DataOutputStream o,int v)throws Exception{EntityOracle.i(o,v);}
 static void text(DataOutputStream o,String s)throws Exception{WorldEventOracle.text(o,s);}
 static void nullable(DataOutputStream o,String s)throws Exception{Trace10.writeBoolean(o, "s!=null", s!=null);if(s!=null)text(o,s);}
 static void state(DataOutputStream o,Item p)throws Exception {
  i(o,Trace10.namedInt("p.shiftedIndex", p.shiftedIndex));i(o,Trace10.namedInt("p.getItemStackLimit()", p.getItemStackLimit()));i(o,Trace10.namedInt("p.getMaxDamage()", p.getMaxDamage()));Trace10.writeBoolean(o, "p.getHasSubtypes()", p.getHasSubtypes());Trace10.writeBoolean(o, "p.isDamageable()", p.isDamageable());Trace10.writeBoolean(o, "p.isFull3D()", p.isFull3D());Trace10.writeBoolean(o, "p.shouldRotateAroundWhenRendering()", p.shouldRotateAroundWhenRendering());
  i(o,Trace10.namedInt("p.getIconFromDamage(Integer.MIN_VALUE)", p.getIconFromDamage(Integer.MIN_VALUE)));i(o,Trace10.namedInt("p.getPlacedBlockMetadata(Integer.MAX_VALUE)", p.getMetadata(Integer.MAX_VALUE)));i(o,Trace10.namedInt("p.getColorFromDamage(-1, 0)", p.getColorFromDamage(-1, 0)));nullable(o,p.getItemName());nullable(o,p.getItemNameIS(null));
  Trace10.writeBoolean(o, "p.hasContainerItem()", p.hasContainerItem());i(o,Trace10.namedInt("p.getContainerItem()==null?-1:handles.get(p.getContainerItem())", p.getContainerItem()==null?-1:handles.get(p.getContainerItem())));text(o,p.getStatName());
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
    text(o,error);text(o,b.toString("UTF-8").replace("\r\n","\n"));Trace10.writeBoolean(o, "p!=null", p!=null);
    if(p!=null){handles.put(p,handles.size());Trace10.writeBoolean(o, "Item.itemsList[shifted]==p", Item.itemsList[shifted]==p);state(o,p);}
   }
   Item p=new Item(30745),container=new Item(30746);handles.put(p,handles.size());handles.put(container,handles.size());
   int[] ints={Integer.MIN_VALUE,-17,-1,0,1,2,64,32767,Integer.MAX_VALUE};
   for(int limit:ints)for(int damage:ints)for(boolean sub:new boolean[]{false,true}) {
    Trace10.writeBoolean(o, "p.setMaxStackSize(limit)==p", p.setMaxStackSize(limit)==p);Trace10.writeBoolean(o, "p.setMaxDamage(damage)==p", p.setMaxDamage(damage)==p);Trace10.writeBoolean(o, "p.setHasSubtypes(sub)==p", p.setHasSubtypes(sub)==p);
    for(Item c:new Item[]{container,null,p}) {String error="";try{Trace10.writeBoolean(o, "p.setContainerItem(c)==p", p.setContainerItem(c)==p);}catch(RuntimeException e){Trace10.writeBoolean(o, "false", false);error=e.getClass().getSimpleName();}text(o,error);state(o,p);}
   }
   for(int x:ints)for(int y:ints){Trace10.writeBoolean(o, "p.setIconCoord(x,y)==p", p.setIconCoord(x,y)==p);state(o,p);Trace10.writeBoolean(o, "p.setIconIndex(y)==p", p.setIconIndex(y)==p);state(o,p);}
   for(String n:new String[]{null,"","stone","a.b","A\u0000B","\u03bb"}){Trace10.writeBoolean(o, "p.setItemName(n)==p", p.setItemName(n)==p);state(o,p);}
   Trace10.writeBoolean(o, "p.setFull3D()==p", p.setFull3D()==p);state(o,p);Trace10.writeBoolean(o, "p.setFull3D()==p", p.setFull3D()==p);state(o,p);
   for(int damage:ints)for(boolean absent:new boolean[]{false,true}) {
    ItemStack s=absent?null:new ItemStack(31001,-7,damage);if(s!=null)s.animationsToGo=91;
    String error="";int icon=0;try{icon=p.getIconIndex(s);}catch(RuntimeException e){error=e.getClass().getSimpleName();}i(o,icon);text(o,error);
    Trace10.writeBoolean(o, "p.onItemUse(s,null,null,-1,Integer.MAX_VALUE,2,-17)", p.onItemUse(s,null,null,-1,Integer.MAX_VALUE,2,-17));EntityOracle.f(o,Trace10.namedFloat("p.getStrVsBlock(s,null)", p.getStrVsBlock(s,null)));Trace10.writeBoolean(o, "p.onItemRightClick(s,null,null)==s", p.onItemRightClick(s,null,null)==s);Trace10.writeBoolean(o, "p.onFoodEaten(s,null,null)==s", p.onFoodEaten(s,null,null)==s);
    Trace10.writeBoolean(o, "p.hitEntity(s,null,null)", p.hitEntity(s,null,null));Trace10.writeBoolean(o, "p.onBlockDestroyed(s,-1,2,3,4,null)", p.onBlockDestroyed(s,-1,2,3,4,null));i(o,Trace10.namedInt("p.getDamageVsEntity(null)", p.getDamageVsEntity(null)));Trace10.writeBoolean(o, "p.canHarvestBlock(null)", p.canHarvestBlock(null));
    p.useItemOnEntity(s,null);p.onUpdate(s,null,null,Integer.MIN_VALUE,true);p.onCreated(s,null,null);p.onPlayerStoppedUsing(s,null,null,-1);i(o,Trace10.namedInt("p.getItemUseAction(s).ordinal()", p.getItemUseAction(s).ordinal()));i(o,Trace10.namedInt("p.getMaxItemUseDuration(s)", p.getMaxItemUseDuration(s)));
    if(s!=null){i(o,Trace10.namedInt("s.itemID", s.itemID));i(o,Trace10.namedInt("s.stackSize", s.stackSize));i(o,Trace10.namedInt("s.getItemDamage()", s.getItemDamage()));i(o,Trace10.namedInt("s.animationsToGo", s.animationsToGo));}
   }
   Icon icon=new Icon();for(int d:ints)for(boolean absent:new boolean[]{false,true})for(boolean fail:new boolean[]{false,true}) {
    icon.calls=0;icon.damage=0;icon.fail=fail;String error="";int value=0;try{value=icon.getIconIndex(absent?null:new ItemStack(1,1,d));}catch(RuntimeException e){error=e.getClass().getSimpleName();}i(o,value);text(o,error);i(o,Trace10.namedInt("icon.calls", icon.calls));i(o,Trace10.namedInt("icon.damage", icon.damage));
   }
   Name name=new Name();for(String n:new String[]{null,"","unregistered","item.stone","A\u0000B"}){name.calls=0;name.value=n;text(o,name.getStatName());i(o,Trace10.namedInt("name.calls", name.calls));}
   // Replacing a registry slot preserves references held by container aliases.
   Item alias=new Item(30747);handles.put(alias,handles.size());p.setMaxStackSize(1).setContainerItem(alias);Item replacement=new Item(30747);handles.put(replacement,handles.size());
   Trace10.writeBoolean(o, "Item.itemsList[31003]==replacement", Item.itemsList[31003]==replacement);Trace10.writeBoolean(o, "p.getContainerItem()==alias", p.getContainerItem()==alias);alias.setItemName("old");replacement.setItemName("new");state(o,p.getContainerItem());state(o,Item.itemsList[31003]);
  }
 }
}
