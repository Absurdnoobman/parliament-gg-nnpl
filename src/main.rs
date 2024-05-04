#![allow(non_snake_case)]

mod data;

use dioxus::prelude::*;
use tracing::Level;

const _TAILWIND_URL: &str = manganis::mg!(file("public/tailwind.css"));

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/hoc")]
    HouseOfCommons {},
    #[route("/hosl")]
    HouseOfSelectedLords {},

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
        text = segments.join("/")
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
                    class: "hover:underline mt-1",
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
                    class: " text-2xl ", "Welcome"
                }
                div {
                    class: "mt-1 pb-5 text-sm",
                    "Welcome to the official website for UK of GG and NNPL House of Parliament (HoP)."
                }
                div {
                    class: "mt-4 p-2 shadow-lg",
                    div {
                        class: "text-2xl px-3 pb-2 border-b-2 border-gray-300", "News"
                    }
                }
                div {
                    class: "mt-4 p-2 shadow-lg",
                    div {
                        class: "text-2xl px-3 pb-2 border-b-2 border-gray-300", "Links"
                    }
                    div {
                        class: "flex md:flex-row md:mt-2 max-md:flex-col max-md:space-y-2",

                        // For Small Screen
                        div { 
                            class: "md:hidden flex flex-row space-x-1 mx-auto",
                            img {
                                src: "HoC 64x64.png",
                                class: "p-1 place-content-center"
                            }
                            Link {
                                to: Route::HouseOfCommons {},
                                class: "hover:underline place-content-center", "House of Commons"
                            }
                        }
                        div {
                            class: "md:hidden flex flex-row space-x-1 mx-auto",
                            img {
                                src: "HoSL 64x64.png",
                                class: "p-1 place-content-center"
                            }
                            Link {
                                to: Route::HouseOfSelectedLords {},
                                class: "hover:underline place-content-center", "House of Selected Lords"
                            }
                        }

                        // For the larger screen
                        div {
                            class: "max-md:hidden flex flex-col p-3 space-y-1 flex-grow shadow-md hover:shadow-xl",
                            img {
                                src: "HoC With Text.png",
                                class: "h-24 w-24 mx-7 self-center"
                            }
                            Link {
                                to: Route::HouseOfCommons {},
                                class: "text-lg border-b-2 ", "House of Commons"
                            }
                            div {
                                class: "text-sm",
                                "The lower chamber of Parliament has 600 MPs, each representing a constituency. MPs enact laws, debate, decide on national issues, and scrutinise government policies by questioning in the Chamber or Committees."
                            }
                        }
                        div {
                            class: "max-md:hidden flex flex-col p-3 space-y-1 flex-grow shadow-md hover:shadow-xl",
                            img {
                                src: "HoSL With Text.png",
                                class: "h-24 w-24 mx-7 self-center"
                            }
                            Link {
                                to: Route::HouseOfSelectedLords {},
                                class: "text-lg border-b-2 ", "House of Selected Lords"
                            }
                            div {
                                class: "text-sm",
                                "The upper chamber of Parliament comprises 300 Lords, each representing a village manor. Their duties include scrutinising laws, debating public policies, and examining government decisions by posing questions in the Chamber or Committees."
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn HouseOfCommons() -> Element {
    rsx! {
        div {
            Header {}
            div {
                class: "px-20 py-4 bg-commonscolour",
                Link {
                    to: Route::Home {},
                    class: "hover:underline text-sm text-white", "Back to Home Page"
                }
                img {
                    class: "max-md:hidden md:float-right md:bg-white md:rounded-md md:border-2 md:border-white md:h-16 md:w-16 md:ml-3",
                    src: "HoC With Text.png"
                }
                div {
                    class: "mt-1 text-2xl text-white font-semibold", "House of Commons"
                }
                div {
                    class: "mt-1 text-white",
                    "The lower chamber of Parliament comprises 600 MPs (Members of Parliament), each representing a constituency. MPs enact laws, participate in debates, and make decisions on various national issues. They also scrutinise government policies by asking questions in the Chamber or in Committees."
                }
            }
            div {
                class: "px-20 my-2",
                div {
                    class: "mt-1 shadow-md",
                    div {
                        class: "flex flex-row pt-2 px-3 space-x-3",
                        img {
                            class: "place-content-center w-14 h-14 m-1",
                            src: "HoC 64x64.png"
                        }
                        div {
                            class: "flex flex-col flex-grow space-y-1",
                            div {
                                class: "text-lg border-b-2 border-gray-400", "Find The Committees"
                            }
                            div {
                                class: "text-sm",
                                "Select committees, standing committees, joint committees and grand committees"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn HouseOfSelectedLords() -> Element {
    rsx! {
        div {
            Header {}
            div {
                class: "px-20 py-4 bg-lordscolour",
                Link {
                    to: Route::Home {},
                    class: "hover:underline text-sm text-white", "Back to Home Page"
                }
                img {
                    class: "max-md:hidden md:float-right md:bg-white md:rounded-md md:border-2 md:border-white md:h-16 md:w-16 md:ml-3",
                    src: "HoSL With Text.png"
                }
                div {
                    class: "mt-1 text-2xl text-white font-semibold", "House of Selected Lords"
                }
                div {
                    class: "mt-1 text-white",
                    "The upper chamber of Parliament, consisting of 300 Lords, each designated to represent a village manor, undertakes a pivotal role in governance. Their responsibilities encompass meticulous scrutiny of legislation, earnest deliberations on public policies, and rigorous examination of governmental decisions, facilitated through probing inquiries conducted within the Chamber or Committees."
                }
            }
        }
    }
}