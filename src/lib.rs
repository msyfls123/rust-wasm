use futures::{future, Future};
use js_sys::Promise;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::future_to_promise;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response, console};


#[derive(Debug, Serialize, Deserialize)]
pub struct Day {
    pub born_person: String,
    pub content: String,
    pub date: String,
    pub detailed_date: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub die_person: Option<String>,
    pub event: String,
    pub lunar_date: String,
    pub number_of_comments: String,
    pub pic: String,
    pub rating: String,
    pub related_information: String,
    pub suggestion: String,
    pub url: String,
}


#[wasm_bindgen]
pub fn run(name: u16) -> Promise {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);
    let url = format!("https://day.ebichu.cc/api/{}", name);

    let request = Request::new_with_str_and_init(
        &url,
        &opts,
    )
    .unwrap();

    request
        .headers()
        .set("Accept", "application/json")
        .unwrap();

    let window = web_sys::window().unwrap();
    let request_promise = window.fetch_with_request(&request);

    let future = JsFuture::from(request_promise)
        .and_then(|resp_value| {
            // `resp_value` is a `Response` object.
            assert!(resp_value.is_instance_of::<Response>());
            let resp: Response = resp_value.dyn_into().unwrap();
            resp.json()
        })
        .and_then(|json_value: Promise| {
            // Convert this other `Promise` into a rust `Future`.
            JsFuture::from(json_value)
        })
        .and_then(|json| {
            // Use serde to parse the JSON into a struct.
            let day_info: Day = json.into_serde().unwrap();
            console::log_1(&day_info.born_person.clone().into());

            // Send the `Day` struct back to JS as an `Object`.
            future::ok(JsValue::from_serde(&day_info).unwrap())
        });

    // Convert this Rust `Future` back into a JS `Promise`.
    future_to_promise(future)
}