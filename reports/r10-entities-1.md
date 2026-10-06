# Release 1.0 entities and player, round 1

Source: the task-local `tools/retromcp-r10` tree was absent. Release source was read from the existing sibling `../beta-rs/tools/retromcp-r10/minecraft/src/net/minecraft/src`; Beta source was read locally. Numbered symbol/local-variable renames were normalized before auditing deltas. No original-bytecode oracle was available or created. **Ported means source delta implemented, not bytecode-verified.** Partial rows identify concrete cross-area integration gaps.

The new dragon, mobs, breeding and projectiles run in `worldgen10`; abstract Java overrides are flattened into the Rust entity dispatch. New mob renderers/models, End generation/transition and new screens are owned by other workers and are not supplied here. The smoke client can exercise the existing terrain/rendering path; it cannot establish visual correctness for those new mobs.

## Validation

- `cargo test -p worldgen10`: passed: 33 tests, including 16 library tests and the new spawn-predicate regression.
- `cargo build --release`: passed for the whole workspace. Existing output-name collision and unused-code warnings remain.
- `target/release/beta.exe --mc 1.0 --seed 42 --no-save --shot target/r10-smoke.png --shot-ticks 60`: exited 0. Screenshot inspected: forest/coast terrain, sky, hand and HUD render. The inherited Beta 1.8.1 overlay label is still present (client-area branding).
- Added focused dragon, potion, XP, capabilities, armor and spawn-predicate tests. Existing integration assertions were corrected to compare canonical NBT values and typed jukebox persistence. The pre-existing Nether test's boxed-array comparison was fixed; its climate/light/malformed-NBT expectations were updated to the current source behavior without changing world generation.

## Class audit and next original-bytecode checks

| ClassName | Status | What changed, what remains; next original-bytecode check |
|---|---|---|
| EntityDragon | Ported | Flight targeting, 64-entry history, seven parts, crystal healing, collisions, block scans, 200-tick death and exit portal. Check constructor RNG, each flight branch, part damage, both block scans and XP/portal draw order. |
| EntityFishHook | Ported | EntityFish rename; normalized source has the inherited fishing algorithm. Check cast, attached entity, submerged bite RNG, reel result and NBT. |
| EntityLiving | Partial | Max health, armor/resistance remainder, jump delay, air watcher, children, potion color/particles, damage/fall sounds and break-particle callback. Looting-aware drop API exists; tagged ItemStack/EnchantmentHelper wiring and automatic break callbacks remain external. Check damage remainders, drowning, death XP, potion HashMap order and break RNG. |
| EntityThrowable | Ported | Shared egg/snowball/pearl/potion construction and update loop; server-only entity interception and subclass gravity/velocity. Check heading casts, owner exclusion, water drag, in-ground expiry, hit-then-advance order and NBT. |
| EntityEgg | Ported | Shared throwable loop and hatch sequence; chicks start at age -24000. Check impact draw order, one/four chick branches and client/server split. |
| EntityPlayer | Partial | Float XP, level cost, abilities NBT, potion combat/mining, critical hit, sprint knockback, safety checks, drink sound, armor and copy API. Two-pass item icon selection is exposed; EnchantmentHelper/tagged stacks and automatic item-break callbacks await blocks/client integration. Copy preserves food values for respawn but cannot retain Java aliasing between two concurrently retained players. Check XP float bits, double armor/resistance pass, critical RNG, unsafe sleep, copy identities and capabilities save bug. |
| EntitySnowball | Ported | Shared throwable loop; impact does three damage to Blaze. Check owner-null damage, collision order and poof particles. |
| EntityAnimal | Ported | Age watcher/NBT, wheat interaction, love, target selection, mating counters, offspring and cooldowns. Check positive addCoord search box, feeding in creative, paired tick order, sheep inheritance draw and 60-tick breeding. |
| EntityBlaze | Ported | Hover/wet damage, smoke/fire RNG, charge watcher and three-small-fireball burst, rod drop and full brightness. Check pre-super order, Gaussian casts, all attack steps and supplied looting levels. |
| SpawnerAnimals | Partial | Border/interior eligibility, species selection timing, release End/Hell/mushroom lists and removed sleep ambush. Fortress lookup and new biome reachability require world-area generation/provider work. Check Java chunk iteration, caps, failed-attempt draws, per-pack selection and fortress override. |
| EntityEnderEye | Ported | Target guidance, network velocity, particles, 80-tick lifetime and 4/5 drop decision. Check float horizontal distance, 12-block target clamp and terminal item/burst RNG. |
| Potion | Ported | Release registry, colors/icons/effectiveness, undead inversion, instant effects and duration formatting. Minimal PotionHelper expression evaluator supplies splash effects. Check all registry entries, full metadata range, instant owner attribution and float color accumulation. |
| EntityMinecart | Ported | Damage/direction/hit-time watchers, powered flag, smoke order, every-tick fuel consumption, network idle position and dead rider unlink. Check server/client interpolation, last fuel tick, damage thresholds and watcher updates. |
| EntityEnderman | Ported | 40 health, 7 damage, vulnerable-player stare, carried block set, wet/sun teleport, portal sounds and indirect-hit evasion. Check 64 failed teleport trials, carried metadata NBT, daylight short circuit and RNG order. |
| EntityBoat | Ported | Damage/direction/hit-time watchers; splashes before movement/interpolation and collision threshold uses starting speed. Check splash count/order, network branch, break drops and damage/watchers. |
| PlayerControllerCreative | Ported | PlayerControllerTest rename; creative initialization and inventory/controller behavior retained. Check mode capability resets, initial hotbar, immediate breaking and item restoration. |
| Entity | Ported | Air watcher, max-preserving fire duration, lava fall-distance halving, opaque probes, step helper, multipart identity and boolean push-out. Check watcher initialization/NBT, fire sources, collision stepping and boundary casts. |
| InventoryPlayer | Partial | Metadata selection, nonstackable copying, creative overflow, armor sum/wear and deep-copy identities. Tagged ItemStack copies and enchantment-sensitive durability depend on blocks-area ItemStack. Check damaged armor, whole-stack insertion, partial-insertion return bug and fresh copy identity. |
| EntityPotion | Ported | Constructors, pitch/velocity/gravity overrides, source expression metadata supplier, splash distance/direct-hit strength, instant/long effects and auxiliary effect 2002. Check all metadata/effect durations, >20 cutoff and rounded coordinates. |
| EntityMagmaCube | Ported | Slime subclass factory, armor, fire immunity, flame particles, jump height/delay, squish decay, attack size+2, brightness and no drops. Check constructor draws, size-one collision, split children, landing sounds and supplied looting. |
| EntitySnowman | Partial | 4 health, hostile targeting, snow placement, heat damage, snowball attack and snowball drops. Uses existing biome temperatures and block support; new release biome temperatures require world-area data. Check four snow positions, climate thresholds, world-vs-entity RNG and heading draw order. |
| EntitySlime | Ported | Smoothed squish, peaceful removal, randomized split count, subtype factory, sounds/attack hooks, 40-block spawn ceiling and spawn collision validation. Check sizes, split RNG, landing/jump sounds and slime-chunk short circuit. |
| EntityEnderCrystal | Ported | Size/health/watcher, random rotation, no-super update, fire at crystal base and power-six explosion. Check initial watcher zero, first rotation tick, repeat attacks and explosion order. |
| EntityVillager | Ported | Profession/NBT, profession texture selection, 20 health, movement, sounds and persistent despawn policy. Check profession values 0-4/unknown, constructor/read texture and idle movement RNG. |
| EntitySmallFireball | Ported | Small constructor size, owner initialization, five damage/fire on successful hit, face-adjacent block ignition and no deflection/collision. Check immune targets, failed damage, all hit faces and constructor Gaussian draws. |
| DamageSource | Ported | Fire/projectile flags, indirectMagic, magic bypass/exhaustion and existing owner accessor semantics. Check every static/factory source and null-owner accessors. |
| EntityPlayerSP | Partial | Blindness/sneak sprint rules, health setter bug, float XP setter, portal achievement and End/screen/magic-crit request APIs. End transition/credits, enchantment/brewing screens and magic-crit renderer need other-area consumers; requests are exposed but not displayed. Check portal draws/achievement order, health deltas, XP bits and End requests. |
| EntitySpider | Ported | Climbing watcher, arthropod attribute, poison immunity and looting-aware extra eye drop. Check server watcher writes, client climbing, poison rejection and short-circuit drop draws. |
| Explosion | Ported | Fire-placement pass moved after block destruction/particle pass. Check ordered affected blocks, world RNG continuation, drop callbacks and flaming destruction. |
| PotionEffect | Ported | Copy/equality/hash/name/description and stronger/equal-amplifier combine rules. Check mismatched IDs, duration shortening rejection, readiness/decrement and descriptions. |
| EntityMooshroom | Ported | Cow subclass, adult bowl replacement, adult shear conversion preserving health/body yaw, explosion particle and five mushrooms. Check baby rejection, stacked bowl replacement, cow constructor RNG and unchanged shears durability. |
| EntityFireball | Ported | Owner-null/dead removal, fire status, unchanged shake on tick, server impact damage/explosion, owner transfer on deflection and continued hit-tick physics. Check network/server differences, null owner, deflection and brightness. |
| EntitySnowmanBase | Ported | Flattened shared overrides: no fall damage, no despawn, none sound and talk interval 120. Check sound attempts/RNG and inherited creature NBT. |
| EntityEnderPearl | Ported | Throwable constructors, zero-damage hit, 32 portal particles, owner teleport/fall reset/five fall damage. Check particle draw order, non-player owners, multiplayer impact and hit-tick continuation. |
| DragonPart | Ported | Seven unregistered child entities forward part attacks, compare identity to parent and participate through parent AABB queries. Check entity/RNG ID allocation, query intersection gating and head/body damage forwarding. |
| EntityGhast | Ported | 10 health, vulnerable-player targeting, auxiliary charge/fire effects, reflected fireball 1000 damage and tear/gunpowder drops. Check charge timeline, omitted sound RNG, achievement and supplied looting draws. |
| EntitySilverfish | Ported | 8 health/1 damage, arthropod, vulnerable targeting, source sounds/step override and spawn predicate. Check infestation/wakeup scan, dark-check bypass, five-block player exclusion and step callback. |
| PlayerCapabilities | Ported | Abilities compound read/write with original flying=disableDamage serialization bug. Check missing/wrong tags, all four flags and creative-to-survival reset. |
| EntityDragonBase | Ported | Flattened base overrides: max health, rejected direct damage, part damage forwarding and explicit super damage path. Check base/dragon health initialization and dispatch through every part. |
| EntityList | Ported | Release entity names and numeric mappings, factories/NBT for new registered entities; potion remains unregistered as original. Check IDs 12-15, 61-63, 96-97, 120, 200 and old names. |
| EntityMob | Ported | Potion strength/weakness melee arithmetic and vulnerable-player selection. Check modifier shifts, creative targets, armor inheritance and target distance. |
| EntitySheep | Ported | 8 max health, inherited wheat interaction and offspring chooses parental fleece after child construction. Check shearing/wheat fallthrough and parent-color Boolean draw. |
| WorldSettings | Ported | Hardcore field/construction; level.dat/scenario propagation and forced hard difficulty. Hardcore world-creation/death UI belongs to client area. Check constructors, save/reload and difficulty enforcement. |
| EntitySquid | Ported | 10 health, entity-age-driven swim stop and y>45/y<63 spawn bounds with base collision test. Check age 100/101, omitted idle RNG and water animation. |
| EntityZombie | Ported | 20 max health, four melee damage, two armor and undead attribute; sun ignition uses max fire duration. Check daylight/wet branches, armor remainder and undead effects. |
| EntityChicken | Ported | 4 health, child factory, baby egg suppression and looting-aware feather drops. Check age-zero transition before egg tick and additional nextInt(looting+1) draw. |
| EntityCow | Ported | 10 health, breeding factory and looting-aware leather/beef count and burning selection. Check drop draw order, inherited feeding and milk behavior. |
| EntityPig | Ported | 10 health, breeding factory and feeding-before-mount interaction. Check saddled wheat use, mount fallback, child age and inherited meat drops. |
| EntityPigZombie | Ported | Undead/armor/health/strength values, rotten flesh and gold nugget drops. Check group anger, fire immunity and both looting-dependent bounds. |
| EntityWolf | Ported | 8/20 health policy, breeding factory, inherited animal state and angry-only despawn override. Check tame watcher health, anger distance/RNG, feeding, child and owner interaction. |
| EntityCreature | Ported | Wander probability 180 and draw/age/flee short-circuit order; breeding attack dispatch. Check path/no-path branches, age boundary and both wander draws. |
| EntitySkeleton | Ported | 20 max health, undead attribute and looting-aware arrow/bone bounds. Check daylight ignition, ranged attack, sniping and both drop bounds. |
| EntityArrow | Ported | Critical impact damage RNG and random.bowhit sound. Check zero-owner attribution, critical hit bound, deflection, stuck arrows and post-impact motion. |
| EntityXPOrb | Ported | No constructor Math.random phase draw, packed-light increment 120, orb pickup sound and float-XP player award. Check IDs/RNG after construction, pickup delay and XP splits. |
| EntityItem | Ported | Diamond/blaze-rod achievements and creative full-inventory absorption. Check zero/partial/full insertion, pickup sound and achievement/stat order. |
| PotionHealth | Ported | Instant heal/harm readiness/registry behavior with six-shift magnitude. Check duration 0/1, undead inversion and both amplifiers. |
| EntityFallingSand | Ported | First-tick matching block removal, mismatch death, piston exception and server-only item drops. Check mismatch continuation, piston landing and timeout placement failure. |
| EnumCreatureAttribute | Ported | Undefined/Undead/Arthropod and subclass classification. Check all living types and potion acceptance. |
| EntityCaveSpider | Ported | 12 max health, inherited climbing/arthropod/poison immunity and existing poison attack. Check difficulty-dependent poison duration and watcher behavior. |
| EntityGiantZombie | Ported | 100 max health through release health accessor; existing giant movement/combat retained. Check spawn/constructor health, scaling and path brightness. |
| EntityWaterMob | Ported | 10-health shared initialization and existing water spawn/XP behavior. Check base water predicate and world RNG XP draw. |
| ChunkPosition | Ported | Vec3 constructor floors each component; existing hashing retained. Check negative/large/nonfinite coordinates and hash bits. |
| DataWatcher | Ported | Typed short/int access and release entity watcher entries. Check signed air truncation, byte/int/short packet round trips and dirty-list order. |
| EntityCreeper | Ported | 20 max health through release accessor; inherited fuse/powered logic retained. Check constructor, fuse/explosion and saved watcher flags. |
| PathEntity | Ported | Normalized delta removes unused getPathEnd; current path operations retain source behavior. Check path index, endpoint, equality and position offset. |
| PlayerController | Ported | Survival capabilities reset flying/mayfly/creative/invulnerable; creative mode uses corresponding initialization. Check both transitions and loaded capabilities. |
| EnumStatus | Ported | Added NotSafe sleep status. Check unsafe-mob bed result before bed state mutation and existing other rejection statuses. |
| FoodStats | Ported | Health accessor rename has equivalent behavior; release XP/capabilities live beside unchanged food algorithm. Check exhaustion single-step, healing/starvation thresholds and NBT. |

Additional explicitly assigned class: **PathFinder — Ported (retained)**. Comparing `Pathfinder` to `PathFinder` after symbol normalization leaves only `MCHash` -> `IntHashMap`; no path algorithm delta. Next checks: open-set ordering, point-map collisions, four neighbors, fence/water/lava flags, fall limits and reconstructed path order.

## Cross-area adapters and remaining integration

- Minimal `client10` HUD/shot changes consume fractional XP and release cap arithmetic; one worker WorldSettings literal supplies `hardcore=false` for converted old maps.
- Minimal WorldInfo/storage/scenario adapters persist hardcore; world tick forces difficulty 3. No hardcore creation/death screens were added.
- Dragon-created portal/egg blocks bypass old unsupported block-added assertions. Full BlockEndPortal/TileEntityEndPortal transport and BlockDragonEgg behavior remain blocks/world work.
- The player item registry adapter provides IDs 369-382 and release armor material values/durability so new drops and unenchanted armor work. Full ItemPotion/ItemEnderPearl/ItemEnderEye actions, ItemStack tags, brewing and enchantments remain blocks work. PotionHelper's expression evaluator was copied only as the EntityPotion effect supplier.
- `drop_few_items_with_looting` implements release mob formulas and RNG for a supplied looting level; ordinary deaths currently supply zero because EnchantmentHelper/tagged held items are absent. Breathing/mining/protection/knockback/fire-aspect enchantment paths still need that integration. The living broken-item particle API is present; ItemStack.damageItem must invoke it in the exact source position.
- EntityPlayerSP exposes End/credits/magic-critical events and enchantment/brewing opening requests. Other-area world/screen/render consumers must handle them. Existing nether travel remains operational.
- Snowman's temperature lookup uses existing biome records; release cold/mushroom biome registration and Nether fortress spawn override await world work.

No protected Beta crates, oracle harnesses or verify scripts were edited. No commit was made. `target/commit-r10-entities.sh` stages only this change's explicit file list and provides the requested author/co-author metadata.
