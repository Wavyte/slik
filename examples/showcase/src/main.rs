use leptos::prelude::*;
use slik::html::{MotionButton, MotionDiv, MotionH1, MotionH3, MotionMain, MotionP, MotionSection};
use slik::prelude::*;

#[component]
fn BinderEntry() -> impl IntoView {
    let node_ref = NodeRef::<leptos::html::Div>::new();
    let _motion = use_motion(
        node_ref,
        MotionOptions {
            initial: Some(Signal::derive(|| MotionStyle::new().opacity(0.0).y(30.0))),
            animate: Signal::derive(|| MotionStyle::new().opacity(1.0).y(0.0)),
            transition: TransitionMap::new(Transition::spring())
                .with(
                    MotionProp::Opacity,
                    Transition::tween(0.45, Easing::EaseOut),
                )
                .into(),
            reduced_motion: MaybeProp::default(),
        },
    );

    view! {
        <div
            node_ref=node_ref
            style="padding:1rem; background:#eef2ff; border:1px solid #c7d2fe; border-radius:14px"
        >
            "Binder-first motion on a plain div"
        </div>
    }
}

#[component]
fn SugarHover() -> impl IntoView {
    let expanded = RwSignal::new(false);
    let target = Signal::derive(move || {
        if expanded.get() {
            MotionStyle::new().scale(1.04).y(-4.0)
        } else {
            MotionStyle::new().scale(1.0).y(0.0)
        }
    });

    view! {
        <div style="display:flex; gap:0.75rem; align-items:center; flex-wrap:wrap">
            <button on:click=move |_| expanded.update(|value| *value = !*value)>"Toggle"</button>
            <MotionDiv
                animate=target
                attr:style="padding:1rem; background:#fef3c7; border:1px solid #f59e0b; border-radius:14px"
            >
                "Thin sugar over the same binder runtime"
            </MotionDiv>
        </div>
    }
}

#[component]
fn TweenFade() -> impl IntoView {
    let visible = RwSignal::new(true);
    let target = Signal::derive(move || {
        if visible.get() {
            MotionStyle::new().opacity(1.0).x(0.0)
        } else {
            MotionStyle::new().opacity(0.15).x(40.0)
        }
    });

    view! {
        <div style="display:flex; gap:0.75rem; align-items:center; flex-wrap:wrap">
            <button on:click=move |_| visible.update(|v| *v = !*v)>"Toggle"</button>
            <MotionP
                animate=target
                transition=TransitionMap::new(Transition::tween(0.35, Easing::EaseInOut))
                attr:style="margin:0"
            >
                "Bezier tween: opacity plus translateX"
            </MotionP>
        </div>
    }
}

#[component]
fn KeyframePulse() -> impl IntoView {
    let active = RwSignal::new(false);
    let target = Signal::derive(move || {
        if active.get() {
            MotionStyle::new().scale(1.18)
        } else {
            MotionStyle::new().scale(1.0)
        }
    });

    let transition = TransitionMap::new(
        Transition::keyframes(
            vec![
                Keyframe::current(0.0),
                Keyframe::absolute(0.35, 1.24).ease(Easing::EaseOut),
                Keyframe::absolute(0.7, 1.06).ease(Easing::EaseInOut),
                Keyframe::target(1.0).ease(Easing::EaseOut),
            ],
            0.55,
        )
        .expect("valid keyframe pulse"),
    );

    view! {
        <div style="display:flex; gap:0.75rem; align-items:center; flex-wrap:wrap">
            <button on:click=move |_| active.update(|value| *value = !*value)>"Pulse"</button>
            <MotionDiv
                animate=target
                transition=transition
                attr:style="width:56px; height:56px; background:coral; border-radius:16px"
            />
        </div>
    }
}

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
            <span style="font-size:2rem; font-variant-numeric:tabular-nums; min-width:6ch; text-align:center">
                {move || format!("{:.1}", display.get())}
            </span>
            <button on:click=move |_| count.update(|value| *value += 1.0)>"+"</button>
        </div>
    }
}

#[component]
fn MixedTransitions() -> impl IntoView {
    let expanded = RwSignal::new(false);
    let target = Signal::derive(move || {
        if expanded.get() {
            MotionStyle::new().scale(1.25).rotate(180.0).opacity(0.7)
        } else {
            MotionStyle::new().scale(1.0).rotate(0.0).opacity(1.0)
        }
    });

    let transitions = TransitionMap::new(Transition::spring_bouncy())
        .with(
            MotionProp::Opacity,
            Transition::tween(0.35, Easing::EaseOut),
        )
        .with(MotionProp::Rotate, Transition::tween(0.55, Easing::Snappy));

    view! {
        <div style="display:flex; gap:0.75rem; align-items:center; flex-wrap:wrap">
            <button on:click=move |_| expanded.update(|value| *value = !*value)>"Transform"</button>
            <MotionDiv
                animate=target
                transition=transitions
                attr:style="width:60px; height:60px; background:#0f766e; border-radius:10px"
            />
        </div>
    }
}

#[component]
fn ReducedMotionExample() -> impl IntoView {
    let toggled = RwSignal::new(false);
    let target = Signal::derive(move || {
        if toggled.get() {
            MotionStyle::new().x(120.0).rotate(25.0)
        } else {
            MotionStyle::new().x(0.0).rotate(0.0)
        }
    });

    view! {
        <div style="display:flex; gap:0.75rem; align-items:center; flex-wrap:wrap">
            <button on:click=move |_| toggled.update(|value| *value = !*value)>"Jump"</button>
            <MotionDiv
                animate=target
                reduced_motion=ReducedMotionConfig::Always
                transition=TransitionMap::new(Transition::spring_bouncy())
                attr:style="width:48px; height:48px; background:#111827; border-radius:12px"
            />
            <span style="font-size:0.95rem; color:#4b5563">
                "ReducedMotionConfig::Always forces immediate updates"
            </span>
        </div>
    }
}

#[component]
fn Section(title: &'static str, children: ChildrenFn) -> impl IntoView {
    view! {
        <MotionSection
            animate=MotionStyle::new().opacity(1.0)
            attr:style="margin-bottom:1.5rem; padding:1rem; border:1px solid #d1d5db; border-radius:16px; background:white"
        >
            <MotionH3 animate=MotionStyle::new().opacity(1.0) attr:style="margin:0 0 0.75rem">
                {title}
            </MotionH3>
            {children()}
        </MotionSection>
    }
}

#[component]
fn App() -> impl IntoView {
    view! {
        <MotionMain
            animate=MotionStyle::new().opacity(1.0)
            attr:style="max-width:760px; margin:2rem auto; padding:0 1rem; font-family:system-ui,-apple-system,sans-serif; color:#111827"
        >
            <MotionH1 animate=MotionStyle::new().opacity(1.0) attr:style="margin-bottom:2rem">
                "Slik v0.2 Showcase"
            </MotionH1>

            <Section title="1. Binder-first entry animation">
                <BinderEntry />
            </Section>
            <Section title="2. Thin sugar over binder">
                <SugarHover />
            </Section>
            <Section title="3. Tween motion style target">
                <TweenFade />
            </Section>
            <Section title="4. Keyframe pulse">
                <KeyframePulse />
            </Section>
            <Section title="5. MotionValue low-level scalar">
                <AnimatedCounter />
            </Section>
            <Section title="6. Per-property overrides">
                <MixedTransitions />
            </Section>
            <Section title="7. Reduced motion policy">
                <ReducedMotionExample />
            </Section>

            <MotionButton
                animate=MotionStyle::new().opacity(1.0)
                attr:style="padding:0.75rem 1rem; border:none; border-radius:12px; background:#2563eb; color:white; cursor:pointer"
            >
                "MotionButton forwards attributes to a real button"
            </MotionButton>
        </MotionMain>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}
