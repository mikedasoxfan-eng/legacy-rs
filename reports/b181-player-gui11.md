# b1.8.1 player-gui, round 11

Both classes assigned to this round are **Done**. This completes their remaining singleplayer b181 delta and boundary proof; no other class is newly counted. The full source/boundary audit is in [b181-player-gui11-audit.md](b181-player-gui11-audit.md).

| Class | Status and EXACT evidence |
| --- | --- |
| StatFileWriter | Done: statfile11_181 / --statfile11check matches 3,859 malformed/UTF-16 inputs, 10 constructor failure/migration disk snapshots, eight JVM-default-charset checksum boundaries, 128 UTF-16/encoded serializers and all 256 byte decodes. Retained statfile10_181 matches 3,050 schema/grammar/checksum/map-order cases and 1,000 byte-identical serializers for supplied iteration orders. statsync181 retains 2,124 exact snapshots covering state/achievement/file/thread behavior. JVM StatBase identity-hash iteration is the explicitly excluded irreproducible input, as instructed; production uses a deterministic sorted-ID iteration input. |
| RenderEngine | Done: renderengine11_181 / --renderengine11check matches 672 cache/flag/failure transactions, 180 mutable dynamic images, 64 colliding/reused-key deletion refreshes, 20 contents-array refresh boundaries, 96 mixed cache refreshes, 32 colliding String-map keys and 12 actual original image-buffer boundaries. Retained renderengine10_181 matches 2,000 single-prefix/filter/reset/cache cases. Full texture conversion, animations, framebuffer/color-mask checks and strict feature/title RGB/alpha captures remain EXACT. |

## Changes

StatFileWriter construction now preserves failed directory enumeration instead of continuing with an empty migration list. File reads/writes and MD5 use the original Windows Cp1252 default, including replacement behavior and the String.length-versus-byte-count exception. The parser and serializer expose exact UTF-16 checked boundaries, preserve the partial-hex exception, and distinguish Argo's nonreturn for an unclosed string from normal syntax rejection. Production retains nonreturn by parking the file reader; the fixture observes it without exhausting the JVM heap.

The new production RenderEngine state owns independent named and contents caches, mutable flags, shared dynamic-image references, original refresh order, deletion and the 16 MiB image-buffer limit. GuiRenderer/GuiSystem use the same state machine as the original-bytecode comparator. Contents refresh retains array identity and stale tails and preserves partial writes on overflow. Failed operations retain the vanilla flags and exception/catch behavior. JavaHashSet gains only a remove API, with original tree deletion/rebalancing; existing world add/iterate behavior is unchanged.

The retained GUI check found two integration issues, both fixed in the existing renderer adapter: frame-local native panorama transfers do not enter the persistent dynamic-image map, and snapshot framebuffer copies apply the original blur callback's linear sampler after refresh. GuiMainMenu and GuiInventory were not reopened or edited.

## Verification

The final player-gui block appended to tools/verify181.sh invokes [tools/verify_player_gui11_181.sh](../tools/verify_player_gui11_181.sh). Its four sections are EXACT:

- statfile10_181 / --statfile10check
- statfile11_181 / --statfile11check
- renderengine10_181 / --renderengine10check
- renderengine11_181 / --renderengine11check

Release build of worldgen181 and client passes. tools/verify_player_gui10_181.sh --built passes the retained original callbacks and all 23 strict RGB/alpha captures (eight animated titles and 15 feature scenes, 9,428,160 alpha bytes). tools/verify_statsync181.sh passes all 2,124 original snapshots. tools/verify_gui181.sh --built passes: its main sequence retains 195/203 strict captures and the same eight inherited achievement/stat scenes within one color level; all 35 containers, nine signs and 15 feature scenes are strict EXACT. No comparator or tolerance was changed. The independent retained texture/framebuffer checks also pass (16,777,216 RGB conversions, 1,000 compass/watch frames, 400 animation frames and 80 original RGBA/color-mask observations). The panorama resize smoke check passes; it does not claim original undefined small-window texels.

Shell syntax checks and git diff --check pass. Evidence is retained in target/player-gui11-build-final.log, target/player-gui11-verify-final.log, target/player-gui11-retained10-final.log, target/player-gui11-retained-statsync.log, target/player-gui11-verify-gui.log, target/player-gui11-texture.log and target/player-gui11-framebuffer.log. target/player-gui11-evidence.json summarizes the new trace counts. The earlier feature regression failure remains in target/player-gui11-retained10.log for diagnosis; the final rerun passes.

## Input limits and handoff

The only excluded stat serialization input is the JVM identity-hash-dependent iteration order, which varies between original processes. The supplied-order serializer remains byte exact, including checksum; files are not compared after sorting to claim serializer equality. The canonical charset is the repository's Windows Mojang Java 8u51 runtime. The liveness probe reports nonreturn separately from vanilla exceptions. These boundaries and the resource/upload adapter observations are documented in the audit.

There is no remaining assigned work in either class. Other classes and their existing ownership/status remain unchanged. The approximately one-hour full tools/verify181.sh integration run remains with the integrator, as requested. No b173 crate/oracle/verifier, Oracle.java, world fixture/listener, Minecraft-loop, entity renderer/model/particle, terrain, world/save or world-renderer/lightmap/fog/FOV algorithm was edited.

Git metadata was not written. [target/commit-b181-player-gui11.sh](../target/commit-b181-player-gui11.sh) stages explicit task files and commits as logiu with the requested email and both co-author trailers.
