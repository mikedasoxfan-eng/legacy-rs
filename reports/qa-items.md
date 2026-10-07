# QA: items / blocks / screens (1.0 client, scripted play tests)

Scenarios: `crates/client10/src/playtest/items.rs`. Tests: `tools/playtest10/items_*.txt`
(run: `python tools/playtest10/run.py items_<name>`; screenshots in `target/playtest10/<test>/`).
All 16 tests pass (plus the existing `end_portal`) and every screenshot was inspected.

| test | result | bug found + fix |
|---|---|---|
| items_eye_of_ender | PASS: eye entity flies with portal particles, stack 4 -> 3 | none |
| items_ender_pearl | PASS: player teleported ~17 blocks south, fall damage taken | none |
| items_splash_potion | PASS: splash particles, Poison applied (inventory lists "Poison 0:14") | none |
| items_drink_potion | PASS: 32-tick drink, Speed 2:59 listed in inventory, empty bottle left | none (see harness note) |
| items_enchanting | PASS: table + 22 shelves, offers 6/15/18 at level 30, sword placed, offer 1 clicked: glint, level 30 -> 24, sword returns to the hotbar | none |
| items_brewing | PASS: wart + 3 water bottles -> after 600 ticks "Awkward Potion", wart consumed | none (1.0 brew time is 600, field_40056_b, not 400) |
| items_cauldron | PASS: bucket fills (level 3, empty bucket back); glass bottle gives a water bottle and lowers the level | none |
| items_nether_wart | PASS: wart planted on soul sand (stack 4 -> 3) | none |
| items_lily_pad | PASS: lily pad placed on the water | none |
| items_dragon_egg | PASS: left click teleports the egg (particles), it falls as a falling block elsewhere, the pillar stays | none |
| items_bow | PASS: draw animation, arrow flies and sticks, 16 -> 15 arrows | none |
| items_creative_inventory | PASS: creative screen ("Item selection", 9x8 grid + hotbar row), scroll, take diamond block into hotbar, place it | none |
| items_end_frame_eye | PASS: eye into the last empty frame completes the ring, 3x3 End portal appears | none |
| items_snow_golem | PASS: pumpkin on two snow blocks builds a snow golem | none |
| items_shears_mooshroom | PASS: mooshroom turns into a cow, drops mushrooms | none |
| items_milk_bucket | PASS: drinking leaves an empty bucket | none |

## Game bugs fixed
None: every item/block interaction in this area behaved like the original 1.0 Java. No panic,
hang or ignored event showed up in any log.

## Harness notes (test-side, no game change)
- The world thread answers screen-opening keys (E, right-click on a table) slowly while it is still
  meshing the first chunks, and more slowly still when another game window runs: an E at tick 15 was
  sometimes answered only seconds later. Tests therefore start interacting at tick >= 40 and leave
  generous gaps (about 40 ticks) before screenshots of screens.
- Aim values (`look 0 N`, N counts down): about 6.5 counts per degree; 190 ~ 29 deg down, 300 ~ 46 deg.
- GUI scale is 2 at 854x480: container slot pixels = 2 * (gui-left + slot x + 8). Hotbar slot i of a
  normal container is at pixel (282 + 36 i, 374); the creative screen is taller (hotbar at y 414,
  grid slots at (282 + 36 c, 82 + 36 r)).
- Mobs wander, so the mooshroom test fences its cell in (fence ray box is 1.0 high, the aim passes over it).
