The models are literal ports of the Beta 1.7.3 client, with Java part names retained
in `Model.parts` and Java class/method names in doc comments. `RenderState` contains
render inputs only. Positions stay doubles until camera subtraction. The world thread supplies
read-only `effects::Snapshot` values, including nearby
light and shadow surfaces. The bridge interpolates each snapshot at the frame
partial tick; the entity pipeline draws after opaque terrain and before water.
Lightning uses `World.weather_effects` and each bolt's own changing seed, and draws
before ordinary entities. Sky flash reads the original bolt field (`field_27172_i`);
its failure to decrement in vanilla is retained.

Geometry and poses are checked against original bytecode, without OpenGL:

```bash
tools/oracle/oracle.sh models models.bin
cargo run --release -p client -- --modelcheck tools/oracle/out/models.bin
tools/verify.sh
```

The oracle covers 22 constructor configurations and eight successive poses per
instance, including inflated/mirrored boxes, all corners/quads/UVs, attack/riding/
sneaking/held-arm flags, standing/sitting/angry/begging/shaking wolf animations,
face normals, and the original Tessellator's packed BYTE normals. Comparison is
by raw float/double bits, including signed zero. It reports 252,120 comparisons.

The offscreen showcase needs no world or window. Left to right: biped, zombie,
skeleton (with bow), pig, cow, sheep, chicken, spider, creeper, slime, squid, wolf.
`--variants` exercises saddle, coloured fleece, wing flap, hurt/death overlays,
dark spider eyes, powered/flashing creeper, squish, and a sitting/tamed/begging wolf.

```bash
cargo run --release -p client -- --modelshot shots/models.png
cargo run --release -p client -- --modelshot shots/models-variants.png --variants
```

An optional visual oracle renders the same scene through Mojang's actual
RenderLiving subclasses and display lists into a LWJGL pbuffer. It uses the
player's jar textures, the same camera and floor, and stubs world/AI inputs only:

```bash
cargo run --release -p client -- --modelshot shots/models.png --dump-scene shots/scene.txt
tools/oracle/modelshot.sh shots/scene.txt shots/models-java.png
```

Keep Beta's quirks: UVs have a 0.1 texel inset; wolf head/ears/snout/tail rotate
Y-X-Z while ordinary parts rotate Z-Y-X; RenderSlime leaves GL_NORMALIZE enabled;
Tessellator multiplies normal X by 128 and packs signed bytes with sign extension.
Lighting clamps after multiplying the material brightness/tint. Spider eye alpha
is `(1-brightnessAtPartialOne)*0.5`, with lighting still enabled. Hurt/flash layers
use untextured, alpha-blended GL_EQUAL depth tests. The wgpu/OpenGL screenshots
are visual checks; rasterization, framebuffer rounding and GL_EQUAL precision
are not claimed to match pixel for pixel. The model oracle is the bit-exact check.

Equipped flat/full-3D item sprites use the original attachment transforms and
1/16-thick, 16-strip extrusion. Pig zombies use the inherited ModelZombie and gold sword attachment. Player
accessories and name labels remain separate from the entity renderers; AI, physics
and spawning stay in worldgen, which this renderer change does not modify.


Dropped items, falling sand/gravel, primed TNT, arrows, fireball/snowball sprites,
ghasts, pig zombies, giants, Fancy shadows and entity fire are literal ports.
`inventory.rs` supplies the RenderBlocks inventory geometry used by RenderItem;
`gui_item` publicly exposes `render_item_into_gui` and
`render_item_overlay_into_gui` for future hotbar/inventory callers. Counts reuse
FontRenderer; durability uses the original three rectangles. Animated terrain
textures update the entity atlas as well as the world atlas.

The new headless oracle runs original renderer bytecode and Tessellator. Its GL
shim records ordered matrix operations, texture, material color, direct and packed
normals, lighting/rescale/normalize, blending, texturing and depth state. The Rust
frame uses the same recorded draw representation that the comparator verifies.
Comparisons use raw bits, including signed zero:

```bash
tools/oracle/oracle.sh renderers renderers.bin
cargo run --release -p client -- --renderercheck tools/oracle/out/renderers.bin
tools/oracle/oracle.sh livingrenderers living.bin
cargo run --release -p client -- --livingrenderercheck tools/oracle/out/living.bin
tools/oracle/oracle.sh rendereffects effects.bin
cargo run --release -p client -- --effectcheck tools/oracle/out/effects.bin
tools/verify.sh
```

The object/GUI suite covers every vanilla item, metadata, stack copy thresholds,
seeded offsets, sand/gravel, TNT fuses, arrow shake, both sprite scales, and tool
wear: 1,828 cases, 8,586 draws, 545,520 comparisons. The three living renderers add
48 cases and 325,080 comparisons, including ghast attack textures/scales, gold
sword extrusion, inherited normalization, varied light, hurt and death. Shadows
and fire add 48 cases and 4,120 comparisons, with mixed surface types and entity
sizes. The existing lightning/weather oracle verifies bolt geometry; bridge tests
check entity seed changes and inactive/restrike gating. All suites are part of
`tools/verify.sh` and must end with `ALL CASES EXACT`.

Preserved quirks include ModelGhast's Random(1660) tentacles, RenderItem's
Random(187), raw yaw interpolation, RenderArrow's initial rotation visibility
gate, constant slime shadow radius, and TNT inventory rendering overwriting its
flash alpha with 1.0. Exactness refers to the original renderer/Tessellator inputs;
wgpu matrix evaluation, rasterization and framebuffer rounding are visual checks,
not a claim of identical OpenGL pixels.

```bash
cargo run --release -p client -- --renderershot shots/renderers.png
cargo run --release -p client -- --no-save --seed 6 --shot-ticks 600 --at=-60,72,-38,0,20 --shot shots/entities-seed6.png
cargo test -p client
```
