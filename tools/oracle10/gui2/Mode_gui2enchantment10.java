package net.minecraft.src;
import java.io.*;import java.util.*;
public final class Mode_gui2enchantment10 {
    public static void main(String[] a)throws Exception {
        Block.stone.getClass();
        GuiEnchantment g=(GuiEnchantment)Oracle.allocate(GuiEnchantment.class);
        ContainerEnchantment c=(ContainerEnchantment)Oracle.allocate(ContainerEnchantment.class);c.inventorySlots=new ArrayList();c.field_40235_c=new int[3];
        InventoryBasic inventory=new InventoryBasic("fixture",1);c.inventorySlots.add(new Slot(inventory,0,25,47));g.inventorySlots=c;
        Gui2Fixture10.field(GuiEnchantment.class,"field_40228_y").set(g,c);
        Random animation=new Random(10203),inputs=new Random(20103);Gui2Fixture10.field(GuiEnchantment.class,"field_40230_x").set(g,animation);
        try(DataOutputStream o=Gui2Fixture10.out(a[0])){
            ItemStack stack=null;
            for(int t=0;t<10000;t++){
                int op=inputs.nextInt(5);
                if(op==0)stack=null;
                else if(op==1)stack=new ItemStack(276,1,inputs.nextInt(20));
                else if(op==2 && stack!=null)stack=stack.copy();
                else if(op==3 && stack!=null)stack.setItemDamage(inputs.nextInt(20));
                else if(op==4 && stack!=null)stack.addEnchantment(Enchantment.sharpness,inputs.nextInt(5)+1);
                inventory.setInventorySlotContents(0,stack);
                for(int i=0;i<3;i++)c.field_40235_c[i]=inputs.nextInt(4)==0?0:inputs.nextInt(50)-10;
                g.func_40219_x_();Gui2Fixture10.i(o,g.field_40227_h);
                for(float f:new float[]{g.field_40229_i,g.field_40225_j,g.field_40226_k,g.field_40223_l,g.field_40224_m,g.field_40221_n})Gui2Fixture10.f(o,f);
            }
            Gui2Fixture10.l(o,animation.nextLong());
            for(int n=0;n<1000;n++) {long seed=inputs.nextLong();EnchantmentNameParts.field_40253_a.func_40250_a(seed);for(int i=0;i<3;i++)Gui2Fixture10.s(o,EnchantmentNameParts.field_40253_a.func_40249_a());}
        }
    }
}
