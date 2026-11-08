//! Android hardware back button: go back in the app instead of sending it to the background.
//!
//! Why this is needed: Dioxus keeps the router history in memory on native platforms, so the
//! WebView has no history of its own. The generated `WryActivity` only goes back when
//! `WebView.canGoBack()` is true, which it never is, so Android closes the app.
//!
//! What this does, without touching the generated Kotlin (dx rewrites it on every build): while
//! the router can go back, a few `history.pushState` entries are kept in the WebView. Then
//! `canGoBack()` is true, the activity calls `goBack()`, the page gets a `popstate`, and that
//! calls `navigator().go_back()`. At the first page of the app the entries are removed again, so
//! back closes the app as usual. No-op on every other platform.
//!
//! One rule shapes the script: Chromium ignores, on back, any history entry that a page created
//! without a user gesture (checked on a real phone). A back press is not a gesture, so entries are
//! only created inside taps and clicks, several at once, so a few quick back presses still work.

use dioxus::prelude::*;

use crate::routes::Route;

/// Installs the listeners once and reports each back press with `dioxus.send`.
const LISTEN: &str = r#"
if (!window.__appBack) {
  var K = 8;            // entries kept in the WebView while the app can go back
  var count = 0;        // entries we created and have not used yet
  var want = false;     // true while the router can go back
  var ignore = 0;       // popstates caused by our own cleanup, not by the user

  // Only inside a user gesture: entries made without one are skipped by back.
  function topUp() {
    if (!want) return;
    if (!(navigator.userActivation && navigator.userActivation.isActive)) return;
    while (count < K) { history.pushState({ appBack: 1 }, ''); count++; }
  }

  window.__appBack = {
    arm: function (on) {
      want = on;
      if (on) { topUp(); }
      else if (count > 0) { var n = count; count = 0; ignore++; history.go(-n); }
    }
  };
  ['click', 'touchend', 'pointerup', 'keyup'].forEach(function (name) {
    document.addEventListener(name, topUp, true);
  });
  window.addEventListener('popstate', function () {
    if (ignore > 0) { ignore--; return; }
    if (count > 0) count--;  // the back press used one entry up
    dioxus.send('back');     // the app goes back, then arms again on the route change
  });
}
await new Promise(function () {}); // keep the channel open
"#;

fn arm_script(can_go_back: bool) -> String {
    // Retries until LISTEN has run (the two scripts start independently).
    format!("(function f() {{ if (window.__appBack) window.__appBack.arm({can_go_back}); else setTimeout(f, 50); }})();")
}

#[component]
pub fn AndroidBack() -> Element {
    if !cfg!(target_os = "android") {
        return rsx! {};
    }
    let nav = navigator();
    let route = use_route::<Route>();

    use_hook(move || {
        spawn(async move {
            let mut eval = document::eval(LISTEN);
            while eval.recv::<String>().await.is_ok() {
                nav.go_back();
            }
        });
    });

    // After every route change: keep the WebView entry only while there is somewhere to go back to.
    use_effect(use_reactive!(|route| {
        let _ = &route;
        document::eval(&arm_script(nav.can_go_back()));
    }));

    rsx! {}
}

#[cfg(test)]
mod tests {
    use super::arm_script;

    #[test]
    fn arm_script_carries_the_flag() {
        assert!(arm_script(true).contains("arm(true)"));
        assert!(arm_script(false).contains("arm(false)"));
    }
}
