use leptos::prelude::*;

#[cfg(target_arch = "wasm32")]
use std::cell::RefCell;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::closure::Closure;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;

#[cfg(target_arch = "wasm32")]
struct ReducedMotionService {
    signal: ArcRwSignal<bool>,
    _media_query: Option<web_sys::MediaQueryList>,
    _callback: Option<Closure<dyn FnMut(web_sys::Event)>>,
}

pub fn use_reduced_motion() -> Signal<bool> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        Signal::derive(|| false)
    }

    #[cfg(target_arch = "wasm32")]
    {
        Signal::from(shared_reduced_motion_signal())
    }
}

#[cfg(target_arch = "wasm32")]
thread_local! {
    static REDUCED_MOTION_SERVICE: RefCell<Option<ReducedMotionService>> = const { RefCell::new(None) };
}

#[cfg(target_arch = "wasm32")]
fn shared_reduced_motion_signal() -> ArcRwSignal<bool> {
    REDUCED_MOTION_SERVICE.with(|service| {
        if let Some(service) = service.borrow().as_ref() {
            return service.signal.clone();
        }

        let signal = ArcRwSignal::new(browser_prefers_reduced_motion());
        let (media_query, callback) = install_listener(signal.clone());

        *service.borrow_mut() = Some(ReducedMotionService {
            signal: signal.clone(),
            _media_query: media_query,
            _callback: callback,
        });

        signal
    })
}

#[cfg(target_arch = "wasm32")]
fn install_listener(
    signal: ArcRwSignal<bool>,
) -> (
    Option<web_sys::MediaQueryList>,
    Option<Closure<dyn FnMut(web_sys::Event)>>,
) {
    let Some(window) = web_sys::window() else {
        return (None, None);
    };
    let Ok(Some(media_query)) = window.match_media("(prefers-reduced-motion: reduce)") else {
        return (None, None);
    };

    let tracked_query = media_query.clone();
    let callback = Closure::wrap(Box::new(move |event: web_sys::Event| {
        let matches = event
            .target()
            .and_then(|target| target.dyn_into::<web_sys::MediaQueryList>().ok())
            .map(|query| query.matches())
            .unwrap_or_else(|| tracked_query.matches());
        signal.set(matches);
    }) as Box<dyn FnMut(web_sys::Event)>);

    let _ =
        media_query.add_event_listener_with_callback("change", callback.as_ref().unchecked_ref());

    (Some(media_query), Some(callback))
}

#[cfg(target_arch = "wasm32")]
fn browser_prefers_reduced_motion() -> bool {
    web_sys::window()
        .and_then(|window| window.match_media("(prefers-reduced-motion: reduce)").ok())
        .flatten()
        .map(|query| query.matches())
        .unwrap_or(false)
}
