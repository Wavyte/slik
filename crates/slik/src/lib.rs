//! # Slik
//!
//! A Motion-inspired animation framework for Leptos.
//!
//! Springs, tweens, and keyframes are exposed as reactive primitives and a
//! declarative `<Motion>` component.

pub mod animated;
pub mod bezier;
pub mod driver;
pub mod easing;
pub mod motion;
pub mod props;
pub mod scheduler;
pub mod transition;

pub mod prelude {
    pub use crate::animated::AnimatedSignal;
    pub use crate::driver::{
        Keyframe, KeyframeError, KeyframeTransition, KeyframeValue, SpringDriver, TweenDriver,
    };
    pub use crate::easing::Easing;
    pub use crate::motion::Motion;
    pub use crate::props::{AnimProps, MotionProp};
    pub use crate::transition::{Transition, TransitionConfig};
}
