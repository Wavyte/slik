# Slick

A `motion.dev`-inspired animation framework for [Leptos](https://leptos.dev).

Physics-based springs, cubic-bézier tweens, and multi-segment keyframe animations — all as first-class reactive primitives.

## Architecture

```
L4  <Motion> component              ← declarative API
L3  AnimatedSignal (Copy)           ← reactive bridge
L2  Scheduler (singleton rAF loop)  ← ticks all active drivers
L1  trait Driver                     ← Spring | Tween | Keyframe
L0  CubicBezier solver              ← Newton-Raphson, pure math
```

## Quick Start

```rust
use leptos::prelude::*;
use slick::prelude::*;

// Entry animation — fade in and slide up
#[component]
fn FadeIn() -> impl IntoView {
    view! {
        <Motion
            initial=AnimProps::new().opacity(0.0).y(20.0)
            animate=AnimProps::new().opacity(1.0).y(0.0)
        >
            <h1>"Hello!"</h1>
        </Motion>
    }
}
```

## Transition Strategies

### Spring (default)

No fixed duration. Physics determines settling time. Velocity is preserved on interruption.

```rust
// Critically damped (snappy, no overshoot)
Transition::spring()

// Bouncy
Transition::spring_bouncy()

// Custom
Transition::spring_custom(200.0, 20.0, 1.0)
```

### Tween (cubic bézier)

Duration-based with a single easing curve.

```rust
Transition::tween(0.3, Easing::EaseOut)
Transition::tween(0.5, Easing::Snappy)     // subtle overshoot
Transition::tween(1.0, Easing::Custom(0.68, -0.55, 0.265, 1.55))
```

### Keyframes

Multi-segment with per-segment easing.

```rust
Transition::keyframes(
    vec![
        Keyframe { offset: 0.0, value: 0.0,   easing: Easing::Linear },
        Keyframe { offset: 0.3, value: 1.2,   easing: Easing::EaseOut },
        Keyframe { offset: 0.7, value: 0.8,   easing: Easing::EaseInOut },
        Keyframe { offset: 1.0, value: 1.0,   easing: Easing::EaseOut },
    ],
    0.6, // total duration in seconds
)
```

## Per-Property Overrides

```rust
<Motion
    initial=AnimProps::new().opacity(0.0).x(-40.0)
    animate=AnimProps::new().opacity(1.0).x(0.0)
    transition=TransitionConfig::new(Transition::spring())
        .with("opacity", Transition::tween(0.4, Easing::EaseOut))
        .with("x", Transition::spring_bouncy())
>
    <div>"Each property animates differently"</div>
</Motion>
```

## Reactive Animations

```rust
#[component]
fn HoverCard() -> impl IntoView {
    let hovered = RwSignal::new(false);
    let animate = Memo::new(move |_| {
        if hovered.get() {
            AnimProps::new().scale(1.05).y(-4.0)
        } else {
            AnimProps::new().scale(1.0).y(0.0)
        }
    });

    view! {
        <Motion
            animate=animate
            transition=TransitionConfig::new(Transition::spring())
        >
            <div
                on:mouseenter=move |_| hovered.set(true)
                on:mouseleave=move |_| hovered.set(false)
            >
                "Hover me"
            </div>
        </Motion>
    }
}
```

## Low-Level: AnimatedSignal

Use `AnimatedSignal` directly for fine-grained control:

```rust
#[component]
fn Counter() -> impl IntoView {
    let count = RwSignal::new(0.0);
    let display = AnimatedSignal::new(0.0, Transition::spring());

    Effect::new(move |_| {
        display.set_target(count.get());
    });

    view! {
        <button on:click=move |_| count.update(|n| *n += 1.0)>"+1"</button>
        <span>{move || format!("{:.1}", display.get())}</span>
    }
}
```

## Easing Presets

| Name       | Curve                   | Feel                    |
|:-----------|:------------------------|:------------------------|
| `Linear`   | `(0, 0, 1, 1)`         | Constant speed          |
| `Ease`     | `(.25, .1, .25, 1)`    | Gentle start & end      |
| `EaseIn`   | `(.42, 0, 1, 1)`       | Slow start, fast end    |
| `EaseOut`  | `(0, 0, .58, 1)`       | Fast start, slow end    |
| `EaseInOut`| `(.42, 0, .58, 1)`     | Symmetric S-curve       |
| `Snappy`   | `(.16, 1, .3, 1)`      | Quick with overshoot    |
| `Custom`   | `(x1, y1, x2, y2)`     | Any cubic bézier        |

## License

MIT
