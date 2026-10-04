# Video Settings parity

`GameSettings` remains owned by the GUI. The app reads a `VideoSettings` snapshot at
startup and on `GameAction::SettingsChanged`, and passes it to the world worker and
GPU. No GUI files or worldgen simulation code were changed.

## Implemented

| Setting | Beta behavior |
| --- | --- |
| Render distance | `loadRenderers` widths 26/17/9/5; original Java negative-coordinate wrapping; far-plane distance 256/128/64/32 (projection clips at twice that); terrain fog starts at one quarter of the distance. Sky draws only on Far/Normal. Clouds still draw on every distance. |
| Fog color | Distance-dependent sky blend and per-tick fade toward brightness at the camera's block position. Float operations retain Java evaluation order. |
| Fast/Fancy | Leaf opacity, leaf texture index, hidden leaf faces and grass side overlays; flat Fast clouds vs volumetric Fancy clouds; single blended water/ice pass vs Fancy depth/color passes. |
| Weather | Fast precipitation radius 5 vs Fancy 10, including vertical extent and alpha falloff; Fast rain strength is halved before particle spawn counts and sound reservoir sampling. |
| Entities | Fancy-only shadows already consume the setting. Original Beta `RenderItem` copy counts and `Entity.isInRangeToRenderDist` do **not** depend on graphics or render distance, so no new graphics/range branch is introduced. |
| Smooth lighting | The original `renderStandardBlockWithColorMultiplier` path when AO is off, including grass tinting, overlays and partial-bound brightness fallbacks. Non-standard shapes with their own lighting retain it. |
| Mesh rebuilds | Distance, graphics and AO changes clear/rebuild the renderer window. An ordered `ResetMeshes` worker message discards old GPU meshes before new uploads, including queued old meshes. |
| Framerate | Max uses uncapped presentation with the surface's supported fallback; Balanced targets 120; Power saver uses the original 40-fps budget, integer millisecond truncation, sleep guard and menu adjustment. Input remains responsive while waiting. Actual fps depends on rendering cost and OS timer scheduling. |
| GUI scale/vignette | Existing GUI behavior remains active. Vignette uses Fancy graphics. |

The CPU algorithms and geometry are bit-exact where tested below. The wgpu rasterizer
is not claimed to be pixel-identical to the original OpenGL framebuffer. Water/ice
pass selection and sky suppression are literal ports of the Java branches.

## Verification

`bash tools/verify.sh quick` completed with **ALL CASES EXACT**. The suite now includes:

- Eight terrain mesh comparisons: seeds 25 and 1, 121 chunks per seed, all four
  Fast/Fancy × AO-on/off combinations. Every position, UV and packed color matches.
- 32 actual-bytecode `RenderGlobal.loadRenderers` and
  `markRenderersForNewPosition` comparisons, including negative and distant positions.
- 48 sky/fog float-bit comparisons across all four render distances, including rain,
  thunder, lightning and fog-fade inputs.
- 14 Fast/Fancy weather cases: precipitation vertices, particle positions, renderer
  RNG, Gaussian state, sound-counter and sound outputs.
- Eight Fast/Fancy cloud geometry cases, covering above/inside/below the cloud layer,
  large and negative coordinates, wrapping, extreme tick values and storm colors.
- All existing terrain, saves, simulation, model and entity-renderer checks.

`cargo test -p client render::video::tests` checks Power saver's whole-millisecond
boundaries, the 500-ms guard and the negative-delay menu adjustment.

The live test uses actual GUI buttons, rather than assigning settings behind the GUI:

```sh
beta --no-save --seed 6 --world-rand 1 --time 6000 \
  --options-dir target/video-options/live --video-flowtest shots/video-live
```

It pauses while meshes build, resumes for each screenshot, and asserts that the GPU
window contains 676 → 25 → 289 → 81 → 676 chunks. It completed with **VIDEO FLOW EXACT**.
The generated Far/Fancy, Tiny/Fast and Far/AO-off screenshots were visually inspected.
Balanced measured approximately 117–118 fps on this Windows machine; Power saver
measured 41, consistent with truncating the original sleep to whole milliseconds.

`--options-dir DIRECTORY` reads the usual `options.txt` without changing the user's
working-directory options. For example:

```text
music:0.0
sound:0.0
renderDistance:0
fancyGraphics:true
ao:true
limitFramerate:1
```

With the directory created, a standalone capture can use:

```sh
beta --no-save --seed 6 --shot-ticks 100 --shot shots/video.png \
  --options-dir DIRECTORY
```

## Remaining limitations

- **Anaglyph is deferred**, as permitted by the task. Selecting it emits a clear log
  note. Exact support needs two eye passes, projection/modelview offsets, channel
  masks, the original float color matrices, and `RenderEngine`'s integer texture
  recoloring (including dynamic textures and entities). A final-frame color filter
  would not implement Beta's behavior.
- **Advanced OpenGL uses the no-occlusion-capability fallback.** The existing wgpu
  renderer performs frustum culling but does not port ARB occlusion query scheduling.
  This retains vanilla's visible rendering without its optional occlusion optimization.
- **View bobbing has no moving-player input in the existing fly-camera viewer.**
  A walking `EntityPlayer` camera and its walking-distance/camera-yaw/pitch state
  are not present. The player bob transform remains future work; stationary captures
  do not exercise it.
- The pre-existing mesher supports render types 0, 1, 4 and 13. This change routes
  its standard-block lighting through the exact non-AO path; it does not add the
  other pre-existing missing block render types.

The sandbox makes `.git` read-only. Finished stages are recorded as incremental patches
under `target/video-stages`; run `bash target/commit-video.sh` outside that restriction
to commit them with the requested author and co-author trailer. The script updates the
index from those snapshots without replacing the final working-tree files.
