# Beta 1.8.1 entities ? round 2 report

Round 2 substantially extends the port but does **not** finish the entire assigned area. The per-class table distinguishes completed behavior with original-bytecode proof from remaining work. A finite fixture is evidence for the states it exercises, not exhaustive proof of all Minecraft states. No b1.7.3 sources, verification scripts, or original decompiled sources were edited.

## Verification

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
| EntityLiving | Not done | combat181 EXACT: 16 mobs x 6 sources x 3 invulnerability timers x 4 damages x 3 attacks. All 16 mob death/XP fixtures pass. Player armor/blocking/creative interactions and complete network APIs remain. |
| SpawnerAnimals | Not done | Partial: all biome spawn lists and population algorithm implemented; rules and population EXACT (320 biome/seed fixtures). Natural spawner still adapts the old biome enum; population entry point is not connected to terrain generation. |
| EntityXPOrb | Done | entities181: XPOrb, attract, rules (split/texture thresholds, pickup XP/cooldown/sound); xprender181 separately verifies rendering. |
| EntitySilverfish | Done | entities181: Silverfish and allies (burrowing and timed emergence, block/event/RNG records). |
| Entity | Not done | Packed and scalar brightness now EXACT in entityapi181 light (1,800 combinations, 14,400 bytes), including out-of-bounds XP bonus; 53 simulation replays pass. Persistent typed watchers/network synchronization remain unwired. |
| Potion | Done | entities181: rules (all 19 IDs, readiness matrix), potions and poison. |
| DamageSource | Done | entities181: rules covers source names, armor bypass, creative damage, exhaustion; damage and death fixtures exercise dispatch. |
| EntityFireball | Not done | Partial: source-aware impact and flight fixture EXACT. Remaining entity-hit/explosion trace proof. |
| PotionEffect | Done | entities181: rules (merge combinations), potions (effect application, tick/removal order). |
| EntityWolf | Not done | Wolf, kill-Wolf and the combat matrix EXACT. Fixed wild wolf retaliation to match world membership. Food/bone/owner/taming interaction matrix remains. |
| EntityBoat | Not done | Boat replay EXACT. Full changed-class source audit and vehicle-specific damage/interaction verification remain. |
| EntityArrow | Done | entities181: Arrow (critical flight, gravity, particles, block impacts, RNG). Entity-hit damage branches need broader fixture coverage. |
| EntityCreature | Done | Changed-class audit plus animal/hostile replays, damage and combat181 EXACT: flee timer/path selection/speed, double path arithmetic, target accessors. |
| EntityCaveSpider | Done | entities181: CaveSpider and poison (attack poisons the victim). |
| EntitySkeleton | Not done | Arrow aiming/speed/cooldown updated; Skeleton movement replay EXACT. Shooter/target combat oracle fixture remains. |
| EntityEgg | Done | entities181: Egg (motion, collisions and thrown damage source). |
| EntityCow | Done | entities181: Cow and kill-Cow (movement and new beef drops). |
| EntityChicken | Done | entities181: Chicken and kill-Chicken (movement and chicken drops). |
| EntityLightningBolt | Not done | Lightning replay EXACT. Broader source-aware damage/ignition verification remains. |
| ChunkCoordinates | Done | entityapi181 coordinates: 10,000 cases, 180,000 bytes EXACT; Java shift masking, wrapping integer arithmetic, Y/Z/X comparison and square-root/overflow behavior. |
| EntityAnimal | Done | Changed-class audit plus animal movement/death fixtures and combat181 EXACT: flee-on-damage, despawn suppression and world-RNG XP selection. |
| EntityMob | Done | Changed-class audit plus hostile movement/death fixtures and combat181 EXACT: XP=5, DamageSource attribution/retaliation and source-aware melee dispatch. |
| EntityPigZombie | Not done | PigZombie movement replay EXACT. Anger and group-retaliation changes need targeted proof. |
| EntityDamageSource | Done | entities181: rules, poison, allies, kill-* exercise entity attribution. |
| EntityDamageSourceIndirect | Done | entities181: rules plus indirect Enderman projectile fixture; vanilla owner accessor behavior preserved. |
| PotionHealth | Done | entities181: rules and potions (instant health/harm shift behavior). |
| EntityMinecart | Not done | Minecart replay EXACT. Full vehicle-specific changed-class verification remains. |
| EntityPainting | Not done | DamageSource adapter added; decoration-specific oracle proof remains. |
| EntitySpider | Done | entities181: Spider and CaveSpider (shared movement, climbing and collision behavior). |
| EntityItem | Done | entities181: Item; shared entity state/RNG checks. |
| EntityList | Not done | Exact original numeric/name/class table, including abstract Mob/Monster and unregistered player/projectile types: entityapi181 registry, 1,631 bytes. General constructor-by-ID and registry NBT factory remain. |
| EntitySheep | Done | entities181: Sheep and kill-Sheep; population records initialization color/RNG. |
| EntitySnowball | Done | entities181: Snowball (motion, collisions and thrown damage source). |
| EntitySquid | Done | Changed-class audit plus Squid, kill-Squid and combat181 EXACT; aquatic movement, null sounds, ink drops and XP hooks. |
| EntityWaterMob | Done | Changed-class audit: abstract base plus new XP hook. Squid and kill-Squid EXACT, including XP RNG and emitted orbs. |
| Vec3D | Done | entitygeometry: 10,000 cases; new dot-product use covered by Enderman targeting fixture. |
| EntityFish | Not done | Thrown DamageSource adapter added; fishing hook oracle proof remains. |
| EntitySlime | Not done | Slime movement replay EXACT; XP size hook present. Full size/death/split oracle matrix remains. |
| DataWatcher | Done | entityapi181 watchers: 69,853 bytes EXACT. All seven types, UTF-16/surrogates, NaN/signed-zero equality, ItemStack identity, negative IDs, HashMap order, update/apply-list flags, wire round trips and invalid headers. Entity network integration remains separate. |
| EntityCreeper | Not done | Creeper movement replay EXACT; explosion/damage branches need dedicated verification. |
| EntityGhast | Not done | Ghast movement replay EXACT; attack/explosion behavior needs dedicated verification. |
| AxisAlignedBB | Done | entitygeometry: 10,000 cases. |
| EntityZombie | Done | entities181: Zombie and kill-Zombie (new rotten-flesh drop). |
| Explosion | Not done | explosion181 EXACT: 16 air-explosion cases, 752,231 bytes of affected-position order, mob damage/motion, particles and RNG. Block destruction/drop/ignition matrix remains cross-area work. |
| EntityFlying | Not done | Ghast movement replay EXACT; full changed-class audit remains. |
| PathEntity | Done | entities181: creature path replay through new and existing mobs. |
| ModelEnderman | Done | models181: constructor geometry, UVs, successive animation poses and bitwise vertex/part values; 208 poses, 293,376 comparisons across the suite. |
| TileEntityChestRenderer | Done | chests181: 4,097 cases, 1,372,320 values EXACT; single/double chest adjacency, metadata, lid interpolation/cubic easing, GL states and vertices. World tile dispatcher/lid simulation hookup remains a dependency. |
| RenderEnderman | Done | newliving181: normal/eye/hurt/death draws, carrying, screaming/jitter, embedded arrows and every captured lightmap coordinate EXACT. Runtime persistent Gaussian RNG and additive eye passes wired. |
| RenderArrow | Done | arrows181: 1,024 cases covering zero rotation, shake, partial ticks and both crit flags; newliving181 also verifies embedded arrows. Removed obsolete first-rotation visibility guard. |
| RenderLiving | Not done | newliving181 EXACT for Enderman/Silverfish/Spider/CaveSpider (192 cases, 3,390,450 values). Embedded-arrow implementation supplied; remaining mob/player raw renderer and embedded-arrow integrations remain. |
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
| RenderPlayer | Not done | No complete b1.8.1 changed-class port and dedicated EXACT check completed; inherited implementation remains. |
| EffectRenderer | Not done | Large/huge explosion and crit2 creation/ticking plus packed-light vertex API supplied; particles2_181 and emitters181 EXACT. Full layer-eviction/factory/clear-world/render order matrix and ordinary particle GPU lightmap hookup remain. |
| EntityPickupFX | Done | particles2_181: 32 pickup cases, 512 squared-interpolation transforms, original float casts, timer/state/RNG and packed light EXACT. Corrected lit draw material to vanilla white. |
| ModelQuadruped | Done | models181: constructor geometry, UVs, successive animation poses and bitwise vertex/part values; 208 poses, 293,376 comparisons across the suite. |
| RenderManager | Not done | No complete b1.8.1 changed-class port and dedicated EXACT check completed; inherited implementation remains. |
| RenderMinecart | Done | vehiclerenders181: 408 cases, 17,784 draws and 1,741,710 values EXACT; rail transforms, storage chest model/offset, furnace contents and hit rocking. |
| EntitySlimeFX | Done | particles181: constructors, 202 successive state/vertex frames per variant; 184 cases total, 594,688 vertices, zero differing fields. Packed world lightmap integration is a separate remaining dependency. |
| ModelSlime | Done | models181: constructor geometry, UVs, successive animation poses and bitwise vertex/part values; 208 poses, 293,376 comparisons across the suite. |
| ModelBase | Done | modelregistry181: 20 model kinds, registration order and 256 random selections each, 79,194 raw-bit comparisons EXACT. |
| ModelCow | Done | models181: constructor geometry, UVs, successive animation poses and bitwise vertex/part values; 208 poses, 293,376 comparisons across the suite. |
| ModelMinecart | Not done | Geometry/animation/render traces EXACT in vehiclerenders181; unified ModelBase registration/random-part API for vehicle models remains. |
| ChestItemRenderHelper | Done | chests181 includes original item helper draw, ignored metadata/brightness arguments and default chest model. |
| ModelBoat | Not done | Geometry/render traces EXACT in vehiclerenders181; unified ModelBase registration/random-part API for vehicle models remains. |
| RenderBiped | Not done | No complete b1.8.1 changed-class port and dedicated EXACT check completed; inherited implementation remains. |
| TileEntityRenderer | Not done | No complete b1.8.1 changed-class port and dedicated EXACT check completed; inherited implementation remains. |
| EntityDiggingFX | Done | particles181: constructors, 202 successive state/vertex frames per variant; 184 cases total, 594,688 vertices, zero differing fields. Packed world lightmap integration is a separate remaining dependency. |
| ModelBed | Done | bedmodel181: all 54 direction/remapping table entries EXACT. Tables supplied to the world-render owner. |
| ModelGhast | Done | models181: constructor geometry, UVs, successive animation poses and bitwise vertex/part values; 208 poses, 293,376 comparisons across the suite. |
| ModelSquid | Done | models181: constructor geometry, UVs, successive animation poses and bitwise vertex/part values; 208 poses, 293,376 comparisons across the suite. |
| TexturedQuad | Done | models181: constructor geometry, UVs, successive animation poses and bitwise vertex/part values; 208 poses, 293,376 comparisons across the suite. |
| ModelSheep1 | Done | models181: constructor geometry, UVs, successive animation poses and bitwise vertex/part values; 208 poses, 293,376 comparisons across the suite. |
| Render | Not done | No complete b1.8.1 changed-class port and dedicated EXACT check completed; inherited implementation remains. |
| RenderBoat | Done | vehiclerenders181: body transforms, hit rocking and model draw traces EXACT. |
| RenderLightningBolt | Not done | No complete b1.8.1 changed-class port and dedicated EXACT check completed; inherited implementation remains. |
| SignModel | Not done | No complete b1.8.1 changed-class port and dedicated EXACT check completed; inherited implementation remains. |
| TileEntitySpecialRenderer | Not done | No complete b1.8.1 changed-class port and dedicated EXACT check completed; inherited implementation remains. |
| EntityFootStepFX | Done | particles181: constructors, 202 successive state/vertex frames per variant; 184 cases total, 594,688 vertices, zero differing fields. Packed world lightmap integration is a separate remaining dependency. |
| EntityHeartFX | Done | particles181: constructors, 202 successive state/vertex frames per variant; 184 cases total, 594,688 vertices, zero differing fields. Packed world lightmap integration is a separate remaining dependency. |
| EntityNoteFX | Done | particles181: constructors, 202 successive state/vertex frames per variant; 184 cases total, 594,688 vertices, zero differing fields. Packed world lightmap integration is a separate remaining dependency. |
| EntityReddustFX | Done | particles181: constructors, 202 successive state/vertex frames per variant; 184 cases total, 594,688 vertices, zero differing fields. Packed world lightmap integration is a separate remaining dependency. |
| EntitySmokeFX | Done | particles181: constructors, 202 successive state/vertex frames per variant; 184 cases total, 594,688 vertices, zero differing fields. Packed world lightmap integration is a separate remaining dependency. |
| EntitySnowShovelFX | Done | particles181: constructors, 202 successive state/vertex frames per variant; 184 cases total, 594,688 vertices, zero differing fields. Packed world lightmap integration is a separate remaining dependency. |
| ModelSheep2 | Done | models181: constructor geometry, UVs, successive animation poses and bitwise vertex/part values; 208 poses, 293,376 comparisons across the suite. |
| RenderFish | Not done | No complete b1.8.1 changed-class port and dedicated EXACT check completed; inherited implementation remains. |
| EntityFX | Done | particles181: constructors, 202 successive state/vertex frames per variant; 184 cases total, 594,688 vertices, zero differing fields. Packed world lightmap integration is a separate remaining dependency. |
| EntityRainFX | Done | particles181: constructors, 202 successive state/vertex frames per variant; 184 cases total, 594,688 vertices, zero differing fields. Packed world lightmap integration is a separate remaining dependency. |
| RenderChicken | Not done | No complete b1.8.1 changed-class port and dedicated EXACT check completed; inherited implementation remains. |
| RenderCreeper | Not done | No complete b1.8.1 changed-class port and dedicated EXACT check completed; inherited implementation remains. |
| RenderSheep | Not done | No complete b1.8.1 changed-class port and dedicated EXACT check completed; inherited implementation remains. |
| RenderSquid | Not done | No complete b1.8.1 changed-class port and dedicated EXACT check completed; inherited implementation remains. |
| RenderWolf | Not done | No complete b1.8.1 changed-class port and dedicated EXACT check completed; inherited implementation remains. |
| TileEntitySignRenderer | Not done | No complete b1.8.1 changed-class port and dedicated EXACT check completed; inherited implementation remains. |

## Integration and remaining work

`Entity::packed_brightness`, particle packed-brightness APIs and weather snapshots supply original packed light values. The existing entity GPU pipeline now uses the new mob renderer traces, persistent Enderman Gaussian jitter, additive eye blending and hurt-eye overlays; light transitions are preserved for embedded arrows and carried blocks. Ordinary particle GPU lightmap upload remains the integrator's work. `EffectRenderer::lightmapped_vertices` supplies the values for that integration; large-explosion lit geometry and texture assets are available.

The chest renderer and item helper are callable and exact; storage minecart and inventory rendering now call the chest helper. The current chest tile representation stores inventory only; world dispatch and lid/open-count animation must be connected by the tile/container/world-render owners. ModelBed tables are likewise supplied without rewriting RenderBlocks. Typed DataWatcher exists as a standalone original-wire implementation; entity-owned watchers and network synchronization are still needed. EntityList's exact table does not substitute for complete constructor/NBT factories.

Remaining gameplay priorities are full player-aware damage/armor behavior, wolf interactions, natural spawning/terrain hookup, vehicle interaction/damage, skeleton shooting/achievement traces, lightning ignition, fireball/ghast hits, slime splitting, painting/fishing interactions and block-affecting explosions. Remaining renderer priorities are the complete general living/player/equipped-item/lightmap traces, sign/tile dispatch and full EffectRenderer lifecycle/factory behavior. The table preserves every unfinished class instead of promoting it from a narrow fixture alone.

Changed-class b1.7.3/b1.8.1 diffs were generated under `target/entities2-audits/`; these use the supplied renames and contain some decompiler/name noise (notably the supplied posZ rename). Detailed reviews were used for watcher/coordinate/model registration, chest/new mob/arrow rendering, particles and the animal/creature/mob/aquatic base changes. Larger unfinished classes still require complete audit and branch-specific tests.

`target/commit-b181-entities2.sh` stages an explicit file list and uses the requested author identity and both co-author trailers. No commit was issued.
