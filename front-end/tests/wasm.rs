use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn it_starts_without_crashing() {
    // Arrange
    let document = web_sys::window()
        .expect("No window exists")
        .document()
        .unwrap();

    let body = document.query_selector("body").unwrap().unwrap();
    body.set_id("phone");

    // Act
    visnake::run().expect("Failed to start");

    // Assert
    assert!(document.get_element_by_id("canvas").is_some());
}
