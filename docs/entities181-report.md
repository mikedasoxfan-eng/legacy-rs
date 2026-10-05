# Beta 1.8.1 entities work report

This is a partial port, not completion of the entire assigned area. The tables below use **Done** only for ported class behavior with a named exact oracle check; **Not done** includes partial implementations and inherited classes without a complete changed-class audit. Passing a finite fixture is not proof of every possible state. No b1.7.3 source or original shared oracle harness was edited.

## Exact verification

`cargo build --release -p worldgen181 -p client -j 2` passed. The final focused rerun passed all 42 simulation fixtures and all four geometry/model/particle/XP-render checks, recorded in `target/entities-final-verify.log`. `tools/verify181.sh` now calls `tools/verify_entities181.sh` in the required final delimited entities block, before the existing final result/exit. The new script observes the shared verification lock.

- `entities181`: 42 focused fixtures all EXACT (39 primary plus the three Enderman fixtures below): rules, population, carry, allies, attract, Boat, Minecart, Item, Lightning, Snowball, Egg, Fireball, Enderman, teleport, CaveSpider, Silverfish, XPOrb, Arrow, Pig, Cow, Chicken, Sheep, Wolf, Zombie, Skeleton, Spider, Creeper, PigZombie, Ghast, Squid, Slime, potions, damage, poison, kill-Cow, kill-Chicken, kill-Sheep, kill-Zombie, kill-Enderman. State includes raw floating bits, RNG states, effect order, spawned entities and events.
- `entitygeometry`: existing section EXACT, 10,000 cases.
- `models181`: EXACT, 26 constructors, 208 successive poses, 1,536 parts, 293,376 bit comparisons. Includes Enderman carrying/screaming, Silverfish and both chest geometries.
- `particles181`: EXACT, 184 cases, 594,688 vertices, zero differing fields. Includes crit, magic crit, cloud with a real nearby player, aura and water suspend. Pickup count is **zero**; packed lightmap outputs are not captured.
- `xprender181`: EXACT, 352 draws and 21,120 bit comparisons (XP value/texture, age/color and partial tick matrix).

These are new `Mode_*.java` classes that execute original bytecode. Existing `entities`, `models`, `particles`, `renderers` and `livingrenderers` modes/sections have not all been made EXACT: obsolete reflection names remain in some old modes, and full renderer parity is unfinished. The full approximately one-hour verify181 run was not run.

Additional `entities181 indirect 20`, `stare 10` and `starepumpkin 10` fixtures are EXACT, recorded in `target/entities-extra-verify.log`. The stare pair distinguishes the initial stare counter from the pumpkin bypass; it is not a sustained six-tick target-acquisition fixture. The indirect fixture confirms that Beta 1.8.1 takes projectile damage normally, without the later-version teleport-on-hit behavior. Silverfish allies was rerun EXACT after correcting spawn-before-particle ordering.

## Per-class status

| Class | Status | Exact proof or remaining work |
|---|---|---|
| EntityEnderman | Done | entities181: Enderman, carry, teleport; rules checks carry eligibility and NBT typo. Indirect and initial stare/pumpkin fixtures are EXACT; sustained target acquisition needs broader coverage. |
| EntityLiving | Not done | Partial: source-aware damage, effects, XP death drops, arrow count, movement and NBT implemented; exact damage/potions/poison/kill fixtures. Remaining complete player blocking/creative/food interactions and broad damage/armor matrix. |
| SpawnerAnimals | Not done | Partial: all biome spawn lists and population algorithm implemented; rules and population EXACT (320 biome/seed fixtures). Natural spawner still adapts the old biome enum; population entry point is not connected to terrain generation. |
| EntityXPOrb | Done | entities181: XPOrb, attract, rules (split/texture thresholds, pickup XP/cooldown/sound); xprender181 separately verifies rendering. |
| EntitySilverfish | Done | entities181: Silverfish and allies (burrowing and timed emergence, block/event/RNG records). |
| Entity | Not done | Partial: new state/source-aware damage/sprinting/velocity flags. Shared replay is EXACT, but full DataWatcher/network/lightmap API parity remains. |
| Potion | Done | entities181: rules (all 19 IDs, readiness matrix), potions and poison. |
| DamageSource | Done | entities181: rules covers source names, armor bypass, creative damage, exhaustion; damage and death fixtures exercise dispatch. |
| EntityFireball | Not done | Partial: source-aware impact and flight fixture EXACT. Remaining entity-hit/explosion trace proof. |
| PotionEffect | Done | entities181: rules (merge combinations), potions (effect application, tick/removal order). |
| EntityWolf | Not done | Partial: flee behavior and new food healing implemented; Wolf replay EXACT. Remaining interaction oracle matrix for food/bone/taming. |
| EntityBoat | Not done | Boat replay EXACT. Full changed-class source audit and vehicle-specific damage/interaction verification remain. |
| EntityArrow | Done | entities181: Arrow (critical flight, gravity, particles, block impacts, RNG). Entity-hit damage branches need broader fixture coverage. |
| EntityCreature | Not done | Partial: flee/path/speed/potion changes; multiple mob replays EXACT. Full changed-class audit and target/path edge cases remain. |
| EntityCaveSpider | Done | entities181: CaveSpider and poison (attack poisons the victim). |
| EntitySkeleton | Not done | Arrow aiming/speed/cooldown updated; Skeleton movement replay EXACT. Shooter/target combat oracle fixture remains. |
| EntityEgg | Done | entities181: Egg (motion, collisions and thrown damage source). |
| EntityCow | Done | entities181: Cow and kill-Cow (movement and new beef drops). |
| EntityChicken | Done | entities181: Chicken and kill-Chicken (movement and chicken drops). |
| EntityLightningBolt | Not done | Lightning replay EXACT. Broader source-aware damage/ignition verification remains. |
| ChunkCoordinates | Not done | No dedicated b1.8.1 implementation/check completed. |
| EntityAnimal | Not done | Partial: flee behavior and animal despawn suppression, tested through animal replays. Full changed-class audit/interaction matrix remains. |
| EntityMob | Not done | Partial: source-aware attacks, XP and movement, tested through hostile replays. Full changed-class audit remains. |
| EntityPigZombie | Not done | PigZombie movement replay EXACT. Anger and group-retaliation changes need targeted proof. |
| EntityDamageSource | Done | entities181: rules, poison, allies, kill-* exercise entity attribution. |
| EntityDamageSourceIndirect | Done | entities181: rules plus indirect Enderman projectile fixture; vanilla owner accessor behavior preserved. |
| PotionHealth | Done | entities181: rules and potions (instant health/harm shift behavior). |
| EntityMinecart | Not done | Minecart replay EXACT. Full vehicle-specific changed-class verification remains. |
| EntityPainting | Not done | DamageSource adapter added; decoration-specific oracle proof remains. |
| EntitySpider | Done | entities181: Spider and CaveSpider (shared movement, climbing and collision behavior). |
| EntityItem | Done | entities181: Item; shared entity state/RNG checks. |
| EntityList | Not done | New mob/XP names wired; creation exercised in replay. Full numeric registry and NBT registry matrix remains. |
| EntitySheep | Done | entities181: Sheep and kill-Sheep; population records initialization color/RNG. |
| EntitySnowball | Done | entities181: Snowball (motion, collisions and thrown damage source). |
| EntitySquid | Not done | Squid movement replay EXACT; aquatic base and XP hooks present. Full changed-class audit remains. |
| EntityWaterMob | Not done | Aquatic replay EXACT via Squid; full changed-class audit remains. |
| Vec3D | Done | entitygeometry: 10,000 cases; new dot-product use covered by Enderman targeting fixture. |
| EntityFish | Not done | Thrown DamageSource adapter added; fishing hook oracle proof remains. |
| EntitySlime | Not done | Slime movement replay EXACT; XP size hook present. Full size/death/split oracle matrix remains. |
| DataWatcher | Not done | State stored locally on entities; typed watcher registry/serialization not ported. |
| EntityCreeper | Not done | Creeper movement replay EXACT; explosion/damage branches need dedicated verification. |
| EntityGhast | Not done | Ghast movement replay EXACT; attack/explosion behavior needs dedicated verification. |
| AxisAlignedBB | Done | entitygeometry: 10,000 cases. |
| EntityZombie | Done | entities181: Zombie and kill-Zombie (new rotten-flesh drop). |
| Explosion | Not done | Large explosion event name updated; complete explosion event/render proof remains. |
| EntityFlying | Not done | Ghast movement replay EXACT; full changed-class audit remains. |
| PathEntity | Done | entities181: creature path replay through new and existing mobs. |
| ModelEnderman | Done | models181: constructor geometry, UVs, successive animation poses and bitwise vertex/part values; 208 poses, 293,376 comparisons across the suite. |
| TileEntityChestRenderer | Not done | No complete b1.8.1 changed-class port and dedicated EXACT check completed; inherited implementation remains. |
| RenderEnderman | Not done | Partial: model carrying/screaming and eye pass wired. Held block draw, scream jitter, lightmap and dedicated render trace remain. |
| RenderArrow | Not done | No complete b1.8.1 changed-class port and dedicated EXACT check completed; inherited implementation remains. |
| RenderLiving | Not done | Partial: new mob transforms and eye passes. Embedded arrows and packed lightmap/render trace remain. |
| EntityCritFX | Done | particles181: constructors, 202 successive state/vertex frames per variant; 184 cases total, 594,688 vertices, zero differing fields. Packed world lightmap integration is a separate remaining dependency. |
| EntityLargeExplodeFX | Not done | No complete b1.8.1 changed-class port and dedicated EXACT check completed; inherited implementation remains. |
| EntityCloudFX | Done | particles181: constructors, 202 successive state/vertex frames per variant; 184 cases total, 594,688 vertices, zero differing fields. Packed world lightmap integration is a separate remaining dependency. |
| ModelSilverfish | Done | models181: constructor geometry, UVs, successive animation poses and bitwise vertex/part values; 208 poses, 293,376 comparisons across the suite. |
| ModelRenderer | Not done | Texture dimensions/UV changes exact in models181. ModelBase registration/random part selection and raw box bounds for embedded arrows remain. |
| RenderXPOrb | Done | xprender181: 11 values x 8 ages x 4 partial ticks = 352 original-bytecode draw traces, 21,120 bit comparisons. |
| EntityCrit2FX | Not done | No complete b1.8.1 changed-class port and dedicated EXACT check completed; inherited implementation remains. |
| RenderSilverfish | Not done | Partial: model/texture/death rotation wired. Dedicated renderer trace and lightmap remain. |
| EntityAuraFX | Done | particles181: constructors, 202 successive state/vertex frames per variant; 184 cases total, 594,688 vertices, zero differing fields. Packed world lightmap integration is a separate remaining dependency. |
| EntitySuspendFX | Done | particles181: constructors, 202 successive state/vertex frames per variant; 184 cases total, 594,688 vertices, zero differing fields. Packed world lightmap integration is a separate remaining dependency. |
| ModelChest | Not done | Constructor geometry exact in models181. Complete lid-angle animation/render integration not completed. |
| EntityPortalFX | Not done | State/vertices exact in particles181; packed brightness override remains. |
| EntityHugeExplodeFX | Not done | No complete b1.8.1 changed-class port and dedicated EXACT check completed; inherited implementation remains. |
| ModelWolf | Done | models181: constructor geometry, UVs, successive animation poses and bitwise vertex/part values; 208 poses, 293,376 comparisons across the suite. |
| ModelSpider | Done | models181: constructor geometry, UVs, successive animation poses and bitwise vertex/part values; 208 poses, 293,376 comparisons across the suite. |
| ModelSkeleton | Done | models181: constructor geometry, UVs, successive animation poses and bitwise vertex/part values; 208 poses, 293,376 comparisons across the suite. |
| EntityFlameFX | Not done | State/vertices exact in particles181; packed brightness override remains. |
| ModelLargeChest | Not done | Constructor geometry exact in models181. Complete lid-angle animation/render integration not completed. |
| EntityLavaFX | Not done | State/vertices exact in particles181; packed brightness override remains. |
| ModelBiped | Done | models181: constructor geometry, UVs, successive animation poses and bitwise vertex/part values; 208 poses, 293,376 comparisons across the suite. |
| ModelChicken | Done | models181: constructor geometry, UVs, successive animation poses and bitwise vertex/part values; 208 poses, 293,376 comparisons across the suite. |
| ModelCreeper | Done | models181: constructor geometry, UVs, successive animation poses and bitwise vertex/part values; 208 poses, 293,376 comparisons across the suite. |
| RenderSpider | Not done | Partial: CaveSpider scale/texture and eye pass updated. Dedicated renderer trace/lightmap remain. |
| ModelPig | Done | models181: constructor geometry, UVs, successive animation poses and bitwise vertex/part values; 208 poses, 293,376 comparisons across the suite. |
| RenderPlayer | Not done | No complete b1.8.1 changed-class port and dedicated EXACT check completed; inherited implementation remains. |
| EffectRenderer | Not done | Partial: new particle kinds, terrain-aware initial ticks and cloud player attraction integrated. Packed lightmap/lit pass, large/huge explosions and crit2 emitter remain. |
| EntityPickupFX | Not done | Not verified: particles181 explicitly emits zero pickup cases. Lit pickup renderer integration remains. |
| ModelQuadruped | Done | models181: constructor geometry, UVs, successive animation poses and bitwise vertex/part values; 208 poses, 293,376 comparisons across the suite. |
| RenderManager | Not done | No complete b1.8.1 changed-class port and dedicated EXACT check completed; inherited implementation remains. |
| RenderMinecart | Not done | No complete b1.8.1 changed-class port and dedicated EXACT check completed; inherited implementation remains. |
| EntitySlimeFX | Done | particles181: constructors, 202 successive state/vertex frames per variant; 184 cases total, 594,688 vertices, zero differing fields. Packed world lightmap integration is a separate remaining dependency. |
| ModelSlime | Done | models181: constructor geometry, UVs, successive animation poses and bitwise vertex/part values; 208 poses, 293,376 comparisons across the suite. |
| ModelBase | Not done | Model geometry exact, but part registry/random-part API not completed. |
| ModelCow | Done | models181: constructor geometry, UVs, successive animation poses and bitwise vertex/part values; 208 poses, 293,376 comparisons across the suite. |
| ModelMinecart | Not done | No complete b1.8.1 changed-class port and dedicated EXACT check completed; inherited implementation remains. |
| ChestItemRenderHelper | Not done | No complete b1.8.1 changed-class port and dedicated EXACT check completed; inherited implementation remains. |
| ModelBoat | Not done | No complete b1.8.1 changed-class port and dedicated EXACT check completed; inherited implementation remains. |
| RenderBiped | Not done | No complete b1.8.1 changed-class port and dedicated EXACT check completed; inherited implementation remains. |
| TileEntityRenderer | Not done | No complete b1.8.1 changed-class port and dedicated EXACT check completed; inherited implementation remains. |
| EntityDiggingFX | Done | particles181: constructors, 202 successive state/vertex frames per variant; 184 cases total, 594,688 vertices, zero differing fields. Packed world lightmap integration is a separate remaining dependency. |
| ModelBed | Not done | No complete b1.8.1 changed-class port and dedicated EXACT check completed; inherited implementation remains. |
| ModelGhast | Done | models181: constructor geometry, UVs, successive animation poses and bitwise vertex/part values; 208 poses, 293,376 comparisons across the suite. |
| ModelSquid | Done | models181: constructor geometry, UVs, successive animation poses and bitwise vertex/part values; 208 poses, 293,376 comparisons across the suite. |
| TexturedQuad | Done | models181: constructor geometry, UVs, successive animation poses and bitwise vertex/part values; 208 poses, 293,376 comparisons across the suite. |
| ModelSheep1 | Done | models181: constructor geometry, UVs, successive animation poses and bitwise vertex/part values; 208 poses, 293,376 comparisons across the suite. |
| Render | Not done | No complete b1.8.1 changed-class port and dedicated EXACT check completed; inherited implementation remains. |
| RenderBoat | Not done | No complete b1.8.1 changed-class port and dedicated EXACT check completed; inherited implementation remains. |
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

## Remaining integration and scope

The major remaining work is full entity/particle packed lightmap brightness, renderer traces and embedded arrows; chest tile rendering/lid easing/item helper; large/huge explosion and crit2 particles; typed DataWatcher and complete registry/network APIs; the changed-class audits and combat/interaction checks marked above. No claim is made that existing livingrenderers or general renderer scenes are exact.

Minimum cross-area dependencies were added locally: block IDs 97/98/99/100/103 get collision/material properties needed by mobs; silverfish emergence uses local entity logic. Full block drops/behavior remain the world-blocks owner's work. Player XP, orb pickup cooldown and exhaustion fields support entity mechanics; full FoodStats, creative/blocking and player persistence remain the player-gui owner's work. New biome lists/population need the terrain owner's biome and generation entry points. GPU/world lightmap integration belongs with the integrator and still needs the entity/particle packed-brightness APIs supplied hereafter.

Vanilla quirks retained include Enderman's `carriedData` write / `carryingData` read mismatch, population Z-range using width, potion merge behavior, Java shift masking and effect HashMap traversal order. The commit script stages an explicit list, uses the requested identity and both co-author trailers. No commit was issued by this worker.
