use crate::scheduler;
use crate::transition::Transition;
use leptos::prelude::*;

#[derive(Clone, Copy)]
pub struct AnimatedSignal {
    slot_id: u64,
    current: RwSignal<f64>,
    last_target: RwSignal<f64>,
}

impl AnimatedSignal {
    pub fn new(initial: f64, transition: Transition) -> Self {
        let current = RwSignal::new(initial);
        let last_target = RwSignal::new(initial);
        let slot_id = scheduler::register(current, transition);

        on_cleanup(move || scheduler::unregister(slot_id));

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
        let current_value = self.current.get_untracked();
        let previous_target = self.last_target.get_untracked();

        if (target - previous_target).abs() < f64::EPSILON {
            return;
        }

        self.last_target.set(target);
        scheduler::start_animation(self.slot_id, current_value, target);
    }

    pub fn set_immediate(&self, value: f64) {
        scheduler::stop_animation(self.slot_id);
        self.current.set(value);
        self.last_target.set(value);
    }

    pub fn signal(&self) -> RwSignal<f64> {
        self.current
    }

    pub fn slot_id(&self) -> u64 {
        self.slot_id
    }
}
