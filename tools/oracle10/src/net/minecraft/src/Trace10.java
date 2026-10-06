package net.minecraft.src;

import java.io.DataOutputStream;
import java.io.ByteArrayOutputStream;
import java.io.OutputStream;
import java.io.FilterOutputStream;
import java.io.FileOutputStream;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.ArrayDeque;
import java.util.HashMap;
import java.util.List;

/** Opt-in dump-field locator. Never patches or replaces original game methods. */
final class Trace10 {
    static final String requested = System.getenv("ORACLE10_TRACE_BYTE");
    static final long target = requested == null ? -1 : Long.parseLong(requested);
    static final ArrayDeque<String> names = new ArrayDeque<String>();
    static final HashMap<String,List<String>> sources = new HashMap<String,List<String>>();
    static boolean printed;
    static String outputFile;
    static final java.util.IdentityHashMap<ByteArrayOutputStream,Candidate> buffered = new java.util.IdentityHashMap<ByteArrayOutputStream,Candidate>();
    static final java.util.IdentityHashMap<OutputStream,Boolean> streamMatch = new java.util.IdentityHashMap<OutputStream,Boolean>();
    static final class Candidate {
        int offset,size;Object value;String name,location;
    }
    static double namedDouble(String name, double value) { if (target >= 0) names.add(name); return value; }
    static float namedFloat(String name, float value) { if (target >= 0) names.add(name); return value; }
    static int namedInt(String name, int value) { if (target >= 0) names.add(name); return value; }
    static long namedLong(String name, long value) { if (target >= 0) names.add(name); return value; }
    static boolean primary(OutputStream stream) {
        if (outputFile == null) return false;
        if (streamMatch.containsKey(stream)) return streamMatch.get(stream);
        OutputStream original=stream;boolean matches=false;
        try {
            java.lang.reflect.Field out = FilterOutputStream.class.getDeclaredField("out");out.setAccessible(true);
            while (stream instanceof FilterOutputStream) stream=(OutputStream)out.get(stream);
            if (stream instanceof FileOutputStream) {
                java.lang.reflect.Field path=FileOutputStream.class.getDeclaredField("path");path.setAccessible(true);
                matches=outputFile.equals(new File((String)path.get(stream)).getCanonicalPath());
            }
        } catch (Exception unavailable) { }
        streamMatch.put(original,matches);return matches;
    }
    static void field(DataOutputStream stream, int size, Object value) {
        if (target < 0) return;
        if (printed) { if (!names.isEmpty()) names.remove();return; }
        if (primary(stream)) field(stream.size(),size,value);
        else if (!names.isEmpty()) names.remove();
    }
    static void buffered(ByteArrayOutputStream stream, int size, Object value) {
        if (target < 0) return;
        String name=names.isEmpty()?null:names.remove();int offset=stream.size();
        if (printed || target<offset || target>=(long)offset+size) return;
        Candidate candidate=locate(offset,size,value,name);buffered.put(stream,candidate);
    }
    static void bufferedOutput(ByteArrayOutputStream stream, OutputStream destination) {
        if (target < 0 || printed || !primary(destination)) return;
        Candidate candidate=buffered.get(stream);
        if (candidate != null) {
            int prefix=destination instanceof DataOutputStream?((DataOutputStream)destination).size():0;
            candidate.offset+=prefix;
            if (target>=candidate.offset && target<(long)candidate.offset+candidate.size) emit(candidate);
        }
    }
    static void writeBoolean(DataOutputStream stream, String name, boolean value) throws java.io.IOException {
        if (target >= 0) names.addFirst(name);
        field(stream, 1, value);
        stream.writeBoolean(value);
    }
    static void direct(DataOutputStream stream, String name, int size, Object value) {
        if (target >= 0) names.addFirst(name + " [big endian]");
        field(stream, size, value);
    }
    static void writeInt(DataOutputStream s, String n, int v) throws java.io.IOException { direct(s,n,4,v); s.writeInt(v); }
    static void writeLong(DataOutputStream s, String n, long v) throws java.io.IOException { direct(s,n,8,v); s.writeLong(v); }
    static void writeFloat(DataOutputStream s, String n, float v) throws java.io.IOException { direct(s,n,4,v); s.writeFloat(v); }
    static void writeDouble(DataOutputStream s, String n, double v) throws java.io.IOException { direct(s,n,8,v); s.writeDouble(v); }
    static void writeShort(DataOutputStream s, String n, int v) throws java.io.IOException { direct(s,n,2,v); s.writeShort(v); }
    static void writeByte(DataOutputStream s, String n, int v) throws java.io.IOException { direct(s,n,1,v); s.writeByte(v); }
    static void writeBytes(OutputStream s,String n,byte[] v) throws java.io.IOException {
        if (target>=0 && s instanceof DataOutputStream) {
            names.addFirst(n);field((DataOutputStream)s,v.length,"byte array");
        }
        s.write(v);
    }
    static void writeBytes(OutputStream s,String n,int v) throws java.io.IOException {
        if (target>=0 && s instanceof DataOutputStream) {
            names.addFirst(n);field((DataOutputStream)s,1,v);
        }
        s.write(v);
    }
    static void field(int offset, int size, Object value) {
        if (target < 0) return;
        String name = names.isEmpty() ? null : names.remove();
        if (printed || target < offset || target >= (long)offset + size) return;
        emit(locate(offset,size,value,name));
    }
    static Candidate locate(int offset,int size,Object value,String name) {
        Candidate candidate=new Candidate();candidate.offset=offset;candidate.size=size;candidate.value=value;
        String location = "";
        for (StackTraceElement frame : Thread.currentThread().getStackTrace()) {
            if (!frame.getClassName().startsWith("net.minecraft.src.") || frame.getClassName().equals(Trace10.class.getName())) continue;
            String method = frame.getMethodName();
            if (method.equals("i") || method.equals("d") || method.equals("f") || method.equals("l") || method.equals("i32") || method.equals("i64") || method.equals("u8") || method.equals("i16") || method.equals("bytes")) continue;
            location = frame.getFileName() + ":" + frame.getLineNumber();
            if (name == null) {
                try {
                    String file = frame.getFileName();
                    if (!sources.containsKey(file)) sources.put(file, Files.readAllLines(Paths.get(System.getenv("ORACLE10_SOURCE"), file)));
                    name = sources.get(file).get(frame.getLineNumber() - 1).trim();
                } catch (Exception missing) { name = frame.getMethodName() + " serialized value"; }
            }
            break;
        }
        candidate.name=name;candidate.location=location;return candidate;
    }
    static void emit(Candidate candidate) {
        System.err.println("FIRST FIELD: byte " + target + ", " + candidate.name + " = " + candidate.value + " (" + candidate.location + ", field starts " + candidate.offset + ", " + candidate.size + " bytes, " + candidate.value.getClass().getSimpleName() + ")");
        printed = true;
    }
}
