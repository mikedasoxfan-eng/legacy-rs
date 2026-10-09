package net.minecraft.src;
import java.io.*;
import java.lang.reflect.*;
import java.util.*;
/** All installed language maps/flags, executed on unchanged StringTranslate bytecode. */
public final class LanguageProbe11 {
    static void text(DataOutputStream o,String s)throws IOException {o.writeInt(s.length());for(int i=0;i<s.length();i++)o.writeChar(s.charAt(i));}
    public static void main(String[] args)throws Exception {
        StringTranslate tr=StringTranslate.getInstance();
        Field table=StringTranslate.class.getDeclaredField("translateTable");table.setAccessible(true);
        try(DataOutputStream o=new DataOutputStream(new FileOutputStream(args[0]))) {
            TreeMap<String,String> langs=tr.func_44022_b();o.writeInt(langs.size());
            for(Map.Entry<String,String> e:langs.entrySet()){text(o,e.getKey());text(o,e.getValue());}
            ArrayList<String> codes=new ArrayList<String>(langs.keySet());codes.add("zz_probe");codes.add("en_US");
            for(String code:codes) {
                tr.func_44023_a(code);text(o,tr.func_44024_c());o.writeBoolean(tr.func_46110_d());o.writeBoolean(StringTranslate.func_46109_d(code));
                Properties props=(Properties)table.get(tr);TreeMap<String,String> sorted=new TreeMap<String,String>();
                for(String key:props.stringPropertyNames())sorted.put(key,props.getProperty(key));
                o.writeInt(sorted.size());for(Map.Entry<String,String> e:sorted.entrySet()){text(o,e.getKey());text(o,e.getValue());}
            }
        }
    }
}
