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
