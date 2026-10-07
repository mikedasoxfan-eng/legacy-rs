| check | result | cause + fix |
|---|---|---|
| entities10 itemblock10 | EXACT (9,714,710 bytes) | Rust check writer bug, not a game bug. Mode_itemblock10_10 builds ProbeWorld via Unsafe.allocateInstance, so World.field_35472_c is 0 in Java; the Rust BlockProbe used the default height 128, so case existing=0,y=127,side=-1,scene=1 returned early at `y == height-1 && solid` (2 events) while Java went on to canBlockBePlacedAt (3 events). Fix: BlockProbe::height() returns 0 in crates/worldgen10/src/entity/round10_verify181.rs. ItemBlock::on_item_use (game code) already matches ItemBlock.onItemUse. |
| entities10 (all), r10blocks4, r10blocks3, entities7_10 | EXACT | rerun after fix |
| playtest10 items_snow_golem, items_lily_pad, items_nether_wart | PASS | rerun after fix |
