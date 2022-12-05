use yew::prelude::*;

#[function_component(MainLogin)]
pub fn main_login() -> Html {
    let is_login = use_state(|| true);
    let title = html! {
        <>
            <h1><u>{"Ours"}</u></h1>
            <p>{"lama"}</p>
        </>
    };

    if *is_login {
        return html!(
            <div>
                {title}
            </div>
        );
    }

    html!(
        <div>
            <p>{"Hello World"}</p>
        </div>
    )
}
