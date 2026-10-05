//! Minecraft.runTick's event and binding queues. The window supplies LWJGL-style
//! events; the world thread and the headless bytecode replay use the same driver.
use crate::gui::settings::GameSettings;

#[derive(Clone, Copy, Debug)]
pub struct MouseEvent {
    pub button: i32,
    pub pressed: bool,
    pub wheel: i32,
    pub fresh: bool,
}
#[derive(Clone, Copy, Debug)]
pub struct KeyEvent {
    pub code: i32,
    pub pressed: bool,
}
#[derive(Clone, Debug, Default)]
pub struct Events {
    pub mouse: Vec<MouseEvent>,
    pub keys: Vec<KeyEvent>,
    pub down: Vec<i32>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Action {
    Focus,
    Inventory,
    Chat,
    Menu,
    Fullscreen,
    Reload,
    Settings,
    Container,
}

#[derive(Clone, Copy, Debug)]
pub struct TickPolicy {
    pub difficulty: i32,
    pub join: bool,
    pub entities: bool,
    pub world: bool,
    pub display: bool,
}

/// The world/difficulty tail of runTick, including ticks while a screen pauses.
pub fn tick_policy(
    counter: &mut i32,
    difficulty: i32,
    paused: bool,
    multiplayer: bool,
) -> TickPolicy {
    *counter = counter.wrapping_add(1);
    let join = *counter == 30;
    if join {
        *counter = 0;
    }
    TickPolicy {
        difficulty: if multiplayer { 1 } else { difficulty },
        join,
        entities: !paused,
        world: !paused || multiplayer,
        display: !paused,
    }
}

pub fn debug_entities(particles: usize, loaded_entities: usize) -> String {
    format!("P: {particles}. T: All: {loaded_entities}")
}

/// Calls across Minecraft's controller/screen boundary, supplied by the live
/// worker or by a fixture. Item use and cooldowns are read from the live player.
pub trait Host {
    fn screen(&self) -> bool;
    fn allow_input(&self) -> bool {
        false
    }
    fn focused(&self) -> bool;
    fn multiplayer(&self) -> bool;
    fn left_counter(&mut self) -> &mut i32;
    fn right_counter(&mut self) -> &mut i32;
    fn action(&mut self, action: Action);
    fn screen_mouse(&mut self, _event: MouseEvent) {}
    fn screen_key(&mut self, _event: KeyEvent) {}
    fn scroll(&mut self, wheel: i32);
    fn hotbar(&mut self, slot: i32);
    fn drop_item(&mut self);
    fn using_item(&self) -> bool;
    fn stop_using(&mut self);
    fn click(&mut self, right: bool);
    fn pick(&mut self);
    fn mine(&mut self, held: bool);
}

fn take(settings: &GameSettings, name: &str) -> bool {
    settings
        .key_bindings
        .iter()
        .find(|k| k.description == name)
        .is_some_and(|k| k.take_press())
}

pub fn run_tick(settings: &mut GameSettings, host: &mut impl Host, events: Events) {
    if *host.right_counter() > 0 {
        *host.right_counter() -= 1;
    }
    if host.screen() {
        *host.left_counter() = 10000;
    }
    // A normal GuiScreen drains both queues before the Minecraft input branch.
    // The live app already handles these callbacks on its window thread.
    if host.screen() {
        for &event in &events.mouse {
            host.screen_mouse(event);
        }
        for &event in &events.keys {
            host.screen_key(event);
        }
        if host.screen() && !host.allow_input() {
            return;
        }
        if *host.left_counter() > 0 {
            *host.left_counter() -= 1;
        }
        // handleInput consumed all of the events, even for allowUserInput screens.
    } else {
        for event in events.mouse {
            settings.input_event(event.button - 100, event.pressed);
            if event.fresh {
                if event.wheel != 0 {
                    host.scroll(event.wheel);
                    if settings.smooth_camera_zoom {
                        settings.smooth_camera_zoom_amount += event.wheel.signum() as f32 * 0.25f32;
                    }
                }
                if !host.screen() && !host.focused() && event.pressed {
                    host.action(Action::Focus);
                } else if host.screen() {
                    host.screen_mouse(event);
                }
            }
        }
        if *host.left_counter() > 0 {
            *host.left_counter() -= 1;
        }
        for event in events.keys {
            settings.input_event(event.code, event.pressed);
            if !event.pressed {
                continue;
            }
            if event.code == 87 {
                host.action(Action::Fullscreen);
                continue;
            }
            if host.screen() {
                host.screen_key(event);
            } else {
                if event.code == 1 {
                    host.action(Action::Menu);
                }
                if event.code == 31 && events.down.contains(&61) {
                    host.action(Action::Reload);
                }
                if event.code == 33 && events.down.contains(&61) {
                    settings.set_option_value(
                        crate::gui::settings::EnumOptions::RenderDistance,
                        if events.down.contains(&42) || events.down.contains(&54) {
                            -1
                        } else {
                            1
                        },
                    );
                    host.action(Action::Settings);
                }
                if event.code == 59 {
                    settings.hide_gui = !settings.hide_gui;
                }
                if event.code == 61 {
                    settings.show_debug_info = !settings.show_debug_info;
                }
                if event.code == 63 {
                    settings.third_person_view = !settings.third_person_view;
                }
            }
            if (2..=10).contains(&event.code) {
                host.hotbar(event.code - 2);
            }
        }
    }
    // Displaying a screen can release every binding in these loops. Do not
    // eagerly drain counts into independent integers before invoking callbacks.
    while take(settings, "key.inventory") {
        host.action(Action::Inventory);
    }
    while take(settings, "key.drop") {
        host.drop_item();
    }
    while host.multiplayer() && take(settings, "key.chat") {
        host.action(Action::Chat);
    }
    if host.using_item() {
        if !settings.binding_pressed("key.use") {
            host.stop_using();
        }
    } else {
        while take(settings, "key.attack") {
            host.click(false);
        }
        while take(settings, "key.use") {
            host.click(true);
        }
        while take(settings, "key.pickItem") {
            host.pick();
        }
    }
    if settings.binding_pressed("key.use") && *host.right_counter() == 0 && !host.using_item() {
        host.click(true);
    }
    host.mine(!host.screen() && settings.binding_pressed("key.attack") && host.focused());
}
