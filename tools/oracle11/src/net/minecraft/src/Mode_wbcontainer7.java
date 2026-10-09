package net.minecraft.src;
import java.io.*;
import java.lang.invoke.*;
import java.lang.reflect.*;

/** Invoke original BlockContainer superclass methods on each real subclass. */
public final class Mode_wbcontainer7 {
    static MethodHandles.Lookup lookup()throws Exception {Field f=MethodHandles.Lookup.class.getDeclaredField("IMPL_LOOKUP");f.setAccessible(true);return (MethodHandles.Lookup)f.get(null);}
    static void type(DataOutputStream o,TileEntity t)throws Exception {
        String[] names={"TileEntityDispenser","TileEntityNote","TileEntityMobSpawner","TileEntityChest","TileEntityFurnace","TileEntitySign","TileEntityRecordPlayer","TileEntityPiston"};
        int id=0;if(t!=null)for(int k=0;k<names.length;k++)if(t.getClass().getSimpleName().equals(names[k]))id=k+1;Trace10.writeByte(o, "id", id);
    }
    static void state(DataOutputStream o,World w,TileEntity old)throws Exception {
        TileEntity t=w.getBlockTileEntity(8,64,8);type(o,t);Trace10.writeBoolean(o, "t==old", t==old);if(old!=null)Trace10.writeBoolean(o, "old.isInvalid()", old.isInvalid());
        EntityOracle.i(o,Trace10.namedInt("w.loadedTileEntityList.size()", w.loadedTileEntityList.size()));
        EntityOracle.i(o,Trace10.namedInt("t instanceof TileEntityChest?((TileEntityChest)t).field_35156_h:0", t instanceof TileEntityChest?((TileEntityChest)t).numUsingPlayers:0));
        EntityOracle.i(o,Trace10.namedInt("old instanceof TileEntityChest?((TileEntityChest)old).field_35156_h:0", old instanceof TileEntityChest?((TileEntityChest)old).numUsingPlayers:0));
    }
    public static void main(String[] a)throws Throwable {
        int init=Block.stone.blockID;MethodHandles.Lookup lookup=lookup();
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            for(int id:new int[]{23,25,36,52,54,61,62,63,68,84}) {
                BlockContainer b=(BlockContainer)Block.blocksList[id];type(o,b.getBlockEntity());
                MethodHandle add=lookup.findSpecial(BlockContainer.class,"onBlockAdded",MethodType.methodType(void.class,World.class,int.class,int.class,int.class),b.getClass());
                MethodHandle remove=lookup.findSpecial(BlockContainer.class,"onBlockRemoval",MethodType.methodType(void.class,World.class,int.class,int.class,int.class),b.getClass());
                MethodHandle play=lookup.findSpecial(BlockContainer.class,"powerBlock",MethodType.methodType(void.class,World.class,int.class,int.class,int.class,int.class,int.class),b.getClass());
                for(boolean remote:new boolean[]{false,true})for(boolean invalid:new boolean[]{false,true})for(int hook=0;hook<3;hook++)for(int event:new int[]{Integer.MIN_VALUE,0,1,2,Integer.MAX_VALUE})for(int value:new int[]{Integer.MIN_VALUE,-1,0,1,Integer.MAX_VALUE}) {
                    World w=Mode_worldblocks10.fixture(true);Mode_worldblocks10.raw(w,8,64,8,id,3);TileEntity old=w.getBlockTileEntity(8,64,8);if(invalid&&old!=null)old.invalidate();w.multiplayerWorld=remote;
                    if(hook==0)add.invokeWithArguments(b,w,8,64,8);if(hook==1)remove.invokeWithArguments(b,w,8,64,8);if(hook==2)play.invokeWithArguments(b,w,8,64,8,event,value);
                    state(o,w,old);
                }
            }
        }
    }
}
