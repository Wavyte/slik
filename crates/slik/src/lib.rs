//! # Slik
//!
//! A Motion-inspired animation framework for Leptos.
//!
//! v0.2 centers on a scalar motion runtime, typed `NodeRef` binding for HTML and
//! SVG elements, and thin component sugar over the binder.

pub mod bezier;
pub mod bind;
mod dom_target;
pub mod driver;
pub mod easing;
pub mod html;
pub mod motion_value;
mod reduced_motion;
pub mod runtime;
pub mod style;
pub mod transition;

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
