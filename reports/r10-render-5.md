# Minecraft 1.0 render round 5: GUI pixel captures

`tools/verify_gui10.sh`: ALL GUI CASES MATCH.

| shot | result | cause + fix |
|---|---|---|
| inventory player while hurt | EXACT | RenderLiving's hurt overlay uses the player's world brightness; GuiShot now carries the in-world player's `getEntityBrightness` into later container captures. |
| inventory_bow | EXACT | RenderPlayer's own held-bow transform (translate 0,2/16,5/16; rotate -20 Y; scale 10/16; -100 X; 45 Y) ahead of the full-3D branch. |
| b10_hud5_xp_overflow | EXACT | The scene set currentXP (always below 1 in the game) to 2147483647, a quad billions of pixels wide that only the graphics drivers disagreed about; it now tests the reachable overflow, the largest level. |
| achievements*, stats_blocks*, b10_creative_*, b10_lightmap1_* | within one level | The known GPU lighting rounding on lit 3D block icons (1-155 pixels, one level), reported apart by `--guidiff` exactly as in Beta 1.7.3 and 1.8.1; the 1.0 suite's main screens use that comparison again. |
