#[cfg(not(target_arch = "wasm32"))]
pub async fn sleep(ms: i32) {
    use std::time::Duration;
    tokio::time::sleep(Duration::from_millis(ms as u64)).await;
}

#[cfg(target_arch = "wasm32")]
pub async fn sleep(ms: i32) {
    use wasm_bindgen_futures::js_sys;

    let mut cb = |resolve: js_sys::Function, _reject: js_sys::Function| {
        let _ = web_sys::window()
            .unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, ms);
    };

    let p = js_sys::Promise::new(&mut cb);

    wasm_bindgen_futures::JsFuture::from(p).await.unwrap();
}
