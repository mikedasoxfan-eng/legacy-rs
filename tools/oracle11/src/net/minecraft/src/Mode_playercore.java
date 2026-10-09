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
        NBTTagCompound n=new NBTTagCompound();food.writeStatsToNBT(n);
        Trace10.writeInt(out, "food.getFoodLevel()", food.getFoodLevel());Trace10.writeInt(out, "food.getPrevFoodLevel()", food.getPrevFoodLevel());
        Trace10.writeInt(out, "n.getInteger(\"foodTickTimer\")", n.getInteger("foodTickTimer"));Trace10.writeFloat(out, "n.getFloat(\"foodSaturationLevel\")", n.getFloat("foodSaturationLevel"));
        Trace10.writeFloat(out, "n.getFloat(\"foodExhaustionLevel\")", n.getFloat("foodExhaustionLevel"));Trace10.writeInt(out, "p.health", p.health);Trace10.writeInt(out, "p.action", p.action);Trace10.writeBoolean(out, "food.needFood()", food.needFood());
    }
    public static void main(String[] args)throws Exception {
        World w=(World)Oracle.allocate(World.class);
        Player p=(Player)Oracle.allocate(Player.class);p.worldObj=w;
        try(DataOutputStream out=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(args[1])))) {
            for(int difficulty=0;difficulty<4;difficulty++)for(int health:new int[]{0,1,10,11,19,20})for(int level:new int[]{0,1,17,18,20})for(float saturation:new float[]{0,0.5F,5}) {
                FoodStats food=new FoodStats();w.difficultySetting=difficulty;p.health=health;
                NBTTagCompound n=new NBTTagCompound();n.setInteger("foodLevel",level);n.setFloat("foodSaturationLevel",saturation);n.setFloat("foodExhaustionLevel",40);food.readStatsFromNBT(n);
                for(int tick=0;tick<200;tick++){p.action=0;food.onUpdate(p);foodState(out,food,p);}
            }
            Random random=new Random(181);FoodStats food=new FoodStats();p.health=11;w.difficultySetting=2;
            for(int tick=0;tick<10000;tick++) {
                if(tick%7==0)food.addStats(random.nextInt(8)-2,random.nextFloat());
                if(tick%3==0)food.addExhaustion(random.nextFloat()*10);
                if(tick%113==0){NBTTagCompound n=new NBTTagCompound();food.writeStatsToNBT(n);FoodStats copy=new FoodStats();copy.readStatsFromNBT(n);food=copy;}
                if(tick%97==0)food.readStatsFromNBT(new NBTTagCompound());
                p.action=0;food.onUpdate(p);foodState(out,food,p);
            }
            p.currentXP=p.playerLevel=p.totalXP=0;
            for(int tick=0;tick<10000;tick++) {
                p.increaseXP(random.nextInt(200)-3);
                Trace10.writeInt(out, "Float.floatToIntBits(p.currentXP)", Float.floatToIntBits(p.currentXP));Trace10.writeInt(out, "p.playerLevel", p.playerLevel);Trace10.writeInt(out, "p.totalXP", p.totalXP);Trace10.writeInt(out, "p.xpBarCap()", p.xpBarCap());
            }
            PlayerCapabilities capabilities=new PlayerCapabilities();
            Trace10.writeBoolean(out, "capabilities.disableDamage", capabilities.disableDamage);Trace10.writeBoolean(out, "capabilities.isFlying", capabilities.isFlying);Trace10.writeBoolean(out, "capabilities.allowFlying", capabilities.allowFlying);Trace10.writeBoolean(out, "capabilities.depleteBuckets", capabilities.depleteBuckets);
            p.capabilities=capabilities;PlayerControllerCreative.enableAbilities(p);
            Trace10.writeBoolean(out, "capabilities.disableDamage", capabilities.disableDamage);Trace10.writeBoolean(out, "capabilities.isFlying", capabilities.isFlying);Trace10.writeBoolean(out, "capabilities.allowFlying", capabilities.allowFlying);Trace10.writeBoolean(out, "capabilities.depleteBuckets", capabilities.depleteBuckets);
            capabilities.isFlying=true;PlayerControllerCreative.disableAbilities(p);
            Trace10.writeBoolean(out, "capabilities.disableDamage", capabilities.disableDamage);Trace10.writeBoolean(out, "capabilities.isFlying", capabilities.isFlying);Trace10.writeBoolean(out, "capabilities.allowFlying", capabilities.allowFlying);Trace10.writeBoolean(out, "capabilities.depleteBuckets", capabilities.depleteBuckets);
            for(long seed:new long[]{0,181,-1,Long.MIN_VALUE,Long.MAX_VALUE})for(int mode:new int[]{0,1,2})for(boolean features:new boolean[]{false,true}) {
                WorldSettings settings=new WorldSettings(seed,mode,features, false, EnumWorldType.DEFAULT);Trace10.writeLong(out, "settings.getSeed()", settings.getSeed());Trace10.writeInt(out, "settings.getGameType()", settings.getGameType());Trace10.writeBoolean(out, "settings.isMapFeaturesEnabled()", settings.isMapFeaturesEnabled());
            }
            GameSettings options=new GameSettings();MovementInputFromOptions movement=new MovementInputFromOptions(options);
            for(int mask=0;mask<64;mask++) {
                options.keyBindForward.pressed=(mask&1)!=0;options.keyBindBack.pressed=(mask&2)!=0;
                options.keyBindLeft.pressed=(mask&4)!=0;options.keyBindRight.pressed=(mask&8)!=0;
                options.keyBindJump.pressed=(mask&16)!=0;options.keyBindSneak.pressed=(mask&32)!=0;
                movement.updatePlayerMoveState(p);Trace10.writeFloat(out, "movement.moveStrafe", movement.moveStrafe);Trace10.writeFloat(out, "movement.moveForward", movement.moveForward);Trace10.writeBoolean(out, "movement.jump", movement.jump);Trace10.writeBoolean(out, "movement.sneak", movement.sneak);
            }
            Block init=Block.stone;
            EntityPlayerSP sp=(EntityPlayerSP)Oracle.allocate(PlayerOracle.HeadlessPlayer.class);sp.capabilities=new PlayerCapabilities();sp.speedOnGround=0.1F;
            for(java.lang.reflect.Field f:EntityLiving.class.getDeclaredFields())if(java.util.Map.class.isAssignableFrom(f.getType())){f.setAccessible(true);f.set(sp,new java.util.HashMap());}
            java.lang.reflect.Field used=EntityPlayer.class.getDeclaredField("itemInUse");used.setAccessible(true);
            java.lang.reflect.Field remaining=EntityPlayer.class.getDeclaredField("itemInUseCount");remaining.setAccessible(true);
            for(boolean flying:new boolean[]{false,true})for(float speed:new float[]{0.02F,0.1F,0.13F,0.2F})for(int draw:new int[]{-1,0,1,10,20,21,71999}) {
                sp.capabilities.isFlying=flying;sp.landMovementFactor=speed;
                used.set(sp,draw<0?null:new ItemStack(261,1,0));remaining.setInt(sp,72000-draw);Trace10.writeFloat(out, "sp.getFOVMultiplier()", sp.getFOVMultiplier());
            }
        }
    }
}
