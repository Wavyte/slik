use crate::animated::AnimatedSignal;
use crate::props::AnimProps;
use crate::transition::{Transition, TransitionConfig};
use leptos::prelude::*;

/// A declarative animated container, inspired by `motion.div` from motion.dev.
///
/// Wraps children in a `<div>` and smoothly animates CSS properties whenever
/// the `animate` prop changes. Supports spring, bezier tween, and keyframe
/// transitions with per-property overrides.
///
/// # Mount animation
///
/// If `initial` is provided, the element starts at that state and animates to
/// `animate` on mount. If omitted, the element appears directly at `animate`
/// with no entry animation.
///
/// # Reactive updates
///
/// Pass an `RwSignal<AnimProps>` for `animate` to re-animate whenever the
/// signal changes. A plain `AnimProps` is auto-wrapped (no updates after mount).
///
/// # Example
///
/// ```ignore
/// <Motion
///     initial=AnimProps::new().opacity(0.0).y(20.0)
///     animate=AnimProps::new().opacity(1.0).y(0.0)
///     transition=TransitionConfig::new(Transition::spring())
///         .with("opacity", Transition::tween(0.4, Easing::EaseOut))
/// >
///     <p>"Hello, animated world!"</p>
/// </Motion>
/// ```
#[component]
pub fn Motion(
    /// Starting state. If present, the component animates `initial → animate`
    /// on mount. If absent, no mount animation.
    #[prop(optional)]
    initial: Option<AnimProps>,
    /// Target state. Accepts `AnimProps` (static), `ReadSignal`, `RwSignal`, or `Memo`.
    #[prop(into)]
    animate: Signal<AnimProps>,
    /// Animation config. Supports a default strategy plus per-property overrides.
    #[prop(optional)]
    transition: Option<TransitionConfig>,
    /// CSS classes on the wrapper `<div>`.
    #[prop(optional)]
    class: &'static str,
    children: Children,
) -> impl IntoView {
    let tc = transition.unwrap_or_default();

    // If initial is provided, start from initial values.
    // Otherwise start from animate values (no mount animation).
    let first_animate = animate.get_untracked();
    let init = initial.unwrap_or_else(|| first_animate.clone());

    // --- Per-property AnimatedSignals ---

    let opacity = AnimatedSignal::new(init.get_or_default("opacity"), tc.for_prop("opacity"));
    let x = AnimatedSignal::new(init.get_or_default("x"), tc.for_prop("x"));
    let y = AnimatedSignal::new(init.get_or_default("y"), tc.for_prop("y"));
    let sc = AnimatedSignal::new(init.get_or_default("scale"), tc.for_prop("scale"));
    let sx = AnimatedSignal::new(init.get_or_default("scale_x"), tc.for_prop("scale_x"));
    let sy = AnimatedSignal::new(init.get_or_default("scale_y"), tc.for_prop("scale_y"));
    let rot = AnimatedSignal::new(init.get_or_default("rotate"), tc.for_prop("rotate"));

    // --- Reactive animation driver ---
    // Runs immediately on mount (triggering initial→animate) and re-runs
    // whenever the animate signal changes.

    Effect::new(move |_| {
        let props = animate.get();
        if let Some(v) = props.opacity { opacity.set_target(v); }
        if let Some(v) = props.x { x.set_target(v); }
        if let Some(v) = props.y { y.set_target(v); }
        if let Some(v) = props.scale { sc.set_target(v); }
        if let Some(v) = props.scale_x { sx.set_target(v); }
        if let Some(v) = props.scale_y { sy.set_target(v); }
        if let Some(v) = props.rotate { rot.set_target(v); }
    });

    // --- Reactive style bindings ---

    let opacity_str = move || format!("{}", opacity.get());

    let transform_str = move || {
        let tx = x.get();
        let ty = y.get();
        let s = sc.get();
        let sxv = sx.get();
        let syv = sy.get();
        let r = rot.get();

        // Use per-axis scale when either deviates from the uniform value
        let scale_part = if (sxv - 1.0).abs() > 1e-6 || (syv - 1.0).abs() > 1e-6 {
            format!("scaleX({sxv}) scaleY({syv})")
        } else {
            format!("scale({s})")
        };

        format!("translateX({tx}px) translateY({ty}px) {scale_part} rotate({r}deg)")
    };

    view! {
        <div
            class=class
            style:opacity=opacity_str
            style:transform=transform_str
            style:will-change="transform, opacity"
        >
            {children()}
        </div>
    }
}
