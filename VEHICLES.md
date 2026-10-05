# Beta 1.7.3 vehicles

Minecarts, boats and riding are literal singleplayer ports of the original
client. The oracle executes Mojang's original vehicle and world bytecode on
the launcher's Java 8u51 runtime, with the existing deterministic RNG hooks.

## Implementation

`crates/worldgen/src/entity/vehicles.rs` implements all three minecart types,
every rail shape, slope acceleration, powered-rail boosts/braking, speed caps,
derailing, rail projection helpers, furnace propulsion, cart/cart and cart/mob
collisions, automatic mob pickup, damage, hurt animation and drops. Detector
rails query the actual carts and schedule the vanilla 20-tick occupancy check.

Boats implement the five buoyancy slices, rider steering, drag, speed caps,
collision damage, snow removal, splash particles and their exact random draws.
Breaking drops three planks and two sticks. Boats and carts participate in
world collision-box queries and entity picking.

Placement creates real entities. Right-clicking toggles riding, opens a chest
cart's 27-slot inventory, or feeds coal to a furnace cart. The existing container
system reads and writes the cart inventory and closes when the cart dies or
the player moves beyond eight blocks. Mounting, recursive rider ticks, rider
offsets, yaw/pitch deltas, animation resets and damage dismounting retain the
original order. Riding distance emits stats events; the client counts minecart,
boat and pig distance and consumes the existing achievement event for onARail.

Vehicle NBT uses the vanilla `Minecart` and `Boat` names. Cart type, push vector,
signed-short fuel and chest contents retain the original conversions. Empty
item lists retain the vanilla byte element type. Boats only write base entity
fields. Existing chunk persistence loads and saves these entity kinds.

`crates/client/src/render/entity/vehicles.rs` ports ModelMinecart, ModelBoat,
RenderMinecart and RenderBoat. This includes rail projection/interpolation,
hurt wobble and the chest/furnace blocks. Textures are loaded from the user's
original jar. The world worker sends a separate rail-cell snapshot for cart
projection; the existing camera receives the rider's updated position.

## Verification

Run `bash tools/verify_vehicles.sh`, or the full `bash tools/verify.sh` which
includes it. A successful focused run ends `VEHICLES ALL CASES EXACT`.

| Check | Inputs and comparisons |
|---|---|
| rails | 16 carts covering all ten rail shapes, powered/unpowered boosters and detector rails |
| train | Furnace cart pushing two carts, opposing cart, mob pickup, derailment and a chest cart broken with items |
| boats | Still/flowing water, collisions between boats, wall crashes, splashes and attack damage |
| riding | Player input/look, a corner and detector rail, mounting/toggling, boat breaking, coal, chest breaking and a saddled pig |
| nbt | Reconstruct a cart train from NBT every 25 ticks |
| longrail | Riding distance and onARail, detector scheduling, and sneak input while mounted |
| ridingnbt | Save and reconstruct the player and vehicles mid-ride every 25 ticks |
| placing | All cart items and boat placement, including snow above water |
| inventory | 10,000 original API operations: get/set/decrement, signed counts, NBT reconstruction, fuel truncation, interaction distance and hurt animation |
| renderers | 408 cases, 16,200 draws and 1,582,650 raw-bit comparisons; all cart types, ten rail shapes/off-track, four partial ticks, hurt states and boats |

Each replay runs 240 ticks and compares the initial frame and every subsequent
frame: position/motion/rotation/bounding-box bits, collision flags, vehicle
fields and inventory, riding links and angle deltas, entity/world/Math RNG,
ordered sound/particle callbacks, rail helper queries, stats events, and
canonical vehicle NBT. Final world dumps compare blocks, metadata, light,
height maps, tile contents, scheduled ticks and lighting queue size. No float
tolerance is used. Additional Rust integration tests exercise a chest cart
window, transfers, distance closure/cursor drops, mounting toggles and breaking
a ridden boat through the recursive tick path.

Validation on October 4, 2026: the complete 137-section `tools/verify.sh` run
finished `ALL CASES EXACT`; `cargo test --workspace` passed all 28 tests.
Logs are in `target/vehicles-verify-full.log`, `target/vehicles-verify-final.log`
and `target/vehicles-tests.log`. Oracle compilation uses a success stamp so
partial output from a failed javac invocation cannot hide a required rebuild.

## Original behavior retained

Two details in the task description differ from Beta 1.7.3 itself:

* Sneaking does not dismount. Right-click toggles riding; breaking a vehicle or
  calling mountEntity(null) dismounts. The longrail replay explicitly supplies
  sneak input and compares the retained links against the original.
* Entity.writeToNBT/readFromNBT do not save ridingEntity/riddenByEntity.
  Mid-ride reconstruction therefore loses the links on both sides. The
  ridingnbt replay checks this exact behavior, including the player.

Other retained quirks include the constructor's 36 cargo slots versus 27
accessible/loadable slots; signed malformed stack counts and fuel; the cart
collision alignment expression using the other cart's prevPosX; the second
zero-count chest-content drop pass consuming offset RNG; and minecart updates
skipping the base Entity.onUpdate call. onARail uses the original misleadingly
named getSqDistanceTo, which actually returns a square root.

Exactness is established for these inputs. The renderer check covers recorded
geometry, UVs, colors, normals and GL operations, rather than a new vehicle GUI
pixel-diff or a guarantee for every possible GPU/world state. Multiplayer
interpolation is outside this singleplayer project's scope.

## Delivery stages

The commit script `target/commit-vehicles.sh` separates simulation/oracle,
client integration, and verification/documentation stages. It uses the task's
required author and committer identity and both required co-author trailers.
The sandbox cannot write the worktree's external Git index, so the script is
provided for execution outside that sandbox.
