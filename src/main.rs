use yew::prelude::*;
use yew_router::prelude::*;

mod pages;
mod route;

use pages::home::Home;
use pages::projects::Projects;
use route::Route;

// Switch function
fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Projects => html! { <Projects /> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <nav style="margin-bottom: 20px;">
                <Link<Route> to={Route::Home}>{ "Home" }</Link<Route>>
                { " | " }
                <Link<Route> to={Route::Projects}>{ "Projects" }</Link<Route>>
                { " | " }
            </nav>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
