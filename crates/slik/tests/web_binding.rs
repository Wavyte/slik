#![cfg(target_arch = "wasm32")]

use leptos::prelude::*;
use slik::html::MotionDiv;
use slik::prelude::*;
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

fn reset_body() -> web_sys::HtmlElement {
    let document = web_sys::window().unwrap().document().unwrap();
    let body = document.body().unwrap();
    body.set_inner_html("");
    body
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

    let element = body
        .query_selector("#binder-box")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::HtmlElement>()
        .unwrap();

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
