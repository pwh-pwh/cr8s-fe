use crate::contexts::CurrentUserProvider;
use pages::home::Home;
use pages::login::Login;
use pages::not_found::NotFound;
use pages::rustaceans::add::RustaceansAdd;
use pages::rustaceans::index::Rustaceans;
use yew::prelude::*;
use yew_router::prelude::*;

mod api;
mod components;
mod contexts;
mod pages;
mod utils;

#[derive(Routable, PartialEq, Clone)]
pub enum Route {
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
    #[at("/rustaceans/add")]
    RustaceansAdd,
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
        Route::Rustaceans => html! {
            <Rustaceans />
        },
        Route::RustaceansAdd => html! {
            <RustaceansAdd />
        },
        _ => html! {
            <Home />
        },
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
