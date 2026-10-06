package net.minecraft.src;
import java.io.*;

/** Pathfinder through World/ChunkCache: liquids, open/closed doors, walls, drops and sizes. */
public final class Mode_r10world3path {
    public static void main(String[] a)throws Exception {
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            int[] walls={0,1,8,10,64,71,53,85,9};
            for(int scenario=0;scenario<walls.length;scenario++) {
                World w=Mode_r10world3.flat(0);
                for(int x=-48;x<64;x++)for(int z=-48;z<64;z++)EntityOracle.rawBlock(w,x,63,z,1,0);
                for(int z=-12;z<=12;z++)if(z!=2)for(int y=64;y<=66;y++)EntityOracle.rawBlock(w,0,y,z,walls[scenario],scenario%2==0?4:0);
                if(scenario==6)for(int x=1;x<=12;x++)for(int z=-5;z<=5;z++)EntityOracle.rawBlock(w,x,64,z,1,0);
                if(scenario==7)for(int x=-10;x<=10;x++)for(int z=-10;z<=10;z++)EntityOracle.rawBlock(w,x,67,z,1,0);
                for(float width:new float[]{0.6F,0.9F,1.4F,2.0F})for(int start:new int[]{-8,-3,0,8})for(int target:new int[]{-12,5,12}) {
                    EntityItem e=new EntityItem(w);e.setSize(width,1.8F);e.yOffset=0;e.setPosition(start+0.5,64,0.5);
                    PathEntity p=w.getEntityPathToXYZ(e,target,64,4,16);
                    Mode_r10world3.b(o,p!=null);
                    if(p!=null) {
                        Mode_r10world3.i(o,p.pathLength);
                        PathPoint[] points=(PathPoint[])Mode_structures10.field(p,"points");
                        for(PathPoint q:points){Mode_r10world3.i(o,q.xCoord);Mode_r10world3.i(o,q.yCoord);Mode_r10world3.i(o,q.zCoord);}
                        while(!p.isFinished()){Vec3D v=p.getPosition(e);o.writeDouble(v.xCoord);o.writeDouble(v.yCoord);o.writeDouble(v.zCoord);p.incrementPathIndex();}
                    }
                }
                Mode_r10world3.l(o,EntityOracle.rngState(w.rand));
            }
        }
    }
}
