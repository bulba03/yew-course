mod app;
mod components;
pub mod pages;
use app::App;
pub mod api;

fn main() {
    yew::Renderer::<App>::new().render();
}