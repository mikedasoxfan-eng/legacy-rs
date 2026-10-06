package net.minecraft.src;
import java.io.*;
public final class Mode_wbstorage {
    static void snap(DataOutputStream o,File f)throws Exception {
        NBTTagCompound n=CompressedStreamTools.func_35622_a(f);Trace10.writeBoolean(o, "n!=null", n!=null);
        if(n!=null){EntityOracle.i(o,Trace10.namedInt("n.getInteger(\"value\")", n.getInteger("value")));EntityOracle.l(o,Trace10.namedLong("n.getLong(\"long\")", n.getLong("long")));EntityOracle.i(o,Trace10.namedInt("n.getString(\"text\").length()", n.getString("text").length()));byte[] b=n.getByteArray("bytes");EntityOracle.i(o,Trace10.namedInt("b.length", b.length));Trace10.writeBytes(o, "b", b);EntityOracle.l(o,Trace10.namedLong("f.length()", f.length()));}
    }
    public static void main(String[] a)throws Exception {
        File dir=new File("wb10-storage-java");dir.mkdirs();File f=new File(dir,"plain.dat"),tmp=new File(f.getAbsolutePath()+"_tmp");f.delete();tmp.delete();
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            snap(o,f);
            for(int k=0;k<32;k++) {
                NBTTagCompound n=new NBTTagCompound();n.setInteger("value",k*997-1000);n.setLong("long",-123456789012345L+k);n.setString("text","plain NBT "+k);byte[] b=new byte[k*127];for(int j=0;j<b.length;j++)b[j]=(byte)(j*31+k);n.setByteArray("bytes",b);
                if(k%2==0)CompressedStreamTools.func_35620_b(n,f);else{new FileOutputStream(tmp).close();CompressedStreamTools.func_35621_a(n,f);}
                snap(o,f);Trace10.writeBoolean(o, "tmp.exists()", tmp.exists());
            }
            File bad=new File(dir,"bad.dat");try(DataOutputStream s=new DataOutputStream(new FileOutputStream(bad))){Trace10.writeByte(s, "1", 1);s.writeUTF("");Trace10.writeByte(s, "7", 7);}
            try{CompressedStreamTools.func_35622_a(bad);Trace10.writeBoolean(o, "false", false);}catch(IOException e){Trace10.writeBoolean(o, "true", true);}bad.delete();
            File target=new File(dir,"nonempty");target.mkdir();File child=new File(target,"child");new FileOutputStream(child).close();
            try{CompressedStreamTools.func_35621_a(new NBTTagCompound(),target);Trace10.writeBoolean(o, "false", false);}catch(IOException e){Trace10.writeBoolean(o, "true", true);}
            Trace10.writeBoolean(o, "target.exists()", target.exists());Trace10.writeBoolean(o, "new File(target.getAbsolutePath()+\"_tmp\").exists()", new File(target.getAbsolutePath()+"_tmp").exists());child.delete();target.delete();new File(target.getAbsolutePath()+"_tmp").delete();
        }finally{f.delete();tmp.delete();dir.delete();}
    }
}
