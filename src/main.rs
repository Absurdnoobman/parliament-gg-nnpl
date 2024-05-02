#![allow(non_snake_case)]

use dioxus:: prelude::*;
use tracing::Level;

const _TAILWIND_URL: &str = manganis::mg!(file("public/tailwind.css"));

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/:..segments")]
    ErrorNotFound {segments : Vec<String>}
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
fn  ErrorNotFound(segments: Vec<String>) -> Element {
    let mut text : String = String::new();
    if !(segments.is_empty()) {
        text = segments.join(", ")
    }
    rsx! {
        div {
            Header {}
            div {
                class: "px-20 mt-4",
                div {
                    class: "text-2xl",
                    "Sorry, Page Not Found"
                }
                div {
                    class: "mt-2 text-lg",
                    "The Page '{text}' can not be found inside the parliament website"
                }
                Link {
                    class: "hover:underline",
                    to: Route::Home {}, "Go back to the Website",
                }
            }
        }
    }
}

#[component]
fn Header() -> Element {
    rsx! {
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
    }
}

#[component]
fn Home() -> Element {
    rsx! {
        div {
            Header {}
            div {
                class: "flex h-80",
                img {
                    src: "westminster palace.jpg",
                    class: "object-cover flex-grow"
                }
            }
            div {
                class: "px-20 py-4 space-y-1 border-t-2 border-gray-400 mt-4",
                div {
                    class: " text-2xl ",
                    "Welcome"
                }
                div {
                    class: "mt-1 pb-5 text-sm",
                    "Welcome to the official website for UK of GG and NNPL House of Parliament (HoP)."
                }
                div {
                    class: " mt-4 p-2 shadow-lg",
                    div {
                        class: "text-2xl px-3 pb-2 border-b-2 border-gray-300",
                        "News"
                    }
                }
                div {
                    class: "mt-4 p-2 shadow-lg",
                    div {
                        class: "text-2xl px-3 pb-2 border-b-2 border-gray-300",
                        "Links"
                    }
                    div {
                        class: "flex flex-col space-y-2",
                        div {
                            class: "flex flex-row space-x-1 mx-auto",
                            img {
                                src: "HoC 64x64.png",
                                class: "p-1 place-content-center"
                            }
                            Link {
                                to: Route::Home {},
                                class: "hover:underline place-content-center",
                                "House of Commons"
                            }
                        }
                    }
                }
            }
        }
    }
}
