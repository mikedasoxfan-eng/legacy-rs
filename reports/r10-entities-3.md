# Release 1.0 entities and player, round 3

EntityPlayer and EntityPlayerSP now have their remaining simulation deltas and original-bytecode checks. This work uses the merged `tools/oracle10` directly. Protected Beta crates, harnesses and scripts were not edited; nothing was committed.

## Changes and integration contract

- `PlayerState.food` is an Arc/Mutex-backed FoodStats object. Replacement-player copying retains the same object, while inventory stacks/tags remain deep copies. Entity clones preserve this object identity. `PlayerHud.food` remains a detached value snapshot. Existing verification callers now use explicit read/write guards; the underlying FoodStats arithmetic is unchanged.
- `World.use_portal_to(id, Dimension, fresh_rand, teleporter_rand, progress)` ports the original explicit-destination switch. It handles Overworld/Nether scaling, End entrance `(100, 50, 0)`, the original repeated yOffset additions, orientation, and the `old_dimension < 1` guard before Teleporter placement. The existing `use_portal` API delegates to it for Nether toggles.
- `World.respawn_player_with_inventory(id, keep_inventory, progress)` ports the replacement-copy flag and controller initialization, retaining the session username. `World.return_from_end(id, fresh_rand, teleporter_rand, progress)` returns `(World, optional_invalid_bed_chat)` after the original Overworld switch and keep-inventory respawn. This is the clean function for the GUI worker's GuiWinGame completion callback.
- `spawn_player` now obtains the exact provider dimension ID, including End=1. Prepare-to-spawn uses the original health setter.
- GuiWinGame display/exit wiring and magic-critical particle consumers are explicitly assigned to the GUI worker this round. Existing PlayerEvent requests are retained; this work verifies their original EntityPlayerSP call sites and supplies the simulation operations. No GUI screen or renderer algorithm was changed. The only client-file change adapts an existing hunger verification fixture to the shared object API.

## Exact-check contract

`tools/verify_r10entities3.sh` is registered in `tools/verify10.sh` immediately before final `pflush`. It creates output directories, has one `== entities3: <check>` section per comparison, continues after failures, and returns nonzero for any failure. The shared oracle build compiles the new Mode with every other src Java file; no extra stubs are needed.

The new Rust binary compares entire byte arrays without tolerances, writes the Rust result and identifies the first differing field when labels are present. Original 1.0 methods execute from the deobfuscated JAR with launcher Java 8u51. Fixture subclasses only intercept GUI/renderer requests or install flat terrain; they do not replace the player, FoodStats, path or fall algorithms. The travel fixture calls original Minecraft.usePortal and Teleporter.placeInPortal, with changeWorld's terrain/UI infrastructure replaced by a flat provider. Its contract covers travel title, player state/position/orientation/motion and every modified platform cell; it does not claim terrain generation or preload equivalence.

There are 37 sections: 14 new checks and 23 explicitly named re-runs of existing original-bytecode fixtures. `covered-*` sections delegate to the previous Mode/replay and strengthen attribution and regression coverage; they are not claimed as new branch coverage.

- `player-copy-alias`: 24 scenes, three retained players, 240 alternating FoodStats mutation/update steps each; all food fields, healing/starvation health, XP/score, inventory and unchanged destination fields. Also checks shared object identity and independent stack identity.
- `player-respawn-copy`: original Minecraft.respawn across survival/creative and copy/reset combinations, checking complete player state, IDs, position/orientation and mutation visible through the retained old player.
- `playersp-end-events`: 384 combinations of seeded RNG, client/server, current dimension and requested dimension; ordered achievements, GUI/travel requests, sound float bits and RNG continuation.
- `player-end-travel`: Overworld to End, Nether to End and End to Overworld using the explicit API and original bytecode, including the 5 by 5 obsidian platform and three air layers.
- `playersp-magic-request`: 32 living/nonliving targets; actual original EntityPlayerSP.func_40183_c invokes EntityCrit2FX and EffectRenderer; the simulation request matches magicCrit, target identity and position. Particle internals remain the renderer area's responsibility.
- `pathfinder-obstacles`: 180 cases over three body sizes, six ranges, walls/water/lava/fences/wood and iron doors/trapdoors/gaps/pits; every ordered path point.
- Eight `fall-*` checks: 56 cases per named class, with seven fall boundaries and four RNG seeds, both with and without a retained rider; complete serialized entity state, rider state, world/entity RNG continuation and ordered events. These are new cross-area branch coverage.

## Validation

- New suite: **37/37 EXACT**, exit 0 (`target/entities3-suite.log`). After the final Java field-label bookkeeping change, `playersp-end-events` was rerun EXACT, exit 0 (`target/entities3-end-final.log`). Byte serialization is unchanged.
- `cargo build --release`: passed for the complete workspace (`target/entities3-final-build.log`). Existing output-name collision and unused-code warnings remain.
- `cargo test -p worldgen10`: **80 tests passed**, exit 0 (`target/entities3-final-tests.log`).
- Smoke: `target/release/beta.exe --mc 1.0 --seed 42 --no-save --shot target/r10-smoke.png --shot-ticks 60`: **exit 0** (`target/entities3-smoke.log`). Screenshot inspected: birch/oak forest, nearby water/sky, player hand, crosshair, full health/hunger/hotbar and inventory tutorial.
- Shared all-source compile and raw oracle: `bash tools/oracle10/oracle.sh raw 0 0 0 0 0 t.bin`: **exit 0** (`target/entities3-oracle-final.log`).
- Bash syntax for both verification scripts and commit helper, plus `git diff --check`: passed.

Round-2 regression: **140/140 EXACT**, exit 0 (`target/entities3-round2-final.log`). The first concurrent run had 139 exact sections and a transient `ClassFormatError: Truncated class file` during `dragondeath`, caused by rebuilding shared oracle classes while an existing replay loaded them. No differing byte comparison occurred. The final run starts after compilation and source changes are finished.

## Class status

The table carries forward all 68 assigned classes plus PathFinder. Unchanged rows retain their round-2 named bytecode coverage, rerun as a regression suite this round. Updated rows name exact section arguments of the new suite. A fixture covers the stated branches, not every possible world.

| Class | Status | Source delta and exact checks, or remaining work |
|---|---|---|
| EntityDragon | Done | Complete flight/history, crystal, seven-part collision/damage, block scans, death/XP/portal delta. `dragonhistory`, `dragondamage`, `dragonflight`, `dragondeath`. |
| EntityFishHook | Done | Cast, bite/reel RNG, attached targets, client/server branches and NBT. `fishhook`. |
| EntityLiving | Done | Health, damage remainders, effects/air, child behavior, drops and merged enchantment callbacks. `living6`, `livingdamage6`, `livingnbt6`, `round5-potionexpiry5`, `replay-potions`, drop matrices. |
| EntityThrowable | Done | Shared constructors, heading, flight, impact, owner filtering, water/ground expiry and NBT. `egg`, `snowball`, `pearl`, `splashpotion`. |
| EntityEgg | Done | Original hatch draws/child ages, particles and shared projectile state. `egg`, `round4-impact4`. |
| EntityPlayer | Done | Shared FoodStats identity (including chained copies and two-way retained-player mutation), deep inventory/tag copies, raw copied health/XP/score and default destination capabilities/dimension. `player-copy-alias`, `player-respawn-copy`, `player-end-travel`; `covered-playerxp`, `covered-inventorycopy`, `covered-sleep`, `covered-player-damage`, `covered-player-bow`, `covered-player-eat373`, `covered-player-regen`, `covered-player-starve`. The other complete source deltas remain covered by round 2. |
| EntitySnowball | Done | Blaze damage, impact particles and shared loop/NBT. `snowball`, `round4-impact4`. |
| EntityAnimal | Done | Wheat/love/age, subclass searches, mating, offspring and cooldowns. `breeding`, `mooshroom`, animal replays/drops. |
| EntityBlaze | Done | Wet/hover logic, watcher charging, three-shot burst, particles/brightness and looting. `blaze`, `drops-Blaze`. |
| SpawnerAnimals | Done | Lazy original provider selection, normal caps/attempt RNG/chunk ordering, Release lists and Nether fortress lookup are integrated. `branches-spawnbranches`, `branches-spawnorder`, `rules`, `fortressspawns` (native ChunkProviderHell and World weighted selection at generated piece boundaries). |
| EntityEnderEye | Done | Guidance casts, target clamp, network velocity, lifetime and drop/burst draws. `endereye`. |
| Potion | Done | Registry/colors/readiness, instant effects/undead inversion and names. `potionregistry`, `splashpotion`, `replay-potions`. |
| EntityMinecart | Done | Watchers, fuel, powered collisions, interpolation, rider removal and NBT. All `vehicle-*`, `cartinventory`, `branches-vehiclebranches`, `round5-vehiclecollision5`, `round5-vehicleinteract5`. |
| EntityEnderman | Done | Health/damage, carried blocks, vulnerable targets, sun/wet/indirect teleport branches and sounds. `replay-Enderman`, `drops-Enderman`, `round4-sun4`, `rules`, `branches-factorybranches`. |
| EntityBoat | Done | Complete boat movement, splash/break/interpolation/place delta remains exact. `covered-vehicle-boats`; round-2 vehicle/network/collision checks. |
| PlayerControllerCreative | Done | Initial inventory, immediate breaking, item restoration and capability transitions. `player-creativecontroller`, `settings`. |
| Entity | Done | Air/fire/lava/collision helpers, multipart identity and push-out delta. `api-watchers`, `api-coordinates`, `living6`, `dragonflight`, `dragondamage`, `round4-network4`. |
| InventoryPlayer | Done | Creative overflow, damage/armor, insertion, slot selection and tagged deep-copy identities. `inventory`, `inventorycopy`, `itempickup`, `player-damage`. |
| EntityPotion | Done | Constructors, pitch/speed/gravity, effects, strength/duration and effect 2002. `splashpotion`. |
| EntitySnowman | Done | Attack/snow placement/wet/temperature/living replay plus direct no-fall rider propagation. `covered-mob-SnowMan`, `fall-SnowMan`. |
| EntityMagmaCube | Done | Slime subclass behavior, split/armor/immunity/jump/brightness plus direct no-fall rider propagation. `covered-mob-LavaSlime`, `fall-LavaSlime`. |
| EntitySlime | Done | Smoothed squish, peaceful death, subtype splits, landing sounds, attacks and spawn predicate. `replay-Slime`, `branches-slimebranches`, `drops-Slime`, `rules`. |
| EntityEnderCrystal | Done | Constructor/watchers, rotation, fire base and repeat-hit power-six explosion. `crystal`. |
| EntityVillager | Done | Profession/NBT/texture/movement/persistence/living replay plus direct fall/rider damage. `covered-mob-Villager`, `fall-Villager`. |
| EntitySmallFireball | Done | Owner acceleration, immunity/hit result, five damage/fire, all block faces and no deflection. `smallfireball`, `blaze`. |
| DamageSource | Done | Static/factory sources, fire/projectile/magic flags, bypass/exhaustion and owner/null accessors. `damagesources`, `livingdamage6`, `dragondamage`. |
| EntityPlayerSP | Done | Complete simulation delta and client requests: End achievements/RNG/sound/travel/WinGame branches, magic-critical kind/target and explicit End world travel/return API. `playersp-end-events`, `playersp-magic-request`, `player-end-travel`, `player-respawn-copy`, `covered-playersp`, `covered-player-flight`. GUI screens, particle construction/rendering and event-consumer wiring belong to the GUI/client worker; this row credits EntityPlayerSP simulation and request dispatch, not those separate classes. |
| EntitySpider | Done | Climb watcher, arthropod/poison immunity and eye looting. `replay-Spider`, `drops-Spider`, `rules`, `round5-watchers5`. |
| Explosion | Done | Original ordered damage/block destruction and final fire placement RNG. `explosion`, `round4-explosionblocks4`, `fireball`, `crystal`. |
| PotionEffect | Done | Copy/hash/description, combine rules, readiness/update expiry. `potionregistry`, `replay-potions`, `round5-potionexpiry5`. |
| EntityMooshroom | Done | Cow subclass feeding, baby rejection, bowls, shear conversion/health/body yaw, mushrooms and particles. `mooshroom`, `breeding`, `drops-MushroomCow`. |
| EntityFireball | Done | Owner validity, impact/continued motion, server/client branch, deflection and brightness. `fireball`, `branches-fireballbranches`, `round4-flying4`. |
| EntitySnowmanBase | Done | Flattened inherited creature behavior and silent/persistent overrides; direct fall cases prove the override also prevents rider fall propagation. `covered-mob-SnowMan`, `fall-SnowMan`. |
| EntityEnderPearl | Done | Shared throwable loop, particles, player-owner teleport/fall reset/damage and client branch. `pearl`. |
| EntityGhast | Done | Flight/target/shooting/reflection/looting plus direct no-fall rider propagation. `covered-replay-Ghast`, `covered-round4-retaliation4`, `fall-Ghast`. |
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
| EntityPigZombie | Done | Undead/group-anger/looting plus direct fall/rider damage. `covered-replay-PigZombie`, `covered-round4-retaliation4`, `fall-PigZombie`. |
| EntityPig | Done | Health/breeding and feeding-before-mount. `breeding`, `replay-Pig`, `drops-Pig`, `round5-vehicleinteract5`. |
| EntityCow | Done | Health/breeding, milk and original leather/beef looting/burning draws. `breeding`, `replay-Cow`, `drops-Cow`, `mooshroom`. |
| EntityChicken | Done | Health, child factory, baby egg suppression and feather looting. `breeding`, `replay-Chicken`, `drops-Chicken`, `egg`. |
| EntitySkeleton | Done | Ranged attack/sun/looting/arrow-owned death branches plus direct fall/rider damage. `covered-branches-skeletonbranches`, `covered-round5-skeletondeath5`, `fall-Skeleton`. |
| EntityCreature | Done | Wander probability/age/flee draw order and breeding target dispatch. Creature replays, `breeding`, `round5-wolffollow5`. |
| EntityXPOrb | Done | Constructor RNG, pickup cooldown/sound/float XP and splits. `replay-XPOrb`, `player-xp`, `playerxp`, `dragondeath`. |
| EntityArrow | Done | Critical projectile replay and bow release remain exact. `covered-replay-Arrow`, `covered-player-bow`; round-2 `round4-impact4`. |
| PotionHealth | Done | Instant readiness, heal/harm shifts and undead inversion. `potionregistry`, `splashpotion`. |
| EntityItem | Done | Diamond/blaze-rod achievements, original-count pickup callback and creative absorption. `itempickup`, `replay-Item`, `branches-factorybranches`. |
| EnumCreatureAttribute | Done | Undefined/Undead/Arthropod classification and effect acceptance. `rules`, `potionregistry`, `mob-LavaSlime`, `mob-SnowMan`, `mob-Villager`. |
| EntityFallingSand | Done | First-tick match/mismatch, moving piston, client continuation, landing and timeout/drop. `fallingsand`. |
| EntityWaterMob | Done | Shared health and retained water spawn/XP behavior. `replay-Squid`, `rules`, `living6`. |
| EntityGiantZombie | Done | Health/scaling/movement plus direct fall/rider damage. `covered-replay-Giant`, `fall-Giant`. |
| EntityCaveSpider | Done | Health, climbing/arthropod/poison immunity and inherited difficulty attack. `replay-CaveSpider`, `drops-CaveSpider`, `rules`. |
| PlayerController | Done | Survival capability reset and creative initialization. `settings`, `player-creativecontroller`, `playersp`. |
| PathEntity | Done | Retained path operations after removal of unused method; widths/index/end positions. `pathvalues`, path-bearing creature replays, `round5-wolffollow5`. |
| EntityCreeper | Done | Fuse/powered/death delta plus direct fall/rider damage, entity state and RNG/event continuation. `covered-round4-creeper4`, `covered-round4-creeperdeath4`, `fall-Creeper`. |
| DataWatcher | Done | Typed short/int access, release entries and original packet ordering. `api-watchers`, `round5-watchers5`, `living6`, `crystal`. |
| ChunkPosition | Done | Vec3 flooring, extreme/nonfinite coordinates and hash bits. `pathvalues`. |
| EnumStatus | Done | NotSafe plus server/client rejection/mutation ordering for all bed directions. `sleep`. |
| FoodStats | Done | Retained original exhaustion/heal/starve/NBT algorithm. `food` (72,000 ticks), `player-regen`, `player-starve`, consumption replays. |
| PathFinder | Done | No algorithm delta after normalized renames. Direct original World.getEntityPathToXYZ / Rust PathFinder matrix: 180 obstacle/range/body-size cases, exact presence, full ordered coordinates and entity RNG continuation. `pathfinder-obstacles`. |

## Handoff

`target/commit-r10-entities.sh` stages exactly the 25 files changed in this round and carries the required author and both co-author trailers. It is prepared, not executed. The new verification suite is already integrated into verify10; no follow-up patch application is needed.
