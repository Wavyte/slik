use crate::bind::{use_motion, MotionOptions, ReducedMotionConfig};
use crate::style::MotionStyle;
use crate::transition::TransitionMap;
use leptos::attribute_interceptor::AttributeInterceptor;
use leptos::prelude::*;

#[component]
pub fn MotionDiv(
    #[prop(optional)] initial: Option<Signal<MotionStyle>>,
    #[prop(into)] animate: Signal<MotionStyle>,
    #[prop(into, optional)] transition: MaybeProp<TransitionMap>,
    #[prop(into, optional)] reduced_motion: MaybeProp<ReducedMotionConfig>,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let node_ref = NodeRef::<leptos::html::Div>::new();
    let _motion = use_motion(
        node_ref,
        MotionOptions {
            initial,
            animate,
            transition,
            reduced_motion,
        },
    );

    view! {
        <AttributeInterceptor let:attrs>
            <div node_ref={node_ref} {..attrs}>{children.as_ref().map(|children| children())}</div>
        </AttributeInterceptor>
    }
}

#[component]
pub fn MotionSpan(
    #[prop(optional)] initial: Option<Signal<MotionStyle>>,
    #[prop(into)] animate: Signal<MotionStyle>,
    #[prop(into, optional)] transition: MaybeProp<TransitionMap>,
    #[prop(into, optional)] reduced_motion: MaybeProp<ReducedMotionConfig>,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let node_ref = NodeRef::<leptos::html::Span>::new();
    let _motion = use_motion(
        node_ref,
        MotionOptions {
            initial,
            animate,
            transition,
            reduced_motion,
        },
    );

    view! {
        <AttributeInterceptor let:attrs>
            <span node_ref={node_ref} {..attrs}>{children.as_ref().map(|children| children())}</span>
        </AttributeInterceptor>
    }
}

#[component]
pub fn MotionP(
    #[prop(optional)] initial: Option<Signal<MotionStyle>>,
    #[prop(into)] animate: Signal<MotionStyle>,
    #[prop(into, optional)] transition: MaybeProp<TransitionMap>,
    #[prop(into, optional)] reduced_motion: MaybeProp<ReducedMotionConfig>,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let node_ref = NodeRef::<leptos::html::P>::new();
    let _motion = use_motion(
        node_ref,
        MotionOptions {
            initial,
            animate,
            transition,
            reduced_motion,
        },
    );

    view! {
        <AttributeInterceptor let:attrs>
            <p node_ref={node_ref} {..attrs}>{children.as_ref().map(|children| children())}</p>
        </AttributeInterceptor>
    }
}

#[component]
pub fn MotionButton(
    #[prop(optional)] initial: Option<Signal<MotionStyle>>,
    #[prop(into)] animate: Signal<MotionStyle>,
    #[prop(into, optional)] transition: MaybeProp<TransitionMap>,
    #[prop(into, optional)] reduced_motion: MaybeProp<ReducedMotionConfig>,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let node_ref = NodeRef::<leptos::html::Button>::new();
    let _motion = use_motion(
        node_ref,
        MotionOptions {
            initial,
            animate,
            transition,
            reduced_motion,
        },
    );

    view! {
        <AttributeInterceptor let:attrs>
            <button node_ref={node_ref} {..attrs}>{children.as_ref().map(|children| children())}</button>
        </AttributeInterceptor>
    }
}

#[component]
pub fn MotionSection(
    #[prop(optional)] initial: Option<Signal<MotionStyle>>,
    #[prop(into)] animate: Signal<MotionStyle>,
    #[prop(into, optional)] transition: MaybeProp<TransitionMap>,
    #[prop(into, optional)] reduced_motion: MaybeProp<ReducedMotionConfig>,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let node_ref = NodeRef::<leptos::html::Section>::new();
    let _motion = use_motion(
        node_ref,
        MotionOptions {
            initial,
            animate,
            transition,
            reduced_motion,
        },
    );

    view! {
        <AttributeInterceptor let:attrs>
            <section node_ref={node_ref} {..attrs}>{children.as_ref().map(|children| children())}</section>
        </AttributeInterceptor>
    }
}

#[component]
pub fn MotionMain(
    #[prop(optional)] initial: Option<Signal<MotionStyle>>,
    #[prop(into)] animate: Signal<MotionStyle>,
    #[prop(into, optional)] transition: MaybeProp<TransitionMap>,
    #[prop(into, optional)] reduced_motion: MaybeProp<ReducedMotionConfig>,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let node_ref = NodeRef::<leptos::html::Main>::new();
    let _motion = use_motion(
        node_ref,
        MotionOptions {
            initial,
            animate,
            transition,
            reduced_motion,
        },
    );

    view! {
        <AttributeInterceptor let:attrs>
            <main node_ref={node_ref} {..attrs}>{children.as_ref().map(|children| children())}</main>
        </AttributeInterceptor>
    }
}

#[component]
pub fn MotionArticle(
    #[prop(optional)] initial: Option<Signal<MotionStyle>>,
    #[prop(into)] animate: Signal<MotionStyle>,
    #[prop(into, optional)] transition: MaybeProp<TransitionMap>,
    #[prop(into, optional)] reduced_motion: MaybeProp<ReducedMotionConfig>,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let node_ref = NodeRef::<leptos::html::Article>::new();
    let _motion = use_motion(
        node_ref,
        MotionOptions {
            initial,
            animate,
            transition,
            reduced_motion,
        },
    );

    view! {
        <AttributeInterceptor let:attrs>
            <article node_ref={node_ref} {..attrs}>{children.as_ref().map(|children| children())}</article>
        </AttributeInterceptor>
    }
}

#[component]
pub fn MotionH1(
    #[prop(optional)] initial: Option<Signal<MotionStyle>>,
    #[prop(into)] animate: Signal<MotionStyle>,
    #[prop(into, optional)] transition: MaybeProp<TransitionMap>,
    #[prop(into, optional)] reduced_motion: MaybeProp<ReducedMotionConfig>,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let node_ref = NodeRef::<leptos::html::H1>::new();
    let _motion = use_motion(
        node_ref,
        MotionOptions {
            initial,
            animate,
            transition,
            reduced_motion,
        },
    );

    view! {
        <AttributeInterceptor let:attrs>
            <h1 node_ref={node_ref} {..attrs}>{children.as_ref().map(|children| children())}</h1>
        </AttributeInterceptor>
    }
}

#[component]
pub fn MotionH3(
    #[prop(optional)] initial: Option<Signal<MotionStyle>>,
    #[prop(into)] animate: Signal<MotionStyle>,
    #[prop(into, optional)] transition: MaybeProp<TransitionMap>,
    #[prop(into, optional)] reduced_motion: MaybeProp<ReducedMotionConfig>,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let node_ref = NodeRef::<leptos::html::H3>::new();
    let _motion = use_motion(
        node_ref,
        MotionOptions {
            initial,
            animate,
            transition,
            reduced_motion,
        },
    );

    view! {
        <AttributeInterceptor let:attrs>
            <h3 node_ref={node_ref} {..attrs}>{children.as_ref().map(|children| children())}</h3>
        </AttributeInterceptor>
    }
}
