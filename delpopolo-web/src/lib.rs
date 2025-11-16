use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div>
            <h1>{ "?? DelPopolo Panificadora" }</h1>
            <p>{ "Sistema em desenvolvimento..." }</p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
