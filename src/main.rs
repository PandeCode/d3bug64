mod components;
mod router;

use weblog::console_log;
use yew::prelude::*;
use yew_router::prelude::*;

use components::nav_bar::NavBar;

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <NavBar />
            <Switch<router::Route> render={router::switch} />
        </BrowserRouter>
    }
}
fn main() {
    console_log!("Hello world!");
    yew::Renderer::<App>::new().render();
}
