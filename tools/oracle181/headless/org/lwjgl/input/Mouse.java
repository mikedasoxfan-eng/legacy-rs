package org.lwjgl.input;
import java.util.*;
public final class Mouse {
    static final ArrayDeque<int[]> events=new ArrayDeque<>();static int[] current=new int[5];
    static boolean[] down=new boolean[16];static boolean grabbed;
    public static void reset(){events.clear();Arrays.fill(down,false);current=new int[5];}
    public static void queue(int button,boolean state,int wheel){events.add(new int[]{button,state?1:0,wheel,0,0});if(button>=0&&button<16)down[button]=state;}
    public static boolean next(){if(events.isEmpty())return false;current=events.remove();return true;}
    public static int getEventButton(){return current[0];}public static boolean getEventButtonState(){return current[1]!=0;}
    public static int getEventDWheel(){return current[2];}public static int getEventX(){return current[3];}public static int getEventY(){return current[4];}
    public static int getEventDX(){return 0;}public static int getEventDY(){return 0;}public static long getEventNanoseconds(){return 0;}
    public static boolean isButtonDown(int button){return button>=0&&button<16&&down[button];}
    public static void setGrabbed(boolean b){grabbed=b;}public static boolean isGrabbed(){return grabbed;}
    public static void setCursorPosition(int x,int y){}public static int getDX(){return 0;}public static int getDY(){return 0;}
    public static int getX(){return 0;}public static int getY(){return 0;}public static int getDWheel(){return 0;}
    public static void create(){}public static void destroy(){}public static boolean isCreated(){return true;}public static void poll(){}
    public static int getButtonCount(){return 16;}
}
