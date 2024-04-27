#![allow(non_snake_case)]

use dioxus::prelude::*;
use tracing::Level;

const _TAILWIND_URL: &str = manganis::mg!(file("public/tailwind.css"));

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}


#[component]
fn Home() -> Element {
    rsx! {
        div {
            div {
                class: "p-5",
                div {
                    class: "flex flex-col mx-auto px-16",
                    div {
                        class: "flex-initial px-1",
                        img {
                            src: "HoP UKGGNPL 32x24.png",
                            class: "size-6"
                        }
                    }
                    div {
                        class: "flex-auto text-2xl", "House of Parliament"
                    }
                }
            }
        }
    }
}
