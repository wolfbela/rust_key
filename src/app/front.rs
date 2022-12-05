pub mod login;
pub mod login_page;

use login_page::MainLogin;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let _is_login = use_state(|| true);

    html!(
        <MainLogin />
    )
}
