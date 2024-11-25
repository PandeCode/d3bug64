use super::gh;

use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use weblog::console_info;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
	pub blogs: Vec<Blog>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Blog {
	pub title: String,
	pub date: String,
	pub id: String,
	pub tags: Vec<String>,
}

const ACCOUNT: &str = "pandecode";
const REPO: &str = "blog";
const DATA_FILE: &str = "data.json";

pub async fn get_data_file() -> Option<Vec<Blog>> {
	if let Some(data) = gh::get_repo_file(ACCOUNT, REPO, "main", DATA_FILE).await {
		if let Some(data) = serde_json::from_str::<Root>(data.as_str()).ok() {
			Some(data.blogs)
		} else {
			None
		}
	} else {
		None
	}

}

pub async fn get_blog_text(id: &str) -> Option<String> {
	gh::get_repo_file(ACCOUNT, REPO, "main", format!("{}.md", id).as_str()).await
}
