# Original 1.1 silverfish renderer

`tools/verify_r11_silverfish_workflow.ps1` compares 64 successive original render calls using one renderer. Result: 378,469 raw-bit comparisons, zero mismatches, empty original renderer stdout.

The complete delta removes the unused model-version getter and first-render model replacement/diagnostic. Model geometry and animation are unchanged. The Rust renderer retains its model across calls and contains no replacement diagnostic; its actual draw path matches the original with varied interpolation, pose, hurt/death and light states. Remaining Java hunks rename overrides and bridge calls. This proves the changed model lifecycle and removed diagnostic in the declared singleplayer draw domain; final GPU pixels are outside the capture.
