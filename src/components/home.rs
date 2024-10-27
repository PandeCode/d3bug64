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

	let svg_asset = |f: &str| format!("assets/svgs/{}.svg", f);

	// Define skills with their corresponding SVG file paths
	let skills = [
		("Rust", svg_asset("rust")),
		("Fullstack", svg_asset("web")),
		("C++", svg_asset("terminal")),
		("Python", svg_asset("python")),
		("Linux", svg_asset("linux")),
	];

	// State for holding projects
	let projects = use_state(|| Vec::<(String, String, Option<HashMap<String, u64>>)>::new());
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
								)
							})
							.collect(),
					);
					projects_loading.set(false);
				}
			})
		})
	};

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
						if let Some(langs) =
							utils::gh::get_langs(ACC.to_string(), project.0.clone()).await
						{
							projects_value[i] = (project.0.clone(), project.1.clone(), Some(langs));
							projects.set(projects_value.clone());
						}
					}
				}
			});
		});
	}

	let projects_section = projects.iter().map(|project| {
		let name = project.0.clone();
		let desc = project.1.clone();
		let langs = project.2.clone();

		html! {
			<div class="bg-white dark:bg-gray-800 p-6 rounded-lg shadow-lg">
				<h3 class="text-xl font-semibold text-gray-800 dark:text-white">{ name.clone() }</h3>
				<p class="text-gray-600 dark:text-gray-300 mt-2">{ desc }</p>
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
							html! {
								<div class="flex gap-2">
									{ for (0..3).map(|_| html! {
										<div class="px-3 py-1 bg-gray-200 dark:bg-gray-700 rounded-full w-24 h-6 animate-pulse"></div>
									}) }
								</div>
							}
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

	// Render component HTML with Tailwind styling and SVG icons
	html! {
			<div class="flex flex-col items-center p-8 bg-gray-100 dark:bg-gray-900 min-h-screen">
				// Introduction Section
				<div class="bg-white dark:bg-gray-800 p-10 rounded-lg shadow-md text-center mb-8 max-width">
					<h2 class="text-5xl font-bold text-blue-600 dark:text-blue-400">{ "Shawn Pande" }</h2>
					<p class="text-gray-700 dark:text-gray-300 text-xl mt-4 leading-relaxed">
						{ "A passionate developer with expertise in Rust, C++, and web technologies. I enjoy building full-stack applications and optimizing performance through efficient coding practices." }
					</p>
				</div>


		// Skills Section
	<h1 class="text-4xl font-bold mb-8 text-gray-800 dark:text-gray-100 text-center">{ "My Skills" }</h1>
	<div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-5 gap-10 mb-12 max-w-6xl mx-auto">
		{ for skills.iter().map(|(skill, icon)| {
			html! {
				<div class="flex flex-col items-center bg-white dark:bg-gray-800 p-8 rounded-lg shadow-lg transform hover:scale-105 transition duration-300 border border-gray-200 dark:border-gray-700">
					<div class="relative mb-6">
						<img src={icon.clone()} alt={*skill} class="w-20 h-20 mb-4 rounded-full shadow-lg transform transition-transform duration-200 hover:scale-110" />
						<div class="absolute inset-0 bg-gradient-to-r from-blue-500 to-purple-500 rounded-full opacity-25"></div>
					</div>
					<span class="text-xl font-semibold text-gray-700 dark:text-gray-300 text-center">{ skill }</span>
				</div>
			}
		}) }
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
										<div class="h-4 bg-blue-200 dark:bg-blue-700 rounded w-1/3 mt-6"></div>
									</div>
								})
							}
						} else {
							html! {
								<>
									{ for projects_section }
								</>
							}
						}
					}
				</div>

				// Experience Section
				<h2 class="text-2xl font-bold mt-10 mb-4 text-gray-800 dark:text-gray-100">{ "Experience" }</h2>
				<div class="bg-white dark:bg-gray-800 p-6 rounded-lg shadow-lg mb-10">
					<h3 class="text-xl font-semibold text-gray-800 dark:text-white">{ "Job Title at Company" }</h3>
					<p class="text-gray-600 dark:text-gray-300 mt-2">{ "Description of your role and contributions." }</p>
					<p class="text-gray-500 dark:text-gray-400 text-sm">{ "Dates of Employment" }</p>
				</div>

				// Education Section
				<h2 class="text-2xl font-bold mt-10 mb-4 text-gray-800 dark:text-gray-100">{ "Education" }</h2>
				<div class="bg-white dark:bg-gray-800 p-6 rounded-lg shadow-lg mb-10">
					<h3 class="text-xl font-semibold text-gray-800 dark:text-white">{ "Degree, Major at University" }</h3>
					<p class="text-gray-500 dark:text-gray-400 text-sm">{ "Graduation Year" }</p>
				</div>

				// Testimonials Section
				<h2 class="text-2xl font-bold mt-10 mb-4 text-gray-800 dark:text-gray-100">{ "Testimonials" }</h2>
				<blockquote class="bg-gray-200 dark:bg-gray-700 p-4 rounded-lg mb-10">
					<p class="text-gray-700 dark:text-gray-300 italic">
						{ "Shawn is a dedicated developer with a keen eye for detail." }
					</p>
					<footer class="mt-2 text-gray-500 dark:text-gray-400">{ "— Person Name, Position" }</footer>
				</blockquote>

				// Contact Information
				<h2 class="text-2xl font-bold mt-10 mb-4 text-gray-800 dark:text-gray-100">{ "Contact Me" }</h2>
				<p class="text-gray-700 dark:text-gray-300">{ "Feel free to reach out!" }</p>
				<a href="mailto:your-email@example.com" class="text-blue-500 hover:underline dark:text-blue-400">
					{ "your-email@example.com" }
				</a>
			</div>
		}
}
