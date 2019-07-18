use web_worker::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run_test() {
    //console_log!("henlo");
}


#[wasm_bindgen]
pub struct BBBB;

#[wasm_bindgen]
impl BBBB {
    #[wasm_bindgen(constructor)]
    pub fn new(object: &JsValue) -> Result<BBBB, JsValue> {
        Ok(BBBB)
    }

    #[wasm_bindgen]
    pub fn something() {

    }
}
