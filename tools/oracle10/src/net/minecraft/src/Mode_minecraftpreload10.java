package net.minecraft.src;
import java.io.*;
import java.lang.reflect.*;
import java.util.*;
import net.minecraft.client.Minecraft;
/** Invokes original preloadWorld, including null loading screen, player
 * coordinates, optional load-provider focus and reduced-controller branches. */
public final class Mode_minecraftpreload10 {
    static ArrayList<Integer> calls;
    static void call(int... ns){for(int n:ns)calls.add(n);}
    static final class Client extends Minecraft {
        Client(){super(null,null,null,1,1,false);}public void displayUnexpectedThrowable(UnexpectedThrowable t){throw new RuntimeException();}
    }
    static final class Progress extends LoadingScreenRenderer {
        Progress(){super(null);}public void printText(String s){call(0);}
        public void displayLoadingString(String s){call(s.equals("Building terrain")?1:6);}
        public void setLoadingProgress(int p){call(2,p);}
    }
    static final class TestWorld extends World {
        IChunkProvider provider;int x,z,remaining;
        TestWorld(){super((ISaveHandler)null,"",new WorldSettings(0,0,true, false));}
        public ChunkCoordinates getSpawnPoint(){return new ChunkCoordinates(x,64,z);}
        public IChunkProvider getIChunkProvider(){return provider;}
        public int getBlockId(int x,int y,int z){call(3,x,y,z);remaining=(x^z)&3;return 0;}
        public boolean updatingLighting(){boolean more=remaining-->0;call(4,more?1:0);return more;}
        public void dropOldChunks(){call(5);}
    }
    static final class Controller extends PlayerControllerSP {
        boolean reduced;Controller(){super(null);}public boolean func_35643_e(){return reduced;}
    }
    public static void main(String[] a)throws Exception {
        Method preload=Minecraft.class.getDeclaredMethod("preloadWorld",String.class);preload.setAccessible(true);
        Random r=new Random(81817);
        try(DataOutputStream out=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            for(int c=0;c<128;c++) {
                calls=new ArrayList<>();Client mc=(Client)Oracle.allocate(Client.class);
                TestWorld w=(TestWorld)Oracle.allocate(TestWorld.class);w.x=r.nextInt();w.z=r.nextInt();
                boolean player=(c&1)!=0,loading=(c&2)!=0,reduced=(c&4)!=0,provider=(c&8)!=0;
                double px=(r.nextInt(4000)-2000)/3.0,pz=(r.nextInt(4000)-2000)/7.0;
                if(player){mc.thePlayer=(EntityPlayerSP)Oracle.allocate(EntityPlayerSP.class);mc.thePlayer.posX=px;mc.thePlayer.posZ=pz;}
                mc.theWorld=w;w.provider=provider?(ChunkProviderLoadOrGenerate)Oracle.allocate(ChunkProviderLoadOrGenerate.class):null;
                if(loading)mc.loadingScreen=(Progress)Oracle.allocate(Progress.class);
                Controller controller=new Controller();controller.reduced=reduced;mc.playerController=controller;
                for(int n:new int[]{Trace10.namedInt("w.x", w.x),Trace10.namedInt("w.z", w.z),Trace10.namedInt("player?1:0", player?1:0),Trace10.namedInt("loading?1:0", loading?1:0),Trace10.namedInt("reduced?1:0", reduced?1:0),Trace10.namedInt("provider?1:0", provider?1:0)})EntityOracle.i(out,n);
                EntityOracle.d(out,px);EntityOracle.d(out,pz);
                preload.invoke(mc,"Replay");
                if(provider){for(String n:new String[]{"curChunkX","curChunkY"}){Field f=ChunkProviderLoadOrGenerate.class.getDeclaredField(n);f.setAccessible(true);call(7,f.getInt(w.provider));}}
                EntityOracle.i(out,Trace10.namedInt("calls.size()", calls.size()));for(int n:calls)EntityOracle.i(out,n);
            }
        }
    }
}
