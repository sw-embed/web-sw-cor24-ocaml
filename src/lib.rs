use yew::prelude::*;

pub struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <main class="page">
                <h1>{ "web-sw-cor24-ocaml" }</h1>
                <p class="placeholder">
                    { "Scaffold placeholder. The OCaml interpreter UI \
                       lands in a later saga step." }
                </p>
                <p class="build">
                    { format!("{} \u{00b7} {} \u{00b7} {}",
                        env!("BUILD_HOST"),
                        env!("BUILD_SHA"),
                        env!("BUILD_TIMESTAMP"),
                    ) }
                </p>
            </main>
        }
    }
}
