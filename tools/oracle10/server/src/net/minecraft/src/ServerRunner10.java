package net.minecraft.src;

/** Flush the original asynchronous region writer before ending a server fixture. */
public final class ServerRunner10 {
    public static void main(String[] arguments) {
        int status=0;
        try { ServerOracle.main(arguments); }
        catch(Throwable error) { error.printStackTrace(); status=1; }
        try {
            ThreadedFileIOBase.field_40514_a.func_40508_a();
            RegionFileCache.clearRegionFileReferences();
        } catch(Throwable error) { error.printStackTrace();status=1; }
        System.exit(status);
    }
}
