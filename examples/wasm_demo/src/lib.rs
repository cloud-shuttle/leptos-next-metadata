use leptos::*;
use leptos_next_metadata::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    log::info!("WASM demo started!");

    leptos::mount_to_body(|| {
        view! {
            <App/>
        }
    });
    Ok(())
}

#[component]
fn App() -> impl IntoView {
    provide_metadata_context();

    let (count, set_count) = create_signal(0);

    metadata! {
        title: format!("WASM Demo - Count: {}", count.get()),
        description: "Interactive WASM demo for leptos-next-metadata",
        keywords: ["leptos", "metadata", "wasm", "rust", "demo"],

        og_title: "Leptos Next Metadata - WASM Demo",
        og_description: "Interactive WASM demo for leptos-next-metadata",
        og_type: "website",
    }

    view! {
        <div>
            <h1>"WASM Demo"</h1>
            <p>"Current count: " {count}</p>
            <button on:click=move |_| set_count.update(|c| *c += 1)>"+"</button>
            <button on:click=move |_| set_count.update(|c| *c -= 1)>"-"</button>
            <p>"Check the browser console and page metadata for updates!"</p>
        </div>
    }
}

#[wasm_bindgen]
pub fn run_wasm_demo() {
    log::info!("run_wasm_demo called from JS!");
    // This function is called by the JS to ensure the WASM module is loaded and ready.
    // The Leptos app is mounted in main_js, so this can be a simple log.
}
