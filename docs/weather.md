# Beta 1.7.3 client weather

`weather.rs` ports the original `EntityRenderer.renderRainSnow`,
`EntityRenderer.addRainParticles`, `EntityRainFX`, `EntitySmokeFX`, and
`RenderLightningBolt` algorithms. `sky.rs` supplies the weather color formulas.
Settings remain Fancy, Far, overworld, without anaglyph, on the existing non-sRGB surface.

## Worker boundary

Each `FromWorker::Time` carries current and previous rain/thunder strengths, a
flash value, nearby terrain/brightness columns, and new `Lightning` render states.
Thunder interpolation is multiplied by interpolated rain, as in vanilla.
The render thread never accesses the live `World`.

Snapshots distinguish the square biome query for precipitation from singleton
biome queries for splashes. Brightness comes from `World.getBlockLightValue`,
including its y=128 clamping behavior, rather than `ChunkCache`'s above-world shortcut.
Snapshots continue for 40 ticks after rain ends so existing particles retain terrain.
Terrain meshes already rebuild when `skylightSubtracted` changes; there is no modern
lightmap texture in this version. Rain changes sky subtraction, not the provider's
constant brightness table.

The world simulation currently records strike positions rather than lightning
entities. Its adapter supplies independent visual seeds and displays each strike
for three ticks. `Lightning { pos, bolt_vertex }` is the integration boundary for
the entity branch's real seeds, visibility, repeated flashes and lifetime. The
adapter never consumes `World.rand`.

There are two distinct original world fields: `field_27168_F` is the weather
countdown already represented by `World.lightning_flash`; sky color reads
`field_27172_i`, which lightning entities set. The sky oracle sets the latter.
The position-list adapter uses the existing countdown for its temporary sky flash.

## Numerical verification

Both `bash tools/verify.sh` and `bash tools/verify.sh quick` ended with
`ALL CASES EXACT`. The expanded cases compare:

- Sky, provider fog, renderer fog, clouds, skylight subtraction, star brightness,
  rendered star brightness, sun/moon alpha, and fog distances as float bits.
  Cases include partial rain/thunder interpolation, lightning flashes and fog fade.
  `setupFog` retains start=64 and end=256 during rain; sky fog uses the existing
  start=0, end=256*0.8 settings.
- Snow and rain vertex positions, UVs, per-column float colors and alpha, plus
  expanded lightning strips. Seven scenarios include rain, snow, dry biomes,
  negative coordinates, tick overflow, weak rain, lava and an overhead roof.
- Splash/smoke spawn types and double positions, the renderer RNG's nextLong,
  sound counter, and rain sound position/volume/pitch, including the muffled case.
- 260 Gaussian double results per scenario (1,820 total), including setSeed
  clearing a cached sample. The local log helper preserves Random.nextGaussian's
  fdlibm/StrictMath arithmetic.

The headless oracle loads Mojang's original renderer and Tessellator bytecode.
Small GL11/GLContext shims replace only graphics calls and capability queries;
`glDrawArrays` records Tessellator data before the original draw method resets it.
No precipitation or lightning formulas are recreated on the Java oracle side.
The weather dump contains terrain inputs so geometry can be tested independently
of world creation/population order. Existing terrain, weather simulation, mesh,
save, server and mob-model oracle checks continue passing.

## Visual checks and commands

The viewer was run with `--shot`; rain, snow and lightning captures were inspected:
`shots/weather-rain.png`, `shots/weather-snow.png`, `shots/weather-lightning.png`.
Regression logs are in `shots/verify-weather.log` and `shots/verify-weather-quick.log`.

```
beta --skycolor seed x z time partial [rain thunder flash [prev-rain prev-thunder [fog-prev fog-current]]]
tools/oracle/oracle.sh skycolor seed x z time partial [rain thunder flash [prev-rain prev-thunder [fog-prev fog-current]]]

tools/oracle/oracle.sh weathermesh seed x y z renderer-tick partial rain output.bin [natural|lava|roof] [sound-counter]
beta --weathercheck seed x y z renderer-tick partial rain tools/oracle/out/output.bin

beta --no-save --seed 11 --weather 1,0 --time 6000 --at 0.5,80,0.5,0,10 --shot shots/weather-snow.png
beta --no-save --seed 3257840388504953787 --weather 1,1 --time 6000 --at=-39,70,65,0,10 --bolt=-39,64,75,42 --shot shots/weather-lightning.png
```

`--weather` initializes simulation weather, which then ticks normally. `--bolt`
is a persistent render preview with an explicit seed; it does not create a world entity.

## Verification limits and integration left

The numerical claims above apply to the exercised oracle inputs. They do not
claim identical GPU rasterization across OpenGL and wgpu drivers. Screenshots
check integration, not mathematical equality.

Particle constructors retain separate Math.random and entity-Random streams,
which vanilla initializes nondeterministically; individual full particle
trajectories are not oracle-diffed. Particle updates and billboards are ported,
including the vanilla bug where rain's setSize(.01,.01) leaves its .2-sized
bounding box intact. Collision uses snapshot blocks and metadata; moving piston
tile collisions and collisions with future simulated entities remain for the
shared entity/collision integration. No entity-rendering or worldgen files changed.

Rain sound selection and RNG are verified and returned by `add_rain_particles`.
The viewer has no audio backend, so it does not play those sound events.
Real bolt seeds/lifecycle and entity-driven `field_27172_i` flashes await the
entity branch, as intended by the task.
