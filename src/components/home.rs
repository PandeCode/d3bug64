use yew::prelude::*;

use crate::{data::Description, utils::gh::request_get_cache};

#[derive(PartialEq, Properties)]
pub struct HomeProps {}

#[function_component]
pub fn Home(props: &HomeProps) -> Html {
	let HomeProps {} = props;

	let nums = [
		("25+", "full projects completed"),
		("7+", "years of experirance"),
		("20+", "projects lost to time"),
	];

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
		html! {
			<div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4 sm:gap-6 md:gap-8 lg:gap-10 mb-12 max-w-10xl mx-auto w-full">
				{ for cards.iter().map(|(text, icon)|  {
					html! {
						<div class="flex flex-col gap-1 items-center transform hover:scale-105 transition duration-300 w-full">
							<div class="relative flex-shrink-0">
								<img src={icon.clone()} alt={text.clone()} class="w-10 sm:w-12 md:w-14 lg:w-16 h-10 sm:h-12 md:h-14 lg:h-16 transform transition-transform duration-200 hover:scale-110" />
							</div>

							<span class="text-sm sm:text-md md:text-lg lg:text-xl text-center font-medium text-gray-700 dark:text-gray-300">{ text.clone() }</span>
						</div>
					}
				})}
			</div>
		}
	};

	// Render component HTML with Tailwind styling and SVG icons
	html! {
		<div class="flex flex-col items-center p-4 sm:p-6 md:p-8 bg-gradient-to-r from-blue-100 to-purple-200 dark:bg-gradient-to-r dark:from-gray-800 dark:to-gray-900 min-h-screen w-full">
			<div class="grid gap-6 sm:gap-8 md:gap-10 grid-cols-1 lg:grid-cols-2">
				<Description />

				<div class="flex flex-col items-center">
					{ section("Languages", skills.1) }
					{ section("Skills", skills.0) }
					{ section("Tools", skills.2) }
				</div>
			</div>

			<div class="flex flex-col sm:flex-row justify-around gap-6 sm:gap-8 md:gap-10">
				{ for nums.iter().map(|n| {
					html!{
						<div>
							<h2 class="text-3xl sm:text-4xl md:text-5xl font-bold text-blue-600 dark:text-blue-400">
								{ n.0 }
							</h2>
							<p class="text-gray-700 dark:text-gray-300 text-lg sm:text-xl mt-2 sm:mt-4 leading-relaxed max-w-3xl mx-auto">
								{ n.1 }
							</p>
						</div>
					}
				}) }
			</div>

			// <h1 class="text-4xl font-bold mb-8 text-gray-800 dark:text-gray-100 text-center">{ "Github Stats" }</h1>

			// <a href={"https://dotfyle.com/plugins/pandecode/nvim-config"}>
			//     <img src={"https://dotfyle.com/plugins/pandecode/nvim-config/shield"} />
			// </a>

			// <div class="flex flex-row gap-10"> <img src={_misc[1]}/> <img src={_misc[2]}/> </div>
			// <img class="w-full" src={_misc[0]}/>


		</div>
	}
}
