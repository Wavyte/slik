use crate::runtime::{self, SlotId};
use crate::transition::Transition;
use leptos::prelude::*;

#[derive(Clone, Copy)]
pub struct MotionValue {
    slot_id: SlotId,
    current: RwSignal<f64>,
    last_target: RwSignal<f64>,
}

impl MotionValue {
    pub fn new(initial: f64, transition: Transition) -> Self {
        let current = RwSignal::new(initial);
        let last_target = RwSignal::new(initial);
        let slot_id = runtime::register(current, transition);

        on_cleanup(move || runtime::unregister(slot_id));

        Self {
            slot_id,
            current,
            last_target,
        }
    }

    #[inline]
    pub fn get(&self) -> f64 {
        self.current.get()
    }

    #[inline]
    pub fn get_untracked(&self) -> f64 {
        self.current.get_untracked()
    }

    #[inline]
    pub fn target(&self) -> f64 {
        self.last_target.get_untracked()
    }

    pub fn set_target(&self, target: f64) {
        self.set_target_internal(target, false);
    }

    pub fn jump(&self, value: f64) {
        runtime::jump(self.slot_id, value);
        self.last_target.set(value);
    }

    pub fn stop(&self) {
        let current_value = self.current.get_untracked();
        runtime::stop(self.slot_id);
        self.last_target.set(current_value);
    }

    pub fn signal(&self) -> RwSignal<f64> {
        self.current
    }

    pub(crate) fn set_transition(&self, transition: Transition) {
        runtime::set_transition(self.slot_id, transition);
    }

    pub(crate) fn set_target_immediate(&self, target: f64) {
        self.set_target_internal(target, true);
    }

    fn set_target_internal(&self, target: f64, immediate: bool) {
        let current_value = self.current.get_untracked();
        let previous_target = self.last_target.get_untracked();

        if (target - previous_target).abs() < f64::EPSILON {
            return;
        }

        self.last_target.set(target);
        runtime::start_animation(self.slot_id, current_value, target, immediate);
    }
}
