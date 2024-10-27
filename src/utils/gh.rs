use gloo_net::http::Request;

use super::github_projects_obj;

// TODO: Implement some sort of caching

pub async fn get_repos(account: String) -> Option<github_projects_obj::Root> {
	let url = format!("https://api.github.com/users/{}/repos", account);

	if let Ok(response) = Request::get(&url).send().await {
		if let Ok(json) = response.json().await {
			return Some(json);
		}
	}

	None
}

pub async fn get_readme(repo: String, default_branch: String) -> Option<String> {
	let url = format!(
		"https://raw.githubusercontent.com/{}/{}/README.md",
		repo, default_branch
	);

	if let Ok(response) = Request::get(&url).send().await {
		if let Ok(text) = response.text().await {
			return Some(text);
		}
	}

	None
}

pub async fn get_langs(account: String, repo: String) -> Option<Vec<String>> {
	let url = format!(
		"https://api.github.com/repos/{}/{}/languages",
		account, repo
	);

	if let Ok(response) = Request::get(&url).send().await {
		if let Ok(json) = response.json().await {
			return Some(json);
		}
	}

	None
}
