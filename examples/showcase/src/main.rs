use leptos::prelude::*;
use slick::prelude::*;

// ---------------------------------------------------------------------------
// 1. Mount animation — fade + slide
// ---------------------------------------------------------------------------

#[component]
fn EntryAnimation() -> impl IntoView {
    view! {
        <Motion
            initial=AnimProps::new().opacity(0.0).y(30.0)
            animate=AnimProps::new().opacity(1.0).y(0.0)
            transition=TransitionConfig::new(Transition::spring())
                .with("opacity", Transition::tween(0.5, Easing::EaseOut))
        >
            <p>"I fade in and slide up on mount (spring Y + tween opacity)"</p>
        </Motion>
    }
}

// ---------------------------------------------------------------------------
// 2. Reactive hover — spring scale + lift
// ---------------------------------------------------------------------------

#[component]
fn HoverScale() -> impl IntoView {
    let hovered = RwSignal::new(false);

    let target = Memo::new(move |_| {
        if hovered.get() {
            AnimProps::new().scale(1.06).y(-3.0)
        } else {
            AnimProps::new().scale(1.0).y(0.0)
        }
    });

    view! {
        <Motion animate=target>
            <div
                style="padding:1.5rem; background:#e8e8e8; border-radius:10px; cursor:pointer; text-align:center"
                on:mouseenter=move |_| hovered.set(true)
                on:mouseleave=move |_| hovered.set(false)
            >
                "Hover me — spring scale + lift"
            </div>
        </Motion>
    }
}

// ---------------------------------------------------------------------------
// 3. Bezier tween toggle
// ---------------------------------------------------------------------------

#[component]
fn TweenFade() -> impl IntoView {
    let visible = RwSignal::new(true);

    let target = Memo::new(move |_| {
        if visible.get() {
            AnimProps::new().opacity(1.0).x(0.0)
        } else {
            AnimProps::new().opacity(0.0).x(40.0)
        }
    });

    view! {
        <button on:click=move |_| visible.update(|v| *v = !*v)>
            "Toggle"
        </button>
        <Motion
            animate=target
            transition=TransitionConfig::new(
                Transition::tween(0.35, Easing::EaseInOut)
            )
        >
            <p style="margin-top:0.5rem">"Bezier tween: opacity + translateX"</p>
        </Motion>
    }
}

// ---------------------------------------------------------------------------
// 4. Keyframe bounce
// ---------------------------------------------------------------------------

#[component]
fn KeyframeBounce() -> impl IntoView {
    let active = RwSignal::new(false);

    let target = Memo::new(move |_| {
        if active.get() {
            AnimProps::new().y(-50.0)
        } else {
            AnimProps::new().y(0.0)
        }
    });

    // Keyframes describe the *shape* of the animation curve through value-space.
    // offset 0→0, 0.4→1 (overshoot), 0.7→0.85 (settle back), 1.0→1 (final)
    let transition = TransitionConfig::new(Transition::keyframes(
        vec![
            Keyframe { offset: 0.0, value: 0.0,  easing: Easing::Linear },
            Keyframe { offset: 0.4, value: 1.15, easing: Easing::EaseOut },
            Keyframe { offset: 0.7, value: 0.9,  easing: Easing::EaseInOut },
            Keyframe { offset: 1.0, value: 1.0,  easing: Easing::EaseOut },
        ],
        0.6,
    ));

    view! {
        <button on:click=move |_| active.update(|v| *v = !*v)>
            "Bounce"
        </button>
        <Motion animate=target transition=transition>
            <div style="width:50px; height:50px; background:coral; border-radius:50%; margin-top:0.5rem" />
        </Motion>
    }
}

// ---------------------------------------------------------------------------
// 5. Low-level AnimatedSignal counter
// ---------------------------------------------------------------------------

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
            <span style="font-size:2rem; font-variant-numeric:tabular-nums; min-width:6ch; text-align:center">
                {move || format!("{:.1}", display.get())}
            </span>
            <button on:click=move |_| count.update(|n| *n += 1.0)>"+"</button>
        </div>
    }
}

// ---------------------------------------------------------------------------
// 6. Per-property transition overrides
// ---------------------------------------------------------------------------

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
        .with("opacity", Transition::tween(0.4, Easing::EaseOut))
        .with("rotate", Transition::tween(0.6, Easing::Snappy));

    view! {
        <button on:click=move |_| expanded.update(|v| *v = !*v)>
            "Transform"
        </button>
        <Motion animate=target transition=transition>
            <div style="width:60px; height:60px; background:#6366f1; border-radius:8px; margin-top:0.5rem" />
        </Motion>
    }
}

// ---------------------------------------------------------------------------
// Root
// ---------------------------------------------------------------------------

#[component]
fn App() -> impl IntoView {
    view! {
        <main style="max-width:640px; margin:2rem auto; font-family:system-ui,-apple-system,sans-serif; padding:0 1rem">
            <h1 style="margin-bottom:2rem">"Slick — Animation Showcase"</h1>

            <Section title="1. Entry Animation (initial → animate)">
                <EntryAnimation />
            </Section>
            <Section title="2. Hover Scale (reactive spring)">
                <HoverScale />
            </Section>
            <Section title="3. Tween Fade (cubic bézier)">
                <TweenFade />
            </Section>
            <Section title="4. Keyframe Bounce">
                <KeyframeBounce />
            </Section>
            <Section title="5. Animated Counter (low-level AnimatedSignal)">
                <AnimatedCounter />
            </Section>
            <Section title="6. Per-Property Overrides">
                <MixedTransitions />
            </Section>
        </main>
    }
}

#[component]
fn Section(title: &'static str, children: Children) -> impl IntoView {
    view! {
        <section style="margin-bottom:2rem; padding:1rem; border:1px solid #ddd; border-radius:8px">
            <h3 style="margin:0 0 0.75rem">{title}</h3>
            {children()}
        </section>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}
