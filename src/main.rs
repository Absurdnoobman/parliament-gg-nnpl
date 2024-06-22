#![allow(non_snake_case)]

mod idk;

use dioxus::prelude::*;
use tracing::Level;

const _TAILWIND_URL: &str = manganis::mg!(file("public/tailwind.css"));

#[derive(Clone)]
enum Chamber {
    Commons,
    Lords,
}

impl PartialEq for Chamber {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[layout(Header)]

        #[route("/")]
        Home {},


        #[nest("/hoc")]

            #[route("/")]
            HouseOfCommons {},
            #[nest("/committees")]

                #[route("/")]
                CommonsCommittees {},
                #[route("/administration-committee")]
                CommonsAdministrationCommittees {},
                #[route("/backbench-business-committee")]
                BackbenchBusinessCommittee {},
                #[route("/business-and-trade-committee")]
                BusinessAndTradeCommittee {},

            #[end_nest]

        #[end_nest]

        #[route("/hosl")]
        HouseOfSelectedLords {},

    #[end_layout]

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
fn ErrorNotFound(segments: Vec<String>) -> Element {
    let mut text : String = String::new();
    if !(segments.is_empty()) {
        text = segments.join("/")
    }
    rsx! {
        div {
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
                        src: "https://github.com/Absurdnoobman/parliament-gg-nnpl/assets/88262542/40830574-d492-4a26-95d2-500fd5bdef7d",
                        class: "self-stretch"
                    }
                }
                div {
                    class: "flex-auto text-2xl", "House of Parliament"
                }
            }
        }
        div {
            Outlet::<Route> {}
        }
    }
}

#[component]
fn Home() -> Element {
    rsx! {
        div {
            // Header {}
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
            // Header {}
            div {
                class: "px-20 py-4 bg-commonscolour",
                Link {
                    to: Route::Home {},
                    class: "hover:underline text-sm text-white", "Back to Home Page"
                }
                img {
                    class: "max-md:hidden md:float-right md:bg-white md:rounded-md md:border-2 md:border-white md:h-16 md:w-16 md:ml-3",
                    src: "../HoC With Text.png"
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
                        class: "flex flex-row py-2 px-3 space-x-3",
                        img {
                            class: "place-content-center w-14 h-14 m-1",
                            src: "../HoC 64x64.png"
                        }
                        div {
                            class: "flex flex-col flex-grow space-y-1",
                            Link {
                                to: Route::CommonsCommittees {},
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
fn CommitteesTemplate(
    committee_name: String, 
    committee_type: String, 
    chamber: Chamber, 
    route: Route
) -> Element {
    let _sad: Chamber = Chamber::Commons {};
    let _da: Chamber = Chamber::Lords {};
    let is_commons: bool = match chamber {
        Chamber::Commons => true,
        Chamber::Lords => false
    };
    rsx!{
        div {
            class: if is_commons {
                "py-2 flex flex-col space-y-1 border-l-4 border-commonscolour shadow-md hover:shadow-xl"
            } else {
                "py-2 flex flex-col space-y-1 border-l-4 border-lordscolour shadow-md hover:shadow-xl"
            },
            Link {
                to: route,
                class: "mx-2 pl-2 text-lg border-b-2",
                "{committee_name}"
            }
            div {
                class: "mx-2 pl-2 text-sm",
                "{committee_type}"
            }
        }
    }
}

#[component]
fn CommitteeMember(
    title: String,
    name: String,
    position: String,
    // party_colour: String,
    constituency: String,
    img_url: String
) -> Element {
    rsx!{
        div {
            class: "flex flex-row space-x-1",
            img {
                class: " w-28 h-36 flex-shrink",
                src: img_url
            }
            div {
                class: "flex flex-col px-1 space-y-2 grow",
                div {
                    class: "text-lg border-b border-slate-300",
                    "{title} {name}"
                }
                div {
                    class: "flex flex-row",
                    div {
                        class: "text-gray-500 text-left text-sm grow",
                        "{constituency}"
                    }
                    div {
                        {
                            match position.as_str() {
                                "Chair" => rsx! {
                                    div {
                                        class: "flex grow-0 flex-row",
                                        img {
                                            src: "https://github.com/Absurdnoobman/parliament-gg-nnpl/assets/88262542/0ad06d61-a6d3-4e27-aeb7-6d884c7e456f",
                                            class: "shrink w-auto h-auto self-center"
                                        }
                                        div {
                                            class: "pl-1 text-sm text-center font-semibold", "Chair"
                                        }
                                    }
                                },
                                "Deputy Chair" => rsx! {
                                    div {
                                        class: "pl-1 text-sm text-center font-semibold", "Deputy Chair"
                                    }
                                },
                                "Member" => rsx! {
                                    div {
                                        class: "pl-1 text-sm text-center font-semibold", "Member"
                                    }
                                },
                                _ => rsx! {
                                    div {}
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn CommonsCommittees() -> Element {
    // let result: Result<Vec<String>, std::io::Error> = idk::get_commons_committee_list();
    // let mut _err: String = String::new();
    // let mut  _committees: Vec<String> = Vec::new();
    // match result {
    //     Ok(list) => {
    //         _committees = list.to_owned()
    //     },
    //     Err(e) => {
    //         _err = e.to_string();
    //     }
    // };
    rsx!{
        div {
            class: "px-20 py-4 bg-commonscolour text-white",
            Link {
                to: Route::HouseOfCommons {},
                class: "text-sm hover:underline",
                "Go back to the House of Commons main page"
            }
            div {
                class: "mt-1 text-2xl font-semibold",
                "Commons Committees"
            }
            div {
                class: "mt-1",
                "Committees look into policy matters, keep tabs on how taxpayer money is being spent, and review proposed laws to make sure they're fair and make sense."
            }
        }
        div {
            class: "px-20 my-2",
            div {
                class: "grid max-md:grid-cols-1 md:grid-cols-2 md:grid-flow-row gap-1",
                CommitteesTemplate {
                    committee_name: String::from("Administration Committee"),
                    committee_type: String::from("Select committee"),
                    chamber: Chamber::Commons,
                    route: Route::CommonsAdministrationCommittees {}
                }
                CommitteesTemplate {
                    committee_name: String::from("Backbench Business Committee"),
                    committee_type: String::from("Select committee"),
                    chamber: Chamber::Commons,
                    route: Route::BackbenchBusinessCommittee {}
                }
                CommitteesTemplate {
                    committee_name: String::from("Business and Trade Committee"),
                    committee_type: String::from("Select committee"),
                    chamber: Chamber::Commons,
                    route: Route::BusinessAndTradeCommittee {}
                }
            }
            
        }
    }
}

#[component]
fn CommonsAdministrationCommittees() -> Element {
    rsx!{
        div {
            class: "px-20 py-4 bg-commonscolour text-white",
            Link {
                to: Route::CommonsCommittees {},
                class: "text-sm hover:underline",
                "Go back to the House of Commons Committee List main page"
            }
            div {
                class: "mt-1 text-2xl font-semibold",
                "Administration Committee"
            }
            div {
                class: "mt-1",
                "The Administration Committee considers the services provided for Members, their staff and visitors by the House of Commons Service and makes recommendations to the House of Commons Commission, the Speaker and Officials on how those services are delivered."
            }
        }
        div {
            class: "px-20 my-2 text-black",
            div {
                class: "text-lg border-b-2 border-gray-400",
                "Members"
            }
            div {
                class: "text-sm my-2",
                "11 current members of this committee are follow:"
            }
            div {
                class: "grid max-md:grid-cols-1 md:grid-cols-2 md:grid-flow-row gap-2 my-2",
                CommitteeMember {
                    title: String::from("Player"),
                    name: String::from("Breadddd 231"),
                    position: String::from("Chair"),
                    constituency: String::from("North Carboxy Castle"),
                    img_url: String::from("https://github.com/Absurdnoobman/parliament-gg-nnpl/assets/88262542/4c3fc8f4-ecdb-4d15-bc7a-decfe049e936")
                }
                CommitteeMember {
                    title: String::from("Player"),
                    name: String::from("FFFFFFF"),
                    position: String::from("Deputy Chair"),
                    constituency: String::from("Ur Mom"),
                    img_url: String::from("https://github.com/Absurdnoobman/parliament-gg-nnpl/assets/88262542/272208a9-c78e-4915-8828-d0929ccca468")
                }
                CommitteeMember {
                    title: String::from("Player"),
                    name: String::from("IamKevin643"),
                    position: String::from("Member"),
                    constituency: String::from("Westerwald"),
                    img_url: String::from("https://github.com/Absurdnoobman/parliament-gg-nnpl/assets/88262542/cdb68641-691d-4c61-868e-90398e91fadc")
                }
                CommitteeMember {
                    title: String::from("Player Dr."),
                    name: String::from("Jo4rk"),
                    position: String::from("Member"),
                    constituency: String::from("North Ohio LOL"),
                    img_url: String::from("https://github.com/Absurdnoobman/parliament-gg-nnpl/assets/88262542/116efac1-080b-4025-8fbb-4c9ffd0447ec")
                }
                CommitteeMember {
                    title: String::from("Player"),
                    name: String::from("ToothlessMental"),
                    position: String::from("Member"),
                    constituency: String::from("St. James Neighbourhood"),
                    img_url: String::from("https://github.com/Absurdnoobman/parliament-gg-nnpl/assets/88262542/a8f94cdc-ae69-48f6-909f-6f278f6052f2")
                }
                CommitteeMember {
                    title: String::from("Player"),
                    name: String::from("lettuceGaming"),
                    position: String::from("Member"),
                    constituency: String::from("North East Sandyland Castle"),
                    img_url: String::from("https://github.com/Absurdnoobman/parliament-gg-nnpl/assets/88262542/66741512-3ef1-4a8a-afd9-6f4f5086d112")
                }
                CommitteeMember {
                    title: String::from("Player Sir"),
                    name: String::from("Dwight Fairfield"),
                    position: String::from("Member"),
                    constituency: String::from("Falen View"),
                    img_url: String::from("https://github.com/Absurdnoobman/parliament-gg-nnpl/assets/88262542/b26bd02a-a6f1-4268-b8c9-916ab86103dd")
                }
                CommitteeMember {
                    title: String::from("Player"),
                    name: String::from("AlbionAlchemist"),
                    position: String::from("Member"),
                    constituency: String::from("New Ploanland"),
                    img_url: String::from("https://github.com/Absurdnoobman/parliament-gg-nnpl/assets/88262542/9733a548-4c7a-4bf1-8cad-a12da5b8fc54")
                }
                CommitteeMember {
                    title: String::from("Player"),
                    name: String::from("FarmingtonGuy"),
                    position: String::from("Member"),
                    constituency: String::from("South Farmington"),
                    img_url: String::from("https://github.com/Absurdnoobman/parliament-gg-nnpl/assets/88262542/d46196b0-94c3-44c2-90a8-273768cda5e8")
                }
                CommitteeMember {
                    title: String::from("Player"),
                    name: String::from("TheyAreRocks"),
                    position: String::from("Member"),
                    constituency: String::from("Namlyunknownhood"),
                    img_url: String::from("https://github.com/Absurdnoobman/parliament-gg-nnpl/assets/88262542/a5d13681-730a-4f5e-8418-0450b3bede19")
                }
                CommitteeMember {
                    title: String::from("Player"),
                    name: String::from("xXx_NOOB_xXx"),
                    position: String::from("Member"),
                    constituency: String::from("West Detroit Became Hood"),
                    img_url: String::from("https://github.com/Absurdnoobman/parliament-gg-nnpl/assets/88262542/6c888a80-1a30-401f-ba81-990fe8d1b655")
                }
            }
        }
    }
}

#[component]
fn BackbenchBusinessCommittee() -> Element {
    rsx! {
        div {
            class: "px-20 py-4 bg-commonscolour text-white",
            Link {
                to: Route::CommonsCommittees {},
                class: "text-sm hover:underline",
                "Go back to the House of Commons Committee List main page"
            }
            div {
                class: "mt-1 text-2xl font-semibold",
                "Backbench Business Committee"
            }
            div {
                class: "mt-1",
                "The Backbench Business Committee gives opportunities to backbench Members of Parliament to bring forward debates of their choice. It was the first select committee of any kind to be established by the House of Commons."
            }
        }
        div {
            class: "px-20 my-2 text-black",
            div {
                class: "text-lg border-b-2 border-gray-400",
                "Members"
            }
            div {
                class: "text-sm my-2",
                "11 current members of this committee are follow:"
            }
            div {
                class: "grid max-md:grid-cols-1 md:grid-cols-2 md:grid-flow-row gap-2 my-2",
                CommitteeMember {
                    title: String::from("Player"),
                    name: String::from("NigelGreen57"),
                    position: String::from("Chair"),
                    constituency: String::from("El colaro"),
                    img_url: String::from("https://github.com/Absurdnoobman/parliament-gg-nnpl/assets/88262542/7e3abcd7-998e-49f6-a79c-7fa1b67f3f3a")
                }
                CommitteeMember {
                    title: String::from("Player"),
                    name: String::from("PupleEye21"),
                    position: String::from("Deputy Chair"),
                    constituency: String::from("South Fornland"),
                    img_url: String::from("https://github.com/Absurdnoobman/parliament-gg-nnpl/assets/88262542/a0886fb0-1ce7-45f6-ae28-26e1f218cf52")
                }
                CommitteeMember {
                    title: String::from("Player"),
                    name: String::from("Refunded69"),
                    position: String::from("Member"),
                    constituency: String::from("Overworld City"),
                    img_url: String::from("https://github.com/Absurdnoobman/parliament-gg-nnpl/assets/88262542/9f32912b-365d-47cf-8b4f-3f8b3eae2fc9")
                }
                CommitteeMember {
                    title: String::from("Player"),
                    name: String::from("_GGEZ_>_<"),
                    position: String::from("Member"),
                    constituency: String::from("Boon"),
                    img_url: String::from("https://github.com/Absurdnoobman/parliament-gg-nnpl/assets/88262542/58346511-c9de-4728-8eee-414d9b1b6ff9")
                }
                CommitteeMember {
                    title: String::from("Player"),
                    name: String::from("Sprintburst"),
                    position: String::from("Member"),
                    constituency: String::from("MacMillan"),
                    img_url: String::from("https://github.com/Absurdnoobman/parliament-gg-nnpl/assets/88262542/8c3e0333-d17b-4f89-8e7d-e9cc74dc8366")
                }
                CommitteeMember {
                    title: String::from("Player"),
                    name: String::from("Tomy_Kick"),
                    position: String::from("Member"),
                    constituency: String::from("Overworld City North"),
                    img_url: String::from("https://github.com/Absurdnoobman/parliament-gg-nnpl/assets/88262542/7250aa25-9371-4643-84fe-6d249a065e7d")
                }
                CommitteeMember {
                    title: String::from("Player"),
                    name: String::from("_GhostActor_"),
                    position: String::from("Member"),
                    constituency: String::from("South West Simon"),
                    img_url: String::from("https://github.com/Absurdnoobman/parliament-gg-nnpl/assets/88262542/4217eddb-adfb-49d7-9133-ef6e408908e2")
                }
                CommitteeMember {
                    title: String::from("Player"),
                    name: String::from("SHMTH6651"),
                    position: String::from("Member"),
                    constituency: String::from("X Neighbourhood"),
                    img_url: String::from("https://github.com/Absurdnoobman/parliament-gg-nnpl/assets/88262542/e87af619-406d-4456-86bf-e180c674c4c2")
                }
                CommitteeMember {
                    title: String::from("Player"),
                    name: String::from("AnnoyingAce"),
                    position: String::from("Member"),
                    constituency: String::from("Kamer"),
                    img_url: String::from("https://github.com/Absurdnoobman/parliament-gg-nnpl/assets/88262542/6f06cbe5-f7c6-4cbe-8958-b197d3b09b15")
                }
            }
        }
    }
}

fn BusinessAndTradeCommittee() -> Element {
    rsx! {
        div {
            class: "px-20 py-4 bg-commonscolour text-white",
            Link {
                to: Route::CommonsCommittees {},
                class: "text-sm hover:underline",
                "Go back to the House of Commons Committee List main page"
            }
            div {
                class: "mt-1 text-2xl font-semibold",
                "Business and Trade Committee"
            }
            div {
                class: "mt-1",
                "The Business and Trade Committee scrutinises the policy, spending and administration of the Department for Business and Trade and its public bodies."
            }
        }
        div {
            class: "px-20 my-2 text-black",
            div {
                class: "text-lg border-b-2 border-gray-400",
                "Members"
            }
            div {
                class: "text-sm my-2",
                "11 current members of this committee are follow:"
            }
            div {
                class: "grid max-md:grid-cols-1 md:grid-cols-2 md:grid-flow-row gap-2 my-2",
                CommitteeMember {
                    title: String::from("Player"),
                    name: String::from("Mongar2"),
                    position: String::from("Chair"),
                    constituency: String::from("Coomland East"),
                    img_url: String::from("https://github.com/Absurdnoobman/parliament-gg-nnpl/assets/88262542/db08e8cd-5a82-4f44-a0a3-21e3d835b1a2")
                }
                CommitteeMember {
                    title: String::from("Player"),
                    name: String::from("BigSmoke31"),
                    position: String::from("Deputy Chair"),
                    constituency: String::from("Absuy"),
                    img_url: String::from("https://github.com/Absurdnoobman/parliament-gg-nnpl/assets/88262542/f3767800-110f-438f-b48f-497099137c71")
                }
                CommitteeMember {
                    title: String::from("Player"),
                    name: String::from("just_A_fish"),
                    position: String::from("Member"),
                    constituency: String::from("South Iofamn"),
                    img_url: String::from("https://github.com/Absurdnoobman/parliament-gg-nnpl/assets/88262542/5b1f5cab-0687-439d-8893-d7ed374fff7a")
                }
                CommitteeMember {
                    title: String::from("Player"),
                    name: String::from("BF_Enjoyner_37"),
                    position: String::from("Member"),
                    constituency: String::from("Hosinne"),
                    img_url: String::from("https://github.com/Absurdnoobman/parliament-gg-nnpl/assets/88262542/59070f13-31e4-44bd-977f-858089216d87")
                }
                CommitteeMember {
                    title: String::from("Player Dr."),
                    name: String::from("1Memo"),
                    position: String::from("Member"),
                    constituency: String::from("North West Larfame"),
                    img_url: String::from("https://github.com/Absurdnoobman/parliament-gg-nnpl/assets/88262542/d1bdba9b-d6d5-4314-9c1d-6520f26acc51")
                }
                CommitteeMember {
                    title: String::from("Player"),
                    name: String::from("Tomy_Kick"),
                    position: String::from("Member"),
                    constituency: String::from("Overworld City North"),
                    img_url: String::from("https://github.com/Absurdnoobman/parliament-gg-nnpl/assets/88262542/7250aa25-9371-4643-84fe-6d249a065e7d")
                }
                CommitteeMember {
                    title: String::from("Player Sir"),
                    name: String::from("Teamkill is Ban"),
                    position: String::from("Member"),
                    constituency: String::from("Oddison"),
                    img_url: String::from("https://github.com/Absurdnoobman/parliament-gg-nnpl/assets/88262542/77960e7e-c2d2-402f-bb52-3e64baa69c94")
                }
                CommitteeMember {
                    title: String::from("Player Sir"),
                    name: String::from("Leland"),
                    position: String::from("Member"),
                    constituency: String::from("Kolony City"),
                    img_url: String::from("https://github.com/Absurdnoobman/parliament-gg-nnpl/assets/88262542/1eb5bbe1-7a86-44bc-883f-41809e93d687")
                }
                CommitteeMember {
                    title: String::from("Player Dr. Sir"),
                    name: String::from("Mike Toreno"),
                    position: String::from("Member"),
                    constituency: String::from("Uiid Town"),
                    img_url: String::from("https://github.com/Absurdnoobman/parliament-gg-nnpl/assets/88262542/d565291c-072f-44c6-8d6f-5ecfb9570b44")
                }
                CommitteeMember {
                    title: String::from("Player"),
                    name: String::from(""),
                    position: String::from("Member"),
                    constituency: String::from(""),
                    img_url: String::from("")
                }
            }
        }
    }
}

#[component]
fn HouseOfSelectedLords() -> Element {
    rsx! {
        div {
            // Header {}
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