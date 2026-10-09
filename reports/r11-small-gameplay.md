# 1.1 brewing and Creeper records

Brewing starts at400ticks instead of600. A skeleton-killed Creeper draws nextInt(10), allowing all ten original record items instead of only two. Both changes are isolated to worldgen11.

Original SmallGameplay11 invokes actual TileEntityBrewingStand.updateEntity and EntityCreeper.onDeath. Brewing comparison covers9,648snapshots across24timelines, including ingredient replacement/removal, NBT reload, slot contents, metadata, dirty callbacks and RNG. Creeper comparison covers1,200deaths with skeleton, zombie or generic sources, multiplayer flags, ordered drops and RNG; all ten records are observed. Three brewing tests and the Creeper original-bytecode test pass.

Reproduce with tools/oracle11/verify_gameplay_slice.ps1. All32regenerated fixtures/tables match their committed original-bytecode vectors. Run the brewing library tests serially with cargo test --release -p worldgen11 --lib container::brewing::tests -- --test-threads=1 and the Creeper original-bytecode test with cargo test --release -p worldgen11 --lib creeper_records_match_original_11_bytecode.

These comparisons cover the two numeric1.1class deltas. Client brewing-progress rendering and complete release regressions remain separate gates.
