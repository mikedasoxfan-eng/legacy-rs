# Original 1.1 biped held-item workflow

Run `tools/verify_r11_biped_workflow.ps1` with the configured original oracle dependencies.

Result: 164 full original RenderBiped cases, 540 glint draws, 1,681,276 comparisons, zero differences. A zombie subclass supplies held stacks through the original virtual getter. Every registered egg and invalid metadata, potion, glint and block controls exercise the original renderer. Captured draw data, texture matrices and packed light are compared with the production Rust biped path, replaying recorded glint clocks.

The sole RenderBiped behavior change replaces the potion predicate with generic multiple passes. The virtual getter seam proves renderer behavior; ordinary singleplayer zombies and skeletons retain their original sword and bow getters. The existing 185 primitive held-item cases cover negative and extreme metadata independently. Final GPU pixels are outside this draw-command proof.
