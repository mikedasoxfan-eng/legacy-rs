# QA fix: potionexpiry5, metadata7, buckethit7, bucketray7

| check | EXACT now / still differs | cause + fix |
|---|---|---|
| entities5 `potionexpiry5` | EXACT (333552 bytes) | The Rust port was already right; the Java fixture was mis-ported from 1.8.1. `Mode_entityround5_10.potionExpiry` called `e.dropFewItems(true, 0)`, which in 1.0 is the real drop method (1.8.1's obfuscated `dropFewItems()` was the potion tick), so Java never ticked the effect and spawned drops. Now calls 1.0 `EntityLiving.updatePotionEffects()`. |
| entities7 `metadata7` | EXACT (16804181 bytes) | Rust writer emitted a name (`getItemNameIS`) for id 31 (ItemColored tallgrass) after every row; `Mode_itemmetadata7_10.row` only writes names for ItemSlab/ItemCloth. Rust now writes names only for 44 and 35 (`round7_verify181.rs`). Icon/colour/metadata logic was already exact. |
| bucketfix `buckethit7` | EXACT (17525376 bytes) | id 335 is `ItemBucketMilk` in 1.0: `onItemRightClick` only calls `setItemInUse` (no ray, no writes, stack returned unchanged). The replay ran it through the 1.8.1 bucket path (ray 3.25/0.5 etc.). Replay now yields zero ray vectors and the unchanged stack for 335. The live game already routed milk to `start_item_use` (items.rs), so no gameplay change. |
| bucketfix `bucketray7` | EXACT (2340912 bytes) | Same milk fix as above. |

Regression: verify_entities5_10, verify_entities7_10, verify_bucketfix10, verify_r10blocks4, verify_r10entities3 and playtests items_cauldron, items_drink_potion, items_splash_potion (plus items_milk_bucket) all pass (see final report).
