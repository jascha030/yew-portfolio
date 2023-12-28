use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <h1 class={classes!("text-lg")}>{ "Test" }</h1>
        </main>
    }
}
