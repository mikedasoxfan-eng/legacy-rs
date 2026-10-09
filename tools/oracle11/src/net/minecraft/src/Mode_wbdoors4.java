package net.minecraft.src;
import java.io.*;
public final class Mode_wbdoors4 {
    public static void main(String[] a)throws Exception {
        int init=Block.stone.blockID;
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            for(int id:new int[]{64,71})for(int dir=0;dir<4;dir++)for(int mask=0;mask<81;mask++) {
                World w=Mode_worldblocks10.fixture(true);EntityPlayer p=PlayerOracle.player(w);p.rotationYaw=(dir-1)*90.0F;w.rand.setSeed(42);EntityOracle.mathRandom().setSeed(123);OracleHooks.reset(100);
                int dx=new int[]{0,-1,0,1}[dir],dz=new int[]{1,0,-1,0}[dir],v=mask;
                for(int[] s:new int[][]{{-dx,0,-dz},{-dx,1,-dz},{dx,0,dz},{dx,1,dz}}){Mode_worldblocks10.raw(w,8+s[0],64+s[1],8+s[2],new int[]{0,1,id}[v%3],0);v/=3;}
                ItemStack s=new ItemStack(id==64?324:330,4,0);Trace10.writeBoolean(o, "s.getItem().onItemUse(s,p,w,8,63,8,1)", s.getItem().onItemUse(s,p,w,8,63,8,1));EntityOracle.i(o,Trace10.namedInt("s.stackSize", s.stackSize));
                Mode_wbplacement4.cells(o,w,6,10,63,66);EntityOracle.l(o,Trace10.namedLong("EntityOracle.rngState(w.rand)", EntityOracle.rngState(w.rand)));EntityOracle.i(o,Trace10.namedInt("w.loadedEntityList.size()", w.loadedEntityList.size()));
            }
        }
    }
}
