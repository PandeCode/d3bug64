use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct AboutProps {}

#[function_component]
pub fn About(props: &AboutProps) -> Html {
    let AboutProps {} = props;
    html! { <div>{ "Built with Yew, Trunk, Rust, Tailwind" }</div> }
}
