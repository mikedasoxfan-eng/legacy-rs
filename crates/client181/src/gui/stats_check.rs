//! Original worker-thread and file-lifecycle comparison (observation barriers only).
use super::{
    stats::{Stats, ACHIEVEMENTS},
    stats_file::Syncher,
};
use std::{collections::HashMap, fmt::Write, path::Path};

fn map(map: Option<HashMap<i32, i32>>) -> String {
    let Some(map) = map else { return "null".into() };
    let mut rows: Vec<_> = map.into_iter().collect();
    rows.sort_unstable();
    let mut out = String::new();
    for (id, value) in rows {
        write!(out, "{id}={value};").unwrap();
    }
    out
}
fn snapshot(out: &mut String, stats: &Stats, dir: &Path, guids: &str) {
    stats.wait_idle();
    let total = stats.total_snapshot().clone();
    write!(
        out,
        "{}|{}|{}",
        stats.ready(),
        map(Some(total)),
        map(Some(stats.unsent_snapshot().clone()))
    )
    .unwrap();
    for name in [
        "stats_mixed_unsent.dat",
        "stats_mixed_unsent.old",
        "stats_mixed_unsent.tmp",
        "stats_mixed.dat",
    ] {
        let path = dir.join("stats").join(name);
        write!(
            out,
            "|{}",
            if path.is_dir() {
                "directory".into()
            } else if path.exists() {
                map(Syncher::read_file(&path, guids))
            } else {
                "missing".into()
            }
        )
        .unwrap();
    }
    out.push('|');
    for index in 0..ACHIEVEMENTS.len() {
        write!(
            out,
            "{},{};",
            stats.unlocked(index),
            stats.can_unlock(index)
        )
        .unwrap();
    }
    out.push('\n');
}
pub fn check(root: &Path) -> bool {
    let guids = crate::assets::Jar::open()
        .unwrap()
        .text("achievement/map.txt")
        .unwrap();
    let mut out = String::new();
    let ids = [
        1000, 1001, 1100, 2000, 2020, 5242880, 5242881, 5242882, 5242896, 16777217, 16842753,
    ];
    for scenario in 0..6 {
        let input = root.join(format!("input{scenario}"));
        let dir = root.join(format!("rust{scenario}"));
        std::fs::create_dir_all(dir.join("stats")).unwrap();
        for file in std::fs::read_dir(input).unwrap().flatten() {
            if file.path().is_dir() {
                for file in std::fs::read_dir(file.path()).unwrap().flatten() {
                    std::fs::copy(file.path(), dir.join("stats").join(file.file_name())).unwrap();
                }
            } else {
                std::fs::copy(file.path(), dir.join(file.file_name())).unwrap();
            }
        }
        let mut stats = Stats::open(&dir, "MiXeD", &guids);
        snapshot(&mut out, &stats, &dir, &guids);
        let mut r = worldgen::JavaRandom::new(181 + scenario);
        for t in 0..350 {
            if t == 210 {
                stats.wait_idle();
                std::fs::create_dir(dir.join("stats/stats_mixed_unsent.tmp")).unwrap();
                stats.sync();
                snapshot(&mut out, &stats, &dir, &guids);
            }
            if t == 211 {
                stats.wait_idle();
                std::fs::remove_dir(dir.join("stats/stats_mixed_unsent.tmp")).unwrap();
            }
            if t == 110 {
                stats.fixture_mailboxes(83 + scenario as i32);
                snapshot(&mut out, &stats, &dir, &guids);
            }
            let id = ids[r.next_int(ids.len() as i32) as usize];
            let n = r.next_int_unbounded();
            if t % 3 == 0 {
                stats.add(id, n);
            }
            if t % 47 == 0 {
                stats.sync();
            }
            stats.tick();
            snapshot(&mut out, &stats, &dir, &guids);
        }
        stats.sync();
        snapshot(&mut out, &stats, &dir, &guids);
    }
    std::fs::write(root.join("actual.txt"), &out).unwrap();
    let expected = std::fs::read_to_string(root.join("expected.txt"))
        .unwrap()
        .replace("\r\n", "\n");
    if out == expected {
        println!("STATS SYNCHRONIZATION EXACT: 2124 snapshots, 6 file recovery cases, both transfer mailboxes and write failures");
        true
    } else {
        let at = out
            .lines()
            .zip(expected.lines())
            .position(|(a, b)| a != b)
            .unwrap_or(0);
        eprintln!("STATS SYNCHRONIZATION DIFFERS at snapshot {at}");
        false
    }
}
