package net.minecraft.src;
import java.io.*;
import java.util.*;

public final class Mode_r10world3misc {
    public static void main(String[] a)throws Exception {
        long seed=Long.parseLong(a[1]);Random r=new Random(seed);
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[2])))) {
            for(int[] row:new int[][]{Direction.field_35871_a,Direction.field_35870_b,Direction.headInvisibleFace,Direction.field_35869_d,
                Direction.footInvisibleFaceRemap,Direction.field_35867_f,Direction.field_35868_g})for(int n:row)o.writeInt(n);
            for(int[] row:Direction.bedDirection)for(int n:row)o.writeInt(n);
            for(int[] row:new int[][]{Facing.field_31057_a,Facing.offsetsXForSide,Facing.offsetsYForSide,Facing.offsetsZForSide})for(int n:row)o.writeInt(n);
            for(String name:new String[]{"","map_0","\ud83d\ude00","\ue000"}) {
                WorldSavedData s=new WorldSavedData(name){public void readFromNBT(NBTTagCompound n){}public void writeToNBT(NBTTagCompound n){}};
                o.writeUTF(s.mapName);o.writeBoolean(s.isDirty());s.markDirty();o.writeBoolean(s.isDirty());s.setDirty(false);o.writeBoolean(s.isDirty());s.setDirty(true);o.writeBoolean(s.isDirty());
                for(boolean hardcore:new boolean[]{false,true})o.writeBoolean(new SaveFormatComparator(name,"display",seed,0,3,true,hardcore).isHardcoreModeEnabled());
            }
            for(int j=0;j<1024;j++) {
                int[] v=new int[6];for(int k=0;k<6;k++)v[k]=r.nextInt();ChunkPosition p=new ChunkCoordIntPair(v[0],v[1]).func_40737_a(v[2]);o.writeInt(p.x);o.writeInt(p.y);o.writeInt(p.z);
                StructureBoundingBox b=new StructureBoundingBox(v[0],v[1],v[2],v[3],v[4],v[5]);o.writeInt(b.func_40597_e());o.writeInt(b.func_40596_f());o.writeInt(b.func_40598_g());
            }
            o.writeLong(EntityOracle.rngState(r));
        }
    }
}
