use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::about::About;
use crate::components::articles::Articles;
use crate::components::home::Home;
use crate::components::not_found::NotFound;
use crate::components::projects::Projects;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
	#[at("/")]
	Home,
	#[at("/about")]
	About,
	#[at("/projects")]
	Projects,
	#[at("/articles")]
	Articles,
	#[not_found]
	#[at("/404")]
	NotFound,
}

pub fn switch(routes: Route) -> Html {
	match routes {
		Route::Home => html! { <Home /> },
		Route::Articles => html! { <Articles /> },
		Route::About => html! { <About /> },
		Route::Projects => html! { <Projects /> },
		Route::NotFound => html! { <NotFound /> },
	}
}
