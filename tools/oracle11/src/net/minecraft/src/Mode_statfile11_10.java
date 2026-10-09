package net.minecraft.src;
import java.io.*;import java.nio.file.*;import java.util.*;
/** Original constructor failures, malformed UTF-16 and Windows charset boundaries. */
public final class Mode_statfile11_10 {
 static void text(DataOutputStream o,String s)throws Exception{byte[] b=s.getBytes("UTF-8");Trace10.writeInt(o, "b.length", b.length);Trace10.writeBytes(o, "b", b);}
 static void units(DataOutputStream o,String s)throws Exception{Trace10.writeInt(o, "s.length()", s.length());for(int i=0;i<s.length();i++)o.writeChar(s.charAt(i));}
 static String parse(String s){try{return Mode_statfile10_10.map(StatFileWriter.func_27177_a(s));}catch(RuntimeException e){return "throw:"+e.getClass().getSimpleName();}}
 static String bounded(final String s)throws Exception{
  final String[] result={null};Thread t=new Thread(()->{result[0]=parse(s);});t.setDaemon(true);t.start();t.join(80);
  if(t.isAlive()){t.stop();t.join();return "throw:Nonterminating";}return result[0];
 }
 static void make(File d,int c)throws Exception {
  if(c==0)return;if(c==1){Files.write(d.toPath(),new byte[]{1});return;}d.mkdirs();
  if(c==2)Files.write(new File(d,"stats").toPath(),new byte[]{2});
  if(c>=3){Files.write(new File(d,"stats_Player_unsent.dat").toPath(),"{}".getBytes("US-ASCII"));Files.write(new File(d,"stats_other.dat").toPath(),new byte[]{3});Files.write(new File(d,"stats_other.tmp").toPath(),new byte[]{4});new File(d,"stats_directory.dat").mkdir();}
  if(c==4||c==5){new File(d,"stats").mkdirs();Files.write(new File(d,"stats/stats_other.dat").toPath(),new byte[]{9});}
  if(c==5)new File(d,"stats/stats_directory.dat").mkdir();
 }
 static String disk(File d)throws Exception {
  TreeMap<String,String> entries=new TreeMap<>();scan(d,"",entries);StringBuilder b=new StringBuilder();for(Map.Entry<String,String> e:entries.entrySet())b.append(e.getKey()).append('=').append(e.getValue()).append(';');return b.toString();
 }
 static void scan(File f,String path,Map<String,String> m)throws Exception{
  if(!f.exists()){m.put(path,"missing");return;}if(!f.isDirectory()){m.put(path,Arrays.toString(Files.readAllBytes(f.toPath())));return;}
  m.put(path,"dir");for(File child:f.listFiles())scan(child,path+"/"+child.getName(),m);
 }
 public static void main(String[] a)throws Exception{
  Block.stone.getClass();Item.shovelSteel.getClass();
  File root=new File("statfile11-"+System.nanoTime());root.mkdir();
  ArrayList<String> cases=new ArrayList<>();String valid=Mode_statfile10_10.valid("[{\"1000\":7}]");
  for(int i=0;i<valid.length();i++)cases.add(valid.substring(0,i));
  for(int i=0;i<256;i++){cases.add(valid.replace("7","\""+(char)i+"\""));cases.add(valid+ (char)i);cases.add("{\"ignored\":\"\\u"+String.format("%04x",i)+"\",\"stats-change\":[],\"checksum\":\"d41d8cd98f00b204e9800998ecf8427e\"}");}
  for(String s:new String[]{"[\"", "{\"", "{\"a\":\"", "[\"\\", "[\"\\u", "[\"\\u0", "[\"\\u000", "[\"\\q", "[1E+", "[1.2.3]", "[1e2]", "[01]", "[true,]", "{\"a\":1,}", "[\"\ud800\"]","[\"\udfff\"]"})cases.add(s);
  Random r=new Random(18111);String[] tokens={"true","false","null","0","-1","2147483648","1E2","1e2","[]","{}","\"7\"","\"\\u+037\"","\"\\u-037\"","\"\\uff17\"","\"\\u????\"","\"\ud800\""};
  for(int i=0;i<3000;i++)cases.add("{\"stats-change\":[{\""+(i%3==0?"bad":"1000")+"\":"+tokens[r.nextInt(tokens.length)]+"}],\"checksum\":"+tokens[r.nextInt(tokens.length)]+"}");
  try(DataOutputStream o=new DataOutputStream(new FileOutputStream(a[1]))){
   text(o,System.getProperty("file.encoding"));Trace10.writeInt(o, "cases.size()", cases.size());
   for(String s:cases){units(o,s);text(o,bounded(s));}
   Trace10.writeInt(o, "10", 10);
   for(int c=0;c<10;c++){
    File d=new File(root,"case"+c);make(d,c%6);Trace10.writeInt(o, "c%6", c%6);Trace10.writeBoolean(o, "c<6", c<6);String status;
    try{StatFileWriter w=new StatFileWriter(c<6?new Session("Player","-"):null,d);Mode_statsync10.idle(w);status="ok";}catch(RuntimeException e){status=e.getClass().getSimpleName();}
    text(o,status);text(o,disk(d));
   }
   String[] salts={"local","\u00e9","\u20ac","\u0100","\ud800","\udfff","\ud83d\ude00","a\ud83d\ude00b"};
   Trace10.writeInt(o, "salts.length", salts.length);for(String salt:salts){units(o,salt);try{text(o,new MD5String(salt).func_27369_a("abc"));}catch(RuntimeException e){text(o,"throw:"+e.getClass().getSimpleName());}}
   Trace10.writeInt(o, "128", 128);for(int c=0;c<128;c++){
    String user="user"+(char)(c*521)+(c%5==0?"\ud83d\ude00":"");String session=salts[c%salts.length];
    units(o,user);units(o,session);Map values=new LinkedHashMap();values.put(StatList.getOneShotStat(1000),c);values.put(StatList.getOneShotStat(5242880),-c);
    String result;try{String formatted=StatFileWriter.func_27185_a(user,session,values);result="ok";text(o,result);units(o,formatted);byte[] raw=formatted.getBytes();Trace10.writeInt(o, "raw.length", raw.length);Trace10.writeBytes(o, "raw", raw);}catch(RuntimeException e){text(o,e.getClass().getSimpleName());}
   }
   Trace10.writeInt(o, "256", 256);for(int b=0;b<256;b++){byte[] raw={(byte)b,13,10,(byte)b,13,(byte)b,10};Trace10.writeInt(o, "raw.length", raw.length);Trace10.writeBytes(o, "raw", raw);StringBuilder s=new StringBuilder();try(BufferedReader in=new BufferedReader(new InputStreamReader(new ByteArrayInputStream(raw)))){String l;while((l=in.readLine())!=null)s.append(l);}units(o,s.toString());}
  }
  System.out.println("STATFILE11 original: "+cases.size()+" malformed/UTF16, 10 constructors, 8 MD5, 256 decoder cases");
 }
}
