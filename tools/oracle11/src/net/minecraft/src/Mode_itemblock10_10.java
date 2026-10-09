package net.minecraft.src;
import java.io.*;import java.util.*;
/** Original ItemBlock with virtual dependencies; no World/block bytecode replaced. */
public final class Mode_itemblock10_10 {
 static ArrayList<long[]> events;static int fail,scene;static Probe first,second;static int slot;
 static void event(long... v){events.add(v);if(v[0]==fail)throw new IllegalStateException("callback");}
 static final class Sound extends StepSound {
  final int serial;Sound(int s){super("probe"+s,s==1?.25F:1.5F,s==1?1.25F:-.75F);serial=s;}
  public String stepSoundDir2(){event(6,serial);return "step.probe"+serial;}
  public float getVolume(){event(7,serial);return super.getVolume();}
  public float getPitch(){event(8,serial);return super.getPitch();}
 }
 static final class Probe extends Block {
  final int serial;Probe(int id,int s){super(id,1,Material.rock);serial=s;stepSound=new Sound(s);}
  public int getBlockTextureFromSide(int side){event(10,serial,side);return serial*1009+side*31;}
  public String getBlockName(){event(11,serial);return serial==1?"tile.first":null;}
  public void onBlockPlaced(World w,int x,int y,int z,int side){event(4,serial,x,y,z,side);if(scene==4){Block.blocksList[253]=second;slot=2;}if(scene==5){Block.blocksList[253]=null;slot=0;}}
  public void onBlockPlacedBy(World w,int x,int y,int z,EntityLiving p){event(5,serial,x,y,z);}
 }
 static final class ProbeItem extends ItemBlock {
  ProbeItem(){super(-3);}
  public int getMetadata(int d){event(12,d);if(scene==6){Block.blocksList[253]=second;slot=2;}return d*17+3;}
 }
 static final class ProbeWorld extends World {
  int existing,calls;
  ProbeWorld(){super(new Oracle.NullSaveHandler(0),"oracle",Oracle.settings(0));}
  public int getBlockId(int x,int y,int z){event(1,x,y,z);return calls++==0?existing:scene==3?1:253;}
  public boolean canBlockBePlacedAt(int id,int x,int y,int z,boolean ignore,int side){event(2,id,x,y,z,ignore?1:0,side);return scene!=1;}
  public boolean setBlockAndMetadataWithNotify(int x,int y,int z,int id,int meta){event(3,id,x,y,z,meta);if(scene==7){Block.blocksList[253]=second;slot=2;}return scene!=2;}
  public void playSoundEffect(double x,double y,double z,String name,float volume,float pitch){event(9,Double.doubleToLongBits(x),Double.doubleToLongBits(y),Double.doubleToLongBits(z),Float.floatToIntBits(volume),Float.floatToIntBits(pitch));}
 }
 static final class Player extends EntityPlayer {
  ItemStack stack;Player(World w){super(w);}public void func_6420_o(){}
  public boolean canPlayerEdit(int x,int y,int z){event(13,x,y,z);if(scene==8)return false;if(scene==9){stack.stackSize=Integer.MIN_VALUE;stack.setItemDamage(-17);}return true;}
 }
 static sun.misc.Unsafe unsafe()throws Exception {java.lang.reflect.Field f=sun.misc.Unsafe.class.getDeclaredField("theUnsafe");f.setAccessible(true);return (sun.misc.Unsafe)f.get(null);}
 static void writeEvents(DataOutputStream o)throws Exception{EntityOracle.i(o,Trace10.namedInt("events.size()", events.size()));for(long[] call:events){EntityOracle.i(o,Trace10.namedInt("call.length", call.length));for(long v:call)EntityOracle.l(o,v);}}
 public static void main(String[] a)throws Exception {
  int init=Block.stone.blockID;events=new ArrayList<long[]>();first=new Probe(253,1);second=new Probe(254,2);sun.misc.Unsafe u=unsafe();
  try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[3])))) {
   for(int index:new int[]{Integer.MIN_VALUE,-257,-256,-3,-2,0,31743,31744,Integer.MAX_VALUE})for(int failure:new int[]{0,10}) {
    fail=failure;scene=0;events.clear();if(index>=-256&&index<=31743)Item.itemsList[256+index]=null;String error="";ItemBlock item=null;
    try{item=new ItemBlock(index);}catch(RuntimeException e){error=e.getClass().getSimpleName();}WorldEventOracle.text(o,error);Trace10.writeBoolean(o, "item!=null", item!=null);
    int id=index+256;Item registered=id<0||id>=32000?null:Item.itemsList[id];Trace10.writeBoolean(o, "registered!=null", registered!=null);if(registered!=null){EntityOracle.i(o,Trace10.namedInt("registered.shiftedIndex", registered.shiftedIndex));EntityOracle.i(o,Trace10.namedInt("registered.getIconFromDamage(0)", registered.getIconFromDamage(0)));}
    if(item!=null){EntityOracle.i(o,Trace10.namedInt("item.getBlockID()", item.getBlockID()));for(int op=0;op<2;op++){String value=null;error="";try{value=op==0?item.getItemName():item.getItemNameIS(null);}catch(RuntimeException e){error=e.getClass().getSimpleName();}WorldEventOracle.text(o,error);Trace10.writeBoolean(o, "value!=null", value!=null);if(value!=null)WorldEventOracle.text(o,value);}}
    writeEvents(o);
   }
   // Null resources, failure order, snow/vine, int wrap, callbacks replacing
   // registry objects and a distinct captured sound object.
   Block.blocksList[253]=first;events.clear();fail=0;ProbeItem virtual=new ProbeItem();ItemBlock base=new ItemBlock(-3);
   for(int existing:new int[]{0,78,106})for(int y:new int[]{64,127,Integer.MIN_VALUE,Integer.MAX_VALUE})for(int side:new int[]{-1,0,1,2,3,4,5,6})for(int scenario=0;scenario<10;scenario++)for(int absent=0;absent<4;absent++)for(int failure:new int[]{0,1,2,3,4,5,6,7,8,9,12,13})for(boolean override:new boolean[]{false,true}) {
    fail=failure;scene=scenario;events.clear();slot=1;Block.blocksList[253]=first;
    ProbeWorld w=(ProbeWorld)u.allocateInstance(ProbeWorld.class);w.existing=existing;Player p=(Player)u.allocateInstance(Player.class);ItemStack stack=new ItemStack(253,scenario==0?0:scenario==1?-1:1,Integer.MAX_VALUE);p.stack=stack;
    String error="";boolean result=false;try{result=(override?virtual:base).onItemUse(absent==1?null:stack,absent==2?null:p,absent==3?null:w,Integer.MAX_VALUE,y,Integer.MIN_VALUE,side);}catch(RuntimeException e){error=e.getClass().getSimpleName();}
    Trace10.writeBoolean(o, "result", result);WorldEventOracle.text(o,error);EntityOracle.i(o,Trace10.namedInt("stack.stackSize", stack.stackSize));EntityOracle.i(o,Trace10.namedInt("stack.getItemDamage()", stack.getItemDamage()));EntityOracle.i(o,slot);writeEvents(o);
   }
  }
 }
}
