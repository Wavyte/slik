use wasm_bindgen::{JsCast, JsValue};
use web_sys::CssStyleDeclaration;

pub(crate) fn style_for_node<T>(node: &T) -> Option<CssStyleDeclaration>
where
    T: JsCast + AsRef<JsValue> + Clone + 'static,
{
    let value = node.as_ref();
    value
        .dyn_ref::<web_sys::HtmlElement>()
        .map(web_sys::HtmlElement::style)
        .or_else(|| {
            value
                .dyn_ref::<web_sys::SvgElement>()
                .map(web_sys::SvgElement::style)
        })
}
