mod components;
mod pages;
mod services;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/cardapio")]
    Menu,
    #[at("/pedidos")]
    Orders,
    #[at("/carrinho")]
    Cart,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <pages::Home /> },
        Route::Menu => html! { <pages::Menu /> },
        Route::Orders => html! { <pages::Orders /> },
        Route::Cart => html! { <pages::Cart /> },
        Route::NotFound => html! { <h1>{ "404 - Página não encontrada" }</h1> },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <div class="app">
                <nav class="navbar">
                    <h1>{ "?? DelPopolo Panificadora" }</h1>
                    <div class="nav-links">
                        <Link<Route> to={Route::Home}>{ "Home" }</Link<Route>>
                        <Link<Route> to={Route::Menu}>{ "Cardápio" }</Link<Route>>
                        <Link<Route> to={Route::Orders}>{ "Meus Pedidos" }</Link<Route>>
                        <Link<Route> to={Route::Cart}>{ "Carrinho" }</Link<Route>>
                    </div>
                </nav>
                
                <main>
                    <Switch<Route> render={switch} />
                </main>
                
                <footer>
                    <p>{ "© 2025 DelPopolo Panificadora - Powered by Avila Framework" }</p>
                </footer>
            </div>
        </BrowserRouter>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
