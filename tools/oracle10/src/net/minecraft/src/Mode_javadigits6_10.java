package net.minecraft.src;
/** Character.digit tables of the required Mojang 8u51 runtime. */
public final class Mode_javadigits6_10 {
 public static void main(String[] a){for(int c=0;c<=65535;c++)if(Character.digit((char)c,10)==0)System.out.printf("0x%04x, ",c);System.out.println();}
}
