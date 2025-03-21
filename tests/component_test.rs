use gloo_timers::future::TimeoutFuture;
use wasm_bindgen_test::*;
use web_sys::Document;
use yew::Renderer;
use gloo_utils::document;
use yew_timer_tdd::app::App;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test(async)]
pub async fn component_renders_title_and_buttons() {
    let document: Document = document();
    let body = document.body().unwrap();
    let root = document.create_element("div").unwrap();
    root.set_id("test-root");
    body.append_child(&root).unwrap();

    Renderer::<App>::with_root(root.clone()).render();

    TimeoutFuture::new(100).await;

    let rendered_html = root.inner_html();

    assert!(rendered_html.contains("Pomodoro Mini Timer"));

    let buttons = root.get_elements_by_tag_name("button");
    assert!(buttons.length() >= 2);

    let labels: Vec<String> = (0..buttons.length()) 
        .filter_map(|i| buttons.item(i))
        .map(|b| b.text_content().unwrap_or_default())
        .collect();

    assert!(labels.contains(&"Start".to_string()));
    assert!(labels.contains(&"Reset".to_string()));
}