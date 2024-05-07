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
                        // src: "https://github.com/Absurdnoobman/parliament-gg-nnpl/assets/88262542/40830574-d492-4a26-95d2-500fd5bdef7d",
                        src: "https://github-production-user-asset-6210df.s3.amazonaws.com/88262542/328037330-40830574-d492-4a26-95d2-500fd5bdef7d.png?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=AKIAVCODYLSA53PQK4ZA%2F20240507%2Fus-east-1%2Fs3%2Faws4_request&X-Amz-Date=20240507T111504Z&X-Amz-Expires=300&X-Amz-Signature=7651402dc0225ab99dbafc5252a42fad1a02d9a0d7500be6e3af87d525db19cd&X-Amz-SignedHeaders=host&actor_id=88262542&key_id=0&repo_id=792878543",
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
                        class: "flex flex-row pt-2 px-3 space-x-3",
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
                                            src: "https://github-production-user-asset-6210df.s3.amazonaws.com/88262542/328478478-0ad06d61-a6d3-4e27-aeb7-6d884c7e456f.png?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=AKIAVCODYLSA53PQK4ZA%2F20240507%2Fus-east-1%2Fs3%2Faws4_request&X-Amz-Date=20240507T111559Z&X-Amz-Expires=300&X-Amz-Signature=7edfc39c420748532480f1e33a4d0224b82c01290815202f3377217142df017a&X-Amz-SignedHeaders=host&actor_id=88262542&key_id=0&repo_id=792878543",
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
                    route: Route::CommonsAdministrationCommittees {}
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
                    img_url: String::from("https://github-production-user-asset-6210df.s3.amazonaws.com/88262542/328495002-c6ec37bf-fd73-44f0-ba98-009a6bcc9312.png?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=AKIAVCODYLSA53PQK4ZA%2F20240507%2Fus-east-1%2Fs3%2Faws4_request&X-Amz-Date=20240507T105554Z&X-Amz-Expires=300&X-Amz-Signature=fd7b5536f03c866e74e7d674fdb17df3264e8d4f00275766475257cfd13850f8&X-Amz-SignedHeaders=host&actor_id=88262542&key_id=0&repo_id=792878543")
                }
                CommitteeMember {
                    title: String::from("Player"),
                    name: String::from("FFFFFFF"),
                    position: String::from("Deputy Chair"),
                    constituency: String::from("Ur Mom"),
                    img_url: String::from("https://github-production-user-asset-6210df.s3.amazonaws.com/88262542/328501633-272208a9-c78e-4915-8828-d0929ccca468.png?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=AKIAVCODYLSA53PQK4ZA%2F20240507%2Fus-east-1%2Fs3%2Faws4_request&X-Amz-Date=20240507T111855Z&X-Amz-Expires=300&X-Amz-Signature=40b23cfa1fa8bc60c9ef3bb70f20c65d388d8de46b5e70d469036b11c16c7323&X-Amz-SignedHeaders=host&actor_id=88262542&key_id=0&repo_id=792878543")
                }
                CommitteeMember {
                    title: String::from("Player"),
                    name: String::from("IamKevin643"),
                    position: String::from("Member"),
                    constituency: String::from("Westerwald"),
                    img_url: String::from("https://github-production-user-asset-6210df.s3.amazonaws.com/88262542/328501651-cdb68641-691d-4c61-868e-90398e91fadc.png?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=AKIAVCODYLSA53PQK4ZA%2F20240507%2Fus-east-1%2Fs3%2Faws4_request&X-Amz-Date=20240507T115557Z&X-Amz-Expires=300&X-Amz-Signature=c54279d5e7b8f31d422a398d70296cb6dbb843b45a173f1770c74e21a066121e&X-Amz-SignedHeaders=host&actor_id=88262542&key_id=0&repo_id=792878543")
                }
                CommitteeMember {
                    title: String::from("Player Dr."),
                    name: String::from("Jo4rk"),
                    position: String::from("Member"),
                    constituency: String::from("North Ohio LOL"),
                    img_url: String::from("https://github-production-user-asset-6210df.s3.amazonaws.com/88262542/328501612-116efac1-080b-4025-8fbb-4c9ffd0447ec.png?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=AKIAVCODYLSA53PQK4ZA%2F20240507%2Fus-east-1%2Fs3%2Faws4_request&X-Amz-Date=20240507T112602Z&X-Amz-Expires=300&X-Amz-Signature=28d8ef6deca613532dfbf6ce1cc48103f6963fa76526eacfa10a662d241adc2d&X-Amz-SignedHeaders=host&actor_id=88262542&key_id=0&repo_id=792878543")
                }
                CommitteeMember {
                    title: String::from("Player"),
                    name: String::from("ToothlessMental"),
                    position: String::from("Member"),
                    constituency: String::from("St. James Neighbourhood"),
                    img_url: String::from("https://github-production-user-asset-6210df.s3.amazonaws.com/88262542/328501629-a8f94cdc-ae69-48f6-909f-6f278f6052f2.png?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=AKIAVCODYLSA53PQK4ZA%2F20240507%2Fus-east-1%2Fs3%2Faws4_request&X-Amz-Date=20240507T114054Z&X-Amz-Expires=300&X-Amz-Signature=1f7eedc0ccb80fcdecb4587c4375d7ff930d1a656e7699bc2f62491bb2804a44&X-Amz-SignedHeaders=host&actor_id=88262542&key_id=0&repo_id=792878543")
                }
                CommitteeMember {
                    title: String::from("Player"),
                    name: String::from("lettuceGaming"),
                    position: String::from("Member"),
                    constituency: String::from("North East Sandyland Castle"),
                    img_url: String::from("https://github-production-user-asset-6210df.s3.amazonaws.com/88262542/328501591-66741512-3ef1-4a8a-afd9-6f4f5086d112.png?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=AKIAVCODYLSA53PQK4ZA%2F20240507%2Fus-east-1%2Fs3%2Faws4_request&X-Amz-Date=20240507T113519Z&X-Amz-Expires=300&X-Amz-Signature=af747ae01cbc86cbedf2b137f2b9b8dc7b2395b25908d96049205cc965be2bc2&X-Amz-SignedHeaders=host&actor_id=88262542&key_id=0&repo_id=792878543")
                }
                CommitteeMember {
                    title: String::from("Player Sir"),
                    name: String::from("Dwight Fairfield"),
                    position: String::from("Member"),
                    constituency: String::from("Falen View"),
                    img_url: String::from("https://github-production-user-asset-6210df.s3.amazonaws.com/88262542/328501660-b26bd02a-a6f1-4268-b8c9-916ab86103dd.png?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=AKIAVCODYLSA53PQK4ZA%2F20240507%2Fus-east-1%2Fs3%2Faws4_request&X-Amz-Date=20240507T115112Z&X-Amz-Expires=300&X-Amz-Signature=6e77b0510ab218aee10a1ea46ba1d20c2d593e4dbf445d3be952daed3e566d2a&X-Amz-SignedHeaders=host&actor_id=88262542&key_id=0&repo_id=792878543")
                }
                CommitteeMember {
                    title: String::from("Player"),
                    name: String::from("AlbionAlchemist"),
                    position: String::from("Member"),
                    constituency: String::from("New Ploanland"),
                    img_url: String::from("https://github-production-user-asset-6210df.s3.amazonaws.com/88262542/328501673-9733a548-4c7a-4bf1-8cad-a12da5b8fc54.png?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=AKIAVCODYLSA53PQK4ZA%2F20240507%2Fus-east-1%2Fs3%2Faws4_request&X-Amz-Date=20240507T112429Z&X-Amz-Expires=300&X-Amz-Signature=8d1e8f73501e5fa8ac3fb32a5305d7e6fbcdf96b5d292d0c38e8aef7432ad74c&X-Amz-SignedHeaders=host&actor_id=88262542&key_id=0&repo_id=792878543")
                }
                CommitteeMember {
                    title: String::from("Player"),
                    name: String::from("FarmingtonGuy"),
                    position: String::from("Member"),
                    constituency: String::from("South Farmington"),
                    img_url: String::from("https://github-production-user-asset-6210df.s3.amazonaws.com/88262542/328501620-d46196b0-94c3-44c2-90a8-273768cda5e8.png?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=AKIAVCODYLSA53PQK4ZA%2F20240507%2Fus-east-1%2Fs3%2Faws4_request&X-Amz-Date=20240507T114142Z&X-Amz-Expires=300&X-Amz-Signature=0f3d22c6d390e32d3ef804505799727811d394db1e6b6440875e4baa97ea8aad&X-Amz-SignedHeaders=host&actor_id=88262542&key_id=0&repo_id=792878543")
                }
                CommitteeMember {
                    title: String::from("Player"),
                    name: String::from("TheyAreRocks"),
                    position: String::from("Member"),
                    constituency: String::from("Namlyunknownhood"),
                    img_url: String::from("https://github-production-user-asset-6210df.s3.amazonaws.com/88262542/328501666-a5d13681-730a-4f5e-8418-0450b3bede19.png?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=AKIAVCODYLSA53PQK4ZA%2F20240507%2Fus-east-1%2Fs3%2Faws4_request&X-Amz-Date=20240507T115225Z&X-Amz-Expires=300&X-Amz-Signature=ace852852db8e356d72efa2d4af342a913ff306f459faa6f161adce9ea543afd&X-Amz-SignedHeaders=host&actor_id=88262542&key_id=0&repo_id=792878543")
                }
                CommitteeMember {
                    title: String::from("Player"),
                    name: String::from("xXx_NOOB_xXx"),
                    position: String::from("Member"),
                    constituency: String::from("West Detroit Became Hood"),
                    img_url: String::from("https://github-production-user-asset-6210df.s3.amazonaws.com/88262542/328501642-6c888a80-1a30-401f-ba81-990fe8d1b655.png?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=AKIAVCODYLSA53PQK4ZA%2F20240507%2Fus-east-1%2Fs3%2Faws4_request&X-Amz-Date=20240507T115325Z&X-Amz-Expires=300&X-Amz-Signature=a055f60c3ab5dd265a58d60e39856d951ebf5d33d72b29e61164438861e07692&X-Amz-SignedHeaders=host&actor_id=88262542&key_id=0&repo_id=792878543")
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
                    name: String::from("Breadddd 231"),
                    position: String::from("Chair"),
                    constituency: String::from("North Carboxy Castle"),
                    img_url: String::from("https://github-production-user-asset-6210df.s3.amazonaws.com/88262542/328495002-c6ec37bf-fd73-44f0-ba98-009a6bcc9312.png?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=AKIAVCODYLSA53PQK4ZA%2F20240507%2Fus-east-1%2Fs3%2Faws4_request&X-Amz-Date=20240507T105554Z&X-Amz-Expires=300&X-Amz-Signature=fd7b5536f03c866e74e7d674fdb17df3264e8d4f00275766475257cfd13850f8&X-Amz-SignedHeaders=host&actor_id=88262542&key_id=0&repo_id=792878543")
                }
                CommitteeMember {
                    title: String::from("Player"),
                    name: String::from("FFFFFFF"),
                    position: String::from("Deputy Chair"),
                    constituency: String::from("Ur Mom"),
                    img_url: String::from("https://github-production-user-asset-6210df.s3.amazonaws.com/88262542/328501633-272208a9-c78e-4915-8828-d0929ccca468.png?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=AKIAVCODYLSA53PQK4ZA%2F20240507%2Fus-east-1%2Fs3%2Faws4_request&X-Amz-Date=20240507T111855Z&X-Amz-Expires=300&X-Amz-Signature=40b23cfa1fa8bc60c9ef3bb70f20c65d388d8de46b5e70d469036b11c16c7323&X-Amz-SignedHeaders=host&actor_id=88262542&key_id=0&repo_id=792878543")
                }
                CommitteeMember {
                    title: String::from("Player"),
                    name: String::from("IamKevin643"),
                    position: String::from("Member"),
                    constituency: String::from("Westerwald"),
                    img_url: String::from("https://github-production-user-asset-6210df.s3.amazonaws.com/88262542/328501651-cdb68641-691d-4c61-868e-90398e91fadc.png?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=AKIAVCODYLSA53PQK4ZA%2F20240507%2Fus-east-1%2Fs3%2Faws4_request&X-Amz-Date=20240507T115557Z&X-Amz-Expires=300&X-Amz-Signature=c54279d5e7b8f31d422a398d70296cb6dbb843b45a173f1770c74e21a066121e&X-Amz-SignedHeaders=host&actor_id=88262542&key_id=0&repo_id=792878543")
                }
                CommitteeMember {
                    title: String::from("Player Dr."),
                    name: String::from("Jo4rk"),
                    position: String::from("Member"),
                    constituency: String::from("North Ohio LOL"),
                    img_url: String::from("https://github-production-user-asset-6210df.s3.amazonaws.com/88262542/328501612-116efac1-080b-4025-8fbb-4c9ffd0447ec.png?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=AKIAVCODYLSA53PQK4ZA%2F20240507%2Fus-east-1%2Fs3%2Faws4_request&X-Amz-Date=20240507T112602Z&X-Amz-Expires=300&X-Amz-Signature=28d8ef6deca613532dfbf6ce1cc48103f6963fa76526eacfa10a662d241adc2d&X-Amz-SignedHeaders=host&actor_id=88262542&key_id=0&repo_id=792878543")
                }
                CommitteeMember {
                    title: String::from("Player"),
                    name: String::from("ToothlessMental"),
                    position: String::from("Member"),
                    constituency: String::from("St. James Neighbourhood"),
                    img_url: String::from("https://github-production-user-asset-6210df.s3.amazonaws.com/88262542/328501629-a8f94cdc-ae69-48f6-909f-6f278f6052f2.png?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=AKIAVCODYLSA53PQK4ZA%2F20240507%2Fus-east-1%2Fs3%2Faws4_request&X-Amz-Date=20240507T114054Z&X-Amz-Expires=300&X-Amz-Signature=1f7eedc0ccb80fcdecb4587c4375d7ff930d1a656e7699bc2f62491bb2804a44&X-Amz-SignedHeaders=host&actor_id=88262542&key_id=0&repo_id=792878543")
                }
                CommitteeMember {
                    title: String::from("Player"),
                    name: String::from("lettuceGaming"),
                    position: String::from("Member"),
                    constituency: String::from("North East Sandyland Castle"),
                    img_url: String::from("https://github-production-user-asset-6210df.s3.amazonaws.com/88262542/328501591-66741512-3ef1-4a8a-afd9-6f4f5086d112.png?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=AKIAVCODYLSA53PQK4ZA%2F20240507%2Fus-east-1%2Fs3%2Faws4_request&X-Amz-Date=20240507T113519Z&X-Amz-Expires=300&X-Amz-Signature=af747ae01cbc86cbedf2b137f2b9b8dc7b2395b25908d96049205cc965be2bc2&X-Amz-SignedHeaders=host&actor_id=88262542&key_id=0&repo_id=792878543")
                }
                CommitteeMember {
                    title: String::from("Player Sir"),
                    name: String::from("Dwight Fairfield"),
                    position: String::from("Member"),
                    constituency: String::from("Falen View"),
                    img_url: String::from("https://github-production-user-asset-6210df.s3.amazonaws.com/88262542/328501660-b26bd02a-a6f1-4268-b8c9-916ab86103dd.png?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=AKIAVCODYLSA53PQK4ZA%2F20240507%2Fus-east-1%2Fs3%2Faws4_request&X-Amz-Date=20240507T115112Z&X-Amz-Expires=300&X-Amz-Signature=6e77b0510ab218aee10a1ea46ba1d20c2d593e4dbf445d3be952daed3e566d2a&X-Amz-SignedHeaders=host&actor_id=88262542&key_id=0&repo_id=792878543")
                }
                CommitteeMember {
                    title: String::from("Player"),
                    name: String::from("AlbionAlchemist"),
                    position: String::from("Member"),
                    constituency: String::from("New Ploanland"),
                    img_url: String::from("https://github-production-user-asset-6210df.s3.amazonaws.com/88262542/328501673-9733a548-4c7a-4bf1-8cad-a12da5b8fc54.png?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=AKIAVCODYLSA53PQK4ZA%2F20240507%2Fus-east-1%2Fs3%2Faws4_request&X-Amz-Date=20240507T112429Z&X-Amz-Expires=300&X-Amz-Signature=8d1e8f73501e5fa8ac3fb32a5305d7e6fbcdf96b5d292d0c38e8aef7432ad74c&X-Amz-SignedHeaders=host&actor_id=88262542&key_id=0&repo_id=792878543")
                }
                CommitteeMember {
                    title: String::from("Player"),
                    name: String::from("FarmingtonGuy"),
                    position: String::from("Member"),
                    constituency: String::from("South Farmington"),
                    img_url: String::from("https://github-production-user-asset-6210df.s3.amazonaws.com/88262542/328501620-d46196b0-94c3-44c2-90a8-273768cda5e8.png?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=AKIAVCODYLSA53PQK4ZA%2F20240507%2Fus-east-1%2Fs3%2Faws4_request&X-Amz-Date=20240507T114142Z&X-Amz-Expires=300&X-Amz-Signature=0f3d22c6d390e32d3ef804505799727811d394db1e6b6440875e4baa97ea8aad&X-Amz-SignedHeaders=host&actor_id=88262542&key_id=0&repo_id=792878543")
                }
                CommitteeMember {
                    title: String::from("Player"),
                    name: String::from("TheyAreRocks"),
                    position: String::from("Member"),
                    constituency: String::from("Namlyunknownhood"),
                    img_url: String::from("https://github-production-user-asset-6210df.s3.amazonaws.com/88262542/328501666-a5d13681-730a-4f5e-8418-0450b3bede19.png?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=AKIAVCODYLSA53PQK4ZA%2F20240507%2Fus-east-1%2Fs3%2Faws4_request&X-Amz-Date=20240507T115225Z&X-Amz-Expires=300&X-Amz-Signature=ace852852db8e356d72efa2d4af342a913ff306f459faa6f161adce9ea543afd&X-Amz-SignedHeaders=host&actor_id=88262542&key_id=0&repo_id=792878543")
                }
                CommitteeMember {
                    title: String::from("Player"),
                    name: String::from("xXx_NOOB_xXx"),
                    position: String::from("Member"),
                    constituency: String::from("West Detroit Became Hood"),
                    img_url: String::from("https://github-production-user-asset-6210df.s3.amazonaws.com/88262542/328501642-6c888a80-1a30-401f-ba81-990fe8d1b655.png?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=AKIAVCODYLSA53PQK4ZA%2F20240507%2Fus-east-1%2Fs3%2Faws4_request&X-Amz-Date=20240507T115325Z&X-Amz-Expires=300&X-Amz-Signature=a055f60c3ab5dd265a58d60e39856d951ebf5d33d72b29e61164438861e07692&X-Amz-SignedHeaders=host&actor_id=88262542&key_id=0&repo_id=792878543")
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