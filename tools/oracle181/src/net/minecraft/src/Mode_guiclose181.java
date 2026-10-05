package net.minecraft.src;
import java.io.*;
import java.util.*;
import net.minecraft.client.Minecraft;

/** Actual GuiContainer.onGuiClosed and actual container callbacks, including vanilla double drops. */
public final class Mode_guiclose181 {
    static final class Player extends EntityPlayerSP {
        final ArrayList<ItemStack> drops=new ArrayList<ItemStack>();
        Player(Minecraft mc,World w){super(mc,w,new Session("Player",""),0);}
        public void dropPlayerItem(ItemStack s){drops.add(s.copy());}
        public void addStat(StatBase s,int amount){}
    }
    static final class Inventory extends InventoryBasic {
        int closes;
        Inventory(){super("Chest",27);}
        public void func_35141_y_(){closes++;}
    }
    static final class Wrapper extends Container {
        final Container inner;int calls;
        Wrapper(Container c){inner=c;inventorySlots=c.inventorySlots;windowId=c.windowId;}
        public boolean canInteractWith(EntityPlayer p){return true;}
        public void onCraftGuiClosed(EntityPlayer p){calls++;inner.onCraftGuiClosed(p);}
    }
    static final class Screen extends GuiContainer {
        Screen(Container c,Minecraft mc){super(c);this.mc=mc;}
        protected void drawGuiContainerBackgroundLayer(float t){}
    }
    public static void main(String[] a)throws Exception {
        World w=Mode_entities181.world();Minecraft mc=(Minecraft)Oracle.allocate(PlayerOracle.HeadlessMinecraft.class);
        mc.theWorld=w;mc.playerController=new PlayerControllerSP(mc);
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            for(int kind=0;kind<6;kind++)for(int c=0;c<64;c++) {
                Player p=new Player(mc,w);mc.thePlayer=p;Inventory inv=new Inventory();
                Container inner=kind==0?p.inventorySlots:kind==1?new ContainerWorkbench(p.inventory,w,0,64,0):kind==2?new ContainerChest(p.inventory,inv):kind==3?new ContainerCreative(p):kind==4?new ContainerFurnace(p.inventory,new TileEntityFurnace()):new ContainerDispenser(p.inventory,new TileEntityDispenser());
                InventoryCrafting matrix=kind==0?((ContainerPlayer)inner).craftMatrix:kind==1?((ContainerWorkbench)inner).craftMatrix:null;
                if(matrix!=null)for(int slot=0;slot<matrix.getSizeInventory();slot++)if((slot+c)%3!=0)matrix.setInventorySlotContents(slot,new ItemStack(5,c%9-3,0));
                p.inventory.setItemStack(c%4==0?null:new ItemStack(264,c%9-3,c));
                Wrapper wrapper=new Wrapper(inner);p.craftingInventory=wrapper;
                new Screen(wrapper,mc).onGuiClosed();
                EntityOracle.i(o,wrapper.calls);EntityOracle.i(o,inv.closes);o.writeBoolean(p.craftingInventory==p.inventorySlots);
                PlayerOracle.stack(o,p.inventory.getItemStack());EntityOracle.i(o,p.drops.size());for(ItemStack s:p.drops)PlayerOracle.stack(o,s);
                EntityOracle.i(o,matrix==null?0:matrix.getSizeInventory());if(matrix!=null)for(int slot=0;slot<matrix.getSizeInventory();slot++)PlayerOracle.stack(o,matrix.getStackInSlot(slot));
            }
        }
    }
}
