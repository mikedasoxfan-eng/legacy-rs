package net.minecraft.src;
public final class Mode_playerdata {
 public static void main(String[] args)throws Exception {
  int initialized=Block.stone.blockID;
  ContainerOracle.recipes(args[1]+".recipes");RendererOracle.itemTable(args[1]+".itemtable.rs");Oracle.renderTable(args[1]+".blockitems.rs");PlayerOracle.registry(args[1]+".registry");
 }
}
