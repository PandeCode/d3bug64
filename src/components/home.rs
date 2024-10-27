use yew::prelude::*;

use crate::utils;
use crate::utils::{gh_repo_url, set_href};

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
	let projects = use_state(|| Vec::<(String, String, Option<Vec<String>>)>::new()); // name description readme
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
									// async {
									//     utils::get_readme(
									//         repo.name.clone(),
									//         repo.default_branch.clone(),
									//     )
									//     .await
									//     .unwrap_or(repo.name.clone())
									// },
								)
							})
							.collect(),
					);
					projects_loading.set(false);
				};
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
			<div class="bg-white p-6 rounded-lg shadow-lg">
				<h3 class="text-xl font-semibold text-gray-800">{ name.clone() }</h3>
				<p class="text-gray-600 mt-2">{ desc }</p>
				<p class="text-gray-600 mt-2">
					{ {

				if let Some(langs) = langs
				{ html! {for langs.iter().map(|l|  html!{{l}}) }} else {
				html! { {"Abb"} }

				}

				} }
				</p>
				<button
					class="text-blue-500 hover:underline mt-4 block"
					onclick={move |_| { set_href(gh_repo_url(ACC.to_string(), name.clone())); }}
				>
					{ "View Project" }
				</button>
			</div>
		}
	});

	// Render component HTML with Tailwind styling and SVG icons
	html! {
		<div class="flex flex-col items-center p-8 bg-gray-100 min-h-screen">
			// Introduction Section
			<div class="bg-white p-10 rounded-lg shadow-md text-center max-w-lg mb-8">
				<h2 class="text-5xl font-bold text-blue-600">{ "Shawn Pande" }</h2>
				<p class="text-gray-700 text-xl mt-4 leading-relaxed">
					{ "A passionate developer with expertise in Rust, C++, and web technologies. I enjoy building full-stack applications and optimizing performance through efficient coding practices." }
				</p>
			</div>
			// Skills Section
			<h1 class="text-3xl font-bold mb-6 text-gray-800">{ "My Skills" }</h1>
			<div class="grid grid-cols-2 gap-6 mb-10">
				{ for skills.iter().map(|(skill, icon)| {
							html! {
								<div class="flex flex-col items-center bg-white p-6 rounded-lg shadow-lg transform hover:scale-105 transition duration-300">
									<img src={icon.clone()} alt={*skill} class="w-12 h-12 mb-4" />
									<span class="text-lg font-semibold text-gray-700">{ skill }</span>
								</div>
							}
						}) }
			</div>
			// Projects Section
			<h2 class="text-2xl font-bold mt-10 mb-4 text-gray-800">{ "Projects" }</h2>
			<div class="grid grid-cols-1 md:grid-cols-2 gap-6 mb-10">
				{ if *projects_loading {
							html! { <div class="text-gray-600">{"Loading projects..."}</div> }
						} else {
							html! {
								<>
									{ for projects_section }
								</>
							}
						} }
			</div>
			// Experience Section
			<h2 class="text-2xl font-bold mt-10 mb-4 text-gray-800">{ "Experience" }</h2>
			<div class="bg-white p-6 rounded-lg shadow-lg mb-10">
				<h3 class="text-xl font-semibold text-gray-800">{ "Job Title at Company" }</h3>
				<p class="text-gray-600 mt-2">{ "Description of your role and contributions." }</p>
				<p class="text-gray-500 text-sm">{ "Dates of Employment" }</p>
			</div>
			// Education Section
			<h2 class="text-2xl font-bold mt-10 mb-4 text-gray-800">{ "Education" }</h2>
			<div class="bg-white p-6 rounded-lg shadow-lg mb-10">
				<h3 class="text-xl font-semibold text-gray-800">{ "Degree, Major at University" }</h3>
				<p class="text-gray-500 text-sm">{ "Graduation Year" }</p>
			</div>
			// Testimonials Section
			<h2 class="text-2xl font-bold mt-10 mb-4 text-gray-800">{ "Testimonials" }</h2>
			<blockquote class="bg-gray-200 p-4 rounded-lg mb-10">
				<p class="text-gray-700 italic">
					{ "Shawn is a dedicated developer with a keen eye for detail." }
				</p>
				<footer class="mt-2 text-gray-500">{ "— Person Name, Position" }</footer>
			</blockquote>
			// Contact Information
			<h2 class="text-2xl font-bold mt-10 mb-4 text-gray-800">{ "Contact Me" }</h2>
			<p class="text-gray-700">{ "Feel free to reach out!" }</p>
			<a href="mailto:your-email@example.com" class="text-blue-500 hover:underline">
				{ "your-email@example.com" }
			</a>
		</div>
	}
}
