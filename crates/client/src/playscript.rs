//! `--play-script <file>`: drive the player from a text file, for testing the game without
//! anyone at the keyboard. One command per line, run that many 20 Hz steps after the player
//! appears (the clock keeps running while the game is paused):
//!
//! ```text
//! 20 key W down        # keys by winit name: W, Space, ShiftLeft, Digit1, Q, Escape...
//! 60 key W up
//! 61 mouse left down   # left / right / middle, down / up
//! 62 look 300 -50      # raw mouse counts (x right, y down), like a mouse move
//! 64 move 282 374      # put the cursor at window pixel (x, y) (for screens)
//! 63 scroll -1         # wheel notches, up is positive
//! 70 shot walked       # screenshot <dir>/walked.png (dir = the script's name without .txt)
//! 80 button 1          # press the open screen's button with this id
//! 90 quit              # save and close
//! ```

use winit::keyboard::KeyCode;

#[derive(Clone, Debug)]
pub enum Command {
    Key(KeyCode, bool),
    Mouse(i32, bool),
    Look(f64, f64),
    Move(f64, f64),
    Scroll(i32),
    Shot(String),
    Button(i32),
    Quit,
}

pub struct Script {
    pub dir: std::path::PathBuf,
    /// (tick, command), in file order.
    pub steps: Vec<(i64, Command)>,
    pub next: usize,
}

impl Script {
    pub fn load(path: &std::path::Path) -> Result<Script, String> {
        let text = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
        let mut steps = Vec::new();
        for (n, line) in text.lines().enumerate() {
            let line = line.split('#').next().unwrap_or("").trim();
            if line.is_empty() {
                continue;
            }
            let bad = || format!("{}:{}: can't read '{line}'", path.display(), n + 1);
            let w: Vec<&str> = line.split_whitespace().collect();
            let tick: i64 = w[0].parse().map_err(|_| bad())?;
            let arg = |i: usize| w.get(i).copied().ok_or_else(bad);
            let updown = |s: &str| match s {
                "down" => Ok(true),
                "up" => Ok(false),
                _ => Err(bad()),
            };
            let cmd = match arg(1)? {
                "key" => Command::Key(key_code(arg(2)?).ok_or_else(bad)?, updown(arg(3)?)?),
                "mouse" => {
                    let b = match arg(2)? {
                        "left" => 0,
                        "right" => 1,
                        "middle" => 2,
                        _ => return Err(bad()),
                    };
                    Command::Mouse(b, updown(arg(3)?)?)
                }
                "look" => Command::Look(arg(2)?.parse().map_err(|_| bad())?, arg(3)?.parse().map_err(|_| bad())?),
                "scroll" => Command::Scroll(arg(2)?.parse().map_err(|_| bad())?),
                "move" => Command::Move(arg(2)?.parse().map_err(|_| bad())?, arg(3)?.parse().map_err(|_| bad())?),
                "shot" => Command::Shot(arg(2)?.to_string()),
                "button" => Command::Button(arg(2)?.parse().map_err(|_| bad())?),
                "quit" => Command::Quit,
                _ => return Err(bad()),
            };
            steps.push((tick, cmd));
        }
        let dir = path.with_extension("");
        std::fs::create_dir_all(&dir).map_err(|e| format!("{}: {e}", dir.display()))?;
        Ok(Script { dir, steps, next: 0 })
    }

    /// The commands due at `tick`.
    pub fn due(&mut self, tick: i64) -> Vec<Command> {
        let mut out = Vec::new();
        while let Some((t, c)) = self.steps.get(self.next) {
            if *t > tick {
                break;
            }
            out.push(c.clone());
            self.next += 1;
        }
        out
    }
}

fn key_code(name: &str) -> Option<KeyCode> {
    use KeyCode::*;
    let k = match name {
        "Escape" => Escape,
        "Space" => Space,
        "ShiftLeft" => ShiftLeft,
        "ControlLeft" => ControlLeft,
        "Tab" => Tab,
        "Enter" => Enter,
        "F1" => F1,
        "F3" => F3,
        "F5" => F5,
        _ => {
            let c = name.strip_prefix("Digit").map(|d| ("Digit", d)).unwrap_or(("Key", name));
            let ch = c.1.chars().next()?;
            if c.1.len() != 1 {
                return None;
            }
            match (c.0, ch) {
                ("Digit", '0'..='9') => {
                    [Digit0, Digit1, Digit2, Digit3, Digit4, Digit5, Digit6, Digit7, Digit8, Digit9]
                        [(ch as u8 - b'0') as usize]
                }
                ("Key", 'A'..='Z') => [
                    KeyA, KeyB, KeyC, KeyD, KeyE, KeyF, KeyG, KeyH, KeyI, KeyJ, KeyK, KeyL, KeyM, KeyN, KeyO, KeyP,
                    KeyQ, KeyR, KeyS, KeyT, KeyU, KeyV, KeyW, KeyX, KeyY, KeyZ,
                ][(ch as u8 - b'A') as usize],
                _ => return None,
            }
        }
    };
    Some(k)
}
