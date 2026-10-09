# Live sheep grazing in 1.1

The live entity bridge now reads EntitySheep's head-position and head-angle getters. ModelSheep1 and ModelSheep2 move the head down by position times nine and retain the prepared pitch for their rotation callback. Both body and wool models receive the animation before drawing; hurt overlays reuse those poses.

tools/verify_r11_sheep_render.ps1 runs unchanged original getters, both original model callbacks and the full original RenderSheep renderer. The production Rust bridge, reused model instances, frame renderer, lightmap routing, matrix operations, raw vertices, UVs, material and packed normals match exactly: 480 cases, 2,974,920 comparisons, zero differences. The cases cover timer boundaries -1 through 41, four partial ticks, adult/child, sheared/unsheared, all fleece colors and hurt overlays. Current pitch differs from interpolated pitch to verify the original EntitySheep getter is used.

This verifies the actual frame rendering path. It does not claim a full game or pixel regression suite. The separate original gameplay probes cover status 10, movement and jump suppression, and grazing timer ordering.

The brewing background is separately corrected to divide progress by 400 ticks, matching the 1.1 simulation. tools/verify_r11_brewing_gui.ps1 compares the original background callback with production rectangles for every timer from -10 through 1200 and randomized screen sizes: all 67,868 bytes match.
