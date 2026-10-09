//! Release 1.0 Profiler; explicit timestamps permit deterministic bytecode checks.
use std::{collections::HashMap, sync::{LazyLock, Mutex}, time::Instant};

#[derive(Clone, Debug)]
pub struct ProfilerResult {
    pub name: String,
    pub local_percent: f64,
    pub root_percent: f64,
}
impl ProfilerResult {
    pub fn compare(&self, other: &Self) -> std::cmp::Ordering {
        if other.local_percent < self.local_percent { std::cmp::Ordering::Less }
        else if other.local_percent > self.local_percent { std::cmp::Ordering::Greater }
        else { other.name.encode_utf16().cmp(self.name.encode_utf16()) }
    }
    pub fn color(&self) -> i32 {
        let hash = self.name.encode_utf16().fold(0i32, |h, c| h.wrapping_mul(31).wrapping_add(c as i32));
        (hash & 11184810) + 4473924
    }
}
#[derive(Default)]
pub struct Profiler {
    pub enabled: bool,
    sections: Vec<String>,
    timestamps: Vec<i64>,
    section: String,
    pub times: HashMap<String, i64>,
}
impl Profiler {
    pub fn clear(&mut self) { self.times.clear(); }
    pub fn start_at(&mut self, name: &str, now: i64) {
        if !self.enabled { return; }
        if !self.section.is_empty() { self.section.push('.'); }
        self.section.push_str(name);
        self.sections.push(self.section.clone());
        self.timestamps.push(now);
    }
    pub fn end_at(&mut self, now: i64) {
        if !self.enabled { return; }
        let elapsed = now.wrapping_sub(self.timestamps.pop().expect("Profiler timestamp stack underflow"));
        self.sections.pop().expect("Profiler section stack underflow");
        let entry = self.times.entry(self.section.clone()).or_default();
        *entry = entry.wrapping_add(elapsed);
        self.section = self.sections.last().cloned().unwrap_or_default();
    }
    pub fn results(&mut self, section: &str) -> Option<Vec<ProfilerResult>> {
        if !self.enabled { return None; }
        let mut root = *self.times.get("root").unwrap_or(&0);
        let section_time = *self.times.get(section).unwrap_or(&-1);
        let prefix = if section.is_empty() { String::new() } else { format!("{section}.") };
        // Java indexOf('.', prefix.length() + 1) intentionally skips the first suffix unit.
        let children: Vec<_> = self.times.iter().filter_map(|(key, value)| {
            let suffix = key.strip_prefix(&prefix)?;
            (!suffix.is_empty() && !suffix.encode_utf16().skip(1).any(|c| c == 46))
                .then_some((suffix.to_owned(), *value))
        }).collect();
        let mut total = children.iter().fold(0i64, |sum, (_, v)| sum.wrapping_add(*v));
        let original = total as f32;
        if total < section_time { total = section_time; }
        if root < total { root = total; }
        let mut results: Vec<_> = children.into_iter().map(|(name, value)| ProfilerResult {
            name, local_percent: value as f64 * 100.0 / total as f64,
            root_percent: value as f64 * 100.0 / root as f64,
        }).collect();
        for time in self.times.values_mut() { *time = time.wrapping_mul(999) / 1000; }
        if total as f32 > original {
            let remainder = (total as f32 - original) as f64;
            results.push(ProfilerResult { name: "unspecified".into(),
                local_percent: remainder * 100.0 / total as f64,
                root_percent: remainder * 100.0 / root as f64 });
        }
        results.sort_by(ProfilerResult::compare);
        results.insert(0, ProfilerResult { name: section.to_owned(), local_percent: 100.0,
            root_percent: total as f64 * 100.0 / root as f64 });
        Some(results)
    }
}
static GLOBAL: LazyLock<Mutex<Profiler>> = LazyLock::new(|| Mutex::new(Profiler::default()));
static EPOCH: LazyLock<Instant> = LazyLock::new(Instant::now);
fn nano_time() -> i64 { EPOCH.elapsed().as_nanos() as i64 }
pub fn enable(enabled: bool) { GLOBAL.lock().unwrap().enabled = enabled; }
pub fn clear() { GLOBAL.lock().unwrap().clear(); }
pub fn start_section(name: &str) { GLOBAL.lock().unwrap().start_at(name, nano_time()); }
pub fn end_section() { GLOBAL.lock().unwrap().end_at(nano_time()); }
pub fn end_start_section(name: &str) { end_section(); start_section(name); }
pub fn get_profiling_data(name: &str) -> Option<Vec<ProfilerResult>> { GLOBAL.lock().unwrap().results(name) }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn nested_sections_account_for_remainder_and_decay_once_per_query() {
        let mut p = Profiler { enabled: true, ..Default::default() };
        p.start_at("root", 0);
        p.start_at("b", 10); p.end_at(30);
        p.start_at("a", 30); p.end_at(50);
        p.end_at(100);
        let r = p.results("root").unwrap();
        assert_eq!(r.iter().map(|r| r.name.as_str()).collect::<Vec<_>>(), ["root", "unspecified", "b", "a"]);
        assert_eq!(r[1].local_percent, 60.0);
        assert_eq!(p.times["root"], 99);
        assert_eq!(p.times["root.a"], 19);
        p.clear();
        assert!(p.times.is_empty());
        assert!(Profiler::default().results("root").is_none());
    }
}
