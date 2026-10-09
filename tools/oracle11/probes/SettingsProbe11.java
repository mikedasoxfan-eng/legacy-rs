package net.minecraft.src;
import java.io.*;
public final class SettingsProbe11 {
    public static void main(String[] args)throws Exception {
        File dir=new File(args[0]);dir.mkdirs();
        GameSettings settings=new GameSettings(null,dir);
        settings.saveOptions();
        System.out.println(settings.field_44018_Q);
    }
}
