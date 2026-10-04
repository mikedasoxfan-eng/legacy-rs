use worldgen::container::{Container, ContainerEvent, ContainerKind, InventoryPlayer, ItemStack};
fn stack(out: &mut Vec<u8>, s: Option<ItemStack>) {
    out.push(s.is_some() as u8);
    if let Some(s) = s {
        for n in [s.id, s.count, s.damage] {
            out.extend(n.to_be_bytes());
        }
    }
}
fn read(a: &[&str]) -> Option<ItemStack> {
    let id: i32 = a[0].parse().unwrap();
    (id >= 0).then(|| ItemStack::new(id, a[1].parse().unwrap(), a[2].parse().unwrap()))
}
fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args[1] == "tiles" {
        let actual = worldgen::container::verify::tile_bytes(args[2].parse().unwrap());
        let expected = std::fs::read(&args[3]).unwrap();
        if actual != expected {
            std::fs::write("target/container-tiles-rust.bin", &actual).unwrap();
            let i = actual
                .iter()
                .zip(&expected)
                .position(|(a, b)| a != b)
                .unwrap_or(actual.len().min(expected.len()));
            panic!(
                "tile replay differs at byte {i}; Rust {} bytes, Java {} bytes",
                actual.len(),
                expected.len()
            );
        }
        println!(
            "CONTAINER TILES EXACT: {} ticks, entities and furnace counters each frame, full world fingerprint each frame and byte-exact final world",
            args[2]
        );
        return;
    }
    let input = std::fs::read_to_string(&args[1]).unwrap();
    let expected = std::fs::read(&args[2]).unwrap();
    let mut p = InventoryPlayer::default();
    let mut c = Container::new(ContainerKind::Player, 0);
    let mut out = Vec::new();
    let mut records = 0;
    for (line_number, line) in input.lines().enumerate() {
        if line.trim().is_empty() || line.starts_with('#') {
            continue;
        }
        let a: Vec<_> = line.split_whitespace().collect();
        let mut result = None;
        match a[0] {
            "open" => {
                let map_id = c.next_map_id;
                p = InventoryPlayer::default();
                c = Container::new(
                    match a[1] {
                        "0" => ContainerKind::Player,
                        "1" => ContainerKind::Workbench,
                        "2" => ContainerKind::Chest { rows: 3 },
                        "3" => ContainerKind::Chest { rows: 6 },
                        "4" => ContainerKind::Furnace,
                        _ => ContainerKind::Dispenser,
                    },
                    0,
                );
                c.next_map_id = map_id;
            }
            "put" => c.put_stack(&mut p, a[1].parse().unwrap(), read(&a[2..])),
            "cursor" => p.cursor = read(&a[1..]),
            "click" => {
                result = c.slot_click(
                    &mut p,
                    a[1].parse().unwrap(),
                    a[2].parse().unwrap(),
                    a[3] == "1",
                )
            }
            "close" => c.close(&mut p, false),
            "expect" => assert_eq!(
                c.stack(&p, 0),
                read(&a[1..]),
                "uncrafted recipe at line {}",
                line_number + 1
            ),
            _ => panic!("unknown operation"),
        }
        let start = out.len();
        stack(&mut out, result);
        out.extend((c.slots.len() as i32).to_be_bytes());
        for i in 0..c.slots.len() {
            stack(&mut out, c.stack(&p, i));
        }
        stack(&mut out, p.cursor);
        for s in p.armor {
            stack(&mut out, s);
        }
        for s in p.main {
            stack(&mut out, s);
        }
        let hooks: Vec<_> = c
            .events
            .drain(..)
            .filter(|e| {
                matches!(
                    e,
                    ContainerEvent::Crafted { .. } | ContainerEvent::Achievement(_)
                )
            })
            .collect();
        out.extend((hooks.len() as i32).to_be_bytes());
        for hook in hooks {
            match hook {
                ContainerEvent::Crafted { count, .. } => {
                    out.push(0);
                    out.extend(count.to_be_bytes());
                }
                ContainerEvent::Achievement(name) => {
                    out.push(1);
                    out.extend((name.len() as u16).to_be_bytes());
                    out.extend(name.as_bytes());
                    out.extend(1i32.to_be_bytes());
                }
                _ => unreachable!(),
            }
        }
        records += 1;
        if expected.get(start..out.len()) != Some(&out[start..]) {
            std::fs::write("target/container-rust.bin", &out).unwrap();
            let at = (start..out.len())
                .find(|&i| expected.get(i) != out.get(i))
                .unwrap_or(start);
            panic!(
                "container mismatch record {records}, input line {} ({line}), byte {at}; Rust output in target/container-rust.bin",
                line_number + 1
            );
        }
    }
    assert_eq!(out.len(), expected.len());
    println!(
        "CONTAINERS EXACT: {records} records (return stack, every slot, cursor, armor, inventory, crafting-stat counts and achievement hooks)"
    );
}
