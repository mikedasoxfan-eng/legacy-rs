package net.minecraft.src;
import java.io.*;
import java.awt.image.BufferedImage;
import java.text.SimpleDateFormat;
// ScreenShotHelper (1.0): numbered names, the new explicit-name variant, pixel flip.
// The date stamp is pinned to "oracle". Usage: screenshot10 <dir> <out>
public final class Mode_screenshot10 {
    static void clean(File f){if(f.isDirectory())for(File c:f.listFiles())clean(c);f.delete();}
    public static void main(String[] a)throws Exception {
        File root=new File(a[1]);clean(root);root.mkdirs();
        java.lang.reflect.Field df=ScreenShotHelper.class.getDeclaredField("dateFormat");df.setAccessible(true);df.set(null,new SimpleDateFormat("'oracle'"));
        String[] names={null,null,"named.png","named.png",null};int[][] dims={{6,6},{5,3},{2,7},{4,4},{3,2}};
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[2])))) {
            for(int i=0;i<names.length;i++) {
                String m=ScreenShotHelper.func_35879_a(root,names[i],dims[i][0],dims[i][1]);
                byte[] b=m.getBytes("UTF-8");EntityOracle.i(o,Trace10.namedInt("b.length", b.length));Trace10.writeBytes(o, "b", b);
                BufferedImage img=javax.imageio.ImageIO.read(new File(root,"screenshots/"+m.substring("Saved screenshot as ".length())));
                EntityOracle.i(o,Trace10.namedInt("img.getWidth()", img.getWidth()));EntityOracle.i(o,Trace10.namedInt("img.getHeight()", img.getHeight()));
                for(int y=0;y<img.getHeight();y++)for(int x=0;x<img.getWidth();x++)EntityOracle.i(o,Trace10.namedInt("img.getRGB(x,y)", img.getRGB(x,y)));
            }
        }
    }
}
