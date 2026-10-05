# Beta 1.8.1 entities - round 3 report

Round 3 makes substantial progress within the requested hour; the whole assigned area is **not finished**. The table below keeps every class and identifies remaining work. No b1.7.3 code, verification scripts, original decompiled sources, player/GUI logic, or terrain generation was edited.

## Round 3 changes and verification

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
| EntityLiving | Not done | combat181 EXACT: 16 mobs x 6 sources x 3 invulnerability timers x 4 damages x 3 attacks. All 16 mob death/XP fixtures pass. Player armor/blocking/creative interactions and complete network APIs remain. |
| SpawnerAnimals | Not done | spawnbranches and spawnorder EXACT; biome adapter, loaded creature counts and colliding chunk-set order fixed. Population/rules checks remain EXACT. Terrain population hookup and sleep spawning remain. |
| EntityXPOrb | Done | entities181: XPOrb, attract, rules (split/texture thresholds, pickup XP/cooldown/sound); xprender181 separately verifies rendering. |
| EntitySilverfish | Done | entities181: Silverfish and allies (burrowing and timed emergence, block/event/RNG records). |
| Entity | Not done | Packed and scalar brightness now EXACT in entityapi181 light (1,800 combinations, 14,400 bytes), including out-of-bounds XP bonus; 53 simulation replays pass. Persistent typed watchers/network synchronization remain unwired. |
| Potion | Done | entities181: rules (all 19 IDs, readiness matrix), potions and poison. |
| DamageSource | Done | entities181: rules covers source names, armor bypass, creative damage, exhaustion; damage and death fixtures exercise dispatch. |
| EntityFireball | Not done | Fireball flight and fireballbranches EXACT: direct/indirect/null source deflection vectors. Dedicated target impact, ghast ownership and block-affecting explosion checks remain. |
| PotionEffect | Done | entities181: rules (merge combinations), potions (effect application, tick/removal order). |
| EntityWolf | Not done | Wolf, kill-Wolf, combat and wolfbranches EXACT; all feeding/bone/owner/sitting branches covered. Player-follow/teleport/leap and group-retaliation matrices still need targeted proof. |
| EntityBoat | Not done | Boat and vehiclebranches EXACT, including repeated damage, overflow, dead state and destruction drops. Multiplayer interpolation/velocity API and interaction matrix remain. |
| EntityArrow | Done | entities181: Arrow (critical flight, gravity, particles, block impacts, RNG). Entity-hit damage branches need broader fixture coverage. |
| EntityCreature | Done | Changed-class audit plus animal/hostile replays, damage and combat181 EXACT: flee timer/path selection/speed, double path arithmetic, target accessors. |
| EntityCaveSpider | Done | entities181: CaveSpider and poison (attack poisons the victim). |
| EntitySkeleton | Not done | Skeleton, kill-Skeleton, combat and skeletonbranches EXACT; speed/aim/cooldown and all distance branches. Vanilla unreachable sniper condition preserved; dedicated achievement/daylight/server-client guard traces remain. |
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
| EntityMinecart | Not done | Minecart and vehiclebranches EXACT; all cart types, inventory drops, repeated damage and overflow. Multiplayer interpolation and full interaction/collision matrix remain. |
| EntityPainting | Not done | DamageSource adapter added; decoration-specific oracle proof remains. |
| EntitySpider | Done | entities181: Spider and CaveSpider (shared movement, climbing and collision behavior). |
| EntityItem | Done | entities181: Item; shared entity state/RNG checks. |
| EntityList | Not done | Registry table plus numeric/string factories and initialized NBT replays are EXACT in registry/factorybranches. Original nullable Item payload/Painting art constructor states and malformed-NBT exception behavior remain. |
| EntitySheep | Done | entities181: Sheep and kill-Sheep; population records initialization color/RNG. |
| EntitySnowball | Done | entities181: Snowball (motion, collisions and thrown damage source). |
| EntitySquid | Done | Changed-class audit plus Squid, kill-Squid and combat181 EXACT; aquatic movement, null sounds, ink drops and XP hooks. |
| EntityWaterMob | Done | Changed-class audit: abstract base plus new XP hook. Squid and kill-Squid EXACT, including XP RNG and emitted orbs. |
| Vec3D | Done | entitygeometry: 10,000 cases; new dot-product use covered by Enderman targeting fixture. |
| EntityFish | Not done | Thrown DamageSource adapter added; fishing hook oracle proof remains. |
| EntitySlime | Done | Changed-class audit; Slime, kill-Slime, combat and slimebranches EXACT, including size XP, byte wrapping, nonzero/zero death health, child constructors/sizes/positions and RNG. |
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
| RenderLiving | Not done | newliving181 and generaliving181 EXACT: 624 traces across 13 mob types, embedded arrows, overlays and per-draw packed light. Player/slime/ghast integration, general equipped blocks/items and labels remain. |
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
| ModelBase | Done | modelregistry181 and modelregistry3_181: 23 model kinds including vehicles/sign, registration order and 256 random selections each, 90,896 comparisons in the expanded suite EXACT. |
| ModelCow | Done | models181: constructor geometry, UVs, successive animation poses and bitwise vertex/part values; 208 poses, 293,376 comparisons across the suite. |
| ModelMinecart | Done | Unified ModelBase constructor/registration/random-part API: modelregistry3_181 EXACT. Animated body and storage/furnace render traces: vehiclerenders181 EXACT. |
| ChestItemRenderHelper | Done | chests181 includes original item helper draw, ignored metadata/brightness arguments and default chest model. |
| ModelBoat | Done | Unified ModelBase constructor/registration/random-part API: modelregistry3_181 EXACT; vehiclerenders181 body/hit transforms EXACT. |
| RenderBiped | Not done | Zombie/Skeleton model, bow transforms, overlays, arrows and packed-light traces EXACT in generaliving181. Full arbitrary equipped block/item branches and general biped integration remain. |
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
| SignModel | Done | Now extends the shared ModelBase representation, with original board/stick registration. modelregistry3_181 and 160 signrenderers cases EXACT. |
| TileEntitySpecialRenderer | Not done | No complete b1.8.1 changed-class port and dedicated EXACT check completed; inherited implementation remains. |
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

Unfinished classes remain explicit in the table. Highest priorities are Entity/EntityLiving networking and player-aware armor/damage, terrain population and sleep spawning, vehicle networking/interactions, lightning/fireball/ghast impact and ignition, PigZombie retaliation, painting/fishing gameplay, and block destruction/drop/ignition in Explosion. Renderer work remains for full RenderLiving/RenderPlayer/RenderBiped/RenderManager/Render/RenderLightningBolt, tile dispatch and EffectRenderer lifecycle/factory/layer eviction.

The particle packed-light API and large-explosion assets remain available to the world-render owner. Chest lid/open-count and world tile dispatch remain dependencies on their owners. No parallel terrain, block, GUI or player implementation was added.

`target/commit-b181-entities3.sh` stages an explicit file list and uses the requested author identity and both co-author trailers. No git commit was issued by this task.
