mod app;
mod timers;

use app::App;
use yew::Renderer;

fn main() {
    Renderer::<App>::new().render();
}
