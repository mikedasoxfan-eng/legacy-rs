package net.minecraft.src;
import java.io.*;
import java.util.*;
import net.minecraft.client.Minecraft;

/** Calls the original inventory screen constructor, init/update and hidden button callbacks. */
public final class Mode_inventorygui181 {
    static final class Client extends Minecraft {
        int displays, kind;
        Client(){super(null,null,null,1,1,false);}
        public void displayUnexpectedThrowable(UnexpectedThrowable t){throw new RuntimeException();}
        public void displayGuiScreen(GuiScreen s){displays++;kind=s==null?-1:s instanceof GuiContainerCreative?1:s instanceof GuiAchievements?2:s instanceof GuiStats?3:4;}
    }
    static final class Player extends EntityPlayerSP {
        int inventoryStats;
        Player(Minecraft mc,World w){super(mc,w,new Session("Player",""),0);}
        public void addStat(StatBase s,int n){if(s==AchievementList.openInventory)inventoryStats+=n;}
    }
    static final class Probe extends GuiInventory {
        Probe(Player p,Client mc){super(p);this.mc=mc;}
        void button(int id){actionPerformed(new GuiButton(id,0,0,"unused"));}
        void dirty(){controlList.add(new GuiButton(17,0,0,"unused"));}
        int buttons(){return controlList.size();}
    }
    public static void main(String[] a)throws Exception {
        World w=Mode_entities181.world();Client mc=(Client)Oracle.allocate(Client.class);mc.theWorld=w;
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            for(int c=0;c<10000;c++) {
                Player p=new Player(mc,w);mc.thePlayer=p;
                boolean creative=(c&1)!=0;
                mc.playerController=creative?new PlayerControllerTest(mc):new PlayerControllerSP(mc);
                p.health=(c&2)!=0?0:20;p.isDead=(c&4)!=0;
                Probe screen=new Probe(p,mc);
                screen.dirty();mc.displays=0;mc.kind=-2;
                screen.initGui();
                EntityOracle.i(o,screen.buttons());EntityOracle.i(o,mc.displays);EntityOracle.i(o,mc.kind);EntityOracle.i(o,p.inventoryStats);
                o.writeBoolean(screen.allowUserInput);o.writeBoolean(screen.doesGuiPauseGame());
                // Keep the old object: every callback must ask for a new redirect independently.
                for(int t=0;t<3;t++) {
                    creative=((c+t)&1)!=0;mc.playerController=creative?new PlayerControllerTest(mc):new PlayerControllerSP(mc);
                    mc.displays=0;mc.kind=-2;screen.updateScreen();
                    EntityOracle.i(o,mc.displays);EntityOracle.i(o,mc.kind);EntityOracle.i(o,p.inventoryStats);
                }
                for(int id:new int[]{-1,0,1,2,100}) {
                    mc.displays=0;mc.kind=-2;screen.button(id);
                    EntityOracle.i(o,mc.displays);EntityOracle.i(o,mc.kind);
                }
            }
        }
    }
}
