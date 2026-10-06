package net.minecraft.src;
import java.io.*;import java.util.*;
public final class Mode_gui2gradient10 {
    public static void main(String[] a)throws Exception {
        Gui g=new Gui();Random r=new Random(10506);Gui2Fixture10.field(Tessellator.class,"convertQuadsToTriangles").setBoolean(null,false);
        try(DataOutputStream o=Gui2Fixture10.out(a[0])) {
            for(int c=0;c<1000;c++) {
                int x=r.nextInt(2000)-1000,y=r.nextInt(2000)-1000,w=r.nextInt(1000)-500,h=r.nextInt(1000)-500,top=r.nextInt(),bottom=r.nextInt();g.zLevel=(r.nextInt(2000)-1000)/7F;
                Oracle.weatherVertices.clear();g.drawGradientRect(x,y,x+w,y+h,top,bottom);
                for(float[] v:Oracle.weatherVertices) {for(int n=1;n<=3;n++)Gui2Fixture10.f(o,v[n]);for(int n=6;n<=9;n++)Gui2Fixture10.f(o,v[n]);}
            }
        }
    }
}
