extern crate wasm_bindgen;
extern crate web_sys;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

// Called by our JS entry point
#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    add_canvas()?;

    Ok(())
}

fn add_canvas() -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let body = document.body().unwrap();

    let canvas = document
        .create_element("canvas")?
        .dyn_into::<HtmlElement>()?;
    canvas.set_id("canvas");

    body.append_child(&canvas)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pass() {
        assert_eq!(1, 1);
    }
}
