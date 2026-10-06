# Minecraft 1.0 harness baseline

Measured against the current worldgen10/client10 code. EXACT means a comparison passed; DIFFERS includes oracle/checker failures explicitly identified below. A run failure is not evidence of a completed simulation comparison. Field diagnostics use the first serialized difference; pixel diagnostics use the first changed RGB pixel. Pure group headings without output are omitted in favor of their individual child comparisons. Completed supplemental rechecks supersede matching master rows after fixture corrections; the rows identify the recheck suite. Repeated supplemental rows are omitted when their corrected result is already represented. All measured runtimes include the original observed run, including failures.

| Suite | Section | Result | Evidence | First difference / run failure |
| --- | --- | --- | --- | --- |
| verify10 | vehicles10: rails | DIFFERS | comparison | byte 21624, frame 1, Minecart id 2.fuel: Java 9, Rust 10 |
| verify10 | vehicles10: train | DIFFERS | comparison | byte 6722, frame 1, Minecart id 0.fuel: Java 1199, Rust 1200 |
| verify10 | vehicles10: boats | DIFFERS | comparison | byte 7587, frame 1, event 0 'splash' (tag 1).x: Java -19.5, Rust -19.12 |
| verify10 | vehicles10: riding | DIFFERS | comparison | byte 6602, frame 1, Player id 0.rand: Java 206789181479531, Rust 203227649148896 |
| verify10 | vehicles10: nbt | DIFFERS | comparison | byte 6722, frame 1, Minecart id 0.fuel: Java 1199, Rust 1200 |
| verify10 | vehicles10: longrail | DIFFERS | comparison | byte 3539, frame 1, Player id 0.rand: Java 206789181479531, Rust 203227649148896 |
| verify10 | vehicles10: ridingnbt | DIFFERS | comparison | byte 6602, frame 1, Player id 0.rand: Java 206789181479531, Rust 203227649148896 |
| verify10 | vehicles10: placing | DIFFERS | comparison | byte 2338, frame 1, Player id 0.rand: Java 206789181479531, Rust 203227649148896 |
| verify10 | vehicles10: inventory | EXACT | comparison | EXACT MATCH: 10000 original minecart inventory/NBT operations |
| verify10 | vehicles10: renderers | DIFFERS | comparison | vehicle case 12 draw 0 matrix op count: Rust 7, Java 6 |
| verify10 | MathHelper sine table | EXACT | comparison | sin table: 0 of 65536 entries differ |
| verify10 | player inventory (10000 original-bytecode operations) | DIFFERS | comparison | byte 13336, inventory operation 20, opcode 0.main[6].count: Java 7, Rust 1 |
| verify10 | containers10: clicks and recipes | DIFFERS | comparison | byte 26549, s != null = true (ContainerOracle.java:132, field starts 26549, 1 bytes, Boolean); Rust False |
| verify10 | containers10: furnace and dispenser tiles | DIFFERS | comparison | byte 1057, frame 1, furnaces.furnace9.slot0.present: Java 1, Rust 0 |
| verify10 | entity geometry (10000 original-bytecode cases) | EXACT | comparison | EXACT MATCH: 10000 geometry cases |
| verify10 | all Beta block collision/selection states (10000 cases) | DIFFERS | comparison | byte 10801, block geometry case 116.box[0].minX: Java 8.375, Rust 8.0 |
| verify10 | chunk meshes: seed 25, world rand 1, fancy, AO on | DIFFERS | comparison | chunk (-6,10) pass 0 vertex 1308: java pos (2,7.999,2) uv (0.8125,0.875) rgba ffffffff light 0 \| rust pos (2,8,2) uv (0.8125,0.875) rgba ffffffff light 0 (rechecked by verify_render10) |
| verify10 | chunk meshes: seed 25, world rand 1, fast, AO on | DIFFERS | comparison | chunk (-6,10) pass 0 vertex 1308: java pos (2,7.999,2) uv (0.8125,0.875) rgba ffffffff light 0 \| rust pos (2,8,2) uv (0.8125,0.875) rgba ffffffff light 0 (rechecked by verify_render10) |
| verify10 | chunk meshes: seed 25, world rand 1, fancy, AO off | DIFFERS | comparison | chunk (-6,10) pass 0 vertex 1308: java pos (2,7.999,2) uv (0.8125,0.875) rgba ffffffff light 0 \| rust pos (2,8,2) uv (0.8125,0.875) rgba ffffffff light 0 (rechecked by verify_render10) |
| verify10 | chunk meshes: seed 25, world rand 1, fast, AO off | DIFFERS | comparison | chunk (-6,10) pass 0 vertex 1308: java pos (2,7.999,2) uv (0.8125,0.875) rgba ffffffff light 0 \| rust pos (2,8,2) uv (0.8125,0.875) rgba ffffffff light 0 (rechecked by verify_render10) |
| verify10 | chunk meshes: seed 1, world rand 1, fancy, AO on | DIFFERS | checker failure | Checker failed before completing comparison: no entry found for key; thread 'main' (38372) panicked at C:\Users\logiu\Desktop\Projects\minecraft\wt181-r10harness\crates\worldgen10\src\world\mod.rs:477:21: (rechecked by verify_render10) |
| verify10 | chunk meshes: seed 1, world rand 1, fast, AO on | DIFFERS | checker failure | Checker failed before completing comparison: no entry found for key; thread 'main' (26996) panicked at C:\Users\logiu\Desktop\Projects\minecraft\wt181-r10harness\crates\worldgen10\src\world\mod.rs:477:21: (rechecked by verify_render10) |
| verify10 | chunk meshes: seed 1, world rand 1, fancy, AO off | DIFFERS | checker failure | Checker failed before completing comparison: no entry found for key; thread 'main' (33120) panicked at C:\Users\logiu\Desktop\Projects\minecraft\wt181-r10harness\crates\worldgen10\src\world\mod.rs:477:21: (rechecked by verify_render10) |
| verify10 | chunk meshes: seed 1, world rand 1, fast, AO off | DIFFERS | checker failure | Checker failed before completing comparison: no entry found for key; thread 'main' (29592) panicked at C:\Users\logiu\Desktop\Projects\minecraft\wt181-r10harness\crates\worldgen10\src\world\mod.rs:477:21: (rechecked by verify_render10) |
| verify10 | chunk meshes: seed -1, world rand -1, fancy, AO on | DIFFERS | checker failure | Checker failed before completing comparison: no entry found for key; thread 'main' (32212) panicked at C:\Users\logiu\Desktop\Projects\minecraft\wt181-r10harness\crates\worldgen10\src\world\mod.rs:477:21: (rechecked by verify_render10) |
| verify10 | chunk meshes: seed -1, world rand -1, fast, AO on | DIFFERS | checker failure | Checker failed before completing comparison: no entry found for key; thread 'main' (36708) panicked at C:\Users\logiu\Desktop\Projects\minecraft\wt181-r10harness\crates\worldgen10\src\world\mod.rs:477:21: (rechecked by verify_render10) |
| verify10 | chunk meshes: seed -1, world rand -1, fancy, AO off | DIFFERS | checker failure | Checker failed before completing comparison: no entry found for key; thread 'main' (18956) panicked at C:\Users\logiu\Desktop\Projects\minecraft\wt181-r10harness\crates\worldgen10\src\world\mod.rs:477:21: (rechecked by verify_render10) |
| verify10 | chunk meshes: seed -1, world rand -1, fast, AO off | DIFFERS | checker failure | Checker failed before completing comparison: no entry found for key; thread 'main' (34444) panicked at C:\Users\logiu\Desktop\Projects\minecraft\wt181-r10harness\crates\worldgen10\src\world\mod.rs:477:21: (rechecked by verify_render10) |
| verify10 | chunk meshes: seed 0, world rand 1, fancy, AO on | DIFFERS | comparison | chunk (10,5) pass 0 vertex 1424: java pos (16,52,1) uv (0.18746094,0.062460937) rgba ffffffff light 0 \| rust pos (4,39,1) uv (0.12496094,0.062460937) rgba ffffffff light 0 (rechecked by verify_render10) |
| verify10 | chunk meshes: seed 0, world rand 1, fast, AO on | DIFFERS | comparison | chunk (10,5) pass 0 vertex 1424: java pos (16,52,1) uv (0.18746094,0.062460937) rgba ffffffff light 0 \| rust pos (4,39,1) uv (0.12496094,0.062460937) rgba ffffffff light 0 (rechecked by verify_render10) |
| verify10 | chunk meshes: seed 0, world rand 1, fancy, AO off | DIFFERS | comparison | chunk (10,5) pass 0 vertex 1424: java pos (16,52,1) uv (0.18746094,0.062460937) rgba ffffffff light 0 \| rust pos (4,39,1) uv (0.12496094,0.062460937) rgba ffffffff light 0 (rechecked by verify_render10) |
| verify10 | chunk meshes: seed 0, world rand 1, fast, AO off | DIFFERS | comparison | chunk (10,5) pass 0 vertex 1424: java pos (16,52,1) uv (0.18746094,0.062460937) rgba ffffffff light 0 \| rust pos (4,39,1) uv (0.12496094,0.062460937) rgba ffffffff light 0 (rechecked by verify_render10) |
| verify10 | save format: Java saves, Rust loads | DIFFERS | comparison | block: (174,63,107) java=111 () rust=0 () |
| verify10 | save format: Rust saves, Java loads | EXACT | comparison | EXACT MATCH |
| verify10 | server mode: 900-tick walk (real World.tick), unload/reload through region files | DIFFERS | checker failure | Checker failed before completing comparison: chunk identity stream exhausted; thread 'main' (37960) panicked at crates\worldgen10\src\world\tick.rs:26:83: (rechecked by verify_serverfix10) |
| verify10 | block showcase: fancy on, normal | DIFFERS | comparison | chunk (0,0) pass 0 vertex 1876: java pos (5,65,1) uv (0.062460937,0.062460937) rgba ff71b98e light 900090 \| rust pos (5,65,1) uv (0.062460937,0.062460937) rgba ff4ec46a light 900090 (rechecked by verify_render10) |
| verify10 | block showcase: fancy on, anaglyph | DIFFERS | comparison | chunk (0,0) pass 0 vertex 1876: java pos (5,65,1) uv (0.062460937,0.062460937) rgba ff79aca4 light 900090 \| rust pos (5,65,1) uv (0.062460937,0.062460937) rgba ff56a89c light 900090 (rechecked by verify_render10) |
| verify10 | block showcase: fancy off, normal | DIFFERS | comparison | chunk (0,0) pass 0 vertex 1876: java pos (5,65,1) uv (0.062460937,0.062460937) rgba ff71b98e light f00070 \| rust pos (5,65,1) uv (0.062460937,0.062460937) rgba ff4ec46a light f00070 (rechecked by verify_render10) |
| verify10 | block showcase: fancy off, anaglyph | DIFFERS | comparison | chunk (0,0) pass 0 vertex 1876: java pos (5,65,1) uv (0.062460937,0.062460937) rgba ff79aca4 light f00070 \| rust pos (5,65,1) uv (0.062460937,0.062460937) rgba ff56a89c light f00070 (rechecked by verify_render10) |
| verify10 | block showcase: fast on, normal | DIFFERS | comparison | chunk (0,0) pass 0 vertex 1876: java pos (5,65,1) uv (0.062460937,0.062460937) rgba ff71b98e light 900090 \| rust pos (5,65,1) uv (0.062460937,0.062460937) rgba ff4ec46a light 900090 (rechecked by verify_render10) |
| verify10 | block showcase: fast on, anaglyph | DIFFERS | comparison | chunk (0,0) pass 0 vertex 1876: java pos (5,65,1) uv (0.062460937,0.062460937) rgba ff79aca4 light 900090 \| rust pos (5,65,1) uv (0.062460937,0.062460937) rgba ff56a89c light 900090 (rechecked by verify_render10) |
| verify10 | block showcase: fast off, normal | DIFFERS | comparison | chunk (0,0) pass 0 vertex 1876: java pos (5,65,1) uv (0.062460937,0.062460937) rgba ff71b98e light f00070 \| rust pos (5,65,1) uv (0.062460937,0.062460937) rgba ff4ec46a light f00070 (rechecked by verify_render10) |
| verify10 | block showcase: fast off, anaglyph | DIFFERS | comparison | chunk (0,0) pass 0 vertex 1876: java pos (5,65,1) uv (0.062460937,0.062460937) rgba ff79aca4 light f00070 \| rust pos (5,65,1) uv (0.062460937,0.062460937) rgba ff56a89c light f00070 (rechecked by verify_render10) |
| verify10 | Nether terrain meshes (fire, glowstone, the Nether light table) | DIFFERS | comparison | chunk (-2,-2) pass 0 vertex 511: java pos (16,0,8) uv (0.12496094,0.12496094) rgba ff7f7f7f light 0 \| rust pos (16,0,8) uv (0.12496094,0.12496094) rgba ff7f7f7f light 3c0000 (rechecked by verify_render10) |
| verify10 | moving piston renderer (original TileEntityRendererPiston vertex bits) | DIFFERS | comparison | case 270 id 2/0 facing 0 head false ext true progress 0/0/0 ao false fancy false vertex 4 field 5: java ff71b98e, rust ff4ec46a (rechecked by verify_render10) |
| verify10 | redstone: basic | DIFFERS | comparison | tick 10, queriedBlock[32].metadata: Java 4, Rust 0 |
| verify10 | redstone: notes | DIFFERS | comparison | tiles[0].previousRedstoneState: Java 1, Rust 0 |
| verify10 | redstone: pistons_basic | EXACT | comparison | EXACT MATCH |
| verify10 | redstone: pistons_boxes | DIFFERS | comparison | property[2]: Java 1112014848, Rust 1092616192 |
| verify10 | redstone: pistons_breaking | EXACT | comparison | EXACT MATCH |
| verify10 | redstone: pistons_chunk_nbt | EXACT | comparison | EXACT MATCH |
| verify10 | redstone: pistons_clock | EXACT | comparison | EXACT MATCH |
| verify10 | redstone: pistons_entities | DIFFERS | comparison | entity[1]: Java 4623198344968142848, Rust 4623252146576425921 |
| verify10 | redstone: pistons_facing_0 | EXACT | comparison | EXACT MATCH |
| verify10 | redstone: pistons_facing_1 | EXACT | comparison | EXACT MATCH |
| verify10 | redstone: pistons_facing_2 | EXACT | comparison | EXACT MATCH |
| verify10 | redstone: pistons_facing_3 | EXACT | comparison | EXACT MATCH |
| verify10 | redstone: pistons_facing_4 | EXACT | comparison | EXACT MATCH |
| verify10 | redstone: pistons_facing_5 | EXACT | comparison | EXACT MATCH |
| verify10 | redstone: pistons_limits | EXACT | comparison | EXACT MATCH |
| verify10 | redstone: pistons_obstructions | EXACT | comparison | EXACT MATCH |
| verify10 | redstone: pistons_placement | EXACT | comparison | EXACT MATCH |
| verify10 | redstone: pistons_pull_failure | EXACT | comparison | EXACT MATCH |
| verify10 | redstone: pistons_pulses | EXACT | comparison | EXACT MATCH |
| verify10 | redstone: rails | DIFFERS | comparison | entities[0]: Java 0, Rust 1 |
| verify10 | redstone: repeaters | EXACT | comparison | EXACT MATCH |
| verify10 | redstone: tile_lifecycle | EXACT | comparison | EXACT MATCH |
| verify10 | mob models: boxes, UVs, successive poses and wolf living animations | DIFFERS | comparison | case 56: part count: Rust 6, Java 7 |
| verify10 | remaining entity renderers and GUI item geometry | DIFFERS | comparison | case 436 id 110: draw count: Rust 6, Java 1 |
| verify10 | ghast, pig zombie and giant full renderer draws | DIFFERS | comparison | living case 0 draw 0.vertex[0][3]: java 3f400000, rust 3f3f999a |
| verify10 | entity shadows and fire | EXACT | comparison | EFFECTS EXACT |
| verify10 | first-person view, player and block targeting | DIFFERS | comparison | case 320 kind 1 item 0 draw 0 texture: java 00000000, rust 00000009 (rechecked by verify_render10) |
| verify10 | renderer window sizes and Java negative-coordinate wrapping | EXACT | comparison | WINDOWS EXACT: 32 comparisons (rechecked by verify_latefix10) |
| verify10 | sky colors | DIFFERS | comparison | sky[0]: Java 1055846127, Rust 1056372471 (rechecked by verify_render10) |
| verify10 | original MusInputStream encrypted audio bytes | EXACT | comparison | MUS EXACT: 8 names, 524296 decoded bytes |
| verify10 | EntityPickupFX item and arrow renderer draws | EXACT | comparison | RENDERERS EXACT |
| verify10 | note instruments, jukebox insertion/ejection and every step/break sound | DIFFERS | comparison | byte 16577, ArrayList<byte[]> list=events.get(w);i(o,list==null?0:list.size());if(list!=null){for(byte[] b:list)o.write(b);list.clear();} = 2 (WorldEventOracle.java:37, field starts 16577, 4 bytes, Integer); Rust 0 |
| verify10 | all ambient block callbacks, metadata and adjacent faces | DIFFERS | comparison | byte 12, for(int t=0;t<100;t++) {Block.blocksList[id].randomDisplayTick(w,x,y,z,display);WorldEventOracle.snapshot(w,o);EntityOracle.l(o,EntityOracle.rngState(w.rand));EntityOracle.l(o,EntityOracle.rngState(display));} = 161802050106438 (ParticleOracle.java:122, field starts 12, 8 bytes, Long); Rust 69500526345287 |
| verify10 | original EntityFX motion and vertex bits | EXACT | comparison | particle checks: 144 cases, 465408 vertices, 0 differing fields |
| verify10 | EffectRenderer destruction, hits, tint, eviction and vertex bits | DIFFERS | comparison | block 18 metadata 0.particleTint: Java 0x71a74d, Rust 0x45b421 (rechecked by verify_harnessfix10) |
| verify10 | ambient weather vertices | DIFFERS | comparison | vertex 0 field 0 (3): Rust 1077936128, Java 0 (rechecked by verify_render10) |
| verify10 | Fast and Fancy clouds | EXACT | comparison | CLOUDS EXACT: 10752 vertices (Fancy) (rechecked by verify_render10) |
| verify10 | sky GL colors and sunrise vertex colors, anaglyph on/off | DIFFERS | comparison | sky case 0, GL color float bits: Rust 1060255561, Java 1060245798 (rechecked by verify_render10) |
| verify10 | statistics file | EXACT | comparison | STATS FILE EXACT: written and read by both, checksum rejection and unknown ids |
| verify10 | world conversion from the old chunk-file format | EXACT | comparison | CONVERSION EXACT: java chunks 289 rust chunks 289 identical data 289 identical sector layout 32, 307 progress steps |
| verify10 | decorations10: signs: placement, editing, NBT | EXACT | comparison | EXACT MATCH: signs placement, supports, drops/RNG, editing, NBT (rechecked by verify_constantfix10) |
| verify10 | decorations10: sign renderer (TileEntitySignRenderer) | DIFFERS | comparison | sign 0 draw 0.vertex[0][3]: java 3ee00000, rust 3edf3333 (rechecked by verify_constantfix10) |
| verify10 | decorations10: paintings | EXACT | comparison | EXACT MATCH: paintings selection, walls, overlaps, support ticks, damage, NBT (rechecked by verify_constantfix10) |
| verify10 | decorations10: painting renderer | DIFFERS | comparison | painting 0 draw 0.color/normal[0]: java 3f800000, rust 00000000 (rechecked by verify_constantfix10) |
| verify10 | decorations10: maps: palette, terrain, markers, NBT | DIFFERS | comparison | byte 356, Oracle.DataOut o=new Oracle.DataOut();for(int id=0;id<256;id++)o.i32(Block.blocksList[id]==null?0:Block.blocksList[id].blockMaterial.materialMapColor.colorIndex); = 0 (DecorationOracle.java:30, field starts 356, 4 bytes, Integer); Rust 11 (rechecked by verify_constantfix10) |
| verify10 | decorations10: map files | EXACT | comparison | EXACT MATCH (rechecked by verify_constantfix10) |
| verify10 | decorations10: map files read back | EXACT | comparison | EXACT MATCH (rechecked by verify_constantfix10) |
| verify10 | decorations10: fishing | DIFFERS | comparison | byte 10979121, EntityOracle.l(o,EntityOracle.rngState(f.rand));EntityOracle.l(o,EntityOracle.rngState(w.rand));EntityOracle.l(o,EntityOracle.rngState(EntityOracle.mathRandom())); = 211094699171000 (DecorationOracle.java:75, field starts 10979121, 8 bytes, Long); Rust 25214904129 (rechecked by verify_constantfix10) |
| verify10 | decorations10: fishing line renderer | EXACT | comparison | fish renderers: 22920 values, 0 differences (rechecked by verify_constantfix10) |
| verify10 | lightmap texels (EntityRenderer.func_35808_e): day, night, rain, thunder, flashes, flicker, brightness | EXACT | comparison | LIGHTMAP EXACT (rechecked by verify_render10) |
| verify10 | world-blocks 1.0: plants | DIFFERS | comparison | byte 36595, w.getBlockMetadata(x,y,z) [big endian] = 1 (Mode_worldblocks10.java:59, field starts 36595, 1 bytes, Integer); Rust 0 |
| verify10 | world-blocks 1.0: geometry | DIFFERS | comparison | byte 1256257, ((BlockFence)Block.fence).isFenceAt(w,8,64,8) = true (Mode_worldblocks10.java:78, field starts 1256257, 1 bytes, Boolean); Rust False |
| verify10 | world-blocks 1.0: drops | DIFFERS | comparison | byte 12, i(o,e.item.itemID);i(o,e.item.stackSize);i(o,e.item.getItemDamage()); = 53 (Mode_worldblocks10.java:88, field starts 12, 4 bytes, Integer); Rust 5 |
| verify10 | world-blocks 1.0: saves | EXACT | comparison | EXACT MATCH: worldblocks181 saves, 1464 bytes |
| verify10 | world-blocks 1.0: lights | DIFFERS | comparison | byte 9605799, i(o,w.getLightBrightnessForSkyBlocks(x,y,z,t%16)); = 8388784 (Mode_worldblocks10.java:129, field starts 9605799, 4 bytes, Integer); Rust 96 |
| verify10 | world-blocks 1.0: chests | DIFFERS | comparison | byte 40353, sound[0].equals("random.door_open") = false (Mode_worldblocks10.java:162, field starts 40353, 1 bytes, Boolean); Rust True |
| verify10 | world-blocks 1.0: helpers | EXACT | comparison | EXACT MATCH: worldblocks181 helpers, 263588 bytes |
| verify10 | world-blocks 1.0: fluids | DIFFERS | comparison | byte 0, b.randomDisplayTick(w,8,64,8,r);l(o,EntityOracle.rngState(r));i(o,events.size()); = 75607230952671 (Mode_worldblocks10.java:200, field starts 0, 8 bytes, Long); Rust 193527703344036 |
| verify10 | world-blocks 1.0: teleporter | EXACT | comparison | EXACT MATCH: worldblocks181 teleporter, 1129872 bytes |
| verify10 | world-blocks 1.0: items | DIFFERS | comparison | byte 2729, s!=null = true (Mode_worldblocks10.java:241, field starts 2729, 1 bytes, Boolean); Rust False |
| verify10 | world-blocks 1.0: actions | DIFFERS | comparison | byte 1308, for(int id=0;id<=2257;id++)if(Item.itemsList[id]!=null){ItemStack s=new ItemStack(id,1,0);i(o,id);i(o,s.getItem().getItemUseAction(s).ordinal());i(o,s.getItem().getMaxItemUseDuration(s));} = 110 (Mode_worldblocks10.java:252, field starts 1308, 4 bytes, Integer); Rust 256 |
| verify10 | world-blocks 1.0: registry | DIFFERS | comparison | byte 142560, i(o,a==null?0:a.getItemStackLimit());i(o,a==null?0:a.getMaxDamage());i(o,a!=null&&a.getHasSubtypes()?1:0);i(o,a instanceof ItemArmor?((ItemArmor)a).damageReduceAmount:0); = 64 (PlayerOracle.java:207, field starts 142560, 4 bytes, Integer); Rust 0 |
| verify10 | world-blocks 1.0: definitions | DIFFERS | comparison | MATERIALS array length: Java 30, Rust 29 |
| verify10 | world-blocks 1.0: region cross-read | EXACT | comparison | EXACT MATCH: region writes, sector reuse/size deltas, and both Java/Rust readers (190406 bytes) |
| verify10 | world-blocks 1.0: mushrooms | DIFFERS | comparison | byte 42805, Trace10.writeBoolean(o, "Item.dyePowder.onItemUse(s,p,w,8,y,8,1)", Item.dyePowder.onItemUse(s,p,w,8,y,8,1));EntityOracle.i(o,s.stackSize);EntityOracle.l(o,EntityOracle.rngState(w.rand)); = 234029835660143 (Mode_wbmushrooms.java:15, field starts 42805, 8 bytes, Long); Rust 25214903917 |
| verify10 | world-blocks 1.0: chestlife | EXACT | comparison | EXACT MATCH: worldblocks181 chestlife, 108062 bytes |
| verify10 | world-blocks 1.0: math | EXACT | comparison | EXACT MATCH: worldblocks181 math, 1060373 bytes |
| verify10 | world-blocks 1.0: storage | EXACT | comparison | EXACT MATCH: worldblocks181 storage, 63957 bytes |
| verify10 | world-blocks 1.0: silverfish | DIFFERS | comparison | byte 36, for(double v:new double[]{Trace10.namedDouble("e.posX", e.posX),Trace10.namedDouble("e.posY", e.posY),Trace10.namedDouble("e.posZ", e.posZ)})EntityOracle.d(o,v);EntityOracle.f(o,e.width);EntityOracle.f(o,e.height);EntityOracle.i(o,((EntityLiving)e).health);EntityOracle.l(o,EntityOracle.rngState(e.rand));EntityOracle.l(o,EntityOracle.rngState(EntityOracle.mathRandom())); = 8 (Mode_wbsilverfish.java:12, field starts 36, 4 bytes, Integer); Rust 20 |
| verify10 | world-blocks 1.0: visual | DIFFERS | comparison | byte 47548, for(int side=0;side<6;side++){int v=0;try{v=b==null?0:b.getBlockTextureFromSideAndMetadata(side,meta);}catch(ArrayIndexOutOfBoundsException e){v=Integer.MIN_VALUE;}EntityOracle.i(o,v);} = 126 (Mode_wbvisual.java:14, field starts 47548, 4 bytes, Integer); Rust 142 |
| verify10 | world-blocks 1.0: colors | DIFFERS | comparison | byte 69268, b.colorMultiplier(access,x,64,z) = 15006747 (Mode_wbcolors.java:19, field starts 69268, 4 bytes, Integer); Rust 10088546 (rechecked by verify_harnessfix10) |
| verify10 | world-blocks 1.0: spawn | DIFFERS | comparison | byte 10809, p.canCoordinateBeSpawn(8,8) = true (Mode_wbspawn.java:23, field starts 10809, 1 bytes, Boolean); Rust False |
| verify10 | world-blocks 1.0: relocation | DIFFERS | comparison | byte 0, c!=null = true (Mode_wbrelocation.java:11, field starts 0, 1 bytes, Boolean); Rust False |
| verify10 | world-blocks 1.0: cache | DIFFERS | comparison | byte 34421, EntityOracle.i(o,c.getSkyBlockTypeBrightness(k,x,y,z));EntityOracle.i(o,c.getSpecialBlockBrightness(k,x,y,z)); = 13 (Mode_wbcache.java:16, field starts 34421, 4 bytes, Integer); Rust 0 |
| verify10 | world-blocks 1.0: textures | DIFFERS | comparison | byte 151872, try {EntityOracle.i(o,b.getBlockTexture(w,8,64,8,side));}catch(ArrayIndexOutOfBoundsException e){EntityOracle.i(o,Integer.MIN_VALUE);} = 126 (Mode_wbtextures.java:5, field starts 151872, 4 bytes, Integer); Rust 142 |
| verify10 | world-blocks 1.0: weather | EXACT | comparison | EXACT MATCH: worldblocks181 weather, 3276800 bytes |
| verify10 | world-blocks 1.0: arrays | DIFFERS | comparison | byte 293781, loadedChunk[3].afterFinalize.blocks[12]: Java 119, Rust 0 |
| verify10 | world-blocks 1.0: savelife | EXACT | comparison | EXACT MATCH: worldblocks181 savelife, 445 bytes |
| verify10 | world-blocks 1.0: sides | EXACT | comparison | EXACT MATCH: worldblocks181 sides, 867128 bytes |
| verify10 | world-blocks 1.0: nextticks | EXACT | comparison | EXACT MATCH: worldblocks181 nextticks, 3871336 bytes |
| verify10 | world-blocks 1.0: frost | DIFFERS | comparison | byte 0, w.updateBlocksAndPlayCaveSounds();EntityOracle.l(o,EntityOracle.rngState(w.rand));EntityOracle.i(o,w.updateLCG);EntityOracle.i(o,field("soundCounter").getInt(w)); = 246107133972888 (Mode_wbfrost.java:20, field starts 0, 8 bytes, Long); Rust 248619153362371 |
| verify10 | world-blocks 1.0: fileclasses | DIFFERS | comparison | byte 3623, EntityOracle.i(o,x.compareTo(y));EntityOracle.l(o,x.getLastTimePlayed());EntityOracle.l(o,x.getLastTimePlayed());EntityOracle.i(o,x.getGameType());Trace10.writeBoolean(o, "x.func_22161_d()", x.func_22161_d());Trace10.writeBoolean(o, "x.getFileName().equals(left)", x.getFileName().equals(left));Trace10.writeBoolean(o, "x.getDisplayName().equals(\"display\")", x.getDisplayName().equals("display")); = -9223372036854775808 (Mode_wbfileclasses.java:22, field starts 3623, 8 bytes, Long); Rust -1 |
| verify10 | world-blocks 1.0: placement4 | DIFFERS | comparison | byte 5969, cells(o,w,6,10,63,67);EntityOracle.l(o,EntityOracle.rngState(w.rand));EntityOracle.l(o,EntityOracle.rngState(itemRand));WorldEventOracle.snapshot(w,o); = 192310312797552 (Mode_wbplacement4.java:29, field starts 5969, 8 bytes, Long); Rust 86896799908266 |
| verify10 | world-blocks 1.0: fire4 | EXACT | comparison | EXACT MATCH: worldblocks181 fire4, 1528320 bytes |
| verify10 | world-blocks 1.0: harvest4 | DIFFERS | comparison | byte 960, Block.blocksList[id].harvestBlock(w,p,8,64,8,meta);EntityOracle.l(o,EntityOracle.rngState(w.rand));EntityOracle.i(o,w.loadedEntityList.size()); = 192310312797552 (Mode_wbharvest4.java:11, field starts 960, 8 bytes, Long); Rust 86896799908266 |
| verify10 | world-blocks 1.0: regioncache4 | EXACT | comparison | EXACT MATCH: worldblocks181 regioncache4, 30107 bytes |
| verify10 | world-blocks 1.0: oldformat4 | DIFFERS | comparison | byte 41, for(Object object:worlds) {SaveFormatComparator w=(SaveFormatComparator)object;text(o,w.getFileName());text(o,w.getDisplayName());EntityOracle.l(o,w.getLastTimePlayed());EntityOracle.l(o,w.getLastTimePlayed());EntityOracle.i(o,w.getGameType());Trace10.writeBoolean(o, "w.func_22161_d()", w.func_22161_d());} = 100 (Mode_wboldformat4.java:17, field starts 41, 8 bytes, Long); Rust 1000 |
| verify10 | world-blocks 1.0: sounds4 | EXACT | comparison | EXACT MATCH: worldblocks181 sounds4, 5192 bytes |
| verify10 | world-blocks 1.0: emptycache4 | EXACT | comparison | EXACT MATCH: worldblocks181 emptycache4, 16560 bytes |
| verify10 | world-blocks 1.0: plates4 | DIFFERS | comparison | byte 1370, Trace10.writeByte(o, "w.getBlockId(8,64,8)", w.getBlockId(8,64,8));EntityOracle.i(o,w.getBlockMetadata(8,64,8));EntityOracle.l(o,EntityOracle.rngState(w.rand));EntityOracle.i(o,w.loadedEntityList.size());WorldEventOracle.snapshot(w,o); = 77990658783604 (Mode_wbplates4.java:11, field starts 1370, 8 bytes, Long); Rust 265170128171373 |
| verify10 | world-blocks 1.0: shears4 | EXACT | comparison | EXACT MATCH: worldblocks181 shears4, 8772 bytes |
| verify10 | world-blocks 1.0: placepred4 | DIFFERS | comparison | byte 37962, w.canBlockBePlacedAt(id,8,64,8,false,side) = true (Mode_wbplacepred4.java:8, field starts 37962, 1 bytes, Boolean); Rust False |
| verify10 | world-blocks 1.0: growth4 | DIFFERS | comparison | byte 784, Mode_wbplacement4.cells(o,w,5,11,61,68);EntityOracle.l(o,EntityOracle.rngState(w.rand));EntityOracle.i(o,w.loadedEntityList.size()); = 266686242967860 (Mode_wbgrowth4.java:16, field starts 784, 8 bytes, Long); Rust 222348659820101 |
| verify10 | world-blocks 1.0: doors4 | DIFFERS | comparison | byte 534, w.getBlockMetadata(x,y,z) [big endian] = 3 (Mode_wbplacement4.java:14, field starts 534, 1 bytes, Integer); Rust 7 |
| verify10 | world-blocks 1.0: furnace4 | EXACT | comparison | EXACT MATCH: worldblocks181 furnace4, 714240 bytes |
| verify10 | world-blocks 1.0: dispenser4 | EXACT | comparison | EXACT MATCH: worldblocks181 dispenser4, 257700 bytes |
| verify10 | world-blocks 1.0: activation4 | EXACT | comparison | EXACT MATCH: worldblocks181 activation4, 461 bytes |
| verify10 | world-blocks 1.0: record4 | EXACT | comparison | EXACT MATCH: worldblocks181 record4, 487 bytes |
| verify10 | world-blocks 1.0: legacy5 | DIFFERS | comparison | byte 603, loadFixture[0].chunkPresent: Java 1, Rust 0 |
| verify10 | world-blocks 1.0: piston5 | DIFFERS | comparison | byte 22171, EntityOracle.l(o,EntityOracle.rngState(w.rand));WorldEventOracle.snapshot(w,o); = 77616182221372 (Mode_wbpiston5.java:16, field starts 22171, 8 bytes, Long); Rust 107422979471445 |
| verify10 | world-blocks 1.0: removal5 | EXACT | comparison | EXACT MATCH: worldblocks181 removal5, 9053 bytes |
| verify10 | world-blocks 1.0: mchash5 | EXACT | comparison | EXACT MATCH: worldblocks181 mchash5, 3207268 bytes |
| verify10 | world-blocks 1.0: chestopen5 | EXACT | comparison | EXACT MATCH: worldblocks181 chestopen5, 164541 bytes |
| verify10 | world-blocks 1.0: flow5 | EXACT | comparison | EXACT MATCH: worldblocks181 flow5, 393216 bytes |
| verify10 | world-blocks 1.0: slabs6 | DIFFERS | comparison | byte 67616, w.getBlockId(x,y,z) [big endian] = 44 (Mode_wbplacement4.java:14, field starts 67616, 1 bytes, Integer); Rust 43 |
| verify10 | world-blocks 1.0: trapdoors6 | DIFFERS | comparison | byte 14824, EntityOracle.rngState(w.rand) = 77990658783604 (Mode_wbtrapdoors6.java:6, field starts 14824, 8 bytes, Long); Rust 187335379451726 (rechecked by verify_harnessfix10) |
| verify10 | world-blocks 1.0: doors6 | DIFFERS | comparison | byte 4443, w.getBlockId(x,y,z) [big endian] = 0 (Mode_wbplacement4.java:14, field starts 4443, 1 bytes, Integer); Rust 76 |
| verify10 | world-blocks 1.0: attached6 | EXACT | comparison | EXACT MATCH: worldblocks181 attached6, 201984 bytes |
| verify10 | world-blocks 1.0: drops6 | DIFFERS | comparison | byte 821832, EntityOracle.rngState(w.rand) = 234029835660143 (Mode_wbtrapdoors6.java:6, field starts 821832, 8 bytes, Long); Rust 205749139540596 |
| verify10 | world-blocks 1.0: conversion6 | EXACT | comparison | EXACT MATCH: worldblocks181 conversion6, 10912 bytes |
| verify10 | world-blocks 1.0: push6 | DIFFERS | comparison | byte 832583, e.posZ = 9.0 (Mode_wbpush6.java:17, field starts 832578, 8 bytes, Double); Rust 8.75 |
| verify10 | world-blocks 1.0: unload6 | DIFFERS | comparison | byte 1626, sound.name: Java 'random.chestopen', Rust 'random.door_open' |
| verify10 | world-blocks 1.0: music6 | DIFFERS | comparison | byte 603, t.previousRedstoneState = true (Mode_wbmusic6.java:19, field starts 603, 1 bytes, Boolean); Rust False |
| verify10 | world-blocks 1.0: materials6 | EXACT | comparison | EXACT MATCH: worldblocks181 materials6, 2880 bytes |
| verify10 | world-blocks 1.0: dispenser6 | DIFFERS | comparison | byte 81792, s.stackSize = 3 (Mode_wbfurnace4.java:5, field starts 81792, 4 bytes, Integer); Rust 2 |
| verify10 | world-blocks 1.0: base6 | EXACT | comparison | EXACT MATCH: worldblocks181 base6, 570024 bytes |
| verify10 | world-blocks 1.0: piston6 | DIFFERS | comparison | byte 487452, w.getBlockId(x,y,z) [big endian] = 76 (Mode_wbplacement4.java:14, field starts 487452, 1 bytes, Integer); Rust 0 |
| verify10 | world-blocks 1.0: furnace7 | DIFFERS | comparison | byte 58619, old.field_35146_o==null?0:old.field_35146_o.blockID = 61 (Mode_wbfurnace7.java:20, field starts 58619, 4 bytes, Integer); Rust 0 |
| verify10 | world-blocks 1.0: inventory7 | DIFFERS | comparison | byte 133009, (Integer)fuel.invoke(new TileEntityFurnace(),new ItemStack(id,-1,32767)) = 0 (Mode_wbinventory7.java:50, field starts 133009, 4 bytes, Integer); Rust 1600 |
| verify10 | world-blocks 1.0: container7 | EXACT | comparison | EXACT MATCH: worldblocks181 container7, 44710 bytes |
| verify10 | world-blocks 1.0: machines7 | DIFFERS | comparison | byte 1057, s != null = true (ContainerOracle.java:132, field starts 1057, 1 bytes, Boolean); Rust False |
| verify10 | world-blocks 1.0: ore7 | EXACT | comparison | EXACT MATCH: worldblocks181 ore7, 515072 bytes |
| verify10 | world-blocks 1.0: thin7 | DIFFERS | comparison | byte 315913, EntityOracle.rngState(w.rand) = 77990658783604 (Mode_wbtrapdoors6.java:6, field starts 315913, 8 bytes, Long); Rust 265170128171373 |
| verify10 | world-blocks 1.0: cake10 | EXACT | comparison | EXACT MATCH: worldblocks181 cake181, 6336 bytes |
| verify10 | world-blocks 1.0: trapdoor8 | DIFFERS | comparison | byte 593, b.canPlaceBlockOnSide(w,8,64,8,side) = false (Mode_wbtrapdoor8.java:17, field starts 593, 1 bytes, Boolean); Rust True |
| verify10 | world-blocks 1.0: repeater8 | DIFFERS | comparison | byte 540, b.canPlaceBlockAt(w,8,64,8) = false (Mode_wbrepeater8.java:18, field starts 540, 1 bytes, Boolean); Rust True |
| verify10 | world-blocks 1.0: reed8 | DIFFERS | comparison | byte 188394, EntityOracle.rngState(w.rand) = 77990658783604 (Mode_wbtrapdoors6.java:6, field starts 188394, 8 bytes, Long); Rust 265170128171373 |
| verify10 | world-blocks 1.0: cactus8 | DIFFERS | comparison | byte 50467, EntityOracle.rngState(w.rand) = 77990658783604 (Mode_wbtrapdoors6.java:6, field starts 50467, 8 bytes, Long); Rust 265170128171373 |
| verify10 | world-blocks 1.0: wire8 | DIFFERS | comparison | byte 216, b.canPlaceBlockAt(w,8,64,8) = false (Mode_wbwire8.java:11, field starts 216, 1 bytes, Boolean); Rust True |
| verify10 | world-blocks 1.0: stairs8 | EXACT | comparison | EXACT MATCH: worldblocks181 stairs8, 146784 bytes |
| verify10 | world-blocks 1.0: spawner8 | EXACT | comparison | EXACT MATCH: worldblocks181 spawner8, 11265 bytes |
| verify10 | world-blocks 1.0: sign8 | DIFFERS | comparison | byte 4336, EntityOracle.rngState(w.rand) = 42772445787197 (Mode_wbtrapdoors6.java:6, field starts 4336, 8 bytes, Long); Rust 130506877387094 |
| verify10 | world-blocks 1.0: cake9 | DIFFERS | comparison | byte 75503, EntityOracle.rngState(w.rand) = 192310312797552 (Mode_wbtrapdoors6.java:6, field starts 75503, 8 bytes, Long); Rust 86896799908266 |
| verify10 | world-blocks 1.0: plate9 | EXACT | comparison | EXACT MATCH: worldblocks181 plate9, 1378602 bytes |
| verify10 | world-blocks 1.0: door9 | DIFFERS | comparison | byte 7902, w.getBlockId(x,y,z) [big endian] = 0 (Mode_wbplacement4.java:14, field starts 7902, 1 bytes, Integer); Rust 76 |
| verify10 | world-blocks 1.0: extension9 | DIFFERS | comparison | byte 27042, w.getBlockId(x,y,z) [big endian] = 34 (Mode_wbplacement4.java:14, field starts 27042, 1 bytes, Integer); Rust 0 |
| verify10 | world-blocks 1.0: bed9 | DIFFERS | comparison | byte 546, w.getBlockId(x,y,z) [big endian] = 0 (Mode_wbplacement4.java:14, field starts 546, 1 bytes, Integer); Rust 26 |
| verify10 | world-blocks 1.0: sponge10 | EXACT | comparison | EXACT MATCH: worldblocks181 sponge181, 832 bytes |
| verify10 | world-blocks 1.0: furnacetile10 | DIFFERS | comparison | byte 3521892, b.size() = 117 (Mode_wbinventory7.java:9, field starts 3521892, 4 bytes, Integer); Rust 81 |
| verify10 | world-blocks 1.0: dispensertile10 | DIFFERS | comparison | byte 2173882, b.size() = 88 (Mode_wbinventory7.java:9, field starts 2173882, 4 bytes, Integer); Rust 52 |
| verify10 | world-blocks 1.0: jukebox10 | DIFFERS | comparison | byte 2488, w.getChunkFromChunkCoords(0,0).isModified = true (Mode_wbjukebox10.java:26, field starts 2488, 1 bytes, Boolean); Rust False |
| verify10 | world-blocks 1.0: tileentity10 | DIFFERS | comparison | byte 6225, if(op==0\|\|op==1){int e=0,v=0;try{if(op==0)v=t.getBlockMetadata();else{Block b=t.getBlockType();v=b==null?0:b.blockID;}}catch(Throwable ex){e=Mode_wbmachine10.error(ex);}Trace10.writeByte(o, "e", e);if(e==0)EntityOracle.i(o,v);} = 110 (Mode_wbtileentity10.java:19, field starts 6225, 4 bytes, Integer); Rust 0 |
| verify10 | world-blocks 1.0: pistontile10 | DIFFERS | comparison | byte 2925951, e [big endian] = 0 (Mode_wbpistontile10.java:34, field starts 2925951, 1 bytes, Integer); Rust 3 |
| verify10 | GenLayer: 0 -37 -29 63 41 | DIFFERS | comparison | byte 3, GenLayer result[0] at (-37,-29): Java 0, Rust 5 |
| verify10 | GenLayer: 3257840388504953787 -256 -512 79 63 | DIFFERS | comparison | byte 2979, GenLayer result[0] at (-223,-503): Java 7, Rust 3 |
| verify10 | GenLayer: -4172144997902289642 300 -100 43 29 | DIFFERS | comparison | byte 19952, GenLayer result array length: Java 6235, Rust 4988 cells |
| verify10 | GenLayer: 9223372036854775807 -1 -1 17 13 | DIFFERS | comparison | byte 3, GenLayer result[0] at (-1,-1): Java 4, Rust 0 |
| verify10 | GenLayer: -9223372036854775808 12550784 -12550784 31 17 | DIFFERS | comparison | byte 3, GenLayer result[0] at (12550784,-12550784): Java 0, Rust 6 |
| verify10 | biomes seed 0 | DIFFERS | oracle failure | Oracle failed before comparison: java.lang.ArrayIndexOutOfBoundsException: 44672; at net.minecraft.src.WorldChunkManager.func_35556_a(WorldChunkManager.java:264) (rechecked by verify_biomes10) |
| verify10 | spawn seed 0 | DIFFERS | comparison | byte 3, pos.posX [big endian] = 251 (Mode_spawn10.java:8, field starts 0, 4 bytes, Integer); Rust 56 |
| verify10 | utilities seed 0 | DIFFERS | comparison | byte 32325, b.maxHeight [big endian] = 0.4 (Mode_terrainutils10.java:15, field starts 32325, 4 bytes, Float); Rust 0.5 |
| verify10 | cache seed 0 | EXACT | comparison | EXACT MATCH (1410 bytes) |
| verify10 | starts seed 0 | EXACT | comparison | EXACT MATCH (10148 bytes) |
| verify10 | biomes seed 3257840388504953787 | DIFFERS | oracle failure | Oracle failed before comparison: java.lang.ArrayIndexOutOfBoundsException: 36915; at net.minecraft.src.WorldChunkManager.func_35556_a(WorldChunkManager.java:264) (rechecked by verify_biomes10) |
| verify10 | spawn seed 3257840388504953787 | DIFFERS | comparison | byte 0, pos.posX [big endian] = 100 (Mode_spawn10.java:8, field starts 0, 4 bytes, Integer); Rust -244 |
| verify10 | utilities seed 3257840388504953787 | DIFFERS | comparison | byte 32325, b.maxHeight [big endian] = 0.4 (Mode_terrainutils10.java:15, field starts 32325, 4 bytes, Float); Rust 0.5 |
| verify10 | cache seed 3257840388504953787 | EXACT | comparison | EXACT MATCH (1410 bytes) |
| verify10 | starts seed 3257840388504953787 | EXACT | comparison | EXACT MATCH (9420 bytes) |
| verify10 | biomes seed -4172144997902289642 | DIFFERS | oracle failure | Oracle failed before comparison: java.lang.ArrayIndexOutOfBoundsException: 52428; at net.minecraft.src.WorldChunkManager.func_35556_a(WorldChunkManager.java:264) (rechecked by verify_biomes10) |
| verify10 | spawn seed -4172144997902289642 | DIFFERS | comparison | byte 0, pos.posX [big endian] = -184 (Mode_spawn10.java:8, field starts 0, 4 bytes, Integer); Rust 184 |
| verify10 | utilities seed -4172144997902289642 | DIFFERS | comparison | byte 32325, b.maxHeight [big endian] = 0.4 (Mode_terrainutils10.java:15, field starts 32325, 4 bytes, Float); Rust 0.5 |
| verify10 | cache seed -4172144997902289642 | EXACT | comparison | EXACT MATCH (1410 bytes) |
| verify10 | starts seed -4172144997902289642 | EXACT | comparison | EXACT MATCH (9476 bytes) |
| verify10 | biomes seed 9223372036854775807 | DIFFERS | oracle failure | Oracle failed before comparison: java.lang.ArrayIndexOutOfBoundsException: 19660; at net.minecraft.src.WorldChunkManager.func_35556_a(WorldChunkManager.java:264) (rechecked by verify_biomes10) |
| verify10 | spawn seed 9223372036854775807 | DIFFERS | comparison | byte 3, pos.posX [big endian] = 164 (Mode_spawn10.java:8, field starts 0, 4 bytes, Integer); Rust 112 |
| verify10 | utilities seed 9223372036854775807 | DIFFERS | comparison | byte 32325, b.maxHeight [big endian] = 0.4 (Mode_terrainutils10.java:15, field starts 32325, 4 bytes, Float); Rust 0.5 |
| verify10 | cache seed 9223372036854775807 | EXACT | comparison | EXACT MATCH (1410 bytes) |
| verify10 | starts seed 9223372036854775807 | EXACT | comparison | EXACT MATCH (9644 bytes) |
| verify10 | biomes seed -9223372036854775808 | DIFFERS | oracle failure | Oracle failed before comparison: java.lang.ArrayIndexOutOfBoundsException: 32768; at net.minecraft.src.WorldChunkManager.func_35556_a(WorldChunkManager.java:264) (rechecked by verify_biomes10) |
| verify10 | spawn seed -9223372036854775808 | DIFFERS | comparison | byte 0, pos.posX [big endian] = -352 (Mode_spawn10.java:8, field starts 0, 4 bytes, Integer); Rust 91 |
| verify10 | utilities seed -9223372036854775808 | DIFFERS | comparison | byte 32325, b.maxHeight [big endian] = 0.4 (Mode_terrainutils10.java:15, field starts 32325, 4 bytes, Float); Rust 0.5 |
| verify10 | cache seed -9223372036854775808 | EXACT | comparison | EXACT MATCH (1410 bytes) |
| verify10 | starts seed -9223372036854775808 | EXACT | comparison | EXACT MATCH (10148 bytes) |
| verify10 | Sky: 0 -4 -4 8 8 | DIFFERS | comparison | byte 18, chunk (-4,-4).block (0,18,0): Java 121, Rust 0 |
| verify10 | Sky: 3257840388504953787 -3 -3 6 6 | DIFFERS | comparison | byte 18, chunk (-3,-3).block (0,18,0): Java 121, Rust 0 |
| verify10 | Sky: -4172144997902289642 -2 -2 4 4 | DIFFERS | comparison | byte 19, chunk (-2,-2).block (0,19,0): Java 121, Rust 0 |
| verify10 | Sky: 42 784420 0 4 2 | DIFFERS | comparison | byte 22, chunk (784420,0).block (0,22,0): Java 0, Rust 1 |
| verify10 | Sky: 42 -3 -784431 2 4 | DIFFERS | comparison | byte 15256, chunk (-3,-784431).block (7,24,7): Java 0, Rust 2 |
| verify10 | Sky: 9223372036854775807 -1 -1 3 3 | DIFFERS | comparison | byte 17, chunk (-1,-1).block (0,17,0): Java 121, Rust 0 |
| verify10 | feature oak fixture 0 | EXACT | comparison | EXACT MATCH (2408469 bytes) |
| verify10 | feature oak fixture 1 | EXACT | comparison | EXACT MATCH (2408469 bytes) |
| verify10 | feature oak fixture 2 | EXACT | comparison | EXACT MATCH (2408469 bytes) |
| verify10 | feature oak fixture 3 | EXACT | comparison | EXACT MATCH (2408469 bytes) |
| verify10 | feature birch fixture 0 | EXACT | comparison | EXACT MATCH (2408469 bytes) |
| verify10 | feature birch fixture 1 | EXACT | comparison | EXACT MATCH (2408469 bytes) |
| verify10 | feature birch fixture 2 | EXACT | comparison | EXACT MATCH (2408469 bytes) |
| verify10 | feature birch fixture 3 | EXACT | comparison | EXACT MATCH (2408469 bytes) |
| verify10 | feature taiga1 fixture 0 | EXACT | comparison | EXACT MATCH (2408469 bytes) |
| verify10 | feature taiga1 fixture 1 | EXACT | comparison | EXACT MATCH (2408469 bytes) |
| verify10 | feature taiga1 fixture 2 | EXACT | comparison | EXACT MATCH (2408469 bytes) |
| verify10 | feature taiga1 fixture 3 | EXACT | comparison | EXACT MATCH (2408469 bytes) |
| verify10 | feature taiga2 fixture 0 | EXACT | comparison | EXACT MATCH (2408469 bytes) |
| verify10 | feature taiga2 fixture 1 | EXACT | comparison | EXACT MATCH (2408469 bytes) |
| verify10 | feature taiga2 fixture 2 | EXACT | comparison | EXACT MATCH (2408469 bytes) |
| verify10 | feature taiga2 fixture 3 | EXACT | comparison | EXACT MATCH (2408469 bytes) |
| verify10 | feature big fixture 0 | EXACT | comparison | EXACT MATCH (2408469 bytes) |
| verify10 | feature big fixture 1 | EXACT | comparison | EXACT MATCH (2408469 bytes) |
| verify10 | feature big fixture 2 | EXACT | comparison | EXACT MATCH (2408469 bytes) |
| verify10 | feature big fixture 3 | EXACT | comparison | EXACT MATCH (2408469 bytes) |
| verify10 | feature swamp fixture 0 | EXACT | comparison | EXACT MATCH (2408469 bytes) |
| verify10 | feature swamp fixture 1 | EXACT | comparison | EXACT MATCH (2408469 bytes) |
| verify10 | feature swamp fixture 2 | EXACT | comparison | EXACT MATCH (2408469 bytes) |
| verify10 | feature swamp fixture 3 | EXACT | comparison | EXACT MATCH (2408469 bytes) |
| verify10 | feature brown fixture 0 | EXACT | comparison | EXACT MATCH (2408469 bytes) |
| verify10 | feature brown fixture 1 | EXACT | comparison | EXACT MATCH (2408469 bytes) |
| verify10 | feature brown fixture 2 | EXACT | comparison | EXACT MATCH (2408469 bytes) |
| verify10 | feature brown fixture 3 | EXACT | comparison | EXACT MATCH (2408469 bytes) |
| verify10 | feature red fixture 0 | EXACT | comparison | EXACT MATCH (2408469 bytes) |
| verify10 | feature red fixture 1 | EXACT | comparison | EXACT MATCH (2408469 bytes) |
| verify10 | feature red fixture 2 | EXACT | comparison | EXACT MATCH (2408469 bytes) |
| verify10 | feature red fixture 3 | EXACT | comparison | EXACT MATCH (2408469 bytes) |
| verify10 | feature clay fixture 0 | EXACT | comparison | EXACT MATCH (2408469 bytes) |
| verify10 | feature clay fixture 1 | EXACT | comparison | EXACT MATCH (2408469 bytes) |
| verify10 | feature clay fixture 2 | EXACT | comparison | EXACT MATCH (2408469 bytes) |
| verify10 | feature clay fixture 3 | EXACT | comparison | EXACT MATCH (2408469 bytes) |
| verify10 | feature sand fixture 0 | EXACT | comparison | EXACT MATCH (2408469 bytes) |
| verify10 | feature sand fixture 1 | EXACT | comparison | EXACT MATCH (2408469 bytes) |
| verify10 | feature sand fixture 2 | EXACT | comparison | EXACT MATCH (2408469 bytes) |
| verify10 | feature sand fixture 3 | EXACT | comparison | EXACT MATCH (2408469 bytes) |
| verify10 | feature gravel fixture 0 | EXACT | comparison | EXACT MATCH (2408469 bytes) |
| verify10 | feature gravel fixture 1 | EXACT | comparison | EXACT MATCH (2408469 bytes) |
| verify10 | feature gravel fixture 2 | EXACT | comparison | EXACT MATCH (2408469 bytes) |
| verify10 | feature gravel fixture 3 | EXACT | comparison | EXACT MATCH (2408469 bytes) |
| verify10 | feature ore fixture 0 | EXACT | comparison | EXACT MATCH (2408469 bytes) |
| verify10 | feature ore fixture 1 | EXACT | comparison | EXACT MATCH (2408469 bytes) |
| verify10 | feature ore fixture 2 | EXACT | comparison | EXACT MATCH (2408469 bytes) |
| verify10 | feature ore fixture 3 | EXACT | comparison | EXACT MATCH (2408469 bytes) |
| verify10 | feature lake fixture 0 | EXACT | comparison | EXACT MATCH (2408469 bytes) |
| verify10 | feature lake fixture 1 | EXACT | comparison | EXACT MATCH (2408469 bytes) |
| verify10 | feature lake fixture 2 | EXACT | comparison | EXACT MATCH (2408469 bytes) |
| verify10 | feature lake fixture 3 | EXACT | comparison | EXACT MATCH (2408469 bytes) |
| verify10 | feature lava fixture 0 | EXACT | comparison | EXACT MATCH (2408469 bytes) |
| verify10 | feature lava fixture 1 | EXACT | comparison | EXACT MATCH (2408469 bytes) |
| verify10 | feature lava fixture 2 | EXACT | comparison | EXACT MATCH (2408469 bytes) |
| verify10 | feature lava fixture 3 | EXACT | comparison | EXACT MATCH (2408469 bytes) |
| verify10 | feature dungeon fixture 0 | EXACT | comparison | EXACT MATCH (2408469 bytes) |
| verify10 | feature dungeon fixture 1 | EXACT | comparison | EXACT MATCH (2408469 bytes) |
| verify10 | feature dungeon fixture 2 | EXACT | comparison | EXACT MATCH (2408469 bytes) |
| verify10 | feature dungeon fixture 3 | EXACT | comparison | EXACT MATCH (2408469 bytes) |
| verify10 | BiomeDecorator 0 fixture 0 | EXACT | comparison | EXACT MATCH (2408469 bytes) |
| verify10 | BiomeDecorator 1 fixture 0 | EXACT | comparison | EXACT MATCH (2408469 bytes) |
| verify10 | BiomeDecorator 2 fixture 0 | EXACT | comparison | EXACT MATCH (2408469 bytes) |
| verify10 | BiomeDecorator 3 fixture 0 | EXACT | comparison | EXACT MATCH (2408469 bytes) |
| verify10 | BiomeDecorator 4 fixture 0 | EXACT | comparison | EXACT MATCH (2408469 bytes) |
| verify10 | BiomeDecorator 5 fixture 0 | EXACT | comparison | EXACT MATCH (2408469 bytes) |
| verify10 | BiomeDecorator 6 fixture 0 | DIFFERS | comparison | byte 3, feature fixture.generatorRand: Java 83057626235731, Rust 215239943444238 |
| verify10 | BiomeDecorator 7 fixture 0 | EXACT | comparison | EXACT MATCH (2408469 bytes) |
| verify10 | BiomeDecorator 8 fixture 0 | EXACT | comparison | EXACT MATCH (2408469 bytes) |
| verify10 | BiomeDecorator 9 fixture 0 | DIFFERS | comparison | byte 3, feature fixture.generatorRand: Java 124234627262370, Rust 166243306382469 |
| verify10 | BiomeDecorator 0 fixture 1 | EXACT | comparison | EXACT MATCH (2408469 bytes) |
| verify10 | BiomeDecorator 1 fixture 1 | EXACT | comparison | EXACT MATCH (2408469 bytes) |
| verify10 | BiomeDecorator 2 fixture 1 | EXACT | comparison | EXACT MATCH (2408469 bytes) |
| verify10 | BiomeDecorator 3 fixture 1 | EXACT | comparison | EXACT MATCH (2408469 bytes) |
| verify10 | BiomeDecorator 4 fixture 1 | EXACT | comparison | EXACT MATCH (2408469 bytes) |
| verify10 | BiomeDecorator 5 fixture 1 | EXACT | comparison | EXACT MATCH (2408469 bytes) |
| verify10 | BiomeDecorator 6 fixture 1 | DIFFERS | comparison | byte 3, feature fixture.generatorRand: Java 20207844908520, Rust 139137832721542 |
| verify10 | BiomeDecorator 7 fixture 1 | EXACT | comparison | EXACT MATCH (2408469 bytes) |
| verify10 | BiomeDecorator 8 fixture 1 | EXACT | comparison | EXACT MATCH (2408469 bytes) |
| verify10 | BiomeDecorator 9 fixture 1 | DIFFERS | comparison | byte 3, feature fixture.generatorRand: Java 124234627262370, Rust 54349428006314 |
| verify10 | dungeon with successful chest placement | EXACT | comparison | EXACT MATCH (2408469 bytes) |
| verify10 | population animals 0 seed 4096 | EXACT | comparison | EXACT MATCH (2408469 bytes) |
| verify10 | population animals 1 seed 4096 | EXACT | comparison | EXACT MATCH (2408633 bytes) |
| verify10 | population animals 2 seed 4096 | EXACT | comparison | EXACT MATCH (2408469 bytes) |
| verify10 | population animals 3 seed 4096 | EXACT | comparison | EXACT MATCH (2408633 bytes) |
| verify10 | population animals 4 seed 4096 | EXACT | comparison | EXACT MATCH (2408617 bytes) |
| verify10 | population animals 5 seed 4096 | EXACT | comparison | EXACT MATCH (2408617 bytes) |
| verify10 | population animals 6 seed 4096 | EXACT | comparison | EXACT MATCH (2408633 bytes) |
| verify10 | population animals 7 seed 4096 | EXACT | comparison | EXACT MATCH (2408469 bytes) |
| verify10 | population animals 8 seed 4096 | EXACT | comparison | EXACT MATCH (2408469 bytes) |
| verify10 | population animals 9 seed 4096 | DIFFERS | comparison | byte 3, feature fixture.generatorRand: Java 25214908013, Rust 47669638442956 |
| verify10 | population animals 0 seed 4097 | EXACT | comparison | EXACT MATCH (2408469 bytes) |
| verify10 | population animals 1 seed 4097 | EXACT | comparison | EXACT MATCH (2408633 bytes) |
| verify10 | population animals 2 seed 4097 | EXACT | comparison | EXACT MATCH (2408469 bytes) |
| verify10 | population animals 3 seed 4097 | EXACT | comparison | EXACT MATCH (2408633 bytes) |
| verify10 | population animals 4 seed 4097 | EXACT | comparison | EXACT MATCH (2408617 bytes) |
| verify10 | population animals 5 seed 4097 | EXACT | comparison | EXACT MATCH (2408617 bytes) |
| verify10 | population animals 6 seed 4097 | EXACT | comparison | EXACT MATCH (2408633 bytes) |
| verify10 | population animals 7 seed 4097 | EXACT | comparison | EXACT MATCH (2408469 bytes) |
| verify10 | population animals 8 seed 4097 | EXACT | comparison | EXACT MATCH (2408469 bytes) |
| verify10 | population animals 9 seed 4097 | DIFFERS | comparison | byte 3, feature fixture.generatorRand: Java 25214908012, Rust 157852240777587 |
| verify10 | population animals 0 seed 4100 | EXACT | comparison | EXACT MATCH (2408469 bytes) |
| verify10 | population animals 1 seed 4100 | EXACT | comparison | EXACT MATCH (2408633 bytes) |
| verify10 | population animals 2 seed 4100 | EXACT | comparison | EXACT MATCH (2408469 bytes) |
| verify10 | population animals 3 seed 4100 | EXACT | comparison | EXACT MATCH (2408633 bytes) |
| verify10 | population animals 4 seed 4100 | EXACT | comparison | EXACT MATCH (2408617 bytes) |
| verify10 | population animals 5 seed 4100 | EXACT | comparison | EXACT MATCH (2408617 bytes) |
| verify10 | population animals 6 seed 4100 | EXACT | comparison | EXACT MATCH (2408633 bytes) |
| verify10 | population animals 7 seed 4100 | EXACT | comparison | EXACT MATCH (2408469 bytes) |
| verify10 | population animals 8 seed 4100 | EXACT | comparison | EXACT MATCH (2408469 bytes) |
| verify10 | population animals 9 seed 4100 | DIFFERS | comparison | byte 3, feature fixture.generatorRand: Java 25214908009, Rust 213689639411352 |
| verify10 | population animals 0 seed 4105 | EXACT | comparison | EXACT MATCH (2408469 bytes) |
| verify10 | population animals 1 seed 4105 | EXACT | comparison | EXACT MATCH (2408633 bytes) |
| verify10 | population animals 2 seed 4105 | EXACT | comparison | EXACT MATCH (2408469 bytes) |
| verify10 | population animals 3 seed 4105 | EXACT | comparison | EXACT MATCH (2408633 bytes) |
| verify10 | population animals 4 seed 4105 | EXACT | comparison | EXACT MATCH (2408617 bytes) |
| verify10 | population animals 5 seed 4105 | EXACT | comparison | EXACT MATCH (2408617 bytes) |
| verify10 | population animals 6 seed 4105 | EXACT | comparison | EXACT MATCH (2408633 bytes) |
| verify10 | population animals 7 seed 4105 | EXACT | comparison | EXACT MATCH (2408469 bytes) |
| verify10 | population animals 8 seed 4105 | EXACT | comparison | EXACT MATCH (2408469 bytes) |
| verify10 | population animals 9 seed 4105 | DIFFERS | comparison | byte 3, feature fixture.generatorRand: Java 25214908004, Rust 32263363637387 |
| verify10 | StructureComponent helpers seed 0 | DIFFERS | comparison | byte 2427376, p.func_35009_c(id,meta) [big endian] = 3 (Mode_component10.java:13, field starts 2427373, 4 bytes, Integer); Rust 2 |
| verify10 | structures layout seed 0 | DIFFERS | comparison | byte 7, b.minY [big endian] = 2 (Mode_structures10.java:9, field starts 4, 4 bytes, Integer); Rust 9 |
| verify10 | structures map seed 0 | DIFFERS | comparison | byte 23, g.coordMap.size() [big endian] = 0 (Mode_structures10.java:56, field starts 20, 4 bytes, Integer); Rust 1 |
| verify10 | structures blocks seed 0 | DIFFERS | comparison | byte 7, b.minY [big endian] = 2 (Mode_structures10.java:9, field starts 4, 4 bytes, Integer); Rust 9 |
| verify10 | structures village-layout seed 0 | EXACT | comparison | EXACT MATCH (33458 bytes) |
| verify10 | structures village-map seed 0 | DIFFERS | comparison | byte 4489, g.coordMap.size() [big endian] = 1 (Mode_structures10.java:53, field starts 4486, 4 bytes, Integer); Rust 2 |
| verify10 | structures village-blocks seed 0 | DIFFERS | comparison | byte 4027066, c.data.data[8032]: Java 0, Rust 64 (array starts 4019034) |
| verify10 | terrain round2 sky seed 0 | DIFFERS | comparison | byte 3, feature fixture.generatorRand: Java 43045035388969, Rust 99332111638576 |
| verify10 | terrain round2 skygrid seed 0 | DIFFERS | comparison | byte 2, EntityOracle.rngState((Random)f.get(gen)) [big endian] = 931132098519 (Mode_terrainround2.java:35, field starts 0, 8 bytes, Long); Rust 155465013058623 |
| verify10 | terrain round2 dungeon seed 0 | EXACT | comparison | EXACT MATCH (2408628 bytes) |
| verify10 | terrain round2 lifetime seed 0 | DIFFERS | comparison | byte 6042205, lifetime world[2], chunk (0,0).metadata (12,69,8): Java 8, Rust 0 |
| verify10 | terrain round2 colors seed 0 | DIFFERS | comparison | byte 289, b.getSkyColorByTemp(t) [big endian] = 0 (Mode_terrainround2.java:39, field starts 288, 4 bytes, Integer); Rust 12632319 |
| verify10 | terrain round2 locations seed 0 | DIFFERS | comparison | byte 3, p.chunkXPos [big endian] = -12 (Mode_terrainround2.java:27, field starts 0, 4 bytes, Integer); Rust -8 |
| verify10 | StructureComponent helpers seed 42 | DIFFERS | comparison | byte 2427340, p.func_35009_c(id,meta) [big endian] = 3 (Mode_component10.java:13, field starts 2427337, 4 bytes, Integer); Rust 2 |
| verify10 | structures layout seed 42 | DIFFERS | comparison | byte 3, b.minX [big endian] = -30 (Mode_structures10.java:9, field starts 0, 4 bytes, Integer); Rust -121 |
| verify10 | structures map seed 42 | DIFFERS | comparison | byte 11, g.coordMap.size() [big endian] = 0 (Mode_structures10.java:56, field starts 8, 4 bytes, Integer); Rust 1 |
| verify10 | structures blocks seed 42 | DIFFERS | comparison | byte 0, b.minX [big endian] = 2 (Mode_structures10.java:9, field starts 0, 4 bytes, Integer); Rust -89 |
| verify10 | structures village-layout seed 42 | EXACT | comparison | EXACT MATCH (30336 bytes) |
| verify10 | structures village-map seed 42 | DIFFERS | comparison | byte 239, g.coordMap.size() [big endian] = 0 (Mode_structures10.java:53, field starts 236, 4 bytes, Integer); Rust 1 |
| verify10 | structures village-blocks seed 42 | EXACT | comparison | EXACT MATCH (11062944 bytes) |
| verify10 | terrain round2 sky seed 42 | DIFFERS | comparison | byte 3, feature fixture.generatorRand: Java 110070860371171, Rust 89691232405205 |
| verify10 | terrain round2 skygrid seed 42 | DIFFERS | comparison | byte 2, EntityOracle.rngState((Random)f.get(gen)) [big endian] = 931132098519 (Mode_terrainround2.java:35, field starts 0, 8 bytes, Long); Rust 247313500694326 |
| verify10 | terrain round2 dungeon seed 42 | EXACT | comparison | EXACT MATCH (2408743 bytes) |
| verify10 | terrain round2 lifetime seed 42 | DIFFERS | comparison | byte 6044252, lifetime world[2], chunk (0,0).metadata (14,66,8): Java 8, Rust 0 |
| verify10 | terrain round2 colors seed 42 | DIFFERS | comparison | byte 289, b.getSkyColorByTemp(t) [big endian] = 0 (Mode_terrainround2.java:39, field starts 288, 4 bytes, Integer); Rust 12632319 |
| verify10 | terrain round2 locations seed 42 | DIFFERS | comparison | byte 3, p.chunkXPos [big endian] = -16 (Mode_terrainround2.java:27, field starts 0, 4 bytes, Integer); Rust -7 |
| verify10 | StructureComponent helpers seed 3257840388504953787 | DIFFERS | comparison | byte 2427388, p.func_35009_c(id,meta) [big endian] = 3 (Mode_component10.java:13, field starts 2427385, 4 bytes, Integer); Rust 2 |
| verify10 | structures layout seed 3257840388504953787 | DIFFERS | comparison | byte 3, b.minX [big endian] = -71 (Mode_structures10.java:9, field starts 0, 4 bytes, Integer); Rust -85 |
| verify10 | structures map seed 3257840388504953787 | DIFFERS | comparison | byte 14, EntityOracle.rngState(g.rand) [big endian] = 219807209278716 (Mode_structures10.java:56, field starts 12, 8 bytes, Long); Rust 50633988078354 |
| verify10 | structures blocks seed 3257840388504953787 | DIFFERS | comparison | byte 3, b.minX [big endian] = -39 (Mode_structures10.java:9, field starts 0, 4 bytes, Integer); Rust -53 |
| verify10 | structures village-layout seed 3257840388504953787 | EXACT | comparison | EXACT MATCH (30082 bytes) |
| verify10 | structures village-map seed 3257840388504953787 | DIFFERS | comparison | byte 599, g.coordMap.size() [big endian] = 1 (Mode_structures10.java:53, field starts 596, 4 bytes, Integer); Rust 0 |
| verify10 | structures village-blocks seed 3257840388504953787 | EXACT | comparison | EXACT MATCH (11062786 bytes) |
| verify10 | terrain round2 sky seed 3257840388504953787 | DIFFERS | comparison | byte 3, feature fixture.generatorRand: Java 118356361728322, Rust 241663834514792 |
| verify10 | terrain round2 skygrid seed 3257840388504953787 | DIFFERS | comparison | byte 2, EntityOracle.rngState((Random)f.get(gen)) [big endian] = 931132098519 (Mode_terrainround2.java:35, field starts 0, 8 bytes, Long); Rust 229799215442339 |
| verify10 | terrain round2 dungeon seed 3257840388504953787 | EXACT | comparison | EXACT MATCH (2408707 bytes) |
| verify10 | terrain round2 lifetime seed 3257840388504953787 | DIFFERS | comparison | byte 8450034, lifetime world[3], chunk (0,0).metadata (11,69,14): Java 8, Rust 0 |
| verify10 | terrain round2 colors seed 3257840388504953787 | DIFFERS | comparison | byte 289, b.getSkyColorByTemp(t) [big endian] = 0 (Mode_terrainround2.java:39, field starts 288, 4 bytes, Integer); Rust 12632319 |
| verify10 | terrain round2 locations seed 3257840388504953787 | DIFFERS | comparison | byte 3, p.chunkXPos [big endian] = -64 (Mode_terrainround2.java:27, field starts 0, 4 bytes, Integer); Rust -60 |
| verify10 | StructureComponent helpers seed -4172144997902289642 | DIFFERS | comparison | byte 2427400, p.func_35009_c(id,meta) [big endian] = 3 (Mode_component10.java:13, field starts 2427397, 4 bytes, Integer); Rust 2 |
| verify10 | structures layout seed -4172144997902289642 | DIFFERS | comparison | byte 3, b.minX [big endian] = -64 (Mode_structures10.java:9, field starts 0, 4 bytes, Integer); Rust -82 |
| verify10 | structures map seed -4172144997902289642 | DIFFERS | comparison | byte 23, g.coordMap.size() [big endian] = 1 (Mode_structures10.java:56, field starts 20, 4 bytes, Integer); Rust 2 |
| verify10 | structures blocks seed -4172144997902289642 | DIFFERS | comparison | byte 3, b.minX [big endian] = -32 (Mode_structures10.java:9, field starts 0, 4 bytes, Integer); Rust -50 |
| verify10 | structures village-layout seed -4172144997902289642 | EXACT | comparison | EXACT MATCH (31797 bytes) |
| verify10 | structures village-map seed -4172144997902289642 | DIFFERS | comparison | byte 151, b.minX [big endian] = -1267 (Mode_structures10.java:9, field starts 148, 4 bytes, Integer); Rust -1258 |
| verify10 | structures village-blocks seed -4172144997902289642 | DIFFERS | comparison | byte 4073732, c.data.data[7328]: Java 0, Rust 64 (array starts 4066404) |
| verify10 | terrain round2 sky seed -4172144997902289642 | DIFFERS | comparison | byte 3, feature fixture.generatorRand: Java 151275524314071, Rust 228530596792634 |
| verify10 | terrain round2 skygrid seed -4172144997902289642 | DIFFERS | comparison | byte 2, EntityOracle.rngState((Random)f.get(gen)) [big endian] = 931132098519 (Mode_terrainround2.java:35, field starts 0, 8 bytes, Long); Rust 112124450636948 |
| verify10 | terrain round2 dungeon seed -4172144997902289642 | EXACT | comparison | EXACT MATCH (2408731 bytes) |
| verify10 | terrain round2 lifetime seed -4172144997902289642 | DIFFERS | comparison | byte 8453169, lifetime world[3], chunk (0,0).metadata (14,66,15): Java 8, Rust 0 |
| verify10 | terrain round2 colors seed -4172144997902289642 | DIFFERS | comparison | byte 289, b.getSkyColorByTemp(t) [big endian] = 0 (Mode_terrainround2.java:39, field starts 288, 4 bytes, Integer); Rust 12632319 |
| verify10 | terrain round2 locations seed -4172144997902289642 | DIFFERS | comparison | byte 3, p.chunkXPos [big endian] = -19 (Mode_terrainround2.java:27, field starts 0, 4 bytes, Integer); Rust -20 |
| verify10 | StructureComponent helpers seed 9223372036854775807 | DIFFERS | comparison | byte 2427328, p.func_35009_c(id,meta) [big endian] = 3 (Mode_component10.java:13, field starts 2427325, 4 bytes, Integer); Rust 2 |
| verify10 | structures layout seed 9223372036854775807 | DIFFERS | comparison | byte 3, b.minX [big endian] = -100 (Mode_structures10.java:9, field starts 0, 4 bytes, Integer); Rust -116 |
| verify10 | structures map seed 9223372036854775807 | DIFFERS | comparison | byte 23, g.coordMap.size() [big endian] = 0 (Mode_structures10.java:56, field starts 20, 4 bytes, Integer); Rust 1 |
| verify10 | structures blocks seed 9223372036854775807 | DIFFERS | comparison | byte 3, b.minX [big endian] = -68 (Mode_structures10.java:9, field starts 0, 4 bytes, Integer); Rust -84 |
| verify10 | structures village-layout seed 9223372036854775807 | EXACT | comparison | EXACT MATCH (29555 bytes) |
| verify10 | structures village-map seed 9223372036854775807 | DIFFERS | comparison | byte 159, b.minZ [big endian] = -1321 (Mode_structures10.java:9, field starts 156, 4 bytes, Integer); Rust -1345 |
| verify10 | structures village-blocks seed 9223372036854775807 | DIFFERS | comparison | byte 7023880, c.data.data[7648]: Java 16, Rust 80 (array starts 7016232) |
| verify10 | terrain round2 sky seed 9223372036854775807 | DIFFERS | comparison | byte 3, feature fixture.generatorRand: Java 68223944728638, Rust 262429790806738 |
| verify10 | terrain round2 skygrid seed 9223372036854775807 | DIFFERS | comparison | byte 2, EntityOracle.rngState((Random)f.get(gen)) [big endian] = 931132098519 (Mode_terrainround2.java:35, field starts 0, 8 bytes, Long); Rust 100957603609058 |
| verify10 | terrain round2 dungeon seed 9223372036854775807 | EXACT | comparison | EXACT MATCH (2408501 bytes) |
| verify10 | terrain round2 lifetime seed 9223372036854775807 | DIFFERS | comparison | byte 8452145, lifetime world[3], chunk (0,0).metadata (13,67,15): Java 8, Rust 0 |
| verify10 | terrain round2 colors seed 9223372036854775807 | DIFFERS | comparison | byte 289, b.getSkyColorByTemp(t) [big endian] = 0 (Mode_terrainround2.java:39, field starts 288, 4 bytes, Integer); Rust 12632319 |
| verify10 | terrain round2 locations seed 9223372036854775807 | DIFFERS | comparison | byte 0, p.chunkXPos [big endian] = -5 (Mode_terrainround2.java:27, field starts 0, 4 bytes, Integer); Rust 2 |
| verify10 | StructureComponent helpers seed -9223372036854775808 | DIFFERS | comparison | byte 2427376, p.func_35009_c(id,meta) [big endian] = 3 (Mode_component10.java:13, field starts 2427373, 4 bytes, Integer); Rust 2 |
| verify10 | structures layout seed -9223372036854775808 | DIFFERS | comparison | byte 7, b.minY [big endian] = 2 (Mode_structures10.java:9, field starts 4, 4 bytes, Integer); Rust 9 |
| verify10 | structures map seed -9223372036854775808 | DIFFERS | comparison | byte 23, g.coordMap.size() [big endian] = 0 (Mode_structures10.java:56, field starts 20, 4 bytes, Integer); Rust 1 |
| verify10 | structures blocks seed -9223372036854775808 | DIFFERS | comparison | byte 7, b.minY [big endian] = 2 (Mode_structures10.java:9, field starts 4, 4 bytes, Integer); Rust 9 |
| verify10 | structures village-layout seed -9223372036854775808 | EXACT | comparison | EXACT MATCH (33458 bytes) |
| verify10 | structures village-map seed -9223372036854775808 | DIFFERS | comparison | byte 159, g.coordMap.size() [big endian] = 0 (Mode_structures10.java:53, field starts 156, 4 bytes, Integer); Rust 1 |
| verify10 | structures village-blocks seed -9223372036854775808 | DIFFERS | comparison | byte 4027066, c.data.data[8032]: Java 0, Rust 64 (array starts 4019034) |
| verify10 | terrain round2 sky seed -9223372036854775808 | DIFFERS | comparison | byte 3, feature fixture.generatorRand: Java 43045035388969, Rust 99332111638576 |
| verify10 | terrain round2 skygrid seed -9223372036854775808 | DIFFERS | comparison | byte 2, EntityOracle.rngState((Random)f.get(gen)) [big endian] = 931132098519 (Mode_terrainround2.java:35, field starts 0, 8 bytes, Long); Rust 155465013058623 |
| verify10 | terrain round2 dungeon seed -9223372036854775808 | EXACT | comparison | EXACT MATCH (2408628 bytes) |
| verify10 | terrain round2 lifetime seed -9223372036854775808 | DIFFERS | comparison | byte 6042205, lifetime world[2], chunk (0,0).metadata (12,69,8): Java 8, Rust 0 |
| verify10 | terrain round2 colors seed -9223372036854775808 | DIFFERS | comparison | byte 289, b.getSkyColorByTemp(t) [big endian] = 0 (Mode_terrainround2.java:39, field starts 288, 4 bytes, Integer); Rust 12632319 |
| verify10 | terrain round2 locations seed -9223372036854775808 | DIFFERS | comparison | byte 0, p.chunkXPos [big endian] = -6 (Mode_terrainround2.java:27, field starts 0, 4 bytes, Integer); Rust 0 |
| verify10 | stronghold layout seed 0 | DIFFERS | comparison | byte 3, b.minX [big endian] = -53 (Mode_structures10.java:9, field starts 0, 4 bytes, Integer); Rust -63 |
| verify10 | stronghold map seed 0 | DIFFERS | comparison | byte 11, p.chunkXPos [big endian] = -12 (Mode_strongholds10.java:56, field starts 8, 4 bytes, Integer); Rust -8 |
| verify10 | stronghold blocks seed 0 | DIFFERS | comparison | byte 3, b.minX [big endian] = -21 (Mode_structures10.java:9, field starts 0, 4 bytes, Integer); Rust -31 |
| verify10 | stronghold wet seed 0 | DIFFERS | comparison | byte 3, b.minX [big endian] = -21 (Mode_structures10.java:9, field starts 0, 4 bytes, Integer); Rust -31 |
| verify10 | stronghold pieces seed 0 | DIFFERS | comparison | byte 34411743, f = true (Mode_strongholds10.java:20, field starts 34411743, 1 bytes, Boolean); Rust False |
| verify10 | stronghold grid seed 0 | DIFFERS | comparison | byte 2, EntityOracle.rngState((Random)field(gen,"rand")) [big endian] = 183172866408027 (Mode_strongholds10.java:43, field starts 0, 8 bytes, Long); Rust 167825629107163 |
| verify10 | chunk provider longset seed 0 | EXACT | comparison | EXACT MATCH (2413960 bytes) |
| verify10 | chunk provider scan seed 0 | EXACT | comparison | EXACT MATCH (19325 bytes) |
| verify10 | chunk provider saves seed 0 | EXACT | comparison | EXACT MATCH (82208 bytes) |
| verify10 | alternate chunk cache cache seed 0 | EXACT | comparison | EXACT MATCH (95820 bytes) |
| verify10 | alternate chunk cache null seed 0 | EXACT | comparison | EXACT MATCH (59844 bytes) |
| verify10 | alternate chunk cache failure seed 0 | EXACT | comparison | EXACT MATCH (87903 bytes) |
| verify10 | NBT compound serialization seed 0 | EXACT | comparison | EXACT MATCH (13592 bytes) |
| verify10 | independently populated region NBT seed 0 (pinned entity/item RNGs) | DIFFERS | comparison | chunk (-5, 1).present: Java False, Rust True (rechecked by verify_regions10) |
| verify10 | Java save / Rust load seed 0 | DIFFERS | comparison | block: (174,63,107) java=111 () rust=0 () |
| verify10 | Rust save / Java load seed 0 | EXACT | comparison | EXACT MATCH |
| verify10 | stronghold layout seed 42 | DIFFERS | comparison | byte 3, b.minX [big endian] = -59 (Mode_structures10.java:9, field starts 0, 4 bytes, Integer); Rust -66 |
| verify10 | stronghold map seed 42 | DIFFERS | comparison | byte 11, p.chunkXPos [big endian] = -16 (Mode_strongholds10.java:56, field starts 8, 4 bytes, Integer); Rust -7 |
| verify10 | stronghold blocks seed 42 | DIFFERS | comparison | byte 3, b.minX [big endian] = -27 (Mode_structures10.java:9, field starts 0, 4 bytes, Integer); Rust -34 |
| verify10 | stronghold wet seed 42 | DIFFERS | comparison | byte 3, b.minX [big endian] = -27 (Mode_structures10.java:9, field starts 0, 4 bytes, Integer); Rust -34 |
| verify10 | stronghold pieces seed 42 | DIFFERS | comparison | byte 34411767, f = true (Mode_strongholds10.java:20, field starts 34411767, 1 bytes, Boolean); Rust False |
| verify10 | stronghold grid seed 42 | DIFFERS | comparison | byte 2, EntityOracle.rngState((Random)field(gen,"rand")) [big endian] = 265682019761872 (Mode_strongholds10.java:43, field starts 0, 8 bytes, Long); Rust 81766801668914 |
| verify10 | chunk provider longset seed 42 | EXACT | comparison | EXACT MATCH (2381524 bytes) |
| verify10 | chunk provider scan seed 42 | EXACT | comparison | EXACT MATCH (19325 bytes) |
| verify10 | chunk provider saves seed 42 | EXACT | comparison | EXACT MATCH (82208 bytes) |
| verify10 | alternate chunk cache cache seed 42 | EXACT | comparison | EXACT MATCH (95515 bytes) |
| verify10 | alternate chunk cache null seed 42 | EXACT | comparison | EXACT MATCH (59844 bytes) |
| verify10 | alternate chunk cache failure seed 42 | EXACT | comparison | EXACT MATCH (87803 bytes) |
| verify10 | NBT compound serialization seed 42 | EXACT | comparison | EXACT MATCH (13592 bytes) |
| verify10 | independently populated region NBT seed 42 (pinned entity/item RNGs) | DIFFERS | comparison | chunk (-9, -22).present: Java True, Rust False (rechecked by verify_regions10) |
| verify10 | Java save / Rust load seed 42 | EXACT | comparison | EXACT MATCH |
| verify10 | Rust save / Java load seed 42 | EXACT | comparison | EXACT MATCH |
| verify10 | stronghold layout seed 3257840388504953787 | DIFFERS | comparison | byte 3, b.minX [big endian] = -88 (Mode_structures10.java:9, field starts 0, 4 bytes, Integer); Rust -55 |
| verify10 | stronghold map seed 3257840388504953787 | DIFFERS | comparison | byte 11, p.chunkXPos [big endian] = -64 (Mode_strongholds10.java:56, field starts 8, 4 bytes, Integer); Rust -60 |
| verify10 | stronghold blocks seed 3257840388504953787 | DIFFERS | comparison | byte 3, b.minX [big endian] = -56 (Mode_structures10.java:9, field starts 0, 4 bytes, Integer); Rust -23 |
| verify10 | stronghold wet seed 3257840388504953787 | DIFFERS | comparison | byte 3, b.minX [big endian] = -56 (Mode_structures10.java:9, field starts 0, 4 bytes, Integer); Rust -23 |
| verify10 | stronghold pieces seed 3257840388504953787 | DIFFERS | comparison | byte 34411767, f = false (Mode_strongholds10.java:20, field starts 34411767, 1 bytes, Boolean); Rust True |
| verify10 | stronghold grid seed 3257840388504953787 | DIFFERS | comparison | byte 2, EntityOracle.rngState((Random)field(gen,"rand")) [big endian] = 102777417467672 (Mode_strongholds10.java:43, field starts 0, 8 bytes, Long); Rust 239036345580245 |
| verify10 | chunk provider longset seed 3257840388504953787 | EXACT | comparison | EXACT MATCH (2502888 bytes) |
| verify10 | chunk provider scan seed 3257840388504953787 | EXACT | comparison | EXACT MATCH (19325 bytes) |
| verify10 | chunk provider saves seed 3257840388504953787 | EXACT | comparison | EXACT MATCH (82208 bytes) |
| verify10 | alternate chunk cache cache seed 3257840388504953787 | EXACT | comparison | EXACT MATCH (95876 bytes) |
| verify10 | alternate chunk cache null seed 3257840388504953787 | EXACT | comparison | EXACT MATCH (59844 bytes) |
| verify10 | alternate chunk cache failure seed 3257840388504953787 | EXACT | comparison | EXACT MATCH (88014 bytes) |
| verify10 | NBT compound serialization seed 3257840388504953787 | EXACT | comparison | EXACT MATCH (13592 bytes) |
| verify10 | independently populated region NBT seed 3257840388504953787 (pinned entity/item RNGs) | DIFFERS | comparison | chunk (-24, -35).present: Java False, Rust True (rechecked by verify_regions10) |
| verify10 | Java save / Rust load seed 3257840388504953787 | DIFFERS | comparison | block: (35,63,118) java=111 () rust=0 () |
| verify10 | Rust save / Java load seed 3257840388504953787 | EXACT | comparison | EXACT MATCH |
| verify10 | stronghold layout seed -4172144997902289642 | DIFFERS | comparison | byte 3, b.minX [big endian] = -122 (Mode_structures10.java:9, field starts 0, 4 bytes, Integer); Rust -144 |
| verify10 | stronghold map seed -4172144997902289642 | DIFFERS | comparison | byte 11, p.chunkXPos [big endian] = -19 (Mode_strongholds10.java:56, field starts 8, 4 bytes, Integer); Rust -20 |
| verify10 | stronghold blocks seed -4172144997902289642 | DIFFERS | comparison | byte 3, b.minX [big endian] = -90 (Mode_structures10.java:9, field starts 0, 4 bytes, Integer); Rust -112 |
| verify10 | stronghold wet seed -4172144997902289642 | DIFFERS | comparison | byte 3, b.minX [big endian] = -90 (Mode_structures10.java:9, field starts 0, 4 bytes, Integer); Rust -112 |
| verify10 | stronghold pieces seed -4172144997902289642 | DIFFERS | comparison | byte 34411676, f = true (Mode_strongholds10.java:20, field starts 34411676, 1 bytes, Boolean); Rust False |
| verify10 | stronghold grid seed -4172144997902289642 | DIFFERS | comparison | byte 2, EntityOracle.rngState((Random)field(gen,"rand")) [big endian] = 49191666026482 (Mode_strongholds10.java:43, field starts 0, 8 bytes, Long); Rust 134960248650252 |
| verify10 | chunk provider longset seed -4172144997902289642 | EXACT | comparison | EXACT MATCH (2339180 bytes) |
| verify10 | chunk provider scan seed -4172144997902289642 | EXACT | comparison | EXACT MATCH (19325 bytes) |
| verify10 | chunk provider saves seed -4172144997902289642 | EXACT | comparison | EXACT MATCH (82208 bytes) |
| verify10 | alternate chunk cache cache seed -4172144997902289642 | EXACT | comparison | EXACT MATCH (95625 bytes) |
| verify10 | alternate chunk cache null seed -4172144997902289642 | EXACT | comparison | EXACT MATCH (59844 bytes) |
| verify10 | alternate chunk cache failure seed -4172144997902289642 | EXACT | comparison | EXACT MATCH (87776 bytes) |
| verify10 | NBT compound serialization seed -4172144997902289642 | EXACT | comparison | EXACT MATCH (13592 bytes) |
| verify10 | independently populated region NBT seed -4172144997902289642 (pinned entity/item RNGs) | DIFFERS | comparison | chunk (-20, 6).present: Java True, Rust False (rechecked by verify_regions10) |
| verify10 | Java save / Rust load seed -4172144997902289642 | DIFFERS | comparison | block: (-312,63,119) java=111 () rust=0 () |
| verify10 | Rust save / Java load seed -4172144997902289642 | EXACT | comparison | EXACT MATCH |
| verify10 | stronghold layout seed 9223372036854775807 | DIFFERS | comparison | byte 3, b.minX [big endian] = -143 (Mode_structures10.java:9, field starts 0, 4 bytes, Integer); Rust -35 |
| verify10 | stronghold map seed 9223372036854775807 | DIFFERS | comparison | byte 8, p.chunkXPos [big endian] = -5 (Mode_strongholds10.java:56, field starts 8, 4 bytes, Integer); Rust 2 |
| verify10 | stronghold blocks seed 9223372036854775807 | DIFFERS | comparison | byte 3, b.minX [big endian] = -111 (Mode_structures10.java:9, field starts 0, 4 bytes, Integer); Rust -3 |
| verify10 | stronghold wet seed 9223372036854775807 | DIFFERS | comparison | byte 3, b.minX [big endian] = -111 (Mode_structures10.java:9, field starts 0, 4 bytes, Integer); Rust -3 |
| verify10 | stronghold pieces seed 9223372036854775807 | DIFFERS | comparison | byte 36869644, f = true (Mode_strongholds10.java:20, field starts 36869644, 1 bytes, Boolean); Rust False |
| verify10 | stronghold grid seed 9223372036854775807 | DIFFERS | comparison | byte 2, EntityOracle.rngState((Random)field(gen,"rand")) [big endian] = 40736577239778 (Mode_strongholds10.java:43, field starts 0, 8 bytes, Long); Rust 160410273646710 |
| verify10 | chunk provider longset seed 9223372036854775807 | EXACT | comparison | EXACT MATCH (2463512 bytes) |
| verify10 | chunk provider scan seed 9223372036854775807 | EXACT | comparison | EXACT MATCH (19325 bytes) |
| verify10 | chunk provider saves seed 9223372036854775807 | EXACT | comparison | EXACT MATCH (82208 bytes) |
| verify10 | alternate chunk cache cache seed 9223372036854775807 | EXACT | comparison | EXACT MATCH (95574 bytes) |
| verify10 | alternate chunk cache null seed 9223372036854775807 | EXACT | comparison | EXACT MATCH (59844 bytes) |
| verify10 | alternate chunk cache failure seed 9223372036854775807 | EXACT | comparison | EXACT MATCH (87781 bytes) |
| verify10 | NBT compound serialization seed 9223372036854775807 | EXACT | comparison | EXACT MATCH (13592 bytes) |
| verify10 | independently populated region NBT seed 9223372036854775807 (pinned entity/item RNGs) | DIFFERS | comparison | chunk (-1, 7).present: Java False, Rust True (rechecked by verify_regions10) |
| verify10 | Java save / Rust load seed 9223372036854775807 | EXACT | comparison | EXACT MATCH |
| verify10 | Rust save / Java load seed 9223372036854775807 | EXACT | comparison | EXACT MATCH |
| verify10 | stronghold layout seed -9223372036854775808 | DIFFERS | comparison | byte 3, b.minX [big endian] = -53 (Mode_structures10.java:9, field starts 0, 4 bytes, Integer); Rust -63 |
| verify10 | stronghold map seed -9223372036854775808 | DIFFERS | comparison | byte 8, p.chunkXPos [big endian] = -6 (Mode_strongholds10.java:56, field starts 8, 4 bytes, Integer); Rust 0 |
| verify10 | stronghold blocks seed -9223372036854775808 | DIFFERS | comparison | byte 3, b.minX [big endian] = -21 (Mode_structures10.java:9, field starts 0, 4 bytes, Integer); Rust -31 |
| verify10 | stronghold wet seed -9223372036854775808 | DIFFERS | comparison | byte 3, b.minX [big endian] = -21 (Mode_structures10.java:9, field starts 0, 4 bytes, Integer); Rust -31 |
| verify10 | stronghold pieces seed -9223372036854775808 | DIFFERS | comparison | byte 34411743, f = true (Mode_strongholds10.java:20, field starts 34411743, 1 bytes, Boolean); Rust False |
| verify10 | stronghold grid seed -9223372036854775808 | DIFFERS | comparison | byte 2, EntityOracle.rngState((Random)field(gen,"rand")) [big endian] = 84295601628748 (Mode_strongholds10.java:43, field starts 0, 8 bytes, Long); Rust 224101998723905 |
| verify10 | chunk provider longset seed -9223372036854775808 | EXACT | comparison | EXACT MATCH (2413960 bytes) |
| verify10 | chunk provider scan seed -9223372036854775808 | EXACT | comparison | EXACT MATCH (19325 bytes) |
| verify10 | chunk provider saves seed -9223372036854775808 | EXACT | comparison | EXACT MATCH (82208 bytes) |
| verify10 | alternate chunk cache cache seed -9223372036854775808 | EXACT | comparison | EXACT MATCH (95820 bytes) |
| verify10 | alternate chunk cache null seed -9223372036854775808 | EXACT | comparison | EXACT MATCH (59844 bytes) |
| verify10 | alternate chunk cache failure seed -9223372036854775808 | EXACT | comparison | EXACT MATCH (87903 bytes) |
| verify10 | NBT compound serialization seed -9223372036854775808 | EXACT | comparison | EXACT MATCH (13592 bytes) |
| verify10 | independently populated region NBT seed -9223372036854775808 (pinned entity/item RNGs) | DIFFERS | comparison | chunk (-30, 5).present: Java True, Rust False (rechecked by verify_regions10) |
| verify10 | Java save / Rust load seed -9223372036854775808 | DIFFERS | comparison | block: (-472,63,119) java=111 () rust=0 () |
| verify10 | Rust save / Java load seed -9223372036854775808 | EXACT | comparison | EXACT MATCH |
| verify10 | entities10: rules | DIFFERS | comparison | byte 117, s.unblockable() = true (Mode_entities10.java:48, field starts 117, 1 bytes, Boolean); Rust False (rechecked by verify_entities10) |
| verify10 | entities10: indirect | DIFFERS | comparison | byte 181, a.health = 40 (Mode_entities10.java:30, field starts 181, 4 bytes, Integer); Rust 20 (rechecked by verify_entities10) |
| verify10 | entities10: stare | DIFFERS | comparison | byte 181, a.health = 40 (Mode_entities10.java:30, field starts 181, 4 bytes, Integer); Rust 20 (rechecked by verify_entities10) |
| verify10 | entities10: starepumpkin | DIFFERS | comparison | byte 181, a.health = 40 (Mode_entities10.java:30, field starts 181, 4 bytes, Integer); Rust 20 (rechecked by verify_entities10) |
| verify10 | entities10: population | DIFFERS | comparison | byte 40680, EntityOracle.rngState(random) = 25214908013 (Mode_entities10.java:66, field starts 40680, 8 bytes, Long); Rust 47669638442956 (rechecked by verify_entities10) |
| verify10 | entities10: carry | DIFFERS | comparison | byte 181, a.health = 40 (Mode_entities10.java:30, field starts 181, 4 bytes, Integer); Rust 20 (rechecked by verify_entities10) |
| verify10 | entities10: allies | DIFFERS | comparison | byte 181, a.health = 6 (Mode_entities10.java:30, field starts 181, 4 bytes, Integer); Rust 18 (rechecked by verify_entities10) |
| verify10 | entities10: attract | DIFFERS | comparison | byte 27, e.motionX = 0.09842009842395782 (Mode_entities10.java:24, field starts 24, 8 bytes, Double); Rust 0.08030473440885544 (rechecked by verify_entities10) |
| verify10 | entities10: Boat | EXACT | comparison | EXACT: entities181 Boat (21513 bytes) (rechecked by verify_entities10) |
| verify10 | entities10: Minecart | EXACT | comparison | EXACT: entities181 Minecart (21513 bytes) (rechecked by verify_entities10) |
| verify10 | entities10: Item | EXACT | comparison | EXACT: entities181 Item (21513 bytes) (rechecked by verify_entities10) |
| verify10 | entities10: Lightning | EXACT | comparison | EXACT: entities181 Lightning (4584 bytes) (rechecked by verify_entities10) |
| verify10 | entities10: Snowball | EXACT | comparison | EXACT: entities181 Snowball (7643 bytes) (rechecked by verify_entities10) |
| verify10 | entities10: Egg | EXACT | comparison | EXACT: entities181 Egg (7643 bytes) (rechecked by verify_entities10) |
| verify10 | entities10: Fireball | DIFFERS | comparison | byte 381, Harness10.getInt(e,"fire") = 20 (Mode_entities10.java:26, field starts 381, 4 bytes, Integer); Rust 10 (rechecked by verify_entities10) |
| verify10 | entities10: Enderman | DIFFERS | comparison | byte 181, a.health = 40 (Mode_entities10.java:30, field starts 181, 4 bytes, Integer); Rust 20 (rechecked by verify_entities10) |
| verify10 | entities10: teleport | DIFFERS | comparison | byte 181, a.health = 40 (Mode_entities10.java:30, field starts 181, 4 bytes, Integer); Rust 20 (rechecked by verify_entities10) |
| verify10 | entities10: CaveSpider | DIFFERS | comparison | byte 181, a.health = 12 (Mode_entities10.java:30, field starts 181, 4 bytes, Integer); Rust 20 (rechecked by verify_entities10) |
| verify10 | entities10: Silverfish | DIFFERS | comparison | byte 181, a.health = 8 (Mode_entities10.java:30, field starts 181, 4 bytes, Integer); Rust 20 (rechecked by verify_entities10) |
| verify10 | entities10: XPOrb | DIFFERS | comparison | byte 27, e.motionX = 0.09842009842395782 (Mode_entities10.java:24, field starts 24, 8 bytes, Double); Rust 0.08030473440885544 (rechecked by verify_entities10) |
| verify10 | entities10: Arrow | DIFFERS | comparison | byte 8995, sound.name: Java 'random.bowhit', Rust 'random.drr' (rechecked by verify_entities10) |
| verify10 | entities10: Pig | DIFFERS | comparison | byte 442, e.rotationYaw = 40.568817 (Mode_entities10.java:25, field starts 442, 4 bytes, Float); Rust 35.0 (rechecked by verify_entities10) |
| verify10 | entities10: Cow | DIFFERS | comparison | byte 442, e.rotationYaw = 40.568817 (Mode_entities10.java:25, field starts 442, 4 bytes, Float); Rust 35.0 (rechecked by verify_entities10) |
| verify10 | entities10: Chicken | DIFFERS | comparison | byte 442, e.rotationYaw = 35.0 (Mode_entities10.java:25, field starts 442, 4 bytes, Float); Rust 40.568817138671875 (rechecked by verify_entities10) |
| verify10 | entities10: Sheep | DIFFERS | comparison | byte 181, a.health = 8 (Mode_entities10.java:30, field starts 181, 4 bytes, Integer); Rust 10 (rechecked by verify_entities10) |
| verify10 | entities10: Wolf | DIFFERS | comparison | byte 442, e.rotationYaw = 40.568817 (Mode_entities10.java:25, field starts 442, 4 bytes, Float); Rust 35.0 (rechecked by verify_entities10) |
| verify10 | entities10: Zombie | DIFFERS | comparison | byte 442, e.rotationYaw = 40.568817 (Mode_entities10.java:25, field starts 442, 4 bytes, Float); Rust 35.0 (rechecked by verify_entities10) |
| verify10 | entities10: Skeleton | DIFFERS | comparison | byte 442, e.rotationYaw = 40.568817 (Mode_entities10.java:25, field starts 442, 4 bytes, Float); Rust 35.0 (rechecked by verify_entities10) |
| verify10 | entities10: Spider | DIFFERS | comparison | byte 181, a.health = 16 (Mode_entities10.java:30, field starts 181, 4 bytes, Integer); Rust 20 (rechecked by verify_entities10) |
| verify10 | entities10: Creeper | DIFFERS | comparison | byte 442, e.rotationYaw = 40.568817 (Mode_entities10.java:25, field starts 442, 4 bytes, Float); Rust 35.0 (rechecked by verify_entities10) |
| verify10 | entities10: PigZombie | DIFFERS | comparison | byte 442, e.rotationYaw = 40.568817 (Mode_entities10.java:25, field starts 442, 4 bytes, Float); Rust 35.0 (rechecked by verify_entities10) |
| verify10 | entities10: Ghast | DIFFERS | comparison | byte 314, e.posX = -3.494552817668196 (Mode_entities10.java:24, field starts 314, 8 bytes, Double); Rust -3.5 (rechecked by verify_entities10) |
| verify10 | entities10: Squid | DIFFERS | comparison | byte 531, a.entityAge = 1 (Mode_entities10.java:30, field starts 531, 4 bytes, Integer); Rust 0 (rechecked by verify_entities10) |
| verify10 | entities10: Slime | DIFFERS | comparison | byte 596, EntityOracle.rngState(e.rand) = 201480731191965 (Mode_entities10.java:41, field starts 596, 8 bytes, Long); Rust 54880878584218 (rechecked by verify_entities10) |
| verify10 | entities10: potions | DIFFERS | comparison | byte 396, e.posX = -3.5 (Mode_entities10.java:24, field starts 394, 8 bytes, Double); Rust -3.5011954854708165 (rechecked by verify_entities10) |
| verify10 | entities10: damage | DIFFERS | comparison | byte 324, e.posX = -3.5124340476468205 (Mode_entities10.java:24, field starts 322, 8 bytes, Double); Rust -3.5011954854708165 (rechecked by verify_entities10) |
| verify10 | entities10: poison | DIFFERS | comparison | byte 181, a.health = 19 (Mode_entities10.java:30, field starts 181, 4 bytes, Integer); Rust 18 (rechecked by verify_entities10) |
| verify10 | entities10: kill-Cow | DIFFERS | comparison | byte 612, EntityOracle.rngState(e.rand) = 85306545087601 (Mode_entities10.java:41, field starts 612, 8 bytes, Long); Rust 187788727644900 (rechecked by verify_entities10) |
| verify10 | entities10: kill-Chicken | DIFFERS | comparison | byte 612, EntityOracle.rngState(e.rand) = 219111625387582 (Mode_entities10.java:41, field starts 612, 8 bytes, Long); Rust 187788727644900 (rechecked by verify_entities10) |
| verify10 | entities10: kill-Sheep | DIFFERS | comparison | byte 181, a.health = 8 (Mode_entities10.java:30, field starts 181, 4 bytes, Integer); Rust 10 (rechecked by verify_entities10) |
| verify10 | entities10: kill-Zombie | DIFFERS | comparison | byte 503, a.health = -900 (Mode_entities10.java:30, field starts 503, 4 bytes, Integer); Rust -980 (rechecked by verify_entities10) |
| verify10 | entities10: kill-Enderman | DIFFERS | comparison | byte 181, a.health = 40 (Mode_entities10.java:30, field starts 181, 4 bytes, Integer); Rust 20 (rechecked by verify_entities10) |
| verify10 | entities10: entitygeometry | EXACT | comparison | EXACT MATCH: 10000 geometry cases (rechecked by verify_entities10) |
| verify10 | entities10: models10 | DIFFERS | comparison | case 56: part count: Rust 6, Java 7 (rechecked by verify_entities10) |
| verify10 | entities10: particles10 | EXACT | comparison | particle checks: 184 cases, 594688 vertices, 0 differing fields (rechecked by verify_entities10) |
| verify10 | entities10: xprender10 | DIFFERS | comparison with oracle failure | byte 612, EntityOracle.rngState(e.rand) = 187788727644900 (Mode_entities10.java:41, field starts 612, 8 bytes, Long); Rust 201480731191965; another mode in this section failed: java.lang.UnsupportedOperationException: ModelBase random-part selection was removed in Minecraft 1.0; at net.minecraft.src.Harness10.removedRandomModel(Harness10.java:57) (rechecked by verify_entities10) |
| verify10 | player-gui 1.0 round 6: creative initialization and controller update policy | EXACT | comparison | CONTROLLER MODE EXACT: 10000 original initialization/update/flag cases (6150000 bytes) (rechecked by verify_constantfix10) |
| verify10 | player-gui 1.0 round 6: SP action state, FOV and sprint expiration | DIFFERS | comparison | byte 341470, bytes.toByteArray()[282]: Java 140, Rust 229 (array starts 341188) (rechecked by verify_constantfix10) |
| verify10 | player-gui 1.0 round 6: actual GUI container close callbacks | EXACT | comparison | GUI CONTAINER CLOSE EXACT: 384 original callbacks, six containers, double close/drop semantics (45600 bytes) (rechecked by verify_constantfix10) |
| verify10 | player-gui 1.0 round 6: world-side GUI chest close hooks | DIFFERS | comparison | byte 796, sound.name: Java 'random.chestopen', Rust 'random.door_open' (rechecked by verify_constantfix10) |
| verify10 | player-gui 1.0 round 5: inventory/NBT and player callbacks | DIFFERS | comparison | byte 3550936, Float.floatToIntBits(p.currentXP) = 1316011909 (Mode_playerstate10.java:55, field starts 3550936, 4 bytes, Integer); Rust 1009770805 (rechecked by verify_constantfix10) |
| verify10 | player-gui 1.0 round 5: queued click/use transitions | DIFFERS | oracle failure | Oracle failed before comparison: java.lang.NullPointerException; at net.minecraft.client.Minecraft.clickMiddleMouseButton(Minecraft.java:2201) (rechecked by verify_constantfix10) |
| verify10 | player-gui 1.0 round 5: production create-world callbacks | DIFFERS | comparison | byte 1611, field("field_35364_f").get(g).equals("creative") = false (Mode_worldcallbacks10.java:25, field starts 1611, 1 bytes, Boolean); Rust True (rechecked by verify_constantfix10) |
| verify10 | player-gui 1.0 round 5: inventory interfaces and base slots | EXACT | comparison | INVENTORY INTERFACES EXACT: 2400 original IInventory/Slot operations (295200 bytes) (rechecked by verify_constantfix10) |
| verify10 | player-gui 1.0 round 4: controllers, picking and creative reach | DIFFERS | oracle failure | Oracle failed before comparison: java.lang.NullPointerException; at net.minecraft.client.Minecraft.clickMiddleMouseButton(Minecraft.java:2201) (rechecked by verify_constantfix10) |
| verify10 | player-gui 1.0 round 4: chest container lifecycle | DIFFERS | comparison | byte 796, sound.name: Java 'random.chestopen', Rust 'random.door_open' (rechecked by verify_constantfix10) |
| verify10 | player-gui 1.0 round 4: global key registry | EXACT | comparison | GLOBAL KEY REGISTRY EXACT: 20000 events, 4 settings, 8 standalone bindings, overflow (rechecked by verify_constantfix10) |
| verify10 | player-gui 1.0 round 4: statistics comparators | DIFFERS | comparison | byte 524, block?bc.compare(new StatCrafting(0,"a",ia),new StatCrafting(0,"b",ib)):ic.compare(new StatCrafting(0,"a",ia),new StatCrafting(0,"b",ib)) = 1 (Mode_statsorder10.java:21, field starts 524, 4 bytes, Integer); Rust 40 (rechecked by verify_constantfix10) |
| verify10 | player-gui 1.0 round 4: create-world seeds | EXACT | comparison | CREATE WORLD SEED POLICY EXACT: 65536 Java digits, 10096 seed/mode/features records (clock-random seeds classified; Rust GUI callbacks not replayed) (rechecked by verify_constantfix10) |
| verify10 | playertick10: idle, 500 ticks | DIFFERS | comparison | byte 1470, frame 1.entityRand: Java 206789181479531, Rust 203227649148896 (rechecked by verify_constantfix10) |
| verify10 | playertick10: movement, 240 ticks | DIFFERS | comparison | byte 1470, frame 1.entityRand: Java 206789181479531, Rust 203227649148896 (rechecked by verify_constantfix10) |
| verify10 | playertick10: sprint, 240 ticks | DIFFERS | comparison | byte 1470, frame 1.entityRand: Java 206789181479531, Rust 203227649148896 (rechecked by verify_constantfix10) |
| verify10 | playertick10: flight, 500 ticks | DIFFERS | comparison | byte 1470, frame 1.entityRand: Java 206789181479531, Rust 203227649148896 (rechecked by verify_constantfix10) |
| verify10 | playertick10: creativecontroller, 500 ticks | DIFFERS | comparison | byte 1470, frame 1.entityRand: Java 206789181479531, Rust 203227649148896 (rechecked by verify_constantfix10) |
| verify10 | playertick10: starve, 500 ticks | DIFFERS | comparison | byte 1470, frame 1.entityRand: Java 206789181479531, Rust 203227649148896 (rechecked by verify_constantfix10) |
| verify10 | playertick10: regen, 500 ticks | DIFFERS | comparison | byte 1470, frame 1.entityRand: Java 206789181479531, Rust 203227649148896 (rechecked by verify_constantfix10) |
| verify10 | playertick10: eat260, 500 ticks | DIFFERS | comparison | byte 1470, frame 1.entityRand: Java 206789181479531, Rust 203227649148896 (rechecked by verify_constantfix10) |
| verify10 | playertick10: eat282, 500 ticks | DIFFERS | comparison | byte 1470, frame 1.entityRand: Java 206789181479531, Rust 203227649148896 (rechecked by verify_constantfix10) |
| verify10 | playertick10: eat322, 500 ticks | DIFFERS | comparison | byte 1470, frame 1.entityRand: Java 206789181479531, Rust 203227649148896 (rechecked by verify_constantfix10) |
| verify10 | playertick10: eat365, 500 ticks | DIFFERS | comparison | byte 1470, frame 1.entityRand: Java 206789181479531, Rust 203227649148896 (rechecked by verify_constantfix10) |
| verify10 | playertick10: eat367, 500 ticks | DIFFERS | comparison | byte 1470, frame 1.entityRand: Java 206789181479531, Rust 203227649148896 (rechecked by verify_constantfix10) |
| verify10 | playertick10: cancel, 500 ticks | DIFFERS | comparison | byte 1470, frame 1.entityRand: Java 206789181479531, Rust 203227649148896 (rechecked by verify_constantfix10) |
| verify10 | playertick10: identity, 500 ticks | DIFFERS | comparison | byte 1470, frame 1.entityRand: Java 206789181479531, Rust 203227649148896 (rechecked by verify_constantfix10) |
| verify10 | playertick10: bow, 500 ticks | DIFFERS | comparison | byte 1470, frame 1.entityRand: Java 206789181479531, Rust 203227649148896 (rechecked by verify_constantfix10) |
| verify10 | playertick10: xp, 500 ticks | DIFFERS | comparison | byte 1470, frame 1.entityRand: Java 206789181479531, Rust 203227649148896 (rechecked by verify_constantfix10) |
| verify10 | playertick10: damage, 500 ticks | DIFFERS | comparison | byte 1184, frame 1.payloadLength: Java 1249, Rust 1244 (rechecked by verify_constantfix10) |
| verify10 | playertick10: creativedamage, 500 ticks | DIFFERS | comparison | byte 1470, frame 1.entityRand: Java 206789181479531, Rust 203227649148896 (rechecked by verify_constantfix10) |
| verify10 | playertick10: creativevoid, 500 ticks | DIFFERS | comparison | byte 1184, frame 1.payloadLength: Java 1249, Rust 1244 (rechecked by verify_constantfix10) |
| verify10 | playertick10: speed, 500 ticks | DIFFERS | comparison | byte 1184, frame 1.payloadLength: Java 1261, Rust 1200 (rechecked by verify_constantfix10) |
| verify10 | playertick10: haste, 500 ticks | DIFFERS | comparison | byte 1184, frame 1.payloadLength: Java 1261, Rust 1200 (rechecked by verify_constantfix10) |
| verify10 | playertick10: fatigue, 500 ticks | DIFFERS | comparison | byte 1184, frame 1.payloadLength: Java 1261, Rust 1200 (rechecked by verify_constantfix10) |
| verify10 | playertick10: nausea, 500 ticks | DIFFERS | comparison | byte 1184, frame 1.payloadLength: Java 1261, Rust 1200 (rechecked by verify_constantfix10) |
| verify10 | statsync10: receive, send and file writer | DIFFERS | comparison | snapshot 0.achievement[17].present: Java True, Rust False (registry length Java 27, Rust 17) (rechecked by verify_constantfix10) |
| verify10 | player-gui 1.0: settings before initial terrain and save | EXACT | comparison | test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 1.01s (rechecked by verify_constantfix10) |
| verify10 | player-gui 1.0: melee and potion mining | DIFFERS | comparison | byte 62, target.motionZ = 0.5 (Mode_playercombat10.java:29, field starts 56, 8 bytes, Double); Rust 1.0 (rechecked by verify_constantfix10) |
| verify10 | player-gui 1.0: scaling, buttons and font measurements | EXACT | comparison | GUI UTILITIES EXACT: 10000 scales, 10000 buttons, 376 font cases (rechecked by verify_constantfix10) |
| verify10 | player-gui 1.0: PlayerList hash table | EXACT | comparison | PLAYER LIST EXACT: 3000 table operations and 10000 entry equality/hash/string cases (rechecked by verify_constantfix10) |
| verify10 | player-gui 1.0: playercore | DIFFERS | comparison | byte 2378000, Float.floatToIntBits(p.currentXP) [big endian] = 1060559712 (Mode_playercore.java:39, field starts 2378000, 4 bytes, Integer); Rust 38 (rechecked by verify_constantfix10) |
| verify10 | player-gui 1.0: creative | DIFFERS | comparison | byte 2, c.itemList.size() [big endian] = 265 (Mode_creative.java:13, field starts 0, 4 bytes, Integer); Rust 234 (rechecked by verify_constantfix10) |
| verify10 | player-gui 1.0: options and keybindings | DIFFERS | comparison | text line 15, 15 field[0]: Java '15', Rust 'FOV:' (rechecked by verify_constantfix10) |
| verify10 | player-gui 1.0: original recipe and smelting catalog | DIFFERS | comparison | byte 3, recipes.size() [big endian] = 174 (ContainerOracle.java:138, field starts 0, 4 bytes, Integer); Rust 161 (rechecked by verify_constantfix10) |
| verify10 | gui10: b10_hud_full | DIFFERS | comparison | pixel (4,4).RGB: Java (25, 25, 25), Rust (255, 255, 255) |
| verify10 | gui10: b10_hud_low_hunger_xp | DIFFERS | comparison | pixel (4,4).RGB: Java (26, 26, 26), Rust (255, 255, 255) |
| verify10 | gui10: b10_hud_starving | DIFFERS | comparison | pixel (4,4).RGB: Java (26, 26, 26), Rust (255, 255, 255) |
| verify10 | gui10: b10_hud_creative | DIFFERS | comparison | pixel (4,4).RGB: Java (26, 26, 26), Rust (255, 255, 255) |
| verify10 | gui10: b10_create_survival | EXACT | comparison | b10_create_survival: EXACT |
| verify10 | gui10: b10_create_creative | DIFFERS | comparison | pixel (322,212).RGB: Java (224, 224, 224), Rust (113, 113, 113) |
| verify10 | gui10: b10_create_more | EXACT | comparison | b10_create_more: EXACT |
| verify10 | gui10: b10_create_features_off | EXACT | comparison | b10_create_features_off: EXACT |
| verify10 | gui10: b10_creative_first | DIFFERS | comparison | pixel (268,76).RGB: Java (79, 79, 79), Rust (93, 93, 93) |
| verify10 | gui10: b10_creative_scroll | DIFFERS | comparison | pixel (304,76).RGB: Java (10, 10, 15), Rust (12, 12, 18) |
| verify10 | gui10: b10_creative_cursor | DIFFERS | comparison | pixel (304,76).RGB: Java (10, 10, 15), Rust (12, 12, 18) |
| verify10 | gui10: b10_options | EXACT | comparison | b10_options: EXACT |
| verify10 | gui10: b10_options_fov | EXACT | comparison | b10_options_fov: EXACT |
| verify10 | gui10: b10_video_gamma | DIFFERS | comparison | pixel (436,272).RGB: Java (0, 0, 0), Rust (40, 40, 40) |
| verify10 | gui10: b10_controls | EXACT | comparison | b10_controls: EXACT |
| verify10 | gui10: b10_controls_waiting | EXACT | comparison | b10_controls_waiting: EXACT |
| verify10 | gui10: b10_controls_duplicate | EXACT | comparison | b10_controls_duplicate: EXACT |
| verify10 | gui10: b10_inventory | DIFFERS | comparison | pixel (355,113).RGB: Java (40, 27, 12), Rust (38, 26, 10) |
| verify10 | gui10: b10_chest | EXACT | comparison | b10_chest: EXACT |
| verify10 | gui10: b10_largechest | EXACT | comparison | b10_largechest: EXACT |
| verify10 | gui10: b10_crafting | EXACT | comparison | b10_crafting: EXACT |
| verify10 | gui10: b10_furnace | EXACT | comparison | b10_furnace: EXACT |
| verify10 | gui10: b10_dispenser | EXACT | comparison | b10_dispenser: EXACT |
| verify10 | gui10: b10_hud_potions | DIFFERS | comparison | pixel (4,4).RGB: Java (25, 25, 25), Rust (255, 255, 255) |
| verify10 | gui10: b10_hud_regen_shift | DIFFERS | comparison | pixel (4,4).RGB: Java (26, 26, 26), Rust (255, 255, 255) |
| verify10 | gui10: b10_hud_air_full | DIFFERS | comparison | pixel (4,4).RGB: Java (26, 26, 26), Rust (255, 255, 255) |
| verify10 | gui10: b10_hud_air_half | DIFFERS | comparison | pixel (4,4).RGB: Java (26, 26, 26), Rust (255, 255, 255) |
| verify10 | gui10: b10_hud_air_popping | DIFFERS | comparison | pixel (4,4).RGB: Java (26, 26, 26), Rust (255, 255, 255) |
| verify10 | gui10: b10_hud_air_empty | DIFFERS | comparison | pixel (4,4).RGB: Java (26, 26, 26), Rust (255, 255, 255) |
| verify10 | gui10: b10_hud_air_creative | DIFFERS | comparison | pixel (4,4).RGB: Java (26, 26, 26), Rust (255, 255, 255) |
| verify10 | gui10: b10_toast_start | EXACT | comparison | b10_toast_start: EXACT |
| verify10 | gui10: b10_toast_enter | EXACT | comparison | b10_toast_enter: EXACT |
| verify10 | gui10: b10_toast_visible | EXACT | comparison | b10_toast_visible: EXACT |
| verify10 | gui10: b10_toast_middle | EXACT | comparison | b10_toast_middle: EXACT |
| verify10 | gui10: b10_toast_exit | EXACT | comparison | b10_toast_exit: EXACT |
| verify10 | gui10: b10_toast_end | EXACT | comparison | b10_toast_end: EXACT |
| verify10 | gui10: b10_toast_expired | EXACT | comparison | b10_toast_expired: EXACT |
| verify10 | gui10: b10_toast_no_revival | EXACT | comparison | b10_toast_no_revival: EXACT |
| verify10 | gui10: b10_toast_zero_clock | EXACT | comparison | b10_toast_zero_clock: EXACT |
| verify10 | gui10: b10_toast_snipe | EXACT | comparison | b10_toast_snipe: EXACT |
| verify10 | gui10: b10_toast_hint | EXACT | comparison | b10_toast_hint: EXACT |
| verify10 | gui10: b10_toast_hint_late | EXACT | comparison | b10_toast_hint_late: EXACT |
| verify10 | gui10: b10_toast_snipe_hint | EXACT | comparison | b10_toast_snipe_hint: EXACT |
| verify10 | gui10: b10_achievement_inventory_close | DIFFERS | comparison | pixel (4,4).RGB: Java (25, 25, 25), Rust (255, 255, 255) |
| verify10 | gui10: b10_create_signed_seed | EXACT | comparison | b10_create_signed_seed: EXACT |
| verify10 | gui10: b10_create_signed_seed_return | DIFFERS | comparison | pixel (322,212).RGB: Java (224, 224, 224), Rust (113, 113, 113) |
| verify10 | gui10: b10_creative_inventory_close | DIFFERS | comparison | pixel (4,4).RGB: Java (25, 25, 25), Rust (255, 255, 255) |
| verify10 | gui10: b10_create_empty_name | EXACT | comparison | b10_create_empty_name: EXACT |
| verify10 | gui10: b10_create_spaces_name | EXACT | comparison | b10_create_spaces_name: EXACT |
| verify10 | gui10: b10_create_folder_collision | EXACT | comparison | b10_create_folder_collision: EXACT |
| verify10 | gui10: b10_create_tab_seed | EXACT | comparison | b10_create_tab_seed: EXACT |
| verify10 | gui10: b10_rename_spaces_disabled | EXACT | comparison | b10_rename_spaces_disabled: EXACT |
| verify10 | gui10: b10_delete_confirmation | DIFFERS | comparison | pixel (452,96).RGB: Java (128, 128, 128), Rust (15, 11, 7) |
| verify10 | gui10: b10_delete_cancel | DIFFERS | comparison | pixel (452,96).RGB: Java (128, 128, 128), Rust (15, 11, 7) |
| verify10 | gui10: b10_hud5_portal_full | DIFFERS | comparison | pixel (4,4).RGB: Java (0, 42, 255), Rust (255, 255, 255) |
| verify10 | gui10: b10_hud5_portal_nausea | DIFFERS | comparison | pixel (4,4).RGB: Java (25, 25, 25), Rust (255, 255, 255) |
| verify10 | gui10: b10_hud5_portal_partial | DIFFERS | comparison | pixel (4,4).RGB: Java (19, 29, 83), Rust (255, 255, 255) |
| verify10 | gui10: b10_hud5_pumpkin_creative | DIFFERS | comparison | pixel (4,4).RGB: Java (0, 0, 0), Rust (255, 255, 255) |
| verify10 | gui10: b10_hud5_sleep_end | DIFFERS | comparison | pixel (4,4).RGB: Java (25, 25, 25), Rust (255, 255, 255) |
| verify10 | gui10: b10_hud5_sleep_enter | DIFFERS | comparison | pixel (4,4).RGB: Java (25, 25, 25), Rust (255, 255, 255) |
| verify10 | gui10: b10_hud5_sleep_exit | DIFFERS | comparison | pixel (4,4).RGB: Java (21, 21, 28), Rust (255, 255, 255) |
| verify10 | gui10: b10_hud5_sleep_full | DIFFERS | comparison | pixel (4,4).RGB: Java (17, 17, 31), Rust (255, 255, 255) |
| verify10 | gui10: b10_hud5_xp_cap_overflow | DIFFERS | comparison | pixel (4,4).RGB: Java (25, 25, 25), Rust (255, 255, 255) |
| verify10 | gui10: b10_hud5_xp_overflow | DIFFERS | comparison | pixel (4,4).RGB: Java (25, 25, 25), Rust (255, 255, 255) |
| verify10 | gui10: inventory_empty | DIFFERS | comparison | pixel (355,113).RGB: Java (40, 27, 12), Rust (38, 26, 10) |
| verify10 | gui10: inventory_items | DIFFERS | comparison | pixel (353,114).RGB: Java (41, 28, 12), Rust (42, 28, 13) |
| verify10 | gui10: inventory_turn | DIFFERS | comparison | pixel (347,114).RGB: Java (36, 24, 8), Rust (47, 31, 15) |
| verify10 | gui10: inventory_cursor | DIFFERS | comparison | pixel (345,114).RGB: Java (42, 29, 13), Rust (36, 24, 8) |
| verify10 | gui10: crafting_items | DIFFERS | comparison | pixel (312,116).RGB: Java (119, 97, 62), Rust (139, 113, 73) |
| verify10 | gui10: crafting_tooltip | DIFFERS | comparison | pixel (522,114).RGB: Java (27, 12, 27), Rust (198, 198, 198) |
| verify10 | gui10: chest_items | DIFFERS | comparison | pixel (268,116).RGB: Java (34, 111, 27), Rust (39, 129, 32) |
| verify10 | gui10: chest_cursor | DIFFERS | comparison | pixel (268,116).RGB: Java (34, 111, 27), Rust (39, 129, 32) |
| verify10 | gui10: largechest_items | DIFFERS | comparison | pixel (556,242).RGB: Java (134, 71, 88), Rust (156, 83, 103) |
| verify10 | gui10: furnace_mid | DIFFERS | comparison | pixel (364,116).RGB: Java (81, 81, 81), Rust (94, 94, 94) |
| verify10 | gui10: furnace_idle | DIFFERS | comparison | pixel (364,116).RGB: Java (81, 81, 81), Rust (94, 94, 94) |
| verify10 | gui10: furnace_missing_fuel_duration | DIFFERS | comparison | pixel (364,116).RGB: Java (81, 81, 81), Rust (94, 94, 94) |
| verify10 | gui10: dispenser_items | DIFFERS | comparison | pixel (394,114).RGB: Java (24, 9, 24), Rust (150, 150, 150) |
| verify10 | gui10: inventory_diamond_pickaxe | DIFFERS | comparison | pixel (352,107).RGB: Java (45, 197, 180), Rust (44, 194, 177) |
| verify10 | gui10: inventory_equipped_turn | DIFFERS | comparison | pixel (354,108).RGB: Java (48, 206, 190), Rust (45, 197, 180) |
| verify10 | gui10: inventory_pumpkin_block | DIFFERS | comparison | pixel (381,139).RGB: Java (49, 211, 193), Rust (44, 194, 176) |
| verify10 | gui10: inventory_fishing_rod | DIFFERS | comparison | pixel (381,139).RGB: Java (49, 211, 193), Rust (44, 194, 176) |
| verify10 | gui10: inventory_bow | DIFFERS | comparison | pixel (381,139).RGB: Java (49, 211, 193), Rust (44, 194, 176) |
| verify10 | gui10: inventory_dye | DIFFERS | comparison | pixel (381,139).RGB: Java (49, 211, 193), Rust (44, 194, 176) |
| verify10 | gui10: chest_right_pickup | EXACT | comparison | chest_right_pickup: EXACT |
| verify10 | gui10: chest_right_place | EXACT | comparison | chest_right_place: EXACT |
| verify10 | gui10: chest_shift_transfer | EXACT | comparison | chest_shift_transfer: EXACT |
| verify10 | gui10: crafting_click_result | DIFFERS | comparison | pixel (312,116).RGB: Java (119, 97, 62), Rust (139, 113, 73) |
| verify10 | gui10: inventory_sneak_swing | DIFFERS | comparison | pixel (356,116).RGB: Java (30, 21, 11), Rust (34, 24, 11) |
| verify10 | gui10: inventory_riding | DIFFERS | comparison | pixel (353,114).RGB: Java (41, 28, 12), Rust (42, 28, 13) |
| verify10 | gui10: inventory_riding_armor | DIFFERS | comparison | pixel (354,108).RGB: Java (48, 206, 190), Rust (45, 197, 180) |
| verify10 | gui10: inventory_sneak_armor | DIFFERS | comparison | pixel (354,111).RGB: Java (48, 206, 190), Rust (45, 197, 180) |
| verify10 | gui10: inventory_sneak_armor_next_frame | DIFFERS | comparison | pixel (354,111).RGB: Java (48, 206, 190), Rust (45, 197, 180) |
| verify10 | gui10: inventory_hurt_armor | DIFFERS | comparison | pixel (357,108).RGB: Java (104, 124, 114), Rust (125, 124, 114) |
| verify10 | gui10: inventory_hurt | DIFFERS | comparison | pixel (357,110).RGB: Java (70, 13, 7), Rust (86, 13, 7) |
| verify10 | gui10: inventory_cloth | DIFFERS | comparison | pixel (354,108).RGB: Java (135, 84, 54), Rust (129, 79, 51) |
| verify10 | gui10: inventory_chain | DIFFERS | comparison | pixel (357,107).RGB: Java (0, 0, 0), Rust (160, 160, 160) |
| verify10 | gui10: inventory_iron | DIFFERS | comparison | pixel (354,108).RGB: Java (193, 193, 193), Rust (188, 188, 188) |
| verify10 | gui10: inventory_gold | DIFFERS | comparison | pixel (354,108).RGB: Java (231, 214, 30), Rust (228, 206, 28) |
| verify10 | gui10: inventory_signed_counts | DIFFERS | comparison | pixel (355,113).RGB: Java (40, 27, 12), Rust (38, 26, 10) |
| verify10 | gui10: b10_lightmap0_inventory | DIFFERS | comparison | pixel (355,113).RGB: Java (40, 27, 12), Rust (38, 26, 10) |
| verify10 | gui10: b10_lightmap0_items | DIFFERS | comparison | pixel (353,114).RGB: Java (41, 28, 12), Rust (42, 28, 13) |
| verify10 | gui10: b10_lightmap0_hurt | DIFFERS | comparison | pixel (357,110).RGB: Java (71, 13, 7), Rust (86, 13, 7) |
| verify10 | gui10: b10_lightmap0_chest | DIFFERS | comparison | pixel (268,116).RGB: Java (95, 26, 23), Rust (111, 30, 27) |
| verify10 | gui10: b10_lightmap1_inventory | DIFFERS | comparison | pixel (355,113).RGB: Java (40, 27, 12), Rust (38, 26, 10) |
| verify10 | gui10: b10_lightmap1_items | DIFFERS | comparison | pixel (353,114).RGB: Java (41, 28, 12), Rust (42, 28, 13) |
| verify10 | gui10: b10_lightmap1_hurt | DIFFERS | comparison | pixel (357,110).RGB: Java (71, 13, 7), Rust (86, 13, 7) |
| verify10 | gui10: b10_lightmap1_chest | DIFFERS | comparison | pixel (268,116).RGB: Java (95, 26, 23), Rust (111, 30, 27) |
| verify10 | gui10: b10_lightmap2_inventory | DIFFERS | comparison | pixel (355,113).RGB: Java (40, 27, 12), Rust (38, 26, 10) |
| verify10 | gui10: b10_lightmap2_items | DIFFERS | comparison | pixel (353,114).RGB: Java (41, 28, 12), Rust (42, 28, 13) |
| verify10 | gui10: b10_lightmap2_hurt | DIFFERS | comparison | pixel (357,110).RGB: Java (71, 13, 7), Rust (86, 13, 7) |
| verify10 | gui10: b10_lightmap2_chest | DIFFERS | comparison | pixel (268,116).RGB: Java (95, 26, 23), Rust (111, 30, 27) |
| verify10 | gui10: b10_lightmap3_inventory | DIFFERS | comparison | pixel (355,113).RGB: Java (40, 27, 12), Rust (38, 26, 10) |
| verify10 | gui10: b10_lightmap3_items | DIFFERS | comparison | pixel (353,114).RGB: Java (41, 28, 12), Rust (42, 28, 13) |
| verify10 | gui10: b10_lightmap3_hurt | DIFFERS | comparison | pixel (357,110).RGB: Java (71, 13, 7), Rust (86, 13, 7) |
| verify10 | gui10: b10_lightmap3_chest | DIFFERS | comparison | pixel (268,116).RGB: Java (95, 26, 23), Rust (111, 30, 27) |
| verify10 | entities3: slimebranches | DIFFERS | comparison | byte 1236, EntityOracle.rngState(e.rand) = 7749588271903 (Mode_entities10.java:41, field starts 1236, 8 bytes, Long); Rust 206789181479531 |
| verify10 | entities3: skeletonbranches | EXACT | comparison | EXACT: entities181 skeletonbranches (17550 bytes) |
| verify10 | entities3: wolfbranches | EXACT | comparison | EXACT: entities181 wolfbranches (641922 bytes) |
| verify10 | entities3: vehiclebranches | EXACT | comparison | EXACT: entities181 vehiclebranches (52576 bytes) |
| verify10 | entities3: fireballbranches | EXACT | comparison | EXACT: entities181 fireballbranches (19040 bytes) |
| verify10 | entities3: spawnorder | EXACT | comparison | EXACT: entities181 spawnorder (11283 bytes) |
| verify10 | entities3: spawnbranches | DIFFERS | comparison | byte 0, SpawnerAnimals.performSpawning(w,scenario==0,scenario!=0) = 0 (Mode_entitybranches10.java:79, field starts 0, 4 bytes, Integer); Rust 6 |
| verify10 | entities3: factorybranches | DIFFERS | comparison with oracle failure | byte 873, Harness10.removedOrbPhase(a) = 0.0 (Mode_entities10.java:39, field starts 873, 4 bytes, Float); Rust 4.624325752258301; another mode in this section failed: java.lang.UnsupportedOperationException: ModelBase random-part selection was removed in Minecraft 1.0; at net.minecraft.src.Harness10.removedRandomModel(Harness10.java:57) |
| verify10 | entities4: creeperdeath4 | EXACT | comparison | EXACT: entities181 creeperdeath4 (39955 bytes) (rechecked by verify_constantfix10) |
| verify10 | entities4: sun4 | DIFFERS | comparison | byte 290, EntityOracle.rngState(e.rand) = 234029835660143 (Mode_entities10.java:41, field starts 290, 8 bytes, Long); Rust 205749139540596 (rechecked by verify_constantfix10) |
| verify10 | entities4: network4 | DIFFERS | comparison | byte 332896, EntityOracle.rngState(e.rand) = 206789181479531 (Mode_entities10.java:41, field starts 332896, 8 bytes, Long); Rust 203227649148896 (rechecked by verify_constantfix10) |
| verify10 | entities4: lightning4 | DIFFERS | comparison | byte 12168, Harness10.getInt(e,"fire") = 160 (Mode_entities10.java:26, field starts 12168, 4 bytes, Integer); Rust 300 (rechecked by verify_constantfix10) |
| verify10 | entities4: retaliation4 | DIFFERS | comparison | byte 92063, EntityOracle.rngState(e.rand) = 201480731191965 (Mode_entities10.java:41, field starts 92063, 8 bytes, Long); Rust 54880878584218 (rechecked by verify_constantfix10) |
| verify10 | entities4: impact4 | DIFFERS | comparison | byte 412, Harness10.getInt(e,"fire") = 20 (Mode_entities10.java:26, field starts 412, 4 bytes, Integer); Rust 10 (rechecked by verify_constantfix10) |
| verify10 | entities4: flying4 | EXACT | comparison | EXACT: entities181 flying4 (206712 bytes) (rechecked by verify_constantfix10) |
| verify10 | entities4: explosionblocks4 | DIFFERS | comparison | byte 107737, w.getBlockId(x,y,z) = 51 (Mode_entityround4_10.java:88, field starts 107737, 4 bytes, Integer); Rust 0 (rechecked by verify_constantfix10) |
| verify10 | entities4: creeper4 | EXACT | comparison | EXACT: entities181 creeper4 (3759104 bytes) (rechecked by verify_constantfix10) |
| verify10 | entities4: generaliving4_181 | DIFFERS | comparison | new living 1 draw count: Rust 42, Java 48 (rechecked by verify_constantfix10) |
| verify10 | entities4: playerliving4_181 | DIFFERS | comparison | player 2 draw count: Rust 108, Java 126 (rechecked by verify_constantfix10) |
| verify10 | entities4: effectlifecycle4_181 | EXACT | comparison | EXACT: EffectRenderer lifecycle, 21 snapshots, 48,036 insertions across four layers (rechecked by verify_constantfix10) |
| verify10 | entities4: livingrenderers | DIFFERS | comparison | living case 0 draw 0.vertex[0][3]: java 3f400000, rust 3f3f999a (rechecked by verify_constantfix10) |
| verify10 | entities4: rendereffects | EXACT | comparison | EFFECTS EXACT (rechecked by verify_constantfix10) |
| verify10 | entities4: particleeffects | DIFFERS | comparison | block 18 metadata 0.particleTint: Java 0x71a74d, Rust 0x45b421 (rechecked by verify_constantfix10) |
| verify10 | entities4: paintings | EXACT | comparison | EXACT MATCH: paintings selection, walls, overlaps, support ticks, damage, NBT (rechecked by verify_constantfix10) |
| verify10 | entities4: fishing | DIFFERS | comparison | byte 10979121, EntityOracle.l(o,EntityOracle.rngState(f.rand));EntityOracle.l(o,EntityOracle.rngState(w.rand));EntityOracle.l(o,EntityOracle.rngState(EntityOracle.mathRandom())); = 211094699171000 (DecorationOracle.java:75, field starts 10979121, 8 bytes, Long); Rust 25214904129 (rechecked by verify_constantfix10) |
| verify10 | entities5: recorditem5 | EXACT | comparison | EXACT: entities181 recorditem5 (435672 bytes) (rechecked by verify_constantfix10) |
| verify10 | entities5: paintingitem5 | EXACT | comparison | EXACT: entities181 paintingitem5 (597564 bytes) (rechecked by verify_constantfix10) |
| verify10 | entities5: vehicleriders5 | DIFFERS | comparison | byte 181, EntityOracle.rngState(e.rand) = 63990555385005 (Mode_entities10.java:41, field starts 181, 8 bytes, Long); Rust 179978824174513 (rechecked by verify_constantfix10) |
| verify10 | entities5: dyesheep5 | DIFFERS | comparison | byte 181, a.health = 8 (Mode_entities10.java:30, field starts 181, 4 bytes, Integer); Rust 10 (rechecked by verify_constantfix10) |
| verify10 | entities5: potionexpiry5 | DIFFERS | comparison | byte 278, ids.size() = 1 (Mode_entities10.java:33, field starts 278, 4 bytes, Integer); Rust 0 (rechecked by verify_constantfix10) |
| verify10 | entities5: watchers5 | DIFFERS | comparison | byte 0, bytes.size() = 6 (Mode_entityround5_10.java:73, field starts 0, 4 bytes, Integer); Rust 3 (rechecked by verify_constantfix10) |
| verify10 | entities5: wolfremote5 | DIFFERS | comparison | byte 362, bytes.size() = 38 (Mode_entityround5_10.java:73, field starts 362, 4 bytes, Integer); Rust 25 (rechecked by verify_constantfix10) |
| verify10 | entities5: health5 | DIFFERS | comparison | byte 156528, sound.volume: Java 0.800000011920929, Rust 0.6000000238418579 (rechecked by verify_constantfix10) |
| verify10 | entities5: wolfattack5 | EXACT | comparison | EXACT: entities181 wolfattack5 (1025856 bytes) (rechecked by verify_constantfix10) |
| verify10 | entities5: wolffollow5 | EXACT | comparison | EXACT: entities181 wolffollow5 (14896 bytes) (rechecked by verify_constantfix10) |
| verify10 | entities5: vehiclecollision5 | DIFFERS | comparison | byte 400542, e.motionX = 0.0 (Mode_entities10.java:24, field starts 400542, 8 bytes, Double); Rust -0.09785534023263556 (rechecked by verify_constantfix10) |
| verify10 | entities5: vehicleinteract5 | DIFFERS | comparison | byte 177628, Harness10.getInt(m,"fuel") = -2147480149 (Mode_entityround5_10.java:61, field starts 177628, 4 bytes, Integer); Rust -2147482549 (rechecked by verify_constantfix10) |
| verify10 | entities5: skeletondeath5 | EXACT | comparison | EXACT: entities181 skeletondeath5 (18900 bytes) (rechecked by verify_constantfix10) |
| verify10 | entities5: bipedequipment5_181 | DIFFERS | comparison | new living 0 draw 0.vertex[0][3]: java 3ec00000, rust 3ebf3333 (rechecked by verify_constantfix10) |
| verify10 | entities5: particlefactory5_181 | DIFFERS | comparison | particle factory DIFF byte 3804: Java 25976, Rust 24296 (rechecked by verify_constantfix10) |
| verify10 | entities6: living6 | DIFFERS | comparison | byte 290, EntityOracle.rngState(e.rand) = 206789181479531 (Mode_entities10.java:41, field starts 290, 8 bytes, Long); Rust 203227649148896 (rechecked by verify_constantfix10) |
| verify10 | entities6: livingnbt6 | DIFFERS | comparison | byte 564, list.tagCount() = 8 (Mode_entityround6_10.java:9, field starts 564, 4 bytes, Integer); Rust 7 (rechecked by verify_constantfix10) |
| verify10 | entities6: livingdamage6 | DIFFERS | comparison | byte 291, EntityOracle.rngState(e.rand) = 212433917123366 (Mode_entities10.java:41, field starts 291, 8 bytes, Long); Rust 145493833091980 (rechecked by verify_constantfix10) |
| verify10 | entities6: sleep6 | DIFFERS | oracle failure | Oracle failed before comparison: java.lang.UnsupportedOperationException: performSleepSpawning was removed in Minecraft 1.0; at net.minecraft.src.Harness10.removedSleepSpawning(Harness10.java:50) (rechecked by verify_constantfix10) |
| verify10 | entities6: dyeproperties6 | DIFFERS | comparison | byte 3, Item.dyePowder.getIconFromDamage(damage) = 78 (Mode_itemdye6_10.java:11, field starts 0, 4 bytes, Integer); Rust -268435378 (rechecked by verify_constantfix10) |
| verify10 | entities6: dyeblocks6 | EXACT | comparison | EXACT: entities181 dyeblocks6 (28550880 bytes) (rechecked by verify_constantfix10) |
| verify10 | entities6: doorhelper6 | DIFFERS | comparison | byte 2676, w.getBlockMetadata(x,yy,z) = 0 (Mode_itemdoor6_10.java:6, field starts 2676, 4 bytes, Integer); Rust 4 (rechecked by verify_constantfix10) |
| verify10 | entities6: dooritem6 | DIFFERS | comparison | byte 459511, w.getBlockMetadata(x,yy,z) = 0 (Mode_itemdoor6_10.java:6, field starts 459511, 4 bytes, Integer); Rust 4 (rechecked by verify_constantfix10) |
| verify10 | entities6: blockbasic6 (limited block callbacks) | EXACT | comparison | EXACT: entities181 blockitem6 (384360 bytes) (rechecked by verify_constantfix10) |
| verify10 | entities6: spawninputs6 (real JVM identity hashes) | DIFFERS | checker failure | Checker failed before completing comparison: missing recorded JVM identity; thread 'main' (29672) panicked at crates\worldgen10\src\entity\round6_verify181.rs:447:47: (rechecked by verify_constantfix10) |
| verify10 | entities6: playerliving6_10 | DIFFERS | comparison | player 0 draw count: Rust 108, Java 126 (rechecked by verify_constantfix10) |
| verify10 | entities6: particlefactory6_10 | EXACT | comparison | EXACT: named particle factory, 765 cases, 92577 bytes (rechecked by verify_constantfix10) |
| verify10 | player-gui round 7: controllerlife181 | EXACT | comparison | CONTROLLER LIFECYCLE EXACT: 1000 factory/flip cases, 20000 partial/reset boundaries, original air failure (1232729 bytes) (rechecked by verify_constantfix10) |
| verify10 | player-gui round 7: inventorybounds181 | DIFFERS | comparison | byte 74440, for(int n:s==null?new int[]{-1,0,0,0}:new int[]{s.itemID,s.stackSize,s.getItemDamage(),s.animationsToGo})i(o,n); = 111 (PlayerOracle.java:201, field starts 74440, 4 bytes, Integer); Rust -1 (rechecked by verify_constantfix10) |
| verify10 | player-gui round 7: original inventory-screen callbacks | EXACT | comparison | INVENTORY GUI EXACT: 10000 original constructor/init cases, 30000 updates and 50000 button callbacks (940000 bytes) (rechecked by verify_constantfix10) |
| verify10 | player-gui round 7: original general-stat drawing arguments | EXACT | comparison | GENERAL STATS EXACT: 1024 original list/row callbacks, every general stat, signed format boundaries and draw arguments (1740218 bytes) (rechecked by verify_constantfix10) |
| verify10 | gui10: b10_previewlight0 | DIFFERS | comparison | pixel (355,113).RGB: Java (24, 16, 7), Rust (23, 16, 6) |
| verify10 | gui10: b10_previewlight1 | DIFFERS | comparison | pixel (355,113).RGB: Java (26, 16, 7), Rust (24, 16, 6) |
| verify10 | gui10: b10_previewlight2 | DIFFERS | comparison | pixel (355,113).RGB: Java (28, 16, 7), Rust (26, 16, 6) |
| verify10 | gui10: b10_previewlight3 | DIFFERS | comparison | pixel (355,113).RGB: Java (30, 16, 7), Rust (28, 16, 6) |
| verify10 | gui10: b10_previewlight4 | DIFFERS | comparison | pixel (355,113).RGB: Java (32, 16, 7), Rust (31, 16, 6) |
| verify10 | gui10: b10_previewlight5 | DIFFERS | comparison | pixel (355,113).RGB: Java (35, 16, 7), Rust (34, 16, 6) |
| verify10 | gui10: b10_previewlight6 | DIFFERS | comparison | pixel (355,113).RGB: Java (38, 16, 7), Rust (36, 16, 6) |
| verify10 | gui10: b10_previewlight7 | DIFFERS | comparison | pixel (355,113).RGB: Java (41, 16, 7), Rust (40, 16, 6) |
| verify10 | gui10: b10_previewlight8 | DIFFERS | comparison | pixel (355,113).RGB: Java (46, 16, 7), Rust (44, 16, 6) |
| verify10 | gui10: b10_previewlight9 | DIFFERS | comparison | pixel (355,113).RGB: Java (50, 16, 7), Rust (49, 16, 6) |
| verify10 | gui10: b10_previewlight10 | DIFFERS | comparison | pixel (355,113).RGB: Java (56, 16, 7), Rust (55, 16, 6) |
| verify10 | gui10: b10_previewlight11 | DIFFERS | comparison | pixel (355,113).RGB: Java (63, 16, 7), Rust (62, 16, 6) |
| verify10 | gui10: b10_previewlight12 | DIFFERS | comparison | pixel (355,113).RGB: Java (72, 16, 7), Rust (71, 16, 6) |
| verify10 | gui10: b10_previewlight13 | DIFFERS | comparison | pixel (355,113).RGB: Java (84, 16, 7), Rust (82, 16, 6) |
| verify10 | gui10: b10_previewlight14 | DIFFERS | comparison | pixel (355,113).RGB: Java (99, 16, 7), Rust (98, 16, 6) |
| verify10 | gui10: b10_previewlight15 | DIFFERS | comparison | pixel (355,113).RGB: Java (120, 16, 7), Rust (119, 16, 6) |
| verify10 | gui10: b10_inventory_to_creative | DIFFERS | comparison | pixel (268,76).RGB: Java (79, 79, 79), Rust (93, 93, 93) |
| verify10 | gui10: b10_creative_to_inventory | DIFFERS | comparison | pixel (355,113).RGB: Java (40, 27, 12), Rust (38, 26, 10) |
| verify10 | entities7: beditem7 | EXACT | comparison | EXACT: entities181 beditem7 (16005120 bytes) |
| verify10 | entities7: signitem7 | EXACT | comparison | EXACT: entities181 signitem7 (16039920 bytes) |
| verify10 | entities7: metadata7 | DIFFERS | comparison | byte 13658016, item.getColorFromDamage(d) = 4764952 (Mode_itemmetadata7_10.java:6, field starts 13658016, 4 bytes, Integer); Rust -8602261 |
| verify10 | entities7: metaplace7 | EXACT | comparison | EXACT: entities181 blockitem6 (529120 bytes) |
| verify10 | entities7: buckethit7 | DIFFERS | comparison | byte 1273940, original==result = true (Mode_itembucket7_10.java:24, field starts 1273940, 1 bytes, Boolean); Rust False (rechecked by verify_bucketfix10) |
| verify10 | entities7: bucketray7 | DIFFERS | comparison | byte 106301, original==result = true (Mode_itembucket7_10.java:24, field starts 106301, 1 bytes, Boolean); Rust False (rechecked by verify_bucketfix10) |
| verify10 | entities7: logconstructor7 | EXACT | comparison | EXACT: entities181 logconstructor7 (7140 bytes) |
| verify10 | entities7: vineconstructor7 | DIFFERS | comparison | byte 8, item.getHasSubtypes() = true (Mode_itemvine7_10.java:14, field starts 8, 1 bytes, Boolean); Rust False |
| verify10 | player-gui round 8: environmental death and self-owned arrows | DIFFERS | comparison | byte 247, sound.name: Java 'damage.hurtflesh', Rust 'random.hurt' (rechecked by verify_constantfix10) |
| verify10 | player-gui round 8: retained player movement (1000 frames) | DIFFERS | comparison | byte 2310, frame 1, Player id 0.rand: Java 206789181479531, Rust 203227649148896 (rechecked by verify_constantfix10) |
| verify10 | player-gui round 8: retained player damage (1000 frames) | DIFFERS | comparison | byte 2310, frame 1, Player id 0.rand: Java 206789181479531, Rust 203227649148896 (rechecked by verify_constantfix10) |
| verify10 | player-gui round 8: retained player pickup (1000 frames) | DIFFERS | comparison | byte 2310, frame 1, Player id 0.rand: Java 206789181479531, Rust 203227649148896 (rechecked by verify_constantfix10) |
| verify10 | player-gui round 8: retained player survival (800 frames) | DIFFERS | comparison | byte 2310, frame 1, Player id 0.rand: Java 206789181479531, Rust 203227649148896 (rechecked by verify_constantfix10) |
| verify10 | player-gui round 8: retained player bucket (100 frames) | DIFFERS | comparison | byte 2350, frame 1, Player id 0.rand: Java 206789181479531, Rust 203227649148896 (rechecked by verify_constantfix10) |
| verify10 | player-gui round 8: retained player combat (400 frames) | DIFFERS | comparison | byte 2318, frame 1, Player id 0.rand: Java 206789181479531, Rust 203227649148896 (rechecked by verify_constantfix10) |
| verify10 | player-gui round 8: guislot8_181 | EXACT | comparison | GUI SLOT ROUND 8 EXACT: 10000 original scroll/drag/header/row callbacks and retained mouse coordinates (2630232 bytes) (rechecked by verify_latefix10) |
| verify10 | player-gui round 8: yesno8_181 | EXACT | comparison | yesno8_181 EXACT (428032 bytes) (rechecked by verify_constantfix10) |
| verify10 | player-gui round 8: rename8_181 | EXACT | comparison | rename8_181 EXACT (47165 bytes) (rechecked by verify_constantfix10) |
| verify10 | player-gui round 8: select8_181 | EXACT | comparison | select8_181 EXACT (864178 bytes) (rechecked by verify_constantfix10) |
| verify10 | player-gui round 8: stats8_181 | DIFFERS | comparison | byte 49838, list.getSize() = 95 (Mode_stats8_10.java:42, field starts 49838, 4 bytes, Integer); Rust 82 (rechecked by verify_latefix10) |
| verify10 | gui10: b10_gui8_select | DIFFERS | comparison | pixel (452,96).RGB: Java (128, 128, 128), Rust (15, 11, 7) |
| verify10 | gui10: b10_gui8_selected | DIFFERS | comparison | pixel (452,96).RGB: Java (128, 128, 128), Rust (15, 11, 7) |
| verify10 | gui10: b10_gui8_yes_hover | DIFFERS | comparison | pixel (452,96).RGB: Java (128, 128, 128), Rust (15, 11, 7) |
| verify10 | gui10: b10_gui8_no_hover | DIFFERS | comparison | pixel (452,96).RGB: Java (128, 128, 128), Rust (15, 11, 7) |
| verify10 | gui10: b10_gui8_cancel_return | DIFFERS | comparison | pixel (452,96).RGB: Java (128, 128, 128), Rust (15, 11, 7) |
| verify10 | gui10: b10_gui8_rename | EXACT | comparison | b10_gui8_rename: EXACT |
| verify10 | gui10: b10_gui8_rename_cursor | EXACT | comparison | b10_gui8_rename_cursor: EXACT |
| verify10 | gui10: b10_gui8_rename_unfocused | EXACT | comparison | b10_gui8_rename_unfocused: EXACT |
| verify10 | gui10: b10_gui8_rename_empty | EXACT | comparison | b10_gui8_rename_empty: EXACT |
| verify10 | gui10: b10_gui8_rename_spaces | EXACT | comparison | b10_gui8_rename_spaces: EXACT |
| verify10 | gui10: b10_gui8_renamed_list | DIFFERS | comparison | pixel (452,96).RGB: Java (128, 128, 128), Rust (15, 11, 7) |
| verify10 | gui10: b10_gui8_deleted_list | DIFFERS | comparison | pixel (452,96).RGB: Java (128, 128, 128), Rust (15, 11, 7) |
| verify10 | gui10: b10_gui8_stats_blocks | EXACT | comparison | b10_gui8_stats_blocks: EXACT |
| verify10 | gui10: b10_gui8_stats_blocks_header | EXACT | comparison | b10_gui8_stats_blocks_header: EXACT |
| verify10 | gui10: b10_gui8_stats_blocks_press | EXACT | comparison | b10_gui8_stats_blocks_press: EXACT |
| verify10 | gui10: b10_gui8_stats_blocks_sorted | EXACT | comparison | b10_gui8_stats_blocks_sorted: EXACT |
| verify10 | gui10: b10_gui8_stats_blocks_tooltip | EXACT | comparison | b10_gui8_stats_blocks_tooltip: EXACT |
| verify10 | gui10: b10_gui8_stats_items | EXACT | comparison | b10_gui8_stats_items: EXACT |
| verify10 | gui10: b10_gui8_stats_items_header | EXACT | comparison | b10_gui8_stats_items_header: EXACT |
| verify10 | gui10: b10_gui8_stats_items_press | EXACT | comparison | b10_gui8_stats_items_press: EXACT |
| verify10 | gui10: b10_gui8_stats_items_sorted | EXACT | comparison | b10_gui8_stats_items_sorted: EXACT |
| verify10 | gui10: b10_gui8_stats_items_reverse_press | EXACT | comparison | b10_gui8_stats_items_reverse_press: EXACT |
| verify10 | gui10: b10_gui8_stats_items_reverse | EXACT | comparison | b10_gui8_stats_items_reverse: EXACT |
| verify10 | gui10: b10_gui8_stats_items_clear_press | EXACT | comparison | b10_gui8_stats_items_clear_press: EXACT |
| verify10 | gui10: b10_gui8_stats_items_clear | EXACT | comparison | b10_gui8_stats_items_clear: EXACT |
| verify10 | gui10: b10_gui8_stats_items_tooltip | EXACT | comparison | b10_gui8_stats_items_tooltip: EXACT |
| verify10 | entities8: EffectRenderer dispatch | DIFFERS | comparison | byte 30, a = 0.0 (Mode_effectdispatch8_10.java:22, field starts 28, 4 bytes, Float); Rust 1.0 |
| verify10 | entities8: ItemMap lifecycle | EXACT | comparison | EXACT: entities181 itemmap8 (9386832 bytes) |
| verify10 | entities8: Wolf despawning | DIFFERS | comparison | byte 228804, e.isDead = false (Mode_wolfdespawn8_10.java:14, field starts 228804, 1 bytes, Boolean); Rust True |
| verify10 | complete 1.8.1 objects fixture (capture consistency) | EXACT | comparison | capture consistency EXACT (fixture integrity; Rust renderer parity is separate) (rechecked by verify_latefix10) |
| verify10 | gui10: b10_gui9_death1_light0 | DIFFERS | comparison | pixel (384,117).RGB: Java (28, 19, 8), Rust (26, 18, 8) |
| verify10 | gui10: b10_gui9_death1_light7 | DIFFERS | comparison | pixel (384,117).RGB: Java (47, 19, 8), Rust (45, 18, 8) |
| verify10 | gui10: b10_gui9_death1_light15 | DIFFERS | comparison | pixel (384,117).RGB: Java (130, 19, 8), Rust (128, 18, 8) |
| verify10 | gui10: b10_gui9_death2_light0 | DIFFERS | comparison | pixel (405,124).RGB: Java (28, 19, 9), Rust (26, 18, 8) |
| verify10 | gui10: b10_gui9_death2_light7 | DIFFERS | comparison | pixel (405,124).RGB: Java (48, 19, 9), Rust (46, 18, 8) |
| verify10 | gui10: b10_gui9_death2_light15 | DIFFERS | comparison | pixel (405,124).RGB: Java (130, 19, 9), Rust (128, 18, 8) |
| verify10 | gui10: b10_gui9_death5_light0 | DIFFERS | comparison | pixel (430,142).RGB: Java (28, 19, 8), Rust (26, 18, 8) |
| verify10 | gui10: b10_gui9_death5_light7 | DIFFERS | comparison | pixel (430,142).RGB: Java (49, 19, 8), Rust (47, 18, 8) |
| verify10 | gui10: b10_gui9_death5_light15 | DIFFERS | comparison | pixel (430,142).RGB: Java (130, 19, 8), Rust (128, 18, 8) |
| verify10 | gui10: b10_gui9_death9_light0 | DIFFERS | comparison | pixel (413,168).RGB: Java (0, 79, 79), Rust (0, 87, 87) |
| verify10 | gui10: b10_gui9_death9_light7 | DIFFERS | comparison | pixel (413,168).RGB: Java (15, 79, 79), Rust (15, 87, 87) |
| verify10 | gui10: b10_gui9_death9_light15 | DIFFERS | comparison | pixel (413,168).RGB: Java (84, 79, 79), Rust (84, 87, 87) |
| verify10 | gui10: b10_gui9_death13_light0 | DIFFERS | comparison | pixel (418,186).RGB: Java (0, 88, 88), Rust (0, 84, 84) |
| verify10 | gui10: b10_gui9_death13_light7 | DIFFERS | comparison | pixel (418,186).RGB: Java (15, 88, 88), Rust (15, 84, 84) |
| verify10 | gui10: b10_gui9_death13_light15 | DIFFERS | comparison | pixel (418,186).RGB: Java (85, 88, 88), Rust (85, 84, 84) |
| verify10 | gui10: b10_gui9_death20_light0 | DIFFERS | comparison | pixel (418,186).RGB: Java (0, 88, 88), Rust (0, 84, 84) |
| verify10 | gui10: b10_gui9_death20_light7 | DIFFERS | comparison | pixel (418,186).RGB: Java (15, 88, 88), Rust (15, 84, 84) |
| verify10 | gui10: b10_gui9_death20_light15 | DIFFERS | comparison | pixel (418,186).RGB: Java (85, 88, 88), Rust (85, 84, 84) |
| verify10 | gui10: b10_gui9_armor_death1 | DIFFERS | comparison | pixel (395,112).RGB: Java (36, 123, 112), Rust (38, 129, 118) |
| verify10 | gui10: b10_gui9_armor_death5 | DIFFERS | comparison | pixel (433,137).RGB: Java (35, 118, 107), Rust (38, 129, 118) |
| verify10 | gui10: b10_gui9_armor_death13 | DIFFERS | comparison | pixel (410,182).RGB: Java (29, 95, 86), Rust (198, 198, 198) |
| verify10 | gui10: b10_gui9_armor_death20 | DIFFERS | comparison | pixel (410,182).RGB: Java (29, 95, 86), Rust (198, 198, 198) |
| verify10 | gui10: b10_gui9_mouse0_first | DIFFERS | comparison | pixel (384,117).RGB: Java (47, 19, 8), Rust (45, 18, 8) |
| verify10 | gui10: b10_gui9_mouse0_retained | DIFFERS | comparison | pixel (391,125).RGB: Java (44, 17, 8), Rust (41, 14, 5) |
| verify10 | gui10: b10_gui9_mouse1_first | DIFFERS | comparison | pixel (430,142).RGB: Java (49, 19, 8), Rust (47, 18, 8) |
| verify10 | gui10: b10_gui9_mouse1_retained | DIFFERS | comparison | pixel (432,149).RGB: Java (44, 16, 6), Rust (43, 16, 6) |
| verify10 | gui10: b10_gui9_mouse2_first | DIFFERS | comparison | pixel (418,186).RGB: Java (15, 88, 88), Rust (15, 84, 84) |
| verify10 | gui10: b10_gui9_mouse2_retained | DIFFERS | comparison | pixel (421,184).RGB: Java (15, 77, 77), Rust (15, 86, 86) |
| verify10 | gui10: b10_gui9_mouse3_first | DIFFERS | comparison | pixel (418,186).RGB: Java (15, 88, 88), Rust (15, 84, 84) |
| verify10 | gui10: b10_gui9_mouse3_retained | DIFFERS | comparison | pixel (418,180).RGB: Java (23, 62, 62), Rust (23, 76, 76) |
| verify10 | gui10: b10_gui9_death1_light0 alpha | EXACT | comparison | b10_gui9_death1_light0: ALPHA EXACT (rechecked by verify_alphafix10) |
| verify10 | gui10: b10_gui9_death1_light7 alpha | EXACT | comparison | b10_gui9_death1_light7: ALPHA EXACT (rechecked by verify_alphafix10) |
| verify10 | gui10: b10_gui9_death1_light15 alpha | EXACT | comparison | b10_gui9_death1_light15: ALPHA EXACT (rechecked by verify_alphafix10) |
| verify10 | gui10: b10_gui9_death2_light0 alpha | EXACT | comparison | b10_gui9_death2_light0: ALPHA EXACT (rechecked by verify_alphafix10) |
| verify10 | gui10: b10_gui9_death2_light7 alpha | EXACT | comparison | b10_gui9_death2_light7: ALPHA EXACT (rechecked by verify_alphafix10) |
| verify10 | gui10: b10_gui9_death2_light15 alpha | EXACT | comparison | b10_gui9_death2_light15: ALPHA EXACT (rechecked by verify_alphafix10) |
| verify10 | gui10: b10_gui9_death5_light0 alpha | EXACT | comparison | b10_gui9_death5_light0: ALPHA EXACT (rechecked by verify_alphafix10) |
| verify10 | gui10: b10_gui9_death5_light7 alpha | EXACT | comparison | b10_gui9_death5_light7: ALPHA EXACT (rechecked by verify_alphafix10) |
| verify10 | gui10: b10_gui9_death5_light15 alpha | EXACT | comparison | b10_gui9_death5_light15: ALPHA EXACT (rechecked by verify_alphafix10) |
| verify10 | gui10: b10_gui9_death9_light0 alpha | EXACT | comparison | b10_gui9_death9_light0: ALPHA EXACT (rechecked by verify_alphafix10) |
| verify10 | gui10: b10_gui9_death9_light7 alpha | EXACT | comparison | b10_gui9_death9_light7: ALPHA EXACT (rechecked by verify_alphafix10) |
| verify10 | gui10: b10_gui9_death9_light15 alpha | EXACT | comparison | b10_gui9_death9_light15: ALPHA EXACT (rechecked by verify_alphafix10) |
| verify10 | gui10: b10_gui9_death13_light0 alpha | EXACT | comparison | b10_gui9_death13_light0: ALPHA EXACT (rechecked by verify_alphafix10) |
| verify10 | gui10: b10_gui9_death13_light7 alpha | EXACT | comparison | b10_gui9_death13_light7: ALPHA EXACT (rechecked by verify_alphafix10) |
| verify10 | gui10: b10_gui9_death13_light15 alpha | EXACT | comparison | b10_gui9_death13_light15: ALPHA EXACT (rechecked by verify_alphafix10) |
| verify10 | gui10: b10_gui9_death20_light0 alpha | EXACT | comparison | b10_gui9_death20_light0: ALPHA EXACT (rechecked by verify_alphafix10) |
| verify10 | gui10: b10_gui9_death20_light7 alpha | EXACT | comparison | b10_gui9_death20_light7: ALPHA EXACT (rechecked by verify_alphafix10) |
| verify10 | gui10: b10_gui9_death20_light15 alpha | EXACT | comparison | b10_gui9_death20_light15: ALPHA EXACT (rechecked by verify_alphafix10) |
| verify10 | gui10: b10_gui9_armor_death1 alpha | EXACT | comparison | b10_gui9_armor_death1: ALPHA EXACT (rechecked by verify_alphafix10) |
| verify10 | gui10: b10_gui9_armor_death5 alpha | DIFFERS | comparison | pixel (420,203).alpha: Java 194, Rust 255 (rechecked by verify_alphafix10) |
| verify10 | gui10: b10_gui9_armor_death13 alpha | DIFFERS | comparison | pixel (410,182).alpha: Java 194, Rust 255 (rechecked by verify_alphafix10) |
| verify10 | gui10: b10_gui9_armor_death20 alpha | DIFFERS | comparison | pixel (410,182).alpha: Java 194, Rust 255 (rechecked by verify_alphafix10) |
| verify10 | gui10: b10_gui9_mouse0_first alpha | EXACT | comparison | b10_gui9_mouse0_first: ALPHA EXACT (rechecked by verify_alphafix10) |
| verify10 | gui10: b10_gui9_mouse0_retained alpha | EXACT | comparison | b10_gui9_mouse0_retained: ALPHA EXACT (rechecked by verify_alphafix10) |
| verify10 | gui10: b10_gui9_mouse1_first alpha | EXACT | comparison | b10_gui9_mouse1_first: ALPHA EXACT (rechecked by verify_alphafix10) |
| verify10 | gui10: b10_gui9_mouse1_retained alpha | EXACT | comparison | b10_gui9_mouse1_retained: ALPHA EXACT (rechecked by verify_alphafix10) |
| verify10 | gui10: b10_gui9_mouse2_first alpha | EXACT | comparison | b10_gui9_mouse2_first: ALPHA EXACT (rechecked by verify_alphafix10) |
| verify10 | gui10: b10_gui9_mouse2_retained alpha | EXACT | comparison | b10_gui9_mouse2_retained: ALPHA EXACT (rechecked by verify_alphafix10) |
| verify10 | gui10: b10_gui9_mouse3_first alpha | EXACT | comparison | b10_gui9_mouse3_first: ALPHA EXACT (rechecked by verify_alphafix10) |
| verify10 | gui10: b10_previewlight0 | DIFFERS | comparison | pixel (355,113).RGB: Java (24, 16, 7), Rust (23, 16, 6) |
| verify10 | gui10: b10_previewlight1 | DIFFERS | comparison | pixel (355,113).RGB: Java (26, 16, 7), Rust (24, 16, 6) |
| verify10 | gui10: b10_previewlight2 | DIFFERS | comparison | pixel (355,113).RGB: Java (28, 16, 7), Rust (26, 16, 6) |
| verify10 | gui10: b10_previewlight3 | DIFFERS | comparison | pixel (355,113).RGB: Java (30, 16, 7), Rust (28, 16, 6) |
| verify10 | gui10: b10_previewlight4 | DIFFERS | comparison | pixel (355,113).RGB: Java (32, 16, 7), Rust (31, 16, 6) |
| verify10 | gui10: b10_previewlight5 | DIFFERS | comparison | pixel (355,113).RGB: Java (35, 16, 7), Rust (34, 16, 6) |
| verify10 | gui10: b10_previewlight6 | DIFFERS | comparison | pixel (355,113).RGB: Java (38, 16, 7), Rust (36, 16, 6) |
| verify10 | gui10: b10_previewlight7 | DIFFERS | comparison | pixel (355,113).RGB: Java (41, 16, 7), Rust (40, 16, 6) |
| verify10 | gui10: b10_previewlight8 | DIFFERS | comparison | pixel (355,113).RGB: Java (46, 16, 7), Rust (44, 16, 6) |
| verify10 | gui10: b10_previewlight9 | DIFFERS | comparison | pixel (355,113).RGB: Java (50, 16, 7), Rust (49, 16, 6) |
| verify10 | gui10: b10_previewlight10 | DIFFERS | comparison | pixel (355,113).RGB: Java (56, 16, 7), Rust (55, 16, 6) |
| verify10 | gui10: b10_previewlight11 | DIFFERS | comparison | pixel (355,113).RGB: Java (63, 16, 7), Rust (62, 16, 6) |
| verify10 | gui10: b10_previewlight12 | DIFFERS | comparison | pixel (355,113).RGB: Java (72, 16, 7), Rust (71, 16, 6) |
| verify10 | gui10: b10_previewlight13 | DIFFERS | comparison | pixel (355,113).RGB: Java (84, 16, 7), Rust (82, 16, 6) |
| verify10 | gui10: b10_previewlight14 | DIFFERS | comparison | pixel (355,113).RGB: Java (99, 16, 7), Rust (98, 16, 6) |
| verify10 | gui10: b10_previewlight15 | DIFFERS | comparison | pixel (355,113).RGB: Java (120, 16, 7), Rust (119, 16, 6) |
| verify10 | gui10: b10_inventory_to_creative | DIFFERS | comparison | pixel (268,76).RGB: Java (79, 79, 79), Rust (93, 93, 93) |
| verify10 | gui10: b10_creative_to_inventory | DIFFERS | comparison | pixel (355,113).RGB: Java (40, 27, 12), Rust (38, 26, 10) |
| verify10 | gui10: b10_previewlight0 alpha | EXACT | comparison | b10_previewlight0: ALPHA EXACT (rechecked by verify_alphafix10) |
| verify10 | gui10: b10_previewlight1 alpha | EXACT | comparison | b10_previewlight1: ALPHA EXACT (rechecked by verify_alphafix10) |
| verify10 | gui10: b10_previewlight2 alpha | EXACT | comparison | b10_previewlight2: ALPHA EXACT (rechecked by verify_alphafix10) |
| verify10 | gui10: b10_previewlight3 alpha | EXACT | comparison | b10_previewlight3: ALPHA EXACT (rechecked by verify_alphafix10) |
| verify10 | gui10: b10_previewlight4 alpha | EXACT | comparison | b10_previewlight4: ALPHA EXACT (rechecked by verify_alphafix10) |
| verify10 | gui10: b10_previewlight5 alpha | EXACT | comparison | b10_previewlight5: ALPHA EXACT (rechecked by verify_alphafix10) |
| verify10 | gui10: b10_previewlight6 alpha | EXACT | comparison | b10_previewlight6: ALPHA EXACT (rechecked by verify_alphafix10) |
| verify10 | gui10: b10_previewlight7 alpha | EXACT | comparison | b10_previewlight7: ALPHA EXACT (rechecked by verify_alphafix10) |
| verify10 | gui10: b10_previewlight8 alpha | EXACT | comparison | b10_previewlight8: ALPHA EXACT (rechecked by verify_alphafix10) |
| verify10 | gui10: b10_previewlight9 alpha | EXACT | comparison | b10_previewlight9: ALPHA EXACT (rechecked by verify_alphafix10) |
| verify10 | gui10: b10_previewlight10 alpha | EXACT | comparison | b10_previewlight10: ALPHA EXACT (rechecked by verify_alphafix10) |
| verify10 | gui10: b10_previewlight11 alpha | EXACT | comparison | b10_previewlight11: ALPHA EXACT (rechecked by verify_alphafix10) |
| verify10 | gui10: b10_previewlight12 alpha | EXACT | comparison | b10_previewlight12: ALPHA EXACT (rechecked by verify_alphafix10) |
| verify10 | gui10: b10_previewlight13 alpha | EXACT | comparison | b10_previewlight13: ALPHA EXACT (rechecked by verify_alphafix10) |
| verify10 | gui10: b10_previewlight14 alpha | EXACT | comparison | b10_previewlight14: ALPHA EXACT (rechecked by verify_alphafix10) |
| verify10 | gui10: b10_previewlight15 alpha | EXACT | comparison | b10_previewlight15: ALPHA EXACT (rechecked by verify_alphafix10) |
| verify10 | gui10: b10_inventory_to_creative alpha | DIFFERS | comparison | pixel (460,249).alpha: Java 255, Rust 159 (rechecked by verify_alphafix10) |
| verify10 | gui10: inventory_empty | DIFFERS | comparison | pixel (355,113).RGB: Java (40, 27, 12), Rust (38, 26, 10) |
| verify10 | gui10: inventory_items | DIFFERS | comparison | pixel (353,114).RGB: Java (41, 28, 12), Rust (42, 28, 13) |
| verify10 | gui10: inventory_turn | DIFFERS | comparison | pixel (347,114).RGB: Java (36, 24, 8), Rust (47, 31, 15) |
| verify10 | gui10: inventory_cursor | DIFFERS | comparison | pixel (345,114).RGB: Java (42, 29, 13), Rust (36, 24, 8) |
| verify10 | gui10: crafting_items | DIFFERS | comparison | pixel (312,116).RGB: Java (119, 97, 62), Rust (139, 113, 73) |
| verify10 | gui10: crafting_tooltip | DIFFERS | comparison | pixel (522,114).RGB: Java (27, 12, 27), Rust (198, 198, 198) |
| verify10 | gui10: chest_items | DIFFERS | comparison | pixel (268,116).RGB: Java (34, 111, 27), Rust (39, 129, 32) |
| verify10 | gui10: chest_cursor | DIFFERS | comparison | pixel (268,116).RGB: Java (34, 111, 27), Rust (39, 129, 32) |
| verify10 | gui10: largechest_items | DIFFERS | comparison | pixel (556,242).RGB: Java (134, 71, 88), Rust (156, 83, 103) |
| verify10 | gui10: furnace_mid | DIFFERS | comparison | pixel (364,116).RGB: Java (81, 81, 81), Rust (94, 94, 94) |
| verify10 | gui10: furnace_idle | DIFFERS | comparison | pixel (364,116).RGB: Java (81, 81, 81), Rust (94, 94, 94) |
| verify10 | gui10: furnace_missing_fuel_duration | DIFFERS | comparison | pixel (364,116).RGB: Java (81, 81, 81), Rust (94, 94, 94) |
| verify10 | gui10: dispenser_items | DIFFERS | comparison | pixel (394,114).RGB: Java (24, 9, 24), Rust (150, 150, 150) |
| verify10 | gui10: inventory_diamond_pickaxe | DIFFERS | comparison | pixel (352,107).RGB: Java (45, 197, 180), Rust (44, 194, 177) |
| verify10 | gui10: inventory_equipped_turn | DIFFERS | comparison | pixel (354,108).RGB: Java (48, 206, 190), Rust (45, 197, 180) |
| verify10 | gui10: inventory_pumpkin_block | DIFFERS | comparison | pixel (381,139).RGB: Java (49, 211, 193), Rust (44, 194, 176) |
| verify10 | gui10: inventory_fishing_rod | DIFFERS | comparison | pixel (381,139).RGB: Java (49, 211, 193), Rust (44, 194, 176) |
| verify10 | gui10: inventory_bow | DIFFERS | comparison | pixel (381,139).RGB: Java (49, 211, 193), Rust (44, 194, 176) |
| verify10 | gui10: inventory_dye | DIFFERS | comparison | pixel (381,139).RGB: Java (49, 211, 193), Rust (44, 194, 176) |
| verify10 | gui10: chest_right_pickup | EXACT | comparison | chest_right_pickup: EXACT |
| verify10 | gui10: chest_right_place | EXACT | comparison | chest_right_place: EXACT |
| verify10 | gui10: chest_shift_transfer | EXACT | comparison | chest_shift_transfer: EXACT |
| verify10 | gui10: crafting_click_result | DIFFERS | comparison | pixel (312,116).RGB: Java (119, 97, 62), Rust (139, 113, 73) |
| verify10 | gui10: inventory_sneak_swing | DIFFERS | comparison | pixel (356,116).RGB: Java (30, 21, 11), Rust (34, 24, 11) |
| verify10 | gui10: inventory_riding | DIFFERS | comparison | pixel (353,114).RGB: Java (41, 28, 12), Rust (42, 28, 13) |
| verify10 | gui10: inventory_riding_armor | DIFFERS | comparison | pixel (354,108).RGB: Java (48, 206, 190), Rust (45, 197, 180) |
| verify10 | gui10: inventory_sneak_armor | DIFFERS | comparison | pixel (354,111).RGB: Java (48, 206, 190), Rust (45, 197, 180) |
| verify10 | gui10: inventory_sneak_armor_next_frame | DIFFERS | comparison | pixel (354,111).RGB: Java (48, 206, 190), Rust (45, 197, 180) |
| verify10 | gui10: inventory_hurt_armor | DIFFERS | comparison | pixel (357,108).RGB: Java (104, 124, 114), Rust (125, 124, 114) |
| verify10 | gui10: inventory_hurt | DIFFERS | comparison | pixel (357,110).RGB: Java (70, 13, 7), Rust (86, 13, 7) |
| verify10 | gui10: inventory_cloth | DIFFERS | comparison | pixel (354,108).RGB: Java (135, 84, 54), Rust (129, 79, 51) |
| verify10 | gui10: inventory_chain | DIFFERS | comparison | pixel (357,107).RGB: Java (0, 0, 0), Rust (160, 160, 160) |
| verify10 | gui10: inventory_iron | DIFFERS | comparison | pixel (354,108).RGB: Java (193, 193, 193), Rust (188, 188, 188) |
| verify10 | gui10: inventory_gold | DIFFERS | comparison | pixel (354,108).RGB: Java (231, 214, 30), Rust (228, 206, 28) |
| verify10 | gui10: inventory_signed_counts | DIFFERS | comparison | pixel (355,113).RGB: Java (40, 27, 12), Rust (38, 26, 10) |
| verify10 | gui10: inventory_empty alpha | EXACT | comparison | inventory_empty: ALPHA EXACT (rechecked by verify_alphafix10) |
| verify10 | gui10: inventory_items alpha | DIFFERS | comparison | pixel (282,212).alpha: Java 241, Rust 255 (rechecked by verify_alphafix10) |
| verify10 | gui10: inventory_turn alpha | DIFFERS | comparison | pixel (348,228).alpha: Java 241, Rust 255 (rechecked by verify_alphafix10) |
| verify10 | gui10: inventory_cursor alpha | EXACT | comparison | inventory_cursor: ALPHA EXACT (rechecked by verify_alphafix10) |
| verify10 | gui10: crafting_items alpha | EXACT | comparison | crafting_items: ALPHA EXACT (rechecked by verify_alphafix10) |
| verify10 | gui10: crafting_tooltip alpha | DIFFERS | comparison | pixel (522,114).alpha: Java 241, Rust 255 (rechecked by verify_alphafix10) |
| verify10 | gui10: chest_items alpha | EXACT | comparison | chest_items: ALPHA EXACT (rechecked by verify_alphafix10) |
| verify10 | gui10: chest_cursor alpha | EXACT | comparison | chest_cursor: ALPHA EXACT (rechecked by verify_alphafix10) |
| verify10 | gui10: largechest_items alpha | EXACT | comparison | largechest_items: ALPHA EXACT (rechecked by verify_alphafix10) |
| verify10 | gui10: furnace_mid alpha | EXACT | comparison | furnace_mid: ALPHA EXACT (rechecked by verify_alphafix10) |
| verify10 | gui10: furnace_idle alpha | EXACT | comparison | furnace_idle: ALPHA EXACT (rechecked by verify_alphafix10) |
| verify10 | gui10: furnace_missing_fuel_duration alpha | EXACT | comparison | furnace_missing_fuel_duration: ALPHA EXACT (rechecked by verify_alphafix10) |
| verify10 | gui10: dispenser_items alpha | DIFFERS | comparison | pixel (394,114).alpha: Java 241, Rust 255 (rechecked by verify_alphafix10) |
| verify10 | gui10: inventory_diamond_pickaxe alpha | EXACT | comparison | inventory_diamond_pickaxe: ALPHA EXACT (rechecked by verify_alphafix10) |
| verify10 | gui10: inventory_equipped_turn alpha | EXACT | comparison | inventory_equipped_turn: ALPHA EXACT (rechecked by verify_alphafix10) |
| verify10 | gui10: inventory_pumpkin_block alpha | EXACT | comparison | inventory_pumpkin_block: ALPHA EXACT (rechecked by verify_alphafix10) |
| verify10 | gui10: inventory_fishing_rod alpha | EXACT | comparison | inventory_fishing_rod: ALPHA EXACT (rechecked by verify_alphafix10) |
| verify10 | gui10: inventory_bow alpha | EXACT | comparison | inventory_bow: ALPHA EXACT (rechecked by verify_alphafix10) |
| verify10 | gui10: inventory_dye alpha | EXACT | comparison | inventory_dye: ALPHA EXACT (rechecked by verify_alphafix10) |
| verify10 | gui10: chest_right_pickup alpha | EXACT | comparison | chest_right_pickup: ALPHA EXACT (rechecked by verify_alphafix10) |
| verify10 | gui10: chest_right_place alpha | EXACT | comparison | chest_right_place: ALPHA EXACT (rechecked by verify_alphafix10) |
| verify10 | gui10: chest_shift_transfer alpha | EXACT | comparison | chest_shift_transfer: ALPHA EXACT (rechecked by verify_alphafix10) |
| verify10 | gui10: crafting_click_result alpha | EXACT | comparison | crafting_click_result: ALPHA EXACT (rechecked by verify_alphafix10) |
| verify10 | gui10: inventory_sneak_swing alpha | EXACT | comparison | inventory_sneak_swing: ALPHA EXACT (rechecked by verify_alphafix10) |
| verify10 | gui10: inventory_riding alpha | EXACT | comparison | inventory_riding: ALPHA EXACT (rechecked by verify_alphafix10) |
| verify10 | gui10: inventory_riding_armor alpha | EXACT | comparison | inventory_riding_armor: ALPHA EXACT (rechecked by verify_alphafix10) |
| verify10 | gui10: inventory_sneak_armor alpha | EXACT | comparison | inventory_sneak_armor: ALPHA EXACT (rechecked by verify_alphafix10) |
| verify10 | gui10: inventory_sneak_armor_next_frame alpha | EXACT | comparison | inventory_sneak_armor_next_frame: ALPHA EXACT (rechecked by verify_alphafix10) |
| verify10 | gui10: inventory_hurt_armor alpha | EXACT | comparison | inventory_hurt_armor: ALPHA EXACT (rechecked by verify_alphafix10) |
| verify10 | gui10: inventory_hurt alpha | EXACT | comparison | inventory_hurt: ALPHA EXACT (rechecked by verify_alphafix10) |
| verify10 | gui10: inventory_cloth alpha | EXACT | comparison | inventory_cloth: ALPHA EXACT (rechecked by verify_alphafix10) |
| verify10 | gui10: inventory_chain alpha | EXACT | comparison | inventory_chain: ALPHA EXACT (rechecked by verify_alphafix10) |
| verify10 | gui10: inventory_iron alpha | EXACT | comparison | inventory_iron: ALPHA EXACT (rechecked by verify_alphafix10) |
| verify10 | gui10: inventory_gold alpha | EXACT | comparison | inventory_gold: ALPHA EXACT (rechecked by verify_alphafix10) |
| verify10 | gui10: packs_list alpha | EXACT | comparison | packs_list: ALPHA EXACT (rechecked by verify_alphafix10) |
| verify10 | gui10: packs_selected alpha | EXACT | comparison | packs_selected: ALPHA EXACT (rechecked by verify_alphafix10) |
| verify10 | gui10: pack_title alpha | DIFFERS | comparison | pixel (0,0).alpha: Java 191, Rust 198 (rechecked by verify_alphafix10) |
| verify10 | gui10: pack_hud alpha | DIFFERS | comparison | pixel (4,4).alpha: Java 0, Rust 255 (rechecked by verify_alphafix10) |
| verify10 | gui10: chat_empty alpha | EXACT | comparison | chat_empty: ALPHA EXACT (rechecked by verify_alphafix10) |
| verify10 | gui10: chat_typed alpha | EXACT | comparison | chat_typed: ALPHA EXACT (rechecked by verify_alphafix10) |
| verify10 | gui10: chat_cursor_off alpha | EXACT | comparison | chat_cursor_off: ALPHA EXACT (rechecked by verify_alphafix10) |
| verify10 | gui10: chat_cursor_on alpha | EXACT | comparison | chat_cursor_on: ALPHA EXACT (rechecked by verify_alphafix10) |
| verify10 | gui10: chat_limit alpha | EXACT | comparison | chat_limit: ALPHA EXACT (rechecked by verify_alphafix10) |
| verify10 | gui10: chat_limit_backspace alpha | EXACT | comparison | chat_limit_backspace: ALPHA EXACT (rechecked by verify_alphafix10) |
| verify10 | gui10: anaglyph_video alpha | DIFFERS | comparison | pixel (436,272).alpha: Java 255, Rust 158 (rechecked by verify_alphafix10) |
| verify10 | gui10: anaglyph_packs alpha | DIFFERS | comparison | pixel (436,272).alpha: Java 255, Rust 158 (rechecked by verify_alphafix10) |
| verify10 | gui10: anaglyph_pack_title alpha | DIFFERS | comparison | pixel (0,0).alpha: Java 191, Rust 198 (rechecked by verify_alphafix10) |
| verify10 | gui10: anaglyph_start_font alpha | DIFFERS | comparison | pixel (0,0).alpha: Java 191, Rust 198 (rechecked by verify_alphafix10) |
| verify10 | gui10: anaglyph_font_palette_retained alpha | DIFFERS | comparison | pixel (0,0).alpha: Java 191, Rust 198 (rechecked by verify_alphafix10) |
| verify10 | entities9: RenderManager registry, camera and dispatch | DIFFERS | comparison | byte 0, RenderManager m=manager();Render[] slots=new Render[registered.length];i(map(m).size()); = 43 (Mode_rendermanager9_10.java:45, field starts 0, 4 bytes, Integer); Rust 32 |
| verify10 | entities9: actual shadow and thrown-item renderers | EXACT | comparison | manager visuals: 160 cases, 7544 comparisons, 0 differences |
| verify10 | entities9: TileEntitySpecialRenderer context and resources | EXACT | comparison | EXACT: TileEntitySpecialRenderer (864 cases, 55872 bytes) |
| verify10 | entities9: TileEntityRenderer class cache, active context and dispatch | DIFFERS | comparison | byte 80; original fixture was not repeatable under tracing; byte 80, TileEntityRenderer m=manager();TileEntitySpecialRenderer[] slots=new TileEntitySpecialRenderer[4];i(map(m).size()); = 6 (Mode_tilemanager9_10.java:23, field starts 80, 4 bytes, Integer) |
| verify10 | minecraft 1.0: original runTick event queues, clicks, screen/focus callbacks and world policy | DIFFERS | oracle failure | Oracle failed before comparison: java.lang.NullPointerException; at net.minecraft.client.Minecraft.clickMiddleMouseButton(Minecraft.java:2201) (rechecked by verify_constantfix10) |
| verify10 | minecraft 1.0: original preloadWorld coordinate, progress and lighting callbacks | EXACT | comparison | MINECRAFT PRELOAD EXACT: 128 cases, 0 differences (rechecked by verify_constantfix10) |
| verify10 | minecraft 1.0: retained clickMouse and sendClickBlockToController effects | DIFFERS | oracle failure | Oracle failed before comparison: java.lang.NullPointerException; at net.minecraft.client.Minecraft.clickMiddleMouseButton(Minecraft.java:2201) (rechecked by verify_constantfix10) |
| verify10 | minecraft 1.0: original debugInfoEntities counts and excluded particle layer | EXACT | comparison | MINECRAFT DEBUG ENTITIES EXACT: 10000 cases, 0 differences (rechecked by verify_constantfix10) |
| verify10 | minecraft 1.0: retained global key registry and press-count overflow | EXACT | comparison | GLOBAL KEY REGISTRY EXACT: 20000 events, 4 settings, 8 standalone bindings, overflow (rechecked by verify_constantfix10) |
| verify10 | minecraft 1.0: retained controller factory, spawn yaw and item-use release | EXACT | comparison | CONTROLLER LIFECYCLE EXACT: 1000 factory/flip cases, 20000 partial/reset boundaries, original air failure (1232729 bytes) (rechecked by verify_constantfix10) |
| verify10 | minecraft 1.0: retained create-world callbacks and WorldSettings | DIFFERS | comparison | byte 1611, field("field_35364_f").get(g).equals("creative") = false (Mode_worldcallbacks10.java:25, field starts 1611, 1 bytes, Boolean); Rust True (rechecked by verify_constantfix10) |
| verify10 | minecraft 1.0: retained world seed parsing and settings | EXACT | comparison | CREATE WORLD SEED POLICY EXACT: 65536 Java digits, 10096 seed/mode/features records (clock-random seeds classified; Rust GUI callbacks not replayed) (rechecked by verify_constantfix10) |
| verify10 | minecraft 1.0: retained saved-world controller selection | EXACT | comparison | select8_181 EXACT (864178 bytes) (rechecked by verify_constantfix10) |
| verify10 | entities10: itembase10 | EXACT | comparison | EXACT: entities181 itembase10 (42739 bytes) |
| verify10 | entities10: itemblock10 | DIFFERS | comparison | byte 816101, events.size() = 3 (Mode_itemblock10_10.java:37, field starts 816101, 4 bytes, Integer); Rust 2 |
| verify10 | entities10: entitylist10 | DIFFERS | comparison | byte 12803, e.isDead = false (Mode_entities10.java:27, field starts 12803, 1 bytes, Boolean); Rust True |
| verify10 | player-gui round 10: mainmenu10_181 | DIFFERS | comparison | byte 34, s.length() = 12 (Mode_worldcallbacks10.java:21, field starts 34, 4 bytes, Integer); Rust 27 (rechecked by verify_constantfix10) |
| verify10 | player-gui round 10: renderengine10_181 | EXACT | comparison | RENDERENGINE10: 2000 original texture-prefix/filter/reset/cache cases, 0 differences (rechecked by verify_constantfix10) |
| verify10 | gui10: b10_title10_0 | DIFFERS | comparison | pixel (0,0).RGB: Java (214, 219, 236), Rust (227, 231, 242) |
| verify10 | gui10: b10_title10_1 | DIFFERS | comparison | pixel (0,0).RGB: Java (214, 219, 236), Rust (227, 231, 242) |
| verify10 | gui10: b10_title10_7 | DIFFERS | comparison | pixel (0,0).RGB: Java (214, 219, 236), Rust (227, 231, 242) |
| verify10 | gui10: b10_title10_37 | DIFFERS | comparison | pixel (0,0).RGB: Java (214, 219, 238), Rust (227, 231, 244) |
| verify10 | gui10: b10_title10_100 | DIFFERS | comparison | pixel (0,0).RGB: Java (213, 222, 244), Rust (227, 233, 248) |
| verify10 | gui10: b10_title10_400 | DIFFERS | comparison | pixel (0,0).RGB: Java (204, 215, 236), Rust (221, 228, 242) |
| verify10 | gui10: b10_title10_999 | DIFFERS | comparison | pixel (0,0).RGB: Java (215, 226, 255), Rust (228, 236, 255) |
| verify10 | gui10: b10_title10_2000 | DIFFERS | comparison | pixel (0,0).RGB: Java (200, 211, 255), Rust (218, 225, 255) |
| verify10 | gui10: packs_list | EXACT | comparison | packs_list: EXACT |
| verify10 | gui10: packs_selected | EXACT | comparison | packs_selected: EXACT |
| verify10 | gui10: pack_title | DIFFERS | comparison | pixel (0,0).RGB: Java (214, 219, 236), Rust (227, 231, 242) |
| verify10 | gui10: pack_hud | DIFFERS | comparison | pixel (4,4).RGB: Java (0, 0, 0), Rust (255, 255, 255) |
| verify10 | gui10: chat_empty | EXACT | comparison | chat_empty: EXACT |
| verify10 | gui10: chat_typed | EXACT | comparison | chat_typed: EXACT |
| verify10 | gui10: chat_cursor_off | EXACT | comparison | chat_cursor_off: EXACT |
| verify10 | gui10: chat_cursor_on | EXACT | comparison | chat_cursor_on: EXACT |
| verify10 | gui10: chat_limit | EXACT | comparison | chat_limit: EXACT |
| verify10 | gui10: chat_limit_backspace | EXACT | comparison | chat_limit_backspace: EXACT |
| verify10 | gui10: anaglyph_video | DIFFERS | comparison | pixel (436,272).RGB: Java (0, 0, 0), Rust (13, 13, 13) |
| verify10 | gui10: anaglyph_packs | EXACT | comparison | anaglyph_packs: EXACT |
| verify10 | gui10: anaglyph_pack_title | DIFFERS | comparison | pixel (0,0).RGB: Java (191, 192, 188), Rust (212, 213, 210) |
| verify10 | gui10: anaglyph_start_font | DIFFERS | comparison | pixel (4,4).RGB: Java (0, 0, 0), Rust (255, 255, 255) |
| verify10 | gui10: anaglyph_font_palette_retained | DIFFERS | comparison | pixel (4,4).RGB: Java (0, 0, 0), Rust (255, 255, 255) |
| verify10 | entities11: blockfull11 (all registered block items) | DIFFERS | comparison | byte 0, ids.size() = 121 (Mode_itemblock6_10.java:10, field starts 0, 4 bytes, Integer); Rust 108 |
| verify10 | entities11: itempiston11 | EXACT | comparison | EXACT: entities181 itempiston11 (611680 bytes) |
| verify10 | entities11: itemstack11 | DIFFERS | comparison | byte 2922168, static void text(DataOutputStream o,String s)throws IOException {if(s==null){i(o,-1);return;}byte[] b=s.getBytes("UTF-8");i(o,b.length);o.write(b);} = 20 (WorldEventOracle.java:11, field starts 2922168, 4 bytes, Integer); Rust 0 |
| verify10 | entities11: stackdata11 | DIFFERS | comparison | byte 189112, s!=null = true (Mode_stackdata11_10.java:6, field starts 189112, 1 bytes, Boolean); Rust False |
| verify10 | entities11: stacknbt11 | EXACT | comparison | EXACT: entities181 stacknbt11 (43840 bytes) |
| verify10 | player-gui round 11: statfile10_181 | EXACT | comparison | STATFILE10: 3050 original schema/grammar/checksum/hash-order cases, 1000 byte-exact serializations, 0 differences (rechecked by verify_constantfix10) |
| verify10 | player-gui round 11: statfile11_181 | EXACT | comparison | STATFILE11: 3859 malformed/UTF16, 10 constructor disk/failure, 8 MD5 charset, 128 UTF16/encoded serializations, 256 decoder cases: 0 differences (rechecked by verify_constantfix10) |
| verify10 | player-gui round 11: renderengine10_181 | EXACT | comparison | RENDERENGINE10: 2000 original texture-prefix/filter/reset/cache cases, 0 differences (rechecked by verify_constantfix10) |
| verify10 | player-gui round 11: renderengine11_181 | EXACT | comparison | RENDERENGINE11: 672 cache/flag/failure cases, 180 mutable dynamic images plus 64 tree-bin deletion/reuse refreshes, 20 contents refresh, 96 mixed refresh, 32 colliding String keys, 12 buffer boundaries: 0 differences (rechecked by verify_constantfix10) |
| verify10 | player: movement, every frame and final world | DIFFERS | comparison | byte 2310, frame 1, Player id 0.rand: Java 206789181479531, Rust 203227649148896 (rechecked by verify_replays10) |
| verify10 | player: damage, every frame and final world | DIFFERS | comparison | byte 2310, frame 1, Player id 0.rand: Java 206789181479531, Rust 203227649148896 (rechecked by verify_replays10) |
| verify10 | player: pickup, every frame and final world | DIFFERS | comparison | byte 2310, frame 1, Player id 0.rand: Java 206789181479531, Rust 203227649148896 (rechecked by verify_replays10) |
| verify10 | player: sleep, every frame and final world | DIFFERS | comparison | byte 2310, frame 1, Player id 0.rand: Java 206789181479531, Rust 203227649148896 (rechecked by verify_replays10) |
| verify10 | player: portal, every frame and final world | DIFFERS | comparison | byte 2310, frame 1, Player id 0.rand: Java 206789181479531, Rust 203227649148896 (rechecked by verify_replays10) |
| verify10 | player: nbt, every frame and final world | DIFFERS | comparison | byte 369, frame 0, Player id 0.armor: Java 6, Rust 4 (rechecked by verify_replays10) |
| verify10 | player: look, every frame and final world | DIFFERS | comparison | byte 2394, frame 1, Player id 0.rand: Java 206789181479531, Rust 203227649148896 (rechecked by verify_replays10) |
| verify10 | player: dig, every frame and final world | DIFFERS | comparison | byte 2350, frame 1, Player id 0.rand: Java 206789181479531, Rust 203227649148896 (rechecked by verify_replays10) |
| verify10 | player: place, every frame and final world | DIFFERS | comparison | byte 2350, frame 1, Player id 0.rand: Java 206789181479531, Rust 203227649148896 (rechecked by verify_replays10) |
| verify10 | player: use, every frame and final world | DIFFERS | comparison | byte 2350, frame 1, Player id 0.rand: Java 206789181479531, Rust 203227649148896 (rechecked by verify_replays10) |
| verify10 | player: shoot, every frame and final world | DIFFERS | comparison | byte 2350, frame 1, Player id 0.rand: Java 206789181479531, Rust 203227649148896 (rechecked by verify_replays10) |
| verify10 | player: interact, every frame and final world | DIFFERS | comparison | byte 2350, frame 1, Player id 0.rand: Java 206789181479531, Rust 203227649148896 (rechecked by verify_replays10) |
| verify10 | player: fertilize, every frame and final world | DIFFERS | comparison | byte 2358, frame 1, Player id 0.rand: Java 206789181479531, Rust 203227649148896 (rechecked by verify_replays10) |
| verify10 | player: activate, every frame and final world | DIFFERS | comparison | byte 2350, frame 1, Player id 0.rand: Java 206789181479531, Rust 203227649148896 (rechecked by verify_replays10) |
| verify10 | player: api, every frame and final world | DIFFERS | comparison | byte 2362, frame 1, Player id 0.rand: Java 206789181479531, Rust 203227649148896 (rechecked by verify_replays10) |
| verify10 | player: pipeline, every frame and final world | DIFFERS | checker failure | Checker failed before completing comparison: chunk identity stream exhausted; thread 'main' (30392) panicked at crates\worldgen10\src\world\tick.rs:26:83: (rechecked by verify_replays10) |
| verify10 | player: night, every frame and final world | DIFFERS | oracle failure | Oracle failed before comparison: java.lang.UnsupportedOperationException: performSleepSpawning was removed in Minecraft 1.0; at net.minecraft.src.Harness10.removedSleepSpawning(Harness10.java:50) (rechecked by verify_replays10) |
| verify10 | player: sleepworld, every frame and final world | DIFFERS | checker failure | Checker failed before completing comparison: chunk identity stream exhausted; thread 'main' (36240) panicked at crates\worldgen10\src\world\tick.rs:26:83: (rechecked by verify_replays10) |
| verify10 | player: survival, every frame and final world | DIFFERS | comparison | byte 2310, frame 1, Player id 0.rand: Java 206789181479531, Rust 203227649148896 (rechecked by verify_replays10) |
| verify10 | player: bucket, every frame and final world | DIFFERS | comparison | byte 2350, frame 1, Player id 0.rand: Java 206789181479531, Rust 203227649148896 (rechecked by verify_replays10) |
| verify10 | player: combat, every frame and final world | DIFFERS | comparison | byte 2318, frame 1, Player id 0.rand: Java 206789181479531, Rust 203227649148896 (rechecked by verify_replays10) |
| verify10 | player: pickentities, every frame and final world | DIFFERS | comparison | byte 3016, frame 1, Player id 0.rand: Java 206789181479531, Rust 203227649148896 (rechecked by verify_replays10) |
| verify10 | player: digextras, every frame and final world | DIFFERS | comparison | byte 2358, frame 1, Player id 0.rand: Java 206789181479531, Rust 203227649148896 (rechecked by verify_replays10) |
| verify10 | player: slimecontact, every frame and final world | DIFFERS | comparison | byte 1202, frame 1.entityCount: Java 1, Rust 2 (rechecked by verify_replays10) |
| verify10 | player: stats, every frame and final world | DIFFERS | comparison | byte 2318, frame 1, Player id 0.rand: Java 206789181479531, Rust 203227649148896 (rechecked by verify_replays10) |
| verify10 | entities: items, every frame for 6005 ticks | DIFFERS | comparison | byte 71954, frame 16, Item id 4.fire: Java 160, Rust 300 (rechecked by verify_replays10) |
| verify10 | entities: sand, every frame for 300 ticks | EXACT | comparison | EXACT MATCH: sand entities, 300 ticks (rechecked by verify_replays10) |
| verify10 | entities: tnt, every frame for 300 ticks | EXACT | comparison | EXACT MATCH: tnt entities, 300 ticks (rechecked by verify_replays10) |
| verify10 | entities: arrows, every frame for 1300 ticks | DIFFERS | comparison | byte 36753, frame 7, event 0.name.length: Java 13, Rust 10 (rechecked by verify_replays10) |
| verify10 | entities: passive, every frame for 13000 ticks | DIFFERS | comparison | byte 932, frame 0, Sheep id 2.health: Java 8, Rust 10 (rechecked by verify_replays10) |
| verify10 | entities: aquatic, every frame for 3000 ticks | DIFFERS | comparison | byte 3600, frame 1, Squid id 0.squidPitch: Java -18.0, Rust 0.0 (rechecked by verify_replays10) |
| verify10 | entities: hostile, every frame for 3000 ticks | DIFFERS | comparison | byte 940, frame 0, Spider id 2.health: Java 16, Rust 20 (rechecked by verify_replays10) |
| verify10 | entities: special, every frame for 3000 ticks | DIFFERS | comparison | byte 3412, frame 1, Wolf id 0.yaw: Java 5.568816661834717, Rust 0.0 (rechecked by verify_replays10) |
| verify10 | entities: spawning, every frame for 3000 ticks | DIFFERS | comparison | byte 98, frame 1.entityCount: Java 0, Rust 20 (rechecked by verify_replays10) |
| verify10 | entities: spawner, every frame for 3000 ticks | DIFFERS | comparison | byte 5415, frame 21, world.rand: Java 268016145202057, Rust 203435489135477 (rechecked by verify_replays10) |
| verify10 | entities: client, every frame for 3000 ticks | DIFFERS | checker failure | Checker failed before completing comparison: chunk identity stream exhausted; thread 'main' (4080) panicked at crates\worldgen10\src\world\tick.rs:26:83: (rechecked by verify_replays10) |
| verify10 | entities: nbt, every frame for 1000 ticks | DIFFERS | comparison | byte 1459, frame 0, Sheep id 4.health: Java 8, Rust 10 (rechecked by verify_replays10) |
| verify10 | entities: nbtall, every frame for 1000 ticks | DIFFERS | comparison | byte 1991, frame 0, Sheep id 6.health: Java 8, Rust 10 (rechecked by verify_replays10) |
| verify10 | entities: slimes, every frame for 3000 ticks | DIFFERS | comparison | byte 3106, frame 1.entityCount: Java 8, Rust 24 (rechecked by verify_replays10) |
| verify10 | entities: hooks, every frame for 1000 ticks | DIFFERS | comparison | byte 10973, frame 1, events.count: Java 6, Rust 4 (rechecked by verify_replays10) |
| verify10 | entities: combat, every frame for 3000 ticks | DIFFERS | comparison | byte 3296, frame 1, Wolf id 0.yaw: Java 5.568816661834717, Rust 0.0 (rechecked by verify_replays10) |
| verify10 | entities: stormclient, every frame for 3000 ticks | DIFFERS | checker failure | Checker failed before completing comparison: chunk identity stream exhausted; thread 'main' (36468) panicked at crates\worldgen10\src\world\tick.rs:26:83: (rechecked by verify_replays10) |
| verify10 | entities: lightning, every frame for 1000 ticks | DIFFERS | comparison | byte 932, frame 0, Sheep id 2.health: Java 8, Rust 10 (rechecked by verify_replays10) |
| verify10 | entities: rider, every frame for 3000 ticks | DIFFERS | comparison | byte 216, frame 0, Spider id 0.health: Java 16, Rust 20 (rechecked by verify_replays10) |
| verify10 | entities: nether, every frame for 3000 ticks | DIFFERS | comparison | byte 3190, frame 1.entityCount: Java 8, Rust 21 (rechecked by verify_replays10) |
| verify10 | raw terrain: origin | DIFFERS | comparison | climate mismatch chunk (-8,-8) col 0: java t=5e-1 h=5e-1, rust t=2.9998779296875e-1 h=7.9998779296875e-1 |
| verify10 | raw terrain: pack.png seed | DIFFERS | comparison | climate mismatch chunk (-6,-6) col 0: java t=7.9998779296875e-1 h=8.99993896484375e-1, rust t=5e-1 h=5e-1 |
| verify10 | raw terrain: negative seed | EXACT | comparison | EXACT MATCH |
| verify10 | raw terrain: mid-range | DIFFERS | comparison | climate mismatch chunk (100,-300) col 0: java t=5e-1 h=5e-1, rust t=1.999969482421875e-1 h=2.9998779296875e-1 |
| verify10 | raw terrain: far lands x | DIFFERS | comparison | chunk (784420,0) block x=0 y=51 z=0: java=3 rust=1 |
| verify10 | raw terrain: far lands -z | DIFFERS | comparison | climate mismatch chunk (-3,-784431) col 0: java t=8.97705078125e-1 h=9.98443603515625e-1, rust t=5e-1 h=5e-1 |
| verify10 | raw terrain: max long seed | DIFFERS | comparison | climate mismatch chunk (-2,-2) col 0: java t=6.999969482421875e-1 h=7.9998779296875e-1, rust t=5e-1 h=5e-1 |
| verify10 | raw Nether: origin | EXACT | comparison | 0 metadata bytes differ |
| verify10 | raw Nether: negative seed | EXACT | comparison | 0 metadata bytes differ |
| verify10 | raw Nether: mid-range | EXACT | comparison | 0 metadata bytes differ |
| verify10 | raw Nether: far lands x | EXACT | comparison | 0 metadata bytes differ |
| verify10 | raw Nether: far lands -z | EXACT | comparison | 0 metadata bytes differ |
| verify10 | raw Nether: max long seed | EXACT | comparison | 0 metadata bytes differ |
| verify10 | Nether grid 8x8, light each | DIFFERS | comparison | skylight: (-51,0..1,-34) java=0x00 rust=0x0f |
| verify10 | Nether grid, negative seed, light each | DIFFERS | comparison | skylight: (-119,0..1,91) java=0x00 rust=0x0f |
| verify10 | Nether grid, far coordinates, light each | DIFFERS | comparison | block: (12550807,60,-27) java=112 () rust=87 (tile.hellrock) |
| verify10 | Nether grid 8x8, light end | DIFFERS | comparison | skylight: (-51,0..1,-34) java=0x00 rust=0x0f |
| verify10 | Nether grid, negative seed, light end | DIFFERS | comparison | skylight: (-119,0..1,91) java=0x00 rust=0x0f |
| verify10 | Nether grid, far coordinates, light end | DIFFERS | comparison | block: (12550807,60,-27) java=112 () rust=87 (tile.hellrock) |
| verify10 | Nether grid 8x8, light none | DIFFERS | comparison | skylight: (-51,0..1,-34) java=0x00 rust=0x0f |
| verify10 | Nether grid, negative seed, light none | DIFFERS | comparison | skylight: (-119,0..1,91) java=0x00 rust=0x0f |
| verify10 | Nether grid, far coordinates, light none | DIFFERS | comparison | block: (12550807,60,-27) java=112 () rust=87 (tile.hellrock) |
| verify10 | new world: seed 0, world rand 1 | DIFFERS | comparison | spawn differs: java (251, 64, 163) rust (56, 64, 157) |
| verify10 | new world: seed 3257840388504953787, world rand 7 | DIFFERS | comparison | spawn differs: java (100, 64, 244) rust (-244, 64, -428) |
| verify10 | new world: seed -4172144997902289642, world rand 99 | DIFFERS | comparison | spawn differs: java (-184, 64, 230) rust (184, 64, 256) |
| verify10 | new world: seed 123456789, world rand 2024 | DIFFERS | comparison | spawn differs: java (-128, 64, 244) rust (-228, 64, 256) |
| verify10 | new world: seed 404, world rand 5 | DIFFERS | comparison | spawn differs: java (147, 64, 175) rust (232, 64, 12) |
| verify10 | new world: seed -1, world rand -1 | DIFFERS | comparison | spawn differs: java (66, 64, 212) rust (-274, 64, 311) |
| verify10 | new world, Fast graphics: seed 25 | DIFFERS | comparison | spawn differs: java (-4, 64, 240) rust (-165, 64, 213) |
| verify10 | new world, Fancy graphics: seed 25 | DIFFERS | comparison | spawn differs: java (-4, 64, 240) rust (-165, 64, 213) |
| verify10 | grid 24x24, light after each chunk | DIFFERS | comparison | block: (-192,41,-192) java=1 (tile.stone) rust=3 (tile.dirt) |
| verify10 | grid 16x16, light only at the end | DIFFERS | comparison | block: (13,68,41) java=38 (tile.rose) rust=37 (tile.flower) |
| verify10 | grid 16x16, no light processing | DIFFERS | comparison | block: (-640,2,320) java=7 (tile.bedrock) rust=1 (tile.stone) |
| verify10 | ticks: 2 3 -77 3000 2000 thunder 0 0 | DIFFERS | comparison | byte 8, System.identityHashCode(p) [big endian] = 1733520096 (Mode_ticksintegration10.java:31, field starts 8, 4 bytes, Integer); Rust -1056964609 |
| verify10 | ticks: 1 7 999 100 2000 rain 40 -30 | DIFFERS | comparison | byte 8, System.identityHashCode(p) [big endian] = 154795841 (Mode_ticksintegration10.java:31, field starts 8, 4 bytes, Integer); Rust -1728053249 |
| verify10 | ticks: -1 5 4242 0 2000 rain -100 60 | DIFFERS | comparison | byte 8, System.identityHashCode(p) [big endian] = 1370494288 (Mode_ticksintegration10.java:31, field starts 8, 4 bytes, Integer); Rust -285278209 |
| verify10 | ticks: forest fire, seed 6 | DIFFERS | comparison | byte 8, System.identityHashCode(p) [big endian] = 1411675719 (Mode_ticksintegration10.java:31, field starts 8, 4 bytes, Integer); Rust -603979776 |
| verify10 | ticks: saplings growing, seed 6 | DIFFERS | comparison | byte 8, System.identityHashCode(p) [big endian] = 1411675719 (Mode_ticksintegration10.java:31, field starts 8, 4 bytes, Integer); Rust -603979776 |
| verify10 | teleporter: 3 1 0 100.5 70 -40.5 7 | DIFFERS | comparison | block: (-2,73,77) java=38 (tile.rose) rust=37 (tile.flower) |
| verify10 | teleporter: 3 1 -1 12.5 64 -5.5 9 | DIFFERS | comparison | skylight: (-115,20..21,-83) java=0x00 rust=0xf0 |
| verify10 | teleporter: 3 1 0 1000.5 70 1000.5 1 | DIFFERS | comparison | block: (864,45,864) java=3 (tile.dirt) rust=1 (tile.stone) |
| verify10 | teleporter: 5 2 -1 200.5 100 -33.5 5 | DIFFERS | comparison | skylight: (74,42..43,-146) java=0x00 rust=0xf0 |
| verify10 | use portal: 3 1 100.5 70 -40.5 11 7 | DIFFERS | comparison | spawn differs: java (157, 64, 106) rust (160, 64, 115) |
| verify10 | use portal: 404 5 -17.5 66 0.5 12 8 | DIFFERS | comparison | spawn differs: java (147, 64, 175) rust (232, 64, 12) |
| verify10 | respawn: 3 1 plain | DIFFERS | comparison | spawn differs: java (157, 64, 106) rust (160, 64, 115) |
| verify10 | respawn: 3 1 bed | DIFFERS | comparison | spawn differs: java (157, 64, 106) rust (160, 64, 115) |
| verify10 | respawn: 404 5 nobed | DIFFERS | comparison | spawn differs: java (147, 64, 175) rust (232, 64, 12) |
| verify_alphafix10 | gui10: b10_gui9_mouse3_retained alpha | EXACT | comparison | b10_gui9_mouse3_retained: ALPHA EXACT |
| verify_alphafix10 | gui10: b10_creative_to_inventory alpha | EXACT | comparison | b10_creative_to_inventory: ALPHA EXACT |
| verify_alphafix10 | gui10: inventory_signed_counts alpha | EXACT | comparison | inventory_signed_counts: ALPHA EXACT |
| verify_constantfix10 | player-gui round 10: statfile10_181 | EXACT | comparison | STATFILE10: 3050 original schema/grammar/checksum/hash-order cases, 1000 byte-exact serializations, 0 differences |
| verify_entities10 | entities10: rules | DIFFERS | comparison | byte 117, s.unblockable() = true (Mode_entities10.java:48, field starts 117, 1 bytes, Boolean); Rust False |
| verify_entities10 | entities10: indirect | DIFFERS | comparison | byte 181, a.health = 40 (Mode_entities10.java:30, field starts 181, 4 bytes, Integer); Rust 20 |
| verify_entities10 | entities10: stare | DIFFERS | comparison | byte 181, a.health = 40 (Mode_entities10.java:30, field starts 181, 4 bytes, Integer); Rust 20 |
| verify_entities10 | entities10: starepumpkin | DIFFERS | comparison | byte 181, a.health = 40 (Mode_entities10.java:30, field starts 181, 4 bytes, Integer); Rust 20 |
| verify_entities10 | entities10: population | DIFFERS | comparison | byte 40680, EntityOracle.rngState(random) = 25214908013 (Mode_entities10.java:66, field starts 40680, 8 bytes, Long); Rust 47669638442956 |
| verify_entities10 | entities10: carry | DIFFERS | comparison | byte 181, a.health = 40 (Mode_entities10.java:30, field starts 181, 4 bytes, Integer); Rust 20 |
| verify_entities10 | entities10: allies | DIFFERS | comparison | byte 181, a.health = 6 (Mode_entities10.java:30, field starts 181, 4 bytes, Integer); Rust 18 |
| verify_entities10 | entities10: attract | DIFFERS | comparison | byte 27, e.motionX = 0.09842009842395782 (Mode_entities10.java:24, field starts 24, 8 bytes, Double); Rust 0.08030473440885544 |
| verify_entities10 | entities10: Boat | EXACT | comparison | EXACT: entities181 Boat (21513 bytes) |
| verify_entities10 | entities10: Minecart | EXACT | comparison | EXACT: entities181 Minecart (21513 bytes) |
| verify_entities10 | entities10: Item | EXACT | comparison | EXACT: entities181 Item (21513 bytes) |
| verify_entities10 | entities10: Lightning | EXACT | comparison | EXACT: entities181 Lightning (4584 bytes) |
| verify_entities10 | entities10: Snowball | EXACT | comparison | EXACT: entities181 Snowball (7643 bytes) |
| verify_entities10 | entities10: Egg | EXACT | comparison | EXACT: entities181 Egg (7643 bytes) |
| verify_entities10 | entities10: Fireball | DIFFERS | comparison | byte 381, Harness10.getInt(e,"fire") = 20 (Mode_entities10.java:26, field starts 381, 4 bytes, Integer); Rust 10 |
| verify_entities10 | entities10: Enderman | DIFFERS | comparison | byte 181, a.health = 40 (Mode_entities10.java:30, field starts 181, 4 bytes, Integer); Rust 20 |
| verify_entities10 | entities10: teleport | DIFFERS | comparison | byte 181, a.health = 40 (Mode_entities10.java:30, field starts 181, 4 bytes, Integer); Rust 20 |
| verify_entities10 | entities10: CaveSpider | DIFFERS | comparison | byte 181, a.health = 12 (Mode_entities10.java:30, field starts 181, 4 bytes, Integer); Rust 20 |
| verify_entities10 | entities10: Silverfish | DIFFERS | comparison | byte 181, a.health = 8 (Mode_entities10.java:30, field starts 181, 4 bytes, Integer); Rust 20 |
| verify_entities10 | entities10: XPOrb | DIFFERS | comparison | byte 27, e.motionX = 0.09842009842395782 (Mode_entities10.java:24, field starts 24, 8 bytes, Double); Rust 0.08030473440885544 |
| verify_entities10 | entities10: Arrow | DIFFERS | comparison | byte 8995, sound.name: Java 'random.bowhit', Rust 'random.drr' |
| verify_entities10 | entities10: Pig | DIFFERS | comparison | byte 442, e.rotationYaw = 40.568817 (Mode_entities10.java:25, field starts 442, 4 bytes, Float); Rust 35.0 |
| verify_entities10 | entities10: Cow | DIFFERS | comparison | byte 442, e.rotationYaw = 40.568817 (Mode_entities10.java:25, field starts 442, 4 bytes, Float); Rust 35.0 |
| verify_entities10 | entities10: Chicken | DIFFERS | comparison | byte 442, e.rotationYaw = 35.0 (Mode_entities10.java:25, field starts 442, 4 bytes, Float); Rust 40.568817138671875 |
| verify_entities10 | entities10: Sheep | DIFFERS | comparison | byte 181, a.health = 8 (Mode_entities10.java:30, field starts 181, 4 bytes, Integer); Rust 10 |
| verify_entities10 | entities10: Wolf | DIFFERS | comparison | byte 442, e.rotationYaw = 40.568817 (Mode_entities10.java:25, field starts 442, 4 bytes, Float); Rust 35.0 |
| verify_entities10 | entities10: Zombie | DIFFERS | comparison | byte 442, e.rotationYaw = 40.568817 (Mode_entities10.java:25, field starts 442, 4 bytes, Float); Rust 35.0 |
| verify_entities10 | entities10: Skeleton | DIFFERS | comparison | byte 442, e.rotationYaw = 40.568817 (Mode_entities10.java:25, field starts 442, 4 bytes, Float); Rust 35.0 |
| verify_entities10 | entities10: Spider | DIFFERS | comparison | byte 181, a.health = 16 (Mode_entities10.java:30, field starts 181, 4 bytes, Integer); Rust 20 |
| verify_entities10 | entities10: Creeper | DIFFERS | comparison | byte 442, e.rotationYaw = 40.568817 (Mode_entities10.java:25, field starts 442, 4 bytes, Float); Rust 35.0 |
| verify_entities10 | entities10: PigZombie | DIFFERS | comparison | byte 442, e.rotationYaw = 40.568817 (Mode_entities10.java:25, field starts 442, 4 bytes, Float); Rust 35.0 |
| verify_entities10 | entities10: Ghast | DIFFERS | comparison | byte 314, e.posX = -3.494552817668196 (Mode_entities10.java:24, field starts 314, 8 bytes, Double); Rust -3.5 |
| verify_entities10 | entities10: Squid | DIFFERS | comparison | byte 531, a.entityAge = 1 (Mode_entities10.java:30, field starts 531, 4 bytes, Integer); Rust 0 |
| verify_entities10 | entities10: Slime | DIFFERS | comparison | byte 596, EntityOracle.rngState(e.rand) = 201480731191965 (Mode_entities10.java:41, field starts 596, 8 bytes, Long); Rust 54880878584218 |
| verify_entities10 | entities10: potions | DIFFERS | comparison | byte 396, e.posX = -3.5 (Mode_entities10.java:24, field starts 394, 8 bytes, Double); Rust -3.5011954854708165 |
| verify_entities10 | entities10: damage | DIFFERS | comparison | byte 324, e.posX = -3.5124340476468205 (Mode_entities10.java:24, field starts 322, 8 bytes, Double); Rust -3.5011954854708165 |
| verify_entities10 | entities10: poison | DIFFERS | comparison | byte 181, a.health = 19 (Mode_entities10.java:30, field starts 181, 4 bytes, Integer); Rust 18 |
| verify_entities10 | entities10: kill-Cow | DIFFERS | comparison | byte 612, EntityOracle.rngState(e.rand) = 85306545087601 (Mode_entities10.java:41, field starts 612, 8 bytes, Long); Rust 187788727644900 |
| verify_entities10 | entities10: kill-Chicken | DIFFERS | comparison | byte 612, EntityOracle.rngState(e.rand) = 219111625387582 (Mode_entities10.java:41, field starts 612, 8 bytes, Long); Rust 187788727644900 |
| verify_entities10 | entities10: kill-Sheep | DIFFERS | comparison | byte 181, a.health = 8 (Mode_entities10.java:30, field starts 181, 4 bytes, Integer); Rust 10 |
| verify_entities10 | entities10: kill-Zombie | DIFFERS | comparison | byte 503, a.health = -900 (Mode_entities10.java:30, field starts 503, 4 bytes, Integer); Rust -980 |
| verify_entities10 | entities10: kill-Enderman | DIFFERS | comparison | byte 181, a.health = 40 (Mode_entities10.java:30, field starts 181, 4 bytes, Integer); Rust 20 |
| verify_entities10 | entities10: entitygeometry | EXACT | comparison | EXACT MATCH: 10000 geometry cases |
| verify_entities10 | entities10: models10 | DIFFERS | comparison | case 56: part count: Rust 6, Java 7 |
| verify_entities10 | entities10: particles10 | EXACT | comparison | particle checks: 184 cases, 594688 vertices, 0 differing fields |
| verify_entities10 | entities10: xprender10 | DIFFERS | comparison with oracle failure | byte 612, EntityOracle.rngState(e.rand) = 187788727644900 (Mode_entities10.java:41, field starts 612, 8 bytes, Long); Rust 201480731191965; another mode in this section failed: java.lang.UnsupportedOperationException: ModelBase random-part selection was removed in Minecraft 1.0; at net.minecraft.src.Harness10.removedRandomModel(Harness10.java:57) |
| verify_gui10 | gui10: oracle capture and HUD brightness hints | EXACT | comparison | HUD BRIGHTNESS HINTS EXACT (33 scenes) |
| verify_gui10 | gui10: mojang | EXACT | comparison | mojang: EXACT |
| verify_gui10 | gui10: title | DIFFERS | comparison | pixel (0,0).RGB: Java (214, 219, 236), Rust (227, 231, 242) |
| verify_gui10 | gui10: title_hover_singleplayer | DIFFERS | comparison | pixel (0,0).RGB: Java (214, 219, 236), Rust (227, 231, 242) |
| verify_gui10 | gui10: title_hover_options | DIFFERS | comparison | pixel (0,0).RGB: Java (214, 219, 236), Rust (227, 231, 242) |
| verify_gui10 | gui10: options | EXACT | comparison | options: EXACT |
| verify_gui10 | gui10: video | DIFFERS | comparison | pixel (436,272).RGB: Java (0, 0, 0), Rust (38, 27, 19) |
| verify_gui10 | gui10: controls | EXACT | comparison | controls: EXACT |
| verify_gui10 | gui10: select | DIFFERS | comparison | pixel (452,96).RGB: Java (128, 128, 128), Rust (15, 11, 7) |
| verify_gui10 | gui10: select_pressed | DIFFERS | comparison | pixel (452,96).RGB: Java (128, 128, 128), Rust (0, 0, 0) |
| verify_gui10 | gui10: select_clicked | DIFFERS | comparison | pixel (452,96).RGB: Java (128, 128, 128), Rust (0, 0, 0) |
| verify_gui10 | gui10: select_hover_rename | DIFFERS | comparison | pixel (452,96).RGB: Java (128, 128, 128), Rust (0, 0, 0) |
| verify_gui10 | gui10: create | EXACT | comparison | create: EXACT |
| verify_gui10 | gui10: create_typed | EXACT | comparison | create_typed: EXACT |
| verify_gui10 | gui10: rename | EXACT | comparison | rename: EXACT |
| verify_gui10 | gui10: texturepacks | EXACT | comparison | texturepacks: EXACT |
| verify_gui10 | gui10: loading | EXACT | comparison | loading: EXACT |
| verify_gui10 | gui10: loading_nobar | EXACT | comparison | loading_nobar: EXACT |
| verify_gui10 | gui10: hud_full | DIFFERS | comparison | pixel (4,4).RGB: Java (25, 25, 25), Rust (255, 255, 255) |
| verify_gui10 | gui10: hud_hurt | DIFFERS | comparison | pixel (4,4).RGB: Java (26, 26, 26), Rust (255, 255, 255) |
| verify_gui10 | gui10: hud_low_flash | DIFFERS | comparison | pixel (4,4).RGB: Java (26, 26, 26), Rust (255, 255, 255) |
| verify_gui10 | gui10: hud_flash | DIFFERS | comparison | pixel (4,4).RGB: Java (26, 26, 26), Rust (255, 255, 255) |
| verify_gui10 | gui10: hud_pumpkin | DIFFERS | comparison | pixel (4,4).RGB: Java (0, 0, 0), Rust (255, 255, 255) |
| verify_gui10 | gui10: achievements | DIFFERS | comparison | pixel (579,171).RGB: Java (24, 19, 12), Rust (22, 18, 11) |
| verify_gui10 | gui10: achievements_hover_locked | DIFFERS | comparison | pixel (579,171).RGB: Java (24, 19, 12), Rust (22, 18, 11) |
| verify_gui10 | gui10: achievements_hover_open | DIFFERS | comparison | pixel (579,171).RGB: Java (24, 19, 12), Rust (22, 18, 11) |
| verify_gui10 | gui10: gameover | DIFFERS | comparison | pixel (320,120).RGB: Java (112, 77, 77), Rust (255, 255, 255) |
| verify_gui10 | gui10: gameover_hover_respawn | DIFFERS | comparison | pixel (320,120).RGB: Java (112, 77, 77), Rust (255, 255, 255) |
| verify_gui10 | gui10: stats_general | EXACT | comparison | stats_general: EXACT |
| verify_gui10 | gui10: stats_blocks | DIFFERS | comparison | pixel (296,144).RGB: Java (81, 81, 81), Rust (51, 51, 51) |
| verify_gui10 | gui10: stats_items | DIFFERS | comparison | pixel (430,222).RGB: Java (144, 144, 144), Rust (15, 11, 7) |
| verify_gui10 | gui10: stats_blocks_hover_header | DIFFERS | comparison | pixel (296,144).RGB: Java (81, 81, 81), Rust (51, 51, 51) |
| verify_gui10 | gui10: stats_blocks_sort_down | DIFFERS | comparison | pixel (296,144).RGB: Java (81, 81, 81), Rust (51, 51, 51) |
| verify_gui10 | gui10: stats_blocks_sorted | DIFFERS | comparison | pixel (296,144).RGB: Java (81, 81, 81), Rust (51, 51, 51) |
| verify_gui10 | gui10: stats_blocks_hover_item | DIFFERS | comparison | pixel (296,144).RGB: Java (81, 81, 81), Rust (51, 51, 51) |
| verify_gui10 | gui10: stats_items_long | DIFFERS | comparison | pixel (430,322).RGB: Java (144, 144, 144), Rust (15, 11, 7) |
| verify_gui10 | gui10: stats_items_sort_used | DIFFERS | comparison | pixel (320,316).RGB: Java (139, 139, 139), Rust (68, 68, 68) |
| verify_gui10 | gui10: stats_items_sorted | DIFFERS | comparison | pixel (320,316).RGB: Java (139, 139, 139), Rust (68, 68, 68) |
| verify_gui10 | gui10: stats_items_hover_item | DIFFERS | comparison | pixel (320,316).RGB: Java (139, 139, 139), Rust (68, 68, 68) |
| verify_gui10 | gui10: stats_items_drag_start | DIFFERS | comparison | pixel (320,316).RGB: Java (139, 139, 139), Rust (68, 68, 68) |
| verify_gui10 | gui10: stats_items_drag_mid | DIFFERS | comparison | pixel (294,64).RGB: Java (4, 4, 4), Rust (9, 9, 9) |
| verify_gui10 | gui10: stats_items_scrolled | DIFFERS | comparison | pixel (294,64).RGB: Java (4, 4, 4), Rust (9, 9, 9) |
| verify_gui10 | gui10: b10_hud_full | DIFFERS | comparison | pixel (4,4).RGB: Java (26, 26, 26), Rust (255, 255, 255) |
| verify_gui10 | gui10: b10_hud_low_hunger_xp | DIFFERS | comparison | pixel (4,4).RGB: Java (26, 26, 26), Rust (255, 255, 255) |
| verify_gui10 | gui10: b10_hud_starving | DIFFERS | comparison | pixel (4,4).RGB: Java (26, 26, 26), Rust (255, 255, 255) |
| verify_gui10 | gui10: b10_hud_creative | DIFFERS | comparison | pixel (4,4).RGB: Java (26, 26, 26), Rust (255, 255, 255) |
| verify_gui10 | gui10: b10_create_survival | EXACT | comparison | b10_create_survival: EXACT |
| verify_gui10 | gui10: b10_create_creative | DIFFERS | comparison | pixel (322,212).RGB: Java (224, 224, 224), Rust (113, 113, 113) |
| verify_gui10 | gui10: b10_create_more | EXACT | comparison | b10_create_more: EXACT |
| verify_gui10 | gui10: b10_create_features_off | EXACT | comparison | b10_create_features_off: EXACT |
| verify_gui10 | gui10: b10_creative_first | DIFFERS | comparison | pixel (268,76).RGB: Java (79, 79, 79), Rust (93, 93, 93) |
| verify_gui10 | gui10: b10_creative_scroll | DIFFERS | comparison | pixel (304,76).RGB: Java (10, 10, 15), Rust (12, 12, 18) |
| verify_gui10 | gui10: b10_creative_cursor | DIFFERS | comparison | pixel (304,76).RGB: Java (10, 10, 15), Rust (12, 12, 18) |
| verify_gui10 | gui10: b10_options | EXACT | comparison | b10_options: EXACT |
| verify_gui10 | gui10: b10_options_fov | EXACT | comparison | b10_options_fov: EXACT |
| verify_gui10 | gui10: b10_video_gamma | DIFFERS | comparison | pixel (436,272).RGB: Java (0, 0, 0), Rust (40, 40, 40) |
| verify_gui10 | gui10: b10_controls | EXACT | comparison | b10_controls: EXACT |
| verify_gui10 | gui10: b10_controls_waiting | EXACT | comparison | b10_controls_waiting: EXACT |
| verify_gui10 | gui10: b10_controls_duplicate | EXACT | comparison | b10_controls_duplicate: EXACT |
| verify_gui10 | gui10: b10_inventory | DIFFERS | comparison | pixel (355,113).RGB: Java (40, 27, 12), Rust (38, 26, 10) |
| verify_gui10 | gui10: b10_chest | EXACT | comparison | b10_chest: EXACT |
| verify_gui10 | gui10: b10_largechest | EXACT | comparison | b10_largechest: EXACT |
| verify_gui10 | gui10: b10_crafting | EXACT | comparison | b10_crafting: EXACT |
| verify_gui10 | gui10: b10_furnace | EXACT | comparison | b10_furnace: EXACT |
| verify_gui10 | gui10: b10_dispenser | EXACT | comparison | b10_dispenser: EXACT |
| verify_gui10 | gui10: b10_hud_potions | DIFFERS | comparison | pixel (4,4).RGB: Java (26, 26, 26), Rust (255, 255, 255) |
| verify_gui10 | gui10: b10_hud_regen_shift | DIFFERS | comparison | pixel (4,4).RGB: Java (26, 26, 26), Rust (255, 255, 255) |
| verify_gui10 | gui10: b10_hud_air_full | DIFFERS | comparison | pixel (4,4).RGB: Java (26, 26, 26), Rust (255, 255, 255) |
| verify_gui10 | gui10: b10_hud_air_half | DIFFERS | comparison | pixel (4,4).RGB: Java (26, 26, 26), Rust (255, 255, 255) |
| verify_gui10 | gui10: b10_hud_air_popping | DIFFERS | comparison | pixel (4,4).RGB: Java (26, 26, 26), Rust (255, 255, 255) |
| verify_gui10 | gui10: b10_hud_air_empty | DIFFERS | comparison | pixel (4,4).RGB: Java (26, 26, 26), Rust (255, 255, 255) |
| verify_gui10 | gui10: b10_hud_air_creative | DIFFERS | comparison | pixel (4,4).RGB: Java (26, 26, 26), Rust (255, 255, 255) |
| verify_gui10 | gui10: b10_toast_start | EXACT | comparison | b10_toast_start: EXACT |
| verify_gui10 | gui10: b10_toast_enter | EXACT | comparison | b10_toast_enter: EXACT |
| verify_gui10 | gui10: b10_toast_visible | EXACT | comparison | b10_toast_visible: EXACT |
| verify_gui10 | gui10: b10_toast_middle | EXACT | comparison | b10_toast_middle: EXACT |
| verify_gui10 | gui10: b10_toast_exit | EXACT | comparison | b10_toast_exit: EXACT |
| verify_gui10 | gui10: b10_toast_end | EXACT | comparison | b10_toast_end: EXACT |
| verify_gui10 | gui10: b10_toast_expired | EXACT | comparison | b10_toast_expired: EXACT |
| verify_gui10 | gui10: b10_toast_no_revival | EXACT | comparison | b10_toast_no_revival: EXACT |
| verify_gui10 | gui10: b10_toast_zero_clock | EXACT | comparison | b10_toast_zero_clock: EXACT |
| verify_gui10 | gui10: b10_toast_snipe | EXACT | comparison | b10_toast_snipe: EXACT |
| verify_gui10 | gui10: b10_toast_hint | EXACT | comparison | b10_toast_hint: EXACT |
| verify_gui10 | gui10: b10_toast_hint_late | EXACT | comparison | b10_toast_hint_late: EXACT |
| verify_gui10 | gui10: b10_toast_snipe_hint | EXACT | comparison | b10_toast_snipe_hint: EXACT |
| verify_gui10 | gui10: b10_achievement_inventory_close | DIFFERS | comparison | pixel (4,4).RGB: Java (26, 26, 26), Rust (255, 255, 255) |
| verify_gui10 | gui10: b10_create_signed_seed | EXACT | comparison | b10_create_signed_seed: EXACT |
| verify_gui10 | gui10: b10_create_signed_seed_return | DIFFERS | comparison | pixel (322,212).RGB: Java (224, 224, 224), Rust (113, 113, 113) |
| verify_gui10 | gui10: b10_creative_inventory_close | DIFFERS | comparison | pixel (4,4).RGB: Java (26, 26, 26), Rust (255, 255, 255) |
| verify_gui10 | gui10: b10_create_empty_name | EXACT | comparison | b10_create_empty_name: EXACT |
| verify_gui10 | gui10: b10_create_spaces_name | EXACT | comparison | b10_create_spaces_name: EXACT |
| verify_gui10 | gui10: b10_create_folder_collision | EXACT | comparison | b10_create_folder_collision: EXACT |
| verify_gui10 | gui10: b10_create_tab_seed | EXACT | comparison | b10_create_tab_seed: EXACT |
| verify_gui10 | gui10: b10_rename_spaces_disabled | EXACT | comparison | b10_rename_spaces_disabled: EXACT |
| verify_gui10 | gui10: b10_delete_confirmation | DIFFERS | comparison | pixel (452,96).RGB: Java (128, 128, 128), Rust (15, 11, 7) |
| verify_gui10 | gui10: b10_delete_cancel | DIFFERS | comparison | pixel (452,96).RGB: Java (128, 128, 128), Rust (15, 11, 7) |
| verify_gui10 | gui10: b10_hud5_portal_full | DIFFERS | comparison | pixel (4,4).RGB: Java (0, 42, 255), Rust (255, 255, 255) |
| verify_gui10 | gui10: b10_hud5_portal_partial | DIFFERS | comparison | pixel (4,4).RGB: Java (19, 30, 83), Rust (255, 255, 255) |
| verify_gui10 | gui10: b10_hud5_portal_nausea | DIFFERS | comparison | pixel (4,4).RGB: Java (26, 26, 26), Rust (255, 255, 255) |
| verify_gui10 | gui10: b10_hud5_pumpkin_creative | DIFFERS | comparison | pixel (4,4).RGB: Java (0, 0, 0), Rust (255, 255, 255) |
| verify_gui10 | gui10: b10_hud5_sleep_enter | DIFFERS | comparison | pixel (4,4).RGB: Java (26, 26, 26), Rust (255, 255, 255) |
| verify_gui10 | gui10: b10_hud5_sleep_full | DIFFERS | comparison | pixel (4,4).RGB: Java (17, 17, 31), Rust (255, 255, 255) |
| verify_gui10 | gui10: b10_hud5_sleep_exit | DIFFERS | comparison | pixel (4,4).RGB: Java (22, 22, 29), Rust (255, 255, 255) |
| verify_gui10 | gui10: b10_hud5_sleep_end | DIFFERS | comparison | pixel (4,4).RGB: Java (26, 26, 26), Rust (255, 255, 255) |
| verify_gui10 | gui10: b10_hud5_xp_overflow | DIFFERS | comparison | pixel (4,4).RGB: Java (26, 26, 26), Rust (255, 255, 255) |
| verify_gui10 | gui10: b10_hud5_xp_cap_overflow | DIFFERS | comparison | pixel (4,4).RGB: Java (26, 26, 26), Rust (255, 255, 255) |
| verify_gui10 | gui10: b10_lightmap0_inventory | DIFFERS | comparison | pixel (355,113).RGB: Java (40, 27, 12), Rust (38, 26, 10) |
| verify_gui10 | gui10: b10_lightmap0_items | DIFFERS | comparison | pixel (353,114).RGB: Java (41, 28, 12), Rust (42, 28, 13) |
| verify_gui10 | gui10: b10_lightmap0_hurt | DIFFERS | comparison | pixel (353,114).RGB: Java (121, 17, 7), Rust (122, 17, 8) |
| verify_gui10 | gui10: b10_lightmap0_chest | DIFFERS | comparison | pixel (268,116).RGB: Java (95, 26, 23), Rust (111, 30, 27) |
| verify_gui10 | gui10: b10_lightmap1_inventory | DIFFERS | comparison | pixel (355,113).RGB: Java (40, 27, 12), Rust (38, 26, 10) |
| verify_gui10 | gui10: b10_lightmap1_items | DIFFERS | comparison | pixel (353,114).RGB: Java (41, 28, 12), Rust (42, 28, 13) |
| verify_gui10 | gui10: b10_lightmap1_hurt | DIFFERS | comparison | pixel (353,114).RGB: Java (121, 17, 7), Rust (122, 17, 8) |
| verify_gui10 | gui10: b10_lightmap1_chest | DIFFERS | comparison | pixel (268,116).RGB: Java (95, 26, 23), Rust (111, 30, 27) |
| verify_gui10 | gui10: b10_lightmap2_inventory | DIFFERS | comparison | pixel (355,113).RGB: Java (40, 27, 12), Rust (38, 26, 10) |
| verify_gui10 | gui10: b10_lightmap2_items | DIFFERS | comparison | pixel (353,114).RGB: Java (41, 28, 12), Rust (42, 28, 13) |
| verify_gui10 | gui10: b10_lightmap2_hurt | DIFFERS | comparison | pixel (353,114).RGB: Java (121, 17, 7), Rust (122, 17, 8) |
| verify_gui10 | gui10: b10_lightmap2_chest | DIFFERS | comparison | pixel (268,116).RGB: Java (95, 26, 23), Rust (111, 30, 27) |
| verify_gui10 | gui10: b10_lightmap3_inventory | DIFFERS | comparison | pixel (355,113).RGB: Java (40, 27, 12), Rust (38, 26, 10) |
| verify_gui10 | gui10: b10_lightmap3_items | DIFFERS | comparison | pixel (353,114).RGB: Java (41, 28, 12), Rust (42, 28, 13) |
| verify_gui10 | gui10: b10_lightmap3_hurt | DIFFERS | comparison | pixel (353,114).RGB: Java (121, 17, 7), Rust (122, 17, 8) |
| verify_gui10 | gui10: b10_lightmap3_chest | DIFFERS | comparison | pixel (268,116).RGB: Java (95, 26, 23), Rust (111, 30, 27) |
| verify_gui10 | gui10: b10_previewlight0 | DIFFERS | comparison | pixel (355,113).RGB: Java (24, 16, 7), Rust (23, 16, 6) |
| verify_gui10 | gui10: b10_previewlight1 | DIFFERS | comparison | pixel (355,113).RGB: Java (26, 16, 7), Rust (24, 16, 6) |
| verify_gui10 | gui10: b10_previewlight2 | DIFFERS | comparison | pixel (355,113).RGB: Java (28, 16, 7), Rust (26, 16, 6) |
| verify_gui10 | gui10: b10_previewlight3 | DIFFERS | comparison | pixel (355,113).RGB: Java (30, 16, 7), Rust (28, 16, 6) |
| verify_gui10 | gui10: b10_previewlight4 | DIFFERS | comparison | pixel (355,113).RGB: Java (32, 16, 7), Rust (31, 16, 6) |
| verify_gui10 | gui10: b10_previewlight5 | DIFFERS | comparison | pixel (355,113).RGB: Java (35, 16, 7), Rust (34, 16, 6) |
| verify_gui10 | gui10: b10_previewlight6 | DIFFERS | comparison | pixel (355,113).RGB: Java (38, 16, 7), Rust (36, 16, 6) |
| verify_gui10 | gui10: b10_previewlight7 | DIFFERS | comparison | pixel (355,113).RGB: Java (41, 16, 7), Rust (40, 16, 6) |
| verify_gui10 | gui10: b10_previewlight8 | DIFFERS | comparison | pixel (355,113).RGB: Java (46, 16, 7), Rust (44, 16, 6) |
| verify_gui10 | gui10: b10_previewlight9 | DIFFERS | comparison | pixel (355,113).RGB: Java (50, 16, 7), Rust (49, 16, 6) |
| verify_gui10 | gui10: b10_previewlight10 | DIFFERS | comparison | pixel (355,113).RGB: Java (56, 16, 7), Rust (55, 16, 6) |
| verify_gui10 | gui10: b10_previewlight11 | DIFFERS | comparison | pixel (355,113).RGB: Java (63, 16, 7), Rust (62, 16, 6) |
| verify_gui10 | gui10: b10_previewlight12 | DIFFERS | comparison | pixel (355,113).RGB: Java (72, 16, 7), Rust (71, 16, 6) |
| verify_gui10 | gui10: b10_previewlight13 | DIFFERS | comparison | pixel (355,113).RGB: Java (84, 16, 7), Rust (82, 16, 6) |
| verify_gui10 | gui10: b10_previewlight14 | DIFFERS | comparison | pixel (355,113).RGB: Java (99, 16, 7), Rust (98, 16, 6) |
| verify_gui10 | gui10: b10_previewlight15 | DIFFERS | comparison | pixel (355,113).RGB: Java (120, 16, 7), Rust (119, 16, 6) |
| verify_gui10 | gui10: b10_inventory_to_creative | DIFFERS | comparison | pixel (268,76).RGB: Java (79, 79, 79), Rust (93, 93, 93) |
| verify_gui10 | gui10: b10_creative_to_inventory | DIFFERS | comparison | pixel (355,113).RGB: Java (40, 27, 12), Rust (38, 26, 10) |
| verify_gui10 | gui10: b10_gui8_select | DIFFERS | comparison | pixel (452,96).RGB: Java (128, 128, 128), Rust (15, 11, 7) |
| verify_gui10 | gui10: b10_gui8_selected | DIFFERS | comparison | pixel (452,96).RGB: Java (128, 128, 128), Rust (15, 11, 7) |
| verify_gui10 | gui10: b10_gui8_yes_hover | DIFFERS | comparison | pixel (452,96).RGB: Java (128, 128, 128), Rust (15, 11, 7) |
| verify_gui10 | gui10: b10_gui8_no_hover | DIFFERS | comparison | pixel (452,96).RGB: Java (128, 128, 128), Rust (15, 11, 7) |
| verify_gui10 | gui10: b10_gui8_cancel_return | DIFFERS | comparison | pixel (452,96).RGB: Java (128, 128, 128), Rust (15, 11, 7) |
| verify_gui10 | gui10: b10_gui8_rename | EXACT | comparison | b10_gui8_rename: EXACT |
| verify_gui10 | gui10: b10_gui8_rename_cursor | EXACT | comparison | b10_gui8_rename_cursor: EXACT |
| verify_gui10 | gui10: b10_gui8_rename_unfocused | EXACT | comparison | b10_gui8_rename_unfocused: EXACT |
| verify_gui10 | gui10: b10_gui8_rename_empty | EXACT | comparison | b10_gui8_rename_empty: EXACT |
| verify_gui10 | gui10: b10_gui8_rename_spaces | EXACT | comparison | b10_gui8_rename_spaces: EXACT |
| verify_gui10 | gui10: b10_gui8_renamed_list | DIFFERS | comparison | pixel (452,96).RGB: Java (128, 128, 128), Rust (15, 11, 7) |
| verify_gui10 | gui10: b10_gui8_deleted_list | DIFFERS | comparison | pixel (452,96).RGB: Java (128, 128, 128), Rust (15, 11, 7) |
| verify_gui10 | gui10: b10_gui8_stats_blocks | EXACT | comparison | b10_gui8_stats_blocks: EXACT |
| verify_gui10 | gui10: b10_gui8_stats_blocks_header | EXACT | comparison | b10_gui8_stats_blocks_header: EXACT |
| verify_gui10 | gui10: b10_gui8_stats_blocks_press | EXACT | comparison | b10_gui8_stats_blocks_press: EXACT |
| verify_gui10 | gui10: b10_gui8_stats_blocks_sorted | EXACT | comparison | b10_gui8_stats_blocks_sorted: EXACT |
| verify_gui10 | gui10: b10_gui8_stats_blocks_tooltip | EXACT | comparison | b10_gui8_stats_blocks_tooltip: EXACT |
| verify_gui10 | gui10: b10_gui8_stats_items | EXACT | comparison | b10_gui8_stats_items: EXACT |
| verify_gui10 | gui10: b10_gui8_stats_items_header | EXACT | comparison | b10_gui8_stats_items_header: EXACT |
| verify_gui10 | gui10: b10_gui8_stats_items_press | EXACT | comparison | b10_gui8_stats_items_press: EXACT |
| verify_gui10 | gui10: b10_gui8_stats_items_sorted | EXACT | comparison | b10_gui8_stats_items_sorted: EXACT |
| verify_gui10 | gui10: b10_gui8_stats_items_reverse_press | EXACT | comparison | b10_gui8_stats_items_reverse_press: EXACT |
| verify_gui10 | gui10: b10_gui8_stats_items_reverse | EXACT | comparison | b10_gui8_stats_items_reverse: EXACT |
| verify_gui10 | gui10: b10_gui8_stats_items_clear_press | EXACT | comparison | b10_gui8_stats_items_clear_press: EXACT |
| verify_gui10 | gui10: b10_gui8_stats_items_clear | EXACT | comparison | b10_gui8_stats_items_clear: EXACT |
| verify_gui10 | gui10: b10_gui8_stats_items_tooltip | EXACT | comparison | b10_gui8_stats_items_tooltip: EXACT |
| verify_gui10 | gui10: b10_gui9_death1_light0 | DIFFERS | comparison | pixel (384,117).RGB: Java (28, 19, 8), Rust (26, 18, 8) |
| verify_gui10 | gui10: b10_gui9_death1_light7 | DIFFERS | comparison | pixel (384,117).RGB: Java (47, 19, 8), Rust (45, 18, 8) |
| verify_gui10 | gui10: b10_gui9_death1_light15 | DIFFERS | comparison | pixel (384,117).RGB: Java (130, 19, 8), Rust (128, 18, 8) |
| verify_gui10 | gui10: b10_gui9_death2_light0 | DIFFERS | comparison | pixel (405,124).RGB: Java (28, 19, 9), Rust (26, 18, 8) |
| verify_gui10 | gui10: b10_gui9_death2_light7 | DIFFERS | comparison | pixel (405,124).RGB: Java (48, 19, 9), Rust (46, 18, 8) |
| verify_gui10 | gui10: b10_gui9_death2_light15 | DIFFERS | comparison | pixel (405,124).RGB: Java (130, 19, 9), Rust (128, 18, 8) |
| verify_gui10 | gui10: b10_gui9_death5_light0 | DIFFERS | comparison | pixel (430,142).RGB: Java (28, 19, 8), Rust (26, 18, 8) |
| verify_gui10 | gui10: b10_gui9_death5_light7 | DIFFERS | comparison | pixel (430,142).RGB: Java (49, 19, 8), Rust (47, 18, 8) |
| verify_gui10 | gui10: b10_gui9_death5_light15 | DIFFERS | comparison | pixel (430,142).RGB: Java (130, 19, 8), Rust (128, 18, 8) |
| verify_gui10 | gui10: b10_gui9_death9_light0 | DIFFERS | comparison | pixel (413,168).RGB: Java (0, 79, 79), Rust (0, 87, 87) |
| verify_gui10 | gui10: b10_gui9_death9_light7 | DIFFERS | comparison | pixel (413,168).RGB: Java (15, 79, 79), Rust (15, 87, 87) |
| verify_gui10 | gui10: b10_gui9_death9_light15 | DIFFERS | comparison | pixel (413,168).RGB: Java (84, 79, 79), Rust (84, 87, 87) |
| verify_gui10 | gui10: b10_gui9_death13_light0 | DIFFERS | comparison | pixel (418,186).RGB: Java (0, 88, 88), Rust (0, 84, 84) |
| verify_gui10 | gui10: b10_gui9_death13_light7 | DIFFERS | comparison | pixel (418,186).RGB: Java (15, 88, 88), Rust (15, 84, 84) |
| verify_gui10 | gui10: b10_gui9_death13_light15 | DIFFERS | comparison | pixel (418,186).RGB: Java (85, 88, 88), Rust (85, 84, 84) |
| verify_gui10 | gui10: b10_gui9_death20_light0 | DIFFERS | comparison | pixel (418,186).RGB: Java (0, 88, 88), Rust (0, 84, 84) |
| verify_gui10 | gui10: b10_gui9_death20_light7 | DIFFERS | comparison | pixel (418,186).RGB: Java (15, 88, 88), Rust (15, 84, 84) |
| verify_gui10 | gui10: b10_gui9_death20_light15 | DIFFERS | comparison | pixel (418,186).RGB: Java (85, 88, 88), Rust (85, 84, 84) |
| verify_gui10 | gui10: b10_gui9_armor_death1 | DIFFERS | comparison | pixel (395,112).RGB: Java (36, 123, 112), Rust (38, 129, 118) |
| verify_gui10 | gui10: b10_gui9_armor_death5 | DIFFERS | comparison | pixel (433,137).RGB: Java (35, 118, 107), Rust (38, 129, 118) |
| verify_gui10 | gui10: b10_gui9_armor_death13 | DIFFERS | comparison | pixel (410,182).RGB: Java (29, 95, 86), Rust (198, 198, 198) |
| verify_gui10 | gui10: b10_gui9_armor_death20 | DIFFERS | comparison | pixel (410,182).RGB: Java (29, 95, 86), Rust (198, 198, 198) |
| verify_gui10 | gui10: b10_gui9_mouse0_first | DIFFERS | comparison | pixel (384,117).RGB: Java (47, 19, 8), Rust (45, 18, 8) |
| verify_gui10 | gui10: b10_gui9_mouse0_retained | DIFFERS | comparison | pixel (391,125).RGB: Java (44, 17, 8), Rust (41, 14, 5) |
| verify_gui10 | gui10: b10_gui9_mouse1_first | DIFFERS | comparison | pixel (430,142).RGB: Java (49, 19, 8), Rust (47, 18, 8) |
| verify_gui10 | gui10: b10_gui9_mouse1_retained | DIFFERS | comparison | pixel (432,149).RGB: Java (44, 16, 6), Rust (43, 16, 6) |
| verify_gui10 | gui10: b10_gui9_mouse2_first | DIFFERS | comparison | pixel (418,186).RGB: Java (15, 88, 88), Rust (15, 84, 84) |
| verify_gui10 | gui10: b10_gui9_mouse2_retained | DIFFERS | comparison | pixel (421,184).RGB: Java (15, 77, 77), Rust (15, 86, 86) |
| verify_gui10 | gui10: b10_gui9_mouse3_first | DIFFERS | comparison | pixel (418,186).RGB: Java (15, 88, 88), Rust (15, 84, 84) |
| verify_gui10 | gui10: b10_gui9_mouse3_retained | DIFFERS | comparison | pixel (418,180).RGB: Java (23, 62, 62), Rust (23, 76, 76) |
| verify_gui10 | gui10: b10_title10_0 | DIFFERS | comparison | pixel (0,0).RGB: Java (214, 219, 236), Rust (227, 231, 242) |
| verify_gui10 | gui10: b10_title10_1 | DIFFERS | comparison | pixel (0,0).RGB: Java (214, 219, 236), Rust (227, 231, 242) |
| verify_gui10 | gui10: b10_title10_7 | DIFFERS | comparison | pixel (0,0).RGB: Java (214, 219, 236), Rust (227, 231, 242) |
| verify_gui10 | gui10: b10_title10_37 | DIFFERS | comparison | pixel (0,0).RGB: Java (214, 219, 238), Rust (227, 231, 244) |
| verify_gui10 | gui10: b10_title10_100 | DIFFERS | comparison | pixel (0,0).RGB: Java (213, 222, 244), Rust (227, 233, 248) |
| verify_gui10 | gui10: b10_title10_400 | DIFFERS | comparison | pixel (0,0).RGB: Java (204, 215, 236), Rust (221, 228, 242) |
| verify_gui10 | gui10: b10_title10_999 | DIFFERS | comparison | pixel (0,0).RGB: Java (215, 226, 255), Rust (228, 236, 255) |
| verify_gui10 | gui10: b10_title10_2000 | DIFFERS | comparison | pixel (0,0).RGB: Java (200, 211, 255), Rust (218, 225, 255) |
| verify_gui10 | gui10: inventory_empty | DIFFERS | comparison | pixel (355,113).RGB: Java (40, 27, 12), Rust (38, 26, 10) |
| verify_gui10 | gui10: inventory_items | DIFFERS | comparison | pixel (353,114).RGB: Java (41, 28, 12), Rust (42, 28, 13) |
| verify_gui10 | gui10: inventory_turn | DIFFERS | comparison | pixel (347,114).RGB: Java (36, 24, 8), Rust (47, 31, 15) |
| verify_gui10 | gui10: inventory_cursor | DIFFERS | comparison | pixel (345,114).RGB: Java (42, 29, 13), Rust (36, 24, 8) |
| verify_gui10 | gui10: crafting_items | DIFFERS | comparison | pixel (312,116).RGB: Java (119, 97, 62), Rust (139, 113, 73) |
| verify_gui10 | gui10: crafting_tooltip | DIFFERS | comparison | pixel (522,114).RGB: Java (27, 12, 27), Rust (198, 198, 198) |
| verify_gui10 | gui10: chest_items | DIFFERS | comparison | pixel (268,116).RGB: Java (34, 111, 27), Rust (39, 129, 32) |
| verify_gui10 | gui10: chest_cursor | DIFFERS | comparison | pixel (268,116).RGB: Java (34, 111, 27), Rust (39, 129, 32) |
| verify_gui10 | gui10: largechest_items | DIFFERS | comparison | pixel (556,242).RGB: Java (134, 71, 88), Rust (156, 83, 103) |
| verify_gui10 | gui10: furnace_mid | DIFFERS | comparison | pixel (364,116).RGB: Java (81, 81, 81), Rust (94, 94, 94) |
| verify_gui10 | gui10: furnace_idle | DIFFERS | comparison | pixel (364,116).RGB: Java (81, 81, 81), Rust (94, 94, 94) |
| verify_gui10 | gui10: furnace_missing_fuel_duration | DIFFERS | comparison | pixel (364,116).RGB: Java (81, 81, 81), Rust (94, 94, 94) |
| verify_gui10 | gui10: dispenser_items | DIFFERS | comparison | pixel (394,114).RGB: Java (24, 9, 24), Rust (150, 150, 150) |
| verify_gui10 | gui10: inventory_diamond_pickaxe | DIFFERS | comparison | pixel (352,107).RGB: Java (45, 197, 180), Rust (44, 194, 177) |
| verify_gui10 | gui10: inventory_equipped_turn | DIFFERS | comparison | pixel (354,108).RGB: Java (48, 206, 190), Rust (45, 197, 180) |
| verify_gui10 | gui10: inventory_pumpkin_block | DIFFERS | comparison | pixel (381,139).RGB: Java (49, 211, 193), Rust (44, 194, 176) |
| verify_gui10 | gui10: inventory_fishing_rod | DIFFERS | comparison | pixel (381,139).RGB: Java (49, 211, 193), Rust (44, 194, 176) |
| verify_gui10 | gui10: inventory_bow | DIFFERS | comparison | pixel (381,139).RGB: Java (49, 211, 193), Rust (44, 194, 176) |
| verify_gui10 | gui10: inventory_dye | DIFFERS | comparison | pixel (381,139).RGB: Java (49, 211, 193), Rust (44, 194, 176) |
| verify_gui10 | gui10: chest_right_pickup | EXACT | comparison | chest_right_pickup: EXACT |
| verify_gui10 | gui10: chest_right_place | EXACT | comparison | chest_right_place: EXACT |
| verify_gui10 | gui10: chest_shift_transfer | EXACT | comparison | chest_shift_transfer: EXACT |
| verify_gui10 | gui10: crafting_click_result | DIFFERS | comparison | pixel (312,116).RGB: Java (119, 97, 62), Rust (139, 113, 73) |
| verify_gui10 | gui10: inventory_sneak_swing | DIFFERS | comparison | pixel (356,116).RGB: Java (30, 21, 11), Rust (34, 24, 11) |
| verify_gui10 | gui10: inventory_riding | DIFFERS | comparison | pixel (353,114).RGB: Java (41, 28, 12), Rust (42, 28, 13) |
| verify_gui10 | gui10: inventory_riding_armor | DIFFERS | comparison | pixel (354,108).RGB: Java (48, 206, 190), Rust (45, 197, 180) |
| verify_gui10 | gui10: inventory_sneak_armor | DIFFERS | comparison | pixel (354,111).RGB: Java (48, 206, 190), Rust (45, 197, 180) |
| verify_gui10 | gui10: inventory_sneak_armor_next_frame | DIFFERS | comparison | pixel (354,111).RGB: Java (48, 206, 190), Rust (45, 197, 180) |
| verify_gui10 | gui10: inventory_hurt_armor | DIFFERS | comparison | pixel (357,108).RGB: Java (104, 124, 114), Rust (125, 124, 114) |
| verify_gui10 | gui10: inventory_hurt | DIFFERS | comparison | pixel (357,110).RGB: Java (70, 13, 7), Rust (86, 13, 7) |
| verify_gui10 | gui10: inventory_cloth | DIFFERS | comparison | pixel (354,108).RGB: Java (135, 84, 54), Rust (129, 79, 51) |
| verify_gui10 | gui10: inventory_chain | DIFFERS | comparison | pixel (357,107).RGB: Java (0, 0, 0), Rust (160, 160, 160) |
| verify_gui10 | gui10: inventory_iron | DIFFERS | comparison | pixel (354,108).RGB: Java (193, 193, 193), Rust (188, 188, 188) |
| verify_gui10 | gui10: inventory_gold | DIFFERS | comparison | pixel (354,108).RGB: Java (231, 214, 30), Rust (228, 206, 28) |
| verify_gui10 | gui10: inventory_signed_counts | DIFFERS | comparison | pixel (355,113).RGB: Java (40, 27, 12), Rust (38, 26, 10) |
| verify_gui10 | gui10: sign_empty | DIFFERS | comparison | pixel (355,137).RGB: Java (159, 132, 77), Rust (173, 141, 87) |
| verify_gui10 | gui10: sign_text_cursor | DIFFERS | comparison | pixel (355,137).RGB: Java (159, 132, 77), Rust (173, 141, 87) |
| verify_gui10 | gui10: sign_text_no_cursor | DIFFERS | comparison | pixel (355,137).RGB: Java (159, 132, 77), Rust (173, 141, 87) |
| verify_gui10 | gui10: sign_second_line | DIFFERS | comparison | pixel (355,137).RGB: Java (159, 132, 77), Rust (173, 141, 87) |
| verify_gui10 | gui10: sign_backspace_escape | DIFFERS | comparison | pixel (355,137).RGB: Java (159, 132, 77), Rust (173, 141, 87) |
| verify_gui10 | gui10: sign_wall_2 | DIFFERS | comparison | pixel (355,195).RGB: Java (159, 132, 77), Rust (173, 141, 87) |
| verify_gui10 | gui10: sign_wall_3 | DIFFERS | comparison | pixel (355,195).RGB: Java (159, 132, 77), Rust (173, 141, 87) |
| verify_gui10 | gui10: sign_wall_4 | DIFFERS | comparison | pixel (355,195).RGB: Java (159, 132, 77), Rust (173, 141, 87) |
| verify_gui10 | gui10: sign_wall_5 | DIFFERS | comparison | pixel (355,195).RGB: Java (159, 132, 77), Rust (173, 141, 87) |
| verify_gui10 | gui10: packs_list | EXACT | comparison | packs_list: EXACT |
| verify_gui10 | gui10: packs_selected | EXACT | comparison | packs_selected: EXACT |
| verify_gui10 | gui10: pack_title | DIFFERS | comparison | pixel (0,0).RGB: Java (214, 219, 236), Rust (227, 231, 242) |
| verify_gui10 | gui10: pack_hud | DIFFERS | comparison | pixel (4,4).RGB: Java (0, 0, 0), Rust (255, 255, 255) |
| verify_gui10 | gui10: chat_empty | EXACT | comparison | chat_empty: EXACT |
| verify_gui10 | gui10: chat_typed | EXACT | comparison | chat_typed: EXACT |
| verify_gui10 | gui10: chat_cursor_off | EXACT | comparison | chat_cursor_off: EXACT |
| verify_gui10 | gui10: chat_cursor_on | EXACT | comparison | chat_cursor_on: EXACT |
| verify_gui10 | gui10: chat_limit | EXACT | comparison | chat_limit: EXACT |
| verify_gui10 | gui10: chat_limit_backspace | EXACT | comparison | chat_limit_backspace: EXACT |
| verify_gui10 | gui10: anaglyph_video | DIFFERS | comparison | pixel (436,272).RGB: Java (0, 0, 0), Rust (13, 13, 13) |
| verify_gui10 | gui10: anaglyph_packs | EXACT | comparison | anaglyph_packs: EXACT |
| verify_gui10 | gui10: anaglyph_pack_title | DIFFERS | comparison | pixel (0,0).RGB: Java (191, 192, 188), Rust (212, 213, 210) |
| verify_gui10 | gui10: anaglyph_start_font | DIFFERS | comparison | pixel (4,4).RGB: Java (0, 0, 0), Rust (255, 255, 255) |
| verify_gui10 | gui10: anaglyph_font_palette_retained | DIFFERS | comparison | pixel (4,4).RGB: Java (0, 0, 0), Rust (255, 255, 255) |
| verify_ingame10 | gui10: b10_portal_early | DIFFERS | comparison | pixel (4,4).RGB: Java (20, 28, 71), Rust (255, 255, 255) |
| verify_ingame10 | gui10: b10_portal_full | DIFFERS | comparison | pixel (4,4).RGB: Java (0, 42, 255), Rust (255, 255, 255) |
| verify_ingame10 | gui10: b10_portal_nausea | DIFFERS | comparison | pixel (4,4).RGB: Java (25, 25, 25), Rust (255, 255, 255) |
| verify_latefix10 | complete 1.8.1 pickup fixture (capture consistency) | EXACT | comparison | capture consistency EXACT (fixture integrity; Rust renderer parity is separate) |
| verify_latefix10 | complete 1.8.1 pickupfast fixture (capture consistency) | EXACT | comparison | capture consistency EXACT (fixture integrity; Rust renderer parity is separate) |
| verify_latefix10 | complete 1.8.1 particles fixture (capture consistency) | EXACT | comparison | capture consistency EXACT (fixture integrity; Rust renderer parity is separate) |
| verify_minecraft10 | minecraft 1.0: original runTick event queues, clicks, screen/focus callbacks and world policy | DIFFERS | oracle failure | Oracle failed before comparison: java.lang.NullPointerException; at net.minecraft.client.Minecraft.clickMiddleMouseButton(Minecraft.java:2201) (rechecked by verify_constantfix10) |
| verify_minecraft10 | minecraft 1.0: original preloadWorld coordinate, progress and lighting callbacks | EXACT | comparison | MINECRAFT PRELOAD EXACT: 128 cases, 0 differences (rechecked by verify_constantfix10) |
| verify_minecraft10 | minecraft 1.0: retained clickMouse and sendClickBlockToController effects | DIFFERS | oracle failure | Oracle failed before comparison: java.lang.NullPointerException; at net.minecraft.client.Minecraft.clickMiddleMouseButton(Minecraft.java:2201) (rechecked by verify_constantfix10) |
| verify_minecraft10 | minecraft 1.0: original debugInfoEntities counts and excluded particle layer | EXACT | comparison | MINECRAFT DEBUG ENTITIES EXACT: 10000 cases, 0 differences (rechecked by verify_constantfix10) |
| verify_minecraft10 | minecraft 1.0: retained global key registry and press-count overflow | EXACT | comparison | GLOBAL KEY REGISTRY EXACT: 20000 events, 4 settings, 8 standalone bindings, overflow (rechecked by verify_constantfix10) |
| verify_minecraft10 | minecraft 1.0: retained controller factory, spawn yaw and item-use release | EXACT | comparison | CONTROLLER LIFECYCLE EXACT: 1000 factory/flip cases, 20000 partial/reset boundaries, original air failure (1232729 bytes) (rechecked by verify_constantfix10) |
| verify_minecraft10 | minecraft 1.0: retained create-world callbacks and WorldSettings | DIFFERS | comparison | byte 1611, field("field_35364_f").get(g).equals("creative") = false (Mode_worldcallbacks10.java:25, field starts 1611, 1 bytes, Boolean); Rust True (rechecked by verify_constantfix10) |
| verify_minecraft10 | minecraft 1.0: retained world seed parsing and settings | EXACT | comparison | CREATE WORLD SEED POLICY EXACT: 65536 Java digits, 10096 seed/mode/features records (clock-random seeds classified; Rust GUI callbacks not replayed) (rechecked by verify_constantfix10) |
| verify_minecraft10 | minecraft 1.0: retained saved-world controller selection | EXACT | comparison | select8_181 EXACT (864178 bytes) (rechecked by verify_constantfix10) |
| verify_playertick10 | playertick10: idle, 500 ticks | DIFFERS | comparison | byte 1470, frame 1.entityRand: Java 206789181479531, Rust 203227649148896 (rechecked by verify_constantfix10) |
| verify_playertick10 | playertick10: movement, 240 ticks | DIFFERS | comparison | byte 1470, frame 1.entityRand: Java 206789181479531, Rust 203227649148896 (rechecked by verify_constantfix10) |
| verify_playertick10 | playertick10: sprint, 240 ticks | DIFFERS | comparison | byte 1470, frame 1.entityRand: Java 206789181479531, Rust 203227649148896 (rechecked by verify_constantfix10) |
| verify_playertick10 | playertick10: flight, 500 ticks | DIFFERS | comparison | byte 1470, frame 1.entityRand: Java 206789181479531, Rust 203227649148896 (rechecked by verify_constantfix10) |
| verify_playertick10 | playertick10: creativecontroller, 500 ticks | DIFFERS | comparison | byte 1470, frame 1.entityRand: Java 206789181479531, Rust 203227649148896 (rechecked by verify_constantfix10) |
| verify_playertick10 | playertick10: starve, 500 ticks | DIFFERS | comparison | byte 1470, frame 1.entityRand: Java 206789181479531, Rust 203227649148896 (rechecked by verify_constantfix10) |
| verify_playertick10 | playertick10: regen, 500 ticks | DIFFERS | comparison | byte 1470, frame 1.entityRand: Java 206789181479531, Rust 203227649148896 (rechecked by verify_constantfix10) |
| verify_playertick10 | playertick10: eat260, 500 ticks | DIFFERS | comparison | byte 1470, frame 1.entityRand: Java 206789181479531, Rust 203227649148896 (rechecked by verify_constantfix10) |
| verify_playertick10 | playertick10: eat282, 500 ticks | DIFFERS | comparison | byte 1470, frame 1.entityRand: Java 206789181479531, Rust 203227649148896 (rechecked by verify_constantfix10) |
| verify_playertick10 | playertick10: eat322, 500 ticks | DIFFERS | comparison | byte 1470, frame 1.entityRand: Java 206789181479531, Rust 203227649148896 (rechecked by verify_constantfix10) |
| verify_playertick10 | playertick10: eat365, 500 ticks | DIFFERS | comparison | byte 1470, frame 1.entityRand: Java 206789181479531, Rust 203227649148896 (rechecked by verify_constantfix10) |
| verify_playertick10 | playertick10: eat367, 500 ticks | DIFFERS | comparison | byte 1470, frame 1.entityRand: Java 206789181479531, Rust 203227649148896 (rechecked by verify_constantfix10) |
| verify_playertick10 | playertick10: cancel, 500 ticks | DIFFERS | comparison | byte 1470, frame 1.entityRand: Java 206789181479531, Rust 203227649148896 (rechecked by verify_constantfix10) |
| verify_playertick10 | playertick10: identity, 500 ticks | DIFFERS | comparison | byte 1470, frame 1.entityRand: Java 206789181479531, Rust 203227649148896 (rechecked by verify_constantfix10) |
| verify_playertick10 | playertick10: bow, 500 ticks | DIFFERS | comparison | byte 1470, frame 1.entityRand: Java 206789181479531, Rust 203227649148896 (rechecked by verify_constantfix10) |
| verify_playertick10 | playertick10: xp, 500 ticks | DIFFERS | comparison | byte 1470, frame 1.entityRand: Java 206789181479531, Rust 203227649148896 (rechecked by verify_constantfix10) |
| verify_playertick10 | playertick10: damage, 500 ticks | DIFFERS | comparison | byte 1184, frame 1.payloadLength: Java 1249, Rust 1244 (rechecked by verify_constantfix10) |
| verify_playertick10 | playertick10: creativedamage, 500 ticks | DIFFERS | comparison | byte 1470, frame 1.entityRand: Java 206789181479531, Rust 203227649148896 (rechecked by verify_constantfix10) |
| verify_playertick10 | playertick10: creativevoid, 500 ticks | DIFFERS | comparison | byte 1184, frame 1.payloadLength: Java 1249, Rust 1244 (rechecked by verify_constantfix10) |
| verify_playertick10 | playertick10: speed, 500 ticks | DIFFERS | comparison | byte 1184, frame 1.payloadLength: Java 1261, Rust 1200 (rechecked by verify_constantfix10) |
| verify_playertick10 | playertick10: haste, 500 ticks | DIFFERS | comparison | byte 1184, frame 1.payloadLength: Java 1261, Rust 1200 (rechecked by verify_constantfix10) |
| verify_playertick10 | playertick10: fatigue, 500 ticks | DIFFERS | comparison | byte 1184, frame 1.payloadLength: Java 1261, Rust 1200 (rechecked by verify_constantfix10) |
| verify_playertick10 | playertick10: nausea, 500 ticks | DIFFERS | comparison | byte 1184, frame 1.payloadLength: Java 1261, Rust 1200 (rechecked by verify_constantfix10) |
| verify_render10 | chunk meshes: seed 25, world rand 1, fancy, AO on | DIFFERS | comparison | chunk (-6,10) pass 0 vertex 1308: java pos (2,7.999,2) uv (0.8125,0.875) rgba ffffffff light 0 \| rust pos (2,8,2) uv (0.8125,0.875) rgba ffffffff light 0 |
| verify_render10 | chunk meshes: seed 25, world rand 1, fast, AO on | DIFFERS | comparison | chunk (-6,10) pass 0 vertex 1308: java pos (2,7.999,2) uv (0.8125,0.875) rgba ffffffff light 0 \| rust pos (2,8,2) uv (0.8125,0.875) rgba ffffffff light 0 |
| verify_render10 | chunk meshes: seed 25, world rand 1, fancy, AO off | DIFFERS | comparison | chunk (-6,10) pass 0 vertex 1308: java pos (2,7.999,2) uv (0.8125,0.875) rgba ffffffff light 0 \| rust pos (2,8,2) uv (0.8125,0.875) rgba ffffffff light 0 |
| verify_render10 | chunk meshes: seed 25, world rand 1, fast, AO off | DIFFERS | comparison | chunk (-6,10) pass 0 vertex 1308: java pos (2,7.999,2) uv (0.8125,0.875) rgba ffffffff light 0 \| rust pos (2,8,2) uv (0.8125,0.875) rgba ffffffff light 0 |
| verify_render10 | chunk meshes: seed 1, world rand 1, fancy, AO on | DIFFERS | checker failure | Checker failed before completing comparison: no entry found for key; thread 'main' (38372) panicked at C:\Users\logiu\Desktop\Projects\minecraft\wt181-r10harness\crates\worldgen10\src\world\mod.rs:477:21: |
| verify_render10 | chunk meshes: seed 1, world rand 1, fast, AO on | DIFFERS | checker failure | Checker failed before completing comparison: no entry found for key; thread 'main' (26996) panicked at C:\Users\logiu\Desktop\Projects\minecraft\wt181-r10harness\crates\worldgen10\src\world\mod.rs:477:21: |
| verify_render10 | chunk meshes: seed 1, world rand 1, fancy, AO off | DIFFERS | checker failure | Checker failed before completing comparison: no entry found for key; thread 'main' (33120) panicked at C:\Users\logiu\Desktop\Projects\minecraft\wt181-r10harness\crates\worldgen10\src\world\mod.rs:477:21: |
| verify_render10 | chunk meshes: seed 1, world rand 1, fast, AO off | DIFFERS | checker failure | Checker failed before completing comparison: no entry found for key; thread 'main' (29592) panicked at C:\Users\logiu\Desktop\Projects\minecraft\wt181-r10harness\crates\worldgen10\src\world\mod.rs:477:21: |
| verify_render10 | chunk meshes: seed -1, world rand -1, fancy, AO on | DIFFERS | checker failure | Checker failed before completing comparison: no entry found for key; thread 'main' (32212) panicked at C:\Users\logiu\Desktop\Projects\minecraft\wt181-r10harness\crates\worldgen10\src\world\mod.rs:477:21: |
| verify_render10 | chunk meshes: seed -1, world rand -1, fast, AO on | DIFFERS | checker failure | Checker failed before completing comparison: no entry found for key; thread 'main' (36708) panicked at C:\Users\logiu\Desktop\Projects\minecraft\wt181-r10harness\crates\worldgen10\src\world\mod.rs:477:21: |
| verify_render10 | chunk meshes: seed -1, world rand -1, fancy, AO off | DIFFERS | checker failure | Checker failed before completing comparison: no entry found for key; thread 'main' (18956) panicked at C:\Users\logiu\Desktop\Projects\minecraft\wt181-r10harness\crates\worldgen10\src\world\mod.rs:477:21: |
| verify_render10 | chunk meshes: seed -1, world rand -1, fast, AO off | DIFFERS | checker failure | Checker failed before completing comparison: no entry found for key; thread 'main' (34444) panicked at C:\Users\logiu\Desktop\Projects\minecraft\wt181-r10harness\crates\worldgen10\src\world\mod.rs:477:21: |
| verify_render10 | chunk meshes: seed 0, world rand 1, fancy, AO on | DIFFERS | comparison | chunk (10,5) pass 0 vertex 1424: java pos (16,52,1) uv (0.18746094,0.062460937) rgba ffffffff light 0 \| rust pos (4,39,1) uv (0.12496094,0.062460937) rgba ffffffff light 0 |
| verify_render10 | chunk meshes: seed 0, world rand 1, fast, AO on | DIFFERS | comparison | chunk (10,5) pass 0 vertex 1424: java pos (16,52,1) uv (0.18746094,0.062460937) rgba ffffffff light 0 \| rust pos (4,39,1) uv (0.12496094,0.062460937) rgba ffffffff light 0 |
| verify_render10 | chunk meshes: seed 0, world rand 1, fancy, AO off | DIFFERS | comparison | chunk (10,5) pass 0 vertex 1424: java pos (16,52,1) uv (0.18746094,0.062460937) rgba ffffffff light 0 \| rust pos (4,39,1) uv (0.12496094,0.062460937) rgba ffffffff light 0 |
| verify_render10 | chunk meshes: seed 0, world rand 1, fast, AO off | DIFFERS | comparison | chunk (10,5) pass 0 vertex 1424: java pos (16,52,1) uv (0.18746094,0.062460937) rgba ffffffff light 0 \| rust pos (4,39,1) uv (0.12496094,0.062460937) rgba ffffffff light 0 |
| verify_render10 | block showcase: fancy on, normal | DIFFERS | comparison | chunk (0,0) pass 0 vertex 1876: java pos (5,65,1) uv (0.062460937,0.062460937) rgba ff71b98e light 900090 \| rust pos (5,65,1) uv (0.062460937,0.062460937) rgba ff4ec46a light 900090 |
| verify_render10 | block showcase: fancy on, anaglyph | DIFFERS | comparison | chunk (0,0) pass 0 vertex 1876: java pos (5,65,1) uv (0.062460937,0.062460937) rgba ff79aca4 light 900090 \| rust pos (5,65,1) uv (0.062460937,0.062460937) rgba ff56a89c light 900090 |
| verify_render10 | block showcase: fancy off, normal | DIFFERS | comparison | chunk (0,0) pass 0 vertex 1876: java pos (5,65,1) uv (0.062460937,0.062460937) rgba ff71b98e light f00070 \| rust pos (5,65,1) uv (0.062460937,0.062460937) rgba ff4ec46a light f00070 |
| verify_render10 | block showcase: fancy off, anaglyph | DIFFERS | comparison | chunk (0,0) pass 0 vertex 1876: java pos (5,65,1) uv (0.062460937,0.062460937) rgba ff79aca4 light f00070 \| rust pos (5,65,1) uv (0.062460937,0.062460937) rgba ff56a89c light f00070 |
| verify_render10 | block showcase: fast on, normal | DIFFERS | comparison | chunk (0,0) pass 0 vertex 1876: java pos (5,65,1) uv (0.062460937,0.062460937) rgba ff71b98e light 900090 \| rust pos (5,65,1) uv (0.062460937,0.062460937) rgba ff4ec46a light 900090 |
| verify_render10 | block showcase: fast on, anaglyph | DIFFERS | comparison | chunk (0,0) pass 0 vertex 1876: java pos (5,65,1) uv (0.062460937,0.062460937) rgba ff79aca4 light 900090 \| rust pos (5,65,1) uv (0.062460937,0.062460937) rgba ff56a89c light 900090 |
| verify_render10 | block showcase: fast off, normal | DIFFERS | comparison | chunk (0,0) pass 0 vertex 1876: java pos (5,65,1) uv (0.062460937,0.062460937) rgba ff71b98e light f00070 \| rust pos (5,65,1) uv (0.062460937,0.062460937) rgba ff4ec46a light f00070 |
| verify_render10 | block showcase: fast off, anaglyph | DIFFERS | comparison | chunk (0,0) pass 0 vertex 1876: java pos (5,65,1) uv (0.062460937,0.062460937) rgba ff79aca4 light f00070 \| rust pos (5,65,1) uv (0.062460937,0.062460937) rgba ff56a89c light f00070 |
| verify_render10 | Nether terrain meshes (fire, glowstone, the Nether light table) | DIFFERS | comparison | chunk (-2,-2) pass 0 vertex 511: java pos (16,0,8) uv (0.12496094,0.12496094) rgba ff7f7f7f light 0 \| rust pos (16,0,8) uv (0.12496094,0.12496094) rgba ff7f7f7f light 3c0000 |
| verify_render10 | first-person view, player and block targeting | DIFFERS | comparison | case 320 kind 1 item 0 draw 0 texture: java 00000000, rust 00000009 |
| verify_render10 | renderer window sizes and Java negative-coordinate wrapping | EXACT | comparison | WINDOWS EXACT: 32 comparisons (rechecked by verify_latefix10) |
| verify_render10 | sky colors | DIFFERS | comparison | sky[0]: Java 1055846127, Rust 1056372471 |
| verify_render10 | Fast and Fancy clouds | EXACT | comparison | CLOUDS EXACT: 10752 vertices (Fancy) |
| verify_render10 | sky GL colors and sunrise vertex colors, anaglyph on/off | DIFFERS | comparison | sky case 0, GL color float bits: Rust 1060255561, Java 1060245798 |
| verify_render10 | ambient weather vertices | DIFFERS | comparison | vertex 0 field 0 (3): Rust 1077936128, Java 0 |
| verify_render10 | moving piston renderer (original TileEntityRendererPiston vertex bits) | DIFFERS | comparison | case 270 id 2/0 facing 0 head false ext true progress 0/0/0 ao false fancy false vertex 4 field 5: java ff71b98e, rust ff4ec46a |
| verify_render10 | World.randomDisplayUpdates: depthsuspend specks and World.rand | EXACT | comparison | EXACT: entities181 displayloop (519111 bytes) |
| verify_render10 | ScreenShotHelper: numbered and named screenshots, pixel order | EXACT | comparison | screenshots: 97 values, 0 differences |
| verify_render10 | BlockVine: random-tick growth, crowding and losing support | DIFFERS | comparison | byte 906, w.getBlockId(x,y,z) [big endian] = 106 (Mode_vine10.java:22, field starts 906, 1 bytes, Integer); Rust 0 |
| verify_render10 | EntityPlayer NBT: read then write, byte order of level.dat player data | DIFFERS | comparison | byte 662, b.size() = 718 (Mode_playernbt10.java:30, field starts 662, 4 bytes, Integer); Rust 654 |
| verify_render10 | lightmap texels (EntityRenderer.func_35808_e): day, night, rain, thunder, flashes, flicker, brightness | EXACT | comparison | LIGHTMAP EXACT |
| verify_render10 | render lists (RenderList: camera offset kept in double, near, far and at the border) | EXACT | comparison | render lists: 20000 cases, 60000 raw-bit comparisons, 0 differences |
| verify_statsync10 | statsync10: receive, send and file writer | DIFFERS | comparison | snapshot 0.achievement[17].present: Java True, Rust False (registry length Java 27, Rust 17) (rechecked by verify_constantfix10) |
| verify_tracefix10 | complete 1.8.1 pickuplight fixture (capture consistency) | EXACT | comparison | capture consistency EXACT (fixture integrity; Rust renderer parity is separate) |

## Measured suite times

| Suite | Seconds | Exit code |
| --- | ---: | ---: |
| verify10 | 3804.33 | 1 |
| verify_alphafix10 | 1.50 | 1 |
| verify_biomes10 | 6.24 | 1 |
| verify_bucketfix10 | 138.97 | 1 |
| verify_constantfix10 | 435.90 | 1 |
| verify_entities10 | 205.45 | 1 |
| verify_gui10 | 130.10 | 1 |
| verify_harnessfix10 | 109.43 | 1 |
| verify_ingame10 | 26.52 | 1 |
| verify_latefix10 | 138.54 | 1 |
| verify_minecraft10 | 30.21 | 1 |
| verify_playerguifix10 | 142.77 | 1 |
| verify_playertick10 | 68.63 | 1 |
| verify_regions10 | 71.47 | 1 |
| verify_render10 | 426.75 | 1 |
| verify_replays10 | 69.74 | 1 |
| verify_serverfix10 | 316.24 | 1 |
| verify_statsync10 | 10.08 | 1 |
| verify_tracefix10 | 4.62 | 0 |

Logs and machine-readable section results: `target/r10/baseline/`.
