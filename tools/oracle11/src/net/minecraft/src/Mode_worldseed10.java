package net.minecraft.src;
import java.io.*;
import java.lang.reflect.*;
import net.minecraft.client.Minecraft;
/** Executes GuiCreateWorld.actionPerformed; only inherently clock-random seeds are classified. */
public final class Mode_worldseed10 {
    static final class Client extends Minecraft {
        WorldSettings settings;int starts,displays;
        Client(){super(null,null,null,1,1,false);}
        public void displayUnexpectedThrowable(UnexpectedThrowable t){throw new RuntimeException();}
        public void displayGuiScreen(GuiScreen g){displays++;}
        public void startWorld(String folder,String name,WorldSettings s){settings=s;starts++;}
    }
    static void set(GuiCreateWorld g,String n,Object value)throws Exception {Field f=GuiCreateWorld.class.getDeclaredField(n);f.setAccessible(true);f.set(g,value);}
    static void seed(DataOutputStream o,String text,int mode,boolean features)throws Exception {
        Client mc=(Client)Oracle.allocate(Client.class);GuiCreateWorld g=new GuiCreateWorld(null);g.mc=mc;
        set(g,"textboxSeed",new GuiTextField(g,null,0,0,200,20,text));set(g,"textboxWorldName",new GuiTextField(g,null,0,0,200,20,"Name"));set(g,"folderName","Folder");set(g,"gameMode",mode==0?"survival":"creative");set(g,"field_35365_g",features);
        GuiButton b=new GuiButton(0,0,0,"create");g.actionPerformed(b);g.actionPerformed(b);
        boolean random=text.length()==0;try{random|=Long.parseLong(text)==0;}catch(NumberFormatException ignored){}
        Trace10.writeBoolean(o, "random", random);EntityOracle.l(o,Trace10.namedLong("random?0:mc.settings.getSeed()", random?0:mc.settings.getSeed()));EntityOracle.i(o,Trace10.namedInt("mc.settings.getGameType()", mc.settings.getGameType()));Trace10.writeBoolean(o, "mc.settings.isMapFeaturesEnabled()", mc.settings.isMapFeaturesEnabled());EntityOracle.i(o,Trace10.namedInt("mc.starts", mc.starts));EntityOracle.i(o,Trace10.namedInt("mc.displays", mc.displays));Trace10.writeBoolean(o, "mc.playerController instanceof PlayerControllerCreative", mc.playerController instanceof PlayerControllerCreative);
    }
    public static void main(String[] args)throws Exception {
        try(DataOutputStream o=new DataOutputStream(new FileOutputStream(args[1]))) {
            for(int c=0;c<65536;c++)Trace10.writeByte(o, "Character.digit((char)c,10)", Character.digit((char)c,10));
            String[] seeds={"","0","-0","+0","00","+1","-1","9223372036854775807","-9223372036854775808","9223372036854775808","-9223372036854775809"," 1","1 ","\t1","+","-","hello","\u0000","\uff11\uff12\uff13","\u0661\u0662\u0663","\u0967\u0968\u0969","\ud83d\ude00","1.0","1e3"};
            for(String s:seeds)for(int mode=0;mode<2;mode++)for(boolean features:new boolean[]{false,true})seed(o,s,mode,features);
            java.util.Random r=new java.util.Random(4181);
            for(int t=0;t<10000;t++) {
                StringBuilder s=new StringBuilder();int n=r.nextInt(32);for(int k=0;k<n;k++)s.append((char)r.nextInt(65536));
                seed(o,s.toString(),r.nextInt(2),r.nextBoolean());
            }
        }
    }
}
