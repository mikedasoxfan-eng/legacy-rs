package net.minecraft.src;
import java.io.*;
/** Actual ItemBlock/World/BlockPistonMoving bytecode, including malformed tiles. */
public final class Mode_itempiston11_10 {
 public static void main(String[] a)throws Exception {
  int init=Block.stone.blockID;
  try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[3])))) {
   for(int x:new int[]{0,15})for(int y:new int[]{-1,0,64,127,128,256,32768})for(int side:new int[]{-1,0,1,5,6})for(int tile=0;tile<4;tile++)for(int stored:new int[]{0,1,6,36,51,85,255})for(int facing:new int[]{-1,0,1,2,3,4,5,6,7,Integer.MAX_VALUE}) {
    World w=Mode_entities10.world();EntityPlayer player=new Mode_entities10.Player(w);player.setPosition(8.5,64,8.5);
    Mode_itemdoor6_10.raw(w,x,y,0,36);
    int tx=x,ty=y,tz=0;if(side==0)--ty;if(side==1)++ty;if(side==5)++tx;
    TileEntity te=null;
    if(tile>0&&ty>=0&&ty<128){
     Mode_itemdoor6_10.raw(w,tx,ty,tz,36);
     te=tile==1?new TileEntitySign():new TileEntityPiston(stored,0,facing,(stored&1)!=0,false);
     if(te instanceof TileEntityPiston){NBTTagCompound n=new NBTTagCompound();te.writeToNBT(n);n.setFloat("progress",.5F);te.readFromNBT(n);}
     w.setBlockTileEntity(tx,ty,tz,te);if(tile==3)te.invalidate();
    }
    ItemStack stack=new ItemStack(36,1,0);boolean result=false;String error="";
    try{result=stack.getItem().onItemUse(stack,player,w,x,y,0,side);}catch(RuntimeException e){error=e.getClass().getSimpleName();}
    Trace10.writeBoolean(o, "result", result);WorldEventOracle.text(o,error);EntityOracle.i(o,Trace10.namedInt("stack.stackSize", stack.stackSize));Trace10.writeBoolean(o, "te!=null&&te.isInvalid()", te!=null&&te.isInvalid());
    Trace10.writeBoolean(o, "w.getChunkFromChunkCoords(tx>>4,tz>>4).chunkTileEntityMap.containsKey(new ChunkPosition(tx&15,ty,tz&15))", w.getChunkFromChunkCoords(tx>>4,tz>>4).chunkTileEntityMap.containsKey(new ChunkPosition(tx&15,ty,tz&15)));
    EntityOracle.l(o,Trace10.namedLong("EntityOracle.rngState(w.rand)", EntityOracle.rngState(w.rand)));WorldEventOracle.snapshot(w,o);
   }
  }
 }
}
