package net.minecraft.src;
import java.io.*;
import java.lang.reflect.*;
import java.util.*;

/** Release Item base/subclass getters: potion-recipe strings, enchantability, rarity, glint,
 * isItemTool, use action/duration, tooltip lines (identity translation), and the visual
 * getters of ItemColored/ItemMetadata/ItemLilyPad/ItemDye/ItemSlab/ItemCloth/ItemLeaves/ItemPotion/ItemRecord. */
public final class Mode_wbb4items {
    static final int[] DAMAGES={-1,0,1,2,3,15,16,8193,8194,16385,16396,32767};
    static final int[] VISUAL={17,18,31,35,44,98,106,110,111,112,113,114,115,116,117,118,119,120,121,122,351,372,373,374,375,376,377,378,379,380,381,382,2256,2257,2258,2266};
    static void text(DataOutputStream o,String s)throws IOException {Blocks4Oracle.bool(o,"s!=null",s!=null);if(s!=null)WorldEventOracle.text(o,s);}
    public static void main(String[] a)throws Exception {
        int init=Block.stone.blockID;
        Field table=StringTranslate.class.getDeclaredField("translateTable");table.setAccessible(true);table.set(StringTranslate.getInstance(),new Properties());
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            for(int id=0;id<2267;id++) {
                Item item=Item.itemsList[id];Blocks4Oracle.bool(o,"item!=null",item!=null);if(item==null)continue;
                Blocks4Oracle.bool(o,"isValidBrewingIngredient",item.isValidBrewingIngredient());text(o,item.getPotionModifier());Blocks4Oracle.i(o,item.getItemEnchantability());
                for(int damage:DAMAGES)for(boolean ench:new boolean[]{false,true}) {
                    ItemStack s=new ItemStack(id,1,damage);if(ench&&item.getItemStackLimit()!=1)continue;if(ench)s.addEnchantment(Enchantment.enchantmentsList[32],2);
                    Blocks4Oracle.i(o,s.func_40707_s().ordinal());Blocks4Oracle.bool(o,"glint",s.func_40713_r());Blocks4Oracle.bool(o,"enchantable",s.isItemEnchantable());
                    Blocks4Oracle.i(o,item.getItemUseAction(s).ordinal());Blocks4Oracle.i(o,item.getMaxItemUseDuration(s));
                    List lines=s.getItemNameandInformation();Blocks4Oracle.i(o,lines.size());
                    for(int n=1;n<lines.size();n++)WorldEventOracle.text(o,(String)lines.get(n));
                    if(id==373)WorldEventOracle.text(o,(String)lines.get(0));
                }
            }
            for(int id:VISUAL)for(int damage=-2;damage<20;damage++) {
                Item item=Item.itemsList[id];ItemStack s=new ItemStack(id,1,damage);
                Blocks4Oracle.i(o,item.getIconFromDamage(damage));Blocks4Oracle.i(o,item.getColorFromDamage(damage, 0));
                if(item instanceof ItemBlock)Blocks4Oracle.i(o,item.getMetadata(damage));
                if(id==31||id==35||id==44||id==351)text(o,item.getItemNameIS(s));
            }
        }
    }
}
