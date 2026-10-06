package net.minecraft.src;

/** Finish fixture I/O and exit despite the 1.0 non-daemon File IO Thread. */
public final class OracleRunner10 {
    public static void main(String[] arguments) {
        int status = 0;
        try {
            // Fixture image decoders need no temporary disk cache. The decoded
            // pixels and original game calls remain identical.
            javax.imageio.ImageIO.setUseCache(false);
            recordArguments(arguments);
            Oracle.main(arguments);
        } catch (Throwable error) {
            error.printStackTrace();
            status = 1;
        }
        try {
            ThreadedFileIOBase.field_40573_a.func_40566_a();
            RegionFileCache.clearRegionFileReferences();
        } catch (Throwable error) {
            error.printStackTrace();
            status = 1;
        }
        System.exit(status);
    }
    static String quote(String text) {
        return "\"" + text.replace("\\", "\\\\").replace("\"", "\\\"").replace("\n", "\\n").replace("\r", "\\r") + "\"";
    }
    static void recordArguments(String[] arguments) throws java.io.IOException {
        String output = null;
        for (String argument : arguments) if (argument.endsWith(".bin")) output = argument;
        if (output == null) return;
        Trace10.outputFile = new java.io.File(output).getCanonicalPath();
        try (java.io.PrintWriter writer = new java.io.PrintWriter(output + ".oracle10.json", "UTF-8")) {
            writer.print("{\"arguments\":[");
            for (int i = 0; i < arguments.length; i++) {
                if (i > 0) writer.print(",");
                writer.print(quote(arguments[i]));
            }
            writer.println("]}");
        }
    }
}
