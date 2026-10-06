package net.minecraft.src;
import java.io.*;
import java.lang.reflect.*;
import java.util.*;
import net.minecraft.client.Minecraft;
import org.lwjgl.opengl.GL11;

/** Fixture inputs and callback spies; every algorithm is executed by the original 1.0 jar. */
final class Gui3Fixture10 {
    static void i(DataOutputStream o,int n)throws IOException {Gui2Fixture10.i(o,n);}
    static void f(DataOutputStream o,float n)throws IOException {Gui2Fixture10.f(o,n);}
    static void stack(DataOutputStream o,ItemStack s)throws Exception {
        o.writeBoolean(s!=null);if(s!=null){i(o,s.itemID);i(o,s.stackSize);i(o,s.getItemDamage());}
    }
    static class Player extends EntityPlayerSP {
        ArrayList<Integer> achievements;
        ArrayList<ItemStack> drops;
        Player(){super(null,null,null,0);}
        public void addStat(StatBase s,int n){if(s instanceof Achievement)achievements.add(s.statId);}
        public void dropPlayerItem(ItemStack s){drops.add(s.copy());}
    }
    static Player player(World w)throws Exception {
        Player p=(Player)Oracle.allocate(Player.class);p.worldObj=w;p.inventory=new InventoryPlayer(p);p.achievements=new ArrayList<Integer>();p.drops=new ArrayList<ItemStack>();return p;
    }
    static class ShelfWorld extends World {
        int[] blocks;
        ShelfWorld(){super(null,"",(WorldSettings)null);}
        public int getBlockId(int x,int y,int z){if(Math.abs(x)>2||y<0||y>1||Math.abs(z)>2)return 0;return blocks[(z+2)*10+y*5+x+2];}
        public boolean isAirBlock(int x,int y,int z){return getBlockId(x,y,z)==0;}
    }
    static ShelfWorld shelves(Random r)throws Exception {
        ShelfWorld w=(ShelfWorld)Oracle.allocate(ShelfWorld.class);w.blocks=new int[50];for(int n=0;n<50;n++){int v=r.nextInt(4);w.blocks[n]=v==0?1:v==1?47:0;}return w;
    }
    static void layout(DataOutputStream o,Container c)throws Exception {
        i(o,c.inventorySlots.size());for(Object value:c.inventorySlots){Slot s=(Slot)value;for(int n:new int[]{s.slotNumber,Gui2Fixture10.field(Slot.class,"slotIndex").getInt(s),s.xDisplayPosition,s.yDisplayPosition,s.getSlotStackLimit()})i(o,n);}
    }
    static void state(DataOutputStream o,Container c,Player p)throws Exception {
        for(Object value:c.inventorySlots)stack(o,((Slot)value).getStack());stack(o,p.inventory.getItemStack());
        i(o,p.achievements.size());for(int n:p.achievements)i(o,n);p.achievements.clear();
        i(o,p.drops.size());for(ItemStack s:p.drops)stack(o,s);p.drops.clear();
    }
    static void enchstate(DataOutputStream o,ContainerEnchantment c,Player p,Random rng)throws Exception {
        Gui2Fixture10.l(o,c.field_40234_b);for(int n:c.field_40235_c)i(o,n);i(o,p.playerLevel);stack(o,c.field_40236_a.getStackInSlot(0));
        ItemStack item=c.field_40236_a.getStackInSlot(0);NBTTagList es=item==null?null:item.func_40714_p();i(o,es==null?0:es.tagCount());if(es!=null)for(int n=0;n<es.tagCount();n++){NBTTagCompound e=(NBTTagCompound)es.tagAt(n);i(o,e.getShort("id"));i(o,e.getShort("lvl"));}
    }
    static void enchanting(String path)throws Exception {
        Block.stone.getClass();Random input=new Random(30031);
        try(DataOutputStream o=Gui2Fixture10.out(path)) {
            for(int n=0;n<600;n++) {
                ShelfWorld w=shelves(input);w.multiplayerWorld=n%3==0;Player p=player(w);p.playerLevel=input.nextInt(100)-10;
                ContainerEnchantment c=new ContainerEnchantment(p.inventory,w,0,0,0);Random rng=new Random(input.nextLong());Gui2Fixture10.field(ContainerEnchantment.class,"field_40237_l").set(c,rng);
                layout(o,c);int[] ids={276,278,306,283,340,1,261,359};int id=ids[input.nextInt(ids.length)];
                c.field_40236_a.setInventorySlotContents(0,new ItemStack(id,1,0));enchstate(o,c,p,rng);
                for(int offer=0;offer<3;offer++){o.writeBoolean(c.func_40233_a(p,offer));enchstate(o,c,p,rng);}
                for(int k=-1;k<4;k++)c.updateProgressBar(k,k*13-5);enchstate(o,c,p,rng);
                c.field_40236_a.setInventorySlotContents(0,null);enchstate(o,c,p,rng);Gui2Fixture10.l(o,rng.nextLong());
            }
        }
    }
    static void containers(String path)throws Exception {
        Block.stone.getClass();Random input=new Random(30032);
        try(DataOutputStream o=Gui2Fixture10.out(path)) {
            for(int kind=0;kind<2;kind++)for(int n=0;n<160;n++) {
                ShelfWorld w=shelves(input);w.multiplayerWorld=n%3==0;Player p=player(w);
                Container c=kind==0?new ContainerEnchantment(p.inventory,w,0,0,0):new ContainerBrewingStand(p.inventory,new TileEntityBrewingStand());
                if(kind==0)Gui2Fixture10.field(ContainerEnchantment.class,"field_40237_l").set(c,new Random(n));
                layout(o,c);
                int[] ids={1,5,264,373,374,353,276};
                for(int k=0;k<c.inventorySlots.size();k++)if(input.nextInt(4)!=0){int id=ids[input.nextInt(ids.length)];c.getSlot(k).putStack(new ItemStack(id,1+input.nextInt(70),input.nextInt(3)));}
                state(o,c,p);
                for(int k=0;k<c.inventorySlots.size();k++){stack(o,c.transferStackInSlot(k));state(o,c,p);}
                p.inventory.setItemStack(new ItemStack(264,3,0));c.onCraftGuiClosed(p);state(o,c,p);c.onCraftGuiClosed(p);state(o,c,p);
            }
        }
    }
    static void creative(String path)throws Exception {
        Player p=player(null);ContainerCreative c=new ContainerCreative(p);Random input=new Random(30033);
        try(DataOutputStream o=Gui2Fixture10.out(path)) {
            i(o,c.itemList.size());for(Object value:c.itemList)stack(o,(ItemStack)value);layout(o,c);
            for(int n=0;n<500;n++){float v=n==0?0:n==1?1:input.nextFloat()*3-1;c.func_35374_a(v);for(int k=0;k<72;k++)stack(o,c.getSlot(k).getStack());}
            // Container's new default enchant hook and inherited transfer fallback.
            o.writeBoolean(c.func_40233_a(p,0));stack(o,c.transferStackInSlot(0));
        }
    }
    static Object field(Object value,String name)throws Exception{return Gui2Fixture10.field(value.getClass(),name).get(value);}
    static void recipes(String path)throws Exception {
        Block.stone.getClass();CraftingManager manager=CraftingManager.getInstance();List recipes=manager.getRecipeList();
        Container owner=new Container(){public boolean canInteractWith(EntityPlayer p){return true;}};
        try(DataOutputStream o=Gui2Fixture10.out(path)) {
            i(o,recipes.size());
            for(Object value:recipes){IRecipe r=(IRecipe)value;boolean shaped=r instanceof ShapedRecipes;int w=shaped?(Integer)field(r,"recipeWidth"):0;int h=shaped?(Integer)field(r,"recipeHeight"):0;List items=shaped?Arrays.asList((ItemStack[])field(r,"recipeItems")):(List)field(r,"recipeItems");
                o.writeBoolean(shaped);i(o,w);i(o,h);stack(o,r.getRecipeOutput());i(o,items.size());for(Object s:items)stack(o,(ItemStack)s);
                for(int width=2;width<=3;width++)for(int variant=0;variant<4;variant++){
                    InventoryCrafting grid=new InventoryCrafting(owner,width,width);
                    for(int k=0;k<items.size();k++){ItemStack s=(ItemStack)items.get(k);if(s==null)continue;s=s.copy();if(s.getItemDamage()==-1)s.setItemDamage(variant);int index=shaped?((variant%2==0?k%w:w-k%w-1)+(k/w)*width):k;if(index<width*width)grid.setInventorySlotContents(index,s);}
                    stack(o,manager.findMatchingRecipe(grid));
                }
            }
            for(int id=-2;id<2260;id++)stack(o,FurnaceRecipes.smelting().getSmeltingResult(id));
            int[] ids={256,257,261,276,359,1};int[] damages={0,1,20,100,1500,Integer.MAX_VALUE,Integer.MIN_VALUE};
            for(int id:ids)for(int a:damages)for(int b:damages){InventoryCrafting grid=new InventoryCrafting(owner,3,3);grid.setInventorySlotContents(0,new ItemStack(id,1,a));grid.setInventorySlotContents(8,new ItemStack(id,1,b));stack(o,manager.findMatchingRecipe(grid));}
        }
    }
    static void slots(String path)throws Exception {
        Block.stone.getClass();
        Player p=player(null);ContainerBrewingStand b=new ContainerBrewingStand(p.inventory,new TileEntityBrewingStand());ContainerEnchantment e=new ContainerEnchantment(p.inventory,null,0,0,0);
        Slot[] slots={b.getSlot(0),b.getSlot(3),e.getSlot(0)};
        try(DataOutputStream o=Gui2Fixture10.out(path)) {
            for(Slot s:slots){i(o,s.getSlotStackLimit());o.writeBoolean(s.isItemValid(null));for(int id=0;id<2258;id++)if(Item.itemsList[id]!=null)o.writeBoolean(s.isItemValid(new ItemStack(id,1,0)));}
            for(int damage:new int[]{-1,0,1,16,16384,32767}){p.achievements.clear();b.getSlot(0).onPickupFromSlot(new ItemStack(373,1,damage));i(o,p.achievements.size());for(int id:p.achievements)i(o,id);}
            Container owner=new Container(){public boolean canInteractWith(EntityPlayer p){return true;}};InventoryCrafting grid=new InventoryCrafting(owner,3,3);SlotCrafting s=new SlotCrafting(p,grid,new InventoryCraftResult(),0,0,0);
            for(int id:new int[]{58,270,61,290,297,354,274,268,116,47,264}){p.achievements.clear();s.onPickupFromSlot(new ItemStack(id,1,0));i(o,p.achievements.size());for(int a:p.achievements)i(o,a);}
        }
    }
}

