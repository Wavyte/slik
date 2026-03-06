use crate::scheduler;
use crate::transition::Transition;
use leptos::prelude::*;

/// A signal whose value transitions smoothly over time rather than changing
/// instantly.
///
/// `AnimatedSignal` is **`Copy`** (just like Leptos signals), so it can be
/// freely captured by multiple closures without cloning.
///
/// Internally owns a scheduler slot. Reading `.get()` returns the current
/// interpolated value. Calling `.set_target()` begins (or redirects) an
/// animation toward the new value.
///
/// ```ignore
/// let opacity = AnimatedSignal::new(0.0, Transition::tween(0.3, Easing::EaseOut));
/// opacity.set_target(1.0); // smoothly fades in
///
/// // Use in views — creates a fine-grained reactive dependency:
/// view! { <div style:opacity=move || opacity.get()>"Hello"</div> }
/// ```
#[derive(Clone, Copy)]
pub struct AnimatedSignal {
    /// Unique scheduler slot identifier.
    slot_id: u64,
    /// Reactive output — updated each rAF tick by the scheduler.
    current: RwSignal<f64>,
    /// The last target we were asked to animate toward.
    last_target: RwSignal<f64>,
}

impl AnimatedSignal {
    /// Create a new animated signal starting at `initial`, using the given
    /// `transition` strategy. The transition config is stored in the scheduler,
    /// keeping this struct `Copy`.
    pub fn new(initial: f64, transition: Transition) -> Self {
        let current = RwSignal::new(initial);
        let last_target = RwSignal::new(initial);
        let slot_id = scheduler::register(current, transition);

        Self {
            slot_id,
            current,
            last_target,
        }
    }

    /// Read the current interpolated value.
    ///
    /// Creates a reactive dependency — any Leptos effect or view binding
    /// that calls `.get()` will rerun when the animation ticks.
    #[inline]
    pub fn get(&self) -> f64 {
        self.current.get()
    }

    /// Read without creating a reactive subscription.
    #[inline]
    pub fn get_untracked(&self) -> f64 {
        self.current.get_untracked()
    }

    /// The last target that was set (what the animation is heading toward).
    #[inline]
    pub fn target(&self) -> f64 {
        self.last_target.get_untracked()
    }

    /// Begin animating toward `target`.
    ///
    /// - If no animation is in flight, starts a new one from the current value.
    /// - If an animation is already running, retargets it mid-flight:
    ///   - **Spring**: velocity is preserved (physically continuous).
    ///   - **Tween/Keyframes**: restarts from current position toward new target.
    ///
    /// If `target` matches both the current value and previous target (within ε),
    /// this is a no-op.
    pub fn set_target(&self, target: f64) {
        let current_val = self.current.get_untracked();
        let prev_target = self.last_target.get_untracked();

        // Skip if already heading there and already there
        if (target - prev_target).abs() < f64::EPSILON
            && (target - current_val).abs() < f64::EPSILON
        {
            return;
        }

        self.last_target.set(target);
        scheduler::start_animation(self.slot_id, current_val, target);
    }

    /// Immediately snap to `value` without animation.
    pub fn set_immediate(&self, value: f64) {
        self.current.set(value);
        self.last_target.set(value);
    }

    /// Access the underlying reactive signal for advanced composition.
    pub fn signal(&self) -> RwSignal<f64> {
        self.current
    }

    /// The scheduler slot ID, useful for cleanup via `scheduler::unregister`.
    pub fn slot_id(&self) -> u64 {
        self.slot_id
    }
}
