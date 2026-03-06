#[cfg(target_arch = "wasm32")]
use crate::driver::Driver;
use crate::transition::Transition;
use leptos::prelude::*;
use std::cell::RefCell;
use std::collections::HashMap;

struct Slot {
    output: RwSignal<f64>,
    #[cfg(target_arch = "wasm32")]
    driver: Option<Box<dyn Driver>>,
    #[cfg(target_arch = "wasm32")]
    transition: Transition,
}

struct Scheduler {
    slots: HashMap<u64, Slot>,
    next_id: u64,
    #[cfg(target_arch = "wasm32")]
    running: bool,
    #[cfg(target_arch = "wasm32")]
    last_time: Option<f64>,
}

impl Scheduler {
    fn new() -> Self {
        Self {
            slots: HashMap::new(),
            next_id: 0,
            #[cfg(target_arch = "wasm32")]
            running: false,
            #[cfg(target_arch = "wasm32")]
            last_time: None,
        }
    }

    fn register(&mut self, output: RwSignal<f64>, transition: Transition) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.slots.insert(
            id,
            Slot {
                output,
                #[cfg(target_arch = "wasm32")]
                driver: None,
                #[cfg(target_arch = "wasm32")]
                transition,
            },
        );
        #[cfg(not(target_arch = "wasm32"))]
        let _ = transition;
        id
    }

    fn unregister(&mut self, id: u64) {
        self.slots.remove(&id);
    }

    #[cfg(target_arch = "wasm32")]
    fn stop_animation(&mut self, id: u64) {
        if let Some(slot) = self.slots.get_mut(&id) {
            slot.driver = None;
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn stop_animation(&mut self, _id: u64) {}

    fn snap_to(&mut self, id: u64, value: f64) {
        if let Some(slot) = self.slots.get_mut(&id) {
            #[cfg(target_arch = "wasm32")]
            {
                slot.driver = None;
            }
            slot.output.set(value);
        }
    }

    #[cfg(target_arch = "wasm32")]
    fn start_animation(&mut self, id: u64, current_value: f64, new_target: f64) {
        if let Some(slot) = self.slots.get_mut(&id) {
            match &mut slot.driver {
                Some(driver) => driver.set_target(current_value, new_target),
                None => {
                    let mut driver = slot.transition.create_driver();
                    driver.set_target(current_value, new_target);
                    slot.driver = Some(driver);
                }
            }
        }
    }

    #[cfg(target_arch = "wasm32")]
    fn tick(&mut self, dt: f64) -> (bool, Vec<(RwSignal<f64>, f64)>) {
        let mut any_active = false;
        let mut updates = Vec::new();

        for slot in self.slots.values_mut() {
            if let Some(ref mut driver) = slot.driver {
                driver.tick(dt);
                let value = driver.value();
                let done = driver.is_done();
                updates.push((slot.output, value));

                if done {
                    slot.driver = None;
                } else {
                    any_active = true;
                }
            }
        }

        (any_active, updates)
    }
}

thread_local! {
    static SCHEDULER: RefCell<Scheduler> = RefCell::new(Scheduler::new());
}

pub(crate) fn register(output: RwSignal<f64>, transition: Transition) -> u64 {
    SCHEDULER.with(|scheduler| scheduler.borrow_mut().register(output, transition))
}

pub(crate) fn unregister(id: u64) {
    SCHEDULER.with(|scheduler| scheduler.borrow_mut().unregister(id));
}

pub(crate) fn stop_animation(id: u64) {
    SCHEDULER.with(|scheduler| scheduler.borrow_mut().stop_animation(id));
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn start_animation(id: u64, current_value: f64, new_target: f64) {
    SCHEDULER.with(|scheduler| {
        scheduler
            .borrow_mut()
            .start_animation(id, current_value, new_target);
    });
    ensure_running();
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn start_animation(id: u64, _current_value: f64, new_target: f64) {
    SCHEDULER.with(|scheduler| scheduler.borrow_mut().snap_to(id, new_target));
}

#[cfg(target_arch = "wasm32")]
thread_local! {
    static RAF_CLOSURE: RefCell<Option<wasm_bindgen::closure::Closure<dyn FnMut(f64)>>> = const { RefCell::new(None) };
}

#[cfg(target_arch = "wasm32")]
fn ensure_raf_closure() {
    use wasm_bindgen::closure::Closure;

    RAF_CLOSURE.with(|slot| {
        if slot.borrow().is_none() {
            let closure = Closure::wrap(Box::new(|timestamp: f64| {
                on_frame(timestamp);
            }) as Box<dyn FnMut(f64)>);
            *slot.borrow_mut() = Some(closure);
        }
    });
}

#[cfg(target_arch = "wasm32")]
fn request_frame() {
    use wasm_bindgen::JsCast;

    RAF_CLOSURE.with(|slot| {
        let borrow = slot.borrow();
        if let Some(closure) = borrow.as_ref() {
            if let Some(window) = web_sys::window() {
                let _ = window.request_animation_frame(closure.as_ref().unchecked_ref());
            }
        }
    });
}

#[cfg(target_arch = "wasm32")]
fn on_frame(timestamp: f64) {
    let (should_continue, updates) = SCHEDULER.with(|scheduler| {
        let mut scheduler = scheduler.borrow_mut();
        let dt = match scheduler.last_time {
            Some(last) => ((timestamp - last) / 1000.0).min(0.064),
            None => 1.0 / 60.0,
        };
        scheduler.last_time = Some(timestamp);

        let (active, updates) = scheduler.tick(dt);
        if !active {
            scheduler.running = false;
            scheduler.last_time = None;
        }

        (active, updates)
    });

    for (signal, value) in updates {
        signal.set(value);
    }

    if should_continue {
        request_frame();
    }
}

#[cfg(target_arch = "wasm32")]
fn ensure_running() {
    let needs_start = SCHEDULER.with(|scheduler| {
        let mut scheduler = scheduler.borrow_mut();
        if scheduler.running {
            return false;
        }
        scheduler.running = true;
        true
    });

    if needs_start {
        ensure_raf_closure();
        request_frame();
    }
}
