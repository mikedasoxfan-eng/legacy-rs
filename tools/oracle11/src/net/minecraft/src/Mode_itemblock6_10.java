package net.minecraft.src;
import java.io.*;import java.util.*;
/** Registered block-item placements, full height/face/replacement/collision matrix. */
public final class Mode_itemblock6_10 {
 public static void main(String[] a)throws Exception {
  // Moving-piston blocks need a live piston tile; malformed tile/height exceptions belong to World.
  int init=Block.stone.blockID;ArrayList<Integer> ids=new ArrayList<Integer>();for(int id=1;id<256;id++)if(id!=36 && Item.itemsList[id] instanceof ItemBlock)ids.add(id);
  if(a[1].equals("blockbasic6"))ids=new ArrayList<Integer>(Arrays.asList(1,4,5,20,35));
  try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[3])))) {
   EntityOracle.i(o,Trace10.namedInt("ids.size()", ids.size()));for(int id:ids)EntityOracle.i(o,id);
   for(int id:ids)for(int scene=0;scene<6;scene++)for(int y:new int[]{-1,0,64,126,127,128})for(int side:new int[]{-1,0,1,2,3,4,5,6}) {
    World w=Mode_entities10.world();EntityPlayer p=new Mode_entities10.Player(w);p.setPosition(8.5,64,8.5);p.rotationYaw=35;p.rotationPitch=-17;
    for(int x=-2;x<=2;x++)for(int z=-2;z<=2;z++){Mode_itemdoor6_10.raw(w,x,y-1,z,1);Mode_itemdoor6_10.raw(w,x,y-2,z,1);}
    Mode_itemdoor6_10.raw(w,0,y,0,new int[]{0,78,106,1,9,0}[scene]);
    if(scene==5){EntityPig pig=new EntityPig(w);pig.setPosition(.5,y+1,.5);w.spawnEntityInWorld(pig);}
    ItemStack stack=new ItemStack(id,scene==4?0:scene==5?Integer.MIN_VALUE:1,scene==1?-1:scene==2?15:0);
    Trace10.writeBoolean(o, "stack.getItem().onItemUse(stack,p,w,0,y,0,side)", stack.getItem().onItemUse(stack,p,w,0,y,0,side));EntityOracle.i(o,Trace10.namedInt("stack.stackSize", stack.stackSize));EntityOracle.l(o,Trace10.namedLong("EntityOracle.rngState(w.rand)", EntityOracle.rngState(w.rand)));EntityOracle.l(o,Trace10.namedLong("EntityOracle.rngState(EntityOracle.mathRandom())", EntityOracle.rngState(EntityOracle.mathRandom())));
    for(int x=-1;x<=1;x++)for(int z=-1;z<=1;z++)for(int yy=y-2;yy<=y+2;yy++){Trace10.writeByte(o, "w.getBlockId(x,yy,z)", w.getBlockId(x,yy,z));EntityOracle.i(o,Trace10.namedInt("w.getBlockMetadata(x,yy,z)", w.getBlockMetadata(x,yy,z)));}
    WorldEventOracle.snapshot(w,o);
   }
  }
 }
}
