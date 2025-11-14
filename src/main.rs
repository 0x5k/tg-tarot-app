mod app;
mod deck;
mod feedback;
mod reading;
mod telegram;
mod ui;

fn main() {
    yew::Renderer::<app::App>::new().render();
}
