use super::*;
// Section: wire functions

#[wasm_bindgen]
pub fn wire_get_usb_info() -> support::WireSyncReturn {
    wire_get_usb_info_impl()
}

#[wasm_bindgen]
pub fn wire_get_tcp_info() -> support::WireSyncReturn {
    wire_get_tcp_info_impl()
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
