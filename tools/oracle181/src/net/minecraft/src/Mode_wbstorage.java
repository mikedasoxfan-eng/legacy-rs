package net.minecraft.src;
import java.io.*;
public final class Mode_wbstorage {
    static void snap(DataOutputStream o,File f)throws Exception {
        NBTTagCompound n=CompressedStreamTools.func_35622_a(f);o.writeBoolean(n!=null);
        if(n!=null){EntityOracle.i(o,n.getInteger("value"));EntityOracle.l(o,n.getLong("long"));EntityOracle.i(o,n.getString("text").length());byte[] b=n.getByteArray("bytes");EntityOracle.i(o,b.length);o.write(b);EntityOracle.l(o,f.length());}
    }
    public static void main(String[] a)throws Exception {
        File dir=new File("wb181-storage-java");dir.mkdirs();File f=new File(dir,"plain.dat"),tmp=new File(f.getAbsolutePath()+"_tmp");f.delete();tmp.delete();
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            snap(o,f);
            for(int k=0;k<32;k++) {
                NBTTagCompound n=new NBTTagCompound();n.setInteger("value",k*997-1000);n.setLong("long",-123456789012345L+k);n.setString("text","plain NBT "+k);byte[] b=new byte[k*127];for(int j=0;j<b.length;j++)b[j]=(byte)(j*31+k);n.setByteArray("bytes",b);
                if(k%2==0)CompressedStreamTools.func_35620_b(n,f);else{new FileOutputStream(tmp).close();CompressedStreamTools.func_35621_a(n,f);}
                snap(o,f);o.writeBoolean(tmp.exists());
            }
            File bad=new File(dir,"bad.dat");try(DataOutputStream s=new DataOutputStream(new FileOutputStream(bad))){s.writeByte(1);s.writeUTF("");s.writeByte(7);}
            try{CompressedStreamTools.func_35622_a(bad);o.writeBoolean(false);}catch(IOException e){o.writeBoolean(true);}bad.delete();
            File target=new File(dir,"nonempty");target.mkdir();File child=new File(target,"child");new FileOutputStream(child).close();
            try{CompressedStreamTools.func_35621_a(new NBTTagCompound(),target);o.writeBoolean(false);}catch(IOException e){o.writeBoolean(true);}
            o.writeBoolean(target.exists());o.writeBoolean(new File(target.getAbsolutePath()+"_tmp").exists());child.delete();target.delete();new File(target.getAbsolutePath()+"_tmp").delete();
        }finally{f.delete();tmp.delete();dir.delete();}
    }
}
