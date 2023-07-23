use crate::contexts::CurrentUserProvider;
use pages::home::Home;
use pages::login::Login;
use pages::not_found::NotFound;
use yew::prelude::*;
use yew_router::prelude::*;

mod api;
mod components;
mod contexts;
mod pages;

#[derive(Routable, PartialEq, Clone)]
enum Route {
    #[at("/home")]
    Home,
    #[at("/crates")]
    Crates,
    #[at("/login")]
    Login,
    #[not_found]
    #[at("/404")]
    NotFound,
    #[at("/rustaceans")]
    Rustaceans,
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <BrowserRouter>
                <CurrentUserProvider>
                    <Switch<Route> render={switch}/>
                </CurrentUserProvider>
            </BrowserRouter>
        </>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Login => html! {
            <Login />
        },
        Route::Home => html! {
            <Home />
        },
        Route::NotFound => html! {
            <NotFound />
        },
        _ => html! {
            <Home />
        },
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
