use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct HomeProps {}

macro_rules! iter_to_html {
    ($iter:expr, $html:expr) => {
        iter.iter().map(|i| <>$html</>).collect::<yew::html::Html>()
    };
}

#[function_component]
pub fn Home(props: &HomeProps) -> Html {
    let HomeProps {} = props;

    let skills = [vec!["Rust"], vec!["Fullstack"], vec!["C++"], vec!["Python"]];
    html! { <div class="">
    {
        skills.iter().map(|s| s.iter().collect::<yew::html::Html>()).collect::<yew::html::Html>()
    }

    </div> }
}
