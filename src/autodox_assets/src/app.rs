use web_sys::console::log_1;
use yew::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/index.js")]
extern "C" {
    #[wasm_bindgen(js_name = my_function)]
    pub fn test_canister(name: String) -> String;
}


#[function_component]
pub fn App() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };
    // let x = test_canister("hello".to_string());
    // log_1(&format!("ondragstart {:?}", x).into());
    //TODO
    // Failed to load module script: Expected a JavaScript module script but the server responded with a MIME type of "text/html". Strict MIME type checking is enforced for module scripts per HTML spec.


    html! {
        <div>
            <button {onclick}>{ "+1" }</button>
            <p>{ *counter }</p>
        </div>
    }
}
