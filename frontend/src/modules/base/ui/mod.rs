//! Design primitives (the Dioxus port of Rust-Svelte's shadcn `base/ui`). Tailwind classes only;
//! tokens live in `frontend/tailwind.css`. Class strings are written out in full so Tailwind finds them.

pub mod avatar;
pub mod badge;
pub mod button;
pub mod card;
pub mod input;
pub mod label;
pub mod separator;
pub mod sheet;
pub mod sidebar;
pub mod switch;
pub mod table;

pub use avatar::Avatar;
pub use badge::{Badge, BadgeVariant};
pub use button::{Button, ButtonSize, ButtonVariant};
pub use card::{Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle};
pub use input::Input;
pub use label::Label;
pub use separator::Separator;
pub use sheet::Sheet;
pub use switch::Switch;
pub use table::{Table, TableBody, TableCell, TableHead, TableHeader, TableRow};
