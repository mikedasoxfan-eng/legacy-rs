package net.minecraft.src;
import java.io.*;import org.lwjgl.opengl.GL11;
public final class CreativeFogProbe11 {
 public static void main(String[] args)throws Exception {
  Oracle.DataOut out=new Oracle.DataOut();out.i32(0x46313152);out.i32(1);out.i32(504);
  for(int dimension:new int[]{0,-1,1})for(EnumWorldType type:EnumWorldType.values())for(int creative=0;creative<2;creative++)for(double y:new double[]{-8,0,1,4,16,32,64})for(float far:new float[]{32,256})for(int mode:new int[]{-1,0,1}) {
   WorldSettings settings=new WorldSettings(42,0,false,false,type);World w=new World(new Oracle.NullSaveHandler(42){public WorldInfo loadWorldInfo(){return new WorldInfo(settings,"oracle");}},"oracle",settings,WorldProvider.getProviderForDimension(dimension));w.chunkProvider=new EntityOracle.FlatProvider(w);
   PlayerOracle.HeadlessPlayer p=FirstPersonOracle.player(w,0);FirstPersonOracle.setup(p);p.capabilities.depleteBuckets=creative!=0;p.posY=p.lastTickPosY=y;EntityRenderer er=p.mc.entityRenderer;Oracle.setField(er,"farPlaneDistance",far);int light=p.getEntityBrightnessForRender(.5F);
   FirstPersonOracle.call(er,"setupFog",new Class[]{int.class,float.class},mode,.5F);
   out.i32(dimension);out.i32(type.ordinal());out.i32(creative);out.i64(Double.doubleToRawLongBits(y));Oracle.modelFloat(out,far);out.i32(mode);out.i32(light);out.i32(GL11.fogMode);Oracle.modelFloat(out,GL11.fogStart);Oracle.modelFloat(out,GL11.fogEnd);
  }
  try(OutputStream f=new FileOutputStream(args[0])){out.writeTo(f);}System.out.println("CreativeFog11:504 original setupFog calls");
 }
}

