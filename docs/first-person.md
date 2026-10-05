# Beta 1.7.3 player view

The camera, first-person hand/items, overlays, player model/armor, block outline,
and destruction textures are literal ports of `EntityRenderer`, `ItemRenderer`,
`RenderPlayer`, and `RenderGlobal`. The existing GUI and world simulation are
unchanged.

`player_view::Snapshot` travels with `FromWorker::Player`. It copies the nearby
chunks, entities, light, block bounds, dimension, and saved map data into a private
render world. Partial-tick picking calls the simulation's `Entity::mouse_over`
against that mirror, including entity occlusion. Camera wall probes and opaque
overlays likewise cannot mutate the simulation's bounds or random streams.

The camera preserves water/death FOV, hurt/death rotations, bobbing, the -0.1
first-person translation, all eight third-person obstruction probes, sleeping
bed rotations, and portal wobble. Fog uses the original medium colors, exponential
water/lava density, linear distance/Nether branches, fade, and anaglyph color math.
GPU fog distance includes the modelview camera transform.

The hand uses equip/swing interpolation, player brightness, rotated item lighting,
3D inventory blocks, extruded flat sprites, and the char.png arm. Map drawing
includes both arms, background, dynamic palette texture, icons, and FontRenderer
caption; the original second anaglyph conversion during texture upload is kept.
Cached map images survive the lowering animation when changing held items.
Fire, water, and opaque-block overlays retain the original UV and double-draw
quirks. Hiding the GUI hides the hand but still draws overlays.

The persistent player renderer uses char.png, held-item and block-helmet
attachments, all five armor materials, inflated armor models, sleeping offsets,
and hurt/death layers. The previous armor-pass model's animation flags retain
RenderLiving's stateful behavior. Singleplayer never draws its own name tag.

Selection bounds and raw width-2 GL line vertices are preserved. Wgpu expands
clipped edges into screen-space triangles. Crack drawing retains the original
Tessellator's disabled vertex color, destination/source-color blend, and polygon
offset. All vanilla RenderBlocks destruction render types are implemented,
including doors, rails, stairs, fences, levers, repeaters, beds, and pistons.
Cracks are drawn by the same RenderBlocks port as the world, with
`overrideBlockTexture` set, exactly as the original does; the bytecode oracle is the
authority for their correctness.

## Verification

```bash
cargo build --release -p client
tools/oracle/oracle.sh firstperson firstperson.bin
target/release/beta --firstpersoncheck tools/oracle/out/firstperson.bin
tools/verify.sh quick
tools/verify.sh
```

The first-person suite executes Mojang's original bytecode through the headless
GL shim and compares **18,396 cases / 8,116,436 raw-bit values / zero differences**.
It checks signed zero, ordered matrix operations, texture/material/normal data,
blend/depth/lighting flags, GL primitive modes, original Tessellator vertex and UV
bits, FOV/projection, fog color/density/distances, equip transitions, partial-tick
picking, player armor state, and map ARGB/RGBA uploads. Cases include all vanilla
item IDs, valid block metadata and ten destroy stages, obstruction fixtures,
sleeping/dead/hurt/bobbing/portal states, both anaglyph eye transforms, and negative
or distant coordinates.

Projection values come from LWJGL's original `GLU.gluPerspective`. Modelview
matrices are evaluated from the original recorded GL arguments using a portable
float matrix stack, and compared to the Rust matrix evaluator. This proves the
renderer input/geometry boundary, rather than an unspecified driver's internal
OpenGL matrix arithmetic. Wgpu clipping, line expansion, lighting evaluation,
rasterization, blending, and framebuffer rounding are **not** claimed pixel exact.

`verify.sh` includes this suite. Both repeated quick runs and the unrestricted
full run ended **ALL CASES EXACT**. Release builds, `cargo check`, and scripted GPU
captures passed. Logs are `target/verify-firstperson-full.log`,
`target/verify-firstperson-final.log`, and `target/firstperson-compare.log`.

Reproduce visual review with disposable saves:

```bash
python tools/firstperson_scene.py target/firstperson-scenes
target/release/beta --world-dir target/firstperson-scenes/equipment/world --play-script target/firstperson-scenes/equipment/play.txt
target/release/beta --world-dir target/firstperson-scenes/water/world --play-script target/firstperson-scenes/water/play.txt
target/release/beta --world-dir target/firstperson-scenes/lava/world --play-script target/firstperson-scenes/lava/play.txt
target/release/beta --world-dir target/firstperson-scenes/fire/world --play-script target/firstperson-scenes/fire/play.txt
target/release/beta --world-dir target/firstperson-scenes/bed/world --play-script target/firstperson-scenes/bed/play.txt
```

Screenshots appear in each `play/` folder. The fixtures cover blocks, sword, map,
empty arm, mining cracks, third-person diamond armor, medium/fire overlays, and
sleeping. They only write inside the selected target folder.

## Remaining boundaries

- Inventory snapshots contain value-type ItemStacks, without Java object
  identity. The exact equip state machine is oracle-checked with explicit
  identity; live snapshots infer identity from slot and item ID. Replacing a
  stack with a different-durability stack of the same ID in the same slot can
  therefore miss the lowering animation, while ordinary durability/count
  mutation remains smooth. Resolving this needs inventory identity provenance.
- Map rendering and valid saved map colors are supported. Existing simulation
  has no map exploration/update state or map counter; this port adds none.
  Unsaved maps are blank, and Nether map icon counter rotation is not simulated.
- The existing GPU still lacks full two-eye anaglyph compositing and general
  texture recoloring. Both camera eye transforms, fog conversion, and map upload
  conversion are covered by the oracle.
- Downloaded skins, cloaks, deadmau5 ears, and fishing-line simulation are outside
  the requested char.png singleplayer renderer. Fishing entities are absent from
  the existing simulation, so the fishing-hook held-item substitutions have no
  live state to consume.

Git metadata is read-only in the task sandbox. `target/commit-firstperson.sh`
commits the renderer stage and the oracle/documentation stage separately, using
the requested author and co-author trailers. It names only this task's files and
preserves unrelated staged changes.
