use yew::prelude::*;

#[function_component(MainLogin)]
pub fn main_login() -> Html {
    html!(
        <div class="login_page">
            <h1 class="login_page">{"Login"}</h1>
            <input class="login_page username" type="username" id="username" name="username" placeholder="username" pattern="[a-zA-Z]" />
            <input class="login_page password" type="password" id="password" name="password" placeholder="password" />
            <button>{"login"}</button>
        </div>
    )
}
