# EntityPlayerSP b1.8.1 delta audit

Compared both original decompilations in `target/player-gui6-sp.diff`.

| Changed method/fields | Production port and original-bytecode evidence |
| --- | --- |
| Sprint toggle/countdown fields, `func_35113_c` | `player/mechanics.rs`; `playersp181` checks both setter branches 10,000 times and 650 full ticks including the 600-tick expiration. Existing `playertick181` sprint/flight/movement/food scenarios remain EXACT. |
| Four view smoothing fields / `updateEntityActionState` | `player.rs::player_action`; `playersp181` checks 10,000 nonzero rotations, movement/jump fields and real rotated ticks/death. First-person snapshot now copies those same fields at the action callback, preserving the freeze when action processing stops. |
| `onLivingUpdate`: timers, use slowdown, sprint, flight, nausea | Existing production `player_before_living`/`update_sprint_and_flight`; 23 complete original `onUpdate` replays in `verify_playertick181.sh`, plus `playersp181`, EXACT. Original `func_35643_e` is false for both vanilla singleplayer controllers; the alternate camera-only branch has no reachable singleplayer caller. Portal behavior is retained from b173; nausea is checked by the complete 500-tick replay. |
| `func_35220_u_` FOV | `player/mechanics.rs::fov_multiplier`; 10,000 `playersp181` cases include flying, negative ground speed, combined speed/slowness and bow draw before zero/at/above full charge; retained tick fixtures check non-bow/no-use. |
| Removed input callbacks | Production input uses global `KeyBinding`/`MovementInputFromOptions`, already checked by `keyregistry181`, `controllerinput181` and whole-player replays. No legacy SP key accumulator remains. |
| `func_35200_b` critical callback | Integrator's credited `PlayerEvent::CriticalHit` callback uses the existing particle owner implementation. This round does not modify it; retained `playercombat181` and integrator critical checks provide the prior evidence. |
| `setHealth` damage source | Existing production `set_player_health`; `playerstate181` checks 6,000 health/XP callbacks, health/hurt timers, armor and exhaustion EXACT. |
| `addStat` | Only `StatBase.func_25067_a` becomes `isAchievement`; existing `StatFileWriter` achievement/parent protocol and `statsync181` cover the unchanged policy. |
| `func_35219_c` experience callback | Existing `set_player_experience`; `playerstate181` checks 6,000 signed random arguments/caps EXACT. |

Unchanged screen, respawn, chat, pickup and NBT callbacks are retained. The integrator credited whole-player NBT separately. The new oracle runs original jar methods rather than copying their control flow. Shared oracle world/listeners and serializers are unchanged.
