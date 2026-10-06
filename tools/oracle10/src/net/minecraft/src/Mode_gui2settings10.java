package net.minecraft.src;
import java.io.*;import java.util.*;import java.nio.file.*;import net.minecraft.client.Minecraft;
public final class Mode_gui2settings10 {
    static class Settings extends GameSettings {public void saveOptions(){}}
    public static void main(String[] a)throws Exception {
        Settings s=new Settings();Random r=new Random(10708);
        try(DataOutputStream o=Gui2Fixture10.out(a[0])) {
            Gui2Fixture10.i(o,s.thirdPersonView);
            for(int c=0;c<1000;c++) {
                s.field_41087_P=r.nextInt();s.clouds=r.nextBoolean();int delta=r.nextInt();s.setOptionValue(EnumOptions.PARTICLES,delta);s.setOptionValue(EnumOptions.RENDER_CLOUDS,delta);
                Gui2Fixture10.i(o,s.field_41087_P);o.writeBoolean(s.clouds);
                for(EnumOptions e:EnumOptions.values())o.writeBoolean(s.getOptionOrdinalValue(e));
                s.field_41087_P=c%3;Gui2Fixture10.s(o,s.getKeyBinding(EnumOptions.PARTICLES));Gui2Fixture10.s(o,s.getKeyBinding(EnumOptions.RENDER_CLOUDS));
            }
            File dir=new File("settings-fixture");dir.mkdirs();File file=new File(dir,"options.txt");
            for(int c=0;c<32;c++) {
                try(PrintWriter p=new PrintWriter(file)){p.println("particles:"+(c-4));p.println("clouds:"+(c%2==0));}
                GameSettings loaded=new GameSettings((Minecraft)Oracle.allocate(PlayerOracle.HeadlessMinecraft.class),dir);Gui2Fixture10.i(o,loaded.field_41087_P);o.writeBoolean(loaded.clouds);loaded.saveOptions();
                Gui2Fixture10.s(o,new String(Files.readAllBytes(file.toPath()),"UTF-8").replace("\r\n","\n"));
            }
        }
    }
}
