# Beta 1.7.3 singleplayer player

The simulation lives in `player.rs` and these modules. The client crate is
unchanged. `EntityKind::Player` uses the shared entity/living physics and world
update order; its inventory, camera, sleeping, portal, and controller state are
in `PlayerState`.

## Client integration

```rust,ignore
use worldgen::entity::player::{PlayerInput, PlayerTickInput};

// Set world.info from the loaded level.dat before this call. A saved Player
// compound is consumed once, after vanilla spawn preparation.
let player = world.spawn_player("Player");

// Call once per unpaused 20 Hz game tick, with the client's tick number.
world.player_tick(player, PlayerTickInput {
    movement: PlayerInput {
        forward: true,
        jump: jump_key_down,
        sneak: sneak_key_down,
        yaw_delta: mouse_dx,
        pitch_delta: mouse_dy,
        ..Default::default()
    },
    left_pressed: left_button_went_down,
    left_held: left_button_down,
    right_pressed: right_button_went_down,
    right_held: right_button_down,
    ..Default::default()
}, ticks);

let hud = world.player_hud(player, partial_tick).unwrap();
let events = world.take_player_events(player);
```

Mouse deltas are the inputs to vanilla `Entity.setAngles`, including its 0.15
scale and pitch clamp. Apply mouse sensitivity/smoothing in the client before
passing them. `PlayerTarget::RayTrace` runs `EntityRenderer.getMouseOver` with
SP block reach 4 and entity reach 3. `ObjectMouseOver(Some(hit))` supplies a
renderer result; `ObjectMouseOver(None)` explicitly supplies a miss. Pressed
buttons are edges; held buttons describe the state after those edges. The API
preserves vanilla's shared five-tick click repeat and left-click miss cooldown.

The complete tick runs input/controller actions, surrounding chunk joins every
30 ticks, entities and tiles, `World.tick`, then random display updates.
`apply_player_tick_input` exposes just the controller/input phase for a client
that already owns this tick sequence. Calling both complete tick methods would
tick the world twice.

Entity position is the player's eye/camera position, with vanilla `yOffset`
included by `set_location_and_angles`; the bounding-box minimum is the feet.
The HUD snapshot includes health, previous health, heart timer, air, armor,
inventory/selected slot/held item, portal and sleep state, brightness, eye
height, camera bobbing, walked distance, swing and interpolated mining progress.
The entity exposes previous/current position and rotation for interpolation.

Events request pickup presentation, a dimension switch, a container screen, or
a vehicle spawn. `SpawnVehicle` includes boat/cart kind and vanilla placement
position; vehicle physics remains a client/integration hook. Container events
identify the block and coordinates so the screen can find its inventory.
The caller handles rendering, sound, particles, screens, pause/focus, and world
switching. Stats/achievements are not maintained by this simulation.

## Persistence and world membership

`player_nbt` writes the original SP `Player` compound, including signed stack
fields, armor slots 100–103, Score, sleeping/spawn tags, and Dimension.
`World.save_level` replaces the raw Player passthrough with this live state.
Players are excluded from chunk EntityList tags. Vanilla does not save selected
hotbar slot, controller, camera, portal timer, input, or random streams.

`World.players` remains a position cache for existing camera-only callers;
`player_entities` associates entries with real player IDs. Mob targets resolve
to the real entity when available. Natural death removes the player from the
loaded entity/chunk update lists but keeps the object for HUD and level saving,
as vanilla keeps `Minecraft.thePlayer`. Respawn UI and explicit replacement of
that object remain the caller's responsibility.

## Verification

Run `bash tools/verify.sh` (or `quick`). The player oracle constructs an original
`EntityPlayerSP` against the original world/controller classes. Its subclass
only suppresses stats and renderer/GUI callbacks; portal switching is recorded
as a request. API cases also invoke the original private `Minecraft.clickMouse`
and held-dig methods, and picking invokes the original `EntityRenderer`.
Original bytecode is retained apart from the existing deterministic Entity and
Explosion Random initializer hooks, plus the display-call Random hook. Tests require Mojang Java 8u51.

The suite compares every recorded frame and the final world dump, including
float/double bits, inventory/armor, shared living state, camera/cloak/swing,
portal/sleep/controller state, entity and world random streams, blocks,
metadata, light, height maps, tile contents, scheduled ticks, and lighting queue
size. The NBT replay compares canonical serialized NBT bytes every frame
(compound key order is sorted; types, values and list order are preserved).
Inventory exercises 10,000 original operations. Player cases cover movement,
damage/pickups, lava, drowning, peaceful regeneration, death/Notch drops, beds
and sleep spawning/day skip, portals, NBT reconstruction, block/entity picking,
tool-based digging, all six placement faces, food, buckets, projectiles, animal
interactions, bone meal, container activation, click timing, and the full tick
pipeline. Rust integration tests also check real level.dat/region persistence,
the public tick/HUD/events API, and dead-player retention.

`tools/oracle/build_player_registry.py` regenerates `registry.bin` from the
original Item/Block methods: stack limits, durability, subtypes, armor,
tool strength/harvest requirements, hardness, and weapon damage.
`diff_player.py oracle.bin target/player-rust.bin ticks scenario` decodes a
failed replay. The compare command writes `player-rust.bin` only on failure.

## Boundaries

Exactness is established for the replay inputs, not every possible state.
Existing unsupported block machinery remains: full redstone/piston circuits,
furnace smelting, filled dispenser operation, and scattering filled furnace/
dispenser inventories. Empty furnace/dispenser placement, activation and NBT
creation are implemented; their existing raw saved contents are preserved.
Sapling random growth, maps, fishing rods, signs/paintings and redstone
dust item use are not implemented. The requested foods, buckets, flint/steel,
bow, snowballs/eggs, hoe/seeds, shears and dyes are implemented. Boats and carts
use explicit spawn requests, as permitted by the task.

Finished stages are stored in `target/player-stages` and can be committed with
`bash target/commit-player.sh` when Git metadata is writable. The script stages
only captured paths, preserves unrelated staged work, refuses newer uncaptured
edits, records completed commit IDs, and restores the finished files on failure.

World sound/particle callbacks are now ordered events; notes and records are supported.
See [the effects coverage report](../../../../../EFFECTS.md) for original-bytecode checks
and remaining presentation boundaries.
