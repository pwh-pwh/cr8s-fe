use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub label: AttrValue,
    pub name: AttrValue,
    pub value: AttrValue,
    pub onchange: Callback<Event>,
}

#[function_component(Textarea)]
pub fn textarea(props: &Props) -> Html {
    let html_id = format!("edit-{}", props.name);
    html! {
        <>
            <label for={html_id.clone()} class="form-label">{props.label.clone()}</label>
            <textarea id={html_id}
        class="form-control"
        name={props.name.clone()}
        value={props.value.clone()}
        onchange={props.onchange.clone()}
        />
        </>
    }
}
