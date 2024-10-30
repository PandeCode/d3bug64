use std::collections::HashMap;
use yew::prelude::*;

use crate::utils;

use crate::data::ACC;
use crate::utils::{gh_repo_url, language_color};

#[derive(PartialEq, Properties)]
pub struct ProjectsProps {}

#[function_component]
pub fn Projects(props: &ProjectsProps) -> Html {
	let ProjectsProps {} = props;

	let wait_html = html! {
		<div class="flex gap-2">
			{ for (0..3).map(|_| html! {
				<div class="px-3 py-1 bg-gray-200 dark:bg-gray-700 rounded-full w-24 h-6 animate-pulse"></div>
			}) }
		</div>
	};

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

		use_effect(move || {
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

	let projects = projects.iter().map(|project| {
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

	html! {
		<>
			<h2 class="text-2xl font-bold mt-10 mb-4 text-gray-800 dark:text-gray-100">{ "Projects" }</h2>
			<div class="grid grid-cols-1 md:grid-cols-2 gap-6 mb-10">
				{
					if *projects_loading {
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
						html! { for projects }
					}
				}
			</div>
		</>
	}
}
