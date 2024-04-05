use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::Route;

#[derive(PartialEq, Properties)]
pub struct NavBarProps {}

#[function_component]
pub fn NavBar(props: &NavBarProps) -> Html {
    let navigator = use_navigator().unwrap();

    let create_route_callback = |route| {
        let navigator = navigator.clone();
        move |_| navigator.push(route)
    };

    let onclick_home = create_route_callback(&Route::Home);
    let onclick_about = create_route_callback(&Route::About);
    let onclick_projects = create_route_callback(&Route::Projects);

    let show_dropdown = use_state(|| false);
    let toggle_dropdown = {
        let show_dropdown = show_dropdown.clone();
        move |_| show_dropdown.set(!*show_dropdown)
    };

    let NavBarProps {} = props;

    let blue = "#ff00ff";

    html! {
        <nav class="bg-white border-gray-200 dark:bg-gray-900">
            <div class="max-w-screen-xl flex flex-wrap items-center justify-between mx-auto p-4">
                <a href="#" class="flex items-center space-x-3 rtl:space-x-reverse">
                    <img src="assets/favicon-32x32.png" class="h-8" alt="d3bug64 logo" />
                    <span
                        class="self-center text-2xl font-semibold whitespace-nowrap dark:text-white"
                    >
                        { "d3bug64" }
                    </span>
                </a>
                <button
                    data-collapse-toggle="navbar-default"
                    type="button"
                    onclick={toggle_dropdown}
                    class="inline-flex items-center p-2 w-10 h-10 justify-center text-sm text-gray-500 rounded-lg md:hidden hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-gray-200 dark:text-gray-400 dark:hover:bg-gray-700 dark:focus:ring-gray-600"
                    aria-controls="navbar-default"
                    aria-expanded="false"
                >
                    <span class="sr-only">{ "Open main menu" }</span>
                    <svg
                        class="w-5 h-5"
                        aria-hidden="true"
                        xmlns="http://www.w3.org/2000/svg"
                        fill="none"
                        viewBox="0 0 17 14"
                    >
                        <path
                            stroke="currentColor"
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            stroke-width="2"
                            d="M1 1h15M1 7h15M1 13h15"
                        />
                    </svg>
                </button>
                <div class="hidden w-full md:block md:w-auto" id="navbar-default">
                    <ul
                        class="font-medium flex flex-col p-4 md:p-0 mt-4 border border-gray-100 rounded-lg bg-gray-50 md:flex-row md:space-x-8 rtl:space-x-reverse md:mt-0 md:border-0 md:bg-white dark:bg-gray-800 md:dark:bg-gray-900 dark:border-gray-700"
                    >
                        <li>
                            <a
                                href="#"
                                onclick={onclick_home}
                                class="nav-btn-active"
                                aria-current="page"
                            >
                                { "Home" }
                            </a>
                        </li>
                        <li>
                            <a href="#" onclick={onclick_projects} class="nav-btn">
                                { "Projects" }
                            </a>
                        </li>
                        <li>
                            <a href="#" onclick={onclick_about} class="nav-btn">{ "About" }</a>
                        </li>
                        <li>
                            <a
                                href="https://ug.linkedin.com/in/shawn-pande-45a79b19b"
                                class="nav-btn"
                            >
                                { "LinkedIn" }
                            </a>
                        </li>
                        <li>
                            <a href="https://github.com/PandeCode/" class="nav-btn">{ "Github" }</a>
                        </li>
                    </ul>
                </div>
            </div>
        </nav>
    }
}
