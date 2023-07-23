use yew::{function_component, html, AttrValue, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct AlertProps {
    pub message: AttrValue,
    pub alert_type: AttrValue,
}

#[function_component]
pub fn Alert(props: &AlertProps) -> Html {
    html! {
        <div class={format!("alert alert-{}",props.alert_type)} role="alert">
            {props.message.clone()}
        </div>
    }
}
