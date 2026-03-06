//! # Slick
//!
//! A `motion.dev`-inspired animation framework for [Leptos](https://leptos.dev).
//!
//! Provides physics-based springs, cubic-bézier tweens, and multi-segment
//! keyframe animations as first-class reactive primitives.
//!
//! ## Quick Start
//!
//! ```ignore
//! use leptos::prelude::*;
//! use slick::prelude::*;
//!
//! #[component]
//! fn FadeIn() -> impl IntoView {
//!     view! {
//!         <Motion
//!             initial=AnimProps::new().opacity(0.0).y(20.0)
//!             animate=AnimProps::new().opacity(1.0).y(0.0)
//!         >
//!             <h1>"Hello!"</h1>
//!         </Motion>
//!     }
//! }
//! ```
//!
//! ## Architecture
//!
//! ```text
//! L4  <Motion> component              ← declarative API
//! L3  AnimatedSignal                  ← reactive bridge
//! L2  Scheduler (singleton rAF loop)  ← ticks all active animations
//! L1  Driver trait                     ← Spring, Tween, Keyframe impls
//! L0  CubicBezier solver              ← pure math
//! ```
//!
//! Each layer depends only on the layer below. L0–L1 are pure Rust with no DOM
//! or framework dependencies. L2–L3 bridge into Leptos's reactive system.
//! L4 is the public component API.

pub mod bezier;
pub mod driver;
pub mod easing;
pub mod scheduler;
pub mod animated;
pub mod props;
pub mod transition;
pub mod motion;

/// Convenient re-exports of the most commonly used types.
pub mod prelude {
    pub use crate::animated::AnimatedSignal;
    pub use crate::driver::{Keyframe, KeyframeDriver, SpringDriver, TweenDriver};
    pub use crate::easing::Easing;
    pub use crate::motion::Motion;
    pub use crate::props::AnimProps;
    pub use crate::transition::{Transition, TransitionConfig};
}
