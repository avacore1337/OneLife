use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

wasm_bindgen_test_configure!(run_in_browser);

const X: i32 = 1;

// This runs a unit test in native Rust, so it can only use Rust APIs.
#[test]
fn rust_test() {
    assert_eq!(X, 1);
}

// This runs a unit test in the browser, so it can use browser APIs.
#[wasm_bindgen_test]
fn web_test() {
    assert_eq!(X, 1);
}
