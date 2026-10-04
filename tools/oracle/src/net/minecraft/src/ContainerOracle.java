package net.minecraft.src;

import java.io.*;
import java.lang.reflect.*;
import java.util.*;

/** Container/recipe ground truth: invokes original classes, never recompiles them. */
public class ContainerOracle {
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
    }
}
