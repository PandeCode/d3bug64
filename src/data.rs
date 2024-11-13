use yew::prelude::*;

use crate::utils::svg_asset;

#[function_component]
pub(crate) fn Description() -> Html {
	html! {
	<div class="parallax-container p-10 rounded-lg shadow-md mb-8 max-width max-height">
		<div class="parallax-bg"></div>
		<div class="foreground-content flex flex-col justify-between  ">
			<h2 class="text-5xl font-bold text-blue-600 dark:text-blue-400">{ "Shawn Pande" }</h2>
			<p class="text-gray-700 dark:text-gray-300 text-xl mt-4 leading-relaxed max-w-3xl mx-auto">
				{ "A developer with a knack for overcomplicating simple projects and a passion for hacking together quirky, sometimes unnecessary tools. Embraces challenges that transform mundane tasks into meticulously crafted systems. For example, this very page could've been a quick HTML mock-up, but instead, it's a full-blown Rust and WebAssembly experience." }
			</p>
		</div>
	</div>

	}
}

pub(crate) fn get_skills() -> (
	Vec<(String, String)>,
	Vec<(String, String)>,
	Vec<(String, String)>,
) {
	(
		vec![
			("Full-Stack Dev".to_string(), svg_asset("web")),
			("Graphics Programming".to_string(), svg_asset("graphics")),
		],
		vec![
			("Rust".to_string(), svg_asset("rust")),
			("C/C++".to_string(), svg_asset("cpp")),
			("Python".to_string(), svg_asset("python")),
			("Bash".to_string(), svg_asset("bash")),
		],
		vec![
			("Linux".to_string(), svg_asset("linux")),
			("Neovim".to_string(), svg_asset("neovim")),
			("Git".to_string(), svg_asset("git")),
			("Blender".to_string(), svg_asset("blender")),
			("Godot".to_string(), svg_asset("godot")),
			("Unity".to_string(), svg_asset("unity")),
		],
	)
}

pub(crate) const ACC: &str = "pandecode";
