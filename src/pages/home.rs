use crate::components::header::Header;
use crate::components::sidebar::Sidebar;
use yew::{function_component, html, Html, Properties};
#[function_component]
pub fn Home() -> Html {
    html! {
        <div class="container">
            <div class="row">
                <div class="col">
                    <Sidebar />
                </div>
                <div class="col mt-3">
                    <Header />
                    {"Have a great day"}
                </div>
            </div>
        </div>
    }
}
