use crate::contexts::CurrentUserProvider;
use crate::pages::crates::delete::CratesDelete;
use crate::pages::crates::edit::CratesEdit;
use pages::crates::add::*;
use pages::crates::index::Crates;
use pages::home::Home;
use pages::login::Login;
use pages::not_found::NotFound;
use pages::rustaceans::add::*;
use pages::rustaceans::delete::*;
use pages::rustaceans::edit::*;
use pages::rustaceans::index::Rustaceans;
use yew::prelude::*;
use yew_router::prelude::*;

mod api;
mod components;
mod contexts;
mod hooks;
mod pages;
mod utils;

#[derive(Routable, PartialEq, Clone)]
pub enum Route {
    #[at("/")]
    Main,
    #[at("/home")]
    Home,
    #[at("/crates")]
    Crates,
    #[at("/crates/add")]
    CratesAdd,
    #[at("/crates/:id/edit")]
    CratesEdit { id: i32 },
    #[at("/crates/:id/delete")]
    CratesDelete { id: i32 },
    #[at("/login")]
    Login,
    #[not_found]
    #[at("/404")]
    NotFound,
    #[at("/rustaceans")]
    Rustaceans,
    #[at("/rustaceans/add")]
    RustaceansAdd,
    #[at("/rustaceans/:id/edit")]
    RustaceansEdit { id: i32 },
    #[at("/rustaceans/:id/delete")]
    RustaceansDelete { id: i32 },
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <HashRouter>
                <CurrentUserProvider>
                    <Switch<Route> render={switch}/>
                </CurrentUserProvider>
            </HashRouter>
        </>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Main => html! {
            <Home />
        },
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
        Route::RustaceansEdit { id } => html! {
            <RustaceansEdit rustacean_id={id} />
        },
        Route::RustaceansDelete { id } => html! {
            <RustaceansDelete rustacean_id={id} />
        },
        Route::Crates => html! {
            <Crates />
        },
        Route::CratesAdd => html! {
            <CratesAdd />
        },
        Route::CratesEdit { id } => html! {
            <CratesEdit crate_id={id} />
        },
        Route::CratesDelete { id } => html! {
            <CratesDelete crate_id={id} />
        },
        /*
        Route::CratesDelete { id } => html! {
            <CratesDelete crate_id={id} />
        },*/
        _ => html! {
            <Home />
        },
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
