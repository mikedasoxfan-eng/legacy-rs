package net.minecraft.src;
import java.io.*;
/** ItemStack constructors, scalar mutations/copies/equality and NBT failure order. */
public final class Mode_stackdata11_10 {
 static void state(DataOutputStream o,ItemStack s)throws Exception{Mode_itemstack11_10.state(o,s);}
 static void maybe(DataOutputStream o,ItemStack s)throws Exception{Trace10.writeBoolean(o, "s!=null", s!=null);if(s!=null)state(o,s);}
 public static void main(String[] a)throws Exception {
  int init=Block.stone.blockID;
  try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[3])))) {
   for(int kind=0;kind<8;kind++)for(int count:new int[]{Integer.MIN_VALUE,-1,0,1,Integer.MAX_VALUE})for(int damage:new int[]{Integer.MIN_VALUE,-1,0,1,Integer.MAX_VALUE}) {
    ItemStack s=null;String error="";try{switch(kind){
     case 0:s=new ItemStack(32000,count,damage);break;case 1:s=new ItemStack(Block.stone);break;case 2:s=new ItemStack(Block.stone,count);break;case 3:s=new ItemStack(Block.stone,count,damage);break;
     case 4:s=new ItemStack(Item.swordSteel);break;case 5:s=new ItemStack(Item.swordSteel,count);break;case 6:s=new ItemStack(Item.swordSteel,count,damage);break;case 7:s=new ItemStack((Item)null,count,damage);break;
    }}catch(RuntimeException e){error=e.getClass().getSimpleName();}WorldEventOracle.text(o,error);maybe(o,s);
   }
   for(int id:new int[]{-1,0,1,267,32000})for(int count:new int[]{Integer.MIN_VALUE,-1,0,1,Integer.MAX_VALUE})for(int damage:new int[]{Integer.MIN_VALUE,-1,0,1,Integer.MAX_VALUE})for(int amount:new int[]{Integer.MIN_VALUE,-1,0,1,Integer.MAX_VALUE}) {
    ItemStack s=new ItemStack(id,count,damage);s.animationsToGo=17;ItemStack alias=s;ItemStack split=s.splitStack(amount);state(o,s);state(o,split);Trace10.writeBoolean(o, "s==alias", s==alias);Trace10.writeBoolean(o, "split==s", split==s);
    ItemStack copy=s.copy();state(o,copy);Trace10.writeBoolean(o, "copy==s", copy==s);Trace10.writeBoolean(o, "ItemStack.areItemStacksEqual(s,copy)", ItemStack.areItemStacksEqual(s,copy));Trace10.writeBoolean(o, "s.isItemEqual(copy)", s.isItemEqual(copy));Trace10.writeBoolean(o, "s.isStackEqual(copy)", s.isStackEqual(copy));
    s.setItemDamage(amount);EntityOracle.i(o,Trace10.namedInt("s.getItemDamage()", s.getItemDamage()));EntityOracle.i(o,Trace10.namedInt("s.getItemDamageForDisplay()", s.getItemDamageForDisplay()));Trace10.writeBoolean(o, "ItemStack.areItemStacksEqual(s,copy)", ItemStack.areItemStacksEqual(s,copy));
    maybe(o,ItemStack.copyItemStack(null));maybe(o,ItemStack.copyItemStack(s));Trace10.writeBoolean(o, "ItemStack.areItemStacksEqual(null,null)", ItemStack.areItemStacksEqual(null,null));Trace10.writeBoolean(o, "ItemStack.areItemStacksEqual(s,null)", ItemStack.areItemStacksEqual(s,null));Trace10.writeBoolean(o, "ItemStack.areItemStacksEqual(null,s)", ItemStack.areItemStacksEqual(null,s));
    String error="";try{s.isItemEqual(null);}catch(RuntimeException e){error=e.getClass().getSimpleName();}WorldEventOracle.text(o,error);
   }
   for(int id:new int[]{Short.MIN_VALUE,-1,0,1,119,255,256,267,2257,30700,31999,32000,Short.MAX_VALUE})for(int count:new int[]{Integer.MIN_VALUE,-129,-128,-1,0,1,127,128,Integer.MAX_VALUE})for(int damage:new int[]{Integer.MIN_VALUE,-32769,-32768,-1,0,1,32767,32768,Integer.MAX_VALUE})for(int mode=0;mode<5;mode++) {
    NBTTagCompound n=new NBTTagCompound();n.setShort("id",(short)id);n.setByte("Count",(byte)count);n.setShort("Damage",(short)damage);
    if(mode==1)n=new NBTTagCompound();if(mode==2)n.setInteger("id",1);if(mode==3)n.setInteger("Count",1);if(mode==4)n.setInteger("Damage",1);
    ItemStack s=new ItemStack(4,29,33);s.animationsToGo=17;String error="";try{s.readFromNBT(n);}catch(RuntimeException e){error=e.getClass().getSimpleName();}WorldEventOracle.text(o,error);state(o,s);
    ItemStack loaded=null;error="";try{loaded=ItemStack.loadItemStackFromNBT(n);}catch(RuntimeException e){error=e.getClass().getSimpleName();}WorldEventOracle.text(o,error);maybe(o,loaded);
    NBTTagCompound dest=new NBTTagCompound();dest.setString("keep","value");dest.setInteger("Count",72);s=new ItemStack(id,count,damage);Trace10.writeBoolean(o, "s.writeToNBT(dest)==dest", s.writeToNBT(dest)==dest);EntityOracle.i(o,Trace10.namedInt("dest.getShort(\"id\")", dest.getShort("id")));EntityOracle.i(o,Trace10.namedInt("dest.getByte(\"Count\")", dest.getByte("Count")));EntityOracle.i(o,Trace10.namedInt("dest.getShort(\"Damage\")", dest.getShort("Damage")));WorldEventOracle.text(o,dest.getString("keep"));
   }
   for(int op=0;op<3;op++){String error="";try{ItemStack s=new ItemStack(1,2,3);if(op==0)s.readFromNBT(null);if(op==1)s.writeToNBT(null);if(op==2)ItemStack.loadItemStackFromNBT(null);}catch(RuntimeException e){error=e.getClass().getSimpleName();}WorldEventOracle.text(o,error);}
   // Every wrong NBT tag type at every sequential assignment.
   for(String key:new String[]{"id","Count","Damage"})for(byte type=1;type<=10;type++) {
    NBTTagCompound n=new NBTTagCompound();n.setShort("id",(short)267);n.setByte("Count",(byte)-17);n.setShort("Damage",(short)99);n.setTag(key,NBTBase.createTagOfType(type, ""));
    ItemStack s=new ItemStack(4,29,33);s.animationsToGo=17;String error="";try{s.readFromNBT(n);}catch(RuntimeException e){error=e.getClass().getSimpleName();}WorldEventOracle.text(o,error);state(o,s);
   }
   // Registered scalar getters, including null and array-index failures.
   for(int id:new int[]{Integer.MIN_VALUE,-1,0,1,267,31999,32000,Integer.MAX_VALUE})for(int damage:new int[]{-1,0,1,6})for(int op=0;op<6;op++) {
    ItemStack s=new ItemStack(id,2,damage);String error="";int result=0;try{switch(op){case 0:result=s.getMaxStackSize();break;case 1:result=s.getMaxDamage();break;case 2:result=s.isItemStackDamageable()?1:0;break;case 3:result=s.isItemDamaged()?1:0;break;case 4:result=s.isStackable()?1:0;break;case 5:result=s.getHasSubtypes()?1:0;break;}}catch(RuntimeException e){error=e.getClass().getSimpleName();}WorldEventOracle.text(o,error);EntityOracle.i(o,result);
   }
  }
 }
}
