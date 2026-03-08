#![deny(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]
//! # Slik
//!
//! `slik` is a Motion-inspired animation framework for Leptos.
//!
//! v0.2 is built around three layers:
//! - [`MotionValue`](motion_value::MotionValue) for low-level scalar animation.
//! - [`use_motion`](bind::use_motion) for binding motion styles to typed Leptos
//!   [`NodeRef`](leptos::prelude::NodeRef) values.
//! - [`html`] for thin motion-enabled HTML components generated on top of
//!   the same binder.
//!
//! The crate deliberately keeps the animation property surface small and explicit:
//! opacity plus transform-related properties such as translation, scale, and
//! rotation.
//!
//! For day-to-day use, import the [`prelude`] module.

/// Cubic-bezier utilities used by easing-based transitions.
pub mod bezier;
/// Binder-first motion APIs for attaching animations to typed DOM nodes.
pub mod bind;
mod dom_target;
/// Keyframe definitions and driver-level transition types.
pub mod driver;
/// Easing presets and custom cubic-bezier support.
pub mod easing;
/// Thin motion-enabled HTML component sugar built on [`bind::use_motion`].
pub mod html;
/// Low-level scalar motion primitive.
pub mod motion_value;
mod reduced_motion;
mod runtime;
/// Typed motion style properties and style composition helpers.
pub mod style;
/// Transition configuration types for motion bindings and values.
pub mod transition;

/// Common imports for `slik` applications.
///
/// This re-exports the main `slik` API surface.
pub mod prelude {
    pub use crate::bind::{
        use_motion, use_reduced_motion, MotionHandle, MotionOptions, MotionValues,
        ReducedMotionConfig,
    };
    pub use crate::driver::{Keyframe, KeyframeError, KeyframeTransition, KeyframeValue};
    pub use crate::easing::Easing;
    pub use crate::motion_value::MotionValue;
    pub use crate::style::{MotionProp, MotionStyle};
    pub use crate::transition::{Transition, TransitionError, TransitionMap};
}
