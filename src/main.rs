mod app;
mod deck;
mod telegram;
mod ui;

fn main() {
    yew::Renderer::<app::App>::new().render();
}
