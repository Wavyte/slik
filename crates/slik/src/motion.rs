use crate::animated::AnimatedSignal;
use crate::props::{AnimProps, MotionProp};
use crate::transition::TransitionConfig;
use leptos::prelude::*;

#[derive(Clone, Copy)]
struct MotionValue {
    owned: RwSignal<bool>,
    signal: AnimatedSignal,
}

impl MotionValue {
    fn new(
        initial: Option<f64>,
        default_value: f64,
        transition: crate::transition::Transition,
    ) -> Self {
        Self {
            owned: RwSignal::new(initial.is_some()),
            signal: AnimatedSignal::new(initial.unwrap_or(default_value), transition),
        }
    }

    fn set_target(&self, value: f64) {
        if !self.owned.get_untracked() {
            self.owned.set(true);
        }
        self.signal.set_target(value);
    }

    fn is_owned(&self) -> bool {
        self.owned.get()
    }
}

#[component]
pub fn Motion(
    #[prop(optional)] initial: Option<AnimProps>,
    #[prop(into)] animate: Signal<AnimProps>,
    #[prop(optional)] transition: Option<TransitionConfig>,
    #[prop(into, optional)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    let transition = transition.unwrap_or_default();
    let first_animate = animate.get_untracked();
    let initial = initial.unwrap_or_else(|| first_animate.clone());

    let opacity = MotionValue::new(
        initial.value_for(MotionProp::Opacity),
        MotionProp::Opacity.default_value(),
        transition.for_prop(MotionProp::Opacity),
    );
    let x = MotionValue::new(
        initial.value_for(MotionProp::X),
        MotionProp::X.default_value(),
        transition.for_prop(MotionProp::X),
    );
    let y = MotionValue::new(
        initial.value_for(MotionProp::Y),
        MotionProp::Y.default_value(),
        transition.for_prop(MotionProp::Y),
    );
    let scale = MotionValue::new(
        initial.value_for(MotionProp::Scale),
        MotionProp::Scale.default_value(),
        transition.for_prop(MotionProp::Scale),
    );
    let scale_x = MotionValue::new(
        initial.value_for(MotionProp::ScaleX),
        MotionProp::ScaleX.default_value(),
        transition.for_prop(MotionProp::ScaleX),
    );
    let scale_y = MotionValue::new(
        initial.value_for(MotionProp::ScaleY),
        MotionProp::ScaleY.default_value(),
        transition.for_prop(MotionProp::ScaleY),
    );
    let rotate = MotionValue::new(
        initial.value_for(MotionProp::Rotate),
        MotionProp::Rotate.default_value(),
        transition.for_prop(MotionProp::Rotate),
    );

    Effect::new(move |_| {
        let props = animate.get();

        if let Some(value) = props.opacity {
            opacity.set_target(value);
        }
        if let Some(value) = props.x {
            x.set_target(value);
        }
        if let Some(value) = props.y {
            y.set_target(value);
        }
        if let Some(value) = props.scale {
            scale.set_target(value);
        }
        if let Some(value) = props.scale_x {
            scale_x.set_target(value);
        }
        if let Some(value) = props.scale_y {
            scale_y.set_target(value);
        }
        if let Some(value) = props.rotate {
            rotate.set_target(value);
        }
    });

    let style = move || {
        let mut styles = Vec::with_capacity(3);

        if opacity.is_owned() {
            styles.push(format!("opacity:{};", opacity.signal.get()));
        }

        let mut transforms = Vec::with_capacity(4);

        if x.is_owned() {
            transforms.push(format!("translateX({}px)", x.signal.get()));
        }
        if y.is_owned() {
            transforms.push(format!("translateY({}px)", y.signal.get()));
        }
        if scale.is_owned() || scale_x.is_owned() || scale_y.is_owned() {
            let effective_x = scale.signal.get()
                * if scale_x.is_owned() {
                    scale_x.signal.get()
                } else {
                    1.0
                };
            let effective_y = scale.signal.get()
                * if scale_y.is_owned() {
                    scale_y.signal.get()
                } else {
                    1.0
                };
            transforms.push(format!("scale({effective_x}, {effective_y})"));
        }
        if rotate.is_owned() {
            transforms.push(format!("rotate({}deg)", rotate.signal.get()));
        }

        if !transforms.is_empty() {
            styles.push(format!("transform:{};", transforms.join(" ")));
        }

        let mut will_change = Vec::with_capacity(2);
        if opacity.is_owned() {
            will_change.push("opacity");
        }
        if !transforms.is_empty() {
            will_change.push("transform");
        }
        if !will_change.is_empty() {
            styles.push(format!("will-change:{};", will_change.join(", ")));
        }

        styles.join(" ")
    };

    view! {
        <div class=move || class.get().unwrap_or_default() style=style>
            {children()}
        </div>
    }
}
