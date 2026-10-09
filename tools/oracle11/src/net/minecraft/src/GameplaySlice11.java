package net.minecraft.src;

import java.io.*;
import java.lang.reflect.*;
import java.util.*;

/** Focused 1.1 regression vectors produced by invoking original class methods. */
public final class GameplaySlice11 {
    static final class HooksPlayer extends EntityPlayerSP {
        int dropped;
        float exhaustion;
        HooksPlayer() {super(null,null,new Session("Player",""),0);}
        public void addStat(StatBase stat,int count) {}
        public void dropPlayerItem(ItemStack stack) {dropped+=stack.stackSize;}
        public void addExhaustion(float amount) {exhaustion+=amount;}
        void damageOnce(int amount) {super.damageEntity(DamageSource.cactus,amount);}
    }
    static sun.misc.Unsafe unsafe() throws Exception {
        Field f = sun.misc.Unsafe.class.getDeclaredField("theUnsafe");
        f.setAccessible(true); return (sun.misc.Unsafe)f.get(null);
    }
    static void stack(DataOutputStream out, ItemStack s) throws IOException {
        out.writeBoolean(s != null);
        if(s != null) {out.writeInt(s.itemID);out.writeInt(s.stackSize);out.writeInt(s.getItemDamage());}
    }
    static Object field(Object value, String name) throws Exception {
        Field f=value.getClass().getDeclaredField(name);f.setAccessible(true);return f.get(value);
    }
    static void recipes(File output) throws Exception {
        DataOutputStream out=new DataOutputStream(new FileOutputStream(output));
        List values=CraftingManager.getInstance().getRecipeList();out.writeInt(values.size());
        for(Object r:values) {
            boolean shaped=r instanceof ShapedRecipes;out.writeBoolean(shaped);
            out.writeInt(shaped?(Integer)field(r,"recipeWidth"):0);
            out.writeInt(shaped?(Integer)field(r,"recipeHeight"):0);
            stack(out,((IRecipe)r).getRecipeOutput());
            List ingredients=shaped?Arrays.asList((ItemStack[])field(r,"recipeItems")):(List)field(r,"recipeItems");
            out.writeInt(ingredients.size());for(Object s:ingredients)stack(out,(ItemStack)s);
        }
        for(int id=0;id<2258;id++) {
            stack(out,FurnaceRecipes.smelting().getSmeltingResult(id));Item item=Item.itemsList[id];
            out.writeInt(item!=null&&item.hasContainerItem()?item.getContainerItem().shiftedIndex:-1);
            out.writeInt(item instanceof ItemArmor?((ItemArmor)item).armorType:-1);
            out.writeBoolean(id<256&&Block.blocksList[id]!=null&&Block.blocksList[id].blockMaterial==Material.wood);
        }
        out.close();
    }
    public static void main(String[] args) throws Exception {
        File directory=new File(args[0]);directory.mkdirs();
        int bootstrap=Block.stone.blockID;
        // Unsafe avoids introducing world/GUI construction into a pure XP check.
        EntityPlayer player=(EntityPlayer)unsafe().allocateInstance(EntityPlayerSP.class);
        PrintWriter xp=new PrintWriter(new File(directory,"xp11.txt"));
        int[][] cases={{0,0,0,14},{0,0,0,100},{5,100,0,70},{30,2147483640,2147483640,20},
            {2,20,2147483647,1},{0,-1,7,3},{8,200,100,-7},{2,20,10,0}};
        float[] fractions={0F,0.25F,0.99999994F};
        for(int[] c:cases)for(float fraction:fractions) {
            player.playerLevel=c[0];player.totalXP=c[1];player.score=c[2];player.currentXP=fraction;
            player.increaseXP(c[3]);
            xp.println(c[0]+" "+c[1]+" "+c[2]+" "+Float.floatToRawIntBits(fraction)+" "+c[3]+" "+
                player.playerLevel+" "+player.totalXP+" "+player.score+" "+Float.floatToRawIntBits(player.currentXP));
        }
        xp.close();
        PrintWriter offers=new PrintWriter(new File(directory,"offers11.txt"));
        for(long seed:new long[]{0L,1L,42L,-1L,3257840388504953787L})
            for(int shelves:new int[]{0,1,15,29,30,31,100})for(int id:new int[]{1,267,278,298}) {
                Random random=new Random(seed);
                for(int slot=0;slot<3;slot++) {
                    int result=EnchantmentHelper.calcItemStackEnchantability(random,slot,shelves,new ItemStack(id,1,0));
                    // nextLong checks how many draws the real helper consumed.
                    long tail=random.nextLong();
                    offers.println(seed+" "+shelves+" "+id+" "+slot+" "+result+" "+tail);
                }
            }
        offers.close();
        PrintWriter containers=new PrintWriter(new File(directory,"remainders11.txt"));
        for(int full=0;full<=1;full++)for(int count=1;count<=2;count++)for(int creative=0;creative<=1;creative++) {
            HooksPlayer p=(HooksPlayer)unsafe().allocateInstance(HooksPlayer.class);
            p.inventory=new InventoryPlayer(p);
            p.capabilities=new PlayerCapabilities();
            p.capabilities.depleteBuckets=creative==1;
            if(full==1)for(int i=0;i<36;i++)p.inventory.mainInventory[i]=new ItemStack(4,64,0);
            ContainerWorkbench c=new ContainerWorkbench(p.inventory,null,0,64,0);
            c.craftMatrix.setInventorySlotContents(0,new ItemStack(335,count,0));
            ((Slot)c.inventorySlots.get(0)).onPickupFromSlot(new ItemStack(354,1,0));
            int buckets=0;for(ItemStack s:p.inventory.mainInventory)if(s!=null&&s.itemID==325)buckets+=s.stackSize;
            ItemStack matrix=c.craftMatrix.getStackInSlot(0);
            containers.println(full+" "+count+" "+creative+" "+buckets+" "+(matrix==null?-1:matrix.itemID)+" "+
                (matrix==null?0:matrix.stackSize)+" "+p.dropped);
        }
        containers.close();
        PrintWriter tags=new PrintWriter(new File(directory,"tags11.txt"));
        for(int left=0;left<=2;left++)for(int right=0;right<=2;right++) {
            ItemStack a=new ItemStack(4,1,0),b=new ItemStack(4,1,0);
            if(left>0){NBTTagCompound n=new NBTTagCompound();n.setInteger("mark",left);a.setTagCompound(n);}
            if(right>0){NBTTagCompound n=new NBTTagCompound();n.setInteger("mark",right);b.setTagCompound(n);}
            HooksPlayer p=(HooksPlayer)unsafe().allocateInstance(HooksPlayer.class);
            p.inventory=new InventoryPlayer(p);p.capabilities=new PlayerCapabilities();
            boolean equal=ItemStack.func_46154_a(a,b);
            p.inventory.addItemStackToInventory(a);p.inventory.addItemStackToInventory(b);
            int filled=0;for(ItemStack s:p.inventory.mainInventory)if(s!=null)filled++;
            tags.println(left+" "+right+" "+(equal?1:0)+" "+filled+" "+p.inventory.mainInventory[0].stackSize);
        }
        tags.close();
        PrintWriter damage=new PrintWriter(new File(directory,"damage11.txt"));
        for(int base:new int[]{0,298,306,310}) {
            HooksPlayer p=(HooksPlayer)unsafe().allocateInstance(HooksPlayer.class);
            p.inventory=new InventoryPlayer(p);p.capabilities=new PlayerCapabilities();p.health=20;
            Field potions=EntityLiving.class.getDeclaredField("activePotionsMap");potions.setAccessible(true);potions.set(p,new HashMap());
            if(base!=0)for(int i=0;i<4;i++)p.inventory.armorInventory[i]=new ItemStack(base+3-i,1,0);
            for(int amount:new int[]{1,2,3,7,20}) {
                p.damageOnce(amount);
                damage.print(base+" "+amount+" "+p.health+" "+p.carryoverDamage+" "+Float.floatToRawIntBits(p.exhaustion));
                for(ItemStack armor:p.inventory.armorInventory)damage.print(" "+(armor==null?-1:armor.getItemDamage()));
                damage.println();
            }
        }
        damage.close();
        recipes(new File(directory,"recipes11.bin"));
        System.out.println("Generated XP, enchanting and recipe vectors from original 1.1 bytecode");
    }
}
