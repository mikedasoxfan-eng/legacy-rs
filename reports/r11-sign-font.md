# Shared 1.1 sign font and normal packing

World signs use the current GUI font configuration, including Unicode and bidirectional flags, and share its random generator and glyph cache. Unicode pages upload before drawing. Screenshot font reconstruction retains the language flags.

Direct original TileEntitySignRenderer followed by the same original FontRenderer is compared against production Rust rendering. The 672 sequences compare 1,373,120 bits of matrix, color, material, packed normal, UV, vertex and random-tail data exactly. Cases include standing/wall signs, metadata and edited lines, four Unicode/bidirectional combinations, and English, Hebrew, Arabic, Japanese, obfuscation and supplementary characters. Texture allocation IDs are normalized to fixed material/page identities; page numbers and all drawing data are retained.

The comparison exposed the 1.1 Tessellator change: each signed normal byte must be masked before packing. Direct original setNormal comparisons match 11,331 vectors, including special IEEE 754 values and random float bit patterns. The shared Rust packing implementation and Fixed implementation are both checked.

Reproduce with tools/verify_r11_signfont.ps1. A headless GPU check also uploads six Unicode pages and resolves all 27 draw bindings. This checks upload and binding, not rendered pixels. The existing text/language regression verifier passes after the shared-font change. FontRenderer's entire 1.1 delta remains uncredited until its remaining integration checks pass.
