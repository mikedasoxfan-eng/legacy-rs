package net.minecraft.src;
import java.io.*;
import java.util.*;
import java.lang.reflect.*;
import java.nio.ByteBuffer;
import net.minecraft.client.Minecraft;
import org.lwjgl.input.Mouse;
/** Original GuiSlot callbacks. Only input buffers and timestamps are supplied by the fixture. */
public final class Mode_guislot8_10 {
    static final class Textures extends RenderEngine {
        Textures(){super(null,null);}
        public int getTexture(String name){return 0;}
    }
    static final class Client extends Minecraft {
        Client(){super(null,null,null,1,1,false);}
        public void displayUnexpectedThrowable(UnexpectedThrowable t){throw new RuntimeException();}
    }
    static final class List extends GuiSlot {
        int size,content=-1;ArrayList<Integer> calls=new ArrayList<>();
        List(Minecraft mc,int w,int h,int top,int bottom,int sh){super(mc,w,h,top,bottom,sh);}
        protected int getSize(){return size;}
        protected int getContentHeight(){return content<0?super.getContentHeight():content;}
        void call(int... values){for(int n:values)calls.add(n);}
        protected boolean isSelected(int i){return i%3==0;}
        protected void elementClicked(int i,boolean d){call(2,i,d?1:0);}
        protected void drawBackground(){call(1);}
        protected void drawSlot(int i,int x,int y,int h,Tessellator t){call(5,i,x,y,h);}
        protected void func_27260_a(int x,int y,Tessellator t){call(4,x,y);}
        protected void func_27255_a(int x,int y){call(3,x,y);}
        protected void func_27257_b(int x,int y){call(6,x,y);}
    }
    static Field field(Class<?> c,String n)throws Exception{Field f=c.getDeclaredField(n);f.setAccessible(true);return f;}
    public static void main(String[] a)throws Exception {
        field(Mouse.class,"created").setBoolean(null,true);field(Mouse.class,"buttonCount").setInt(null,1);
        ByteBuffer buttons=ByteBuffer.allocate(1);field(Mouse.class,"buttons").set(null,buttons);
        Client mc=(Client)Oracle.allocate(Client.class);mc.renderEngine=(Textures)Oracle.allocate(Textures.class);
        Random r=new Random(8181);
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            for(int c=0;c<2000;c++) {
                int width=320+r.nextInt(800),height=200+r.nextInt(500),top=20+r.nextInt(40),bottom=height-30,sh=8+r.nextInt(60);
                List s=new List(mc,width,height,top,bottom,sh);s.size=r.nextInt(40);if(c%3==0)s.content=r.nextInt(3000);
                s.func_27258_a(c%2==0);s.func_27259_a(c%3!=0,r.nextInt(50));s.registerScrollButtons(new ArrayList(),7,8);
                for(int t=0;t<5;t++) {
                    int mx=t==0?width/2:t==1?width/2+125:r.nextInt(width+40)-20;
                    int my=t==0?top+1:t==1?top+20:r.nextInt(height+40)-20;
                    buttons.put(0,(byte)(t%3!=2?1:0));
                    field(GuiSlot.class,"initialClickY").setFloat(s,(c+t)%4==0?-1f:(c+t)%4==1?-2f:(float)(top+15));
                    field(GuiSlot.class,"amountScrolled").setFloat(s,(r.nextInt(4001)-1000)/7f);
                    field(GuiSlot.class,"scrollMultiplier").setFloat(s,(r.nextInt(2001)-1000)/317f);
                    field(GuiSlot.class,"selectedElement").setInt(s,c%2==0?0:-1);
                    long last=c%2==0?Long.MAX_VALUE:0;field(GuiSlot.class,"lastClicked").setLong(s,last);
                    GuiButton button=new GuiButton(t==0?7:t==1?8:99,0,0,"");button.enabled=c%4!=0;s.actionPerformed(button);
                    s.drawScreen(mx,my,0);
                    for(int n:new int[]{Trace10.namedInt("s.field_35409_k", s.field_35409_k),Trace10.namedInt("s.field_35408_l", s.field_35408_l),Trace10.namedInt("field(GuiSlot.class,\"selectedElement\").getInt(s)", field(GuiSlot.class,"selectedElement").getInt(s)),Trace10.namedInt("s.getContentHeight()", s.getContentHeight()),Trace10.namedInt("s.func_27256_c(mx,my)", s.func_27256_c(mx,my)),Trace10.namedInt("s.func_27256_c(width/2,top)", s.func_27256_c(width/2,top)),Trace10.namedInt("s.func_27256_c(width/2,bottom+100)", s.func_27256_c(width/2,bottom+100))})EntityOracle.i(o,n);
                    for(String n:new String[]{"initialClickY","scrollMultiplier","amountScrolled"})EntityOracle.f(o,Trace10.namedFloat("field(GuiSlot.class,n).getFloat(s)", field(GuiSlot.class,n).getFloat(s)));
                    Trace10.writeBoolean(o, "field(GuiSlot.class,\"lastClicked\").getLong(s)!=last", field(GuiSlot.class,"lastClicked").getLong(s)!=last);
                    EntityOracle.i(o,Trace10.namedInt("s.calls.size()", s.calls.size()));for(int n:s.calls)EntityOracle.i(o,n);s.calls.clear();
                    Oracle.weatherVertices.clear();Oracle.weatherLights.clear();
                }
            }
        }
    }
}
