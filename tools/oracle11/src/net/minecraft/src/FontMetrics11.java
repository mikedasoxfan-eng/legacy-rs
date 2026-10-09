package net.minecraft.src;
import java.io.*;
/** Exercise every UTF-16 code unit through the unchanged original width method. */
public final class FontMetrics11 {
    public static void main(String[] args) throws Exception {
        FontRenderer font=new FontRenderer(new GameSettings(),"/font/default.png",new Mode_guiutility10.Textures(),false);
        try(DataOutputStream output=new DataOutputStream(new FileOutputStream(args[0]))) {
            for(boolean unicode:new boolean[]{false,true}) {
                font.func_44032_a(unicode);
                for(int unit=0;unit<65536;unit++) output.writeInt(font.getStringWidth(String.valueOf((char)unit)));
                String[] texts={""," ","\u00a7","\u00a7kAB\u00a7cC","hello\u0971\u09F7\u0C79\u1161", "\uD800\uDC00\uD800"};
                for(String text:texts) output.writeInt(font.getStringWidth(text));
            }
        }
    }
}
