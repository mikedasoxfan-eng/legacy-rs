# Original 1.1 held-item workflow

Run `tools/verify_r11_held_workflow.ps1` with the configured toolchain and original 1.1 oracle dependencies.

Result: 352 first- and third-person frame cases, 2,640,736 raw-bit comparisons, zero differences. The probe calls the original `EntityRenderer.renderHand` and a reused `RenderPlayer.doRender`; Rust exercises the actual first-person and player renderers. It captures draw geometry, matrices, textures and colors through the existing oracle adapters.

The corpus includes every registered spawn egg, invalid and extreme signed metadata, non-glint potion variants and snowball controls, four interpolation/equipment states, camera motion, hurt, sneak and riding states. Clock-dependent potion glint is excluded from this workflow corpus; the existing held primitive/glint checks cover that separate path. This proof covers the new generic second-pass branch and player per-pass colors, not arbitrary item stacks or final GPU pixels.
