package net.minecraft.src;

import java.io.*;
import java.util.Random;
import java.lang.reflect.Field;

/** Direct original player callbacks and inventory/NBT boundary methods; no copied policy. */
public final class Mode_playerstate10 {
    static void inv(DataOutputStream o, InventoryPlayer inv) throws Exception {
        for(ItemStack s:inv.mainInventory)PlayerOracle.stack(o,s);
        for(ItemStack s:inv.armorInventory)PlayerOracle.stack(o,s);
        PlayerOracle.stack(o,inv.getItemStack());
        EntityOracle.i(o,Trace10.namedInt("inv.currentItem", inv.currentItem));Trace10.writeBoolean(o, "inv.inventoryChanged", inv.inventoryChanged);
        NBTTagList tags=inv.writeToNBT(new NBTTagList());
        ByteArrayOutputStream b=new ByteArrayOutputStream();PlayerOracle.canonical(tags,new DataOutputStream(b));
        EntityOracle.i(o,Trace10.namedInt("b.size()", b.size()));Trace10.writeBytes(o, "b.toByteArray()", b.toByteArray());
    }
    static void tag(NBTTagList tags,int slot,int id,int count,int damage) {
        NBTTagCompound n=new NBTTagCompound();n.setByte("Slot",(byte)slot);n.setShort("id",(short)id);
        n.setByte("Count",(byte)count);n.setShort("Damage",(short)damage);tags.setTag(n);
    }
    public static void main(String[] args)throws Exception {
        World w=Mode_entities10.world();PlayerOracle.HeadlessPlayer p=PlayerOracle.player(w);
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(args[1])))) {
            // Exhaustive vanilla item-id space, every unsigned slot and signed stack boundaries.
            InventoryPlayer v=p.inventory;v.currentItem=8;v.inventoryChanged=true;v.setItemStack(new ItemStack(35,3,14));
            int[] counts={-128,-1,0,1,64,127,128,255};int[] damages={-32768,-1,0,32767,32768,65535};
            for(int c=0;c<4516;c++) {
                NBTTagList tags=new NBTTagList();
                tag(tags,c%256,c%2258,counts[c%8],damages[c%6]);
                tag(tags,0,260,c%127,0);tag(tags,0,c%2==0?400:261,1,0);
                tag(tags,100,310,1,c%400);tag(tags,103,313,-1,-1);
                v.readFromNBT(tags);v.openChest();v.closeChest();inv(o,v);
                for(int id:new int[]{260,261,262,310,313,400,c%2258})Trace10.writeBoolean(o, "v.hasItem(id)", v.hasItem(id));
            }
            // First match outside the hotbar masks a later hotbar match; count is irrelevant.
            for(int slot=0;slot<36;slot++)for(int count:new int[]{-1,0,1}) {
                v.mainInventory=new ItemStack[36];v.armorInventory=new ItemStack[4];v.currentItem=5;
                v.mainInventory[slot]=new ItemStack(262,count,0);v.armorInventory[0]=new ItemStack(260,1,0);
                Trace10.writeBoolean(o, "v.hasItem(262)", v.hasItem(262));Trace10.writeBoolean(o, "v.hasItem(260)", v.hasItem(260));
                v.setCurrentItem(262,0,false,false);EntityOracle.i(o,Trace10.namedInt("v.currentItem", v.currentItem));
            }
            // Original setHealth is deliberately distinct from attackEntityFrom (no stats or armor).
            Random r=new Random(5181);
            for(int c=0;c<6000;c++) {
                p.health=r.nextInt(80)-20;p.prevHealth=r.nextInt(80)-20;p.heartsLife=r.nextInt(40);
                p.naturalArmorRating=r.nextInt(20);p.hurtTime=r.nextInt(20);p.maxHurtTime=r.nextInt(20);
                if(c%2==0)PlayerControllerCreative.enableAbilities(p);else PlayerControllerCreative.disableAbilities(p);
                p.inventory.armorInventory[0]=new ItemStack(313,1,0);p.setItemInUse(new ItemStack(267,1,0),72000);
                p.setHealth(r.nextInt(80)-20);
                for(int n:new int[]{Trace10.namedInt("p.health", p.health),Trace10.namedInt("p.prevHealth", p.prevHealth),Trace10.namedInt("p.heartsLife", p.heartsLife),Trace10.namedInt("p.naturalArmorRating", p.naturalArmorRating),Trace10.namedInt("p.hurtTime", p.hurtTime),Trace10.namedInt("p.maxHurtTime", p.maxHurtTime)})EntityOracle.i(o,n);
                NBTTagCompound food=new NBTTagCompound();p.getFoodStats().writeStatsToNBT(food);EntityOracle.f(o,Trace10.namedFloat("food.getFloat(\"foodExhaustionLevel\")", food.getFloat("foodExhaustionLevel")));
                PlayerOracle.stack(o,p.inventory.armorInventory[0]);
                int points=r.nextInt(),total=r.nextInt(),level=r.nextInt();p.setXPStats(points,total,level);
                for(int n:new int[]{Trace10.namedInt("Float.floatToIntBits(p.currentXP)", Float.floatToIntBits(p.currentXP)),Trace10.namedInt("p.totalXP", p.totalXP),Trace10.namedInt("p.playerLevel", p.playerLevel),Trace10.namedInt("p.xpBarCap()", p.xpBarCap())})EntityOracle.i(o,n);
            }
            p.mc.ingameGUI=(GuiIngame)Oracle.allocate(GuiIngame.class);
            p.mc.renderGlobal=(RenderGlobal)Oracle.allocate(RenderGlobal.class);
            PlayerControllerSP controller=new PlayerControllerSP(p.mc);
            for(int c=0;c<10000;c++) {
                float current=(r.nextInt(2001)-1000)/317.0F,previous=(r.nextInt(2001)-1000)/319.0F,partial=(r.nextInt(2001)-1000)/321.0F;
                for(String name:new String[]{"curBlockDamage","prevBlockDamage"}) {Field f=PlayerControllerSP.class.getDeclaredField(name);f.setAccessible(true);f.setFloat(controller,name.equals("curBlockDamage")?current:previous);}
                controller.setPartialTime(partial);EntityOracle.f(o,Trace10.namedFloat("p.mc.ingameGUI.damageGuiPartialTime", p.mc.ingameGUI.damageGuiPartialTime));EntityOracle.f(o,Trace10.namedFloat("p.mc.renderGlobal.damagePartialTime", p.mc.renderGlobal.damagePartialTime));
            }
        }
    }
}
