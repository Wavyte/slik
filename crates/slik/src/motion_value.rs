use crate::runtime::{self, SlotId};
use crate::transition::Transition;
use leptos::prelude::*;

#[derive(Clone, Copy)]
pub struct MotionValue {
    slot_id: SlotId,
    current: RwSignal<f64>,
    animating: RwSignal<bool>,
    last_target: RwSignal<f64>,
}

impl MotionValue {
    pub fn new(initial: f64, transition: Transition) -> Self {
        let current = RwSignal::new(initial);
        let animating = RwSignal::new(false);
        let last_target = RwSignal::new(initial);
        let slot_id = runtime::register(current, animating, transition);

        on_cleanup(move || runtime::unregister(slot_id));

        Self {
            slot_id,
            current,
            animating,
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

    #[inline]
    pub fn is_animating(&self) -> bool {
        self.animating.get()
    }

    #[inline]
    pub fn is_animating_untracked(&self) -> bool {
        self.animating.get_untracked()
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
        let is_animating = self.animating.get_untracked();
        self.last_target.set(target);

        if immediate {
            if !same_value(current_value, target) || is_animating {
                runtime::jump(self.slot_id, target);
            }
            return;
        }

        if same_value(current_value, target) && !is_animating {
            return;
        }

        runtime::start_animation(self.slot_id, current_value, target, immediate);
    }
}

#[inline]
fn same_value(left: f64, right: f64) -> bool {
    (left - right).abs() <= 1.0e-9
}
