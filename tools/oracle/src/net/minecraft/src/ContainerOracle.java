package net.minecraft.src;

import java.io.*;
import java.lang.reflect.*;
import java.util.*;

/** Container/recipe ground truth: invokes original classes, never recompiles them. */
public class ContainerOracle {
    static void seed(Object o, Class type, String name, long seed) throws Exception {
        Field f = type.getDeclaredField(name); f.setAccessible(true); ((Random)f.get(o)).setSeed(seed);
    }
    static long hash(byte[] data) {
        long n = 0xcbf29ce484222325L; for (byte b : data) { n ^= b & 255; n *= 0x100000001b3L; } return n;
    }
    public static void tiles(int ticks, String path) throws Exception {
        World w = Oracle.newWorld(0); w.chunkProvider = new EntityOracle.FlatProvider(w);
        Field ids = Entity.class.getDeclaredField("nextEntityID"); ids.setAccessible(true); ids.setInt(null,0);
        OracleHooks.reset(100); EntityOracle.mathRandom().setSeed(200); w.rand.setSeed(300); Item.itemRand.setSeed(600); w.field_9437_g=0;
        Field sound = World.class.getDeclaredField("soundCounter"); sound.setAccessible(true); sound.setInt(w,0);
        seed(Block.stoneOvenIdle,BlockFurnace.class,"furnaceRand",700);
        seed(Block.stoneOvenActive,BlockFurnace.class,"furnaceRand",701);
        seed(Block.dispenser,BlockDispenser.class,"random",702);
        int[] inputs = {15,14,56,12,4,319,349,81,17,16,1,15};
        int[] fuels = {263,280,5,327,6,17,54,53,85,58,264,263};
        for(int i=0;i<inputs.length;i++) {
            w.setBlockWithNotify(-12+i*2,64,8,61);
            TileEntityFurnace f=(TileEntityFurnace)w.getBlockTileEntity(-12+i*2,64,8);
            f.setInventorySlotContents(0,new ItemStack(inputs[i],12,i%3));
            f.setInventorySlotContents(1,new ItemStack(fuels[i],4,i%2));
            if(i==11)f.setInventorySlotContents(2,new ItemStack(265,64,0));
        }
        w.setBlockWithNotify(8,64,-8,23);w.setBlockMetadataWithNotify(8,64,-8,3);
        TileEntityDispenser d=(TileEntityDispenser)w.getBlockTileEntity(8,64,-8);
        seed(d,TileEntityDispenser.class,"dispenserRandom",900);
        int[] items={262,344,332,35,263,264,327,5,351};
        for(int i=0;i<9;i++)d.setInventorySlotContents(i,new ItemStack(items[i],6,items[i]==35?14:0));
        Method dispense=BlockDispenser.class.getDeclaredMethod("dispenseItem",World.class,int.class,int.class,int.class,Random.class);dispense.setAccessible(true);
        try(DataOutputStream out=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(path)))) {
            for(int t=0;t<=ticks;t++) {
                if(t>0) {
                    if(t<400&&t%5==0) {w.setBlockMetadataWithNotify(8,64,-8,2+t/5%4);dispense.invoke(Block.dispenser,w,8,64,-8,w.rand);}
                    if(t==450)w.setBlockWithNotify(-12,64,8,0);
                    if(t==500)w.setBlockWithNotify(8,64,-8,0);
                    if(t==600){TileEntityFurnace f=(TileEntityFurnace)w.getBlockTileEntity(-6,64,8);f.furnaceBurnTime=1;f.setInventorySlotContents(1,null);}
                    AxisAlignedBB.clearBoundingBoxPool();Vec3D.initialize();w.updateEntities();
                }
                EntityOracle.snapshot(w,out);
                for(int i=0;i<inputs.length;i++) {
                    TileEntity te=w.getBlockTileEntity(-12+i*2,64,8);
                    TileEntityFurnace f=te instanceof TileEntityFurnace?(TileEntityFurnace)te:null;
                    out.writeBoolean(f!=null);
                    if(f!=null){for(int j=0;j<3;j++)stack(out,f.getStackInSlot(j));out.writeInt(f.furnaceBurnTime);out.writeInt(f.currentItemBurnTime);out.writeInt(f.furnaceCookTime);}
                }
                out.writeLong(hash(Oracle.worldBytes(w,0,0,0).buf.toByteArray()));
            }
            Oracle.worldBytes(w,0,0,0).writeTo(out);
        }
    }
    static Container make(EntityPlayer p, int kind) {
        switch (kind) {
            case 0: return new ContainerPlayer(p.inventory);
            case 1: return new ContainerWorkbench(p.inventory, p.worldObj, 0, 64, 0);
            case 2: return new ContainerChest(p.inventory, new TileEntityChest());
            case 3: return new ContainerChest(p.inventory, new InventoryLargeChest("Large chest", new TileEntityChest(), new TileEntityChest()));
            case 4: return new ContainerFurnace(p.inventory, new TileEntityFurnace());
            default: return new ContainerDispenser(p.inventory, new TileEntityDispenser());
        }
    }
    static ItemStack readStack(String[] a, int offset) {
        int id = Integer.parseInt(a[offset]);
        return id < 0 ? null : new ItemStack(id, Integer.parseInt(a[offset+1]), Integer.parseInt(a[offset+2]));
    }
    public static void replay(String input, String path) throws Exception {
        World w = Oracle.newWorld(0); w.chunkProvider = new EntityOracle.FlatProvider(w);
        EntityPlayer p = PlayerOracle.player(w);
        Container c = make(p, 0);
        DataOutputStream out = new DataOutputStream(new BufferedOutputStream(new FileOutputStream(path)));
        BufferedReader in = new BufferedReader(new FileReader(input));
        String line;
        while ((line = in.readLine()) != null) {
            if (line.trim().isEmpty() || line.startsWith("#")) continue;
            String[] a = line.split(" ");
            ItemStack result = null;
            if (a[0].equals("open")) { p.inventory = new InventoryPlayer(p); c = make(p, Integer.parseInt(a[1])); }
            else if (a[0].equals("put")) c.putStackInSlot(Integer.parseInt(a[1]), readStack(a, 2));
            else if (a[0].equals("cursor")) p.inventory.setItemStack(readStack(a, 1));
            else if (a[0].equals("click")) result = c.func_27280_a(Integer.parseInt(a[1]), Integer.parseInt(a[2]), a[3].equals("1"), p);
            else if (a[0].equals("close")) c.onCraftGuiClosed(p);
            else throw new IllegalArgumentException(line);
            stack(out, result);
            out.writeInt(c.slots.size());
            for (Object o : c.slots) stack(out, ((Slot)o).getStack());
            stack(out, p.inventory.getItemStack());
            for (ItemStack s : p.inventory.armorInventory) stack(out, s);
            for (ItemStack s : p.inventory.mainInventory) stack(out, s);
        }
        in.close(); out.close();
    }
    static Object field(Object o, String name) throws Exception {
        Field f = o.getClass().getDeclaredField(name); f.setAccessible(true); return f.get(o);
    }
    static void stack(DataOutputStream out, ItemStack s) throws Exception {
        out.writeBoolean(s != null);
        if (s != null) { out.writeInt(s.itemID); out.writeInt(s.stackSize); out.writeInt(s.getItemDamage()); }
    }
    public static void recipes(String path) throws Exception {
        DataOutputStream out = new DataOutputStream(new FileOutputStream(path));
        List recipes = CraftingManager.getInstance().getRecipeList();
        out.writeInt(recipes.size());
        for (Object r : recipes) {
            boolean shaped = r instanceof ShapedRecipes;
            out.writeBoolean(shaped);
            out.writeInt(shaped ? (Integer)field(r, "recipeWidth") : 0);
            out.writeInt(shaped ? (Integer)field(r, "recipeHeight") : 0);
            stack(out, ((IRecipe)r).getRecipeOutput());
            List items = shaped ? Arrays.asList((ItemStack[])field(r, "recipeItems")) : (List)field(r, "recipeItems");
            out.writeInt(items.size());
            for (Object s : items) stack(out, (ItemStack)s);
        }
        for (int id = 0; id < 2258; id++) {
            stack(out, FurnaceRecipes.smelting().getSmeltingResult(id));
            Item i = Item.itemsList[id];
            out.writeInt(i != null && i.hasContainerItem() ? i.getContainerItem().shiftedIndex : -1);
            out.writeInt(i instanceof ItemArmor ? ((ItemArmor)i).armorType : -1);
            out.writeBoolean(id < 256 && Block.blocksList[id] != null && Block.blocksList[id].blockMaterial == Material.wood);
        }
        out.close();
        PrintWriter names = new PrintWriter(path + ".names", "UTF-8");
        for(int id=0;id<2258;id++)if(Item.itemsList[id]!=null) {
            for(int damage=0;damage<16;damage++) {
                try { names.println(id+"\t"+damage+"\t"+new ItemStack(id,1,damage).getItemName()); }
                catch(ArrayIndexOutOfBoundsException e) {} // Vanilla invalid slab/dye subtype.
            }
        }
        names.close();
    }
}
