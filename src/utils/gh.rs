use std::{collections::HashMap, time::Duration};

use crate::data::ACC;

use super::github_projects_obj;
use gloo_net::http::Request;
use gloo_timers::future::sleep;
use web_sys::js_sys::Date;

const CACHE_EXPIRY_MS: f64 = 24.0 * 60.0 * 60.0 * 1000.0; // 1 day in milliseconds
const CACHE_DELAY_MS: u64 = 0; // To simulate loading, not necessay for now

pub async fn request_get_cache(url: &str) -> Option<String> {
	if let Some(window) = web_sys::window() {
		if let Ok(local_storage) = window.local_storage() {
			if let Some(local_storage) = local_storage {
				// Retrieve cached data
				if let Ok(Some(item)) = local_storage.get_item(url) {
					sleep(Duration::from_millis(CACHE_DELAY_MS)).await; // adjust duration as needed

					// Split the timestamp and data
					if let Some((timestamp_str, data)) = item.split_once('|') {
						if let Ok(timestamp) = timestamp_str.parse::<f64>() {
							// Check if the data is still valid
							let current_time = Date::now();
							if current_time - timestamp < CACHE_EXPIRY_MS {
								return Some(data.to_string());
							} else {
								// Remove the old data and proceed
								if let Err(err) = local_storage.remove_item(url) {
									weblog::console_warn!(
										"Failed to remove old data from the cache"
									);
								}
							}
						}
					}
				}

				// If cache is expired or doesn't exist, fetch fresh data
				if let Ok(response) = Request::get(&url).send().await {
					if let Ok(text) = response.text().await {
						// Prepend the current timestamp to the data
						let timestamped_data = format!("{}|{}", Date::now(), text);
						// Store the new data in local storage
						if let Err(err) = local_storage.set_item(url, &timestamped_data) {
							weblog::console_warn!("Failed to store in local storage:", err);
						}
						return Some(text);
					}
				}
			} else {
				weblog::console_warn!("LocalStorage not available");
			}
		} else {
			weblog::console_warn!("LocalStorage not found");
		}
	} else {
		weblog::console_warn!("Window not found");
	}

	None
}

pub async fn get_repos(account: String) -> Option<github_projects_obj::Root> {
	let url = format!("https://api.github.com/users/{}/repos", account);
	if let Some(data) = request_get_cache(&url).await {
		serde_json::from_str(&data).ok()
	} else {
		None
	}
}
pub async fn get_blog_files() -> Option<github_projects_obj::Root> {
	let url = format!("https://api.github.com/users/{}/repos", ACC);
	if let Some(data) = request_get_cache(&url).await {
		serde_json::from_str(&data).ok()
	} else {
		None
	}
}

pub async fn get_repo_file(account: &str, repo: &str, default_branch: &str, filename: &str) -> Option<String> {
	let url = format!(
		"https://raw.githubusercontent.com/{}/{}/{}/{}",
		account, repo, default_branch, filename
	);
	request_get_cache(&url).await
}

pub async fn get_readme(account: &str, repo: &str, default_branch: &str) -> Option<String> {
	get_repo_file(account, repo, default_branch, "README.md").await
}

pub async fn get_langs(account: String, repo: String) -> Option<HashMap<String, u64>> {
	let url = format!(
		"https://api.github.com/repos/{}/{}/languages",
		account, repo
	);

	if let Some(data) = request_get_cache(&url).await {
		let languages: HashMap<String, u64> = serde_json::from_str(&data).ok()?;
		Some(languages)
	} else {
		None
	}
}

pub async fn get_topics(account: &str, repo: &str) -> Option<Vec<String>> {
	let url = format!("https://api.github.com/repos/{}/{}/topics", account, repo);
	let data = request_get_cache(&url).await?;
	let topics: serde_json::Value = serde_json::from_str(&data).ok()?;
	let names_array = topics.get("names")?.as_array()?;

	Some(
		names_array
			.iter()
			.filter_map(|n| n.as_str().map(|s| s.to_string()))
			.collect(),
	)
}
