package net.minecraft.src;

import java.io.*;
import java.util.*;
import java.lang.reflect.*;
import net.minecraft.client.Minecraft;

/** Player tests call the original jar's inventory, entity and controller methods. */
public final class PlayerOracle {
    static final class HeadlessMinecraft extends Minecraft {
        int portalRequests;
        HeadlessMinecraft(){super(null,null,null,1,1,false);}
        public void displayUnexpectedThrowable(UnexpectedThrowable t) {throw new RuntimeException();}
        public void usePortal() {portalRequests++;}
    }
    static final class HeadlessEffects extends EffectRenderer {
        HeadlessEffects(World w){super(w,null);}
        public void addBlockHitEffects(int x,int y,int z,int side) {}
    }
    static final class HeadlessPlayer extends EntityPlayerSP {
        HeadlessPlayer(Minecraft mc,World w) {super(mc,w,new Session("Player",""),0);}
        public void addStat(StatBase stat,int count) {}
        public void onItemPickup(Entity e,int count) {}
        public void displayGUIChest(IInventory i) {}
        public void displayWorkbenchGUI(int x,int y,int z) {}
        public void displayGUIFurnace(TileEntityFurnace t) {}
        public void displayGUIDispenser(TileEntityDispenser t) {}
    }
    static HeadlessPlayer player(World w)throws Exception {
        Minecraft mc=(Minecraft)Oracle.allocate(HeadlessMinecraft.class);
        mc.theWorld=w;mc.sndManager=(SoundManager)Oracle.allocate(SoundManager.class);
        StatFileWriter stats=(StatFileWriter)Oracle.allocate(StatFileWriter.class);
        Field map=StatFileWriter.class.getDeclaredField("field_25102_a");map.setAccessible(true);Map values=new HashMap();values.put(AchievementList.openInventory,1);map.set(stats,values);mc.statFileWriter=stats;
        HeadlessPlayer p=new HeadlessPlayer(mc,w);mc.thePlayer=p;p.movementInput=new MovementInput();return p;
    }
    static void state(EntityPlayer p,DataOutputStream o)throws Exception {
        for(float v:new float[]{p.field_775_e,p.field_774_f,p.cameraPitch,p.field_9328_R,p.timeInPortal,p.prevTimeInPortal})EntityOracle.f(o,v);
        for(int v:new int[]{p.swingProgressInt,p.timeUntilPortal,p.func_22060_M(),p.inventory.getTotalArmorValue(),p.inventory.currentItem,EntityOracle.intField(EntityPlayer.class,"damageRemainder",p)})i(o,v);
        o.writeBoolean(p.isSwinging);o.writeBoolean(p.isPlayerSleeping());o.writeBoolean(p.isSneaking());
        for(double v:new double[]{p.field_20066_r,p.field_20065_s,p.field_20064_t,p.field_20063_u,p.field_20062_v,p.field_20061_w})EntityOracle.d(o,v);
        for(ItemStack s:p.inventory.mainInventory)stack(o,s);for(ItemStack s:p.inventory.armorInventory)stack(o,s);
    }
    static void picking(HeadlessPlayer p,DataOutputStream o)throws Exception {
        p.mc.renderViewEntity=p;p.mc.playerController=new PlayerControllerSP(p.mc);
        EntityRenderer renderer=(EntityRenderer)Oracle.allocate(EntityRenderer.class);Field mc=EntityRenderer.class.getDeclaredField("mc");mc.setAccessible(true);mc.set(renderer,p.mc);
        for(float partial:new float[]{0,0.5F,1}) {
            Vec3D look=p.getLook(partial);for(double v:new double[]{look.xCoord,look.yCoord,look.zCoord})EntityOracle.d(o,v);
            renderer.getMouseOver(partial);MovingObjectPosition hit=p.mc.objectMouseOver;
            i(o,hit==null?-1:hit.typeOfHit==EnumMovingObjectType.ENTITY?1:0);
            if(hit!=null) {if(hit.typeOfHit==EnumMovingObjectType.ENTITY)i(o,hit.entityHit.entityId);else {i(o,hit.blockX);i(o,hit.blockY);i(o,hit.blockZ);i(o,hit.sideHit);for(double v:new double[]{hit.hitVec.xCoord,hit.hitVec.yCoord,hit.hitVec.zCoord})EntityOracle.d(o,v);}}
        }
    }
    static void simulation(String scenario,int ticks,String path)throws Exception {
        World w=Oracle.newWorld(0);w.chunkProvider=new EntityOracle.FlatProvider(w);w.difficultySetting=2;w.setAllowedMobSpawns(false,false);
        Field ids=Entity.class.getDeclaredField("nextEntityID");ids.setAccessible(true);ids.setInt(null,0);
        Item.itemRand.setSeed(600);OracleHooks.reset(100);EntityOracle.mathRandom().setSeed(200);w.rand.setSeed(300);w.field_9437_g=0;Field sound=World.class.getDeclaredField("soundCounter");sound.setAccessible(true);sound.setInt(w,0);
        Field chest=BlockChest.class.getDeclaredField("random");chest.setAccessible(true);((Random)chest.get(Block.chest)).setSeed(500);
        if(scenario.equals("pipeline")||scenario.equals("sleepworld")) {EntityOracle.FlatProvider f=(EntityOracle.FlatProvider)w.chunkProvider;for(int x=-12;x<=12;x++)for(int z=-12;z<=12;z++)if(!f.chunkExists(x,z)){byte[] blocks=new byte[32768];for(int bx=0;bx<16;bx++)for(int bz=0;bz<16;bz++)blocks[(bx<<11)|(bz<<7)|63]=1;Chunk c=new Chunk(w,blocks,x,z);c.isTerrainPopulated=true;c.isChunkLoaded=true;f.chunks.put(f.key(x,z),c);}}
        HeadlessPlayer p=player(w);p.setLocationAndAngles(0.5,64,0.5,0,0);w.entityJoinedWorld(p);
        PlayerControllerSP controller=new PlayerControllerSP(p.mc);p.mc.playerController=controller;p.mc.effectRenderer=new HeadlessEffects(w);p.mc.entityRenderer=(EntityRenderer)Oracle.allocate(EntityRenderer.class);p.mc.entityRenderer.itemRenderer=(ItemRenderer)Oracle.allocate(ItemRenderer.class);boolean interaction=scenario.equals("digextras")||scenario.equals("dig")||scenario.equals("place")||scenario.equals("use")||scenario.equals("shoot")||scenario.equals("interact")||scenario.equals("fertilize")||scenario.equals("activate")||scenario.equals("api")||scenario.equals("pipeline")||scenario.equals("bucket");int result=0;
        for(int z=3;z<=5;z++)for(int x=-2;x<=2;x++)EntityOracle.rawBlock(w,x,64,z,1,0);
        for(int x=4;x<=8;x++)for(int z=-2;z<=2;z++)EntityOracle.rawBlock(w,x,64,z,53,0);
        for(int x=10;x<=15;x++)for(int y=64;y<=67;y++)for(int z=-2;z<=2;z++)EntityOracle.rawBlock(w,x,y,z,9,0);
        for(int x=-15;x<=-10;x++)for(int y=64;y<=67;y++)for(int z=-2;z<=2;z++)EntityOracle.rawBlock(w,x,y,z,11,0);
        EntityOracle.rawBlock(w,1,64,-5,30,0);for(int y=64;y<=71;y++){EntityOracle.rawBlock(w,0,y,-8,65,3);EntityOracle.rawBlock(w,0,y,-9,1,0);}
        if(scenario.equals("pickentities")){EntityPig pig=new EntityPig(w);pig.setLocationAndAngles(0.5,65,2.5,0,0);w.entityJoinedWorld(pig);EntityTNTPrimed tnt=new EntityTNTPrimed(w,0.5,65,-2.5);w.entityJoinedWorld(tnt);}
        if(scenario.equals("nbt")){p.inventory.mainInventory[35]=new ItemStack(35,63,14);p.inventory.mainInventory[8]=new ItemStack(260,3,0);p.inventory.armorInventory[2]=new ItemStack(312,1,103);p.inventory.currentItem=8;p.score=123;p.dimension=-1;p.setPlayerSpawnCoordinate(new ChunkCoordinates(10,64,-5));}
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(path)))) {
            EntityOracle.snapshot(w,o);if((scenario.equals("look")||scenario.equals("pickentities")))picking(p,o);if(interaction)controllerState(controller,result,o);if(scenario.equals("api")||scenario.equals("pipeline"))apiState(p,o);if(scenario.equals("nbt"))nbtState(p,o);
            for(int t=0;t<ticks;t++) {
                if(!p.isDead) {
                    if(t%120==0) {
                        int phase=t/120%8;p.motionX=p.motionY=p.motionZ=0;
                        double x=phase==2?4.5:phase==3?11.5:phase==4?-12.5:0.5;
                        double z=phase==6?-7.5:phase==7?-4.5:0.5;
                        double y=phase==5?76:phase==3||phase==4?66:64;
                        p.setLocationAndAngles(x,y,z,phase==2?-90:phase>=6?180:0,0);
                        if(scenario.equals("movement")||(scenario.equals("look")||scenario.equals("pickentities"))){p.health=20;p.heartsLife=0;}
                    }
                    if(scenario.equals("damage")&&t==1) {for(int a=0;a<4;a++)p.inventory.armorInventory[a]=new ItemStack(310+a,1,a*40);p.attackEntityFrom(null,3);}
                    if(scenario.equals("damage")&&t==30) {EntityZombie mob=new EntityZombie(w);mob.setLocationAndAngles(0.5,64,1.5,0,0);w.entityJoinedWorld(mob);p.attackEntityFrom(mob,5);}
                    if(scenario.equals("pickup")&&t%20==0){EntityItem item=new EntityItem(w,p.posX,p.posY-1.3,p.posZ,new ItemStack(t%40==0?35:263,40,t%3));w.entityJoinedWorld(item);}
                    p.movementInput.moveForward=t%120<80?1:0;p.movementInput.moveStrafe=t%120>=80?1:0;p.movementInput.jump=t%31<8;p.movementInput.sneak=t%120>=90;
                    if(p.movementInput.sneak){p.movementInput.moveForward=(float)((double)p.movementInput.moveForward*0.3);p.movementInput.moveStrafe=(float)((double)p.movementInput.moveStrafe*0.3);}
                    if(t%13==0)p.swingItem();
                    if(scenario.equals("nbt")&&t%100==60){w.skylightSubtracted=11;w.worldInfo.setWorldTime(18000);p.setLocationAndAngles(0.5,64,0.5,0,0);EntityOracle.rawBlock(w,0,64,0,26,0);EntityOracle.rawBlock(w,0,64,1,26,8);p.sleepInBedAt(0,64,1);}
                    if(scenario.equals("combat")) {
                        p.movementInput.moveForward=p.movementInput.moveStrafe=0;p.movementInput.jump=p.movementInput.sneak=false;
                        if(t%100==0){p.health=20;p.heartsLife=0;w.difficultySetting=t/100%4;EntityLiving mob=t%200==0?new EntityWolf(w):new EntityPigZombie(w);mob.setLocationAndAngles(0.5,64,1.5,0,0);w.entityJoinedWorld(mob);p.inventory.mainInventory[0]=null;p.attackTargetEntityWithCurrentItem(mob);}
                        if(t%100==20){EntityArrow arrow=new EntityArrow(w,p);w.entityJoinedWorld(arrow);p.heartsLife=0;p.attackEntityFrom(arrow,5);}
                    }
                    if(scenario.equals("survival")) {
                        p.movementInput.moveForward=p.movementInput.moveStrafe=0;p.movementInput.jump=p.movementInput.sneak=false;
                        if(t<400){p.setLocationAndAngles(11.5,65.5,0.5,0,0);p.motionX=p.motionY=p.motionZ=0;}
                        if(t==400){p.setLocationAndAngles(0.5,64,0.5,0,0);p.motionX=p.motionY=p.motionZ=0;p.health=5;w.difficultySetting=0;}
                        if(t==700){p.username="Notch";p.inventory.mainInventory[0]=new ItemStack(264,20,0);p.attackEntityFrom(null,100);}
                    }

                    if(scenario.equals("sleep")||scenario.equals("night")||scenario.equals("sleepworld")) {
                        if(t==0){w.skylightSubtracted=11;w.worldInfo.setWorldTime(18000);p.setLocationAndAngles(0.5,64,0.5,0,0);EntityOracle.rawBlock(w,0,64,0,26,0);EntityOracle.rawBlock(w,0,64,1,26,8);p.sleepInBedAt(0,64,1);}
                        if(scenario.equals("sleep")&&t==120)p.wakeUpPlayer(false,true,true);
                    }
                    if(scenario.equals("night")&&t==100)SpawnerAnimals.performSleepSpawning(w,w.playerEntities);
                    if(scenario.equals("portal")){p.movementInput.moveForward=p.movementInput.moveStrafe=0;p.movementInput.jump=false;if(t<110||t>=150&&t<160)p.setInPortal();}
                    if((scenario.equals("look")||scenario.equals("pickentities"))){p.health=20;p.heartsLife=0;p.func_346_d((t%7-3)*33.7F,(t%11-5)*9.8F);}
                    if(interaction) {
                        p.setLocationAndAngles(0.5,64,0.5,t*33.7F,0);p.motionX=p.motionY=p.motionZ=0;p.movementInput.moveStrafe=p.movementInput.moveForward=0;p.movementInput.jump=p.movementInput.sneak=false;p.onGround=true;
                        controller.updateController();result=0;
                        if(scenario.equals("bucket")) {
                            p.setLocationAndAngles(0.5,64,0.5,0,90);int item=t%4==0||t%4==1?325:t%4==2?326:327;
                            EntityOracle.rawBlock(w,0,64,0,t%4==0?8:t%4==1?10:0,0);p.inventory.mainInventory[0]=new ItemStack(item,1,0);result=controller.sendUseItem(p,w,p.inventory.mainInventory[0])?1:0;
                        } else if(scenario.equals("dig")||scenario.equals("digextras")) {
                            int[] blocks=scenario.equals("digextras")?new int[]{6,59,43,47,21,73,82,26,64,71,20,30,32,31,52,92}:new int[]{1,17,56,49,35,18,78,79,4,3},tools=scenario.equals("digextras")?new int[]{0,0,278,0,278,278,269,0,0,0,0,359,0,359,278,0}:new int[]{0,271,257,278,359,359,269,278,0,0};int phase=t/100%blocks.length;int meta=scenario.equals("digextras")?(phase==0||phase==2?2:phase==1?7:0):(phase==4?7:0);
                            if(t%100==0){EntityOracle.rawBlock(w,8,64,8,blocks[phase],meta);if(blocks[phase]==26)EntityOracle.rawBlock(w,8,64,9,26,8);if(blocks[phase]==64||blocks[phase]==71)EntityOracle.rawBlock(w,8,65,8,blocks[phase],8);p.inventory.mainInventory[0]=tools[phase]==0?null:new ItemStack(tools[phase],1,0);controller.resetBlockRemoving();controller.clickBlock(8,64,8,1);}
                            controller.sendBlockRemoving(8,64,8,1);
                        } else if(scenario.equals("place")) {
                            int[] blocks={1,5,35,44,53,67,86,50,65,69,77,324,330,355,54,23,61,96,356,354,338,32,39,40};int block=blocks[t/6%blocks.length],x=8+t/6%6*4,z=8+t/36%6*4,side=t%6;
                            for(int dx=-1;dx<=1;dx++)for(int dz=-1;dz<=1;dz++){for(int y=64;y<=67;y++){w.removeBlockTileEntity(x+dx,y,z+dz);EntityOracle.rawBlock(w,x+dx,y,z+dz,y==64?1:0,0);}}
                            EntityOracle.rawBlock(w,x,64,z,1,0);p.inventory.mainInventory[0]=new ItemStack(block,10,t%4);result=controller.sendPlaceBlock(p,w,p.inventory.mainInventory[0],x,64,z,side)?1:0;
                            if(p.inventory.mainInventory[0]!=null&&p.inventory.mainInventory[0].stackSize==0)p.inventory.mainInventory[0]=null;
                        } else if(scenario.equals("api")||scenario.equals("pipeline")) {
                            if(t%40==0){EntityOracle.rawBlock(w,8,64,8,1,0);p.inventory.mainInventory[0]=new ItemStack(278,1,0);}
                            if(t%40==20)p.inventory.mainInventory[0]=new ItemStack(5,64,0);
                            if(t%40==30)p.inventory.mainInventory[0]=new ItemStack(260,3,0);
                            p.mc.objectMouseOver=t%40<30?new MovingObjectPosition(8,64,8,1,Vec3D.createVector(8,65,8)):null;
                            api(p,controller,t);
                        } else if(scenario.equals("interact")) {
                            int phase=t/30%7;int[] items={359,351,325,329,352,276,278};int item=items[phase];p.inventory.mainInventory[0]=new ItemStack(item,10,phase==1?t%16:0);
                            if(t%30==0){EntityLiving mob=phase<2?new EntitySheep(w):phase==2?new EntityCow(w):phase==3?new EntityPig(w):phase==4?new EntityWolf(w):new EntityZombie(w);mob.setLocationAndAngles(0.5,64,1.5,0,0);w.entityJoinedWorld(mob);}
                            Entity mob=(Entity)w.loadedEntityList.get(w.loadedEntityList.size()-1);
                            if(mob instanceof EntityItem){for(Object o2:w.loadedEntityList)if(o2 instanceof EntityLiving&&o2!=p)mob=(Entity)o2;}
                            if(phase>=5)p.attackTargetEntityWithCurrentItem(mob);else p.useCurrentItemOnEntity(mob);
                        } else if(scenario.equals("fertilize")) {
                            int phase=t%5;int block=phase<3?6:phase==3?59:2;int x=8,z=8;
                            EntityOracle.rawBlock(w,x,63,z,phase==3?60:2,0);EntityOracle.rawBlock(w,x,64,z,block,phase<3?phase:0);p.inventory.mainInventory[0]=new ItemStack(351,10,15);result=controller.sendPlaceBlock(p,w,p.inventory.mainInventory[0],x,64,z,1)?1:0;
                        } else if(scenario.equals("activate")) {
                            int[] blocks={64,69,77,96,93,92,58,54,61,23};int block=blocks[t%blocks.length];w.removeBlockTileEntity(8,64,8);EntityOracle.rawBlock(w,8,64,8,block,block==64?0:block==69||block==77?5:0);if(block==64)EntityOracle.rawBlock(w,8,65,8,block,8);
                            p.health=5;p.inventory.mainInventory[0]=null;result=controller.sendPlaceBlock(p,w,null,8,64,8,1)?1:0;
                        } else if(scenario.equals("shoot")) {
                            int item=t%3==0?261:t%3==1?332:344;p.inventory.mainInventory[0]=new ItemStack(item,16,0);p.inventory.mainInventory[1]=new ItemStack(262,64,0);result=controller.sendUseItem(p,w,p.inventory.mainInventory[0])?1:0;
                        } else {
                            int[] items={260,282,297,319,320,322,349,350,357,259,290,325,326,327,335};int item=items[t%items.length];p.health=5;p.inventory.mainInventory[0]=new ItemStack(item,1,0);
                            if(item==259||item==290){EntityOracle.rawBlock(w,8,64,8,3,0);result=controller.sendPlaceBlock(p,w,p.inventory.mainInventory[0],8,64,8,t%6)?1:0;}else result=controller.sendUseItem(p,w,p.inventory.mainInventory[0])?1:0;
                            if(p.inventory.mainInventory[0]!=null&&p.inventory.mainInventory[0].stackSize==0)p.inventory.mainInventory[0]=null;
                        }
                    }
                    if(scenario.equals("nbt")&&t%100==99) {
                        NBTTagCompound n=new NBTTagCompound();p.writeToNBT(n);w.loadedEntityList.remove(p);w.playerEntities.remove(p);for(Chunk c:((EntityOracle.FlatProvider)w.chunkProvider).chunks.values())for(List list:c.entities)list.remove(p);
                        p=player(w);p.readFromNBT(n);w.entityJoinedWorld(p);
                    }
                }
                AxisAlignedBB.clearBoundingBoxPool();Vec3D.initialize();if((scenario.equals("pipeline")||scenario.equals("sleepworld"))&&(t+1)%30==0)w.joinEntityInSurroundings(p);w.updateEntities();if(scenario.equals("pipeline")||scenario.equals("sleepworld")){w.tick();w.randomDisplayUpdates(MathHelper.floor_double(p.posX),MathHelper.floor_double(p.posY),MathHelper.floor_double(p.posZ));}EntityOracle.snapshot(w,o);if((scenario.equals("look")||scenario.equals("pickentities")))picking(p,o);if(interaction)controllerState(controller,result,o);if(scenario.equals("api")||scenario.equals("pipeline"))apiState(p,o);if(scenario.equals("nbt"))nbtState(p,o);
            }
        }
        Oracle.dumpWorld(w,path+".world",0,0,0);
        try(FileOutputStream o=new FileOutputStream(path,true)){o.write(java.nio.file.Files.readAllBytes(new File(path+".world").toPath()));}new File(path+".world").delete();
    }
    static void canonical(NBTBase n,DataOutputStream o)throws Exception {
        if(n instanceof NBTTagCompound) {Field map=NBTTagCompound.class.getDeclaredField("tagMap");map.setAccessible(true);Map<String,NBTBase> tags=(Map)map.get(n);for(String key:new TreeSet<String>(tags.keySet())) {NBTBase child=tags.get(key);o.writeByte(child.getType());o.writeUTF(key);canonical(child,o);}o.writeByte(0);}
        else if(n instanceof NBTTagList) {NBTTagList list=(NBTTagList)n;o.writeByte(list.tagCount()==0?1:list.tagAt(0).getType());o.writeInt(list.tagCount());for(int i=0;i<list.tagCount();i++)canonical(list.tagAt(i),o);}
        else n.writeTagContents(o);
    }
    static void nbtState(HeadlessPlayer p,DataOutputStream o)throws Exception {NBTTagCompound n=new NBTTagCompound();p.writeToNBT(n);ByteArrayOutputStream b=new ByteArrayOutputStream();DataOutputStream d=new DataOutputStream(b);d.writeByte(10);d.writeUTF("");canonical(n,d);byte[] bytes=b.toByteArray();i(o,bytes.length);o.write(bytes);}
    static void api(HeadlessPlayer p,PlayerControllerSP c,int ticks)throws Exception {
        Minecraft mc=p.mc;Method click=Minecraft.class.getDeclaredMethod("clickMouse",int.class);click.setAccessible(true);Method held=Minecraft.class.getDeclaredMethod("func_6254_a",int.class,boolean.class);held.setAccessible(true);
        boolean left=ticks%40<15||ticks%40>=35,right=ticks%40>=20&&ticks%40<34;
        int last=EntityOracle.intField(Minecraft.class,"mouseTicksRan",mc);
        if(ticks%40==0||ticks%40==35){click.invoke(mc,0);last=ticks;}
        if(ticks%40==20){click.invoke(mc,1);last=ticks;}
        int counter=EntityOracle.intField(Minecraft.class,"leftClickCounter",mc);Field f=Minecraft.class.getDeclaredField("leftClickCounter");f.setAccessible(true);if(counter>0)f.setInt(mc,counter-1);
        if(left&&ticks-last>=5){click.invoke(mc,0);last=ticks;}
        if(right&&ticks-last>=5){click.invoke(mc,1);last=ticks;}
        Field mouse=Minecraft.class.getDeclaredField("mouseTicksRan");mouse.setAccessible(true);mouse.setInt(mc,last);held.invoke(mc,0,left);
    }
    static void apiState(HeadlessPlayer p,DataOutputStream o)throws Exception {
        i(o,EntityOracle.intField(Minecraft.class,"leftClickCounter",p.mc));i(o,EntityOracle.intField(Minecraft.class,"mouseTicksRan",p.mc));EntityOracle.f(o,p.getEntityBrightness(1));
    }
    static void controllerState(PlayerControllerSP c,int result,DataOutputStream o)throws Exception {
        i(o,result);for(String name:new String[]{"curBlockDamage","prevBlockDamage","field_1069_h"})EntityOracle.f(o,EntityOracle.floatField(PlayerControllerSP.class,name,c));
        for(String name:new String[]{"field_1074_c","field_1073_d","field_1072_e","blockHitWait"})i(o,EntityOracle.intField(PlayerControllerSP.class,name,c));
        EntityOracle.l(o,EntityOracle.rngState(Item.itemRand));
    }
    static void i(DataOutputStream o,int n)throws IOException {EntityOracle.i(o,n);}
    static void stack(DataOutputStream o,ItemStack s)throws IOException {
        for(int n:s==null?new int[]{-1,0,0,0}:new int[]{s.itemID,s.stackSize,s.getItemDamage(),s.animationsToGo})i(o,n);
    }
    static void registry(String path)throws Exception {
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(path)))) {
            for(int id=0;id<=2257;id++) {
                Item a=Item.itemsList[id];
                i(o,a==null?0:a.getItemStackLimit());i(o,a==null?0:a.getMaxDamage());i(o,a!=null&&a.getHasSubtypes()?1:0);i(o,a instanceof ItemArmor?((ItemArmor)a).damageReduceAmount:0);
                for(int b=0;b<256;b++) {
                    Block block=Block.blocksList[b];
                    EntityOracle.f(o,a==null||block==null?1:a.getStrVsBlock(new ItemStack(id,1,0),block));
                    o.writeBoolean(a!=null&&block!=null&&a.canHarvestBlock(block));
                }
            }
            java.lang.reflect.Field strength=Block.class.getDeclaredField("blockHardness");strength.setAccessible(true);
            for(int b=0;b<256;b++) {Block a=Block.blocksList[b];EntityOracle.f(o,a==null?0:strength.getFloat(a));o.writeBoolean(a!=null&&a.blockMaterial.getIsHarvestable());}
            for(int id=0;id<=2257;id++)i(o,Item.itemsList[id]==null?1:Item.itemsList[id].getDamageVsEntity(null));
        }
    }
    static void inventory(String path)throws Exception {
        World w=Oracle.newWorld(0);w.chunkProvider=new EntityOracle.FlatProvider(w);
        EntityOtherPlayerMP player=new EntityOtherPlayerMP(w,"Player");InventoryPlayer inv=player.inventory;
        Random r=new Random(1729);int[] ids={1,35,263,256,257,298,299,300,301,332,351,357,2256};
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(path)))) {
            for(int t=0;t<10000;t++) {
                int op=r.nextInt(7),id=ids[r.nextInt(ids.length)],count=r.nextInt(80)+1,damage=r.nextInt(150),result=0;
                if(op<=1) {ItemStack s=new ItemStack(id,count,damage);result=inv.addItemStackToInventory(s)?1:0;stack(o,s);}
                else if(op==2)result=inv.consumeInventoryItem(id)?1:0;
                else if(op==3)stack(o,inv.decrStackSize(r.nextInt(40),count));
                else if(op==4) {inv.changeCurrentItem(count-40);result=inv.currentItem;}
                else if(op==5) {int slot=r.nextInt(4);inv.armorInventory[slot]=new ItemStack(298+r.nextInt(20),1,damage);inv.damageArmor(count);result=inv.getTotalArmorValue();}
                else {inv.decrementAnimations();inv.readFromNBT(inv.writeToNBT(new NBTTagList()));}
                i(o,result);for(ItemStack s:inv.mainInventory)stack(o,s);for(ItemStack s:inv.armorInventory)stack(o,s);i(o,inv.currentItem);i(o,inv.getTotalArmorValue());
            }
        }
    }
}
