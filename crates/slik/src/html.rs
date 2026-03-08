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
            #[component]
            pub fn $name(
                #[prop(optional)] initial: Option<Signal<MotionStyle>>,
                #[prop(into)] animate: Signal<MotionStyle>,
                #[prop(into, optional)] transition: MaybeProp<TransitionMap>,
                #[prop(into, optional)] reduced_motion: MaybeProp<ReducedMotionConfig>,
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
