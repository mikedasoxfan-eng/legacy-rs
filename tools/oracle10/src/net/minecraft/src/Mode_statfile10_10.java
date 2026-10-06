package net.minecraft.src;
import java.io.*;
import java.util.*;

/** Calls original static parsing/serialization. Malformed schema exceptions are
 * observed at the API boundary, independently of StatsSyncher's file catch. */
public final class Mode_statfile10_10 {
    static String map(Map m) {
        if(m==null)return "null";
        TreeMap<Integer,Integer> sorted=new TreeMap<>();
        for(Object k:m.keySet())sorted.put(((StatBase)k).statId,(Integer)m.get(k));
        StringBuilder b=new StringBuilder();
        for(Map.Entry<Integer,Integer> e:sorted.entrySet())b.append(e.getKey()).append('=').append(e.getValue()).append(';');
        return b.toString();
    }
    static void text(DataOutputStream o,String s)throws Exception {
        byte[] b=s.getBytes("UTF-8");Trace10.writeInt(o, "b.length", b.length);Trace10.writeBytes(o, "b", b);
    }
    static String valid(String changes)throws Exception {
        // Build only the checksum input; actual parsing is always the original method.
        List nodes=new J_JdomParser().parse("{\"stats-change\":"+changes+"}").getArrayNode("stats-change");
        StringBuilder sum=new StringBuilder();
        for(Object obj:nodes) {
            Map f=((J_JsonNode)obj).getFields();
            Map.Entry e=(Map.Entry)f.entrySet().iterator().next();
            int id=Integer.parseInt(((J_JsonStringNode)e.getKey()).getText());
            int value=Integer.parseInt(((J_JsonNode)e.getValue()).getText());
            StatBase stat=StatList.func_27361_a(id);
            if(stat!=null)sum.append(stat.statGuid).append(',').append(value).append(',');
        }
        return "{\"stats-change\":"+changes+",\"checksum\":\""+new MD5String("local").func_27369_a(sum.toString())+"\"}";
    }
    public static void main(String[] a)throws Exception {
        Block.stone.getClass();Item.shovelSteel.getClass();
        ArrayList<String> cases=new ArrayList<>();
        String empty=valid("[]"), one=valid("[{\"1000\":7}]");
        Collections.addAll(cases,empty,one," "+one,"\t"+one,one+"\n",one+"x","null","0","[]","{}",
            "{\"stats-change\":{}}","{\"stats-change\":[{}]}","{\"stats-change\":[7]}",
            "{\"stats-change\":[null]}","{\"stats-change\":[{\"1000\":null}]}",
            "{\"stats-change\":[{\"1000\":true}]}","{\"stats-change\":[{\"1000\":[]}]}",
            "{\"stats-change\":[{\"x\":1}]}","{\"stats-change\":[{\"1000\":2147483648}]}",
            "{\"stats-change\":[{\"1000\":-2147483649}]}","{\"stats-change\":[{\"1000\":01}]}",
            "{\"stats-change\":[{\"1000\":1.}]}","{\"stats-change\":[{\"1000\":1e2}]}",
            "{\"stats-change\":[{\"1000\":1E2}]}","{\"stats-change\":[{\"1000\":1.2.3}]}",
            "{\"stats-change\":[{\"1000\":\"\\q\"}]}","{\"stats-change\":[],\"checksum\":123}",
            "{\"stats-change\":[],\"checksum\":null}","{\"stats-change\":[],\"checksum\":\"wrong\"}",
            "{\"stats-change\":[],\"checksum\":\"\\ud800\"}",one.replace("7","\"+7\""),
            one.replace("7","\" 7\""),one.replace("7","\"\u0667\""));
        for(String value:new String[]{"0","-0","2147483647","-2147483648","\"+7\"","\"0007\"","\"\u0667\"","\"\uff17\""})
            cases.add(valid("[{\"1000\":"+value+"}]"));
        cases.add(valid("[{\"1000\":1,\"1000\":9}]"));
        cases.add(valid("[{\"1000\":1},{\"1000\":9}]"));
        cases.add(valid("[{\"999\":4},{\"1000\":2}]"));
        cases.add(empty.replace("\"stats-change\":[]","\"stats-change\":7,\"stats-change\":[]"));
        cases.add(one.replace("7","\"\\u0037\""));
        cases.add(one.replace("7","\"\\u+037\""));
        cases.add(one.replace("7","\"\\u-037\""));
        cases.add(one.replace("7","\"\\u????\""));
        cases.add(empty.replace("\"checksum\":", "\"ignored\":\"literal\ncontrol\",\"checksum\":"));
        Random r=new Random(18110);int[] ids={1000,1001,1100,2000,2020,5242880,5242881,16777217,16842753,999};
        for(int c=0;c<3000;c++) {
            StringBuilder changes=new StringBuilder("[");
            int n=r.nextInt(20);
            for(int i=0;i<n;i++) {
                if(i>0)changes.append(',');changes.append('{');
                int fields=1+r.nextInt(20);
                for(int f=0;f<fields;f++) {
                    if(f>0)changes.append(',');
                    changes.append('"').append(ids[r.nextInt(ids.length)]).append("\":").append(r.nextInt());
                }
                changes.append('}');
            }
            changes.append(']');cases.add(valid(changes.toString()));
        }
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            Trace10.writeInt(o, "cases.size()", cases.size());
            for(String input:cases) {
                text(o,input);
                try {text(o,map(StatFileWriter.func_27177_a(input)));}
                catch(RuntimeException e){text(o,"throw:"+e.getClass().getSimpleName());}
            }
            Trace10.writeInt(o, "1000", 1000);
            for(int c=0;c<1000;c++) {
                String user=c%4==0?null:c%4==1?"Player":c%4==2?"quoted\"\\name":"line\nbreak";
                String session=c%3==0?null:c%3==1?"local":"-";
                text(o,user==null?"null":"value:"+user);text(o,session==null?"null":"value:"+session);
                Map entries=new LinkedHashMap();int n=r.nextInt(80);
                for(int i=0;i<n;i++)entries.put(StatList.func_27361_a(ids[r.nextInt(ids.length-1)]),c%7==0?null:r.nextInt());
                Trace10.writeInt(o, "entries.size()", entries.size());
                for(Object key:entries.keySet()) {
                    StatBase stat=(StatBase)key;Trace10.writeInt(o, "stat.statId", stat.statId);
                    Object value=entries.get(key);Trace10.writeBoolean(o, "value!=null", value!=null);if(value!=null)Trace10.writeInt(o, "(Integer)value", (Integer)value);
                }
                text(o,StatFileWriter.func_27185_a(user,session,entries));
            }
        }
        System.out.println("STATFILE10 original parser: "+cases.size()+" cases");
    }
}
