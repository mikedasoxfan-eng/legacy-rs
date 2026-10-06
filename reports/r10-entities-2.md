# Release 1.0 entities and player, round 2

The source audit and exact checks close 66 of the task's 68 classes. EntityPlayer and EntityPlayerSP retain concrete gaps described below. PathFinder, additionally assigned by the area description, has no algorithm delta after the decompiler renames and is covered by the path-bearing entity replays.

Original Release source/JAR: `../beta-rs/tools/retromcp-r10/minecraft`. The base oracle and `tools/verify10.sh` have not been merged into this worktree. Following the common task's instruction not to create the base harness, only the new `Mode_r10entities2.java` was added here. `run_entities2_oracle.py` compiles that mode into this worktree's `target` and uses the existing sibling `../wt181-r10harness/tools/oracle10/build` read-only, with launcher Java 8u51. `ORACLE10_HOME` and `ORACLE_JAVA` support an explicit harness/runtime. Original game methods execute from the deobfuscated original JAR through the existing harness hooks; they are not rewritten as Java approximations.

## Fixes

- FishHook server/client ownership and attached-entity branches, reel-distance float narrowing, and original six-field NBT persistence; shared throwable NBT persistence.
- Animal class matching includes Mooshroom in Cow's inherited class query, preserving Java subclass breeding/parent-follow behavior.
- Powered minecart fuel adds 3600; cart collision uses the source facing/separation dot product and 0.95 damping. Living riders clear fall distance. Boat placement observes the DDA-mutated ray start, as Java does.
- Squid looting increases the drop bound. Villager ambient sound interval is 80.
- Inventory copies deep-copy stack tags as well as stack identity. Item pickup honors the client guard; new blaze rod, ghast tear and gold nugget inventory defaults prevent null-item panics.
- All sleep rejection checks execute only on the server, matching the original method.
- SpawnerAnimals now calls the merged world/provider spawn-entry API, so Nether fortress lists can reach normal spawning instead of being bypassed by a direct biome query.

Verification-only corrections: synchronize factory air with its watcher; serialize fractional XP in the original field position; omit Creature-only fields for MagmaCube and Dragon; use Java simple spawn-class names independently of EntityList registration names; invoke actual damage in the living NBT fixture. The new mode corrects two inherited harness adapters locally: the Release potion-expiry fixture calls `updatePotionEffects`, and its spawning provider supplies the original biome lists rather than an empty list. Neither correction replaces the original game algorithm or changes the sibling harness.

## Exact-check contract and validation

`bash tools/verify_r10entities2.sh --built` runs each named check under its own `== entities2:` section, creates output folders, continues after individual failures and returns failure if any check fails. The Rust binary compares the entire serialized byte sequence, including Java float/double bits, RNG continuation, watcher/entity state, ordered effects and applicable block changes. Java's canonical NaN encoding is mirrored; there is no epsilon or tolerant comparison. New layouts also emit byte-offset field labels for diagnosing the first difference.

- Full suite: **139/139 EXACT**, exit 0, recorded in `target/r10/entities2-suite-final.log`. Following the final SpawnerAnimals integration, the suite's focused `--built fortressspawns branches-spawnbranches branches-spawnorder rules` run passed **4/4 EXACT**, exit 0 (`target/r10/entities2-focused-final.log`). Together these are **140 distinct exact checks**. The new fortress mode compares 3,911,253 bytes across eight original generated fortress starts, piece boundaries, all three spawn categories, weighted entry values and RNG continuation.
- `cargo build --release`: passed for the whole workspace after the final source change. Existing output-name collision and unused-code warnings remain.
- `cargo test -p worldgen10`: all **80 tests passed** on the final source (47 library tests and 33 integration tests).
- `target/release/beta.exe --mc 1.0 --seed 42 --no-save --shot target/r10-smoke.png --shot-ticks 60`: exit 0 after the final source change. Screenshot inspected: forest/coast terrain, sky, hand, crosshair, health/hunger/hotbar and tutorial display. The inherited Beta 1.8.1 overlay label remains a client-area branding issue.
- Python compilation, Bash syntax and `git diff --check` passed. The handoff audit confirms one row for all 68 requested classes plus PathFinder and an explicit staging list for all 27 changed/new files.

## Class status

`Done` means the audited source delta is present and the named original-bytecode checks pass exactly. Check names below are arguments to the new suite's runner, including wrappers around existing original-bytecode modes. A matching fixture establishes the covered branches, not an exhaustive proof of all possible worlds.

| Class | Status | Source delta and exact checks, or remaining work |
|---|---|---|
| EntityDragon | Done | Complete flight/history, crystal, seven-part collision/damage, block scans, death/XP/portal delta. `dragonhistory`, `dragondamage`, `dragonflight`, `dragondeath`. |
| EntityFishHook | Done | Cast, bite/reel RNG, attached targets, client/server branches and NBT. `fishhook`. |
| EntityLiving | Done | Health, damage remainders, effects/air, child behavior, drops and merged enchantment callbacks. `living6`, `livingdamage6`, `livingnbt6`, `round5-potionexpiry5`, `replay-potions`, drop matrices. |
| EntityThrowable | Done | Shared constructors, heading, flight, impact, owner filtering, water/ground expiry and NBT. `egg`, `snowball`, `pearl`, `splashpotion`. |
| EntityEgg | Done | Original hatch draws/child ages, particles and shared projectile state. `egg`, `round4-impact4`. |
| EntityPlayer | Not done | XP, damage, armor, flight, consumption, sleep and tagged inventory now compare exactly (`playerxp`, all `player-*`, `sleep`, `inventorycopy`). `copy_player_from` clones FoodStats by value; original replacement-player copying aliases the same FoodStats object. Preserve shared identity and add a two-retained-player mutation oracle before Done. |
| EntitySnowball | Done | Blaze damage, impact particles and shared loop/NBT. `snowball`, `round4-impact4`. |
| EntityAnimal | Done | Wheat/love/age, subclass searches, mating, offspring and cooldowns. `breeding`, `mooshroom`, animal replays/drops. |
| EntityBlaze | Done | Wet/hover logic, watcher charging, three-shot burst, particles/brightness and looting. `blaze`, `drops-Blaze`. |
| SpawnerAnimals | Done | Lazy original provider selection, normal caps/attempt RNG/chunk ordering, Release lists and Nether fortress lookup are integrated. `branches-spawnbranches`, `branches-spawnorder`, `rules`, `fortressspawns` (native ChunkProviderHell and World weighted selection at generated piece boundaries). |
| EntityEnderEye | Done | Guidance casts, target clamp, network velocity, lifetime and drop/burst draws. `endereye`. |
| Potion | Done | Registry/colors/readiness, instant effects/undead inversion and names. `potionregistry`, `splashpotion`, `replay-potions`. |
| EntityMinecart | Done | Watchers, fuel, powered collisions, interpolation, rider removal and NBT. All `vehicle-*`, `cartinventory`, `branches-vehiclebranches`, `round5-vehiclecollision5`, `round5-vehicleinteract5`. |
| EntityEnderman | Done | Health/damage, carried blocks, vulnerable targets, sun/wet/indirect teleport branches and sounds. `replay-Enderman`, `drops-Enderman`, `round4-sun4`, `rules`, `branches-factorybranches`. |
| EntityBoat | Done | Watchers, splash order, motion/break threshold, network interpolation and placement adapter. All `vehicle-*`, `branches-vehiclebranches`, `round4-network4`, `round5-vehiclecollision5`. |
| PlayerControllerCreative | Done | Initial inventory, immediate breaking, item restoration and capability transitions. `player-creativecontroller`, `settings`. |
| Entity | Done | Air/fire/lava/collision helpers, multipart identity and push-out delta. `api-watchers`, `api-coordinates`, `living6`, `dragonflight`, `dragondamage`, `round4-network4`. |
| InventoryPlayer | Done | Creative overflow, damage/armor, insertion, slot selection and tagged deep-copy identities. `inventory`, `inventorycopy`, `itempickup`, `player-damage`. |
| EntityPotion | Done | Constructors, pitch/speed/gravity, effects, strength/duration and effect 2002. `splashpotion`. |
| EntitySnowman | Done | Hostile attack, snow placement, wet/climate behavior, health and drops; merged biome data is consumed. `mob-SnowMan`, `drops-SnowMan`. |
| EntityMagmaCube | Done | Slime subclass constructor/splitting, armor, immunity, jump/squish, attack and brightness. `mob-LavaSlime`, `drops-LavaSlime`, `branches-factorybranches`. |
| EntitySlime | Done | Smoothed squish, peaceful death, subtype splits, landing sounds, attacks and spawn predicate. `replay-Slime`, `branches-slimebranches`, `drops-Slime`, `rules`. |
| EntityEnderCrystal | Done | Constructor/watchers, rotation, fire base and repeat-hit power-six explosion. `crystal`. |
| EntityVillager | Done | Profession/texture/NBT, movement, persistent despawn, health and 80-tick ambient interval. `mob-Villager`, `drops-Villager`, `branches-factorybranches`. |
| EntitySmallFireball | Done | Owner acceleration, immunity/hit result, five damage/fire, all block faces and no deflection. `smallfireball`, `blaze`. |
| DamageSource | Done | Static/factory sources, fire/projectile/magic flags, bypass/exhaustion and owner/null accessors. `damagesources`, `livingdamage6`, `dragondamage`. |
| EntityPlayerSP | Not done | `playersp` and player replays pass exactly for covered movement/sprint/flight, health and XP boundaries. End transition/WinGame and magic-critical event consumers remain absent in the client; implement and verify those original screen/particle paths with the client/world owners. Brewing/enchantment container support has merged since round 1. |
| EntitySpider | Done | Climb watcher, arthropod/poison immunity and eye looting. `replay-Spider`, `drops-Spider`, `rules`, `round5-watchers5`. |
| Explosion | Done | Original ordered damage/block destruction and final fire placement RNG. `explosion`, `round4-explosionblocks4`, `fireball`, `crystal`. |
| PotionEffect | Done | Copy/hash/description, combine rules, readiness/update expiry. `potionregistry`, `replay-potions`, `round5-potionexpiry5`. |
| EntityMooshroom | Done | Cow subclass feeding, baby rejection, bowls, shear conversion/health/body yaw, mushrooms and particles. `mooshroom`, `breeding`, `drops-MushroomCow`. |
| EntityFireball | Done | Owner validity, impact/continued motion, server/client branch, deflection and brightness. `fireball`, `branches-fireballbranches`, `round4-flying4`. |
| EntitySnowmanBase | Done | Flattened no-fall/no-despawn/silent shared overrides and inherited creature behavior. `mob-SnowMan`, `branches-factorybranches`. |
| EntityEnderPearl | Done | Shared throwable loop, particles, player-owner teleport/fall reset/damage and client branch. `pearl`. |
| EntityGhast | Done | Health, vulnerable targeting, charge/fire events, reflection and tear/gunpowder looting. `replay-Ghast`, `drops-Ghast`, `round4-flying4`, `round4-retaliation4`. |
| DragonPart | Done | Part identity/query and all seven damage forwards, including head/body scaling. `dragondamage`, `dragonflight`. |
| EntitySilverfish | Done | Health/attribute, infestation/wakeup, sounds, vulnerable targets and spawn predicate. `replay-Silverfish`, `drops-Silverfish`, `rules`. |
| PlayerCapabilities | Done | All flag combinations, NBT including original flying/disableDamage serialization bug, creative/survival reset. `playerxp`, `settings`, `player-flight`, `playersp`. |
| EntityDragonBase | Done | Flattened health/direct-damage rejection and part forwarding. `dragondamage`, `dragonflight`, `dragondeath`. |
| EntityList | Done | Release names/IDs, factories/NBT; Potion remains unregistered. `registry10`, `api-registry`, `branches-factorybranches`. |
| EntitySheep | Done | Health, wheat, child constructor and parent fleece draw. `breeding`, `replay-Sheep`, `drops-Sheep`. |
| EntityMob | Done | Potion melee shifts, health and vulnerable targeting. `livingdamage6`, `replay-damage`, `replay-potions`, hostile replays. |
| WorldSettings | Done | Seed/game-mode/features/hardcore values and capability mode initialization. `settings`. World/client hardcore UI belongs to its owner. |
| EntitySquid | Done | Health, age swim stop, spawn height/collision and looting-bound fix. `replay-Squid`, `drops-Squid`, `rules`. |
| EntityZombie | Done | Health/damage/armor/undead and max-preserving sun fire. `replay-Zombie`, `drops-Zombie`, `potionregistry`, `round4-sun4`. |
| EntityWolf | Done | Wild/tame health, angry-only despawn, breeding and owner behavior. `replay-Wolf`, `breeding`, `branches-wolfbranches`, all `round5-wolf*`, `drops-Wolf`. |
| EntityPigZombie | Done | Undead health/armor/strength, group anger and flesh/nugget looting. `replay-PigZombie`, `drops-PigZombie`, `round4-retaliation4`. |
| EntityPig | Done | Health/breeding and feeding-before-mount. `breeding`, `replay-Pig`, `drops-Pig`, `round5-vehicleinteract5`. |
| EntityCow | Done | Health/breeding, milk and original leather/beef looting/burning draws. `breeding`, `replay-Cow`, `drops-Cow`, `mooshroom`. |
| EntityChicken | Done | Health, child factory, baby egg suppression and feather looting. `breeding`, `replay-Chicken`, `drops-Chicken`, `egg`. |
| EntitySkeleton | Done | Health/undead, sun fire, arrows and arrow/bone looting. `replay-Skeleton`, `branches-skeletonbranches`, `round5-skeletondeath5`, `drops-Skeleton`, `round4-sun4`. |
| EntityCreature | Done | Wander probability/age/flee draw order and breeding target dispatch. Creature replays, `breeding`, `round5-wolffollow5`. |
| EntityXPOrb | Done | Constructor RNG, pickup cooldown/sound/float XP and splits. `replay-XPOrb`, `player-xp`, `playerxp`, `dragondeath`. |
| EntityArrow | Done | Critical impact RNG, sound and owner attribution. `replay-Arrow`, `round4-impact4`, `player-bow`. |
| PotionHealth | Done | Instant readiness, heal/harm shifts and undead inversion. `potionregistry`, `splashpotion`. |
| EntityItem | Done | Diamond/blaze-rod achievements, original-count pickup callback and creative absorption. `itempickup`, `replay-Item`, `branches-factorybranches`. |
| EnumCreatureAttribute | Done | Undefined/Undead/Arthropod classification and effect acceptance. `rules`, `potionregistry`, `mob-LavaSlime`, `mob-SnowMan`, `mob-Villager`. |
| EntityFallingSand | Done | First-tick match/mismatch, moving piston, client continuation, landing and timeout/drop. `fallingsand`. |
| EntityWaterMob | Done | Shared health and retained water spawn/XP behavior. `replay-Squid`, `rules`, `living6`. |
| EntityGiantZombie | Done | Health accessor and retained scaling/movement. `replay-Giant`, `drops-Giant`, `branches-factorybranches`. |
| EntityCaveSpider | Done | Health, climbing/arthropod/poison immunity and inherited difficulty attack. `replay-CaveSpider`, `drops-CaveSpider`, `rules`. |
| PlayerController | Done | Survival capability reset and creative initialization. `settings`, `player-creativecontroller`, `playersp`. |
| PathEntity | Done | Retained path operations after removal of unused method; widths/index/end positions. `pathvalues`, path-bearing creature replays, `round5-wolffollow5`. |
| EntityCreeper | Done | Health accessor and retained fuse/powered/death paths. `replay-Creeper`, `round4-creeper4`, `round4-creeperdeath4`, `drops-Creeper`. |
| DataWatcher | Done | Typed short/int access, release entries and original packet ordering. `api-watchers`, `round5-watchers5`, `living6`, `crystal`. |
| ChunkPosition | Done | Vec3 flooring, extreme/nonfinite coordinates and hash bits. `pathvalues`. |
| EnumStatus | Done | NotSafe plus server/client rejection/mutation ordering for all bed directions. `sleep`. |
| FoodStats | Done | Retained original exhaustion/heal/starve/NBT algorithm. `food` (72,000 ticks), `player-regen`, `player-starve`, consumption replays. |
| PathFinder | Done | Additional assigned class: only MCHash/IntHashMap symbol delta after normalization. Original path-bearing `replay-Wolf`, `replay-Zombie`, `round5-wolffollow5` compare path state exactly. |

## Integration and handoff

The ray cursor change and `ItemBoat` call site are the smallest cross-area adapter needed for exact original boat placement. Inventory copying calls the already-merged ItemStack tag API; `player/registry.rs` adds only the three missing new-drop defaults. No other area's algorithms were replaced.

`tools/r10/entities2-verify10.patch` supplies the required suite call immediately before the final `pflush`; `git apply --check` passes against the sibling harness's current script without modifying it. Apply the patch when the base `tools/verify10.sh` arrives; that file is currently absent, and creating it would violate the common task instruction. The standalone suite is runnable now with the sibling harness, so the individual class checks have executed independently of that pending umbrella integration.

No protected Beta crate/harness/verification file was edited. No commit was made. `target/commit-r10-entities.sh` contains only the explicit changed-file list and the required author/co-author metadata.
