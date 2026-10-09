package net.minecraft.src;
import java.io.*;
import java.util.Random;
import java.lang.reflect.Field;
import net.minecraft.client.Minecraft;

/** Original controller factory, flip, no-op lifecycle, reset and partial-time boundaries. */
public final class Mode_controllerlife10 {
    static void f(DataOutputStream o,float n)throws Exception{EntityOracle.f(o,n);}
    static void state(EntityPlayer p,DataOutputStream o)throws Exception {
        for(double n:new double[]{Trace10.namedDouble("p.posX", p.posX),Trace10.namedDouble("p.posY", p.posY),Trace10.namedDouble("p.posZ", p.posZ),Trace10.namedDouble("p.prevPosX", p.prevPosX),Trace10.namedDouble("p.prevPosY", p.prevPosY),Trace10.namedDouble("p.prevPosZ", p.prevPosZ),Trace10.namedDouble("p.boundingBox.minX", p.boundingBox.minX),Trace10.namedDouble("p.boundingBox.minY", p.boundingBox.minY),Trace10.namedDouble("p.boundingBox.minZ", p.boundingBox.minZ),Trace10.namedDouble("p.boundingBox.maxX", p.boundingBox.maxX),Trace10.namedDouble("p.boundingBox.maxY", p.boundingBox.maxY),Trace10.namedDouble("p.boundingBox.maxZ", p.boundingBox.maxZ)})EntityOracle.d(o,n);
        for(float n:new float[]{Trace10.namedFloat("p.rotationYaw", p.rotationYaw),Trace10.namedFloat("p.rotationPitch", p.rotationPitch),Trace10.namedFloat("p.prevRotationYaw", p.prevRotationYaw),Trace10.namedFloat("p.prevRotationPitch", p.prevRotationPitch),Trace10.namedFloat("p.yOffset", p.yOffset),Trace10.namedFloat("p.width", p.width),Trace10.namedFloat("p.height", p.height)})f(o,n);
        for(int n:new int[]{Trace10.namedInt("p.dimension", p.dimension),Trace10.namedInt("p.health", p.health),Trace10.namedInt("p.fireResistance", p.fireResistance),Trace10.namedInt("p.inventory.currentItem", p.inventory.currentItem)})EntityOracle.i(o,n);
        WorldEventOracle.text(o,p.username);
        Trace10.writeBoolean(o, "p instanceof EntityPlayerSP", p instanceof EntityPlayerSP);Trace10.writeBoolean(o, "p.craftingInventory==p.inventorySlots", p.craftingInventory==p.inventorySlots);
        for(ItemStack s:p.inventory.mainInventory)PlayerOracle.stack(o,s);
    }
    public static void main(String[] a)throws Exception {
        World w=Mode_entities10.world();PlayerOracle.HeadlessPlayer owner=PlayerOracle.player(w);Minecraft mc=owner.mc;
        mc.ingameGUI=(GuiIngame)Oracle.allocate(GuiIngame.class);mc.renderGlobal=(RenderGlobal)Oracle.allocate(RenderGlobal.class);
        Random r=new Random(7181);
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            for(int c=0;c<1000;c++) {
                boolean creative=c%2==0;PlayerController controller=creative?new PlayerControllerCreative(mc):new PlayerControllerSP(mc);
                int x=r.nextInt(600001)-300000,y=r.nextInt(256)-64,z=r.nextInt(600001)-300000;
                w.worldInfo.setSpawn(x,y,z);w.worldProvider.worldType=new int[]{0,-1,1}[c%3];
                mc.session=new Session(new String[]{"Player","","Notch","J\u00f6rg"}[c%4],"local");
                EntityPlayer p=controller.createPlayer(w);state(p,o);Trace10.writeBoolean(o, "p.worldObj==w", p.worldObj==w);
                p.rotationYaw=(r.nextInt(200001)-100000)/317f;p.rotationPitch=(r.nextInt(200001)-100000)/319f;
                p.prevRotationYaw=123.25f;p.prevRotationPitch=-47.75f;
                controller.flipPlayer(p);controller.onWorldChange(w);controller.func_35637_a(null,-1);controller.func_35639_a(null);
                f(o,Trace10.namedFloat("p.rotationYaw", p.rotationYaw));f(o,Trace10.namedFloat("p.rotationPitch", p.rotationPitch));f(o,Trace10.namedFloat("p.prevRotationYaw", p.prevRotationYaw));f(o,Trace10.namedFloat("p.prevRotationPitch", p.prevRotationPitch));
            }
            PlayerControllerSP controller=new PlayerControllerSP(mc);
            Field damage=PlayerControllerSP.class.getDeclaredField("curBlockDamage"),previous=PlayerControllerSP.class.getDeclaredField("prevBlockDamage"),sound=PlayerControllerSP.class.getDeclaredField("blockDestroySoundCounter"),wait=PlayerControllerSP.class.getDeclaredField("blockHitWait");
            for(Field field:new Field[]{damage,previous,sound,wait})field.setAccessible(true);
            float[] bounds={-Float.MAX_VALUE,-1f,-0f,0f,Float.MIN_VALUE,1f,Float.MAX_VALUE,Float.NEGATIVE_INFINITY,Float.POSITIVE_INFINITY,Float.NaN};
            for(int c=0;c<20000;c++) {
                float d=c%3==0?bounds[c%bounds.length]:(r.nextInt(200001)-100000)/317f;
                float p=c%5==0?bounds[(c/5)%bounds.length]:(r.nextInt(200001)-100000)/319f;
                float t=c%7==0?bounds[(c/7)%bounds.length]:(r.nextInt(200001)-100000)/331f;
                damage.setFloat(controller,d);previous.setFloat(controller,p);sound.setFloat(controller,17.25f);wait.setInt(controller,-13);
                mc.ingameGUI.damageGuiPartialTime=123.25f;mc.renderGlobal.damagePartialTime=-47.75f;
                controller.setPartialTime(t);f(o,Trace10.namedFloat("mc.ingameGUI.damageGuiPartialTime", mc.ingameGUI.damageGuiPartialTime));f(o,Trace10.namedFloat("mc.renderGlobal.damagePartialTime", mc.renderGlobal.damagePartialTime));
                controller.resetBlockRemoving();
                f(o,Trace10.namedFloat("damage.getFloat(controller)", damage.getFloat(controller)));f(o,Trace10.namedFloat("previous.getFloat(controller)", previous.getFloat(controller)));f(o,Trace10.namedFloat("sound.getFloat(controller)", sound.getFloat(controller)));EntityOracle.i(o,Trace10.namedInt("wait.getInt(controller)", wait.getInt(controller)));
            }
            // Direct air removal still fails in SP after the base false return.
            mc.thePlayer=owner;EntityOracle.rawBlock(w,4,64,4,0,0);
            boolean failed=false;try{controller.onPlayerDestroyBlock(4,64,4,1);}catch(NullPointerException expected){failed=true;}
            Trace10.writeBoolean(o, "failed", failed);
            for(int c=0;c<256;c++) {
                int id=new int[]{267,260,261,1}[c%4];
                ItemStack item=new ItemStack(id,c%9-3,c%17);
                owner.inventory.mainInventory[0]=item;
                if(c%5!=0)owner.setItemInUse(item,new int[]{-1,0,32,72000}[c%4]);
                controller.onStoppedUsingItem(owner);
                PlayerOracle.stack(o,owner.getItemInUse());EntityOracle.i(o,Trace10.namedInt("owner.func_35205_Y()", owner.func_35205_Y()));
                Trace10.writeBoolean(o, "owner.isUsingItem()", owner.isUsingItem());Trace10.writeBoolean(o, "owner.func_35162_ad()", owner.isBlocking());PlayerOracle.stack(o,owner.inventory.mainInventory[0]);
            }
        }
    }
}
