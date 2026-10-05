# Minecraft Beta 1.8.1 — round 1

| Class | Status / proof |
| --- | --- |
| Minecraft | NOT DONE. Original runTick input/controller replay: 2,080 ticks EXACT; original preloadWorld callback loop: 128 cases EXACT; original debugInfoEntities: 10,000 cases EXACT. Retained controller input, global binding registry, controller factory/release, create/select-world callbacks and seed/settings checks remain EXACT. Full startWorld/changeWorld/respawn/usePortal application callback transactions, general renderer debug metrics and startup splash/GL sequence still need the checks listed in b181-minecraft1-audit.md. |

Implemented a shared Minecraft input driver for the actual window/worker path and the replay. Mouse events remain ahead of keyboard events; wheel events retain their individual signs; physical chord state is supplied after queue polling; inventory/drop/chat actions consume one press at a time; item-use release retains queued clicks for the following tick; held use repeats after six ticks; focus and screen-opening callbacks update logical focus and release bindings immediately. Block-triggered container opening now releases queued bindings inside the click loop. Input settings persist on the worker until the UI explicitly updates options, preventing delayed window snapshots from undoing toggles.

The original-bytecode replay records each tick's input events, screen/focus, drops, actual controller click invocations and swings, all inventory stacks and binding states/counts, setting toggles, both Minecraft counters, item-use state/count, mining controller state, world difficulty/flash and world-tail callback counts. It calls original Minecraft.runTick; the platform, renderer, screen and world-tick boundary recorders do not replace that method. Input stubs have a separate classpath selected only for the new input replay, preserving old oracle modes and guishot's real LWJGL.

Also factored the actual preload coordinate/progress/lighting loop without changing its vanilla 128-block policy. Both survival and creative inherit the non-reduced policy; the 64-block branch belongs to a controller extension, not creative mode. Added “Switching level” before world opening and WorldSettings(0,0,true) after conversion. The debug particle/entity line now uses the actual loaded entity count rather than a constant zero. Focus loss exits fullscreen and the input path no longer includes the old F8 toggle.

## Verification

`tools/verify_minecraft181.sh --built` is appended before the final pflush in verify181.sh. It has one section per check and no set -e:

- Original runTick event queues/clicks/screens/focus/world policy: 2,080 ticks EXACT.
- Original preloadWorld coordinate/progress/lighting callbacks: 128 cases EXACT.
- Retained clickMouse/sendClickBlockToController effects: 2,800 transitions, 2,681,544 bytes EXACT.
- Original debugInfoEntities: 10,000 cases EXACT.
- Retained global key registry: 20,000 events, multiple settings, duplicate/stale entries and overflow EXACT.
- Retained controller lifecycle: 1,000 factory/flip cases and 20,000 partial/reset boundaries, 1,232,729 bytes EXACT.
- Retained create-world callbacks: 336 production screen replays, 1,344 snapshots and launch/display traces EXACT.
- Retained seed/settings policy: 65,536 Java digits and 10,096 records EXACT; unseeded clock-random seeds are classified by original policy, not claimed identical timestamps.
- Retained saved-world controller selection: 864,178 bytes EXACT.

`cargo build --release -p worldgen181 -p client` succeeds. `git diff --check` passes. The full ~hour verify181 suite was not run. Existing b173 crates/oracles/verifiers and all shared oracle fixtures/dispatch remain untouched.

The live play script is `target/minecraft181-play.txt`; captures and logs remain under target. It exercises hotbar/wheels, movement, F3+F, inventory open/close, pause/resume and F5. The final script exited successfully, moved before inventory and after resume (z 115.5 -> 116.58 -> 117.46), and retained the F5 third-person toggle in the resumed capture. These are smoke/play-test observations, not original-client pixel comparisons. Startup methods startGame/loadScreen/run/fullscreen have no simulation effect and use this play-test category; literal startup splash and GL failure paths remain unproven.

## Remaining work

See the complete per-method/branch table in `reports/b181-minecraft1-audit.md`. The main open work is complete Minecraft lifecycle callback replay (including nullable guards, failures, saved/new statistics, render/effect/container handoff and post-world state), exact cross-thread tick/renderer scheduling, chunk-provider focus at unusual coordinates, real-screen sequences beyond the recording input boundary, the resource reload transaction, native fullscreen/focus-loss branches, actual renderer debug metrics, and original startup splash/quad capture. Existing portal/respawn/conversion checks compare constituent original methods; they are not mislabeled as direct Minecraft method coverage.

Git metadata was not written. `target/commit-b181-player-gui8.sh` stages the explicit task paths and commits using the requested logiu identity and both trailers.
