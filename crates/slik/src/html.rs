//! Motion-enabled HTML component sugar.
//!
//! These components are thin wrappers over [`crate::bind::use_motion`]. They do
//! not add a separate runtime or wrapper element. Attributes are forwarded via
//! Leptos' `AttributeInterceptor`, so element attributes should be passed with
//! `attr:*` when needed.

use crate::bind::{use_motion, MotionOptions, ReducedMotionConfig};
use crate::style::MotionStyle;
use crate::transition::TransitionMap;
use leptos::attribute_interceptor::AttributeInterceptor;
use leptos::prelude::*;
use leptos::tachys::html::element::ElementType;
use wasm_bindgen::{JsCast, JsValue};

fn prepare_motion_node<E>(options: MotionOptions) -> NodeRef<E>
where
    E: ElementType + 'static,
    E::Output: AsRef<JsValue> + JsCast + Clone + 'static,
{
    let node_ref = NodeRef::<E>::new();
    let _ = use_motion(node_ref, options);
    node_ref
}

macro_rules! motion_html_elements {
    ($($name:ident => ($element:ty, $tag:ident)),+ $(,)?) => {
        $(
            /// Motion-enabled HTML component generated over
            /// [`crate::bind::use_motion`].
            ///
            /// Motion-specific props are handled directly by the component, and
            /// additional DOM attributes can be forwarded with `attr:*`.
            #[component]
            pub fn $name(
                /// Initial owned style snapshot.
                #[prop(optional)] initial: Option<Signal<MotionStyle>>,
                /// Reactive target style snapshot.
                #[prop(into)] animate: Signal<MotionStyle>,
                /// Default and per-property transition configuration.
                #[prop(into, optional)] transition: MaybeProp<TransitionMap>,
                /// Reduced-motion policy for this motion binding.
                #[prop(into, optional)] reduced_motion: MaybeProp<ReducedMotionConfig>,
                /// Child content rendered inside the element.
                #[prop(optional)] children: Option<ChildrenFn>,
            ) -> impl IntoView {
                let node_ref = prepare_motion_node::<$element>(MotionOptions {
                    initial,
                    animate,
                    transition,
                    reduced_motion,
                });

                view! {
                    <AttributeInterceptor let:attrs>
                        <$tag node_ref={node_ref} {..attrs}>
                            {children.as_ref().map(|children| children())}
                        </$tag>
                    </AttributeInterceptor>
                }
            }
        )+
    };
}

motion_html_elements! {
    MotionA => (leptos::html::A, a),
    MotionAside => (leptos::html::Aside, aside),
    MotionBlockquote => (leptos::html::Blockquote, blockquote),
    MotionCode => (leptos::html::Code, code),
    MotionDetails => (leptos::html::Details, details),
    MotionDiv => (leptos::html::Div, div),
    MotionDl => (leptos::html::Dl, dl),
    MotionEm => (leptos::html::Em, em),
    MotionFigcaption => (leptos::html::Figcaption, figcaption),
    MotionFigure => (leptos::html::Figure, figure),
    MotionFooter => (leptos::html::Footer, footer),
    MotionForm => (leptos::html::Form, form),
    MotionFieldset => (leptos::html::Fieldset, fieldset),
    MotionHeader => (leptos::html::Header, header),
    MotionButton => (leptos::html::Button, button),
    MotionArticle => (leptos::html::Article, article),
    MotionH1 => (leptos::html::H1, h1),
    MotionH2 => (leptos::html::H2, h2),
    MotionH3 => (leptos::html::H3, h3),
    MotionH4 => (leptos::html::H4, h4),
    MotionH5 => (leptos::html::H5, h5),
    MotionH6 => (leptos::html::H6, h6),
    MotionLabel => (leptos::html::Label, label),
    MotionLegend => (leptos::html::Legend, legend),
    MotionLi => (leptos::html::Li, li),
    MotionMain => (leptos::html::Main, main),
    MotionNav => (leptos::html::Nav, nav),
    MotionOl => (leptos::html::Ol, ol),
    MotionP => (leptos::html::P, p),
    MotionPre => (leptos::html::Pre, pre),
    MotionSection => (leptos::html::Section, section),
    MotionSmall => (leptos::html::Small, small),
    MotionSpan => (leptos::html::Span, span),
    MotionStrong => (leptos::html::Strong, strong),
    MotionSummary => (leptos::html::Summary, summary),
    MotionTextarea => (leptos::html::Textarea, textarea),
    MotionUl => (leptos::html::Ul, ul)
}
