package net.minecraft.src;
import java.io.*;
import java.lang.reflect.*;
import java.util.*;
/** Read-only original StringTranslate/Formatter corpus, including thrown exceptions. */
public final class LanguageFormatProbe11 {
    static String b64(String s)throws Exception{return Base64.getEncoder().encodeToString(s.getBytes("UTF-8"));}
    static void emit(PrintWriter out,StringTranslate tr,String code,String key,int variant,Object[] args)throws Exception {
        String status,value;
        try {value=tr.translateKeyFormat(key,args);status="OK";}catch(Exception ex){status=ex.getClass().getName();value=ex.getMessage();}
        out.println(code+"\t"+b64(key)+"\t"+variant+"\t"+b64(tr.translateKey(key))+"\t"+status+"\t"+b64(value));
    }
    public static void main(String[] args)throws Exception {
        Locale.setDefault(Locale.US);StringTranslate tr=StringTranslate.getInstance();
        Field table=StringTranslate.class.getDeclaredField("translateTable");table.setAccessible(true);
        Object[][] vectors={new Object[0],{"Aé"},{"Aé","Bß"},{"Aé","Bß","C中","D🙂"},{Integer.valueOf(7)}, {null}};
        try(PrintWriter out=new PrintWriter(new OutputStreamWriter(new FileOutputStream(args[0]),"UTF-8"))) {
            for(Object rawCode:tr.func_44022_b().keySet()) {
                String code=(String)rawCode;
                tr.func_44023_a(code);Properties props=(Properties)table.get(tr);
                for(String key:new TreeSet<String>(props.stringPropertyNames())) {
                    if(props.getProperty(key).indexOf('%')<0)continue;
                    for(int v=0;v<vectors.length;v++)emit(out,tr,code,key,v,vectors[v]);
                }
            }
            tr.func_44023_a("en_US");
            for(String fmt:new String[]{"%s","%d","%1$s %s %<s","%2$s %s","%0$s","%100$s","%S","%5s","%-5s","%.2s","%%","%n","100% accurate","%q","%","%s %s","%1$d","%+s","%1$s %1$s"}) {
                for(int v=0;v<vectors.length;v++)emit(out,tr,"synthetic-missing-key",fmt,v,vectors[v]);
            }
        }
    }
}
