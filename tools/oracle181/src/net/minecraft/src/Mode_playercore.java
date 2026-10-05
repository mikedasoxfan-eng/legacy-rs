package net.minecraft.src;
import java.io.*;
import java.util.Random;
/** Calls the original FoodStats and EntityPlayer XP code, with isolated health callbacks. */
public final class Mode_playercore {
    static final class Player extends EntityPlayer {
        int action;
        Player(World w) { super(w); }
        public void func_6420_o() {}
        public void heal(int n) { action=1; health=Math.min(health+n,20); }
        public boolean attackEntityFrom(DamageSource s,int n) { action=2; health-=n; return true; }
    }
    static void foodState(DataOutputStream out,FoodStats food,Player p)throws Exception {
        NBTTagCompound n=new NBTTagCompound();food.func_35763_b(n);
        out.writeInt(food.func_35765_a());out.writeInt(food.func_35769_b());
        out.writeInt(n.getInteger("foodTickTimer"));out.writeFloat(n.getFloat("foodSaturationLevel"));
        out.writeFloat(n.getFloat("foodExhaustionLevel"));out.writeInt(p.health);out.writeInt(p.action);out.writeBoolean(food.func_35770_c());
    }
    public static void main(String[] args)throws Exception {
        World w=(World)Oracle.allocate(World.class);
        Player p=(Player)Oracle.allocate(Player.class);p.worldObj=w;
        try(DataOutputStream out=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(args[1])))) {
            for(int difficulty=0;difficulty<4;difficulty++)for(int health:new int[]{0,1,10,11,19,20})for(int level:new int[]{0,1,17,18,20})for(float saturation:new float[]{0,0.5F,5}) {
                FoodStats food=new FoodStats();w.difficultySetting=difficulty;p.health=health;
                NBTTagCompound n=new NBTTagCompound();n.setInteger("foodLevel",level);n.setFloat("foodSaturationLevel",saturation);n.setFloat("foodExhaustionLevel",40);food.func_35766_a(n);
                for(int tick=0;tick<200;tick++){p.action=0;food.func_35768_a(p);foodState(out,food,p);}
            }
            Random random=new Random(181);FoodStats food=new FoodStats();p.health=11;w.difficultySetting=2;
            for(int tick=0;tick<10000;tick++) {
                if(tick%7==0)food.func_35771_a(random.nextInt(8)-2,random.nextFloat());
                if(tick%3==0)food.func_35762_a(random.nextFloat()*10);
                if(tick%113==0){NBTTagCompound n=new NBTTagCompound();food.func_35763_b(n);FoodStats copy=new FoodStats();copy.func_35766_a(n);food=copy;}
                if(tick%97==0)food.func_35766_a(new NBTTagCompound());
                p.action=0;food.func_35768_a(p);foodState(out,food,p);
            }
            p.field_35211_aX=p.field_35210_aY=p.field_35209_aZ=0;
            for(int tick=0;tick<10000;tick++) {
                p.func_35204_c(random.nextInt(200)-3);
                out.writeInt(p.field_35211_aX);out.writeInt(p.field_35210_aY);out.writeInt(p.field_35209_aZ);out.writeInt(p.func_35193_as());
            }
            PlayerCapabilities capabilities=new PlayerCapabilities();
            out.writeBoolean(capabilities.field_35759_a);out.writeBoolean(capabilities.field_35757_b);out.writeBoolean(capabilities.field_35758_c);out.writeBoolean(capabilities.field_35756_d);
            p.field_35212_aW=capabilities;PlayerControllerTest.func_35646_d(p);
            out.writeBoolean(capabilities.field_35759_a);out.writeBoolean(capabilities.field_35757_b);out.writeBoolean(capabilities.field_35758_c);out.writeBoolean(capabilities.field_35756_d);
            capabilities.field_35757_b=true;PlayerControllerTest.func_35645_e(p);
            out.writeBoolean(capabilities.field_35759_a);out.writeBoolean(capabilities.field_35757_b);out.writeBoolean(capabilities.field_35758_c);out.writeBoolean(capabilities.field_35756_d);
            for(long seed:new long[]{0,181,-1,Long.MIN_VALUE,Long.MAX_VALUE})for(int mode:new int[]{0,1,2})for(boolean features:new boolean[]{false,true}) {
                WorldSettings settings=new WorldSettings(seed,mode,features);out.writeLong(settings.func_35518_a());out.writeInt(settings.func_35519_b());out.writeBoolean(settings.func_35520_c());
            }
        }
    }
}
