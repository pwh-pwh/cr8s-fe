use crate::components::header::Header;
use crate::components::rustacean_form::RustaceanForm;
use crate::components::rustacean_list::RustaceanList;
use crate::components::sidebar::Sidebar;
use yew::{function_component, html, Html};

#[function_component(RustaceansAdd)]
pub fn rustaceans_add() -> Html {
    html! {
        <div class="container">
            <div class="row">
                <div class="col-sm-auto">
                    <Sidebar />
                </div>
                <div class="col mt-3">
                    <Header />
                    <RustaceanForm />
                </div>
            </div>
        </div>
    }
}
