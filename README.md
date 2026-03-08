# Slik

**Slik** is a binder-first, Motion-inspired animation framework for **Leptos**.

It is built around a small numeric motion core with three public layers:

- `use_motion` for binding animation directly to typed `NodeRef`s
- `slik::html::*` for thin motion-enabled HTML component sugar
- `MotionValue` for low-level animated scalar state

v0.2 is a clean rewrite over the earlier wrapper-first MVP. This README documents
the **current** API only.

---

## Why Slik exists

Leptos already gives you precise reactive control over state and DOM ownership.
Slik focuses on the missing piece: making motion feel native to that model.

The design goals are intentionally narrow:

- Leptos-first reactivity
- binder-first architecture instead of wrapper-first architecture
- explicit numeric motion props
- spring, tween, and keyframe transitions
- per-property transition overrides
- real browser runtime on `wasm32`
- clean native checking and testing outside the browser

If you are familiar with `motion.dev`, the mental model is similar: declare a
target, and Slik animates from the current value to the new one. The difference
is that v0.2 is grounded in typed Leptos `NodeRef` binding rather than a
wrapper-centric component runtime.

---

## What Slik is

Slik is a **small, focused motion system for Leptos apps**.

It is good at:

- entry animations
- hover and press interactions
- fades, lifts, slides, scale, and rotation
- per-property motion tuning
- interruptible numeric keyframes
- animated counters, meters, and dashboards
- binding motion to real HTML and SVG nodes

## What Slik is not

v0.2 is intentionally not trying to cover the full Motion surface area.

It does **not** currently include:

- variants
- exit / presence orchestration
- gesture APIs
- layout animation
- color interpolation
- arbitrary CSS string interpolation
- SVG sugar components
- a generic catch-all motion element API

That scope is intentional. v0.2 is the architectural foundation.

---

## At a Glance

| Layer | Use it when | Main API |
|---|---|---|
| Binder | You want maximum control on a real node | `use_motion`, `MotionOptions` |
| Sugar | You want ergonomic motion-enabled HTML components | `slik::html::*` |
| Scalar | You want a single animated `f64` | `MotionValue` |

---

## Installation

If you are using Slik from this workspace or before a public crates.io release:

```toml
[dependencies]
slik = { path = "crates/slik", features = ["csr"] }
leptos = { version = "0.8", features = ["csr"] }
```

Slik mirrors Leptos target features:

- `csr`
- `hydrate`
- `ssr`

Pick the same feature set you use for `leptos`.

---

## Quick Start

The ergonomic starting point is usually the HTML sugar layer.

```rust
use leptos::prelude::*;
use slik::html::MotionDiv;
use slik::prelude::*;

#[component]
fn FadeInCard() -> impl IntoView {
    view! {
        <MotionDiv
            initial=Signal::derive(|| MotionStyle::new().opacity(0.0).y(24.0))
            animate=MotionStyle::new().opacity(1.0).y(0.0)
            transition=TransitionMap::new(
                Transition::tween(0.35, Easing::EaseOut).expect("valid tween"),
            )
            attr:style="padding:1rem; border-radius:14px; background:#eef2ff; border:1px solid #c7d2fe"
        >
            "Hello from Slik v0.2"
        </MotionDiv>
    }
}
```

This mounts with:

- `opacity: 0 -> 1`
- `translateY: 24px -> 0px`

using a tweened opacity + transform target on a real `<div>`, with no extra
wrapper inserted by Slik.

---

## The v0.2 Mental Model

Slik v0.2 is easiest to understand through four concepts.

### 1. `MotionStyle`

`MotionStyle` is the sparse target definition for supported motion properties.

```rust
MotionStyle::new()
    .opacity(1.0)
    .x(24.0)
    .scale(1.04)
```

Each property is optional. If you do not set a property, it is absent from that
style snapshot.

### 2. `Transition`

`Transition` defines how a value moves to its target.

Slik ships with three transition families:

- `Transition::spring()`
- `Transition::tween(duration_secs, easing)`
- `Transition::keyframes(keyframes, duration_secs)`

### 3. `use_motion`

`use_motion` is the canonical v0.2 API.

It binds motion to a typed Leptos `NodeRef`, owns the target node's inline
`opacity`, `transform`, and `will-change`, and keeps them synced to reactive
motion values.

### 4. `MotionValue`

`MotionValue` is the low-level scalar primitive.

Use it when you want animation without binding a full `MotionStyle` to a DOM
node.

---

## Binder-First Example

The binder is the foundation. Sugar components are layered on top of it.

```rust
use leptos::prelude::*;
use slik::prelude::*;

#[component]
fn BinderCard() -> impl IntoView {
    let node_ref = NodeRef::<leptos::html::Div>::new();
    let raised = RwSignal::new(false);

    let _motion = use_motion(
        node_ref,
        MotionOptions {
            initial: Some(Signal::derive(|| MotionStyle::new().opacity(0.0).y(20.0))),
            animate: Signal::derive(move || {
                if raised.get() {
                    MotionStyle::new().opacity(1.0).y(-6.0).scale(1.02)
                } else {
                    MotionStyle::new().opacity(1.0).y(0.0).scale(1.0)
                }
            }),
            transition: TransitionMap::new(Transition::spring_bouncy())
                .with(
                    MotionProp::Opacity,
                    Transition::tween(0.22, Easing::EaseOut).expect("valid tween"),
                )
                .into(),
            reduced_motion: MaybeProp::default(),
        },
    );

    view! {
        <div style="display:flex; gap:0.75rem; align-items:center">
            <button on:click=move |_| raised.update(|value| *value = !*value)>
                "Toggle"
            </button>
            <div
                node_ref=node_ref
                style="padding:1rem; border-radius:14px; background:#fef3c7; border:1px solid #f59e0b"
            >
                "Motion bound directly to this div"
            </div>
        </div>
    }
}
```

This is the core of the library:

- you create a real `NodeRef`
- you describe `initial` and `animate`
- Slik manages the motion runtime
- your element remains your element

No polymorphic wrapper, no hidden extra node.

### `MotionHandle`

`use_motion` returns a `MotionHandle`.

That handle exposes dense per-property `MotionValue`s:

```rust
let handle = use_motion(node_ref, options);
let x_value = handle.values.get(MotionProp::X);
```

This is useful when you want binder-driven DOM motion plus direct access to one
or more animated scalar channels.

---

## Thin HTML Sugar

The HTML sugar layer exists for ergonomics, not as a separate architecture.

Every motion HTML component is a thin wrapper over `use_motion`.

```rust
use leptos::prelude::*;
use slik::html::MotionButton;
use slik::prelude::*;

#[component]
fn HoverButton() -> impl IntoView {
    let hovered = RwSignal::new(false);

    let animate = Signal::derive(move || {
        if hovered.get() {
            MotionStyle::new().scale(1.04).y(-2.0)
        } else {
            MotionStyle::new().scale(1.0).y(0.0)
        }
    });

    view! {
        <MotionButton
            animate=animate
            transition=TransitionMap::new(Transition::spring_bouncy())
            on:mouseenter=move |_| hovered.set(true)
            on:mouseleave=move |_| hovered.set(false)
            attr:style="padding:0.8rem 1rem; border:none; border-radius:12px; background:#111827; color:white; cursor:pointer"
        >
            "Hover me"
        </MotionButton>
    }
}
```

### Important attribute forwarding note

Motion HTML components use Leptos `AttributeInterceptor`.

That means:

- event handlers such as `on:click` work normally
- plain DOM attributes should be forwarded with `attr:*`

Examples:

```rust
<MotionDiv attr:style="padding:1rem" attr:id="hero-card">
    "..."
</MotionDiv>
```

---

## Available Motion HTML Components

Current sugar coverage includes:

- Structure: `MotionArticle`, `MotionAside`, `MotionDiv`, `MotionFooter`, `MotionHeader`, `MotionMain`, `MotionNav`, `MotionSection`
- Headings: `MotionH1`, `MotionH2`, `MotionH3`, `MotionH4`, `MotionH5`, `MotionH6`
- Text: `MotionBlockquote`, `MotionCode`, `MotionEm`, `MotionP`, `MotionPre`, `MotionSmall`, `MotionSpan`, `MotionStrong`
- Lists: `MotionDl`, `MotionLi`, `MotionOl`, `MotionUl`
- Forms and controls: `MotionButton`, `MotionDetails`, `MotionFieldset`, `MotionForm`, `MotionLabel`, `MotionLegend`, `MotionSummary`, `MotionTextarea`
- Other content: `MotionA`, `MotionFigcaption`, `MotionFigure`

If you need an element that does not yet have sugar, the binder layer is already
more general than the current macro list.

---

## Supported Motion Properties

Slik currently supports these numeric motion props:

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

Transforms are composed in this fixed order:

```text
translateX -> translateY -> scale -> rotate
```

Axis-specific scale multiplies the uniform scale:

```text
effective_scale_x = scale * scale_x
effective_scale_y = scale * scale_y
```

So this:

```rust
MotionStyle::new().scale(1.2).scale_x(0.8)
```

produces:

```text
scale(0.96, 1.2)
```

---

## `initial` vs `animate`

### `initial`

`initial` seeds the starting visual state for owned properties.

### `animate`

`animate` is the live target snapshot.

Example:

```rust
<MotionDiv
    initial=Signal::derive(|| MotionStyle::new().opacity(0.0).y(20.0))
    animate=MotionStyle::new().opacity(1.0).y(0.0)
>
    "Fade + slide on mount"
</MotionDiv>
```

If `initial` is omitted, the first `animate` snapshot becomes the seed state.

That means:

- no mount transition unless `initial` differs from `animate`
- the binder starts from the first known target by default

### Property ownership semantics

Once a property appears in either `initial` or `animate`, that property becomes
**owned** by the binding.

In practice this means:

- Slik keeps writing that property's current animated value
- omitting a previously owned property from a later `animate` snapshot does not
  implicitly hand control back to other inline styles

This is deliberate. It avoids ambiguous DOM ownership after motion has claimed a
property.

---

## Reactive Targets

Reactive motion targets are a natural fit for Leptos signals and memos.

```rust
use leptos::prelude::*;
use slik::html::MotionDiv;
use slik::prelude::*;

#[component]
fn ReactiveCard() -> impl IntoView {
    let expanded = RwSignal::new(false);

    let target = Signal::derive(move || {
        if expanded.get() {
            MotionStyle::new().scale(1.04).y(-4.0)
        } else {
            MotionStyle::new().scale(1.0).y(0.0)
        }
    });

    view! {
        <div style="display:flex; gap:0.75rem; align-items:center">
            <button on:click=move |_| expanded.update(|value| *value = !*value)>
                "Toggle"
            </button>
            <MotionDiv
                animate=target
                attr:style="padding:1rem; border-radius:14px; background:#ecfccb; border:1px solid #84cc16"
            >
                "Reactive motion target"
            </MotionDiv>
        </div>
    }
}
```

When the signal changes, Slik retargets from the current sampled value.

---

## Transitions

Slik has three transition families.

### Spring

The default transition is a spring.

```rust
Transition::spring()
```

Included presets:

```rust
Transition::spring()
Transition::spring_bouncy()
Transition::spring_gentle()
Transition::spring_custom(stiffness, damping, mass)?
```

Use springs for:

- hover interactions
- press feedback
- panels and cards
- UI motion that should feel responsive and physical

`spring_custom` is validated and returns `Result<Transition, TransitionError>`.

### Tween

Tweens run for a fixed duration with cubic-bezier easing.

```rust
let tween = Transition::tween(0.35, Easing::EaseInOut)?;
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
- motion where a fixed duration matters more than spring feel

`tween` is validated and returns `Result<Transition, TransitionError>`.

### Keyframes

Keyframes are literal numeric sequences over normalized progress from `0.0` to
`1.0`.

Each keyframe has:

- an `offset`
- a `value`
- an easing used for the segment that ends at that keyframe

Helpers:

```rust
Keyframe::current(offset)
Keyframe::absolute(offset, value)
Keyframe::target(offset)
```

Example:

```rust
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

### Keyframe validation rules

A keyframe transition must satisfy all of these:

- at least 2 keyframes
- first offset must be `0.0`
- last offset must be `1.0`
- offsets must strictly increase
- absolute values must be finite
- final keyframe must be `Keyframe::target(1.0)`

That final target requirement is intentional. It keeps retargeting semantics
explicit and interruptible.

---

## Per-Property Transitions

Use `TransitionMap` when different properties should move differently.

```rust
let transitions = TransitionMap::new(Transition::spring_bouncy())
    .with(
        MotionProp::Opacity,
        Transition::tween(0.25, Easing::EaseOut).expect("valid tween"),
    )
    .with(
        MotionProp::Rotate,
        Transition::tween(0.5, Easing::Snappy).expect("valid tween"),
    );
```

This lets you do things like:

- spring translation
- tween opacity
- snap rotation differently from position

Example:

```rust
use leptos::prelude::*;
use slik::html::MotionDiv;
use slik::prelude::*;

#[component]
fn MixedTransitions() -> impl IntoView {
    let expanded = RwSignal::new(false);

    let animate = Signal::derive(move || {
        if expanded.get() {
            MotionStyle::new().scale(1.25).rotate(180.0).opacity(0.7)
        } else {
            MotionStyle::new().scale(1.0).rotate(0.0).opacity(1.0)
        }
    });

    let transitions = TransitionMap::new(Transition::spring_bouncy())
        .with(
            MotionProp::Opacity,
            Transition::tween(0.35, Easing::EaseOut).expect("valid tween"),
        )
        .with(
            MotionProp::Rotate,
            Transition::tween(0.55, Easing::Snappy).expect("valid tween"),
        );

    view! {
        <div style="display:flex; gap:0.75rem; align-items:center">
            <button on:click=move |_| expanded.update(|value| *value = !*value)>
                "Transform"
            </button>
            <MotionDiv
                animate=animate
                transition=transitions
                attr:style="width:60px; height:60px; background:#0f766e; border-radius:10px"
            />
        </div>
    }
}
```

---

## Reduced Motion

Slik exposes both system-level reduced-motion state and per-binding policy.

### Browser preference signal

```rust
let prefers_reduced_motion = use_reduced_motion();
```

This mirrors the browser's `prefers-reduced-motion` media query.

### Binding policy

`ReducedMotionConfig` controls how a specific motion binding responds:

- `ReducedMotionConfig::Auto`
- `ReducedMotionConfig::Always`
- `ReducedMotionConfig::Never`

Example:

```rust
<MotionDiv
    animate=target
    transition=TransitionMap::new(Transition::spring_bouncy())
    reduced_motion=ReducedMotionConfig::Always
    attr:style="width:48px; height:48px; background:#111827; border-radius:12px"
/>
```

`Always` forces immediate jumps. `Auto` follows the browser preference. `Never`
ignores it.

---

## Low-Level API: `MotionValue`

`MotionValue` animates a single `f64`.

Use it when:

- you want animated numeric state without DOM binding
- you want to drive text, counters, or readouts
- the binder layer is too high-level for the job

Example:

```rust
use leptos::prelude::*;
use slik::prelude::*;

#[component]
fn AnimatedCounter() -> impl IntoView {
    let count = RwSignal::new(0.0_f64);
    let display = MotionValue::new(0.0, Transition::spring());

    Effect::new(move |_| {
        display.set_target(count.get());
    });

    view! {
        <div style="display:flex; gap:0.75rem; align-items:center">
            <button on:click=move |_| count.update(|value| *value -= 1.0)>"-"</button>
            <span>{move || format!("{:.1}", display.get())}</span>
            <button on:click=move |_| count.update(|value| *value += 1.0)>"+"</button>
        </div>
    }
}
```

### `MotionValue` methods

```rust
MotionValue::new(initial, transition)
value.get()
value.get_untracked()
value.target()
value.is_animating()
value.is_animating_untracked()
value.set_target(next)
value.jump(next)
value.stop()
value.signal()
```

### Semantics

- `set_target(v)` animates from the current sampled value toward `v`
- spring retargeting preserves spring momentum
- tween and keyframe retargeting restart from the current sampled value
- `jump(v)` snaps immediately and clears active animation
- `stop()` halts animation at the current sampled value

---

## DOM Behavior

### No wrapper node

This is the most important v0.2 shift.

Slik no longer centers the library around a dedicated wrapper component. The
binder targets real nodes directly, and the HTML sugar components simply create
those nodes for you.

### HTML and SVG binding

`use_motion` supports style-capable HTML and SVG nodes.

That means:

- HTML binding is covered by both `use_motion` and `slik::html::*`
- SVG binding is available through the binder layer today
- HTML sugar is explicit; SVG sugar does not exist yet

### DOM properties owned by Slik

The binder writes:

- `opacity`
- `transform`
- `will-change`

`will-change` is only emitted for actively animating groups:

- `opacity`
- `transform`

It is not left on permanently after animation settles.

---

## Browser and Native Runtime Behavior

Slik is designed to be pleasant in a mixed Rust + wasm workflow.

In practice:

- the real animation loop runs in the browser on `wasm32`
- the browser runtime is driven by `requestAnimationFrame`
- native `cargo check`, `cargo test`, and workspace builds remain straightforward
- on non-wasm targets, animation requests snap immediately to the latest target

This is deliberate. It keeps the browser behavior correct without making native
tooling awkward.

---

## API Overview

### `slik::prelude::*`

The prelude re-exports the main working surface:

| Item | Purpose |
|---|---|
| `use_motion` | binder-first motion hook |
| `use_reduced_motion` | browser reduced-motion signal |
| `MotionOptions` | binder configuration |
| `MotionHandle` | handle returned by `use_motion` |
| `MotionValues` | dense per-property motion values |
| `ReducedMotionConfig` | per-binding reduced-motion policy |
| `MotionStyle` | sparse motion target definition |
| `MotionProp` | explicit property enum |
| `MotionValue` | low-level animated scalar |
| `Transition` | spring, tween, keyframe families |
| `TransitionMap` | default + per-property transitions |
| `TransitionError` | spring/tween validation errors |
| `Easing` | easing presets and custom bezier |
| `Keyframe` | keyframe builder |
| `KeyframeValue` | current / absolute / target keyframe values |
| `KeyframeTransition` | validated keyframe sequence |
| `KeyframeError` | keyframe validation errors |

### `slik::html`

Use this module when you want motion-enabled HTML components such as:

- `MotionDiv`
- `MotionButton`
- `MotionSection`
- `MotionMain`
- `MotionP`
- `MotionH1` through `MotionH6`

---

## Current Limitations

The current scope is intentionally lean.

Notable constraints today:

- numeric properties only
- no layout animation
- no color interpolation
- no arbitrary CSS string interpolation
- no exit / presence orchestration
- no variants
- no SVG sugar components
- sugar coverage is explicit, not open-ended

These are reasonable next-step candidates, but they are not required to use the
current v0.2 foundation effectively.

---

## Design Choices in v0.2

These choices are deliberate.

### Binder-first, sugar-second

`use_motion` is the architectural center.

The sugar layer exists because ergonomics matter, but it is intentionally a thin
layer over the binder instead of a competing runtime.

### Numeric-only motion surface

All current motion values are `f64`.

That keeps interpolation simple, predictable, and easy to test.

### Explicit property enum

Per-property overrides use `MotionProp`, not string keys.

That avoids typo-driven configuration mistakes.

### Keyframe target is explicit

The final keyframe must target `KeyframeValue::Target`.

That keeps interrupt and retarget behavior coherent.

---

## Showcase

The repository includes a working showcase in:

```text
examples/showcase/src/main.rs
```

It demonstrates:

- binder-first entry animation
- HTML sugar components
- tween targets
- keyframe pulse
- `MotionValue` counters
- per-property overrides
- reduced-motion policy
- an interactive `MotionButton`

---

## Status

Slik v0.2 is the first real public-facing foundation for the crate.

It is intentionally narrow, but the pieces that are present are meant to be
coherent:

- typed DOM binding
- thin ergonomic sugar
- explicit motion properties
- predictable numeric interpolation
- validated transition inputs
- browser-aware reduced-motion handling
- a low-level primitive when the binder layer is too high-level

If you understand this README, you understand the current shape of the crate.

---

## License and Contributions

MIT.

Contributions are currently not being actively solicited while the roadmap from
v0.2 toward v1.0 is still being shaped.

Issues, forks, and local experimentation are welcome.
