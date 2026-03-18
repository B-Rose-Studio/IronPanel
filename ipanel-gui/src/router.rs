use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[route("/")]
    Home {},
}

/// Home page
#[component]
fn Home() -> Element {
    rsx! {
        div { class: "home-container",
            button { "Cancelar" }
            button { "Prosseguir" }
        }
    }
}
