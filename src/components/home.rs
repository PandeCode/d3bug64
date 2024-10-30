use yew::prelude::*;

use crate::{components::projects::Projects, data::Description, utils::gh::request_get_cache};

#[derive(PartialEq, Properties)]
pub struct HomeProps {}

#[function_component]
pub fn Home(props: &HomeProps) -> Html {
	let HomeProps {} = props;

	let _misc = [
		"https://raw.githubusercontent.com/PandeCode/PandeCode/main/assets/bar_graph.png",
		"https://github-readme-stats.vercel.app/api/top-langs/?username=PandeCode&layout=compact&theme=dracula&hide_border=true)](https://github.com/anuraghazra/github-readme-stats",
		"https://github-readme-stats.vercel.app/api?username=PandeCode&theme=dracula&hide_border=true&show_icons=true"
	];

	let gh_readme =
		"https://raw.githubusercontent.com/PandeCode/PandeCode/refs/heads/main/README.md";
	let raw_data = use_state(|| String::new());
	{
		let raw_data = raw_data.clone();

		use_effect(move || {
			wasm_bindgen_futures::spawn_local(async move {
				let raw_data = raw_data.clone();
				if let Some(rm) = request_get_cache(gh_readme).await {
					raw_data.set(rm);
				};
			});
		});
	}

	// Define skills with their corresponding SVG file paths
	let skills = crate::data::get_skills();

	let section = |title, cards: Vec<(String, String)>| {
		html! {<>
			<h1 class="text-4xl font-bold mb-8 text-gray-800 dark:text-gray-100 text-center">{ title }</h1>
			<div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-5 gap-10 mb-12 max-w-10xl mx-auto">
				{ for cards.iter().map(|(text, icon)|  {
					html! {
						<div class="flex flex-row justify-between items-center bg-white dark:bg-gray-800 p-4 rounded-lg shadow-lg transform hover:scale-105 transition duration-300 border border-gray-200 dark:border-gray-700 w-full max-w-lg">
							<span class="text-md font-medium text-gray-700 dark:text-gray-300">{ text.clone() }</span>

							<div class="relative ml-3 flex-shrink-0">
								<img src={icon.clone()} alt={text.clone()} class="w-12 h-12 rounded-full shadow-md transform transition-transform duration-200 hover:scale-110" />
								<div class="absolute inset-0 bg-gradient-to-r from-blue-500 to-purple-500 rounded-full opacity-25"></div>
							</div>
						</div>
					}
				})}
			</div>
		</>}
	};

	// Render component HTML with Tailwind styling and SVG icons
	html! {
		<div class="flex flex-col items-center p-8 bg-gradient-to-r from-blue-100 to-purple-200 dark:bg-gradient-to-r dark:from-gray-800 dark:to-gray-900 min-h-screen">
			<Description />

			{ section("Skills", skills.0) }
			{ section("Languages", skills.1) }
			{ section("Tools", skills.2) }


			<Projects />

			<h1 class="text-4xl font-bold mb-8 text-gray-800 dark:text-gray-100 text-center">{ "Github Stats" }</h1>


		</div>
	}
}
