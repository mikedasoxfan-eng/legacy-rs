package net.minecraft.src;
import java.io.*;import java.lang.reflect.*;import java.util.*;
public final class EggSlice11 {
 static final class P extends EntityPlayerSP {int stat,amount;P(){super(null,null,null,0);}public void addStat(StatBase s,int a){stat=s==null?-1:s.statId;amount=a;}}
 static final class W extends World {Entity spawned;boolean accepted;W(){super((ISaveHandler)null,"",new WorldSettings(0L,0,false,false,EnumWorldType.DEFAULT));}public boolean spawnEntityInWorld(Entity e){spawned=e;return accepted;}}
 static sun.misc.Unsafe unsafe()throws Exception{Field f=sun.misc.Unsafe.class.getDeclaredField("theUnsafe");f.setAccessible(true);return(sun.misc.Unsafe)f.get(null);}
 static W world()throws Exception{W w=(W)unsafe().allocateInstance(W.class);w.rand=new Random(42);w.worldHeight=128;return w;}
 static void le(DataOutputStream out,int v)throws Exception{out.writeByte(v);out.writeByte(v>>8);out.writeByte(v>>16);out.writeByte(v>>24);}
 public static void main(String[] args)throws Exception {
  int bootstrap=Block.stone.blockID;File dir=new File(args[0]);Item egg=Item.field_44019_bC;
  PrintWriter out=new PrintWriter(new File(dir,"eggs11.txt"));
  for(int id=-1;id<=255;id++){String name=EntityList.func_44040_a(id);for(int pass:new int[]{-1,0,1,2})out.println(id+" "+pass+" "+egg.getItemStackLimit()+" "+egg.getMaxDamage()+" "+(egg.getHasSubtypes()?1:0)+" "+(egg.func_46058_c()?1:0)+" "+egg.func_46057_a(id,pass)+" "+egg.getColorFromDamage(id,pass)+" "+(name==null?"-":name));}out.close();
  out=new PrintWriter(new File(dir,"eggnames11.txt"));for(int id=-1;id<=255;id++)out.println(id+"|"+egg.getItemDisplayName(new ItemStack(383,1,id)));out.close();
  P player=(P)unsafe().allocateInstance(P.class);player.inventory=new InventoryPlayer(player);player.capabilities=new PlayerCapabilities();
  ContainerCreative creative=new ContainerCreative(player);out=new PrintWriter(new File(dir,"creative11.txt"));DataOutputStream bin=new DataOutputStream(new FileOutputStream(new File(dir,"creative11.bin")));le(bin,creative.itemList.size());for(Object value:creative.itemList){ItemStack s=(ItemStack)value;out.println(s.itemID+" "+s.stackSize+" "+s.getItemDamage());le(bin,s.itemID);le(bin,s.stackSize);le(bin,s.getItemDamage());}out.close();bin.close();
  out=new PrintWriter(new File(dir,"eggstats11.txt"));out.println(StatList.objectUseStats[383].statId+" "+(StatList.objectBreakStats[383]==null?-1:StatList.objectBreakStats[383].statId)+" "+(StatList.objectCraftStats[383]==null?-1:StatList.objectCraftStats[383].statId));out.close();
  out=new PrintWriter(new File(dir,"eggstackuse11.txt"));for(int metadata:new int[]{-1,48,50,91,200})for(int count:new int[]{0,1,3})for(int creativeMode=0;creativeMode<2;creativeMode++)for(int remote=0;remote<2;remote++){W w=world();w.multiplayerWorld=remote==1;w.accepted=true;player.capabilities.depleteBuckets=creativeMode==1;player.worldObj=w;player.stat=0;player.amount=0;ItemStack stack=new ItemStack(383,count,metadata);boolean result=stack.useItem(player,w,0,64,0,1);out.println(metadata+" "+count+" "+creativeMode+" "+remote+" "+(result?1:0)+" "+stack.stackSize+" "+player.stat+" "+player.amount);}out.close();
  out=new PrintWriter(new File(dir,"egguse11.txt"));
  for(int id:new int[]{-1,0,1,2,9,10,11,12,13,14,15,20,21,40,41,48,49,50,51,52,53,54,55,56,57,58,59,60,61,62,63,90,91,92,93,94,95,96,97,120,200,255})for(int side=0;side<6;side++)for(int x:new int[]{0,-19,16777217})for(int remote=0;remote<2;remote++)for(int mode=0;mode<2;mode++)for(int accepted=0;accepted<2;accepted++){
   W w=world();w.multiplayerWorld=remote==1;w.accepted=accepted==1;player.worldObj=w;player.capabilities.depleteBuckets=mode==1;ItemStack stack=new ItemStack(383,3,id);boolean result=egg.onItemUse(stack,player,w,x,64,x,side);Entity e=w.spawned;
   out.print(id+" "+side+" "+x+" "+remote+" "+mode+" "+accepted+" "+(result?1:0)+" "+stack.stackSize+" "+(e==null?0:1));if(e!=null)out.print(" "+EntityList.getEntityID(e)+" "+Double.doubleToRawLongBits(e.posX)+" "+Double.doubleToRawLongBits(e.posY)+" "+Double.doubleToRawLongBits(e.posZ)+" "+Float.floatToRawIntBits(e.rotationYaw)+" "+Float.floatToRawIntBits(e.rotationPitch));out.println(" "+w.rand.nextLong());
  }out.close();
  out=new PrintWriter(new File(dir,"egginvalidside11.txt"));for(int metadata:new int[]{-1,50,91})for(int side:new int[]{-1,6,Integer.MIN_VALUE,Integer.MAX_VALUE})for(int remote=0;remote<2;remote++){
   W w=world();w.multiplayerWorld=remote==1;player.worldObj=w;player.capabilities.depleteBuckets=false;ItemStack stack=new ItemStack(383,3,metadata);String error="-";boolean result=false;try{result=stack.useItem(player,w,0,64,0,side);}catch(RuntimeException ex){error=ex.getClass().getSimpleName();}out.println(metadata+" "+side+" "+remote+" "+error+" "+(result?1:0)+" "+stack.stackSize+" "+(w.spawned==null?0:1)+" "+w.rand.nextLong());
  }out.close();
 }
}
