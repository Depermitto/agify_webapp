use yew::{function_component, Html, html};
use agify::Agify;

mod counter;
mod agify;

#[function_component]
fn App() -> Html {
    html! {
        <>
            <Agify/>
        </>
    }
}


fn main() {
    yew::Renderer::<App>::new().render();
}