# Slik

**Slik** is a Motion-inspired animation library for **Leptos**.

It gives you two layers:

- a declarative `<Motion>` component for animating UI props
- a low-level `AnimatedSignal` primitive for animating arbitrary numeric state

The v0.1 MVP is deliberately small and sharp:

- numeric animation only
- Leptos-first reactive API
- springs, tweens, and keyframes
- per-property transition overrides
- browser runtime driven by `requestAnimationFrame`
- safe lifecycle cleanup through Leptos owners

If you are familiar with `motion.dev`, the mental model is similar: you declare a target, and Slik animates from the current value to the new one. The keyframe model also follows Motion-style timing with normalized offsets between `0.0` and `1.0`, and supports “start from current value” semantics for interruptible animations.

---

## What Slik is

Slik is a **small animation runtime for Leptos apps**.

It is built for the most common UI motion needs:

- fade in / fade out
- slide and lift animations
- scale and rotate interactions
- springy reactive transitions
- keyframed micro-interactions
- animated numeric state for counters, meters, and dashboards

## What Slik is not

v0.1 is **not** trying to cover the full Motion surface area.

It does **not** currently include:

- variants
- exit / presence orchestration
- gesture APIs
- SVG-specific motion components
- color interpolation
- layout animation
- animation of arbitrary CSS strings

That is intentional. v0.1 focuses on a tight, reliable numeric core.

---

## Core mental model

Slik has four core concepts.

### 1. `AnimProps`

This is the target animation state for a `Motion` wrapper.

```rust
AnimProps::new()
    .opacity(1.0)
    .x(24.0)
    .scale(1.05)
```

Each field is optional. A property is only animated if you set it.

### 2. `Transition`

This defines **how** a property moves to its target.

Slik ships with three transition families:

- `Transition::spring()`
- `Transition::tween(duration, easing)`
- `Transition::keyframes(keyframes, duration)`

### 3. `Motion`

`<Motion>` is the declarative component.

You give it:

- an optional `initial`
- an `animate` target
- an optional `transition`
- children

When `animate` changes, Slik animates any owned properties to their new targets.

### 4. `AnimatedSignal`

This is the lower-level primitive.

It is useful when you want an animated numeric value without routing everything through a wrapper component.

---

## Supported properties

v0.1 supports these numeric props:

| Property | Meaning | Units |
|---|---|---|
| `opacity` | CSS opacity | unitless |
| `x` | horizontal translation | px |
| `y` | vertical translation | px |
| `scale` | uniform scale | unitless |
| `scale_x` | x-axis scale multiplier | unitless |
| `scale_y` | y-axis scale multiplier | unitless |
| `rotate` | rotation | deg |

### Transform composition order

Slik composes transforms in this order:

```text
translateX -> translateY -> scale -> rotate
```

Axis scale multiplies the uniform scale:

```text
effective_scale_x = scale * scale_x
effective_scale_y = scale * scale_y
```

So this:

```rust
AnimProps::new().scale(1.2).scale_x(0.8)
```

produces:

```text
effective x scale = 0.96
effective y scale = 1.2
```

---

## Installation

Until Slik is published, use it as a workspace member or path dependency.

```toml
[dependencies]
slik = { path = "../../crates/slik" }
```

Once published, this becomes the usual crates.io dependency.

---

## Quick start

```rust
use leptos::prelude::*;
use slik::prelude::*;

#[component]
fn FadeInCard() -> impl IntoView {
    view! {
        <Motion
            initial=AnimProps::new().opacity(0.0).y(24.0)
            animate=AnimProps::new().opacity(1.0).y(0.0)
        >
            <div class="card">
                "Hello from Slik"
            </div>
        </Motion>
    }
}
```

This mounts with:

- `opacity: 0 -> 1`
- `translateY: 24px -> 0px`

using the default spring.

---

## Using reactive targets

Leptos component props can accept reactive sources directly, and `Signal<T>` is the preferred modern prop wrapper for values that may be static or reactive. Slik uses that pattern for `animate`, so you can pass a plain `AnimProps`, a signal, or a memo. Optional props like `class` are represented with `MaybeProp<T>`.

```rust
use leptos::prelude::*;
use slik::prelude::*;

#[component]
fn HoverCard() -> impl IntoView {
    let hovered = RwSignal::new(false);

    let target = Memo::new(move |_| {
        if hovered.get() {
            AnimProps::new().scale(1.05).y(-4.0)
        } else {
            AnimProps::new().scale(1.0).y(0.0)
        }
    });

    view! {
        <Motion animate=target>
            <div
                style="padding:1rem; border-radius:12px; background:#ececec; cursor:pointer"
                on:mouseenter=move |_| hovered.set(true)
                on:mouseleave=move |_| hovered.set(false)
            >
                "Hover me"
            </div>
        </Motion>
    }
}
```

When `hovered` changes, the memo changes, and Slik animates to the new target.

---

## `initial` vs `animate`

### `initial`

`initial` is the starting visual state.

### `animate`

`animate` is the live target.

Example:

```rust
<Motion
    initial=AnimProps::new().opacity(0.0).y(20.0)
    animate=AnimProps::new().opacity(1.0).y(0.0)
>
    <p>"Fade + slide on mount"</p>
</Motion>
```

If `initial` is omitted, Slik uses the first `animate` value as the starting state.

That means:

- no unwanted jump on mount
- controlled properties begin from the first known target

---

## Transitions

### Spring

The default transition is a spring.

```rust
let t = Transition::spring();
```

Included presets:

```rust
Transition::spring()
Transition::spring_bouncy()
Transition::spring_gentle()
Transition::spring_custom(stiffness, damping, mass)
```

Use springs for:

- hover interactions
- panel expansion
- UI elements that should feel responsive and physical

### Example

```rust
<Motion
    animate=AnimProps::new().scale(1.08).y(-2.0)
    transition=Transition::spring().into()
>
    <button>"Springy button"</button>
</Motion>
```

---

### Tween

Tweens run for a fixed duration with a cubic Bézier easing.

```rust
let t = Transition::tween(0.35, Easing::EaseInOut);
```

Available easings:

```rust
Easing::Linear
Easing::Ease
Easing::EaseIn
Easing::EaseOut
Easing::EaseInOut
Easing::Snappy
Easing::Custom(x1, y1, x2, y2)
```

Use tweens for:

- fades
- deterministic micro-interactions
- deliberate UI choreography where fixed duration matters more than physical feel

### Example

```rust
<Motion
    animate=AnimProps::new().opacity(1.0).x(0.0)
    transition=Transition::tween(0.4, Easing::EaseOut).into()
>
    <div>"Tweened content"</div>
</Motion>
```

---

## Per-property transitions

You can override the default transition per property using `TransitionConfig`.

```rust
use slik::prelude::*;

let transition = TransitionConfig::new(Transition::spring())
    .with(MotionProp::Opacity, Transition::tween(0.25, Easing::EaseOut))
    .with(MotionProp::Rotate, Transition::tween(0.5, Easing::Snappy));
```

This lets you do things like:

- spring position
- tween opacity
- snappier rotation

### Example

```rust
#[component]
fn MixedTransitions() -> impl IntoView {
    let expanded = RwSignal::new(false);

    let target = Memo::new(move |_| {
        if expanded.get() {
            AnimProps::new().scale(1.3).rotate(180.0).opacity(0.7)
        } else {
            AnimProps::new().scale(1.0).rotate(0.0).opacity(1.0)
        }
    });

    let transition = TransitionConfig::new(Transition::spring_bouncy())
        .with(MotionProp::Opacity, Transition::tween(0.4, Easing::EaseOut))
        .with(MotionProp::Rotate, Transition::tween(0.6, Easing::Snappy));

    view! {
        <button on:click=move |_| expanded.update(|v| *v = !*v)>
            "Transform"
        </button>
        <Motion animate=target transition=transition>
            <div style="width:60px; height:60px; background:#6366f1; border-radius:8px; margin-top:0.5rem" />
        </Motion>
    }
}
```

---

## Keyframes

Slik keyframes are **literal numeric keyframes**.

They are defined as a sequence of keyframes over normalized progress from `0.0` to `1.0`.

Each keyframe has:

- an `offset`
- a `value`
- an easing applied for the segment ending at that keyframe

### Keyframe value kinds

```rust
KeyframeValue::Current
KeyframeValue::Absolute(f64)
KeyframeValue::Target
```

You will normally create them using helpers:

```rust
Keyframe::current(offset)
Keyframe::absolute(offset, value)
Keyframe::target(offset)
```

### Validation rules

A keyframe transition must satisfy all of these:

- at least 2 keyframes
- first offset must be `0.0`
- last offset must be `1.0`
- offsets must strictly increase
- absolute values must be finite
- final keyframe must be `Keyframe::target(1.0)`

That final `target` requirement is intentional: it makes retargeting semantics explicit and keeps the sequence interruptible.

### Motion-style current value semantics

Motion supports keyframe timing through a normalized `times` array and allows sequences to begin from the current value. Slik mirrors that idea through `offset` and `Keyframe::current(0.0)`, while making the final target explicit with `Keyframe::target(1.0)`.

### Example: pulse

```rust
use slik::prelude::*;

let pulse = Transition::keyframes(
    vec![
        Keyframe::current(0.0),
        Keyframe::absolute(0.35, 1.24).ease(Easing::EaseOut),
        Keyframe::absolute(0.7, 1.06).ease(Easing::EaseInOut),
        Keyframe::target(1.0).ease(Easing::EaseOut),
    ],
    0.55,
)?;
```

This means:

- start from the live current value
- overshoot to `1.24`
- settle toward `1.06`
- finish at the requested target

### Example: closed-loop style sequence

```rust
let bounce = Transition::keyframes(
    vec![
        Keyframe::current(0.0),
        Keyframe::absolute(0.3, 100.0).ease(Easing::EaseOut),
        Keyframe::absolute(0.7, -12.0).ease(Easing::EaseInOut),
        Keyframe::target(1.0).ease(Easing::EaseOut),
    ],
    0.6,
)?;
```

This is the Slik equivalent of a Motion-style multi-step sequence with custom timing.

---

## Low-level API: `AnimatedSignal`

`AnimatedSignal` animates a single `f64` value.

Use it when you want animation without a wrapper component.

### Example: animated counter

```rust
use leptos::prelude::*;
use slik::prelude::*;

#[component]
fn AnimatedCounter() -> impl IntoView {
    let count = RwSignal::new(0.0_f64);
    let display = AnimatedSignal::new(0.0, Transition::spring());

    Effect::new(move |_| {
        display.set_target(count.get());
    });

    view! {
        <div style="display:flex; gap:0.5rem; align-items:center">
            <button on:click=move |_| count.update(|n| *n -= 1.0)>"-"</button>
            <span>{move || format!("{:.1}", display.get())}</span>
            <button on:click=move |_| count.update(|n| *n += 1.0)>"+"</button>
        </div>
    }
}
```

### Methods

```rust
AnimatedSignal::new(initial, transition)
animated.get()
animated.get_untracked()
animated.target()
animated.set_target(value)
animated.set_immediate(value)
animated.signal()
```

### Semantics

- `set_target(v)` animates from the current value to `v`
- setting the same target again is a no-op
- `set_immediate(v)` stops any running animation and snaps immediately to `v`
- cleanup is automatic inside a Leptos owner via `on_cleanup`

---

## Styling and DOM behavior

`<Motion>` currently renders a wrapper `<div>` around its children.

```rust
view! {
    <Motion animate=AnimProps::new().opacity(1.0)>
        <p>"Hello"</p>
    </Motion>
}
```

becomes conceptually:

```html
<div style="...animated styles...">
  <p>Hello</p>
</div>
```

### `class`

You can pass a static or reactive class value:

```rust
<Motion
    class="card"
    animate=AnimProps::new().opacity(1.0)
>
    <div>"content"</div>
</Motion>
```

### `will-change`

When Slik owns `opacity` and/or transform props, it emits `will-change` for those properties.

This helps hint browser optimization for common UI motion paths.

---

## SSR and native `cargo check` behavior

Leptos effects are intended to synchronize reactive state with the outside world, and browser-only work is commonly wrapped in `Effect::new()` because effects run on the client. Slik follows that pattern for wiring motion updates.

In practice, that means:

- the codebase can be checked and tested natively
- the actual animation loop runs in the browser on `wasm32`
- non-wasm animation starts by snapping immediately to the target instead of trying to run a browser scheduler

That split is deliberate. It keeps the crate ergonomic for workspace builds while preserving correct browser runtime behavior.

---

## Complete example

```rust
use leptos::prelude::*;
use slik::prelude::*;

#[component]
fn ExampleCard() -> impl IntoView {
    let hovered = RwSignal::new(false);

    let animate = Memo::new(move |_| {
        if hovered.get() {
            AnimProps::new()
                .opacity(0.92)
                .y(-4.0)
                .scale(1.03)
                .rotate(1.5)
        } else {
            AnimProps::new()
                .opacity(1.0)
                .y(0.0)
                .scale(1.0)
                .rotate(0.0)
        }
    });

    let transition = TransitionConfig::new(Transition::spring())
        .with(MotionProp::Opacity, Transition::tween(0.18, Easing::EaseOut))
        .with(MotionProp::Rotate, Transition::tween(0.24, Easing::Snappy));

    view! {
        <Motion
            class="demo-card"
            animate=animate
            transition=transition
        >
            <button
                style="padding:1rem 1.25rem; border:none; border-radius:14px; background:#111827; color:white; cursor:pointer"
                on:mouseenter=move |_| hovered.set(true)
                on:mouseleave=move |_| hovered.set(false)
            >
                "Hover me"
            </button>
        </Motion>
    }
}
```

---

## API overview

### Re-exported through `slik::prelude::*`

| Item | Purpose |
|---|---|
| `Motion` | declarative animated wrapper component |
| `AnimProps` | target values for supported motion props |
| `MotionProp` | property enum for transition overrides |
| `Transition` | spring, tween, and keyframe transitions |
| `TransitionConfig` | default + per-property transitions |
| `AnimatedSignal` | low-level animated numeric signal |
| `Easing` | tween / keyframe easing presets |
| `Keyframe` | keyframe builder type |
| `KeyframeValue` | current / absolute / target keyframe values |
| `KeyframeError` | validation errors for keyframes |

---

## Design choices in v0.1

These choices are intentional:

### Wrapper component, not polymorphic elements

`Motion` always renders a `<div>` in v0.1.

That keeps the internal model simple while the numeric animation core stabilizes.

### Numeric-only interpolation

All supported values are `f64`.

This avoids the complexity of string parsing and mixed-unit interpolation in the MVP.

### Explicit property enum for overrides

`TransitionConfig::with(...)` uses `MotionProp`, not string keys.

That avoids typo-driven silent failures.

### Keyframe target is explicit

The final keyframe must be `target`.

This makes mid-flight retargeting consistent and predictable.

---

## Limitations and roadmap candidates

Potential next steps after v0.1:

- polymorphic motion elements
- `MotionSpan`, `MotionButton`, or generic element rendering
- variants and orchestration
- exit / presence support
- colors and CSS variable animation
- transform origin
- SVG support
- layout animation
- gesture APIs

But none of those are required to understand or use the current MVP.

---

## Minimal checklist for using Slik well

- use `Motion` when animating UI wrappers
- use `AnimatedSignal` for numeric state
- use springs for interactive motion
- use tweens when exact duration matters
- use keyframes for multi-step motion
- start keyframes with `Keyframe::current(0.0)` when you want interruptible animations
- always end keyframes with `Keyframe::target(1.0)`
- use `TransitionConfig` when different properties need different motion styles

---

## Example imports

```rust
use leptos::prelude::*;
use slik::prelude::*;
```

That is enough for essentially all v0.1 usage.

---

## Status

Slik v0.1 is a focused MVP animation core for Leptos.

It is intentionally narrow, but the pieces that are present are designed to be coherent:

- reactive targets
- lifecycle-safe cleanup
- predictable numeric interpolation
- Motion-like declarative ergonomics
- a low-level primitive when the component layer is too high-level

If you read this README end to end, you should have enough context to use the current MVP comfortably.

## License and Contributions

MIT!

Feel free to fork and play around with the code!

Contributions are currently not encouraged because the team is cooking up the future roadmap on how the project should shape up.

Once the roadmap is clear and we have clarity on how to go from v0.1 to v1.0, we will be happy to take contributions!