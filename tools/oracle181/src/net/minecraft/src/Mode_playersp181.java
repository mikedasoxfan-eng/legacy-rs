package net.minecraft.src;

import java.io.*;
import java.util.Random;

/** SP action/FOV boundaries and real onUpdate with nonzero view and death. */
public final class Mode_playersp181 {
    public static void main(String[] a) throws Exception {
        World w=Mode_entities181.world();
        PlayerOracle.HeadlessPlayer p=PlayerOracle.player(w);
        p.mc.playerController=new PlayerControllerSP(p.mc);
        Random r=new Random(6181);
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            for(int c=0;c<10000;c++) {
                p.rotationYaw=(r.nextInt(200001)-100000)/317.0F;
                p.rotationPitch=(r.nextInt(200001)-100000)/319.0F;
                p.movementInput.moveStrafe=(c%3)-1;
                p.movementInput.moveForward=(c%3)-1;
                p.movementInput.jump=c%2==0;
                p.updateEntityActionState();
                for(float v:new float[]{p.field_35222_e,p.field_35223_ap,p.field_35226_aq,p.field_35225_ar,p.moveStrafing,p.moveForward})EntityOracle.f(o,v);
                o.writeBoolean(p.isJumping);
                p.func_35113_c(c%2==0);
                EntityOracle.i(o,p.field_35221_d);o.writeBoolean(p.func_35117_Q());
                p.field_35212_aW.field_35757_b=c%3==0;
                p.field_35169_bv=(r.nextInt(2001)-1000)/1000.0F;
                if(c%4==0)p.func_35165_a(new PotionEffect(1,100,c%8));
                if(c%4==1)p.func_35165_a(new PotionEffect(2,100,c%8));
                p.func_35199_b(new ItemStack(c%3==0?261:267,1,0),72000-(c%101-30));
                EntityOracle.f(o,p.func_35220_u_());
                p.func_35207_ac();
            }
            // Calls the complete original onUpdate; captures all existing state/events.
            p=PlayerOracle.player(w);p.mc.playerController=new PlayerControllerSP(p.mc);
            p.mc.gameSettings=new GameSettings();p.movementInput=new MovementInputFromOptions(p.mc.gameSettings);
            p.mc.gameSettings.keyBindForward.field_35965_e=true;
            p.setLocationAndAngles(0.5,64,0.5,0,0);
            Mode_playertick181.frame(w,p,o);
            for(int t=0;t<650;t++) {
                p.setLocationAndAngles(0.5,64,0.5,0,0);p.motionX=p.motionZ=0;
                p.rotationYaw=(t%17-8)*33.7F;p.rotationPitch=(t%11-5)*9.8F;
                if(t==0)p.func_35113_c(true);
                if(t==610)p.health=0;
                AxisAlignedBB.clearBoundingBoxPool();Vec3D.initialize();p.onUpdate();
                Mode_playertick181.frame(w,p,o);
            }
        }
    }
}
