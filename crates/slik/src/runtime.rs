#[cfg(target_arch = "wasm32")]
use crate::driver::ActiveAnimation;
use crate::transition::Transition;
use leptos::prelude::*;
use std::cell::RefCell;

pub(crate) type SlotId = usize;

struct Slot {
    output: RwSignal<f64>,
    transition: Transition,
    #[cfg(target_arch = "wasm32")]
    active: Option<ActiveAnimation>,
}

struct Runtime {
    slots: Vec<Option<Slot>>,
    free_list: Vec<SlotId>,
    #[cfg(target_arch = "wasm32")]
    running: bool,
    #[cfg(target_arch = "wasm32")]
    last_time: Option<f64>,
}

impl Runtime {
    fn new() -> Self {
        Self {
            slots: Vec::new(),
            free_list: Vec::new(),
            #[cfg(target_arch = "wasm32")]
            running: false,
            #[cfg(target_arch = "wasm32")]
            last_time: None,
        }
    }

    fn register(&mut self, output: RwSignal<f64>, transition: Transition) -> SlotId {
        let slot = Slot {
            output,
            transition,
            #[cfg(target_arch = "wasm32")]
            active: None,
        };

        if let Some(id) = self.free_list.pop() {
            self.slots[id] = Some(slot);
            id
        } else {
            let id = self.slots.len();
            self.slots.push(Some(slot));
            id
        }
    }

    fn unregister(&mut self, id: SlotId) {
        if let Some(entry) = self.slots.get_mut(id) {
            if entry.take().is_some() {
                self.free_list.push(id);
            }
        }
    }

    fn set_transition(&mut self, id: SlotId, transition: Transition) {
        if let Some(Some(slot)) = self.slots.get_mut(id) {
            slot.transition = transition;
        }
    }

    fn jump(&mut self, id: SlotId, value: f64) {
        if let Some(Some(slot)) = self.slots.get_mut(id) {
            #[cfg(target_arch = "wasm32")]
            {
                slot.active = None;
            }
            slot.output.set(value);
        }
    }

    fn stop(&mut self, id: SlotId) {
        #[cfg(target_arch = "wasm32")]
        if let Some(Some(slot)) = self.slots.get_mut(id) {
            slot.active = None;
        }

        #[cfg(not(target_arch = "wasm32"))]
        let _ = id;
    }

    #[cfg(target_arch = "wasm32")]
    fn start_animation(
        &mut self,
        id: SlotId,
        current_value: f64,
        new_target: f64,
        immediate: bool,
    ) {
        if immediate {
            self.jump(id, new_target);
            return;
        }

        if let Some(Some(slot)) = self.slots.get_mut(id) {
            match &mut slot.active {
                Some(active) => {
                    active.set_target(&slot.transition, current_value, new_target);
                }
                None => {
                    let mut active = ActiveAnimation::from_transition(&slot.transition);
                    active.set_target(&slot.transition, current_value, new_target);
                    slot.active = Some(active);
                }
            }
        }
    }

    #[cfg(target_arch = "wasm32")]
    fn tick(&mut self, dt: f64) -> (bool, Vec<(RwSignal<f64>, f64)>) {
        let mut updates = Vec::new();
        let mut any_active = false;

        for slot in self.slots.iter_mut().flatten() {
            if let Some(active) = &mut slot.active {
                active.tick(dt);
                let value = active.value();
                let done = active.is_done();
                updates.push((slot.output, value));

                if done {
                    slot.active = None;
                } else {
                    any_active = true;
                }
            }
        }

        (any_active, updates)
    }
}

thread_local! {
    static RUNTIME: RefCell<Runtime> = RefCell::new(Runtime::new());
}

pub(crate) fn register(output: RwSignal<f64>, transition: Transition) -> SlotId {
    RUNTIME.with(|runtime| runtime.borrow_mut().register(output, transition))
}

pub(crate) fn unregister(id: SlotId) {
    RUNTIME.with(|runtime| runtime.borrow_mut().unregister(id));
}

pub(crate) fn set_transition(id: SlotId, transition: Transition) {
    RUNTIME.with(|runtime| runtime.borrow_mut().set_transition(id, transition));
}

pub(crate) fn jump(id: SlotId, value: f64) {
    RUNTIME.with(|runtime| runtime.borrow_mut().jump(id, value));
}

pub(crate) fn stop(id: SlotId) {
    RUNTIME.with(|runtime| runtime.borrow_mut().stop(id));
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn start_animation(id: SlotId, current_value: f64, new_target: f64, immediate: bool) {
    RUNTIME.with(|runtime| {
        runtime
            .borrow_mut()
            .start_animation(id, current_value, new_target, immediate);
    });
    ensure_running();
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn start_animation(id: SlotId, _current_value: f64, new_target: f64, _immediate: bool) {
    jump(id, new_target);
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
    let (should_continue, updates) = RUNTIME.with(|runtime| {
        let mut runtime = runtime.borrow_mut();
        let dt = match runtime.last_time {
            Some(last) => ((timestamp - last) / 1000.0).min(0.064),
            None => 1.0 / 60.0,
        };
        runtime.last_time = Some(timestamp);

        let (active, updates) = runtime.tick(dt);
        if !active {
            runtime.running = false;
            runtime.last_time = None;
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
    let needs_start = RUNTIME.with(|runtime| {
        let mut runtime = runtime.borrow_mut();
        if runtime.running {
            return false;
        }
        runtime.running = true;
        true
    });

    if needs_start {
        ensure_raf_closure();
        request_frame();
    }
}
