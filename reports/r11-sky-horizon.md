# Original 1.1 world horizon sky

`tools/verify_r11_sky_horizon.ps1`: 128 original full sky frames, 2,815,472 raw-bit comparisons, zero mismatches. Original WorldInfo uses default and flat world types; the unchanged original horizon getter supplies 63 and zero respectively. The corpus varies dimensions, height near both horizons, phase, weather, stars, sunrise and anaglyph.

Production GPU rendering and captured sky commands share the height-minus-world-horizon calculation. The app supplies the actual player's World.horizon value. The complete RenderGlobal runtime delta changes that horizon and adds pass-zero potion color; the latter is unchanged color behavior covered by potion/item pass vectors. Other raw hunks rename height, provider fields, particles, comparators and overrides. This is geometry/state evidence, not final GPU pixel equivalence.
