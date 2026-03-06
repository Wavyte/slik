use crate::driver::Driver;
use crate::transition::Transition;
use leptos::prelude::*;
use std::cell::RefCell;
use std::collections::HashMap;

// ---------------------------------------------------------------------------
// Slot — the scheduler's view of one animated value
// ---------------------------------------------------------------------------

struct Slot {
    driver: Option<Box<dyn Driver>>,
    output: RwSignal<f64>,
    transition: Transition,
}

// ---------------------------------------------------------------------------
// Scheduler state
// ---------------------------------------------------------------------------

struct Scheduler {
    slots: HashMap<u64, Slot>,
    next_id: u64,
    running: bool,
    last_time: Option<f64>,
}

impl Scheduler {
    fn new() -> Self {
        Self {
            slots: HashMap::new(),
            next_id: 0,
            running: false,
            last_time: None,
        }
    }

    fn register(&mut self, output: RwSignal<f64>, transition: Transition) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.slots.insert(
            id,
            Slot {
                driver: None,
                output,
                transition,
            },
        );
        id
    }

    fn unregister(&mut self, id: u64) {
        self.slots.remove(&id);
    }

    fn start_animation(&mut self, id: u64, current_value: f64, new_target: f64) {
        if let Some(slot) = self.slots.get_mut(&id) {
            match &mut slot.driver {
                Some(driver) => {
                    // Retarget in-flight driver (velocity preserved for springs)
                    driver.set_target(current_value, new_target);
                }
                None => {
                    // Spawn fresh driver from the slot's transition config
                    let mut driver = slot.transition.create_driver();
                    driver.set_target(current_value, new_target);
                    slot.driver = Some(driver);
                }
            }
        }
    }

    /// Tick all active slots. Returns `true` if any are still active.
    fn tick(&mut self, dt: f64) -> bool {
        let mut any_active = false;

        for slot in self.slots.values_mut() {
            if let Some(ref mut driver) = slot.driver {
                driver.tick(dt);
                slot.output.set(driver.value());

                if driver.is_done() {
                    slot.driver = None;
                } else {
                    any_active = true;
                }
            }
        }

        any_active
    }
}

// ---------------------------------------------------------------------------
// Thread-local singleton
// ---------------------------------------------------------------------------

thread_local! {
    static SCHEDULER: RefCell<Scheduler> = RefCell::new(Scheduler::new());
}

/// Register a new animated signal slot. Returns the slot ID.
pub(crate) fn register(output: RwSignal<f64>, transition: Transition) -> u64 {
    SCHEDULER.with(|s| s.borrow_mut().register(output, transition))
}

/// Remove a slot when the animated signal is cleaned up.
pub(crate) fn unregister(id: u64) {
    SCHEDULER.with(|s| s.borrow_mut().unregister(id));
}

/// Push or retarget an animation for the given slot.
pub(crate) fn start_animation(id: u64, current_value: f64, new_target: f64) {
    SCHEDULER.with(|s| {
        s.borrow_mut().start_animation(id, current_value, new_target);
    });
    ensure_running();
}

// ---------------------------------------------------------------------------
// rAF loop
// ---------------------------------------------------------------------------

#[cfg(target_arch = "wasm32")]
thread_local! {
    static RAF_CLOSURE: RefCell<Option<wasm_bindgen::closure::Closure<dyn FnMut(f64)>>>
        = const { RefCell::new(None) };
}

#[cfg(target_arch = "wasm32")]
fn ensure_raf_closure() {
    use wasm_bindgen::closure::Closure;

    RAF_CLOSURE.with(|c| {
        if c.borrow().is_none() {
            let closure = Closure::wrap(Box::new(|timestamp: f64| {
                on_frame(timestamp);
            }) as Box<dyn FnMut(f64)>);
            *c.borrow_mut() = Some(closure);
        }
    });
}

#[cfg(target_arch = "wasm32")]
fn request_frame() {
    use wasm_bindgen::JsCast;

    RAF_CLOSURE.with(|c| {
        let borrow = c.borrow();
        if let Some(closure) = borrow.as_ref() {
            if let Some(window) = web_sys::window() {
                let _ = window.request_animation_frame(closure.as_ref().unchecked_ref());
            }
        }
    });
}

#[cfg(target_arch = "wasm32")]
fn on_frame(timestamp: f64) {
    let should_continue = SCHEDULER.with(|s| {
        let mut sched = s.borrow_mut();

        let dt = match sched.last_time {
            Some(last) => ((timestamp - last) / 1000.0).min(0.064),
            None => 1.0 / 60.0,
        };
        sched.last_time = Some(timestamp);

        let active = sched.tick(dt);

        if !active {
            sched.running = false;
            sched.last_time = None;
        }

        active
    });

    if should_continue {
        request_frame();
    }
}

#[cfg(target_arch = "wasm32")]
fn ensure_running() {
    let needs_start = SCHEDULER.with(|s| {
        let mut sched = s.borrow_mut();
        if sched.running {
            return false;
        }
        sched.running = true;
        true
    });

    if needs_start {
        ensure_raf_closure();
        request_frame();
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn ensure_running() {}
