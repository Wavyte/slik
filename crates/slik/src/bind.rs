use crate::motion_value::MotionValue;
use crate::style::{
    compose_dom_style, mask_for_style, owns_prop, prop_bit, MotionProp, MotionStyle,
};
use crate::transition::TransitionMap;
use leptos::prelude::*;
use leptos::tachys::html::element::ElementType;
use wasm_bindgen::JsCast;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ReducedMotionConfig {
    #[default]
    Auto,
    Always,
    Never,
}

#[derive(Clone, Copy)]
pub struct MotionValues {
    values: [MotionValue; MotionProp::COUNT],
}

impl MotionValues {
    pub(crate) fn new(values: [MotionValue; MotionProp::COUNT]) -> Self {
        Self { values }
    }

    pub fn get(&self, prop: MotionProp) -> MotionValue {
        self.values[prop.index()]
    }
}

#[derive(Clone, Copy)]
pub struct MotionHandle {
    pub values: MotionValues,
}

#[derive(Clone, Copy)]
pub struct MotionOptions {
    pub initial: Option<Signal<MotionStyle>>,
    pub animate: Signal<MotionStyle>,
    pub transition: MaybeProp<TransitionMap>,
    pub reduced_motion: MaybeProp<ReducedMotionConfig>,
}

pub fn use_motion<E>(node_ref: NodeRef<E>, options: MotionOptions) -> MotionHandle
where
    E: ElementType + 'static,
    E::Output: JsCast + Clone + 'static,
{
    let first_animate = options.animate.get_untracked();
    let seeded = options
        .initial
        .map(|initial| initial.get_untracked())
        .unwrap_or_else(|| first_animate.clone());
    let owned_mask = RwSignal::new(mask_for_style(&seeded));
    let initial_transition = options.transition.get().unwrap_or_default();

    let values = std::array::from_fn(|index| {
        let prop = MotionProp::ALL[index];
        MotionValue::new(
            seeded.get(prop).unwrap_or_else(|| prop.default_value()),
            initial_transition.for_prop(prop),
        )
    });
    let values = MotionValues::new(values);
    let system_reduced_motion = use_reduced_motion();

    Effect::new(move |_| {
        let target = options.animate.get();
        let transitions = options.transition.get().unwrap_or_default();
        let reduced = match options.reduced_motion.get().unwrap_or_default() {
            ReducedMotionConfig::Auto => system_reduced_motion.get(),
            ReducedMotionConfig::Always => true,
            ReducedMotionConfig::Never => false,
        };

        for prop in MotionProp::ALL {
            let value = values.get(prop);
            value.set_transition(transitions.for_prop(prop));

            if let Some(target_value) = target.get(prop) {
                if !owns_prop(owned_mask.get_untracked(), prop) {
                    owned_mask.update(|mask| *mask |= prop_bit(prop));
                }

                if reduced {
                    value.set_target_immediate(target_value);
                } else {
                    value.set_target(target_value);
                }
            }
        }
    });

    Effect::new(move |_| {
        let Some(node) = node_ref.get() else {
            return;
        };
        let element = node.unchecked_into::<web_sys::HtmlElement>();
        let mask = owned_mask.get();
        let mut current = [0.0; MotionProp::COUNT];

        for prop in MotionProp::ALL {
            current[prop.index()] = if owns_prop(mask, prop) {
                values.get(prop).get()
            } else {
                prop.default_value()
            };
        }

        let composed = compose_dom_style(mask, &current);
        let style = element.style();

        patch_style_property(&style, "opacity", composed.opacity.as_deref());
        patch_style_property(&style, "transform", composed.transform.as_deref());
        patch_style_property(&style, "will-change", composed.will_change.as_deref());
    });

    MotionHandle { values }
}

pub fn use_reduced_motion() -> Signal<bool> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        return RwSignal::new(false).into();
    }

    #[cfg(target_arch = "wasm32")]
    {
        let reduced = RwSignal::new(browser_prefers_reduced_motion());

        if let Some(window) = web_sys::window() {
            if let Ok(Some(media_query)) = window.match_media("(prefers-reduced-motion: reduce)") {
                let signal = reduced;
                let tracked_query = media_query.clone();
                let callback =
                    wasm_bindgen::closure::Closure::wrap(Box::new(move |_event: web_sys::Event| {
                        signal.set(tracked_query.matches());
                    })
                        as Box<dyn FnMut(web_sys::Event)>);

                media_query.set_onchange(Some(callback.as_ref().unchecked_ref()));
                callback.forget();
            }
        }

        reduced.into()
    }
}

#[cfg(target_arch = "wasm32")]
fn browser_prefers_reduced_motion() -> bool {
    web_sys::window()
        .and_then(|window| window.match_media("(prefers-reduced-motion: reduce)").ok())
        .flatten()
        .map(|query| query.matches())
        .unwrap_or(false)
}

fn patch_style_property(style: &web_sys::CssStyleDeclaration, name: &str, value: Option<&str>) {
    match value {
        Some(value) => {
            let _ = style.set_property(name, value);
        }
        None => {
            let _ = style.remove_property(name);
        }
    }
}
