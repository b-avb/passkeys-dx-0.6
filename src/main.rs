use dioxus::prelude::*;
use wasm_bindgen::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Hero {}
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = globalThis, js_name = initKreivo)]
    pub fn initKreivo();
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = globalThis, js_name = initPass)]
    pub fn initPass();
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = globalThis, js_name = loginPass)]
    pub fn loginPass();
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = globalThis, js_name = dispatchPass)]
    pub fn dispatchPass();
}
#[component]
pub fn Hero() -> Element {
    rsx! {
        div {
            id: "hero",
            button {
                onclick: move |_| {
                    initKreivo()
                },
                "Init Kreivo",
            }
            button {
                onclick: move |_| {
                    initPass()
                },
                "Register passkeys",
            }
            button {
                onclick: move |_| {
                    loginPass()
                },
                "Login with passkeys",
            }
            button {
                onclick: move |_| {
                    dispatchPass()
                },
                "Dispatch passkeys",
            }
        }
    }
}
