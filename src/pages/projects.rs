use yew::prelude::*;

#[function_component(Projects)]
pub fn projects() -> Html {
    html! {
        <div>
            <h1>{ "Projects" }</h1>
            <p>{ "List your cool Rust projects here." }</p>
        </div>
    }
}
