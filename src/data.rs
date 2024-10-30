use yew::prelude::*;

use crate::utils::svg_asset;

pub(crate) fn get_description() -> Html {
	html! {
			<>
				{"A developer with a knack for overcomplicating simple projects and a passion for hacking together quirky, sometimes unnecessary tools. Embraces challenges that transform mundane tasks into meticulously crafted systems. For example, this very page could've been a quick HTML mock-up, but instead, it's a full-blown Rust and WebAssembly experience."}
			</>

	}
}

pub(crate) fn get_skills() -> Vec<(String, String)> {
	vec![
		("Full-Stack Dev".to_string(), svg_asset("web")),
		("Graphics Programming".to_string(), svg_asset("graphics")),
	]
}

pub(crate) fn get_langs() -> Vec<(String, String)> {
	vec![
		("Rust".to_string(), svg_asset("rust")),
		("C/C++".to_string(), svg_asset("cpp")),
		("Python".to_string(), svg_asset("python")),
		("Bash".to_string(), svg_asset("bash")),
	]
}

pub(crate) fn get_tools() -> Vec<(String, String)> {
	vec![
		("Linux".to_string(), svg_asset("linux")),
		("Neovim".to_string(), svg_asset("neovim")),
		("Git".to_string(), svg_asset("git")),
		("Blender".to_string(), svg_asset("blender")),
		("Godot".to_string(), svg_asset("godot")),
		("Unity".to_string(), svg_asset("unity")),
	]
}
