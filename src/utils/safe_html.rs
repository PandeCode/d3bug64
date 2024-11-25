use yew::{function_component, Html, Properties};

use comrak::plugins::syntect::SyntectAdapterBuilder;
use comrak::{markdown_to_html_with_plugins, Options, Plugins};

use comrak::{
	adapters::{HeadingAdapter, HeadingMeta},
	nodes::Sourcepos,
};
use std::io::{self, Write};

#[derive(Properties, PartialEq)]
pub struct Props {
	pub markdown: String,
}

#[function_component(MarkdownHTML)]
pub fn safe_html(props: &Props) -> Html {
	let builder = SyntectAdapterBuilder::new().theme("base16-ocean.dark");
	//let builder = SyntectAdapterBuilder::new().theme("Solarized (dark)");
	//let builder = SyntectAdapterBuilder::new().theme("base16-mocha.dark");
	//let builder = SyntectAdapterBuilder::new().theme("base16-eighties.dark");

	let adapter = builder.build();
	let mut options = Options::default();

	options.extension.strikethrough = true;
	options.extension.table = true;
	options.extension.math_code = true;

	let mut plugins = Plugins::default();
	plugins.render.codefence_syntax_highlighter = Some(&adapter);

	let adapter = CustomHeadingAdapter;
	plugins.render.heading_adapter = Some(&adapter);

	let formatted =
		markdown_to_html_with_plugins(props.markdown.clone().as_str(), &options, &plugins)
		.replace("<p>", "<p class=\"text-gray-700 dark:text-gray-300 text-xl mt-4 leading-relaxed max-w-3xl mx-auto p-4\">")
		.replace("<hr>", "<hr class=\"h-px my-8 bg-gray-200 border-0 dark:bg-gray-700 p-10\">")
		.replace("<ul>", "<ul class=\"list-disc list-inside text-gray-700 dark:text-gray-300 mt-4 pl-6\">")
		.replace("<ol>", "<ol class=\"list-decimal list-inside text-gray-700 dark:text-gray-300 mt-4 pl-6\">")
		.replace("<li>", "<li class=\"mt-2 pl-2\">")
		.replace("<table>", "<table class=\"min-w-full bg-white dark:bg-gray-800 p-4\">")
		.replace("<thead>", "<thead class=\"bg-gray-200 dark:bg-gray-700 p-4\">")
		.replace("<tbody>", "<tbody class=\"bg-white dark:bg-gray-800 p-4\">")
		.replace("<tr>", "<tr class=\"border-b border-gray-200 dark:border-gray-700 p-4\">")
		.replace("<th>", "<th class=\"px-4 py-2 text-left text-gray-700 dark:text-gray-300 p-4\">")
		.replace("<td>", "<td class=\"px-4 py-2 text-gray-700 dark:text-gray-300 p-4\">")
		.replace("<a", "<a class=\"text-blue-600 dark:text-blue-400 underline\"")
		.replace("<pre", "<pre class=\"relative bg-gray-200 dark:bg-gray-700 p-4 rounded-lg\" ")
		.replace("</pre>", "<button class=\"absolute top-2 right-2 bg-blue-500 text-white px-2 py-1 rounded transition duration-300 ease-in-out transform hover:scale-105\" onclick=\"copyToClipboard(this.parentElement)\">Copy</button><div class=\"copy-popup hidden absolute top-2 right-16 bg-green-500 text-white px-2 py-1 rounded transition transition-opacity duration-500 ease-in-out opacity-0\">Copied!</div></pre>")
	;

	let script = gloo_utils::document().create_element("script").unwrap();
	script.set_inner_html(
		r#"
 		function copyToClipboard(element) {
			const text = element.innerText;
			navigator.clipboard.writeText(text).then(function() {
				console.log('Copied to clipboard');
				const popup = element.querySelector('.copy-popup');
				popup.classList.remove('hidden');
				popup.classList.remove('opacity-0');
				popup.classList.add('opacity-100');
				setTimeout(() => {
					popup.classList.add('opacity-0');
					setTimeout(() => {
						popup.classList.add('hidden');
					}, 500); // Match the duration of the transition
				}, 2000);
			}, function(err) {
				console.error('Could not copy text: ', err);
			});
		}              
 	"#,
	);
	gloo_utils::document()
		.head()
		.unwrap()
		.append_child(&script)
		.unwrap();

	let div = gloo_utils::document().create_element("div").unwrap();
	div.set_inner_html(formatted.as_str());

	Html::VRef(div.into())
}

struct CustomHeadingAdapter;

impl HeadingAdapter for CustomHeadingAdapter {
	fn enter(
		&self,
		output: &mut dyn Write,
		heading: &HeadingMeta,
		sourcepos: Option<Sourcepos>,
	) -> io::Result<()> {
		let id = slug::slugify(&heading.content);

		let search_include = !&heading.content.contains("hide");

		write!(output, "<h{}", heading.level)?;

		if let Some(sourcepos) = sourcepos {
			write!(output, " data-sourcepos=\"{}\"", sourcepos)?;
		}

		let font_size_class = match heading.level {
			1 => "text-red-600 dark:text-red-400 text-4xl",
			2 => "text-green-600 dark:text-green-400 text-3xl",
			3 => "text-blue-600 dark:text-blue-400 text-2xl",
			4 => "text-yellow-600 dark:text-yellow-400 text-xl",
			5 => "text-purple-600 dark:text-purple-400 text-lg",
			6 => "text-pink-600 dark:text-pink-400 text-base",
			_ => "text-gray-600 dark:text-gray-400 text-base",
		};

		write!(
			output,
			" id=\"{}\" data-search-include=\"{}\" class=\"{} font-bold\"># ",
			id, search_include, font_size_class
		)
	}

	fn exit(&self, output: &mut dyn Write, heading: &HeadingMeta) -> io::Result<()> {
		write!(output, "</h{}>", heading.level)
	}
}
