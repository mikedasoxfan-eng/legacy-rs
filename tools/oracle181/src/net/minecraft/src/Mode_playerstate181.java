package net.minecraft.src;

import java.io.*;
import java.util.Random;
import java.lang.reflect.Field;

/** Direct original player callbacks and inventory/NBT boundary methods; no copied policy. */
public final class Mode_playerstate181 {
    static void inv(DataOutputStream o, InventoryPlayer inv) throws Exception {
        for(ItemStack s:inv.mainInventory)PlayerOracle.stack(o,s);
        for(ItemStack s:inv.armorInventory)PlayerOracle.stack(o,s);
        PlayerOracle.stack(o,inv.getItemStack());
        EntityOracle.i(o,inv.currentItem);o.writeBoolean(inv.inventoryChanged);
        NBTTagList tags=inv.writeToNBT(new NBTTagList());
        ByteArrayOutputStream b=new ByteArrayOutputStream();PlayerOracle.canonical(tags,new DataOutputStream(b));
        EntityOracle.i(o,b.size());o.write(b.toByteArray());
    }
    static void tag(NBTTagList tags,int slot,int id,int count,int damage) {
        NBTTagCompound n=new NBTTagCompound();n.setByte("Slot",(byte)slot);n.setShort("id",(short)id);
        n.setByte("Count",(byte)count);n.setShort("Damage",(short)damage);tags.setTag(n);
    }
    public static void main(String[] args)throws Exception {
        World w=Mode_entities181.world();PlayerOracle.HeadlessPlayer p=PlayerOracle.player(w);
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(args[1])))) {
            // Exhaustive vanilla item-id space, every unsigned slot and signed stack boundaries.
            InventoryPlayer v=p.inventory;v.currentItem=8;v.inventoryChanged=true;v.setItemStack(new ItemStack(35,3,14));
            int[] counts={-128,-1,0,1,64,127,128,255};int[] damages={-32768,-1,0,32767,32768,65535};
            for(int c=0;c<4516;c++) {
                NBTTagList tags=new NBTTagList();
                tag(tags,c%256,c%2258,counts[c%8],damages[c%6]);
                tag(tags,0,260,c%127,0);tag(tags,0,c%2==0?400:261,1,0);
                tag(tags,100,310,1,c%400);tag(tags,103,313,-1,-1);
                v.readFromNBT(tags);v.func_35142_x_();v.func_35141_y_();inv(o,v);
                for(int id:new int[]{260,261,262,310,313,400,c%2258})o.writeBoolean(v.func_35157_d(id));
            }
            // First match outside the hotbar masks a later hotbar match; count is irrelevant.
            for(int slot=0;slot<36;slot++)for(int count:new int[]{-1,0,1}) {
                v.mainInventory=new ItemStack[36];v.armorInventory=new ItemStack[4];v.currentItem=5;
                v.mainInventory[slot]=new ItemStack(262,count,0);v.armorInventory[0]=new ItemStack(260,1,0);
                o.writeBoolean(v.func_35157_d(262));o.writeBoolean(v.func_35157_d(260));
                v.setCurrentItem(262,false);EntityOracle.i(o,v.currentItem);
            }
            // Original setHealth is deliberately distinct from attackEntityFrom (no stats or armor).
            Random r=new Random(5181);
            for(int c=0;c<6000;c++) {
                p.health=r.nextInt(80)-20;p.prevHealth=r.nextInt(80)-20;p.heartsLife=r.nextInt(40);
                p.field_9346_af=r.nextInt(20);p.hurtTime=r.nextInt(20);p.maxHurtTime=r.nextInt(20);
                if(c%2==0)PlayerControllerTest.func_35646_d(p);else PlayerControllerTest.func_35645_e(p);
                p.inventory.armorInventory[0]=new ItemStack(313,1,0);p.func_35199_b(new ItemStack(267,1,0),72000);
                p.setHealth(r.nextInt(80)-20);
                for(int n:new int[]{p.health,p.prevHealth,p.heartsLife,p.field_9346_af,p.hurtTime,p.maxHurtTime})EntityOracle.i(o,n);
                NBTTagCompound food=new NBTTagCompound();p.func_35191_at().func_35763_b(food);EntityOracle.f(o,food.getFloat("foodExhaustionLevel"));
                PlayerOracle.stack(o,p.inventory.armorInventory[0]);
                int points=r.nextInt(),total=r.nextInt(),level=r.nextInt();p.func_35219_c(points,total,level);
                for(int n:new int[]{p.field_35211_aX,p.field_35209_aZ,p.field_35210_aY,p.func_35193_as()})EntityOracle.i(o,n);
            }
            p.mc.ingameGUI=(GuiIngame)Oracle.allocate(GuiIngame.class);
            p.mc.renderGlobal=(RenderGlobal)Oracle.allocate(RenderGlobal.class);
            PlayerControllerSP controller=new PlayerControllerSP(p.mc);
            for(int c=0;c<10000;c++) {
                float current=(r.nextInt(2001)-1000)/317.0F,previous=(r.nextInt(2001)-1000)/319.0F,partial=(r.nextInt(2001)-1000)/321.0F;
                for(String name:new String[]{"curBlockDamage","prevBlockDamage"}) {Field f=PlayerControllerSP.class.getDeclaredField(name);f.setAccessible(true);f.setFloat(controller,name.equals("curBlockDamage")?current:previous);}
                controller.setPartialTime(partial);EntityOracle.f(o,p.mc.ingameGUI.damageGuiPartialTime);EntityOracle.f(o,p.mc.renderGlobal.damagePartialTime);
            }
        }
    }
}
