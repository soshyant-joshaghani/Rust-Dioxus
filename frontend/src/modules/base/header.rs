use dioxus::prelude::*;

use crate::modules::base::theme_toggle::ThemeToggle;
use crate::modules::base::ui::sidebar::SidebarTrigger;
use crate::modules::base::ui::Separator;
use crate::modules::base::user_nav::UserNav;

#[component]
pub fn Header() -> Element {
    rsx! {
        header { class: "flex h-14 shrink-0 items-center gap-2 border-b px-4",
            SidebarTrigger { class: "-ml-1" }
            Separator { vertical: true, class: "mr-2 h-4" }
            div { class: "flex flex-1 items-center justify-end gap-2",
                ThemeToggle {}
                UserNav {}
            }
        }
    }
}
