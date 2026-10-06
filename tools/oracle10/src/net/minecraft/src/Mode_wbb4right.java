package net.minecraft.src;
import java.io.*;

/** Item.onItemRightClick through PlayerController.sendUseItem for every changed/new release item. */
public final class Mode_wbb4right {
    static final int[] IDS={325,326,327,335,333,332,344,368,373,374,111,261,322,260,282,297,319,320,349,350,357,360,363,364,365,366,367,375,267,1,358};
    static final int[] POTIONS={0,16,8193,8204,16385,16396,32767};
    static void terrain(World w,int t)throws Exception {
        int[][] cell={{1,0},{9,0},{8,3},{11,0},{0,0},{78,0},{9,0},{9,0}};
        Mode_worldblocks10.raw(w,8,64,8,cell[t][0],cell[t][1]);
        if(t==6)Mode_worldblocks10.raw(w,8,65,8,111,0);
        if(t==7){Entity pig=EntityList.createEntityInWorld("Pig",w);pig.setLocationAndAngles(8.5,65,10.5,0,0);w.entityJoinedWorld(pig);}
    }
    public static void main(String[] a)throws Exception {
        int init=Block.stone.blockID;
        PlayerController controller=(PlayerController)Oracle.allocate(PlayerControllerSP.class);
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            for(int id:IDS)for(int damage:id==373?POTIONS:new int[]{0})for(int t=0;t<8;t++)for(float pitch:new float[]{90F,45F,0F})for(float yaw:new float[]{0F,135F})for(int mode=0;mode<3;mode++)for(boolean remote:new boolean[]{false,true}) {
                World w=Mode_worldblocks10.fixture(true);EntityPlayer p=Blocks4Oracle.newPlayer(w);terrain(w,t);
                Blocks4Oracle.place(p,8.5,65.0,8.5,yaw,pitch);
                boolean creative=mode==2;p.capabilities.depleteBuckets=creative;p.capabilities.disableDamage=creative;
                if(mode==1)p.getFoodStats().setFoodLevel(6);
                if(yaw==0F)p.inventory.mainInventory[5]=new ItemStack(262,3,0);
                if(mode==1)p.addPotionEffect(new PotionEffect(1,200,1));
                ItemStack s=new ItemStack(id,mode==1?2:1,damage);p.inventory.mainInventory[0]=s;
                w.multiplayerWorld=remote;Blocks4Oracle.seeds(w);
                Blocks4Oracle.bool(o,"sendUseItem",controller.sendUseItem(p,w,s));
                Blocks4Oracle.player(o,p);Mode_wbplacement4.cells(o,w,6,10,62,67);Blocks4Oracle.entities(o,w);Blocks4Oracle.rngs(o,w);WorldEventOracle.snapshot(w,o);
            }
        }
    }
}
