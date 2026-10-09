# Minecraft 1.1 text foundation

This is focused evidence, not complete release certification.

## Changes

The production font now wraps UTF-16 units, preserves Java trimming/split rules, carries the last formatting unit, and applies Bidi once to the full text before wrapping. Private line draws use the already ordered text. Newline recursion clears shadow as in the original. Partial drawing and RNG state survive an original formatting-index exception. Integer shadow offsets and line positions wrap as Java integers do.

The preceding checkpoint includes Unicode6.1 class data, the Java8-compatible Bidi engine, language startup flags, and the language options screen/live-selection wiring. This commit adds a single reproducible headless validation entry point for those pieces.

## Reproduce

With the repository toolchain configured, the official1.1 deobfuscated reference installed, and ORACLE_JAVA pointing to Mojang Java8u51:

```powershell
./tools/verify_r11_text.ps1 -McpPath tools/retromcp-r11
```

The helper compiles isolated recorders against unchanged original1.1 game bytecode. Its LWJGL copies change only library manifest sealing so the GL recorder can share the package; original jars remain untouched. Those copies and provenance live under ignored target/r11/font-libs.

## Results

| Check | Exact coverage |
| --- | --- |
| Character classes | All1114112Unicode code points against the pinned JVM |
| Bidi | 85569original corpus +148416independent boundary cases; requires predicate, base, every unit level, logical runs, final UTF-16 output |
| Public drawing | 1792Unicode/Bidi/shadow/formatting cases; geometry/UV/color/texture page/end position/private RNG |
| Wrapped drawing | 10752public split-method cases; six widths, Unicode/Bidi/shadow combinations, UTF-16 surrogate boundaries, format exceptions/partial drawing, height and RNG |
| Language callbacks | Three resolutions,168live selections; original options/language button geometry, complete language list/order, selected state, code, Unicode/Bidi flags and Done label |

The full reproduction finished with R11 TEXT AND LANGUAGE FOCUSED EXACT. Runtime tests also replay successful valid strings through the public Rust drawing wrappers.

Reference jar SHA256:44BB6ACE88FDFF460D6C428C4BECEE8CAD52F2622E1F800DAB7A70F8A716F75B.

| Fixture | SHA256 |
| --- | --- |
| font-bidi-draw11.bin | 8D02DBC592193DDC98C6ED8EEE9E682D2047D37D3D30C1F48B40FCC1BE16055A |
| font-wrap11.bin | AC1F50FC6E0F123BA51A242E081DF82C60ED439D812452812B1493CB68178A49 |
| language-screen11.bin | 0B163DAF959B0A03B7AC6950FF2B6B3AD51E2D367D791CDCAFE8FC1092DE6395 |

## Remaining gates

GPU/pixel comparisons, resource refresh, sign-font flag routing, actual language screen input/drawing and saved-option restart remain. Java Formatter translation errors need a separate audit. Widths unable to fit a glyph can make the original wrapping loop stall; Rust reports NonProgress rather than hanging, and these inputs are not included in the valid-width parity claim. No whole-class progress credit is asserted here.
