package net.minecraft.src;
import java.io.*;import java.util.*;
public final class InventoryTags11 {
 static GameplaySlice11.HooksPlayer player()throws Exception{GameplaySlice11.HooksPlayer p=(GameplaySlice11.HooksPlayer)GameplaySlice11.unsafe().allocateInstance(GameplaySlice11.HooksPlayer.class);p.inventory=new InventoryPlayer(p);p.capabilities=new PlayerCapabilities();return p;}
 static ItemStack stack(int count,int mark){if(count==0)return null;ItemStack s=new ItemStack(4,count,0);if(mark!=0){NBTTagCompound n=new NBTTagCompound();n.setInteger("mark",mark);s.setTagCompound(n);}return s;}
 static void mutate(ItemStack s,int mark){if(s!=null&&s.stackTagCompound!=null)s.stackTagCompound.setInteger("mark",mark);}
 static void out(PrintWriter o,ItemStack s){o.print(s==null?" -1 0 0 0":" "+s.itemID+" "+s.stackSize+" "+s.getItemDamage()+" "+(s.stackTagCompound==null?0:s.stackTagCompound.getInteger("mark")));}
 static void snapshot(PrintWriter o,Container c,InventoryPlayer inv,ItemStack result){out(o,inv.getItemStack());out(o,result);for(Object raw:c.inventorySlots)out(o,((Slot)raw).getStack());}
 public static void main(String[] args)throws Exception{
  int boot=Block.stone.blockID;File dir=new File(args[0]);PrintWriter o=new PrintWriter(new File(dir,"containertags11.txt"));
  for(int kind=0;kind<2;kind++)for(int lt=0;lt<3;lt++)for(int rt=0;rt<3;rt++)for(int lc:new int[]{0,1,63})for(int rc:new int[]{0,1,2,64})for(int button=0;button<2;button++)for(int shift=0;shift<2;shift++){
   GameplaySlice11.HooksPlayer p=player();IInventory tile=kind==0?new InventoryBasic("Chest",9):new TileEntityFurnace();Container c=kind==0?new ContainerChest(p.inventory,tile):new ContainerFurnace(p.inventory,(TileEntityFurnace)tile);int slot=kind==0?0:2;
   ItemStack left=stack(lc,lt),held=stack(rc,rt),target=stack(rc,rt);tile.setInventorySlotContents(slot,left);p.inventory.setItemStack(held);
   if(shift!=0){for(int i=0;i<36;i++)p.inventory.mainInventory[i]=new ItemStack(1,64,0);p.inventory.mainInventory[7]=null;p.inventory.mainInventory[8]=target;}
   ItemStack result=c.slotClick(slot,button,shift!=0,p);o.print(kind+" "+lt+" "+rt+" "+lc+" "+rc+" "+button+" "+shift+" "+c.inventorySlots.size());snapshot(o,c,p.inventory,result);mutate(left,31);mutate(held,32);mutate(result,33);mutate(target,34);snapshot(o,c,p.inventory,result);o.println();
  }o.close();o=new PrintWriter(new File(dir,"inventoryalias11.txt"));
  for(int mark=0;mark<3;mark++)for(int existing=0;existing<4;existing++)for(int count:new int[]{1,2,64,65}){
   GameplaySlice11.HooksPlayer p=player();ItemStack input=stack(count,mark),old=existing==0?null:stack(63,existing);p.inventory.mainInventory[0]=old;if(existing==3)for(int i=1;i<36;i++)p.inventory.mainInventory[i]=new ItemStack(1,64,0);
   boolean ok=p.inventory.addItemStackToInventory(input);o.print(mark+" "+existing+" "+count+" "+(ok?1:0));out(o,input);for(ItemStack s:p.inventory.mainInventory)out(o,s);mutate(input,11);mutate(old,12);out(o,input);for(ItemStack s:p.inventory.mainInventory)out(o,s);for(ItemStack s:p.inventory.mainInventory)if(s!=null&&s.itemID==4)mutate(s,22);out(o,input);for(ItemStack s:p.inventory.mainInventory)out(o,s);o.println();
  }o.close();o=new PrintWriter(new File(dir,"stacknames11.txt"));
  for(int save=0;save<4;save++)for(int op=0;op<5;op++){
   ItemStack a=stack(2,1),b=stack(2,1);if((save&1)!=0)a.writeToNBT(new NBTTagCompound());if((save&2)!=0)b.writeToNBT(new NBTTagCompound());
   ItemStack c=op==0?a.copy():op==1?a.splitStack(1):op==2?ItemStack.loadItemStackFromNBT(a.writeToNBT(new NBTTagCompound())):null;GameplaySlice11.HooksPlayer p=player();if(op==3){p.inventory.addItemStackToInventory(a);c=p.inventory.mainInventory[0];}if(op==4){c=stack(2,0);c.setTagCompound(a.getTagCompound());}
   o.print(save+" "+op+" "+(ItemStack.func_46154_a(a,b)?1:0)+" "+(ItemStack.func_46154_a(a,c)?1:0));for(ItemStack v:new ItemStack[]{a,b,c})o.print(" "+(v.stackTagCompound.getKey().equals("tag")?1:0)+" "+v.stackSize);a.writeToNBT(new NBTTagCompound());o.print(" "+(ItemStack.func_46154_a(a,c)?1:0));for(ItemStack v:new ItemStack[]{a,b,c})o.print(" "+(v.stackTagCompound.getKey().equals("tag")?1:0));o.println();
  }o.close();o=new PrintWriter(new File(dir,"stacknulltags11.txt"));for(int a=-1;a<3;a++)for(int b=-1;b<3;b++){ItemStack x=a<0?null:stack(1,a),y=b<0?null:stack(1,b);o.println(a+" "+b+" "+(ItemStack.func_46154_a(x,y)?1:0)+" "+(ItemStack.areItemStacksEqual(x,y)?1:0));}o.close();
 }
}
