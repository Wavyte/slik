#![cfg(target_arch = "wasm32")]

use leptos::prelude::*;
use slik::html::MotionDiv;
use slik::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

async fn next_frame() {
    let promise = js_sys::Promise::new(&mut |resolve, _reject| {
        let callback = wasm_bindgen::closure::Closure::wrap(Box::new(move |_timestamp: f64| {
            let _ = resolve.call0(&JsValue::NULL);
        }) as Box<dyn FnMut(f64)>);

        web_sys::window()
            .unwrap()
            .request_animation_frame(callback.as_ref().unchecked_ref())
            .unwrap();
        callback.forget();
    });

    JsFuture::from(promise).await.unwrap();
}

async fn next_frames(count: usize) {
    for _ in 0..count {
        next_frame().await;
    }
}

fn reset_body() -> web_sys::HtmlElement {
    let document = web_sys::window().unwrap().document().unwrap();
    let body = document.body().unwrap();
    body.set_inner_html("");
    body
}

fn div_by_id(body: &web_sys::HtmlElement, id: &str) -> web_sys::HtmlElement {
    body.query_selector(&format!("#{id}"))
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::HtmlElement>()
        .unwrap()
}

#[wasm_bindgen_test(async)]
async fn binder_applies_dom_styles_with_reduced_motion() {
    #[component]
    fn Harness() -> impl IntoView {
        let node_ref = NodeRef::<leptos::html::Div>::new();
        let _motion = use_motion(
            node_ref,
            MotionOptions {
                initial: None,
                animate: Signal::derive(|| MotionStyle::new().opacity(0.5).x(12.0)),
                transition: TransitionMap::new(Transition::spring()).into(),
                reduced_motion: ReducedMotionConfig::Always.into(),
            },
        );

        view! { <div id="binder-box" node_ref={node_ref}></div> }
    }

    let body = reset_body();
    leptos::mount::mount_to_body(Harness);
    next_frame().await;

    let element = div_by_id(&body, "binder-box");

    assert_eq!(
        element.style().get_property_value("opacity").unwrap(),
        "0.5"
    );
    assert_eq!(
        element.style().get_property_value("transform").unwrap(),
        "translateX(12px)"
    );
}

#[wasm_bindgen_test(async)]
async fn sugar_forwards_attributes_without_wrapper() {
    #[component]
    fn Harness() -> impl IntoView {
        view! {
            <MotionDiv
                animate=MotionStyle::new().opacity(1.0)
                reduced_motion=ReducedMotionConfig::Always
                attr:id="sugar-box"
                attr:data-kind="demo"
            ></MotionDiv>
        }
    }

    let body = reset_body();
    leptos::mount::mount_to_body(Harness);
    next_frame().await;

    let first = body.first_element_child().unwrap();
    assert_eq!(first.tag_name(), "DIV");
    assert_eq!(first.get_attribute("id").as_deref(), Some("sugar-box"));
    assert_eq!(first.get_attribute("data-kind").as_deref(), Some("demo"));
    assert_eq!(body.children().length(), 1);
}

#[wasm_bindgen_test(async)]
async fn same_target_retarget_applies_new_transition() {
    #[component]
    fn Harness() -> impl IntoView {
        let transition = RwSignal::new(TransitionMap::new(
            Transition::tween(1.0, Easing::Linear).unwrap(),
        ));

        let transition_for_timeout = transition;
        let callback = Closure::once_into_js(move || {
            transition_for_timeout.set(TransitionMap::new(
                Transition::tween(0.0, Easing::Linear).unwrap(),
            ));
        });

        web_sys::window()
            .unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                callback.as_ref().unchecked_ref(),
                16,
            )
            .unwrap();

        let node_ref = NodeRef::<leptos::html::Div>::new();
        let _motion = use_motion(
            node_ref,
            MotionOptions {
                initial: Some(Signal::derive(|| MotionStyle::new().x(0.0))),
                animate: Signal::derive(|| MotionStyle::new().x(120.0)),
                transition: Signal::derive(move || transition.get()).into(),
                reduced_motion: ReducedMotionConfig::Never.into(),
            },
        );

        view! { <div id="retarget-box" node_ref={node_ref}></div> }
    }

    let body = reset_body();
    leptos::mount::mount_to_body(Harness);
    next_frames(3).await;

    let element = div_by_id(&body, "retarget-box");
    assert_eq!(
        element.style().get_property_value("transform").unwrap(),
        "translateX(120px)"
    );
    assert_eq!(
        element.style().get_property_value("will-change").unwrap(),
        ""
    );
}

#[wasm_bindgen_test(async)]
async fn reduced_motion_flip_snaps_in_flight_animation() {
    #[component]
    fn Harness() -> impl IntoView {
        let reduced = RwSignal::new(ReducedMotionConfig::Never);
        let reduced_for_timeout = reduced;
        let callback = Closure::once_into_js(move || {
            reduced_for_timeout.set(ReducedMotionConfig::Always);
        });

        web_sys::window()
            .unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                callback.as_ref().unchecked_ref(),
                16,
            )
            .unwrap();

        let node_ref = NodeRef::<leptos::html::Div>::new();
        let _motion = use_motion(
            node_ref,
            MotionOptions {
                initial: Some(Signal::derive(|| MotionStyle::new().x(0.0))),
                animate: Signal::derive(|| MotionStyle::new().x(96.0)),
                transition: TransitionMap::new(Transition::tween(1.0, Easing::Linear).unwrap())
                    .into(),
                reduced_motion: Signal::derive(move || reduced.get()).into(),
            },
        );

        view! { <div id="reduced-box" node_ref={node_ref}></div> }
    }

    let body = reset_body();
    leptos::mount::mount_to_body(Harness);
    next_frames(3).await;

    let element = div_by_id(&body, "reduced-box");
    assert_eq!(
        element.style().get_property_value("transform").unwrap(),
        "translateX(96px)"
    );
    assert_eq!(
        element.style().get_property_value("will-change").unwrap(),
        ""
    );
}

#[wasm_bindgen_test(async)]
async fn will_change_clears_after_animation_completion() {
    #[component]
    fn Harness() -> impl IntoView {
        let node_ref = NodeRef::<leptos::html::Div>::new();
        let _motion = use_motion(
            node_ref,
            MotionOptions {
                initial: Some(Signal::derive(|| MotionStyle::new().x(0.0))),
                animate: Signal::derive(|| MotionStyle::new().x(48.0)),
                transition: TransitionMap::new(Transition::tween(0.01, Easing::Linear).unwrap())
                    .into(),
                reduced_motion: ReducedMotionConfig::Never.into(),
            },
        );

        view! { <div id="will-change-box" node_ref={node_ref}></div> }
    }

    let body = reset_body();
    leptos::mount::mount_to_body(Harness);
    next_frames(5).await;

    let element = div_by_id(&body, "will-change-box");
    assert_eq!(
        element.style().get_property_value("transform").unwrap(),
        "translateX(48px)"
    );
    assert_eq!(
        element.style().get_property_value("will-change").unwrap(),
        ""
    );
}

#[wasm_bindgen_test(async)]
async fn binder_supports_svg_nodes() {
    #[component]
    fn Harness() -> impl IntoView {
        let node_ref = NodeRef::<leptos::svg::Svg>::new();
        let _motion = use_motion(
            node_ref,
            MotionOptions {
                initial: None,
                animate: Signal::derive(|| MotionStyle::new().opacity(0.4).x(12.0)),
                transition: TransitionMap::new(Transition::spring()).into(),
                reduced_motion: ReducedMotionConfig::Always.into(),
            },
        );

        view! {
            <svg id="svg-box" node_ref={node_ref} width="24" height="24"></svg>
        }
    }

    let body = reset_body();
    leptos::mount::mount_to_body(Harness);
    next_frame().await;

    let element = body
        .query_selector("#svg-box")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::SvgElement>()
        .unwrap();

    assert_eq!(
        element.style().get_property_value("opacity").unwrap(),
        "0.4"
    );
    assert_eq!(
        element.style().get_property_value("transform").unwrap(),
        "translateX(12px)"
    );
}

#[wasm_bindgen_test(async)]
async fn sugar_and_binder_have_equivalent_final_styles() {
    #[component]
    fn Harness() -> impl IntoView {
        let node_ref = NodeRef::<leptos::html::Div>::new();
        let _motion = use_motion(
            node_ref,
            MotionOptions {
                initial: Some(Signal::derive(|| MotionStyle::new().opacity(0.0).x(0.0))),
                animate: Signal::derive(|| MotionStyle::new().opacity(0.6).x(32.0)),
                transition: TransitionMap::new(Transition::spring()).into(),
                reduced_motion: ReducedMotionConfig::Always.into(),
            },
        );

        view! {
            <>
                <div id="binder-eq" node_ref={node_ref}></div>
                <MotionDiv
                    animate=MotionStyle::new().opacity(0.6).x(32.0)
                    reduced_motion=ReducedMotionConfig::Always
                    attr:id="sugar-eq"
                ></MotionDiv>
            </>
        }
    }

    let body = reset_body();
    leptos::mount::mount_to_body(Harness);
    next_frame().await;

    let binder = div_by_id(&body, "binder-eq");
    let sugar = div_by_id(&body, "sugar-eq");

    for property in ["opacity", "transform", "will-change"] {
        assert_eq!(
            binder.style().get_property_value(property).unwrap(),
            sugar.style().get_property_value(property).unwrap()
        );
    }
}
