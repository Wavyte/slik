//! Binder-first motion APIs.
//!
//! This module is the core v0.2 public surface. It binds a typed Leptos
//! [`NodeRef`](leptos::prelude::NodeRef) to one
//! [`MotionStyle`](crate::style::MotionStyle) target signal and keeps the
//! underlying DOM node's inline `opacity`, `transform`, and
//! `will-change` properties in sync with animated values.

/// Returns the browser's `prefers-reduced-motion` setting as a reactive signal.
///
/// On non-wasm targets this always resolves to `false`.
pub use crate::reduced_motion::use_reduced_motion;

use crate::dom_target::style_for_node;
use crate::motion_value::MotionValue;
use crate::style::{
    compose_dom_style, mask_for_style, owns_prop, prop_bit, MotionProp, MotionStyle,
};
use crate::transition::TransitionMap;
use leptos::prelude::*;
use leptos::tachys::html::element::ElementType;
use wasm_bindgen::{JsCast, JsValue};

/// Controls how [`use_motion`] responds to reduced-motion preferences.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ReducedMotionConfig {
    /// Respect the browser's `prefers-reduced-motion` media query.
    #[default]
    Auto,
    /// Always disable animation and snap directly to the latest target.
    Always,
    /// Always animate, ignoring the browser preference.
    Never,
}

/// Dense storage for the [`MotionValue`] instances created by [`use_motion`].
///
/// Each property in [`MotionProp`] gets one scalar motion value, regardless of
/// whether the property is currently owned by the bound style.
#[derive(Clone, Copy)]
pub struct MotionValues {
    values: [MotionValue; MotionProp::COUNT],
}

impl MotionValues {
    pub(crate) fn new(values: [MotionValue; MotionProp::COUNT]) -> Self {
        Self { values }
    }

    /// Returns the scalar motion value for a specific property slot.
    pub fn get(&self, prop: MotionProp) -> MotionValue {
        self.values[prop.index()]
    }
}

/// Handle returned by [`use_motion`].
///
/// This currently exposes the underlying per-property motion values so callers
/// can observe or imperatively control them when needed.
#[derive(Clone, Copy)]
pub struct MotionHandle {
    /// Dense motion values allocated for the binding.
    pub values: MotionValues,
}

/// Options used to configure a motion binding.
#[derive(Clone, Copy)]
pub struct MotionOptions {
    /// Initial owned style snapshot.
    ///
    /// When omitted, the first `animate` snapshot seeds the binding and no mount
    /// animation occurs.
    pub initial: Option<Signal<MotionStyle>>,
    /// Reactive target style snapshot.
    pub animate: Signal<MotionStyle>,
    /// Default and per-property transition configuration.
    pub transition: MaybeProp<TransitionMap>,
    /// Reduced-motion policy for this binding.
    pub reduced_motion: MaybeProp<ReducedMotionConfig>,
}

/// Binds motion styles to a typed Leptos node reference.
///
/// The binder owns inline `opacity`, `transform`, and `will-change` on the bound
/// node. It currently supports HTML and SVG elements with writable inline styles.
///
/// `initial` seeds owned properties. If it is omitted, the first `animate`
/// snapshot becomes the initial value and the binding starts in-place.
///
/// Properties become owned the first time they appear in either `initial` or
/// `animate`. Once owned, the binder continues to write that property's current
/// animated value until the binding is destroyed.
pub fn use_motion<E>(node_ref: NodeRef<E>, options: MotionOptions) -> MotionHandle
where
    E: ElementType + 'static,
    E::Output: AsRef<JsValue> + JsCast + Clone + 'static,
{
    let first_animate = options.animate.get_untracked();
    let seeded = options
        .initial
        .map(|initial| initial.get_untracked())
        .unwrap_or_else(|| first_animate.clone());
    let owned_mask = RwSignal::new(mask_for_style(&seeded));
    let initial_transition = options.transition.get().unwrap_or_default();

    let values = std::array::from_fn(|index| {
        let prop = MotionProp::ALL[index];
        MotionValue::new(
            seeded.get(prop).unwrap_or_else(|| prop.default_value()),
            initial_transition.for_prop(prop),
        )
    });
    let values = MotionValues::new(values);
    let system_reduced_motion = use_reduced_motion();

    Effect::new(move |_| {
        let target = options.animate.get();
        let transitions = options.transition.get().unwrap_or_default();
        let reduced = match options.reduced_motion.get().unwrap_or_default() {
            ReducedMotionConfig::Auto => system_reduced_motion.get(),
            ReducedMotionConfig::Always => true,
            ReducedMotionConfig::Never => false,
        };

        for prop in MotionProp::ALL {
            let value = values.get(prop);
            value.set_transition(transitions.for_prop(prop));

            if let Some(target_value) = target.get(prop) {
                if !owns_prop(owned_mask.get_untracked(), prop) {
                    owned_mask.update(|mask| *mask |= prop_bit(prop));
                }

                if reduced {
                    value.set_target_immediate(target_value);
                } else {
                    value.set_target(target_value);
                }
            }
        }
    });

    Effect::new(move |_| {
        let Some(node) = node_ref.get() else {
            return;
        };
        let mask = owned_mask.get();
        let mut active_mask = 0;
        let mut current = [0.0; MotionProp::COUNT];

        for prop in MotionProp::ALL {
            current[prop.index()] = if owns_prop(mask, prop) {
                let value = values.get(prop);
                if value.is_animating() {
                    active_mask |= prop_bit(prop);
                }
                value.get()
            } else {
                prop.default_value()
            };
        }

        let Some(style) = style_for_node(&node) else {
            return;
        };
        let composed = compose_dom_style(mask, active_mask, &current);

        patch_style_property(&style, "opacity", composed.opacity.as_deref());
        patch_style_property(&style, "transform", composed.transform.as_deref());
        patch_style_property(&style, "will-change", composed.will_change.as_deref());
    });

    MotionHandle { values }
}
fn patch_style_property(style: &web_sys::CssStyleDeclaration, name: &str, value: Option<&str>) {
    match value {
        Some(value) => {
            let _ = style.set_property(name, value);
        }
        None => {
            let _ = style.remove_property(name);
        }
    }
}
