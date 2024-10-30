mod components;
mod router;
mod theme;
mod utils;
mod data;

use weblog::{
	console_error, // ,console_log
};
use yew::prelude::*;
use yew_router::prelude::*;

use components::nav_bar::NavBar;

#[function_component(App)]
fn app() -> Html {
	use_effect(|| {
		let res = theme::setup_theme();
		if res.is_err() {
			console_error!("Failed to load theme: ", res.err());
		}
	});

	html! {
		<BrowserRouter>
			<NavBar />
			<Switch<router::Route> render={router::switch} />
		</BrowserRouter>
	}
}
fn main() {
	yew::Renderer::<App>::new().render();
}
