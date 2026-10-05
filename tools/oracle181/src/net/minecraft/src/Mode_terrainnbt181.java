package net.minecraft.src;
import java.io.*;
import java.util.*;
/** Exact compound serialization through original NBTTagCompound, including tree bins. */
public final class Mode_terrainnbt181 {
    public static void main(String[] a) throws Exception {
        Random r=new Random(Long.parseLong(a[1]));
        ArrayList<String> keys=new ArrayList<String>(Arrays.asList("xPos","zPos","LastUpdate","Blocks","Data","SkyLight","BlockLight","HeightMap","TerrainPopulated","Entities","TileEntities","id","x","y","z","Items","Slot","Count","Damage","EntityId","Delay","Pos","Motion","Rotation","FallDistance","Fire","Air","OnGround","Health","HurtTime","DeathTime","AttackTime","Sheared","Color"));
        for(int i=0;i<64;i++){String s="";for(int b=0;b<6;b++)s+=(i&(1<<b))==0?"Aa":"BB";keys.add(s);}
        try(DataOutputStream out=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[2])))) {
            for(int n=0;n<8;n++) {NBTTagCompound c=new NBTTagCompound();ArrayList<String> order=new ArrayList<String>(keys);Collections.shuffle(order,r);for(int i=0;i<order.size();i++)c.setInteger(order.get(i),i);for(int i=0;i<12;i++)c.setInteger(order.get(i),r.nextInt());c.setTag("Empty",new NBTTagList());NBTTagCompound root=new NBTTagCompound();root.setCompoundTag("Level",c);NBTBase.writeTag(root,out);}
            out.writeLong(EntityOracle.rngState(r));
        }
    }
}
