# Dashboard UI

Dioxus 0.7 with Tailwind v4. The client lives in `frontend/` and reproduces Fast-Svelte's dashboard: the same routes, sidebar, header, theme toggle, login and dev signup, notes CRUD, and admin users screen, with the same color tokens (`frontend/tailwind.css`).

The same UI runs in the browser, in a desktop window (WebView2, WKWebView, WebKitGTK), and on Android and iOS. The sidebar collapses on wide screens and opens as an overlay on narrow ones, so phones get a usable layout.

The notes page (`/sample/notes`) is the canonical screen. Keep the route and the module name when moving a frontend between FoxG families. The API client is reqwest against `/api/v1`, so it works unchanged against any backend that follows the contract.

The dashboard home has a "Client" card that shows the platform and the API base URL. It is the quickest way to check that a native build talks to the right server.
