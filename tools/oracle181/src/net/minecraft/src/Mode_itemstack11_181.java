package net.minecraft.src;
import java.io.*;import java.util.*;
/** Every original ItemStack method; virtual Items observe the same mutable alias. */
public final class Mode_itemstack11_181 {
 static ItemStack alias,other;static int scene,fail,absent;static ArrayList<long[]> events=new ArrayList<long[]>();
 static void event(int op,int serial,long...args){long[] row=new long[7+args.length];row[0]=op;row[1]=serial;row[2]=alias.itemID;row[3]=alias.stackSize;row[4]=alias.getItemDamage();row[5]=alias.animationsToGo;row[6]=absent;System.arraycopy(args,0,row,7,args.length);events.add(row);if(op==fail)throw new IllegalStateException("callback");}
 static void mutate(){switch(scene){case 1:alias.itemID=30701;break;case 2:alias.itemID=30702;break;case 3:alias.itemID=-1;break;case 4:alias.itemID=32000;break;case 5:alias.stackSize+=7;alias.setItemDamage(alias.getItemDamage()+3);alias.animationsToGo+=11;break;}}
 static void call(int op,int serial,long...args){event(op,serial,args);mutate();}
 static final class Probe extends Item {
  int serial;Probe(int id,int serial){super(id-256);this.serial=serial;}
  public int getIconFromDamage(int d){call(0,serial,d);return d*17+serial;}
  public boolean onItemUse(ItemStack s,EntityPlayer p,World w,int x,int y,int z,int side){call(1,serial,x,y,z,side,s==alias?1:0,p==null?0:1,w==null?0:1);return scene!=6;}
  public float getStrVsBlock(ItemStack s,Block b){call(2,serial,s==alias?1:0,b==null?0:1);return serial==1?-1.25F:3.5F;}
  ItemStack returned(){return scene==7?null:scene==8?new ItemStack(1,-17,33):scene==9?other:alias;}
  public ItemStack onItemRightClick(ItemStack s,World w,EntityPlayer p){call(3,serial,s==alias?1:0,w==null?0:1,p==null?0:1);return returned();}
  public ItemStack func_35413_b(ItemStack s,World w,EntityPlayer p){call(4,serial,s==alias?1:0,w==null?0:1,p==null?0:1);return returned();}
  public int getItemStackLimit(){call(5,serial);return serial==1?64:1;}
  public int getMaxDamage(){call(6,serial);return serial==1?5:0;}
  public boolean getHasSubtypes(){call(7,serial);return serial==1;}
  public boolean hitEntity(ItemStack s,EntityLiving target,EntityLiving player){call(8,serial,s==alias?1:0,target==null?0:1,player==null?0:1);return scene!=6;}
  public boolean onBlockDestroyed(ItemStack s,int b,int x,int y,int z,EntityLiving p){call(9,serial,b,x,y,z,s==alias?1:0,p==null?0:1);return scene!=6;}
  public int getDamageVsEntity(Entity target){call(10,serial,target==null?0:1);return serial*19;}
  public boolean canHarvestBlock(Block b){call(11,serial,b==null?0:1);return serial==1;}
  public void saddleEntity(ItemStack s,EntityLiving target){call(12,serial,s==alias?1:0,target==null?0:1);}
  public String getItemNameIS(ItemStack s){call(13,serial,s==alias?1:0);return scene==7?null:"stack."+serial;}
  public String getItemName(){call(14,serial);return scene==7?null:"plain."+serial;}
  public void onUpdate(ItemStack s,World w,Entity e,int slot,boolean selected){call(15,serial,slot,selected?1:0,s==alias?1:0,w==null?0:1,e==null?0:1);}
  public void onCreated(ItemStack s,World w,EntityPlayer p){call(16,serial,s==alias?1:0,w==null?0:1,p==null?0:1);}
  public int func_35411_c(ItemStack s){call(17,serial,s==alias?1:0);return serial*72000;}
  public EnumAction func_35412_b(ItemStack s){call(18,serial,s==alias?1:0);return scene==7?null:serial==1?EnumAction.bow:EnumAction.eat;}
  public void func_35414_a(ItemStack s,World w,EntityPlayer p,int remain){call(19,serial,remain,s==alias?1:0,w==null?0:1,p==null?0:1);}
 }
 static final class Player extends EntityPlayer {
  Player(World w){super(w);}public void func_6420_o(){}
  public void addStat(StatBase stat,int amount){event(20,stat==null?-1:stat.statId,amount);if(scene==10)alias.itemID=30701;if(scene==11)throw new IllegalStateException("stat");}
 }
 static sun.misc.Unsafe unsafe()throws Exception{java.lang.reflect.Field f=sun.misc.Unsafe.class.getDeclaredField("theUnsafe");f.setAccessible(true);return(sun.misc.Unsafe)f.get(null);}
 static void state(DataOutputStream o,ItemStack s)throws Exception{EntityOracle.i(o,s.itemID);EntityOracle.i(o,s.stackSize);EntityOracle.i(o,s.getItemDamage());EntityOracle.i(o,s.animationsToGo);}
 static void nullableText(DataOutputStream o,String s)throws Exception{o.writeBoolean(s!=null);if(s!=null)WorldEventOracle.text(o,s);}
 static void value(DataOutputStream o,Object v)throws Exception {
  if(v==null){o.writeByte(0);}else if(v instanceof Boolean){o.writeByte(1);o.writeBoolean((Boolean)v);}else if(v instanceof Integer){o.writeByte(2);EntityOracle.i(o,(Integer)v);}else if(v instanceof Float){o.writeByte(3);EntityOracle.f(o,(Float)v);}else if(v instanceof String){o.writeByte(4);nullableText(o,(String)v);}else if(v instanceof ItemStack){o.writeByte(5);o.writeBoolean(true);EntityOracle.i(o,v==alias?1:v==other?2:3);state(o,(ItemStack)v);}else if(v instanceof EnumAction){o.writeByte(6);EntityOracle.i(o,((EnumAction)v).ordinal());}
 }
 public static void main(String[] a)throws Exception {
  int init=Block.stone.blockID;new Probe(30700,1);new Probe(30701,2);Item.itemsList[30702]=null;
  for(int id:new int[]{30700,30701}){StatList.objectUseStats[id]=new StatBasic(700000+id,"use");StatList.objectBreakStats[id]=new StatBasic(800000+id,"break");StatList.objectCraftStats[id]=new StatBasic(900000+id,"craft");}
  sun.misc.Unsafe u=unsafe();Player player=(Player)u.allocateInstance(Player.class);EntityLiving target=(EntityLiving)u.allocateInstance(EntityPig.class);World world=(World)u.allocateInstance(World.class);
  try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[3])))) {
   for(int op=0;op<26;op++)for(int scenario=0;scenario<12;scenario++)for(int missing=0;missing<4;missing++)for(int failure:new int[]{-1,0,1,3,5,6,8,9,15,16,20})for(int config=0;config<5;config++) {
    scene=scenario;fail=failure;absent=missing;events.clear();alias=new ItemStack(config==3?30702:30700,(config==1||config==4)?Integer.MIN_VALUE:config==2?0:2,config==1?Integer.MAX_VALUE:config==2?-1:6);alias.animationsToGo=config==2?-1:3;other=new ItemStack(4,9,2);
    EntityPlayer p=missing==1?null:player;World w=missing==2?null:world;EntityLiving t=missing==3?null:target;Block b=missing==3?null:Block.stone;
    Object result=null;String error="";try{switch(op){
     case 0:result=alias.getIconIndex();break;case 1:result=alias.useItem(p,w,Integer.MAX_VALUE,-17,Integer.MIN_VALUE,6);break;case 2:result=alias.getStrVsBlock(b);break;
     case 3:result=alias.useItemRightClick(w,p);break;case 4:result=alias.func_35863_b(w,p);break;case 5:result=alias.getMaxStackSize();break;case 6:result=alias.isStackable();break;
     case 7:result=alias.isItemStackDamageable();break;case 8:result=alias.getHasSubtypes();break;case 9:result=alias.isItemDamaged();break;case 10:result=alias.getMaxDamage();break;
     case 11:alias.damageItem(config==1?Integer.MAX_VALUE:1,missing==1?null:missing==3?target:player);break;case 12:alias.hitEntity(t,p);break;
     case 13:alias.onDestroyBlock(-17,Integer.MAX_VALUE,-17,Integer.MIN_VALUE,p);break;case 14:result=alias.getDamageVsEntity(t);break;case 15:result=alias.canHarvestBlock(b);break;
     case 16:alias.useItemOnEntity(t);break;case 17:result=alias.getItemName();break;case 18:result=alias.toString();break;
     case 19:alias.updateAnimation(w,t,-17,true);break;case 20:alias.onCrafting(w,p);break;case 21:result=alias.func_35866_m();break;case 22:result=alias.func_35865_n();break;
     case 23:alias.func_35862_a(w,p,Integer.MIN_VALUE);break;case 24:alias.onItemDestroyedByUse(p);break;case 25:Item item=alias.getItem();result=item==null?-1:item==Item.itemsList[30700]?1:2;break;
    }}catch(RuntimeException e){error=e.getClass().getSimpleName();}
    WorldEventOracle.text(o,error);value(o,result);state(o,alias);EntityOracle.i(o,events.size());for(long[] row:events){EntityOracle.i(o,row.length);for(long v:row)EntityOracle.l(o,v);}
   }
  }
 }
}
