mod app;
use app::front::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
