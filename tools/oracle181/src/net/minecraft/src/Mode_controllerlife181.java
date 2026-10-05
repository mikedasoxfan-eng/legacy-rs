package net.minecraft.src;
import java.io.*;
import java.util.Random;
import java.lang.reflect.Field;
import net.minecraft.client.Minecraft;

/** Original controller factory, flip, no-op lifecycle, reset and partial-time boundaries. */
public final class Mode_controllerlife181 {
    static void f(DataOutputStream o,float n)throws Exception{EntityOracle.f(o,n);}
    static void state(EntityPlayer p,DataOutputStream o)throws Exception {
        for(double n:new double[]{p.posX,p.posY,p.posZ,p.prevPosX,p.prevPosY,p.prevPosZ,p.boundingBox.minX,p.boundingBox.minY,p.boundingBox.minZ,p.boundingBox.maxX,p.boundingBox.maxY,p.boundingBox.maxZ})EntityOracle.d(o,n);
        for(float n:new float[]{p.rotationYaw,p.rotationPitch,p.prevRotationYaw,p.prevRotationPitch,p.yOffset,p.width,p.height})f(o,n);
        for(int n:new int[]{p.dimension,p.health,p.fireResistance,p.inventory.currentItem})EntityOracle.i(o,n);
        WorldEventOracle.text(o,p.username);
        o.writeBoolean(p instanceof EntityPlayerSP);o.writeBoolean(p.craftingInventory==p.inventorySlots);
        for(ItemStack s:p.inventory.mainInventory)PlayerOracle.stack(o,s);
    }
    public static void main(String[] a)throws Exception {
        World w=Mode_entities181.world();PlayerOracle.HeadlessPlayer owner=PlayerOracle.player(w);Minecraft mc=owner.mc;
        mc.ingameGUI=(GuiIngame)Oracle.allocate(GuiIngame.class);mc.renderGlobal=(RenderGlobal)Oracle.allocate(RenderGlobal.class);
        Random r=new Random(7181);
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            for(int c=0;c<1000;c++) {
                boolean creative=c%2==0;PlayerController controller=creative?new PlayerControllerTest(mc):new PlayerControllerSP(mc);
                int x=r.nextInt(600001)-300000,y=r.nextInt(256)-64,z=r.nextInt(600001)-300000;
                w.worldInfo.setSpawn(x,y,z);w.worldProvider.worldType=new int[]{0,-1,1}[c%3];
                mc.session=new Session(new String[]{"Player","","Notch","J\u00f6rg"}[c%4],"local");
                EntityPlayer p=controller.createPlayer(w);state(p,o);o.writeBoolean(p.worldObj==w);
                p.rotationYaw=(r.nextInt(200001)-100000)/317f;p.rotationPitch=(r.nextInt(200001)-100000)/319f;
                p.prevRotationYaw=123.25f;p.prevRotationPitch=-47.75f;
                controller.flipPlayer(p);controller.func_717_a(w);controller.func_35637_a(null,-1);controller.func_35639_a(null);
                f(o,p.rotationYaw);f(o,p.rotationPitch);f(o,p.prevRotationYaw);f(o,p.prevRotationPitch);
            }
            PlayerControllerSP controller=new PlayerControllerSP(mc);
            Field damage=PlayerControllerSP.class.getDeclaredField("curBlockDamage"),previous=PlayerControllerSP.class.getDeclaredField("prevBlockDamage"),sound=PlayerControllerSP.class.getDeclaredField("field_1069_h"),wait=PlayerControllerSP.class.getDeclaredField("blockHitWait");
            for(Field field:new Field[]{damage,previous,sound,wait})field.setAccessible(true);
            float[] bounds={-Float.MAX_VALUE,-1f,-0f,0f,Float.MIN_VALUE,1f,Float.MAX_VALUE,Float.NEGATIVE_INFINITY,Float.POSITIVE_INFINITY,Float.NaN};
            for(int c=0;c<20000;c++) {
                float d=c%3==0?bounds[c%bounds.length]:(r.nextInt(200001)-100000)/317f;
                float p=c%5==0?bounds[(c/5)%bounds.length]:(r.nextInt(200001)-100000)/319f;
                float t=c%7==0?bounds[(c/7)%bounds.length]:(r.nextInt(200001)-100000)/331f;
                damage.setFloat(controller,d);previous.setFloat(controller,p);sound.setFloat(controller,17.25f);wait.setInt(controller,-13);
                mc.ingameGUI.damageGuiPartialTime=123.25f;mc.renderGlobal.damagePartialTime=-47.75f;
                controller.setPartialTime(t);f(o,mc.ingameGUI.damageGuiPartialTime);f(o,mc.renderGlobal.damagePartialTime);
                controller.resetBlockRemoving();
                f(o,damage.getFloat(controller));f(o,previous.getFloat(controller));f(o,sound.getFloat(controller));EntityOracle.i(o,wait.getInt(controller));
            }
            // Direct air removal still fails in SP after the base false return.
            mc.thePlayer=owner;EntityOracle.rawBlock(w,4,64,4,0,0);
            boolean failed=false;try{controller.sendBlockRemoved(4,64,4,1);}catch(NullPointerException expected){failed=true;}
            o.writeBoolean(failed);
            for(int c=0;c<256;c++) {
                int id=new int[]{267,260,261,1}[c%4];
                ItemStack item=new ItemStack(id,c%9-3,c%17);
                owner.inventory.mainInventory[0]=item;
                if(c%5!=0)owner.func_35199_b(item,new int[]{-1,0,32,72000}[c%4]);
                controller.func_35638_c(owner);
                PlayerOracle.stack(o,owner.func_35195_X());EntityOracle.i(o,owner.func_35205_Y());
                o.writeBoolean(owner.func_35196_Z());o.writeBoolean(owner.func_35162_ad());PlayerOracle.stack(o,owner.inventory.mainInventory[0]);
            }
        }
    }
}
