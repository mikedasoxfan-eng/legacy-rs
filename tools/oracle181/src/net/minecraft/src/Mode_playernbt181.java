package net.minecraft.src;
import java.io.*;
// EntityPlayer NBT (b1.8.1): readFromNBT then writeToNBT for crafted players (XP, food, spawn,
// potion effects, armour, Nether dimension; a stale b1.7.3 "Score" tag is ignored).
// Usage: playernbt181 <out>: per case, the input then the output compound (uncompressed).
public final class Mode_playernbt181 {
    static NBTTagList doubles(double... v){NBTTagList l=new NBTTagList();for(double d:v)l.setTag(new NBTTagDouble(d));return l;}
    static NBTTagList floats(float... v){NBTTagList l=new NBTTagList();for(float f:v)l.setTag(new NBTTagFloat(f));return l;}
    static void bytes(DataOutputStream o,NBTTagCompound t)throws IOException{ByteArrayOutputStream b=new ByteArrayOutputStream();CompressedStreamTools.func_1139_a(t,new DataOutputStream(b));EntityOracle.i(o,b.size());o.write(b.toByteArray());}
    public static void main(String[] a)throws Exception {
        World w=Mode_entities181.world();
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            EntityOracle.i(o,6);
            for(int c=0;c<6;c++) {
                NBTTagCompound t=new NBTTagCompound();
                t.setTag("Pos",doubles(0.5+c,65.0+c*0.25,0.5-c));t.setTag("Motion",doubles(0.1*c,-0.0784,0.0));
                t.setTag("Rotation",floats(10.0F*c,-5.0F*c));t.setFloat("FallDistance",c*0.5F);
                t.setShort("Fire",(short)(c==2?5:-20));t.setShort("Air",(short)(300-c));t.setBoolean("OnGround",c%2==0);
                t.setShort("Health",(short)(20-c));t.setShort("HurtTime",(short)(c%3));t.setShort("DeathTime",(short)0);t.setShort("AttackTime",(short)c);
                if(c>=3){NBTTagList fx=new NBTTagList();for(int k=0;k<c-2;k++){NBTTagCompound e=new NBTTagCompound();e.setByte("Id",(byte)(k*3+1));e.setByte("Amplifier",(byte)k);e.setInteger("Duration",100*c+k);fx.setTag(e);}t.setTag("ActiveEffects",fx);}
                NBTTagList inv=new NBTTagList();
                int[][] items={{0,276,1,c*7},{1,3,64,0},{8,260,c+1,0},{20,17,12,c%4},{100,313,1,c*11},{103,86,1,0}};
                for(int[] it:items){NBTTagCompound s=new NBTTagCompound();s.setByte("Slot",(byte)it[0]);s.setShort("id",(short)it[1]);s.setByte("Count",(byte)it[2]);s.setShort("Damage",(short)it[3]);inv.setTag(s);}
                t.setTag("Inventory",inv);t.setInteger("Dimension",c==4?-1:0);t.setBoolean("Sleeping",false);t.setShort("SleepTimer",(short)c);
                t.setInteger("Xp",c*7);t.setInteger("XpLevel",c);t.setInteger("XpTotal",c*31);t.setInteger("Score",99);
                if(c%2==0){t.setInteger("SpawnX",c*5-3);t.setInteger("SpawnY",64+c);t.setInteger("SpawnZ",-c*9);}
                t.setInteger("foodLevel",20-c*3);t.setInteger("foodTickTimer",c*13);t.setFloat("foodSaturationLevel",c*0.75F);t.setFloat("foodExhaustionLevel",c*0.3F);
                PlayerOracle.HeadlessPlayer p=PlayerOracle.player(w);p.readFromNBT(t);
                NBTTagCompound out=new NBTTagCompound();p.writeToNBT(out);
                bytes(o,t);bytes(o,out);
            }
        }
    }
}
