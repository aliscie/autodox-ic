extern crate yew;

use yew::prelude::*;
mod utils;
mod extensions;
mod components;
mod app;

pub use app::App;


fn main() {
    yew::Renderer::<App>::new().render();
}

