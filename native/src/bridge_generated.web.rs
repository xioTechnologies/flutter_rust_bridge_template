use super::*;
// Section: wire functions

#[wasm_bindgen]
pub fn wire_platform(port_: MessagePort) {
    wire_platform_impl(port_)
}

#[wasm_bindgen]
pub fn wire_rust_release_mode(port_: MessagePort) {
    wire_rust_release_mode_impl(port_)
}

#[wasm_bindgen]
pub fn wire_say_hello() -> support::WireSyncReturn {
    wire_say_hello_impl()
}

#[wasm_bindgen]
pub fn wire_get_info() -> support::WireSyncReturn {
    wire_get_info_impl()
}

// Section: allocate functions

// Section: related functions

// Section: impl Wire2Api

// Section: impl Wire2Api for JsValue

impl<T> Wire2Api<Option<T>> for JsValue
where
    JsValue: Wire2Api<T>,
{
    fn wire2api(self) -> Option<T> {
        (!self.is_null() && !self.is_undefined()).then(|| self.wire2api())
    }
}
