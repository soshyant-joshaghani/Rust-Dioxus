use dioxus::prelude::*;

use crate::modules::base::icons::{BriefcaseIcon, HomeIcon, UsersIcon};
use crate::modules::base::stores::auth::use_auth;
use crate::modules::base::ui::sidebar::{use_sidebar, Sidebar, SidebarMenuButton};
use crate::modules::base::APP_NAME;
use crate::routes::Route;

#[derive(Clone, Copy, PartialEq)]
enum Icon {
    Home,
    Briefcase,
    Users,
}

#[component]
pub fn AppSidebar() -> Element {
    let auth = use_auth();
    let current = use_route::<Route>();
    let mut mobile_open = use_sidebar().mobile_open;

    let mut items = vec![
        ("Dashboard", Route::Home {}, Icon::Home),
        ("Sample Notes", Route::SampleNotes {}, Icon::Briefcase),
    ];
    if auth.state().is_superuser() {
        items.push(("Admin", Route::Admin {}, Icon::Users));
    }

    rsx! {
        Sidebar {
            div { class: "flex flex-col gap-1 border-b border-sidebar-border p-4",
                p { class: "text-xs font-semibold tracking-widest text-muted-foreground uppercase", "Dashboard" }
                p { class: "text-lg font-bold", "{APP_NAME}" }
            }
            div { class: "flex min-h-0 flex-1 flex-col gap-2 overflow-auto p-2",
                div { class: "flex h-8 items-center px-2 text-xs font-medium text-sidebar-foreground/70", "Menu" }
                ul { class: "flex w-full flex-col gap-1",
                    for (title, route, icon) in items {
                        li { key: "{title}",
                            Link { to: route.clone(), onclick: move |_| mobile_open.set(false),
                                SidebarMenuButton { active: current == route,
                                    match icon {
                                        Icon::Home => rsx! { HomeIcon {} },
                                        Icon::Briefcase => rsx! { BriefcaseIcon {} },
                                        Icon::Users => rsx! { UsersIcon {} },
                                    }
                                    span { "{title}" }
                                }
                            }
                        }
                    }
                }
            }
            div { class: "border-t border-sidebar-border p-4 text-xs text-muted-foreground", "{APP_NAME} From FoxG" }
        }
    }
}
