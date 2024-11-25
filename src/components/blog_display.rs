use crate::utils::safe_html::MarkdownHTML;
use weblog::{console_error, console_info};
use yew::prelude::*;

use crate::utils;

#[derive(PartialEq, Properties)]
pub struct BlogDisplayProps {
	pub path: String, // Path representing the blog's ID or unique identifier
}

#[function_component]
pub fn BlogDisplay(props: &BlogDisplayProps) -> Html {
	let BlogDisplayProps { path } = props;

	// State for blog content and loading status
	let blog_text = use_state(|| String::new());
	let title = use_state(|| String::new());
	let date = use_state(|| String::new());
	let tags = use_state(|| Vec::<String>::new());
	let blog_text_loading = use_state(|| true);

	{
		let blog_text = blog_text.clone();
		let blog_text_loading = blog_text_loading.clone();
		let path = path.clone();

		let title = title.clone();
		let date = date.clone();
		let tags = tags.clone();

		use_effect_with((), move |_| {
			let blog_text = blog_text.clone();
			let blog_text_loading = blog_text_loading.clone();

			let title = title.clone();
			let date = date.clone();
			let tags = tags.clone();

			wasm_bindgen_futures::spawn_local(async move {
				if let Some(content) = utils::gh_blog::get_blog_text(&path).await {
					blog_text.set(content);

					if let Some(data_file) = utils::gh_blog::get_data_file().await {
						data_file.iter().for_each(|blog| {
							if blog.id == path {
								title.set(blog.title.clone());
								tags.set(blog.tags.clone());
								date.set(blog.date.clone());
							}
						})
					}

					blog_text_loading.set(false);
				} else {
					console_error!("Failed to fetch blog content for path: {}", path);
				}
			});

			|| ()
		});
	}

	let history = web_sys::window().unwrap().history().unwrap();

	html! {
		<div class="p-8 bg-gradient-to-r from-blue-100 to-purple-200 dark:bg-gradient-to-r dark:from-gray-800 dark:to-gray-900 min-h-screen">
			<div class="max-w-4xl mx-auto bg-white dark:bg-gray-800 p-6 rounded-lg shadow-lg">
				<div class="mb-4">
					<button
						onclick={Callback::from(move |_| { let _ = history.back(); } )}
						class="text-blue-500 hover:underline dark:text-blue-400 text-sm"
					>
						{ "← Back to Blogs" }
					</button>
				</div>
				{
					if *blog_text_loading {
						html! {
							<div class="p-6 bg-gray-200 dark:bg-gray-800 rounded-lg animate-pulse">
								<div class="h-6 bg-gray-300 dark:bg-gray-700 rounded w-1/2 mb-4"></div>
								<div class="h-4 bg-gray-300 dark:bg-gray-700 rounded w-full mb-2"></div>
								<div class="h-4 bg-gray-300 dark:bg-gray-700 rounded w-5/6 mb-2"></div>
								<div class="h-4 bg-gray-300 dark:bg-gray-700 rounded w-4/5"></div>
							</div>
						}
					} else {
						html! {
							<>
								<h1 class="text-4xl font-bold text-gray-800 dark:text-gray-100 mb-6 text-center">{ (*title).clone() }</h1>
								<p class="text-gray-600 dark:text-gray-400 text-center mb-4">{ (*date).clone() }</p>
								<div class="flex justify-center mb-6">
									{
										for (*tags).iter().map(|tag| {
											html! {
												<span class="bg-blue-200 dark:bg-blue-700 text-blue-800 dark:text-blue-200 text-xs font-semibold mr-2 px-2.5 py-0.5 rounded">{ tag }</span>
											}
										})
									}
								</div>
								<div class="prose dark:prose-invert">
									<MarkdownHTML markdown={ (*blog_text).clone() } />
								</div>
							</>
						}
					}
				}
			</div>
		</div>
	}
}
