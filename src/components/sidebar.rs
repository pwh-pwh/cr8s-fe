use crate::Route;
use yew::{classes, function_component, html, Html, Properties};
use yew_router::prelude::*;

#[function_component(Sidebar)]
pub fn sidebar() -> Html {
    let current_route = use_route::<Route>().expect("not current route");
    let home_classes = if current_route == Route::Home {
        classes!("nav-link", "active")
    } else {
        classes!("nav-link")
    };
    let rustaceans_classes = if current_route == Route::Rustaceans {
        classes!("nav-link", "active")
    } else {
        classes!("nav-link")
    };
    let crates_classes = if current_route == Route::Crates {
        classes!("nav-link", "active")
    } else {
        classes!("nav-link")
    };
    html! {
        <nav class="navbar navbar-light">
            <ul class="nav navbar-nav">
                <li class="nav-item">
                    <Link<Route> to={Route::Home} classes={home_classes}>{"Home"}</Link<Route>>
                </li>
                <li class="nav-item">
                    <Link<Route> to={Route::Rustaceans} classes={rustaceans_classes}>{"Rustaceans"}</Link<Route>>
                </li>
                <li class="nav-item">
                    <Link<Route> to={Route::Crates} classes={crates_classes}>{"Crates"}</Link<Route>>
                </li>
            </ul>
        </nav>
    }
}
