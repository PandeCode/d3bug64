pub(crate) mod gh;
pub(crate) mod github_projects_obj;

pub fn set_href(url: String) -> Result<(), wasm_bindgen::JsValue> {
	web_sys::window()
		.expect("Missing Window")
		.location()
		.set_href(url.as_str())
}

pub fn gh_repo_url(account: String, repo: String) -> String {
	format!("https://github.com/{}/{}", account, repo)
}

pub fn language_color(lang: &str) -> &'static str {
	match lang {
		"HTML" => "#E44D26",       // Bright orange
		"TypeScript" => "#007ACC", // Darker blue
		"Shell" => "#4EAA1F",      // Bright green
		"Lua" => "#5B82A1",        // Lighter blue
		"JavaScript" => "#F7DF1E", // Bright yellow
		"Rust" => "#B5742D",       // Lighter rust
		"C++" => "#FF7F50",        // Coral
		"CSS" => "#264de4",        // Blue
		"C" => "#6A6A6A",          // Gray
		"Python" => "#3572A5",     // Blue
		"Haskell" => "#7B2B77",    // Purple
		"Awk" => "#AAB1D9",        // Light blue
		"GLSL" => "#B4C8D4",       // Soft cyan
		"Makefile" => "#2A6344",   // Dark green
		"Roff" => "#BEC3C7",       // Light gray
		"CMake" => "#4A4A4A",      // Dark gray
		"Vim Script" => "#17B168", // Bright teal
		"Nix" => "#B0B87D",        // Light olive
		// Add more languages as needed
		_ => "#000000", // Default color if not found
	}
}

pub fn svg_asset(f: &str) -> String {
	format!("assets/svgs/{}.svg", f)
}
