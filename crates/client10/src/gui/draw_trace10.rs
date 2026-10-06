//! Optional original-callback capture used by the bytecode verifiers. Disabled in game frames.
#[derive(Clone, Debug)]
pub(crate) enum Call {
    Rect([i32; 6], f32),
    Gradient([i32; 6], f32),
    Text(String, [i32; 5]),
    Item([i32; 6], f32),
}
thread_local! { static CALLS: std::cell::RefCell<Option<Vec<Call>>> = const { std::cell::RefCell::new(None) }; }
pub(crate) fn begin() {
    CALLS.with(|c| *c.borrow_mut() = Some(Vec::new()));
}
pub(crate) fn record(call: impl FnOnce() -> Call) {
    CALLS.with(|c| {
        if let Some(c) = c.borrow_mut().as_mut() {
            c.push(call());
        }
    });
}
pub(crate) fn take() -> Vec<Call> {
    CALLS.with(|c| c.borrow_mut().take().unwrap_or_default())
}
pub(crate) fn write(out: &mut Vec<u8>, calls: Vec<Call>) {
    use super::check10_round2::{i, s};
    i(out, calls.len() as i32);
    for call in calls {
        match call {
            Call::Rect(args, z) | Call::Gradient(args, z) | Call::Item(args, z) => {
                let kind = match call {
                    Call::Rect(..) => 0,
                    Call::Gradient(..) => 1,
                    _ => 3,
                };
                i(out, kind);
                for n in args {
                    i(out, n);
                }
                out.extend(z.to_le_bytes());
            }
            Call::Text(text, args) => {
                i(out, 2);
                s(out, &text);
                for n in args {
                    i(out, n);
                }
            }
        }
    }
}
