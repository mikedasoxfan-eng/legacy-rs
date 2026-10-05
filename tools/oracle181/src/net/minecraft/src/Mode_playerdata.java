package net.minecraft.src;
public final class Mode_playerdata {
 public static void main(String[] args)throws Exception {
  int initialized=Block.stone.blockID;
  ContainerOracle.recipes(args[1]+".recipes");PlayerOracle.registry(args[1]+".registry");
 }
}
