//! Lucide icons used by the shell, as inline SVG (no icon font, works on every platform).

use dioxus::prelude::*;

#[component]
fn Svg(class: String, children: Element) -> Element {
    rsx! {
        svg {
            class: "{class}",
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            "aria-hidden": "true",
            {children}
        }
    }
}

macro_rules! icon {
    ($name:ident, $($body:tt)*) => {
        #[component]
        pub fn $name(#[props(into, default = "size-4".to_string())] class: String) -> Element {
            rsx! { Svg { class, $($body)* } }
        }
    };
}

icon!(HomeIcon,
    path { d: "M15 21v-8a1 1 0 0 0-1-1h-4a1 1 0 0 0-1 1v8" }
    path { d: "M3 10a2 2 0 0 1 .709-1.528l7-5.999a2 2 0 0 1 2.582 0l7 5.999A2 2 0 0 1 21 10v9a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z" }
);

icon!(BriefcaseIcon,
    path { d: "M16 20V4a2 2 0 0 0-2-2h-4a2 2 0 0 0-2 2v16" }
    rect { width: "20", height: "14", x: "2", y: "6", rx: "2" }
);

icon!(UsersIcon,
    path { d: "M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2" }
    circle { cx: "9", cy: "7", r: "4" }
    path { d: "M22 21v-2a4 4 0 0 0-3-3.87" }
    path { d: "M16 3.13a4 4 0 0 1 0 7.75" }
);

icon!(LogOutIcon,
    path { d: "M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4" }
    polyline { points: "16 17 21 12 16 7" }
    line { x1: "21", x2: "9", y1: "12", y2: "12" }
);

icon!(SunIcon,
    circle { cx: "12", cy: "12", r: "4" }
    path { d: "M12 2v2" }
    path { d: "M12 20v2" }
    path { d: "m4.93 4.93 1.41 1.41" }
    path { d: "m17.66 17.66 1.41 1.41" }
    path { d: "M2 12h2" }
    path { d: "M20 12h2" }
    path { d: "m6.34 17.66-1.41 1.41" }
    path { d: "m19.07 4.93-1.41 1.41" }
);

icon!(MoonIcon,
    path { d: "M12 3a6 6 0 0 0 9 9 9 9 0 1 1-9-9Z" }
);

icon!(Trash2Icon,
    path { d: "M3 6h18" }
    path { d: "M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6" }
    path { d: "M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2" }
    line { x1: "10", x2: "10", y1: "11", y2: "17" }
    line { x1: "14", x2: "14", y1: "11", y2: "17" }
);

icon!(PanelLeftIcon,
    rect { width: "18", height: "18", x: "3", y: "3", rx: "2" }
    path { d: "M9 3v18" }
);

icon!(XIcon,
    path { d: "M18 6 6 18" }
    path { d: "m6 6 12 12" }
);
