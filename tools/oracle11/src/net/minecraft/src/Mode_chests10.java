package net.minecraft.src;
import java.io.*;import java.util.*;
public final class Mode_chests10 {
 static class Mode_entities10Field {static Object get(Object o,String n)throws Exception{java.lang.reflect.Field f=o.getClass().getDeclaredField(n);f.setAccessible(true);return f.get(o);}}
 static final class Chest extends TileEntityChest {
  int metadata; public int getBlockMetadata(){return metadata;}
  public Block getBlockType(){return Block.chest;}
  public void checkForAdjacentChests(){}
 }
 public static void main(String[] a)throws Exception {
  World w=Mode_entities10.world();RenderManager manager=RendererOracle.manager(w);
  Map textures=(Map)Mode_entities10Field.get(manager.renderEngine,"textureMap");
  textures.put("/item/chest.png",31);textures.put("/item/largechest.png",32);
  TileEntityRenderer.instance.renderEngine=manager.renderEngine;
  RendererOracle.field("convertQuadsToTriangles").setBoolean(null,false);
  TileEntityChestRenderer renderer=new TileEntityChestRenderer();renderer.setTileEntityRenderer(TileEntityRenderer.instance);
  Oracle.DataOut out=new Oracle.DataOut();out.i32(16*8*8*4+1);
  for(int adj=0;adj<16;adj++)for(int meta=0;meta<8;meta++)for(int pose=0;pose<8;pose++)for(float partial:new float[]{0,.25F,.75F,1}) {
   Chest chest=new Chest();chest.worldObj=w;chest.metadata=meta;chest.lidAngle=pose/7.0F;chest.prevLidAngle=(7-pose)/7.0F;
   if((adj&1)!=0)chest.adjacentChestZNeg=new TileEntityChest();if((adj&2)!=0)chest.adjacentChestXPos=new TileEntityChest();
   if((adj&4)!=0)chest.adjacentChestXNeg=new TileEntityChest();if((adj&8)!=0)chest.adjacentChestZPos=new TileEntityChest();
   out.i32(adj);out.i32(meta);Oracle.modelFloat(out,chest.lidAngle);Oracle.modelFloat(out,chest.prevLidAngle);Oracle.modelFloat(out,partial);
   RendererOracle.reset(1);RendererOracle.active=true;renderer.renderTileEntityAt(chest,-.25,10.5,3.75,partial);RendererOracle.active=false;
   out.i32(RendererOracle.count);out.bytes(RendererOracle.draws.buf.toByteArray());
  }
  out.i32(-1);out.i32(0);for(int i=0;i<3;i++)Oracle.modelFloat(out,0);
  RendererOracle.reset(1);RendererOracle.active=true;ChestItemRenderHelper.instance.func_35609_a(Block.chest,15,.3F);RendererOracle.active=false;
  out.i32(RendererOracle.count);out.bytes(RendererOracle.draws.buf.toByteArray());
  try(OutputStream f=new FileOutputStream(a[1])){out.writeTo(f);}
 }
}
