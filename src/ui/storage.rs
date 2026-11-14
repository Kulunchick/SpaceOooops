//! Browser local storage and the page URL, behind small best-effort helpers.
//! Everything here degrades silently: no storage, blocked storage or a missing window means "nothing stored".

fn storage() -> Option<web_sys::Storage> {
    web_sys::window()?.local_storage().ok().flatten()
}

pub fn read(key: &str) -> Option<String> {
    storage()?.get_item(key).ok().flatten()
}

pub fn write(key: &str, value: &str) {
    if let Some(s) = storage() {
        let _ = s.set_item(key, value);
    }
}

pub fn remove(key: &str) {
    if let Some(s) = storage() {
        let _ = s.remove_item(key);
    }
}

/// The URL fragment without the leading `#`, or empty.
pub fn url_fragment() -> String {
    web_sys::window()
        .and_then(|w| w.location().hash().ok())
        .map(|h| h.trim_start_matches('#').to_string())
        .unwrap_or_default()
}

/// Removes the fragment from the address bar without reloading, so a reload does not re-open a share link.
pub fn clear_url_fragment() {
    if let Some(w) = web_sys::window()
        && let (Ok(path), Ok(history)) = (w.location().pathname(), w.history())
    {
        let _ = history.replace_state_with_url(&wasm_bindgen_null(), "", Some(&path));
    }
}

fn wasm_bindgen_null() -> web_sys::wasm_bindgen::JsValue {
    web_sys::wasm_bindgen::JsValue::NULL
}

/// Link to the game itself with `fragment` appended (works under any sub-path, no server routing needed).
pub fn page_url_with_fragment(fragment: &str) -> String {
    let base = web_sys::window()
        .and_then(|w| {
            let l = w.location();
            Some(format!("{}{}", l.origin().ok()?, l.pathname().ok()?))
        })
        .unwrap_or_default();
    format!("{base}#{fragment}")
}

/// Asks the browser to copy `text`; resolves to whether it worked. Calls `navigator.clipboard.writeText`
/// through reflection because the typed web-sys Clipboard API is still behind an unstable cfg flag.
pub async fn copy_to_clipboard(text: &str) -> bool {
    use web_sys::js_sys::{Function, Promise, Reflect};
    use web_sys::wasm_bindgen::{JsCast, JsValue};
    let Some(w) = web_sys::window() else { return false };
    let nav = w.navigator();
    let Ok(clipboard) = Reflect::get(&nav, &JsValue::from_str("clipboard")) else { return false };
    if clipboard.is_undefined() || clipboard.is_null() {
        return false;
    }
    let Ok(write) = Reflect::get(&clipboard, &JsValue::from_str("writeText")) else { return false };
    let Ok(write) = write.dyn_into::<Function>() else { return false };
    let Ok(promise) = write.call1(&clipboard, &JsValue::from_str(text)) else { return false };
    let Ok(promise) = promise.dyn_into::<Promise>() else { return false };
    wasm_bindgen_futures::JsFuture::from(promise).await.is_ok()
}
