# Beta 1.7.3 decorations and fishing

The simulation ports `BlockSign`, `ItemSign`, `TileEntitySign`, `EntityPainting`,
`EnumArt`, `ItemPainting`, `ItemMap`, `MapData`, `MapStorage`, `EntityFish` and
`ItemFishingRod`. The client renders signs, paintings and fishing hooks from
worker snapshots, opens `GuiEditSign` after placement, sends edited text back to
the world, and supplies live map pixels and markers to the existing held-map path.

## Verification

Run from the repository root in Git Bash:

```sh
bash tools/verify_decorations.sh
bash tools/verify_gui.sh
bash tools/verify.sh
```

`DecorationOracle.java` calls the original classes in the deobfuscated jar on
Mojang's Java 8u51. Random streams are pinned independently. Comparisons use
raw IEEE float/double bits and the original renderer's tessellator vertices,
UVs, normals, colours, matrix operations and fixed-function state.

| Feature | Oracle coverage |
| --- | --- |
| Signs | 462 placements across support materials, faces and yaw boundaries; support removal and RNG; 2,000 editing operations; NBT truncation; 160 renderer cases / 521,440 bit comparisons |
| Sign GUI | Nine strictly pixel-equal scenes: empty/text, blinking cursor, line switching, backspace/Escape, four wall orientations |
| Paintings | 384 wall/direction/seed cases; selection, NBT round trips, overlapping paintings, 103 support ticks and damage; another 400 art/direction/action cases including repeated damage, movement, velocity, invalid art names and complete drop motion/RNG; 300 renderer cases / 283,140 bit comparisons |
| Maps | All 256 block palette entries; 480 terrain-update frames across five scales and two dimensions; player marker boundaries, removal and multiple holders; dirty column ranges; NBT sizes/scales, creation/missing data and signed-short ID rollover |
| Map saves | Java gzip map files loaded by Rust, saved through `World.save_level`, and loaded again by Java; uncompressed `idcounts.dat`, allocation and rollover. Compares NBT fields, not gzip implementation or compound tag ordering |
| Fishing | 30 casts / 48,000 replay ticks; complete position/bounds/rotation/motion, RNG and sound/particle events; water, rain, bites, fish items, hooked mobs, dead targets, ground impacts, distance/held-item line breaks, interpolation, rod durability, expired-hook reeling and recasting |
| Fishing rendering | 120 first/third-person, swing and partial-tick cases / 22,920 bit comparisons; the actual GPU line uses a 17-vertex line strip |
| Held items | Existing first-person oracle extended with cast-rod and third-person stick substitution cases; live maps use the existing map texture, marker and caption checks |

Gameplay and renderer checks are part of both the quick and full regression
suites. The GUI suite removes requested old sign captures before comparison.

## Vanilla details retained

- Sign placement uses solid material support and the original yaw rounding.
  Escape does not close `GuiEditSign`; Done closes it and marks the chunk dirty.
- Paintings test every art in enum order, draw once among every fitting art,
  and check support every 101 ticks. Movement and added velocity can drop an
  already dead painting again; repeated attacks cannot.
- Map terrain scanning retains column staggering, integer division, liquid
  depth counting, first-ID tie breaking, checker shading and Nether arithmetic.
  Marker rotation uses the current updating player's yaw for other holders;
  removing one holder skips its successor for that iteration.
- Map data is shared across dimension transfers, and map IDs wrap as signed
  Java shorts. Map colours and markers come from the simulation while playing.
- Fishing ground impacts leave tile coordinates at -1. The player retains an
  expired hook after it leaves the loaded entity list, so reeling it still damages
  the rod and permits another cast. Hooks are absent from the original entity
  save registry and are therefore not serialized into chunk entity lists.
- Catches record `fishCaughtStat` (2025); breaking a rod records its break-item
  statistic. A cast uses icon 85 in first person and a stick in third person.

## Commit fallback

The linked worktree's Git index is outside the writable sandbox. Stages are
snapshotted under `target/decoration-stages`; `target/commit-signs.sh` commits
those snapshots in order with the required author, committer and co-author
trailers. It does not replace the final working files.
