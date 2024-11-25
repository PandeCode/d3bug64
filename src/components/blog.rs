use weblog::{console_error, console_info};
use yew::prelude::*;

use crate::router::Route; 
use crate::utils;

#[derive(PartialEq, Properties)]
pub struct BlogProps {}

#[function_component]
pub fn Blog(props: &BlogProps) -> Html {
	let BlogProps {} = props;
	let blogs = use_state(|| Vec::<utils::gh_blog::Blog>::new());
	let blogs_loading = use_state(|| true);

	{
		let blogs = blogs.clone();
		let blogs_loading = blogs_loading.clone();

		use_effect_with((), move |_| {
			let blogs = blogs.clone();
			let blogs_loading = blogs_loading.clone();

			wasm_bindgen_futures::spawn_local(async move {
				if let Some(data_file) = utils::gh_blog::get_data_file().await {
					blogs.set(data_file);
					blogs_loading.set(false);
				} else {
					console_error!("Failed to get blog data");
				}
			})
		});
	}

	let navigator = yew_router::prelude::use_navigator().unwrap();

	let blog_cards = blogs.iter().map(|blog| {
		let id = blog.id.clone();
		let title = blog.title.clone();
		let navigate_to_blog = {
			let navigator = navigator.clone();
			Callback::from(move |_| {
				navigator.push(&Route::BlogDisplay { path: id.clone() });
			})
		};

		html! {
			<div
				onclick={navigate_to_blog}
				class="bg-white dark:bg-gray-800 p-6 rounded-lg shadow-lg transition-transform duration-300 hover:scale-105 flex flex-col cursor-pointer"
			>
				<h3 class="text-xl font-bold text-gray-800 dark:text-white hover:underline">{ &title }</h3>
				<p class="text-gray-600 dark:text-gray-300 text-sm mt-2">{ blog.date.clone() }</p>
				<div class="flex flex-wrap gap-2 mt-4">
					{ for blog.tags.iter().map(|tag| html! {
						<span class="px-3 py-1 bg-green-500 text-white rounded-full text-xs font-medium">{ tag }</span>
					}) }
				</div>
				<div class="mt-6 text-gray-700 dark:text-gray-300 text-sm">
					{ format!("Blog ID: {}", &blog.id) }
				</div>
			</div>
		}
	});

	let loading_cards = (0..3).map(|_| {
		html! {
			<div class="bg-white dark:bg-gray-800 p-6 rounded-lg shadow-lg animate-pulse flex flex-col">
				<div class="h-6 bg-gray-200 dark:bg-gray-700 rounded w-3/4 mb-4"></div>
				<div class="h-4 bg-gray-200 dark:bg-gray-700 rounded w-full mb-2"></div>
				<div class="h-4 bg-gray-200 dark:bg-gray-700 rounded w-5/6 mb-4"></div>
				<div class="flex gap-2 mt-4">
					{ for (0..3).map(|_| html! {
						<span class="px-3 py-1 bg-gray-200 dark:bg-gray-700 rounded-full w-16 h-6"></span>
					}) }
				</div>
			</div>
		}
	});

	html! {
		<div class="p-8 bg-gradient-to-r from-blue-100 to-purple-200 dark:bg-gradient-to-r dark:from-gray-800 dark:to-gray-900 min-h-screen">
			<h1 class="text-4xl font-bold text-gray-800 dark:text-gray-100 mb-8 text-center">{ "Blog, Research, Notes and Anything really" }</h1>
			<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
				{
					if *blogs_loading {
						html! { for loading_cards }
					} else {
						html! { for blog_cards }
					}
				}
			</div>
		</div>
	}
}
