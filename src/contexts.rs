use crate::api::user::{LoginResponse, MeResponse, User};
use std::rc::Rc;
use yew::{
    function_component, html, use_reducer, Children, ContextProvider, Html, Properties, Reducible,
    UseReducerHandle,
};

pub type CurrentUserContext = UseReducerHandle<CurrentUser>;

#[derive(Default, PartialEq)]
pub struct CurrentUser {
    pub user: Option<User>,
    pub token: Option<String>,
}

impl Reducible for CurrentUser {
    type Action = CurrentDispatchActions;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action.action_type {
            CurrentUserActions::LoginSuccess => {
                let login_resp = action.login_response.expect("missing login_resp");
                let me_resp = action.me_response.expect("missing me_resp");
                Self {
                    user: Some(me_resp.into()),
                    token: Some(login_resp.token),
                }
                .into()
            }
            CurrentUserActions::LoginFail => Self {
                user: None,
                token: None,
            }
            .into(),
        }
    }
}

pub struct CurrentDispatchActions {
    pub action_type: CurrentUserActions,
    pub login_response: Option<LoginResponse>,
    pub me_response: Option<MeResponse>,
}

pub enum CurrentUserActions {
    LoginSuccess,
    LoginFail,
}

#[derive(PartialEq, Properties)]
pub struct Props {
    pub children: Children,
}

#[function_component(CurrentUserProvider)]
pub fn current_user_provider(props: &Props) -> Html {
    let user = use_reducer(CurrentUser::default);
    html! {
        <ContextProvider<CurrentUserContext> context={user}>
            {props.children.clone()}
        </ContextProvider<CurrentUserContext>>
    }
}
