use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct HomeProps {}

#[function_component]
pub fn Home(props: &HomeProps) -> Html {
    let HomeProps {} = props;
    html! { <div class="">{ "Home" }</div> }
}
