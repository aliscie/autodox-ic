use yew::prelude::*;
pub mod app;
use app::App;
fn main() {
    yew::Renderer::<App>::new().render();
}

