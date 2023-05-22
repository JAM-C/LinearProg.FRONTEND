
use gloo::console::log;
use reqwasm::http::Request;

use super::api_url;

pub fn log_in() {
    wasm_bindgen_futures::spawn_local(async move {
        let response = Request::get(&api_url("log_in"))
            .send().await.unwrap();
        log!(response.status())
    })
}
