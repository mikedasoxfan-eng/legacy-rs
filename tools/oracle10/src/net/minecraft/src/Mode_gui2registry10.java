package net.minecraft.src;
import java.io.*;
public final class Mode_gui2registry10 {
    public static void main(String[] a)throws Exception {
        try(DataOutputStream o=Gui2Fixture10.out(a[0])) {
            for(Object value:AchievementList.achievementList){Achievement x=(Achievement)value;
                for(int n:new int[]{x.statId,x.displayColumn,x.displayRow,x.theItemStack.itemID,x.parentAchievement==null?-1:x.parentAchievement.statId})Gui2Fixture10.i(o,n);
                o.writeBoolean(x.getSpecial());Gui2Fixture10.s(o,x.statName);Gui2Fixture10.s(o,x.getDescription());
            }
            for(int n:new int[]{AchievementList.minDisplayColumn,AchievementList.minDisplayRow,AchievementList.maxDisplayColumn,AchievementList.maxDisplayRow})Gui2Fixture10.i(o,n);
            GameSettings s=new GameSettings();
            for(EnumOptions e:EnumOptions.values()){Gui2Fixture10.i(o,e.ordinal());Gui2Fixture10.s(o,e.getEnumString());o.writeBoolean(e.getEnumFloat());o.writeBoolean(e.getEnumBoolean());Gui2Fixture10.s(o,s.getKeyBinding(e));}
            for(int distance=-2;distance<6;distance++)for(boolean clouds:new boolean[]{false,true}){s.renderDistance=distance;s.clouds=clouds;o.writeBoolean(s.shouldRenderClouds());}
        }
    }
}
