//! Original GuiInventory callback replay. Drawing is independently pixel checked.
use super::{Action, Screen, screens::container::GuiContainer, widgets::GuiButton};
use worldgen::container::{Container, ContainerKind, InventoryPlayer};
fn i(o: &mut Vec<u8>, n: i32) { o.extend(n.to_le_bytes()); }
fn actions(ui: &mut super::Ui, stats: &mut i32, o: &mut Vec<u8>) {
    let actions = std::mem::take(&mut ui.actions);
    i(o, actions.len() as i32);
    let mut kind = -2;
    for action in actions {
        match action {
            Action::RedirectInventory => { kind = 1; *stats += 1; }
            Action::Display(Some(_)) => kind = 2,
            Action::DisplayChild(_) => kind = 3,
            _ => panic!("unexpected inventory callback"),
        }
    }
    i(o, kind);
}
pub fn check(path: &str) -> bool {
    let mut ui = super::utility_check::fixture_ui("target".into());
    let mut out = Vec::new();
    for c in 0..10000 {
        let mut snapshot = Container::new(ContainerKind::Player, 0).snapshot(&InventoryPlayer::default());
        snapshot.creative = c & 1 != 0;
        snapshot.alive = c & 2 == 0 && c & 4 == 0;
        let mut screen = GuiContainer::new(&snapshot);
        ui.container_snapshot = Some(snapshot);
        screen.base().buttons.push(GuiButton::new(17,0,0,"unused"));
        let mut stats = 1;
        screen.init_gui(&mut ui);
        i(&mut out, screen.base().buttons.len() as i32);
        actions(&mut ui, &mut stats, &mut out);i(&mut out, stats);
        out.push(1);out.push(screen.does_gui_pause_game() as u8);
        for t in 0..3 {
            ui.container_snapshot.as_mut().unwrap().creative = (c+t)&1 != 0;
            screen.update_screen(&mut ui);
            actions(&mut ui, &mut stats, &mut out);i(&mut out, stats);
        }
        for id in [-1,0,1,2,100] {
            screen.action_performed(&mut ui, id);
            actions(&mut ui, &mut stats, &mut out);
        }
    }
    let expected = std::fs::read(path).unwrap();
    if out != expected {
        let at = out.iter().zip(&expected).position(|(a,b)|a!=b).unwrap_or(out.len().min(expected.len()));
        eprintln!("INVENTORY GUI DIFFERS byte {at}, lengths {}/{}",out.len(),expected.len());return false;
    }
    println!("INVENTORY GUI EXACT: 10000 original constructor/init cases, 30000 updates and 50000 button callbacks ({} bytes)",out.len());true
}
