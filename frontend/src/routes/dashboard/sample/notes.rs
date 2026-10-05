//! Canonical CRUD page. API calls live in `modules/apps/sample/api.rs`.

use dioxus::prelude::*;

use crate::modules::apps::sample::api::{create_note, delete_note, list_notes, update_note, Note, NoteCreate, NoteUpdate};
use crate::modules::base::icons::Trash2Icon;
use crate::modules::base::stores::auth::use_auth;
use crate::modules::base::ui::{
    Button, ButtonSize, ButtonVariant, Card, CardContent, CardDescription, CardHeader, CardTitle, Input, Label, Sheet,
    Table, TableBody, TableCell, TableHead, TableHeader, TableRow,
};
use crate::modules::base::utils::api_error::ApiError;
use crate::routes::Route;

#[component]
pub fn SampleNotes() -> Element {
    let auth = use_auth();
    let nav = navigator();

    let mut notes = use_signal(Vec::<Note>::new);
    let mut title = use_signal(String::new);
    let mut content = use_signal(String::new);
    let mut error = use_signal(|| None::<String>);
    let mut loading = use_signal(|| false);
    let mut saving = use_signal(|| false);
    let mut selected = use_signal(|| None::<Note>);
    let mut edit_title = use_signal(String::new);
    let mut edit_content = use_signal(String::new);

    // 401 anywhere: drop the session and go to /login. Other errors are shown above the table.
    let mut fail = move |err: ApiError| {
        if err.is_unauthorized() {
            auth.logout();
            nav.replace(Route::Login {});
        } else {
            error.set(Some(err.message));
        }
    };

    let refresh = move || async move {
        let Some(token) = auth.token() else { return };
        loading.set(true);
        error.set(None);
        match list_notes(&token).await {
            Ok(list) => notes.set(list),
            Err(err) => fail(err),
        }
        loading.set(false);
    };

    use_hook(move || {
        spawn(refresh());
    });

    let mut close_edit = move || {
        selected.set(None);
        edit_title.set(String::new());
        edit_content.set(String::new());
    };

    let mut open_edit = move |note: Note| {
        edit_title.set(note.title.clone());
        edit_content.set(note.content.clone());
        error.set(None);
        selected.set(Some(note));
    };

    let on_create = move |evt: FormEvent| async move {
        evt.prevent_default();
        let Some(token) = auth.token() else { return };
        let new_title = title().trim().to_string();
        if new_title.is_empty() {
            return;
        }
        saving.set(true);
        error.set(None);
        let data = NoteCreate { title: new_title, content: content().trim().to_string() };
        match create_note(&token, &data).await {
            Ok(_) => {
                title.set(String::new());
                content.set(String::new());
                refresh().await;
            }
            Err(err) => fail(err),
        }
        saving.set(false);
    };

    let on_save_edit = move |evt: FormEvent| async move {
        evt.prevent_default();
        let (Some(token), Some(note)) = (auth.token(), selected()) else { return };
        let new_title = edit_title().trim().to_string();
        if new_title.is_empty() {
            return;
        }
        saving.set(true);
        error.set(None);
        let data = NoteUpdate { title: Some(new_title), content: Some(edit_content().trim().to_string()) };
        match update_note(&token, &note.id, &data).await {
            Ok(_) => {
                close_edit();
                refresh().await;
            }
            Err(err) => fail(err),
        }
        saving.set(false);
    };

    let remove = move |id: String| async move {
        let Some(token) = auth.token() else { return };
        saving.set(true);
        error.set(None);
        match delete_note(&token, &id).await {
            Ok(()) => {
                if selected().is_some_and(|n| n.id == id) {
                    close_edit();
                }
                refresh().await;
            }
            Err(err) => fail(err),
        }
        saving.set(false);
    };

    let busy = loading() || saving();
    let count = if loading() { "Loading…".to_string() } else { format!("{} note(s)", notes.read().len()) };
    let selected_id = selected().map(|n| n.id);

    rsx! {
        div { class: "space-y-6",
            div {
                h1 { class: "text-3xl font-bold tracking-tight", "Sample Notes" }
                p { class: "text-muted-foreground", "Canonical CRUD module — Router → Service → Repository" }
            }

            div { class: "grid gap-6 lg:grid-cols-3",
                Card { class: "lg:col-span-1",
                    CardHeader {
                        CardTitle { "New note" }
                        CardDescription { "Create a note via POST /sample/notes" }
                    }
                    CardContent {
                        form { class: "space-y-4", onsubmit: on_create,
                            div { class: "space-y-2",
                                Label { r#for: "title", "Title" }
                                Input { id: "title", value: title(), required: true, oninput: move |v| title.set(v) }
                            }
                            div { class: "space-y-2",
                                Label { r#for: "content", "Content" }
                                Input { id: "content", value: content(), oninput: move |v| content.set(v) }
                            }
                            Button { r#type: "submit", disabled: busy, "Create note" }
                        }
                    }
                }

                Card { class: "lg:col-span-2",
                    CardHeader {
                        CardTitle { "Your notes" }
                        CardDescription { "{count}" }
                    }
                    CardContent {
                        if let Some(message) = error() {
                            p { class: "mb-4 text-sm text-destructive", "{message}" }
                        }
                        Table {
                            TableHeader {
                                TableRow {
                                    TableHead { "Title" }
                                    TableHead { "Content" }
                                    TableHead { class: "w-[80px]" }
                                }
                            }
                            TableBody {
                                if notes.read().is_empty() {
                                    TableRow {
                                        TableCell { colspan: 3, class: "text-muted-foreground", "No notes yet." }
                                    }
                                } else {
                                    for note in notes() {
                                        TableRow {
                                            key: "{note.id}",
                                            class: "cursor-pointer",
                                            selected: selected_id.as_deref() == Some(note.id.as_str()),
                                            onclick: {
                                                let note = note.clone();
                                                move |_| open_edit(note.clone())
                                            },
                                            TableCell { class: "font-medium", "{note.title}" }
                                            TableCell { class: "text-muted-foreground",
                                                if note.content.is_empty() { "—" } else { "{note.content}" }
                                            }
                                            TableCell {
                                                // Stop the row click from opening the editor.
                                                span { onclick: move |evt| evt.stop_propagation(),
                                                    Button {
                                                        variant: ButtonVariant::Ghost,
                                                        size: ButtonSize::Icon,
                                                        disabled: busy,
                                                        aria_label: "Delete note",
                                                        onclick: {
                                                            let id = note.id.clone();
                                                            move |_| {
                                                                spawn(remove(id.clone()));
                                                            }
                                                        },
                                                        Trash2Icon { class: "size-4 text-destructive" }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            Sheet {
                open: selected().is_some(),
                title: "Edit note",
                description: "Update the title and content, then save.",
                onclose: move |_| close_edit(),
                form { class: "flex h-full flex-col", onsubmit: on_save_edit,
                    div { class: "flex flex-1 flex-col gap-4 overflow-y-auto px-4 py-2",
                        div { class: "space-y-2",
                            Label { r#for: "edit-title", "Title" }
                            Input { id: "edit-title", value: edit_title(), required: true, oninput: move |v| edit_title.set(v) }
                        }
                        div { class: "space-y-2",
                            Label { r#for: "edit-content", "Content" }
                            Input { id: "edit-content", value: edit_content(), oninput: move |v| edit_content.set(v) }
                        }
                    }
                    div { class: "mt-auto flex flex-row justify-between gap-2 p-4",
                        Button {
                            variant: ButtonVariant::Destructive,
                            disabled: busy || selected().is_none(),
                            onclick: move |_| {
                                if let Some(note) = selected() {
                                    spawn(remove(note.id));
                                }
                            },
                            "Delete"
                        }
                        div { class: "flex gap-2",
                            Button { variant: ButtonVariant::Outline, disabled: busy, onclick: move |_| close_edit(), "Cancel" }
                            Button { r#type: "submit", disabled: busy, "Save changes" }
                        }
                    }
                }
            }
        }
    }
}
