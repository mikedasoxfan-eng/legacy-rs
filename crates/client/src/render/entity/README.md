The models are literal ports of the Beta 1.7.3 client, with Java part names retained
in `Model.parts` and Java class/method names in doc comments. `RenderState` contains
render inputs only. Positions stay doubles until camera subtraction. The simulation
can fill `gpu.entities.states` before `Gpu.render`; the entity pipeline draws after
opaque terrain and before water.

Geometry and poses are checked against original bytecode, without OpenGL:

```bash
tools/oracle/oracle.sh models models.bin
cargo run --release -p client -- --modelcheck tools/oracle/out/models.bin
tools/verify.sh
```

The oracle covers 21 constructor configurations and eight successive poses per
instance, including inflated/mirrored boxes, all corners/quads/UVs, attack/riding/
sneaking/held-arm flags, standing/sitting/angry/begging/shaking wolf animations,
face normals, and the original Tessellator's packed BYTE normals. Comparison is
by raw float/double bits, including signed zero. It reports 236,840 comparisons.

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
1/16-thick, 16-strip extrusion. Inventory-rendered held blocks, player accessories,
entity shadows/fire/name labels, AI, physics and spawning belong to their respective
render/game systems; this module supplies postRender/child attachment transforms.
