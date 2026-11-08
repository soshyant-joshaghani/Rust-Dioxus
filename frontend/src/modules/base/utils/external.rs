//! Open a page outside the app (a payment gateway, a store page).
//!
//! Web: navigate the tab. Desktop and mobile: Dioxus refuses programmatic navigation to an outside
//! site ("External Navigation Failure"), so the page opens in the system browser. The app stays where
//! it is; a caller that wants to show something else meanwhile navigates itself.

#[cfg(target_arch = "wasm32")]
use dioxus::prelude::NavigationTarget;
use dioxus::router::Navigator;

#[cfg(target_arch = "wasm32")]
use crate::routes::Route;

pub fn open_external(nav: Navigator, url: String) {
    #[cfg(target_arch = "wasm32")]
    {
        nav.push(NavigationTarget::<Route>::External(url));
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = nav;
        if let Err(err) = webbrowser::open(&url) {
            eprintln!("could not open the system browser: {err}");
        }
    }
}
