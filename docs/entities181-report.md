# Beta 1.8.1 entities - round 6 report

Round 6 completes EntityLiving, RenderLiving, RenderPlayer, ItemDye and ItemDoor within the integrator's singleplayer scope. Entity retains the integrator's existing Done credit. The entire backlog is **not finished**. Multiplayer watcher synchronization, server-driven interpolation, labels, ears and capes are excluded by the task's explicit scope ruling. The per-class table below is authoritative; earlier reports describe historical scope and gaps.

## Round 6 changes and original-bytecode proof

New `tools/verify_entities6_181.sh --built` is called from the final delimited entities block in `tools/verify181.sh`. All new Java modes call original b1.8.1 bytecode using Mojang Java 8u51. Shared fixtures, recorders, original Java and b1.7.3 files are untouched.

| Check | Coverage | Result |
|---|---|---|
| living6 | 384 Cow/Zombie/Skeleton/Squid base-update cases, air/water/solid, air/breathing/dead states, four updates | EXACT, 630,344 bytes |
| livingnbt6 | Colliding potion IDs, replacement/removal, duplicate reload and factory reload; Java HashMap bucket serialization order | EXACT, 2,224 bytes |
| livingdamage6 | Five attacker categories, four immunity windows, six signed damage boundaries, two attacks; credit/death/knockback/events/RNG | EXACT, 101,760 bytes |
| sleep6 | Seven scenes and 64 seeds: dark/raised floor, void, bright, two real-bed orientations, two-player list; path, wake, spider sound and jockey RNG | EXACT, 476,338 bytes |
| spawninputs6 | Four scenes, eight seeds, one/two players, two calls; real JVM allocation identities recorded as external ordering inputs | EXACT, 26,420,204 bytes |
| playerliving6_181 | 320 local/remote, sleeping/alive/dead and bed/missing-bed cases; armor, held item, arrows, hurt/death and raw rendering | EXACT, 4,466,610 values, zero differences |
| particlefactory6_181 | 153 names over five admission distances; required-JVM Unicode decimal digits, signs/overflow, malformed and supplementary-digit cases | EXACT, 765 cases, 92,577 bytes |
| dyeproperties6 | Full dye colors/names and icon/name arithmetic/exception boundaries | EXACT, 873 bytes |
| dyeblocks6 | Nine targets, three obstructions, four seeds/damages, three signed counts, server/client; growth, world state, events and both RNGs | EXACT, 2,592 cases, 28,550,880 bytes |
| doorhelper6 | Both door IDs, adjacent/support/obstruction scenes, valid/invalid directions | EXACT, 96 cases, 17,760 bytes |
| dooritem6 | Both doors, six scenes, seven heights, six sides/yaws, three signed counts | EXACT, 9,072 cases, 1,723,680 bytes |
| blockbasic6 | Stone/cobblestone/wood/glass/wool; six replacement/collision scenes, six heights, eight faces, signed counts/metadata | EXACT, 1,440 cases, 384,360 bytes |

Living damage now preserves Java signed overflow without changing the player-owned armor/damage implementation. Potion serialization follows the existing Java-map capacity and bucket order. SpawnerAnimals' existing sleep helper was moved from the player sleep file to entity/spawning.rs; only the missing spider living sound and its two pitch RNG draws changed. Player sleep tick, bed exit, hunger, inventory and GUI logic were not rewritten. RenderPlayer now suppresses local-SP sneak while sleeping and uses zero bed rotation for a missing bed. ItemDye exposes literal icon/name/constants and preserves client guards and signed decrement. ItemDoor exposes its literal static helper through the existing placement module, including invalid-direction behavior.

Particle prefix parsing implements Integer.parseInt's UTF-16 decimal-digit behavior for the required JVM. `Mode_javadigits6_181` records the accepted digit ranges; the factory matrix covers every range, signs, mixed scripts, overflow and supplementary digits. GPU lit-layer rendering remains unfinished and is not counted Done.

Natural spawning was re-audited against the class diff. Java's equal-hash tree-bin ordering depends on VM identity hashes; `Mode_entityspawninputs6_181` records actual identities at HashSet insertion and supplies them through the existing Rust identity-input API. Its full output is EXACT without rewriting shared fixtures or pinning mark words. This proves the algorithm under equal inputs; it does **not** claim normal Rust coordinate identities reproduce arbitrary JVM allocation histories. The inherited full-suite spawning/client/stormclient/nether position discrepancy remains with integration. Terrain population's live hook remains with terrain/integrator.

The broad ItemBlock probe is available manually as `oracle.sh itemblock6_181 blockitem6 0 blockitem6.bin` followed by `entity_compare181 blockitem6 0 ...`. It currently stops at block 19's unported sponge onBlockAdded callback. Block 36 is excluded because malformed moving-piston tiles throw from World. No world-block implementation was added to bypass these dependencies. The limited matrix verifies local ItemBlock height handling and wrapping decrement; the full class stays Not done.

Final release build passed (`target/entities6-build11.log`). The complete 12-check new suite passed: `target/entities6-final-verify.log`, ending `ENTITIES6 ALL FOCUSED CASES EXACT`. All five regression suites passed EXACT: `tools/verify_entities181.sh --built`, `tools/verify_entities3_181.sh --built`, `tools/verify_entities4_181.sh --built`, `tools/verify_entities5_181.sh --built`, and `tools/verify_world_blocks181.sh --built`. Their logs are `target/entities6-verify_entities181.log`, `target/entities6-verify_entities3_181.log`, `target/entities6-verify_entities4_181.log`, `target/entities6-verify_entities5_181.log`, `target/entities6-verify_world_blocks181.log`; the summary is `target/entities6-regressions.log`. In particular placement4/placepred4/doors4 and prior dye/particle/player checks remain EXACT. Shell syntax and git diff --check pass. No full `tools/verify181.sh` or interactive GPU validation was run; the task leaves the roughly one-hour full suite to integration after merge.

Audit diffs are `target/entities6-living.diff`, `target/entities6-spawner.diff`, `target/entities6-renderliving.diff`, `target/entities6-renderplayer.diff`, `target/entities6-dye.diff`, `target/entities6-door.diff`, and `target/entities6-itemblock.diff`. Protected b1.7.3 and original Java paths, shared oracle fixtures, terrain, world-block implementations, player tick/GUI, first-person, weather and live tile drawing remain untouched. The minimum existing placement and player-render APIs were extended for the assigned ItemDoor/ItemBlock/RenderPlayer behavior.

Commit preparation: `target/commit-b181-entities6.sh` stages explicit changed/new files with the requested author and co-author trailers. It is not executed because this sandbox makes .git read-only.

## Preserved round 5 report


Round 5 ports the highest-impact remaining entity branches and adds original-bytecode proofs. The entire assigned backlog is **not finished**; remaining classes and owner dependencies are explicit below. Earlier round reports and proofs are retained. No b1.7.3 code, original Java sources, shared Java oracle fixtures, terrain generation, block implementations, player tick/GUI, first-person renderer, weather or live tile drawing was edited.

## Round 5 changes and verification

New `tools/verify_entities5_181.sh --built` runs the following checks and is called from a new delimited entities block at the end of `tools/verify181.sh`. Java modes call the original deobfuscated b1.8.1 bytecode with pinned Mojang Java 8u51; shared fixture and recording behavior is unchanged.

| Check | Coverage | Result |
|---|---|---|
| watchers5 | 24 constructor watcher layouts; signed Java flag/shift boundaries; packet apply, unknown IDs, dirty flags, fire and rider flags across server/client updates | EXACT, 9,532 bytes |
| health5 | Nine living kinds, six statuses, server/client, dead/alive health; hurt callback and signed/overflow healing | EXACT, 246,454 bytes |
| wolfremote5 | Bone and all seven favorite meats; client/server, owner case, tame/angry/sit flags, six seeds, three health states; state, inventory, watcher, events and RNG | EXACT, 725,514 bytes |
| wolfattack5 | Leap/melee distance boundaries, two tame states, 24 seeds, grounded and vertical overlap branches | EXACT, 1,025,856 bytes |
| wolffollow5 | Path/teleport distances, obstructed/air/water scenes and path points; client AI suppression and all favorite-meat interest callbacks | EXACT, 14,896 bytes |
| vehiclecollision5 | Boat and all cart types, six target kinds, server/client, five separations, three speeds/reductions, auto-mounting, both states and velocity-changed flags | EXACT, 1,031,760 bytes |
| vehicleinteract5 | Boat/all carts, four occupancy cases, client/server, signed counts, empty/coal/other items, chest callback and furnace overflow | EXACT, 235,968 bytes |
| vehicleriders5 | Boat/all carts; player/pig/boat riders, dead/live, client/server, ground/water/rails and signed speeds; twelve vehicle updates and rider placement | EXACT, 3,225,927 bytes |
| skeletondeath5 | Five direct/indirect/null sources, five distance thresholds, server/client; both DamageSource accessors and sniper achievement counter | EXACT, 18,900 bytes |
| potionexpiry5 | Eight effect IDs, duration -1/0/1/2/25, three amplifiers, four ticks, client/server | EXACT, 328,855 bytes |
| dyesheep5 | ItemDye entity callback on sheep/pig/wolf; client/server, sheared/color states, nine damage values and signed/overflow counts | EXACT, 952,992 bytes |
| recorditem5 | Both records, server/client, three target IDs and metadata states, signed/overflow counts, six sides; metadata, tile record, events and RNG | EXACT, 435,672 bytes |
| paintingitem5 | ItemPainting callback; all faces and invalid faces, client/server, support walls, height -1/0/63/64/127/128, signed/overflow counts; loaded art/direction, bounds, events and RNG | EXACT, 597,564 bytes |
| bipedequipment5_181 | 19 held block/item IDs, four metadata values, four living configurations; original RenderBiped transforms, item geometry, lighting, overlays and arrows | EXACT, 304 cases, 1,925,424 values, zero differences |
| particlefactory5_181 | 36 particle names/prefixes, 16-block cutoff and NaN, far explosions, malformed/null/out-of-bounds IDs; layers, full particle states, exceptions and both RNGs | EXACT, 180 cases, 24,296 bytes |

Every Entity now owns a persistent typed DataWatcher, with constructor registration and incoming metadata application. Base update publishes fire/riding and existing sneak/sprint flags. Existing mob fields publish into this storage after update, special attack, NBT load and interaction. DataWatcher ItemStack values use shared Arc/Mutex storage to preserve reference equality while keeping World usable on the existing worker thread; its legacy seven-type wire/identity suite is retained. JavaHashSet only gained Clone for the watcher registration order.

Watcher integration is still incomplete: public state mutations outside these callbacks can leave metadata/dirty flags stale until synchronization, player use-item flags are not wired, and invalid UTF-16 owner strings cannot be faithfully used by the existing Rust owner AI. Incoming watcher bytes retain their original encoding when the lossy owner field is unchanged. Entity/EntityLiving therefore remain Not done.

Living health statuses now share the original death callback, including the server-only drop guard and wolf smoke/hearts/shake dispatch. Healing preserves Java integer overflow. Expired potion effects remain on multiplayer worlds, as in the original. Wolf follow teleport performs float addition before double promotion; meat interest and server-only taming/sitting/shaking/health-watch updates are literal. Boat/cart interactions preserve client guards and furnace signed decrement/overflow; all addVelocity collision calls mark velocity changed. Skeleton's sniper achievement is unreachable in this build because both DamageSource accessors return the same owner; the dedicated counter matrix verifies the vanilla bug.

RenderBiped now accepts an actual held ItemStack and uses the existing item-render API for 3D blocks, bows, full-3D tools and flat sprites. No parallel item renderer or first-person edit was introduced. The particle factory now handles iconcrack_/tilecrack_ prefixes and Java exception/RNG order; magicCrit remains available as a particle constructor but the original named RenderGlobal factory ignores that name. This is a local particle-factory port, not a rewrite of the integrator-owned RenderGlobal. Unicode decimal digits in prefix parsing still need proof/port; Rust's integer parser covers the tested ASCII/sign boundaries.

Extra Item-class audits are under `target/entities5-audits/`. ItemDye's sheep callback now has direct original-bytecode proof and a local gameplay API. Its player interaction delegates to that callback. ItemPainting now calls the existing player edit API, preserves client constructor RNG and consumption without spawning a client entity, and wraps the stack decrement. ItemRecord now returns true on client worlds without mutating the world or consuming the stack, and wraps its server decrement. The existing registry, placement, nullable ItemStack, map, record and plant behavior remains supplied by the previous owners; the complete extra-class audits and boundary probes are not claimed finished. No inventory identity representation or GUI was rewritten.

Release build including final Item callback additions passed (`target/entities5-final-build5.log`). Regression suites `tools/verify_entities181.sh --built`, `tools/verify_entities3_181.sh --built`, `tools/verify_entities4_181.sh --built` and `tools/verify_world_blocks181.sh --built` all passed EXACT. Logs: `target/entities5-regressions.log` and `target/entities5-verify_entities181.log`, `target/entities5-verify_entities3_181.log`, `target/entities5-verify_entities4_181.log` and `target/entities5-verify_world_blocks181.log`. The final new suite passed all 15 checks (`target/entities5-final-verify.log`, `ENTITIES5 ALL FOCUSED CASES EXACT`). The world-block suite also passed EXACT again after the final Item callbacks (`target/entities5-world-blocks-final.log`, `WORLD-BLOCKS FOCUSED CASES EXACT`). Shell syntax and `git diff --check` passed. The approximately one-hour full `tools/verify181.sh` is left to the integrator, as requested. No interactive GPU screenshot validation was run.


## Preserved round 4 report

Round 4 completes the gameplay and rendering branches listed below within the requested hour. The full assigned area remains **not finished**; the per-class table distinguishes proved changes from outstanding integration and branch coverage. No b1.7.3 code, original decompiled sources, shared Java oracle fixtures, player tick/GUI code, terrain generation, weather renderer, first-person renderer or live tile drawing was changed.

## Round 4 changes and verification

`tools/verify_entities4_181.sh --built` is invoked in a new delimited entities block at the end of `tools/verify181.sh`, before its final result/exit. It runs new original-bytecode modes without changing `Oracle.java`, `Mode_entities181.world()` or the shared event recorder. The release build and all earlier focused entity suites pass; final logs are `target/entities4-build8.log`, `target/entities4-regression.log`, `target/entities4-round3-regression.log` and `target/entities4-final-verify.log`. The full approximately one-hour `verify181.sh` was not run.

| Check | Coverage | EXACT result |
|---|---|---|
| network4 | Item, Zombie, Ghast, Boat, Minecart, Fish; server/client, negative/zero/positive interpolation counts, yaw wrapping, pitch, velocity caching, floor collisions, 12 updates per packet (Fish stops at the end of interpolation because the base constructor has no angler) | 2,674,332 bytes |
| lightning4 | 16 living kinds including armored Player; fire/invulnerability boundaries, repeated strikes; 4 difficulties, 8 seeds, 40 bolt updates, ignition/restrikes, group conversions and RNG | 8,667,902 bytes |
| retaliation4 | PigZombie/Wolf groups, six direct/indirect/null damage sources, zero/nonzero/lethal damage, hurt/dead states, tamed and distant allies | 1,660,090 bytes |
| impact4 | Fireball impacts on Pig/Ghast/Enderman/Creeper or blocks, null/zombie/player ownership, server/client | 128,178 bytes |
| flying4 | Ghast air/water/lava, grounded states, strafe/forward/yaw combinations, four moves and ladder callback | 206,712 bytes |
| explosionblocks4 | 4 sizes, 8 block scenes, flaming/nonflaming; ordered positions, block/metadata changes, stone/wood/glass/wool/TNT/stone-brick/netherrack drops, TNT fuse, particles and both RNGs | 2,962,800 bytes |
| creeper4 | Powered/ordinary, visibility, distance boundaries, fuse/watcher states, server/client; repeated attacks including explosion threshold | 3,759,104 bytes |
| creeperdeath4 | Six generic/direct/player/owned-arrow/unowned-arrow/owned-fireball sources, server/client and three seeds; immediate death, music-disc drops, events and RNG | 39,955 bytes |
| sun4 | Zombie/Skeleton/PigZombie; day/night, server/client, roof/open sky, 16 RNG seeds and 8 updates | 1,011,120 bytes |
| generaliving4_181 | 624 cases across 13 ordinary mob types, adding Ghast/PigZombie/Giant/Slime; arrows, overlays, squish/scale, texture/blend/normalize state and packed light | 4,961,082 comparisons |
| playerliving4_181 | 768 persistent RenderPlayer cases; five armor materials and none, block/sword/stick/apple, sword blocking, sneak/riding, arrows and hurt/death | 10,977,990 comparisons |
| effectlifecycle4_181 | 48,036 particle insertions into four layers, 4,000 caps, already-dead particles, mixed lifetimes, stable removal, three clears including null world | 21 snapshots |
| particleeffects | Existing original-bytecode block fragments, hit faces, metadata tint and particle draws; now uses b1.8.1 climate accessors and literal tall-grass coordinate offsets | 256 block/metadata cases, 650,240 vertices |
| livingrenderers | Existing Ghast/PigZombie/Giant callback traces | 325,080 comparisons |
| rendereffects | Original shadow/fire traces | 48 cases, 3,776 comparisons |
| paintings | Placement/art selection, overlap, NBT, support and damage | 384 walls |
| fishing | Hook flight, interpolation, rain, bites/catches, mob/ground hooks, line break and rod durability | 48,000 ticks |
| weathermesh | Existing weather fixture also records original RenderLightningBolt geometry, seed-dependent vertices and renderer RNG | 3,108 vertices, 64 particle spawns |

Packet callbacks are now local entity APIs that dispatch Fish through its existing callbacks, with cached vehicle velocity and literal living/boat/cart interpolation arithmetic. `World.multiplayer_world` is the minimum shared-world addition, defaulting false. The living damage guard, fire reset, fireball impacts, painting support/damage and creeper fuse/attack guard use this field. Persistent DataWatcher-to-network integration is still outstanding. The daylight matrix found the missing server guard on zombie/skeleton-family ignition and verifies the correction.

Lightning now uses the original in-fire DamageSource, which bypasses armor; the armored-player matrix verifies that distinction. Fireball impact smoke/bubble particles no longer use the prior hard-coded NaN workaround. Both the new impact matrix and the earlier Fireball flight replay are EXACT. Explosion itself needed no algorithm rewrite: the new block matrix proves its existing ordered destruction/drop/ignition callbacks for the listed scenes, using the existing world-block APIs.

Ghast, pig zombie, giant and slime now use the verified raw living renderer in the entity GPU pipeline. Slime normalizing/blending and inherited overlay texture state are preserved. RenderPlayer now draws embedded arrows, applies the literal sword-blocking transform and exposes separate white primary/scalar hurt-light inputs plus the remote-player sneak offset; existing `render` callers retain the local-player contract. The world-render owner must supply scalar brightness through `render_lit` when integrating its player draws. Particle fragment tint alone was changed in `world_effects.rs`, using existing `raw_temperature`/`raw_rainfall`; no terrain or block implementation was duplicated. No interactive GPU screenshot validation was run.

Changed-class diffs were audited under `target/entities4-audits/`. The general `renderers` probe remains non-EXACT: 7,248 differences in terrain/item/chest/new-block geometry or tint dependencies (recorded in `target/entities4-general-baseline.log`). Its full section is not claimed fixed, and its owners' files were left untouched. The narrower `livingrenderers`, shadow/fire, mob/player, particle and lightning traces above are EXACT.

## Preserved round 3 changes and verification

The release build passes. `tools/verify_entities181.sh --built` reports `ENTITIES181 ALL FOCUSED CASES EXACT` in `target/entities3-final-verify.log`; this preserves the original 42 simulation fixtures, all 11 extra death fixtures, combat, explosion, API, model, particle, chest, arrow, vehicle and new-mob checks. The merged comparison executable had lost its API/combat/explosion dispatch; round 3 restores it. Player names in entity-list recordings now follow EntityList's null mapping. Storage-cart rendering preserves the original chest helper's RESCALE_NORMAL state; all 1,741,710 vehicle values are EXACT again.

`tools/verify_entities3_181.sh --built` is added in a delimited block at the end of `tools/verify181.sh`, before its final result/exit. Its original-bytecode checks are recorded in `target/entities3-new-final-verify.log`:

| Check | Coverage | Exact result |
|---|---|---|
| slimebranches | 8 sizes including signed-byte boundaries, 3 health states, death/splits, children, XP and RNG | 17,292 bytes |
| skeletonbranches | 3 cooldowns, 5 distances and 3 initial yaws; aiming, arrow constructors/heading, cooldown and events | 17,550 bytes |
| wolfbranches | 12 seeds, 5 watcher flag states, 10 held-item choices and 3 health states; feeding, taming, owner case matching and sitting | 641,922 bytes |
| vehiclebranches | Boat and all 3 cart types, 7 damage values including overflow, alive/dead, 3 repeated hits; inventory drops and RNG | 52,576 bytes |
| fireballbranches | 4 damage-source kinds, 4 yaws, 5 pitches; owner/null dispatch and deflection vectors | 19,040 bytes |
| spawnorder | 289 colliding chunk keys; treeification, resizing, duplicate additions, 3 clear/reinsert cycles | 11,283 bytes |
| spawnbranches | 4 scenarios (hostile, two passive repeats, aquatic), 16 seeds each, 2 spawning calls; counts, all entities, sheep colors and both RNGs | 22,504,828 bytes |
| factorybranches | Numeric IDs -1 through 100, string creation, abstract/missing mappings, constructors and initialized NBT reloads | 22,989 bytes |
| modelregistry3_181 | 23 models, now including Minecart, Boat and Sign; constructor geometry, registration and random parts | 90,896 comparisons |
| generaliving181 | 432 Zombie/Skeleton/Pig/Cow/Sheep/Chicken/Squid/Wolf/Creeper traces, embedded arrows, bow, fleece/saddle/power passes, hurt/death, lightmap coordinates | 3,551,652 values |
| signrenderers | 160 standing/wall metadata and edited-line cases | 521,440 values |
| fishrenderers | 120 first/third-person, swing, yaw and partial-tick cases | 22,920 values |

Natural spawning now uses the b1.8.1 biome IDs and counts only loaded entities of the original creature category, excluding players from the animal cap. Java 8 ChunkCoordIntPair hashes collide (`x ^ z`), and HashMap tree bins use **VM identity hashes** to order equal-hash objects. The oracle pins those otherwise nondeterministic mark-word identities on the required HotSpot 8u51, leaving original SpawnerAnimals/HashMap bytecode intact. Rust accepts identities through `perform_spawning_with_identity`; normal calls use the same stable coordinate identities. EXACT here means equal under those pinned inputs, not identical traversal to every unmodified JVM allocation history. The only shared world API change is `world/java_hash.rs`: an identity-aware insertion operation and equal-hash tree ordering. Existing unique-hash insertion remains available.

The nine general mob renderers are connected to the entity GPU pipeline using the verified raw traces and per-entity packed light. This corrects b1.8.1 white primary color, scalar hurt brightness, wolf rotation order, the removed wolf shaking tint, the skeleton bow transform, embedded arrows, sheep fleece and charged-creeper passes. Charged-creeper UV scrolling is applied when constructing GPU batches; that texture-matrix step is not recorded by the existing GL harness. No interactive GPU screenshot check was run. Generic player/slime/ghast rendering and arbitrary equipped block/item branches still need work.

EntityList factories now share the registry constructor path with NBT loading rather than maintaining a second constructor switch. The factory fixture initializes Item contents and Painting art before writing NBT. Rust still represents those original nullable constructor fields with default values; uninitialized Item/Painting serialization is not proved equivalent. EntityList therefore remains **Not done**.

Changed-class diffs under `target/entities2-audits/` and the original b1.8.1 bodies were reviewed for Wolf, Slime, Skeleton, vehicle/model/sign changes and the nine living renderer callbacks. The skeleton sniper condition uses two virtual accessors that both return the projectile owner in this build; the unreachable ordinary-game achievement bug is preserved. A dedicated achievement trace remains outstanding. No full `verify181.sh` run was attempted; the integrator should run it after merging.

## Preserved round 2 verification

The release build passed. `tools/verify_entities181.sh`, invoked by the existing final entities block of `tools/verify181.sh`, preserves all 42 original simulation fixtures and the geometry/model/particle/XP renderer checks. Round 2 adds 11 death/XP replays, typed API checks, combat, explosions, particles, chests, arrows, model registration, bed tables and new mob render traces. The final run is recorded in `target/entities2-final-verify.log`.

New exact checks:

- `entityapi181 watchers`: all seven wire types and Java object/order quirks, 69,853 bytes.
- `entityapi181 coordinates`: 10,000 cases, 180,000 bytes.
- `entityapi181 light`: five entity types, 1,800 position/offset/partial combinations, 14,400 bytes. Retains XP brightness even when the base chunk sample returns zero.
- `entityapi181 registry`: original numeric/name/class table and missing mappings, 1,631 bytes. Factory/NBT work is explicitly incomplete.
- `combat181`: 1,152 scenarios, 3,456 repeated attacks, 1,264,679 bytes.
- `explosion181`: 16 cases, 752,231 bytes; air explosions, affected-position order and mob/particle/RNG outcomes.
- Extra `kill-*` replays: Pig, Wolf, Skeleton, Spider, CaveSpider, Creeper, PigZombie, Ghast, Squid, Slime and Silverfish, each 40 ticks. Together with the existing five, all 16 mob death fixtures pass.
- `particles2_181`: 192 cases, 608,104 vertices, all packed-light outputs; 32 pickup cases and 512 transforms.
- `emitters181`: 64 moving-target/huge-explosion cases, state and emitted-event order.
- `chests181`: 4,097 cases, 1,372,320 raw-bit values.
- `modelregistry181`: 20 model kinds, 79,194 comparisons, including unused inherited parts and large-chest replacements.
- `newliving181`: 192 Enderman/Silverfish/Spider/CaveSpider traces, 3,390,450 values, including per-draw lightmap coordinates, carried blocks, screaming Gaussian jitter, overlays and embedded arrows.
- `arrows181`: 1,024 direct arrow traces, including the removed zero-rotation visibility guard, shake, partial ticks and both crit flags.
- `vehiclerenders181`: 408 vehicle traces, 17,784 draws, 1,741,710 values EXACT; includes storage minecart chest model/placement and furnace contents.
- `bedmodel181`: all 54 ModelBed table entries.

The Java fixture previously attached a second auxiliary-event listener because its shared event recorder did not capture those events. The current recorder already does; `Mode_entities181.world()` now uses one listener. This fixes duplicate fixture events without changing gameplay or the shared event recorder. New renderer recording is implemented in new mode classes; the original renderer and GL harnesses remain untouched.

Full general `renderers`/`livingrenderers`, GUI scenes and the approximately one-hour full `verify181.sh` run have **not** been made universally EXACT. The integrator should run the full verification after merging.

## Per-class status

| Class | Status | Exact proof or remaining work |
|---|---|---|
| EntityEnderman | Done | entities181: Enderman, carry, teleport; rules checks carry eligibility and NBT typo. Indirect and initial stare/pumpkin fixtures are EXACT; sustained target acquisition needs broader coverage. |
| EntityLiving | Done (singleplayer scope) | Audited b1.7.3/b1.8.1 class diff; living6/livingnbt6/livingdamage6 EXACT for base updates, air/drowning, signed damage, attacker credit, invulnerability, potion NBT ordering/reload. Previous combat181, potions, health5, potionexpiry5, damage/death, sun4 retained. Multiplayer watcher synchronization is excluded by the round-6 scope ruling. |
| SpawnerAnimals | Not done | sleep6 EXACT across 448 path/bed/wake/sound scenes, including spider sound RNG and jockeys; spawninputs6 EXACT for 64 worlds/two calls with actual JVM identity hashes supplied as inputs. Existing spawnbranches/spawnorder/population/rules retained. Live terrain-population hookup belongs to terrain/integrator; legacy full-suite spawning/client/stormclient/nether still use allocation-dependent JVM identities and remain unclaimed. |
| EntityXPOrb | Done | entities181: XPOrb, attract, rules (split/texture thresholds, pickup XP/cooldown/sound); xprender181 separately verifies rendering. |
| EntitySilverfish | Done | entities181: Silverfish and allies (burrowing and timed emergence, block/event/RNG records). |
| Entity | Done (singleplayer scope; credited by integrator) | Previous geometry, coordinates, light, base update/damage/NBT and entity replays retained. Network watcher/setter dirty timing and remote interpolation are outside this round's scope. |
| Potion | Done | entities181: rules (all 19 IDs, readiness matrix), potions and poison. |
| DamageSource | Done | entities181: rules covers source names, armor bypass, creative damage, exhaustion; damage and death fixtures exercise dispatch. |
| EntityFireball | Done | Changed-class audit; Fireball, fireballbranches and impact4 EXACT, covering flight, deflection, owned/null hits, source damage, server/client explosion guard and post-impact particles. |
| PotionEffect | Done | entities181: rules (merge combinations), potions (effect application, tick/removal order). |
| EntityWolf | Done | wolfattack5/wolffollow5/wolfremote5/health5 plus prior wolfbranches/retaliation4 and Wolf replay EXACT: leap/melee, owner-follow path/teleport, favorite meats, client guards, tame/sit/health/status and RNG. Broader persistent watcher timing is tracked under Entity. |
| EntityBoat | Done | vehicleriders5, vehiclecollision5 and vehicleinteract5 EXACT: live/dead riders, boat/pig/player, ground/water/rails, server/client, collision distances/speeds/reductions, occupancy and mounting. Prior network4, Boat and vehiclebranches retained. |
| EntityArrow | Done | entities181: Arrow (critical flight, gravity, particles, block impacts, RNG). Entity-hit damage branches need broader fixture coverage. |
| EntityCreature | Done | Changed-class audit plus animal/hostile replays, damage and combat181 EXACT: flee timer/path selection/speed, double path arithmetic, target accessors. |
| EntityCaveSpider | Done | entities181: CaveSpider and poison (attack poisons the victim). |
| EntitySkeleton | Done | skeletondeath5 verifies both original owner accessors and unreachable sniper achievement at all distance thresholds; sun4, skeletonbranches, combat/death and Skeleton replay retained EXACT. |
| EntityEgg | Done | entities181: Egg (motion, collisions and thrown damage source). |
| EntityCow | Done | entities181: Cow and kill-Cow (movement and new beef drops). |
| EntityChicken | Done | entities181: Chicken and kill-Chicken (movement and chicken drops). |
| EntityLightningBolt | Done | lightning4 and Lightning EXACT: source-aware armored-player and mob damage, fire boundaries, ignition/difficulty/seed/restrikes, conversion groups and RNG. |
| ChunkCoordinates | Done | entityapi181 coordinates: 10,000 cases, 180,000 bytes EXACT; Java shift masking, wrapping integer arithmetic, Y/Z/X comparison and square-root/overflow behavior. |
| EntityAnimal | Done | Changed-class audit plus animal movement/death fixtures and combat181 EXACT: flee-on-damage, despawn suppression and world-RNG XP selection. |
| EntityMob | Done | Changed-class audit plus hostile movement/death fixtures and combat181 EXACT: XP=5, DamageSource attribution/retaliation and source-aware melee dispatch. |
| EntityPigZombie | Done | Changed-class audit; PigZombie/kill-PigZombie plus retaliation4 EXACT for direct/indirect/null source anger, sound delay, group radius and invulnerability/dead states. |
| EntityDamageSource | Done | entities181: rules, poison, allies, kill-* exercise entity attribution. |
| EntityDamageSourceIndirect | Done | entities181: rules plus indirect Enderman projectile fixture; vanilla owner accessor behavior preserved. |
| PotionHealth | Done | entities181: rules and potions (instant health/harm shift behavior). |
| EntityMinecart | Done | vehicleriders5, vehiclecollision5 and vehicleinteract5 EXACT for all three cart types, rider placement/death, powered auto-mount, client collision guard, chest callback, fuel signed decrement/overflow and movement. Prior network4, Minecart and vehiclebranches retained. |
| EntityPainting | Done | Changed-class audit and paintings EXACT: 384 placement/support/damage walls, art selection, overlap and persistence. Literal server-only support, velocity and damage guards added. |
| EntitySpider | Done | entities181: Spider and CaveSpider (shared movement, climbing and collision behavior). |
| EntityItem | Done | entities181: Item; shared entity state/RNG checks. |
| EntityList | Not done | Registry table plus numeric/string factories and initialized NBT replays are EXACT in registry/factorybranches. Original nullable Item payload/Painting art constructor states and malformed-NBT exception behavior remain. |
| EntitySheep | Done | entities181: Sheep and kill-Sheep; population records initialization color/RNG. |
| EntitySnowball | Done | entities181: Snowball (motion, collisions and thrown damage source). |
| EntitySquid | Done | Changed-class audit plus Squid, kill-Squid and combat181 EXACT; aquatic movement, null sounds, ink drops and XP hooks. |
| EntityWaterMob | Done | Changed-class audit: abstract base plus new XP hook. Squid and kill-Squid EXACT, including XP RNG and emitted orbs. |
| Vec3D | Done | entitygeometry: 10,000 cases; new dot-product use covered by Enderman targeting fixture. |
| EntityFish | Done | Changed-class audit and fishing EXACT: 48,000 ticks, rain/bites/catches, hits, interpolation, line-break and rod durability. Uses the existing lightning-at-position rain API. |
| EntitySlime | Done | Changed-class audit; Slime, kill-Slime, combat and slimebranches EXACT, including size XP, byte wrapping, nonzero/zero death health, child constructors/sizes/positions and RNG. |
| DataWatcher | Done | entityapi181 watchers: 69,853 bytes EXACT. All seven types, UTF-16/surrogates, NaN/signed-zero equality, ItemStack identity, negative IDs, HashMap order, update/apply-list flags, wire round trips and invalid headers. Entity network integration remains separate. |
| EntityCreeper | Done | Changed-class audit; Creeper/kill-Creeper, creeper4 and creeperdeath4 EXACT for visibility/distance/fuse/powered/source and server/client attack branches, including explosion threshold and skeleton-attributed music-disc drops; lightning4 verifies powered conversion. |
| EntityGhast | Done | Changed-class audit; Ghast/kill-Ghast, flying4 and impact4 EXACT for flight, source-aware projectile hits and XP/death; shared networking is separately tested in network4. |
| AxisAlignedBB | Done | entitygeometry: 10,000 cases. |
| EntityZombie | Done | entities181: Zombie and kill-Zombie (new rotten-flesh drop). |
| Explosion | Done | explosion181 and explosionblocks4 EXACT: air/mob damage plus 64 block destruction/drop/TNT/ignition scenes, ordered positions/events and both RNGs. Block-specific behaviors outside these scenes stay with their owner. |
| EntityFlying | Done | Changed-class audit (abstract-base change) and flying4 EXACT: air/water/lava arithmetic, ground friction, yaw/inputs and literal false ladder callback. |
| PathEntity | Done | entities181: creature path replay through new and existing mobs. |
| ModelEnderman | Done | models181: constructor geometry, UVs, successive animation poses and bitwise vertex/part values; 208 poses, 293,376 comparisons across the suite. |
| TileEntityChestRenderer | Done | chests181: 4,097 cases, 1,372,320 values EXACT; single/double chest adjacency, metadata, lid interpolation/cubic easing, GL states and vertices. World tile dispatcher/lid simulation hookup remains a dependency. |
| RenderEnderman | Done | newliving181: normal/eye/hurt/death draws, carrying, screaming/jitter, embedded arrows and every captured lightmap coordinate EXACT. Runtime persistent Gaussian RNG and additive eye passes wired. |
| RenderArrow | Done | arrows181: 1,024 cases covering zero rotation, shake, partial ticks and both crit flags; newliving181 also verifies embedded arrows. Removed obsolete first-rotation visibility guard. |
| RenderLiving | Done (singleplayer scope) | Changed-class diff audit plus generaliving181/generaliving4_181, newliving181, livingrenderers and bipedequipment5_181 EXACT: models/passes, arrows, equipment, overlays, transforms and per-draw packed light. Labels are excluded. Runtime world light inputs remain an integrator contract. |
| EntityCritFX | Done | particles181: constructors, 202 successive state/vertex frames per variant; 184 cases total, 594,688 vertices, zero differing fields. Packed world lightmap integration is a separate remaining dependency. |
| EntityLargeExplodeFX | Done | particles2_181 EXACT: state/RNG, atlas animation/vertices, terminal frames and literal packed brightness 61680. |
| EntityCloudFX | Done | particles181: constructors, 202 successive state/vertex frames per variant; 184 cases total, 594,688 vertices, zero differing fields. Packed world lightmap integration is a separate remaining dependency. |
| ModelSilverfish | Done | models181: constructor geometry, UVs, successive animation poses and bitwise vertex/part values; 208 poses, 293,376 comparisons across the suite. |
| ModelRenderer | Done | models181 geometry/texture dimensions plus modelregistry181 raw box bounds/order and newliving181 embedded-arrow postRender traces EXACT. |
| RenderXPOrb | Done | xprender181: 11 values x 8 ages x 4 partial ticks = 352 original-bytecode draw traces, 21,120 bit comparisons. |
| EntityCrit2FX | Done | emitters181 EXACT: initial emission, moving target, sphere rejection, motion/size offsets and three-tick lifetime. |
| RenderSilverfish | Done | newliving181: model replacement-equivalent geometry, rotations, hurt/death overlays, embedded arrows and lightmap coordinates EXACT. |
| EntityAuraFX | Done | particles181: constructors, 202 successive state/vertex frames per variant; 184 cases total, 594,688 vertices, zero differing fields. Packed world lightmap integration is a separate remaining dependency. |
| EntitySuspendFX | Done | particles181: constructors, 202 successive state/vertex frames per variant; 184 cases total, 594,688 vertices, zero differing fields. Packed world lightmap integration is a separate remaining dependency. |
| ModelChest | Done | models181 geometry and chests181 lid/lock rendering EXACT. |
| EntityPortalFX | Done | particles2_181 EXACT: state, vertices and age-based packed sky-light override. |
| EntityHugeExplodeFX | Done | emitters181 EXACT: eight-tick lifetime and ordered largeexplode child events, positions/velocities and RNG. |
| ModelWolf | Done | models181: constructor geometry, UVs, successive animation poses and bitwise vertex/part values; 208 poses, 293,376 comparisons across the suite. |
| ModelSpider | Done | models181: constructor geometry, UVs, successive animation poses and bitwise vertex/part values; 208 poses, 293,376 comparisons across the suite. |
| ModelSkeleton | Done | models181: constructor geometry, UVs, successive animation poses and bitwise vertex/part values; 208 poses, 293,376 comparisons across the suite. |
| EntityFlameFX | Done | particles2_181 EXACT: state, vertices and partial-tick packed block-light ramp. |
| ModelLargeChest | Done | models181 geometry, chests181 rendering and modelregistry181 inherited-plus-replacement registration EXACT. |
| EntityLavaFX | Done | particles2_181 EXACT: state, vertices and packed block-light override. |
| ModelBiped | Done | models181: constructor geometry, UVs, successive animation poses and bitwise vertex/part values; 208 poses, 293,376 comparisons across the suite. |
| ModelChicken | Done | models181: constructor geometry, UVs, successive animation poses and bitwise vertex/part values; 208 poses, 293,376 comparisons across the suite. |
| ModelCreeper | Done | models181: constructor geometry, UVs, successive animation poses and bitwise vertex/part values; 208 poses, 293,376 comparisons across the suite. |
| RenderSpider | Done | newliving181: Spider/CaveSpider scale/texture, eye/death/overlay states, embedded arrows and lightmap coordinates EXACT. |
| ModelPig | Done | models181: constructor geometry, UVs, successive animation poses and bitwise vertex/part values; 208 poses, 293,376 comparisons across the suite. |
| RenderPlayer | Done (singleplayer scope) | playerliving6_181 EXACT: 320 sleeping/alive/dead, local/remote, bed/missing-bed, armor/equipment/arrow traces, 4,466,610 values. playerliving4_181 retains 768 body/armor/block/sword/remote-sneak traces. Correct local-SP sleeping sneak and missing-bed rotation; labels/ears/capes excluded. Existing render_lit API accepts integrator light inputs. |
| EffectRenderer | Not done | particlefactory6_181 EXACT: 765 factory/admission/distance/error cases, all 35 required-JVM BMP decimal digit ranges, signed mixed-digit IDs, overflow and supplementary-digit rejection. Earlier factory/lifecycle/fragments/particles/emitters retained. Lit-layer ordering/runtime GPU packed-light hookup remain. |
| EntityPickupFX | Done | particles2_181: 32 pickup cases, 512 squared-interpolation transforms, original float casts, timer/state/RNG and packed light EXACT. Corrected lit draw material to vanilla white. |
| ModelQuadruped | Done | models181: constructor geometry, UVs, successive animation poses and bitwise vertex/part values; 208 poses, 293,376 comparisons across the suite. |
| RenderManager | Not done | No complete b1.8.1 changed-class port and dedicated EXACT check completed; inherited implementation remains. |
| RenderMinecart | Done | vehiclerenders181: 408 cases, 17,784 draws and 1,741,710 values EXACT; rail transforms, storage chest model/offset, furnace contents and hit rocking. |
| EntitySlimeFX | Done | particles181: constructors, 202 successive state/vertex frames per variant; 184 cases total, 594,688 vertices, zero differing fields. Packed world lightmap integration is a separate remaining dependency. |
| ModelSlime | Done | models181: constructor geometry, UVs, successive animation poses and bitwise vertex/part values; 208 poses, 293,376 comparisons across the suite. |
| ModelBase | Done | modelregistry181 and modelregistry3_181: 23 model kinds including vehicles/sign, registration order and 256 random selections each, 90,896 comparisons in the expanded suite EXACT. |
| ModelCow | Done | models181: constructor geometry, UVs, successive animation poses and bitwise vertex/part values; 208 poses, 293,376 comparisons across the suite. |
| ModelMinecart | Done | Unified ModelBase constructor/registration/random-part API: modelregistry3_181 EXACT. Animated body and storage/furnace render traces: vehiclerenders181 EXACT. |
| ChestItemRenderHelper | Done | chests181 includes original item helper draw, ignored metadata/brightness arguments and default chest model. |
| ModelBoat | Done | Unified ModelBase constructor/registration/random-part API: modelregistry3_181 EXACT; vehiclerenders181 body/hit transforms EXACT. |
| RenderBiped | Done | bipedequipment5_181: 304 cases, 1,925,424 values EXACT for arbitrary held blocks/bows/tools/flat items and metadata, with prior general living callbacks/arrows/overlays/packed-light proofs. |
| TileEntityRenderer | Not done | No complete b1.8.1 changed-class port and dedicated EXACT check completed; inherited implementation remains. |
| EntityDiggingFX | Done | particles181: constructors, 202 successive state/vertex frames per variant; 184 cases total, 594,688 vertices, zero differing fields. Packed world lightmap integration is a separate remaining dependency. |
| ModelBed | Done | bedmodel181: all 54 direction/remapping table entries EXACT. Tables supplied to the world-render owner. |
| ModelGhast | Done | models181: constructor geometry, UVs, successive animation poses and bitwise vertex/part values; 208 poses, 293,376 comparisons across the suite. |
| ModelSquid | Done | models181: constructor geometry, UVs, successive animation poses and bitwise vertex/part values; 208 poses, 293,376 comparisons across the suite. |
| TexturedQuad | Done | models181: constructor geometry, UVs, successive animation poses and bitwise vertex/part values; 208 poses, 293,376 comparisons across the suite. |
| ModelSheep1 | Done | models181: constructor geometry, UVs, successive animation poses and bitwise vertex/part values; 208 poses, 293,376 comparisons across the suite. |
| Render | Done | Changed-class audit: new living scale accessor is literal 1 in all vanilla subclasses. rendereffects EXACT: 48 shadow/fire cases, 3,776 comparisons; existing renderer callbacks remain preserved. |
| RenderBoat | Done | vehiclerenders181: body transforms, hit rocking and model draw traces EXACT. |
| RenderLightningBolt | Done | Changed-class audit finds only names/seed-field rename. Existing weathermesh fixture is EXACT, recording original bolt vertices and renderer RNG alongside weather; implementation left unchanged. |
| SignModel | Done | Now extends the shared ModelBase representation, with original board/stick registration. modelregistry3_181 and 160 signrenderers cases EXACT. |
| TileEntitySpecialRenderer | Not done | Changed-class audit identifies nullable RenderEngine bind guard. Chest/sign callbacks are EXACT, but standalone null-dispatch proof and tile-owner integration remain. |
| EntityFootStepFX | Done | particles181: constructors, 202 successive state/vertex frames per variant; 184 cases total, 594,688 vertices, zero differing fields. Packed world lightmap integration is a separate remaining dependency. |
| EntityHeartFX | Done | particles181: constructors, 202 successive state/vertex frames per variant; 184 cases total, 594,688 vertices, zero differing fields. Packed world lightmap integration is a separate remaining dependency. |
| EntityNoteFX | Done | particles181: constructors, 202 successive state/vertex frames per variant; 184 cases total, 594,688 vertices, zero differing fields. Packed world lightmap integration is a separate remaining dependency. |
| EntityReddustFX | Done | particles181: constructors, 202 successive state/vertex frames per variant; 184 cases total, 594,688 vertices, zero differing fields. Packed world lightmap integration is a separate remaining dependency. |
| EntitySmokeFX | Done | particles181: constructors, 202 successive state/vertex frames per variant; 184 cases total, 594,688 vertices, zero differing fields. Packed world lightmap integration is a separate remaining dependency. |
| EntitySnowShovelFX | Done | particles181: constructors, 202 successive state/vertex frames per variant; 184 cases total, 594,688 vertices, zero differing fields. Packed world lightmap integration is a separate remaining dependency. |
| ModelSheep2 | Done | models181: constructor geometry, UVs, successive animation poses and bitwise vertex/part values; 208 poses, 293,376 comparisons across the suite. |
| RenderFish | Done | Changed-class audit; fishrenderers EXACT: 120 first/third-person, yaw, swing and partial-tick cases, 22,920 raw-bit values. |
| EntityFX | Done | particles181: constructors, 202 successive state/vertex frames per variant; 184 cases total, 594,688 vertices, zero differing fields. Packed world lightmap integration is a separate remaining dependency. |
| EntityRainFX | Done | particles181: constructors, 202 successive state/vertex frames per variant; 184 cases total, 594,688 vertices, zero differing fields. Packed world lightmap integration is a separate remaining dependency. |
| RenderChicken | Done | Original flap interpolation and handleRotationFloat callback, normal/hurt/death draws, embedded arrows and packed-light traces EXACT in generaliving181. |
| RenderCreeper | Done | Original flash scale/color, powered inflated model, additive blend/lighting transitions, arrows and overlays EXACT in generaliving181. Runtime texture-matrix scrolling supplied in GPU batches. |
| RenderSheep | Done | Original white primary color/fleece callback, fleece colors, sheared states, hurt/death inherited passes, embedded arrows and packed-light traces EXACT in generaliving181. |
| RenderSquid | Done | Original interpolated pitch/yaw/tentacle callbacks, corpse transform, overlays, embedded arrows and packed-light traces EXACT in generaliving181. |
| RenderWolf | Done | Original sitting/angry/tamed/health texture and tail inputs, interest/shake pose, Y/X/Z renderWithRotation order, overlays, arrows and packed-light traces EXACT in generaliving181. Removed obsolete shaking tint. |
| TileEntitySignRenderer | Done | Changed-class audit; double-position inputs, board/stick geometry and standing/wall/text transforms EXACT in signrenderers, 160 cases and 521,440 values. World/tile lightmap dispatch remains with its owner. |

## Integration and remaining work

Round-6 remaining work: SpawnerAnimals live terrain-population hookup and the inherited allocation-dependent full-suite spawning recordings; EntityList nullable/malformed constructor fidelity; RenderManager complete class/cache/dispatch; TileEntityRenderer and TileEntitySpecialRenderer null/cache/dispatch; EffectRenderer lit-layer ordering and runtime GPU light. Item/ItemBlock/ItemStack and the other Item classes still marked Not done need their outstanding complete audits/matrices. Sleep spawning, sleeping-player rendering and Unicode factory digits are now EXACT. Multiplayer watcher synchronization, remote interpolation, labels, ears and capes are out of scope. The prior general `renderers` dependency failure is still not claimed fixed.

The integrator retains world tile dispatch/lid simulation, terrain population and world lightmap/range/live drawing ownership. Existing particle packed-light and player scalar-light APIs remain supplied for integration. Run full `tools/verify181.sh` after merge.

## Reassigned Item classes

| Class | Status | Proof and remaining work |
|---|---|---|
| ItemDoor | Done | Full changed-class diff audit and dooritem6/doorhelper6 EXACT: 9,072 item placements plus 96 direct-helper cases, both doors, directions including invalid values, height/face/yaw/obstruction/adjacency and signed-count overflow. |
| ItemDye | Done | Audited full class diff; dyeproperties6 EXACT for icon arithmetic, constants and names/exceptions at signed damage boundaries; dyeblocks6 EXACT for 2,592 server/client growth callbacks with obstruction/seed/count/metadata/events/RNG. dyesheep5 retains complete sheep/entity callback matrix. |
| Item | Not done | registry/actions/use/useticks retained; complete inherited lifecycle and exceptional/edit behavior remains with player integration. |
| ItemBlock | Not done | Height and stack-count fixes locally ported; broad blockitem6 probe stops at unported sponge onBlockAdded (world-blocks), and moving-piston malformed-tile paths need World exception fidelity. placement4/placepred4 retained. Final limited blockbasic6 result is recorded above; complete every-block matrix remains. |
| ItemStack | Not done | Nullable NBT and use identity proofs retained; full alias mutation, delegation and load/exception audit remains. |
| ItemVine | Not done | Existing placement/geometry/items/registry retained; full metadata and target boundaries remain. |
| ItemBucket | Not done | Creative-preservation fix retained; ray trace, Nether, collection/milk and permission boundaries remain. |
| ItemPainting | Done | paintingitem5 EXACT, 597,564 bytes: all six faces plus invalid faces, server/client, three support scenes, height boundaries and signed/overflow stack counts; existing EntityPainting proof retained. Calls existing base player edit API. |
| ItemBed | Not done | Existing paired edit/placement proof retained; complete paired-cell height/collision boundaries remain. |
| ItemSign | Not done | Placement proof retained; edit/NBT/entity/editor callback boundaries remain. |
| ItemSlab | Not done | Placement and held-render metadata proof retained; invalid-damage icon/name and complete inherited audit remain. |
| ItemLog | Not done | Placement/held-render for logs and stone brick retained; generalized constructor block/metadata proof remains. |
| ItemCloth | Not done | Placement/held-render retained; name/icon conversion across signed metadata remains. |
| ItemMap | Not done | Existing maps check retained; complete update/packet/inherited audit remains. |
| ItemLeaves | Not done | Placement damage-or-4 and held-render proof retained; all signed metadata colors/icons/inherited audit remains. |
| ItemRecord | Done | recorditem5 EXACT, 435,672 bytes: both records, server/client, air/stone/jukebox, metadata 0/1/15, signed/overflow counts and side boundaries. record4/null-tile and musicevents proofs retained; original no-edit-check behavior preserved. |

The current round-6 commit script is `target/commit-b181-entities6.sh`; it stages the explicit changed-file list with author logiu, the requested email and both co-author trailers. No git commit was issued.
