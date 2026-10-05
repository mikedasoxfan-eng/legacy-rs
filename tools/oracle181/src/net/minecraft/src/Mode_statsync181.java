package net.minecraft.src;

import java.io.*;
import java.lang.reflect.*;
import java.nio.file.*;
import java.util.*;

/** Real StatFileWriter/StatsSyncher and both original worker threads, with barriers
 * only at observation points so disk timing cannot change the trace. */
public final class Mode_statsync181 {
    static Object field(Object o, String name) throws Exception {
        Field f=o.getClass().getDeclaredField(name); f.setAccessible(true); return f.get(o);
    }
    static void set(Object o,String name,Object value)throws Exception {
        Field f=o.getClass().getDeclaredField(name);f.setAccessible(true);f.set(o,value);
    }
    static void idle(StatFileWriter w) throws Exception {
        StatsSyncher s=(StatsSyncher)field(w,"statsSyncher");
        while((Boolean)field(s,"isBusy")) Thread.sleep(1);
    }
    static Map values(int n) {
        Map map=new LinkedHashMap();
        map.put(StatList.func_27361_a(1000),n);
        map.put(StatList.func_27361_a(5242880),0);
        return map;
    }
    static void write(File f, Map m) throws Exception {
        try(PrintWriter p=new PrintWriter(f)) { p.print(StatFileWriter.func_27185_a("MiXeD","local",m)); }
    }
    static String map(Map m) {
        if(m==null)return "null";
        TreeMap<Integer,Integer> sorted=new TreeMap<>();
        for(Object key:m.keySet())sorted.put(((StatBase)key).statId,(Integer)m.get(key));
        StringBuilder b=new StringBuilder();
        for(Map.Entry<Integer,Integer> e:sorted.entrySet())b.append(e.getKey()).append('=').append(e.getValue()).append(';');
        return b.toString();
    }
    static String disk(File f) throws Exception {
        if(!f.exists())return "missing";
        if(f.isDirectory())return "directory";
        String text=new String(Files.readAllBytes(f.toPath()),"UTF-8");
        return map(StatFileWriter.func_27177_a(text));
    }
    static void snapshot(PrintWriter p,StatFileWriter w,File dir) throws Exception {
        idle(w);
        StatsSyncher s=(StatsSyncher)field(w,"statsSyncher");
        p.print(s.func_27420_b()+"|"+map((Map)field(w,"field_25102_a"))+"|"+map(w.func_27176_a()));
        for(String name:new String[]{"stats_mixed_unsent.dat","stats_mixed_unsent.old","stats_mixed_unsent.tmp","stats_mixed.dat"})p.print("|"+disk(new File(dir,"stats/"+name)));
        p.print('|');
        for(Object o:AchievementList.achievementList){Achievement a=(Achievement)o;p.print(w.hasAchievementUnlocked(a)+","+w.canUnlockAchievement(a)+";");}
        p.println();
    }
    public static void main(String[] args) throws Exception {
        Block.stone.getClass(); Item.shovelSteel.getClass();
        File root=new File(args[1]);root.mkdirs();
        int[] ids={1000,1001,1100,2000,2020,5242880,5242881,5242882,5242896,16777217,16842753};
        try(PrintWriter p=new PrintWriter(new File(root,"expected.txt"),"UTF-8")) {
            for(int scenario=0;scenario<6;scenario++) {
                File input=new File(root,"input"+scenario);new File(input,"stats").mkdirs();
                if(scenario==1){write(new File(input,"stats_MiXeD_unsent.dat"),values(7));write(new File(input,"stats/stats_MiXeD.dat"),values(41));}
                if(scenario==2){write(new File(input,"stats/stats_mixed_unsent.old"),values(13));write(new File(input,"stats/stats_mixed.old"),values(19));}
                if(scenario==3){write(new File(input,"stats/stats_mixed_unsent.dat"),values(Integer.MAX_VALUE));write(new File(input,"stats/stats_mixed_unsent.old"),values(17));write(new File(input,"stats/stats_mixed_unsent.tmp"),values(23));write(new File(input,"stats/stats_mixed.dat"),values(Integer.MIN_VALUE));}
                if(scenario==4){try(PrintWriter q=new PrintWriter(new File(input,"stats/stats_mixed_unsent.dat"))){q.print("{invalid json");}}
                if(scenario==5){write(new File(input,"stats/stats_mixed_unsent.dat"),values(29));java.nio.file.Path f=new File(input,"stats/stats_mixed_unsent.dat").toPath();String text=new String(Files.readAllBytes(f),"UTF-8").replace("\"checksum\":\"","\"checksum\":\"BAD");Files.write(f,text.getBytes("UTF-8"));}
                File dir=new File(root,"java"+scenario);new File(dir,"stats").mkdirs();
                for(File f:input.listFiles()){if(f.isDirectory()){for(File g:f.listFiles())Files.copy(g.toPath(),new File(dir,"stats/"+g.getName()).toPath(),StandardCopyOption.REPLACE_EXISTING);}else Files.copy(f.toPath(),new File(dir,f.getName()).toPath(),StandardCopyOption.REPLACE_EXISTING);}
                StatFileWriter w=new StatFileWriter(new Session("MiXeD","-"),dir);
                snapshot(p,w,dir);Random r=new Random(181+scenario);
                for(int t=0;t<350;t++) {
                    if(t==210){idle(w);new File(dir,"stats/stats_mixed_unsent.tmp").mkdir();w.syncStats();snapshot(p,w,dir);}
                    if(t==211){idle(w);new File(dir,"stats/stats_mixed_unsent.tmp").delete();}
                    if(t==110) {
                        idle(w);StatsSyncher s=(StatsSyncher)field(w,"statsSyncher");
                        set(s,"field_27437_b",values(83+scenario));
                        set(s,"field_27436_c",values(-11));
                        s.beginReceiveStats();snapshot(p,w,dir);
                    }
                    int id=ids[r.nextInt(ids.length)],n=r.nextInt();
                    if(t%3==0)w.readStat(StatList.func_27361_a(id),n);
                    if(t%47==0)w.syncStats();
                    w.func_27178_d();snapshot(p,w,dir);
                }
                w.syncStats();snapshot(p,w,dir);
            }
        }
    }
}
