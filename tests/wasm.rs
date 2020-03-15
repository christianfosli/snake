extern crate visnake;
extern crate wasm_bindgen;
extern crate wasm_bindgen_test;
extern crate web_sys;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn it_should_have_a_canvas_element() {
    visnake::run().expect("Failed to start");
    let document = web_sys::window()
        .expect("No window exists")
        .document()
        .unwrap();

    assert!(document.get_element_by_id("canvas").is_some());
}
