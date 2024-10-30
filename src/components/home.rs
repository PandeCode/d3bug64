use std::collections::HashMap;

use yew::prelude::*;

use crate::utils;
use crate::utils::{gh_repo_url, language_color};

// Define component properties
#[derive(PartialEq, Properties)]
pub struct HomeProps {}

const ACC: &str = "pandecode";

#[function_component]
pub fn Home(props: &HomeProps) -> Html {
	let HomeProps {} = props;

	let description = crate::data::get_description();

	let _misc = [
		"https://raw.githubusercontent.com/PandeCode/PandeCode/main/assets/bar_graph.png",
		"https://github-readme-stats.vercel.app/api/top-langs/?username=PandeCode&layout=compact&theme=dracula&hide_border=true)](https://github.com/anuraghazra/github-readme-stats",
		"https://github-readme-stats.vercel.app/api?username=PandeCode&theme=dracula&hide_border=true&show_icons=true"
	];

	let _file_to_parse =
		"https://raw.githubusercontent.com/PandeCode/PandeCode/refs/heads/main/README.md";

	// Define skills with their corresponding SVG file paths
	let skills = crate::data::get_skills();
	let tools = crate::data::get_tools();
	let langs = crate::data::get_langs();

	// State for holding projects
	let projects = use_state(|| {
		Vec::<(
			String,
			String,
			Option<Vec<String>>,
			Option<HashMap<String, u64>>,
		)>::new()
	});
	let projects_loading = use_state(|| true);

	{
		let projects = projects.clone();
		let projects_loading = projects_loading.clone();

		use_effect_with((), move |_| {
			let projects = projects.clone();
			let projects_loading = projects_loading.clone();

			wasm_bindgen_futures::spawn_local(async move {
				if let Some(repos) = utils::gh::get_repos(ACC.to_owned()).await {
					projects.set(
						repos
							.iter()
							.map(|repo| {
								(
									repo.name.clone(),
									repo.description.clone().unwrap_or(repo.name.clone()),
									None,
									None,
								)
							})
							.collect(),
					);
					projects_loading.set(false);
				}
			})
		});
	}

	{
		let projects = projects.clone();
		let projects_loading = projects_loading.clone();

		use_effect_with(projects_loading.clone(), move |_| {
			let projects = projects.clone();
			let projects_loading = projects_loading.clone();

			wasm_bindgen_futures::spawn_local(async move {
				if !*projects_loading {
					let mut projects_value = (*projects).clone();

					for (i, project) in (*projects).iter().enumerate() {
						if let Some(topics) = utils::gh::get_topics(ACC, &project.0.clone()).await {
							if let Some(langs) =
								utils::gh::get_langs(ACC.to_string(), project.0.clone()).await
							{
								projects_value[i] = (
									project.0.clone(),
									project.1.clone(),
									Some(topics),
									Some(langs),
								);
								projects.set(projects_value.clone());
							}
						}
					}
				}
			});
		});
	}

	let wait_html = html! {
		<div class="flex gap-2">
			{ for (0..3).map(|_| html! {
				<div class="px-3 py-1 bg-gray-200 dark:bg-gray-700 rounded-full w-24 h-6 animate-pulse"></div>
			}) }
		</div>
	};

	let projects_section = projects.iter().map(|project| {
		let name = project.0.clone();
		let desc = project.1.clone();
		let topics = project.2.clone();
		let langs = project.3.clone();

		html! {
			<div class="bg-white dark:bg-gray-800 p-6 rounded-lg shadow-lg transition-transform duration-300 hover:shadow-2xl hover:scale-101">
				<h3 class="text-xl font-semibold text-gray-800 dark:text-white">{ name.clone() }</h3>
				<p class="text-gray-600 dark:text-gray-300 mt-2">{ desc }</p>

				<div class="flex flex-wrap gap-2 mt-2">
					{
						if let Some(topics) = topics {
							html! {
								for topics.iter().map(|topic| html! {
									<span class="px-3 py-1 bg-green-500 text-white rounded-full text-sm font-semibold transform wobble transition-all duration-200 hover:scale-110">
										{ topic }
									</span>
								})
							}
						} else {
							wait_html.clone()
						}
					}
				</div>

				<div class="flex flex-wrap gap-2 mt-2">
					{
						if let Some(langs) = langs {
							// Calculate total bytes for percentage calculation
							let total_bytes: u64 = langs.values().sum();

							html! {
								for langs.iter().map(|(lang, &bytes)| {
									let percentage = (bytes as f64 / total_bytes as f64) * 100.0;
									let bg_color = language_color(lang);
									let style = format!("background: linear-gradient(90deg, {} {}%, transparent {}%);", bg_color, percentage, percentage);
									html! {
										<span class="px-3 py-1 dark:text-gray-100 text-gray-900 rounded-full text-sm font-semibold" style={format!("{}", style)}>
											{ format!("{} ( {:.2}% )", lang, percentage) }
										</span>
									}
								})
							}
						} else {
							wait_html.clone()
						}
					}
				</div>

				<a
					class="text-blue-500 hover:underline mt-4 block dark:text-blue-400"
					href={gh_repo_url(ACC.to_string(), name.clone())}
				>
					{ "View Project" }
				</a>
			</div>
		}
	});

	let langs_cards = 
		langs.iter().map(|(lang, icon)| {
    html! {
        <div class="flex flex-row justify-between items-center bg-white dark:bg-gray-800 p-4 rounded-lg shadow-lg transform hover:scale-105 transition duration-300 border border-gray-200 dark:border-gray-700 w-full max-w-lg">
            <span class="text-md font-medium text-gray-700 dark:text-gray-300">{ lang }</span>

            <div class="relative ml-3 flex-shrink-0">
                <img src={icon.clone()} alt={lang.clone()} class="w-12 h-12 rounded-full shadow-md transform transition-transform duration-200 hover:scale-110" />
                <div class="absolute inset-0 bg-gradient-to-r from-blue-500 to-purple-500 rounded-full opacity-25"></div>
            </div>
        </div>
    }
});
	let tools_cards = 
		tools.iter().map(|(tool, icon)| {
    html! {
        <div class="flex flex-row justify-between items-center bg-white dark:bg-gray-800 p-4 rounded-lg shadow-lg transform hover:scale-105 transition duration-300 border border-gray-200 dark:border-gray-700 w-full max-w-lg">
            <span class="text-md font-medium text-gray-700 dark:text-gray-300">{ tool }</span>

            <div class="relative ml-3 flex-shrink-0">
                <img src={icon.clone()} alt={tool.clone()} class="w-12 h-12 rounded-full shadow-md transform transition-transform duration-200 hover:scale-110" />
                <div class="absolute inset-0 bg-gradient-to-r from-blue-500 to-purple-500 rounded-full opacity-25"></div>
            </div>
        </div>
    }
});

	let skills_cards = 
		skills.iter().map(|(skill, icon)| {
    html! {
        <div class="flex flex-row justify-between items-center bg-white dark:bg-gray-800 p-4 rounded-lg shadow-lg transform hover:scale-105 transition duration-300 border border-gray-200 dark:border-gray-700 w-full max-w-lg">
            <span class="text-md font-medium text-gray-700 dark:text-gray-300">{ skill }</span>

            <div class="relative ml-3 flex-shrink-0">
                <img src={icon.clone()} alt={skill.clone()} class="w-12 h-12 rounded-full shadow-md transform transition-transform duration-200 hover:scale-110" />
                <div class="absolute inset-0 bg-gradient-to-r from-blue-500 to-purple-500 rounded-full opacity-25"></div>
            </div>
        </div>
    }
});



	// Render component HTML with Tailwind styling and SVG icons
	html! {
		<div class="flex flex-col items-center p-8 bg-gradient-to-r from-blue-100 to-purple-200 dark:bg-gradient-to-r dark:from-gray-800 dark:to-gray-900 min-h-screen">
			// Introduction Section
			<div class="bg-white dark:bg-gray-800 p-10 rounded-lg shadow-md text-center mb-8 max-width fade-in-up">
				<h2 class="text-5xl font-bold text-blue-600 dark:text-blue-400">{ "Shawn Pande" }</h2>
				<p class="text-gray-700 dark:text-gray-300 text-xl mt-4 leading-relaxed">
					{ description }
				</p>
			</div>

			// Skills Section
			<h1 class="text-4xl font-bold mb-8 text-gray-800 dark:text-gray-100 text-center">{ "Skills" }</h1>
			<div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-5 gap-10 mb-12 max-w-10xl mx-auto">
				{ for skills_cards }
			</div>


			// Langs Section
			<h1 class="text-4xl font-bold mb-8 text-gray-800 dark:text-gray-100 text-center">{ "Langs" }</h1>
			<div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-5 gap-10 mb-12 max-w-10xl mx-auto">
				{ for langs_cards }
			</div>


			// Tools Section
			<h1 class="text-4xl font-bold mb-8 text-gray-800 dark:text-gray-100 text-center">{ "Tools" }</h1>
			<div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-5 gap-10 mb-12 max-w-10xl mx-auto">
				{ for tools_cards }
			</div>

			// Projects Section
			<h2 class="text-2xl font-bold mt-10 mb-4 text-gray-800 dark:text-gray-100">{ "Projects" }</h2>
			<div class="grid grid-cols-1 md:grid-cols-2 gap-6 mb-10">
				{
					if *projects_loading {
						// Render loading skeletons as placeholders
						html! {
							for (0..2).map(|_| html! {
								<div class="bg-white dark:bg-gray-800 p-6 rounded-lg shadow-lg animate-pulse">
									<div class="h-6 bg-gray-200 dark:bg-gray-700 rounded w-3/4 mb-4"></div>
									<div class="h-4 bg-gray-200 dark:bg-gray-700 rounded w-full mb-2"></div>
									<div class="h-4 bg-gray-200 dark:bg-gray-700 rounded w-5/6 mb-4"></div>
									<div class="flex gap-2 mt-2">
										{ for (0..3).map(|_| html! {
											<span class="px-3 py-1 bg-gray-200 dark:bg-gray-700 rounded-full w-16 h-6"></span>
										}) }
									</div>
									<div class="h-4 bg-blue-200 dark:bg-gray-700 rounded w-1/2 mt-4"></div>
								</div>
							})
						}
					} else {
						html!{ for projects_section }
					}
				}
			</div>
		</div>
	}
}
