package net.minecraft.src;
import java.io.*;
import java.util.Random;
import java.lang.reflect.Field;

/** Original creative initialization uses the supplied player and retains existing stacks/flying. */
public final class Mode_controllermode181 {
    static final class Sounds extends SoundManager {
        int music;
        public void playRandomMusicIfReady(){music++;}
    }
    static void state(EntityPlayer p,DataOutputStream o)throws Exception {
        PlayerCapabilities b=p.field_35212_aW;
        for(boolean v:new boolean[]{b.field_35759_a,b.field_35757_b,b.field_35758_c,b.field_35756_d})o.writeBoolean(v);
        for(ItemStack s:p.inventory.mainInventory)PlayerOracle.stack(o,s);
    }
    public static void main(String[] a)throws Exception {
        World w=Mode_entities181.world();PlayerOracle.HeadlessPlayer owner=PlayerOracle.player(w);
        EntityPlayer supplied=new PlayerOracle.HeadlessPlayer(owner.mc,w);
        Sounds sounds=new Sounds();owner.mc.sndManager=sounds;
        Random r=new Random(6281);
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            for(int c=0;c<10000;c++) {
                boolean creative=c%2==0;
                PlayerController controller=creative?new PlayerControllerTest(owner.mc):new PlayerControllerSP(owner.mc);
                PlayerCapabilities b=supplied.field_35212_aW;
                b.field_35759_a=(c&1)!=0;b.field_35757_b=(c&2)!=0;b.field_35758_c=(c&4)!=0;b.field_35756_d=(c&8)!=0;
                for(int slot=0;slot<36;slot++)supplied.inventory.mainInventory[slot]=(c+slot)%4==0?null:new ItemStack(new int[]{260,261,35,278}[slot%4],r.nextInt(256)-128,r.nextInt(1000)-100);
                owner.inventory.mainInventory[0]=new ItemStack(264,91,c);
                if(!creative)PlayerControllerTest.func_35645_e(supplied);
                controller.func_6473_b(supplied);controller.func_717_a(w);
                controller.func_35637_a(new ItemStack(35,37,14),c);controller.func_35639_a(null);
                float damage=(r.nextInt(2001)-1000)/317f,previous=(r.nextInt(2001)-1000)/319f;
                if(!creative)for(String name:new String[]{"curBlockDamage","prevBlockDamage"}) {Field f=PlayerControllerSP.class.getDeclaredField(name);f.setAccessible(true);f.setFloat(controller,name.equals("curBlockDamage")?damage:previous);}
                sounds.music=0;controller.updateController();
                state(supplied,o);PlayerOracle.stack(o,owner.inventory.mainInventory[0]);
                EntityOracle.i(o,sounds.music);EntityOracle.f(o,creative?previous:EntityOracle.floatField(PlayerControllerSP.class,"prevBlockDamage",controller));
                EntityOracle.f(o,controller.getBlockReachDistance());
                for(boolean v:new boolean[]{controller.isInTestMode,controller.shouldDrawHUD(),controller.func_35643_e(),controller.func_35642_f(),controller.func_35641_g(),controller.func_35640_h(),controller.func_35636_i()})o.writeBoolean(v);
            }
        }
    }
}
