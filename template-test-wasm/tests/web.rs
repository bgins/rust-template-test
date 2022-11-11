//! Test suite for the Web and headless browsers.

use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};
wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_add() {
    assert_eq!(template_test_wasm::add(3, 2), 5);
    template_test_wasm::console_log!("{}", "Test passes!");
}
