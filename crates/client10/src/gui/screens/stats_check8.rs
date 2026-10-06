use super::*;
fn int(out: &mut Vec<u8>, n: i32) {
    out.extend(n.to_le_bytes());
}
fn string(out: &mut Vec<u8>, s: &str) {
    int(out, s.len() as i32);
    out.extend(s.as_bytes());
}
pub fn check(path: &str) -> bool {
    let mut ui = crate::gui::utility_check::fixture_ui("target/stats8-saves".into());
    ui.textures.insert("gui/slot.png".into(), 0);
    let mut out = Vec::new();
    for c in 0i32..128 {
        ui.stats = Default::default();
        let value = |id: i32| match c % 8 {
            0 => i32::MIN,
            1 => i32::MAX,
            2 => -1,
            3 => 0,
            4 => 1,
            _ => (id ^ c).wrapping_mul(1664525).wrapping_add(1013904223) % 999 - 200,
        };
        let mut seen = std::collections::HashSet::new();
        for table in [
            &table::MINE[..],
            &table::USE[..],
            &table::BREAK[..],
            &table::CRAFT[..],
        ] {
            for &(item, id) in table {
                let _ = item;
                if seen.insert(id) {
                    ui.stats.add(id, value(id));
                }
            }
        }
        for kind in [Kind::Block, Kind::Item] {
            let mut list = StatsList::new(&ui, kind);
            int(&mut out, list.size());
            int(&mut out, list.size() * 20 + 20);
            out.push(list.is_selected(-1) as u8);
            for &(stat, item) in &list.rows {
                int(&mut out, stat);
                int(&mut out, item);
            }
            for column in -1..5 {
                string(&mut out, list.column_key(column));
            }
            for i in 0..list.size() {
                let x = 105 + c;
                let y = 60 + i * 20 - c;
                let (item, cells) = list.row_args(&ui, i, x, y);
                for n in [item, x + 42, y + 2] {
                    int(&mut out, n);
                }
                for cell in cells {
                    string(&mut out, &cell.text);
                    for n in [cell.x, cell.y, cell.color] {
                        int(&mut out, n);
                    }
                }
            }
            for x in [
                78, 79, 114, 115, 128, 129, 164, 165, 178, 179, 214, 215, 79, 79, 79, 129, 129,
                129, 179, 179, 179,
            ] {
                list.header_clicked(&mut ui, x, 0);
                for n in [list.pressed, list.sort_column, list.sort_dir] {
                    int(&mut out, n);
                }
                int(&mut out, ui.sounds.len() as i32);
                ui.sounds.clear();
                for &(stat, item) in &list.rows {
                    int(&mut out, stat);
                    int(&mut out, item);
                }
                ui.mouse_down = c % 2 == 0;
                list.draw_header(&mut ui, 105, 36);
                int(&mut out, list.pressed);
                int(&mut out, (ui.gl.verts.len() / 6 * 4) as i32);
                for quad in ui.gl.verts.chunks_exact(6) {
                    for index in [0, 1, 2, 5] {
                        let v = &quad[index];
                        for n in v.pos.into_iter().chain(v.uv).chain(v.color) {
                            out.extend(n.to_le_bytes());
                        }
                    }
                }
                ui.gl.verts.clear();
                ui.gl.draws.clear();
            }
            let mut slot = GuiSlot::new(427, 240, 32, 176, 20);
            slot.set_header(true, 20);
            slot.set_show_selection(false);
            for my in [31, 32, 36, 51, 52, 56, 60, 75, 175, 176, 177] {
                for mx in [
                    104, 105, 144, 145, 165, 166, 201, 202, 220, 221, 251, 252, 270, 271, 301, 302,
                    320, 321, 324, 325, 326,
                ] {
                    let mut calls = Vec::new();
                    if let Some(cell) = list.tooltip_args(&ui, &slot, mx, my) {
                        for n in [
                            cell.x - 3,
                            cell.y - 3,
                            cell.x + ui.string_width(&cell.text) + 3,
                            cell.y + 11,
                            -1073741824,
                            -1073741824,
                        ] {
                            int(&mut calls, n);
                        }
                        string(&mut calls, &cell.text);
                        for n in [cell.x, cell.y, cell.color] {
                            int(&mut calls, n);
                        }
                    }
                    int(&mut out, calls.len() as i32);
                    out.extend(calls);
                }
            }
        }
    }
    std::fs::write("target/stats8_181-rust.bin", &out).unwrap();
    let expected = std::fs::read(path).unwrap();
    if out != expected {
        let at = out
            .iter()
            .zip(&expected)
            .position(|(a, b)| a != b)
            .unwrap_or(out.len().min(expected.len()));
        eprintln!(
            "STATS LIST ROUND 8 DIFFERS byte {at}, lengths {}/{}",
            out.len(),
            expected.len()
        );
        false
    } else {
        println!("STATS LIST ROUND 8 EXACT: 256 catalog/row scenarios, 5376 header/sorting/vertex cases and 59136 tooltip boundaries ({} bytes)",out.len());
        true
    }
}
