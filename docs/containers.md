# Container screen integration

The simulation owns inventories, crafting matrices and tile entities. The UI receives
`worldgen::container::ContainerSnapshot` and sends `ContainerCommand`; it never edits
an authoritative inventory. All six layouts use `screens::container::GuiContainer`
(also exported as `GuiInventory`, `GuiCrafting`, `GuiChest`, `GuiFurnace`, `GuiDispenser`).

1. For the inventory key, send `gui::container_api::inventory_key_request()` through
   the existing world-thread channel. For `PlayerEvent::OpenContainer`, use
   `activation_request(&event)`. On the world thread call
   `world.container_command(player_entity_id, command)` and return its snapshot.
2. An answer to Open calls `GuiSystem::open_container(snapshot)` on the UI thread;
   process the returned `GameAction`s exactly like other screen transitions.
   The world handler closes the preceding container, including its cursor/matrix
   drops, before opening the replacement. `open_container` suppresses the old
   screen's redundant close callback, including when reopening window zero.
3. After UI input and screen transitions, drain
   `GuiSystem::take_container_commands()` onto the world channel, in order.
   Each click includes window, slot, button and shift; close includes window.
   The world handler ignores stale windows, reads current tile inventories,
   applies the vanilla click and writes the updated tile/player inventories.
4. Return click answers through `GuiSystem::update_container(snapshot)`. While a
   container is open, also call `world.container_snapshot(player_entity_id)` once
   per world tick and send the result to `update_container`. This updates furnace
   progress and externally changed chest inventories. The UI ignores old windows.
5. Distance/block validity is checked during player ticks. Forward
   `PlayerEvent::Container(ContainerEvent::Closed { window })` to
   `GuiSystem::container_closed(window)`. A dead-player snapshot sets `alive=false`.
   The next screen update closes the GUI and produces the usual Close command.

Forward non-drop `PlayerEvent::Container` events to the app's existing stat and
achievement systems. `Crafted { item, count }` includes furnace output statistics;
`Achievement` uses the original achievement field name. Opening inventory emits
`openInventory`. Drops are already spawned by the world handler and must not be
spawned again. `MapCreated { id }` reports the assigned map id; map-data generation
and persistence require the app's map subsystem. Drain `world.container_effects`
for the original dispenser auxiliary effects (1000/1001/1002 and smoke 2000).

The inventory preview renders the shared biped, mouse-following pose, armor,
pumpkin helmet and current held item, plus live walking, riding, sneaking, swinging
and damage animation. Snapshots include `armor`, `held` and `preview` so no
world entity or mutable renderer state crosses the thread boundary. Default skin
is `mob/char.png`; downloadable skins/capes require an additional app resource.

The app, worker and renderer files intentionally have no integration edits. Add
the messages to their existing channels using this contract. Container screens
do not pause world ticking, as in the original client.

## Reproduction

`bash tools/verify_containers.sh` verifies all 151 recipe outputs, mirrored and
offset matrices, scripted/random clicks in every layout, and furnace/dispenser
world ticks against original bytecode. `bash tools/verify_gui.sh` additionally
captures the new container scenes from the original client and Rust. Container
pixel checks use a strict comparison: a one-level channel difference fails.
