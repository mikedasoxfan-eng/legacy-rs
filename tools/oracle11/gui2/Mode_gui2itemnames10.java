package net.minecraft.src;
import java.io.*;import java.util.*;
public final class Mode_gui2itemnames10 {
    public static void main(String[] a)throws Exception {
        Block.stone.getClass();ArrayList<Integer> ids=new ArrayList<Integer>();ArrayList<Integer> damages=new ArrayList<Integer>();ArrayList<String> names=new ArrayList<String>();
        for(int id=0;id<Item.itemsList.length;id++)if(Item.itemsList[id]!=null)for(int damage=0;damage<16;damage++) {
            try{String key=Item.itemsList[id].getItemNameIS(new ItemStack(id,1,damage));if(key!=null){ids.add(id);damages.add(damage);names.add(key);}}catch(ArrayIndexOutOfBoundsException ignored){}
        }
        try(DataOutputStream o=Gui2Fixture10.out(a[0])){Gui2Fixture10.i(o,names.size());for(int n=0;n<names.size();n++){Gui2Fixture10.i(o,ids.get(n));Gui2Fixture10.i(o,damages.get(n));Gui2Fixture10.s(o,names.get(n));}}
    }
}
