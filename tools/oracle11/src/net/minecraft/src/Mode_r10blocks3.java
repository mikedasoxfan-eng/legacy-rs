package net.minecraft.src;

import java.io.*;
import java.lang.reflect.*;
import java.util.*;

/** Round 3 fixtures call the original release classes; no game algorithm is recompiled. */
public final class Mode_r10blocks3 {
    static final String[] EXPRESSIONS = {"", "0", "!0", "-0", "0*12", "0|1", "0 | 1", "0 & 1", "!0 & 1 & !2 & !3 & 1+6", "=0", ">2", "<3", "31+32+63+64", "2147483648", "99999999999999999", "0-1-2+3", "0!!1", "0--1", "0&&1", "0||1", "\ud83d\ude00+5", "+14&13-13", "-0+1-2-3&4-4+13"};
    static final int[] INGREDIENTS = {353,370,375,376,382,377,378,331,348,289,372};
    static void i(DataOutputStream o, int n) throws Exception {o.writeInt(n);}
    static void text(DataOutputStream o, String s) throws Exception {byte[] b=s.getBytes("UTF-8");i(o,b.length);o.write(b);}
    static void effects(DataOutputStream o,List list) throws Exception {
        i(o,list==null?-1:list.size());
        if(list!=null)for(Object obj:list){PotionEffect p=(PotionEffect)obj;i(o,p.getPotionID());i(o,p.getDuration());i(o,p.getAmplifier());}
    }
    static void potion(DataOutputStream o) throws Exception {
        Method eval=PotionHelper.class.getDeclaredMethod("func_40355_a",String.class,int.class,int.class,int.class);eval.setAccessible(true);
        for(int data=-128;data<65536;data++) {
            i(o,PotionHelper.func_40352_a(data));text(o,PotionHelper.func_40359_b(data));
            for(boolean flag:new boolean[]{false,true}) {effects(o,PotionHelper.getPotionEffects(data,flag));i(o,PotionHelper.func_40358_a(data,flag));i(o,PotionHelper.func_40358_a(data,flag));}
            for(int id:INGREDIENTS)i(o,PotionHelper.brewPotionData(data,Item.itemsList[id].getPotionModifier()));
        }
        for(int data:new int[]{Integer.MIN_VALUE,Integer.MAX_VALUE,-1,0,1,31,32,32767,65535})for(String s:EXPRESSIONS) {
            i(o,(Integer)eval.invoke(null,s,0,s.length(),data));i(o,PotionHelper.brewPotionData(data,s));
            for(int bit=-65;bit<=65;bit++)o.writeBoolean(PotionHelper.isBitOn(data,bit));
            i(o,PotionHelper.func_40351_a(data,-1,32,33,64,65));
        }
        effects(o,null);i(o,PotionHelper.func_40354_a(null));
        for(int amp=-2;amp<=4;amp++)for(int id=1;id<20;id++) {List<PotionEffect> list=Arrays.asList(new PotionEffect(id,100,amp),new PotionEffect(1,1,2));i(o,PotionHelper.func_40354_a(list));}
    }
    static void data(DataOutputStream o,Collection list) throws Exception {
        i(o,list==null?-1:list.size());if(list!=null)for(Object obj:list){EnchantmentData d=(EnchantmentData)obj;i(o,d.enchantmentobj.effectId);i(o,d.enchantmentLevel);i(o,d.itemWeight);}
    }
    static void enchantments(DataOutputStream o) throws Exception {
        World w=Mode_entities10.world();
        EntityLiving[] mobs={new EntityPig(w),new EntityZombie(w),new EntitySpider(w)};
        DamageSource[] sources={DamageSource.generic,DamageSource.inFire,DamageSource.onFire,DamageSource.lava,DamageSource.fall,DamageSource.explosion,DamageSource.outOfWorld,DamageSource.cactus,DamageSource.drown,new EntityDamageSourceIndirect("arrow",null,null).setProjectile()};
        for(int id=0;id<256;id++) {
            Enchantment e=Enchantment.enchantmentsList[id];o.writeBoolean(e!=null);if(e==null)continue;
            i(o,e.effectId);i(o,e.getWeight());i(o,e.type.ordinal());text(o,e.getName());i(o,e.getMinLevel());i(o,e.getMaxLevel());
            for(int level:new int[]{Integer.MIN_VALUE,-100,-1,0,1,2,3,4,5,100,46341,Integer.MAX_VALUE}) {
                i(o,e.getMinEnchantability(level));i(o,e.getMaxEnchantability(level));
                for(DamageSource s:sources)i(o,e.calcModifierDamage(level,s));
                for(EntityLiving m:mobs)i(o,e.calcModifierLiving(level,m));
            }
            for(Enchantment other:Enchantment.enchantmentsList)if(other!=null)o.writeBoolean(e.canApplyTogether(other));
        }
        for(EnumEnchantmentType type:EnumEnchantmentType.values())for(int id=0;id<Item.itemsList.length;id++)if(Item.itemsList[id]!=null){i(o,id);o.writeBoolean(type.canEnchantItem(Item.itemsList[id]));}
        for(EnumArmorMaterial m:EnumArmorMaterial.values()){i(o,m.ordinal());i(o,m.getEnchantability());for(int slot=0;slot<4;slot++){i(o,m.func_40576_a(slot));i(o,m.getDamageReductionAmount(slot));}}
        for(EnumRarity r:EnumRarity.values()){i(o,r.ordinal());i(o,r.field_40535_e);text(o,r.field_40532_f);}
    }
    static void helper(DataOutputStream o) throws Exception {
        for(int id=256;id<=382;id++)if(Item.itemsList[id]!=null)for(int power=0;power<=80;power++) {
            ItemStack stack=new ItemStack(id,1,0);Map map=EnchantmentHelper.mapEnchantmentData(power,stack);data(o,map==null?null:map.values());
        }
        for(long seed=0;seed<128;seed++)for(int id:new int[]{1,256,257,258,267,268,272,276,283,298,301,302,305,306,309,310,313,314,317,261,373}) {
            ItemStack stack=new ItemStack(id,1,0);
            for(int power:new int[]{0,1,5,15,30,50,100}){Random rng=new Random(seed);data(o,EnchantmentHelper.buildEnchantmentList(rng,stack,power));o.writeLong(EntityOracle.rngState(rng));}
            for(int shelves:new int[]{0,1,15,30,31,100})for(int slot=-1;slot<4;slot++){Random rng=new Random(seed);i(o,EnchantmentHelper.calcItemStackEnchantability(rng,slot,shelves,stack));o.writeLong(EntityOracle.rngState(rng));}
        }
    }
    static void modifiers(DataOutputStream o) throws Exception {
        World w=Mode_entities10.world();Mode_entities10.Player p=new Mode_entities10.Player(w);
        EntityLiving[] mobs={new EntityPig(w),new EntityZombie(w),new EntitySpider(w)};
        Field random=EnchantmentHelper.class.getDeclaredField("enchantmentRand");random.setAccessible(true);Random rng=(Random)random.get(null);
        Method lookup=EnchantmentHelper.class.getDeclaredMethod("getEnchantmentLevel",int.class,ItemStack.class);lookup.setAccessible(true);
        Method maximum=EnchantmentHelper.class.getDeclaredMethod("getMaxEnchantmentLevel",int.class,ItemStack[].class);maximum.setAccessible(true);
        DamageSource[] sources={DamageSource.generic,DamageSource.inFire,DamageSource.fall,DamageSource.explosion,DamageSource.outOfWorld,new DamageSource("projectile").setProjectile()};
        for(int level:new int[]{0,1,2,4,10,127,255,256,32767}) {
            ItemStack held=new ItemStack(276,1,0);
            for(Enchantment e:Enchantment.enchantmentsList)if(e!=null){held.addEnchantment(e,level);held.addEnchantment(e,1);}
            p.inventory.mainInventory[0]=held;
            for(int slot=0;slot<4;slot++){ItemStack armor=new ItemStack(310+slot,1,0);for(Enchantment e:Enchantment.enchantmentsList)if(e!=null)armor.addEnchantment(e,level);p.inventory.armorInventory[slot]=armor;}
            for(int id=0;id<256;id++){i(o,(Integer)lookup.invoke(null,id,held));i(o,(Integer)maximum.invoke(null,id,p.inventory.armorInventory));}
            for(int seed=0;seed<16;seed++)for(DamageSource s:sources){rng.setSeed(seed);try{i(o,0);int n=EnchantmentHelper.getEnchantmentModifierDamage(p.inventory,s);i(o,n);}catch(IllegalArgumentException ex){i(o,Integer.MIN_VALUE);}o.writeLong(EntityOracle.rngState(rng));}
            for(int seed=0;seed<16;seed++)for(EntityLiving m:mobs){rng.setSeed(seed);i(o,EnchantmentHelper.getEnchantmentModifierLiving(p.inventory,m));o.writeLong(EntityOracle.rngState(rng));}
            i(o,EnchantmentHelper.getKnockbackModifier(p.inventory,mobs[0]));i(o,EnchantmentHelper.getFireAspectModifier(p.inventory,mobs[0]));i(o,EnchantmentHelper.getRespiration(p.inventory));i(o,EnchantmentHelper.getEfficiencyModifier(p.inventory));i(o,EnchantmentHelper.getUnbreakingModifier(p.inventory));o.writeBoolean(EnchantmentHelper.getSilkTouchModifier(p.inventory));i(o,EnchantmentHelper.getFortuneModifier(p.inventory));i(o,EnchantmentHelper.getLootingModifier(p.inventory));o.writeBoolean(EnchantmentHelper.getAquaAffinityModifier(p.inventory));
        }
    }
    static void names(DataOutputStream o) throws Exception {
        for(long seed:new long[]{0,1,42,-1,Long.MIN_VALUE,Long.MAX_VALUE}){EnchantmentNameParts names=EnchantmentNameParts.field_40253_a;names.func_40250_a(seed);for(int n=0;n<1000;n++)text(o,names.func_40249_a());names.func_40250_a(seed);text(o,names.func_40249_a());}
    }
    static void animation(DataOutputStream o) throws Exception {
        World w=Mode_entities10.world();Mode_entities10.Player player=new Mode_entities10.Player(w);w.playerEntities.add(player);
        Field f=TileEntityEnchantmentTable.class.getDeclaredField("field_40064_r");f.setAccessible(true);Random rng=(Random)f.get(null);
        for(int x:new int[]{0,-20,16777216})for(long seed=0;seed<16;seed++) {
            rng.setSeed(seed);TileEntityEnchantmentTable t=new TileEntityEnchantmentTable();t.worldObj=w;t.xCoord=x;t.yCoord=65;t.zCoord=-3;
            for(int tick=0;tick<400;tick++) {
                double offset=tick<100?.5:tick<200?-1:tick<300?3:2;
                player.setPosition((double)((float)x+.5F)+offset,65.5,-2.5+(tick%3-1));
                t.updateEntity();i(o,t.field_40068_a);
                for(float value:new float[]{t.field_40063_b,t.field_40065_c,t.field_40061_d,t.field_40062_e,t.field_40059_f,t.field_40060_g,t.field_40069_h,t.field_40067_p,t.field_40066_q})i(o,Float.floatToRawIntBits(value));
                o.writeLong(EntityOracle.rngState(rng));
            }
        }
    }
    static void potionitem(DataOutputStream o) throws Exception {
        Field table=StringTranslate.class.getDeclaredField("translateTable");table.setAccessible(true);Properties translations=new Properties();
        table.set(StringTranslate.getInstance(),translations);
        translations.setProperty("item.potion.name"," \tBASE\n");
        translations.setProperty("item.emptyPotion.name"," \tEMPTY\n");translations.setProperty("potion.prefix.grenade"," \tSPLASH\n");
        World w=Mode_entities10.world();Mode_entities10.Player p=new Mode_entities10.Player(w);
        for(int damage=-32;damage<32768;damage++){
            ItemStack s=new ItemStack(373,1,damage);effects(o,Item.potion.getEffectNames(s));o.writeBoolean(Item.potion.getEffectNamesFromDamage(damage)==Item.potion.getEffectNamesFromDamage(damage));i(o,Item.potion.getIconFromDamage(damage));i(o,Item.potion.getColorFromDamage(damage, 0));o.writeBoolean(Item.potion.isEffectInstant(damage));o.writeBoolean(Item.potion.hasEffect(s));text(o,Item.potion.getItemDisplayName(s));List lines=new ArrayList();Item.potion.addInformation(s,lines);i(o,lines.size());for(Object line:lines)text(o,(String)line);
        }
        for(boolean remote:new boolean[]{false,true})for(int count:new int[]{-1,0,1,2,64})for(int damage:new int[]{0,16,8194,8197,16421}){
            w.multiplayerWorld=remote;Arrays.fill(p.inventory.mainInventory,null);p.activePotionsMap.clear();ItemStack s=new ItemStack(373,count,damage);ItemStack result=Item.potion.onFoodEaten(s,w,p);stack(o,s);stack(o,result);o.writeBoolean(s==result);for(ItemStack inv:p.inventory.mainInventory)stack(o,inv);for(int id=1;id<20;id++){PotionEffect effect=(PotionEffect)p.activePotionsMap.get(id);o.writeBoolean(effect!=null);if(effect!=null){i(o,effect.getDuration());i(o,effect.getAmplifier());}}
        }
    }
    static void box(DataOutputStream o,AxisAlignedBB b) throws Exception {o.writeBoolean(b!=null);if(b!=null)for(double v:new double[]{b.minX,b.minY,b.minZ,b.maxX,b.maxY,b.maxZ})o.writeLong(Double.doubleToRawLongBits(v));}
    static void geometry(DataOutputStream o) throws Exception {
        World w=Mode_entities10.world();
        int[] neighbors={0,1,20,54,85,86,107,113,117,118};
        for(int id:new int[]{85,110,111,112,113,114,115,116,117,118,119,120,121,122})for(int meta=0;meta<16;meta++)for(int n:neighbors) {
            EntityOracle.rawBlock(w,8,64,8,id,meta);for(int[] d:new int[][]{{-1,0},{1,0},{0,-1},{0,1}})EntityOracle.rawBlock(w,8+d[0],64,8+d[1],n,0);
            Block b=Block.blocksList[id];b.setBlockBoundsBasedOnState(w,8,64,8);box(o,b.getSelectedBoundingBoxFromPool(w,8,64,8));box(o,b.getCollisionBoundingBoxFromPool(w,8,64,8));
            List<AxisAlignedBB> list=w.getCollidingBoundingBoxes(null,AxisAlignedBB.getBoundingBoxFromPool(8,64,8,9,66,9));i(o,list.size());for(AxisAlignedBB bb:list)box(o,bb);
            for(int side=0;side<6;side++)i(o,b.getBlockTextureFromSideAndMetadata(side,meta));
        }
        for(int height:new int[]{64,65,128})for(int y:new int[]{-1,0,63,64,65,127,128})for(int below:new int[]{0,1,8,9,88})for(int meta=0;meta<2;meta++) {
            w.worldHeight=height;if(y>0&&y<=128)EntityOracle.rawBlock(w,8,y-1,8,below,meta);
            o.writeBoolean(Block.waterlily.canBlockStay(w,8,y,8));o.writeBoolean(Block.netherStalk.canBlockStay(w,8,y,8));
        }
    }
    static void particles(DataOutputStream o) throws Exception {
        World w=Mode_entities10.world();
        for(int id:new int[]{110,116,117,119})for(long seed=0;seed<128;seed++) {
            for(int x=6;x<=10;x++)for(int z=6;z<=10;z++)for(int y=64;y<=65;y++)EntityOracle.rawBlock(w,x,y,z,Math.abs(x-8)==2||Math.abs(z-8)==2?47:0,0);
            Random rng=new Random(seed);Block.blocksList[id].randomDisplayTick(w,8,64,8,rng);o.writeLong(EntityOracle.rngState(rng));WorldEventOracle.snapshot(w,o);
        }
    }
    static void dragon(DataOutputStream o) throws Exception {
        World w=Mode_entities10.world();Mode_entities10.Player p=new Mode_entities10.Player(w);
        for(boolean remote:new boolean[]{false,true})for(long seed=0;seed<64;seed++) {
            w.multiplayerWorld=remote;w.rand.setSeed(seed);EntityOracle.rawBlock(w,8,64,8,122,11);
            o.writeBoolean(Block.blocksList[122].blockActivated(w,8,64,8,p));o.writeLong(EntityOracle.rngState(w.rand));WorldEventOracle.snapshot(w,o);
            for(int x=-7;x<=23;x++)for(int y=57;y<=71;y++)for(int z=-7;z<=23;z++)if(w.getBlockId(x,y,z)==122){i(o,x);i(o,y);i(o,z);i(o,w.getBlockMetadata(x,y,z));EntityOracle.rawBlock(w,x,y,z,0,0);}i(o,Integer.MIN_VALUE);
        }
        w.multiplayerWorld=false;for(int x=-7;x<=23;x++)for(int y=57;y<=71;y++)for(int z=-7;z<=23;z++)EntityOracle.rawBlock(w,x,y,z,1,0);EntityOracle.rawBlock(w,8,64,8,122,0);w.rand.setSeed(42);Block.blocksList[122].onBlockClicked(w,8,64,8,p);o.writeLong(EntityOracle.rngState(w.rand));WorldEventOracle.snapshot(w,o);i(o,w.getBlockId(8,64,8));
    }
    static void inventory(DataOutputStream o,InventoryPlayer inv) throws Exception {for(ItemStack s:inv.mainInventory)stack(o,s);}
    static void cauldron(DataOutputStream o) throws Exception {
        World w=Mode_entities10.world();Mode_entities10.Player p=new Mode_entities10.Player(w);
        for(boolean remote:new boolean[]{false,true})for(boolean creative:new boolean[]{false,true})for(boolean full:new boolean[]{false,true})for(int meta=0;meta<16;meta++)for(int id:new int[]{0,1,326,374})for(int count:new int[]{-1,0,1,3}) {
            w.multiplayerWorld=remote;p.capabilities.depleteBuckets=creative;for(int slot=0;slot<36;slot++)p.inventory.mainInventory[slot]=full?new ItemStack(1,64,0):null;p.inventory.mainInventory[0]=id==0?null:new ItemStack(id,count,0);w.loadedEntityList.clear();OracleHooks.reset(100);EntityOracle.mathRandom().setSeed(200);w.rand.setSeed(300);
            EntityOracle.rawBlock(w,8,64,8,118,meta);o.writeBoolean(Block.cauldron.blockActivated(w,8,64,8,p));i(o,w.getBlockMetadata(8,64,8));inventory(o,p.inventory);i(o,w.loadedEntityList.size());for(Object e:w.loadedEntityList){EntityItem item=(EntityItem)e;stack(o,item.item);for(double v:new double[]{item.posX,item.posY,item.posZ,item.motionX,item.motionY,item.motionZ})o.writeLong(Double.doubleToRawLongBits(v));o.writeLong(EntityOracle.rngState(item.rand));}o.writeLong(EntityOracle.rngState(w.rand));
        }
    }
    static void eye(DataOutputStream o) throws Exception {
        World w=Mode_entities10.world();Mode_entities10.Player p=new Mode_entities10.Player(w);
        Field f=Item.class.getDeclaredField("itemRand");f.setAccessible(true);Random rng=(Random)f.get(null);
        for(boolean remote:new boolean[]{false,true})for(boolean creative:new boolean[]{false,true})for(int dir=0;dir<4;dir++)for(int missing=-1;missing<12;missing++) {
            w.multiplayerWorld=remote;p.capabilities.depleteBuckets=creative;for(int x=2;x<=14;x++)for(int z=2;z<=14;z++)EntityOracle.rawBlock(w,x,64,z,0,0);
            int sx=Direction.field_35871_a[(dir+1)%4],sz=Direction.field_35870_b[(dir+1)%4],dx=Direction.field_35871_a[dir],dz=Direction.field_35870_b[dir];int at=0;
            for(int forward:new int[]{0,4})for(int side=-1;side<=1;side++){EntityOracle.rawBlock(w,8+sx*side+dx*forward,64,8+sz*side+dz*forward,at==missing?0:120,dir+4);at++;}
            for(int side:new int[]{-2,2})for(int forward=1;forward<=3;forward++){EntityOracle.rawBlock(w,8+sx*side+dx*forward,64,8+sz*side+dz*forward,at==missing?0:120,dir+4);at++;}
            EntityOracle.rawBlock(w,8,64,8,120,dir);rng.setSeed(42);ItemStack stack=new ItemStack(381,3,0);o.writeBoolean(Item.eyeOfEnder.onItemUse(stack,p,w,8,64,8,1));stack(o,stack);o.writeLong(EntityOracle.rngState(rng));WorldEventOracle.snapshot(w,o);for(int x=2;x<=14;x++)for(int z=2;z<=14;z++){i(o,w.getBlockId(x,64,z));i(o,w.getBlockMetadata(x,64,z));}
        }
    }
    static final class EyeWorld extends World {
        int calls;int[] query;ChunkPosition target;
        EyeWorld(){super(new Oracle.NullSaveHandler(0),"oracle",Oracle.settings(0));chunkProvider=new EntityOracle.FlatProvider(this);}
        public ChunkPosition func_40477_b(String name,int x,int y,int z){if(!name.equals("Stronghold"))throw new AssertionError(name);calls++;query=new int[]{x,y,z};return target;}
    }
    static void eyeright(DataOutputStream o) throws Exception {
        Field f=Item.class.getDeclaredField("itemRand");f.setAccessible(true);Random rng=(Random)f.get(null);
        for(boolean remote:new boolean[]{false,true})for(boolean creative:new boolean[]{false,true})for(boolean frame:new boolean[]{false,true})for(int target=0;target<3;target++)for(double x:new double[]{-1.75,.5,9.9}) {
            EyeWorld w=new EyeWorld();WorldEventOracle.attach(w);OracleHooks.reset(100);Mode_entities10.Player p=new Mode_entities10.Player(w);p.setPosition(x,65,-3.25);p.capabilities.depleteBuckets=creative;w.multiplayerWorld=remote;w.target=target==0?null:target==1?new ChunkPosition(2,66,1):new ChunkPosition(100,45,-200);
            if(frame)EntityOracle.rawBlock(w,MathHelper.floor_double(x),65,0,120,0);
            rng.setSeed(42);ItemStack s=new ItemStack(381,3,0);ItemStack result=Item.eyeOfEnder.onItemRightClick(s,w,p);stack(o,result);o.writeBoolean(s==result);i(o,w.calls);if(w.calls>0)for(int v:w.query)i(o,v);o.writeLong(EntityOracle.rngState(rng));i(o,w.loadedEntityList.size());
            for(Object obj:w.loadedEntityList){EntityEnderEye eye=(EntityEnderEye)obj;for(double v:new double[]{eye.posX,eye.posY,eye.posZ})o.writeLong(Double.doubleToRawLongBits(v));for(String key:new String[]{"field_40094_b","field_40095_c","field_40091_d"}){Field field=EntityEnderEye.class.getDeclaredField(key);field.setAccessible(true);o.writeLong(Double.doubleToRawLongBits(field.getDouble(eye)));}o.writeLong(EntityOracle.rngState(eye.rand));}
            WorldEventOracle.snapshot(w,o);
        }
    }
    static void fortune(DataOutputStream o) throws Exception {
        for(int id=1;id<=122;id++)for(int meta=0;meta<16;meta++)for(int fortune=-1;fortune<=4;fortune++)for(long seed=0;seed<8;seed++)for(int method=0;method<3;method++) {
            Random rng=new Random(seed);Block b=Block.blocksList[id];
            try{i(o,method==0?b.quantityDropped(rng):method==1?b.quantityDroppedWithBonus(fortune,rng):b.idDropped(meta,rng,fortune));}catch(IllegalArgumentException ex){i(o,Integer.MIN_VALUE);}
            o.writeLong(EntityOracle.rngState(rng));
        }
    }
    static void stack(DataOutputStream o,ItemStack s) throws Exception {o.writeBoolean(s!=null);if(s!=null){i(o,s.itemID);i(o,s.stackSize);i(o,s.getItemDamage());}}
    static void brewingState(DataOutputStream o,TileEntityBrewingStand b) throws Exception {
        i(o,b.getBrewTime());i(o,b.getFilledSlots());for(int slot=0;slot<4;slot++)stack(o,b.getStackInSlot(slot));
    }
    static void brewing(DataOutputStream o) throws Exception {
        Method can=TileEntityBrewingStand.class.getDeclaredMethod("canBrew");can.setAccessible(true);
        Method brew=TileEntityBrewingStand.class.getDeclaredMethod("brewPotions");brew.setAccessible(true);
        for(int id:INGREDIENTS)for(int damage=0;damage<32768;damage++) {
            TileEntityBrewingStand b=new TileEntityBrewingStand();b.setInventorySlotContents(0,new ItemStack(373,1,damage));b.setInventorySlotContents(1,new ItemStack(373,2,damage|16384));b.setInventorySlotContents(2,new ItemStack(1,0,7));b.setInventorySlotContents(3,new ItemStack(id,2,0));
            o.writeBoolean((Boolean)can.invoke(b));brew.invoke(b);brewingState(o,b);
        }
        World w=Mode_entities10.world();w.setBlockAndMetadata(0,65,0,117,0);
        for(int scenario=0;scenario<8;scenario++) {
            TileEntityBrewingStand b=new TileEntityBrewingStand();b.worldObj=w;b.xCoord=0;b.yCoord=65;b.zCoord=0;
            b.setInventorySlotContents(0,new ItemStack(373,1,scenario==7?8194:0));b.setInventorySlotContents(3,new ItemStack(scenario==7?331:372,2,0));
            for(int tick=0;tick<605;tick++) {
                if(tick==50){if(scenario==1)b.setInventorySlotContents(3,null);if(scenario==2)b.setInventorySlotContents(3,new ItemStack(353,1,0));if(scenario==3)b.getStackInSlot(3).stackSize=0;if(scenario==4)b.setInventorySlotContents(0,null);if(scenario==5){NBTTagCompound n=new NBTTagCompound();b.writeToNBT(n);b=new TileEntityBrewingStand();b.readFromNBT(n);b.worldObj=w;}if(scenario==6)b.setBrewTime(-1);}
                b.updateEntity();brewingState(o,b);i(o,w.getBlockMetadata(0,65,0));
            }
        }
        for(int slot=-2;slot<6;slot++)for(int count:new int[]{-1,0,1,100}){TileEntityBrewingStand b=new TileEntityBrewingStand();b.setInventorySlotContents(slot,new ItemStack(353,99,3));stack(o,b.getStackInSlot(slot));stack(o,b.decrStackSize(slot,count));stack(o,b.getStackInSlot(slot));}
    }
    static void enchantState(DataOutputStream o,ContainerEnchantment c,Random rng,EntityPlayer p) throws Exception {
        o.writeLong(c.nameSeed);for(int cost:c.enchantLevels)i(o,cost);o.writeLong(EntityOracle.rngState(rng));i(o,p.playerLevel);
        ItemStack s=c.tableInventory.getStackInSlot(0);stack(o,s);
        Method maximum=EnchantmentHelper.class.getDeclaredMethod("getMaxEnchantmentLevel",int.class,ItemStack[].class);maximum.setAccessible(true);
        for(int id=0;id<256;id++)i(o,s==null?0:(Integer)maximum.invoke(null,id,new ItemStack[]{s}));
    }
    static void enchantcontainer(DataOutputStream o) throws Exception {
        Field field=ContainerEnchantment.class.getDeclaredField("field_40237_l");field.setAccessible(true);
        for(boolean remote:new boolean[]{false,true})for(int mask=0;mask<4;mask++)for(long seed=0;seed<16;seed++)for(int id:new int[]{1,256,261,267,276,283,298,310,373})for(int count:new int[]{1,2}) {
            World w=Mode_entities10.world();w.multiplayerWorld=remote;Mode_entities10.Player p=new Mode_entities10.Player(w);p.playerLevel=100;
            for(int x=6;x<=10;x++)for(int z=6;z<=10;z++)for(int y=64;y<=65;y++)EntityOracle.rawBlock(w,x,y,z,(Math.abs(x-8)==2||Math.abs(z-8)==2)&&mask>0?47:(mask==2&&x==9&&z==9?1:0),0);
            ContainerEnchantment c=new ContainerEnchantment(p.inventory,w,8,64,8);Random rng=(Random)field.get(c);rng.setSeed(seed);
            ((Slot)c.inventorySlots.get(0)).putStack(new ItemStack(id,count,mask==3?5:0));enchantState(o,c,rng,p);
            for(int button=0;button<3;button++){o.writeBoolean(c.enchantItem(p,button));enchantState(o,c,rng,p);}
            for(int progress=-1;progress<=3;progress++){c.updateProgressBar(progress,progress*3);enchantState(o,c,rng,p);}
        }
    }
    static void sounds(DataOutputStream o) throws Exception {
        for(int id=1;id<=122;id++){StepSound s=Block.blocksList[id].stepSound;i(o,id);text(o,s.stepSoundDir2());text(o,s.stepSoundDir());i(o,Float.floatToRawIntBits(s.getVolume()));i(o,Float.floatToRawIntBits(s.getPitch()));}
    }
    static final class SlotPlayer extends EntityPlayer {
        int potions;SlotPlayer(World w){super(w);}public void func_6420_o(){}public void addStat(StatBase stat,int amount){if(stat==AchievementList.potion)potions+=amount;}
    }
    static void brewcontainer(DataOutputStream o) throws Exception {
        World w=Mode_entities10.world();SlotPlayer p=new SlotPlayer(w);TileEntityBrewingStand b=new TileEntityBrewingStand();ContainerBrewingStand c=new ContainerBrewingStand(p.inventory,b);
        Field index=Slot.class.getDeclaredField("slotIndex");index.setAccessible(true);
        i(o,c.inventorySlots.size());for(Object obj:c.inventorySlots){Slot slot=(Slot)obj;i(o,index.getInt(slot));i(o,slot.xDisplayPosition);i(o,slot.yDisplayPosition);i(o,slot.getSlotStackLimit());}
        for(int id=0;id<Item.itemsList.length;id++)if(Item.itemsList[id]!=null)for(int damage:new int[]{0,1,8194,16384})for(int slot=0;slot<4;slot++){ItemStack stack=new ItemStack(id,1,damage);Slot target=(Slot)c.inventorySlots.get(slot);o.writeBoolean(target.isItemValid(stack));p.potions=0;target.onPickupFromSlot(stack);i(o,p.potions);}
        for(int source=0;source<40;source++)for(int id:new int[]{1,353,372,373,374})for(int damage:new int[]{0,8194})for(int count:new int[]{1,3})for(int action=0;action<3;action++)for(boolean full:new boolean[]{false,true}) {
            p.inventory=new InventoryPlayer(p);p.potions=0;b=new TileEntityBrewingStand();c=new ContainerBrewingStand(p.inventory,b);
            if(full)for(int slot=4;slot<40;slot++)((Slot)c.inventorySlots.get(slot)).putStack(new ItemStack(1,63,0));
            ((Slot)c.inventorySlots.get(source)).putStack(new ItemStack(id,count,damage));
            stack(o,c.slotClick(source,action==1?1:0,action==2,p));for(Object obj:c.inventorySlots)stack(o,((Slot)obj).getStack());stack(o,p.inventory.getItemStack());i(o,p.potions);
        }
    }
    public static void main(String[] a) throws Exception {
        // Match the game's bootstrap order (Item first causes AchievementList's
        // recursive initializer to observe incomplete static item fields).
        int bootstrap=Block.stone.blockID;
        if(a[1].equals("recipes")){ContainerOracle.recipes(a[2]);return;}
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[2])))){
            switch(a[1]){case "potion":potion(o);break;case "enchantments":enchantments(o);break;case "helper":helper(o);break;case "modifiers":modifiers(o);break;case "names":names(o);break;case "animation":animation(o);break;case "potionitem":potionitem(o);break;case "geometry":geometry(o);break;case "particles":particles(o);break;case "brewing":brewing(o);break;case "dragon":dragon(o);break;case "cauldron":cauldron(o);break;case "eye":eye(o);break;case "eyeright":eyeright(o);break;case "fortune":fortune(o);break;case "enchantcontainer":enchantcontainer(o);break;case "sounds":sounds(o);break;case "brewcontainer":brewcontainer(o);break;default:throw new IllegalArgumentException(a[1]);}
        }
    }
}
