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
                    class: "flex flex-row place-content-center space-x-4 mx-auto px-4 md:px-16 ",
                    div {
                        class: "place-content-center",
                        img {
                            src: "HoP UKGGNPL 32x24.png",
                            class: "self-stretch"
                        }
                    }
                    div {
                        class: "flex-auto text-2xl", "House of Parliament"
                    }
                }
            }
            div {
                class: "flex max-h-52 justify-stretch",
                img {
                    src: "westminster palace.jpg",
                    class: "object-coverr"
                }
            }
        }
    }
}
