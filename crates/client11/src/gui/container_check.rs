use super::{screens::container::GuiContainer, Screen};
use worldgen::container::{
    Container, ContainerCommand, ContainerEvent, ContainerKind, InventoryPlayer, ItemStack,
};

fn i(o: &mut Vec<u8>, v: i32) {
    o.extend(v.to_le_bytes());
}
fn stack(o: &mut Vec<u8>, s: Option<ItemStack>) {
    for n in s.map_or([-1, 0, 0, 0], |s| [s.id, s.count, s.damage, s.animations]) {
        i(o, n);
    }
}
pub fn check(path: &str) -> bool {
    let mut ui = super::utility_check::fixture_ui("target".into());
    let mut out = vec![];
    for kind in [
        ContainerKind::Player,
        ContainerKind::Workbench,
        ContainerKind::Chest { rows: 3 },
        ContainerKind::Creative,
        ContainerKind::Furnace,
        ContainerKind::Dispenser,
    ] {
        for c in 0..64 {
            let mut p = InventoryPlayer::default();
            let mut container = Container::new(kind, 0);
            for slot in 0..container.matrix.len() {
                if (slot as i32 + c) % 3 != 0 {
                    container.matrix[slot] = Some(ItemStack::new(5, c % 9 - 3, 0));
                }
            }
            p.cursor = if c % 4 == 0 {
                None
            } else {
                Some(ItemStack::new(264, c % 9 - 3, c))
            };
            let mut screen = GuiContainer::new(&container.snapshot(&p));
            screen.on_gui_closed(&mut ui);
            let commands = std::mem::take(&mut ui.container_commands);
            assert_eq!(commands.len(), 1);
            let ContainerCommand::CloseFromGui { window: 0 } = commands[0] else {
                panic!("GUI close protocol differs")
            };
            container.close(&mut p, false);
            container.close(&mut p, false);
            i(&mut out, 2);
            i(
                &mut out,
                if matches!(kind, ContainerKind::Chest { .. }) {
                    2
                } else {
                    0
                },
            );
            out.push(1);
            stack(&mut out, p.cursor);
            let drops: Vec<_> = container
                .events
                .iter()
                .filter_map(|e| {
                    if let ContainerEvent::Drop(s) = e {
                        Some(*s)
                    } else {
                        None
                    }
                })
                .collect();
            i(&mut out, drops.len() as i32);
            for s in drops {
                stack(&mut out, Some(s));
            }
            i(&mut out, container.matrix.len() as i32);
            for s in container.matrix {
                stack(&mut out, s);
            }
        }
    }
    let expected = std::fs::read(path).unwrap();
    if out != expected {
        let at = out
            .iter()
            .zip(&expected)
            .position(|(a, b)| a != b)
            .unwrap_or(out.len().min(expected.len()));
        eprintln!(
            "GUI CONTAINER CLOSE DIFFERS byte {at}, lengths {}/{}",
            out.len(),
            expected.len()
        );
        return false;
    }
    println!("GUI CONTAINER CLOSE EXACT: 384 original callbacks, six containers, double close/drop semantics ({} bytes)",out.len());
    true
}
