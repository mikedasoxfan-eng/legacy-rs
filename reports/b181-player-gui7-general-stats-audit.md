# GuiSlotStatsGeneral b1.8.1 delta audit

Both decompilations were compared. The class delta consists of GuiStats accessor renames; its list policy and draw formulas are unchanged. The shared general-stat table already comes from the original b181 registry.

| Method | Port and exact evidence |
| --- | --- |
| Constructor | Existing GuiStats general slot uses width/height, top 32, bottom height-64, row height 10 and disabled selection. Retained general-stat pixel scenes prove the actual screen wiring. |
| getSize / getContentHeight | General::size/content_height use the original ordered registry and size*10. generalstats181 compares 1,024 actual original list cases. |
| elementClicked / isSelected | Literal no-op/false callbacks. The same fixture invokes the original methods with arbitrary index/double-click inputs. |
| drawBackground | Existing Ui world-background API. Retained isolated general-stat screens are pixel EXACT. |
| drawSlot | Production and checker share general_cells. generalstats181 intercepts the actual original GuiSlotStatsGeneral.drawSlot virtual drawString calls and compares every name, formatted value, alternating color, measured alignment and x/y argument for every general stat across 1,024 signed-value scenarios. |
| Decimal formatter used by distance/time rows | The new fixture found a double-rounding bug in the inherited formatter. The port now formats the original binary double directly instead of multiplying it by 100 first. 30,000 actual original distance formatter calls around 10,000 random decimal-half boundaries are EXACT, in addition to all 1,024 screen scenarios and the retained 15,072 formatter cases. |

The final generalstats181 artifact is 1,740,218 bytes EXACT. GuiSlotStats, GuiSlotStatsBlock and GuiSlotStatsItem retain their separate NOT DONE statuses; this audit does not credit their sorting/rendering protocols.
