use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::blog_display::BlogDisplay;
use crate::components::home::Home;
use crate::components::blog::Blog;
use crate::components::not_found::NotFound;
use crate::components::projects::Projects;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
	#[at("/")]
	Home,
	#[at("/projects")]
	Projects,
	#[at("/blog")]
	Blog,
	#[at("/blog/:path")]
	BlogDisplay { path: String },
	#[not_found]
	#[at("/404")]
	NotFound,
}

pub fn switch(routes: Route) -> Html {
	match routes {
		Route::Home => html! { <Home /> },
		Route::Blog => html! { <Blog /> },
		Route::Projects => html! { <Projects /> },
		Route::BlogDisplay { path } => html! { <BlogDisplay path={path}/> },
		Route::NotFound => html! { <NotFound /> },
	}
}
