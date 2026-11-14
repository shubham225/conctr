use crate::components::ui::button::{Button, ButtonSize, ButtonVariant};
use dioxus::html::{FileData, HasFileData};
use dioxus::prelude::*;
use dioxus_icons::lucide::{FileText, Upload, X};
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};

#[css_module("/src/components/ui/file_uploader/style.css")]
struct Styles;

// Per-document registration is required for the Dioxus 0.7 SSR workaround.
const STYLES: Asset = asset!(
    "/src/components/ui/file_uploader/style.css",
    manganis::AssetOptions::css_module()
);

/// Reason a selected file was rejected before reading its contents.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FileRejectionReason {
    FileType,
    FileSize,
    FileCount,
}

/// Rejected file metadata. No file contents or paths are exposed in messages.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FileRejection {
    pub name: String,
    pub reason: FileRejectionReason,
}

impl FileRejection {
    fn message(&self) -> String {
        let reason = match self.reason {
            FileRejectionReason::FileType => "file type is not accepted",
            FileRejectionReason::FileSize => "file exceeds the size limit",
            FileRejectionReason::FileCount => "file count limit reached",
        };
        format!("{}: {reason}.", self.name)
    }
}

fn accepts_file(name: &str, mime: &str, accept: &str) -> bool {
    let name = name.to_lowercase();
    let mime = mime.to_lowercase();
    let rules: Vec<_> = accept
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .collect();
    rules.is_empty()
        || rules.iter().any(|rule| {
            let rule = rule.to_lowercase();
            if rule == "*/*" {
                true
            } else if rule.starts_with('.') {
                name.ends_with(&rule)
            } else if let Some(prefix) = rule.strip_suffix("/*") {
                mime.starts_with(&format!("{prefix}/"))
            } else {
                mime == rule
            }
        })
}

fn file_size(size: u64) -> String {
    if size >= 1_048_576 {
        format!("{:.1} MB", size as f64 / 1_048_576.0)
    } else if size >= 1024 {
        format!("{:.0} KB", size as f64 / 1024.0)
    } else {
        format!("{size} B")
    }
}

/// Drop zone, native file picker and removable selection list.
/// `on_files_change` receives the full current selection, including renderer file
/// handles with `read_bytes`, `read_string` and `byte_stream`. Uploads belong to the app.
#[component]
pub fn FileUploader(
    id: String,
    #[props(default = "Upload files".to_string())] title: String,
    #[props(default = "Drag and drop files here, or choose files from your device.".to_string())]
    description: String,
    #[props(default)] accept: String,
    #[props(default = true)] multiple: bool,
    #[props(default = 10)] max_files: usize,
    #[props(default = 10 * 1024 * 1024)] max_file_size: u64,
    #[props(default)] disabled: bool,
    #[props(default)] on_files_change: Option<EventHandler<Vec<FileData>>>,
    #[props(default)] on_reject: Option<EventHandler<Vec<FileRejection>>>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let mut files = use_signal(Vec::<FileData>::new);
    let mut rejections = use_signal(Vec::<FileRejection>::new);
    let mut drag_depth = use_signal(|| 0u32);
    let mut picker_version = use_signal(|| 0u64);
    let limit = if multiple { max_files.max(1) } else { 1 };
    let accept_filter = accept.clone();
    let select_files = use_callback(move |incoming: Vec<FileData>| {
        if disabled || incoming.is_empty() {
            return;
        }
        let mut next = if multiple {
            files.peek().clone()
        } else {
            Vec::new()
        };
        let mut rejected = Vec::new();
        for file in incoming {
            if next.contains(&file) {
                continue;
            }
            let name = file.name();
            let reason = if !accepts_file(
                &name,
                &file.content_type().unwrap_or_default(),
                &accept_filter,
            ) {
                Some(FileRejectionReason::FileType)
            } else if file.size() > max_file_size {
                Some(FileRejectionReason::FileSize)
            } else if next.len() >= limit {
                Some(FileRejectionReason::FileCount)
            } else {
                None
            };
            if let Some(reason) = reason {
                rejected.push(FileRejection { name, reason });
            } else {
                next.push(file);
            }
        }
        // A rejected replacement leaves the previous single-file selection intact.
        if !multiple && next.is_empty() && !rejected.is_empty() {
            next = files.peek().clone();
        }
        if next != *files.peek() {
            files.set(next.clone());
            if let Some(callback) = on_files_change {
                callback.call(next);
            }
        }
        if !rejected.is_empty()
            && let Some(callback) = on_reject
        {
            callback.call(rejected.clone());
        }
        rejections.set(rejected);
    });
    let file_word = if limit == 1 { "file" } else { "files" };
    let selected_count = files().len();
    let selected_word = if selected_count == 1 { "file" } else { "files" };
    let hint_id = format!("{id}-hint");
    let error_id = format!("{id}-errors");
    let described_by = if rejections().is_empty() {
        hint_id.clone()
    } else {
        format!("{hint_id} {error_id}")
    };
    let base = attributes!(section {
        class: "cn-file-uploader {Styles::uploader.inner} grid gap-3",
        "data-disabled": disabled,
    });
    let merged = merge_attributes(vec![base, attributes]);
    rsx! {
        document::Stylesheet { href: STYLES }
        section {..merged,
            label {
                class: "cn-file-dropzone {Styles::dropzone.inner} relative grid justify-items-center text-center gap-3",
                r#for: id.clone(),
                "data-dragging": drag_depth() > 0 && !disabled,
                ondragover: move |event| event.prevent_default(),
                ondragenter: move |event| {
                    event.prevent_default();
                    if !disabled {
                        drag_depth += 1;
                    }
                },
                ondragleave: move |_| {
                    let depth = *drag_depth.peek();
                    drag_depth.set(depth.saturating_sub(1));
                },
                ondrop: move |event| {
                    event.prevent_default();
                    drag_depth.set(0);
                    if !disabled {
                        select_files.call(event.files());
                    }
                },
                input {
                    key: "{picker_version}",
                    id: id.clone(),
                    r#type: "file",
                    accept,
                    multiple,
                    disabled,
                    class: "cn-file-picker {Styles::picker.inner} absolute inset-0 w-full h-full opacity-0 cursor-pointer",
                    aria_label: title.clone(),
                    aria_describedby: described_by,
                    aria_invalid: !rejections().is_empty(),
                    onchange: move |event| {
                        select_files.call(event.files());
                        picker_version += 1;
                    },
                    onmounted: move |event| async move {
                        if picker_version() > 0 && !disabled {
                            let _ = event.data().set_focus(true).await;
                        }
                    },
                }
                span {
                    class: "cn-file-upload-icon {Styles::upload_icon.inner} grid place-items-center",
                    aria_hidden: "true",
                    Upload { size: "24" }
                }
                div { class: "cn-file-upload-copy {Styles::copy.inner} grid gap-1",
                    strong { "{title}" }
                    span { "{description}" }
                }
                span { class: "cn-file-choose {Styles::choose.inner} inline-flex items-center justify-center",
                    if multiple {
                        "Choose files"
                    } else {
                        "Choose file"
                    }
                }
            }
            p { id: hint_id, class: "cn-file-hint {Styles::hint.inner}",
                "Up to {limit} {file_word}. Maximum {file_size(max_file_size)} per file."
            }
            if !rejections().is_empty() {
                ul {
                    id: error_id,
                    role: "alert",
                    class: "cn-file-errors {Styles::errors.inner} grid gap-1",
                    for rejected in rejections().iter().take(3) {
                        li { "{rejected.message()}" }
                    }
                    if rejections().len() > 3 {
                        li { "And {rejections().len() - 3} more." }
                    }
                }
            }
            if !files().is_empty() {
                ul {
                    class: "cn-file-list {Styles::list.inner} grid gap-2",
                    aria_label: "Selected files",
                    for (index, file) in files().iter().enumerate() {
                        li {
                            key: "{file.name()}-{file.size()}-{file.last_modified()}",
                            class: "cn-file-row {Styles::row.inner} flex items-center gap-3",
                            span {
                                class: "cn-file-type {Styles::file_type.inner} shrink-0",
                                aria_hidden: "true",
                                FileText { size: "20" }
                            }
                            div { class: "cn-file-metadata {Styles::metadata.inner} grid gap-1 min-w-0 flex-1",
                                span { "{file.name()}" }
                                small { "{file_size(file.size())}" }
                            }
                            Button {
                                variant: ButtonVariant::Ghost,
                                size: ButtonSize::IconSm,
                                disabled,
                                aria_label: format!("Remove {}", file.name()),
                                onclick: move |_| {
                                    if disabled {
                                        return;
                                    }
                                    let mut next = files.peek().clone();
                                    if index < next.len() {
                                        next.remove(index);
                                    }
                                    files.set(next.clone());
                                    rejections.set(Vec::new());
                                    if let Some(callback) = on_files_change {
                                        callback.call(next);
                                    }
                                },
                                X { size: "16" }
                            }
                        }
                    }
                }
            }
            span { class: "sr-only", role: "status", aria_live: "polite",
                "{selected_count} {selected_word} selected."
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn accept_rules_cover_extensions_mime_wildcards_and_empty_filters() {
        assert!(accepts_file("FLOW.JSON", "", ".json, .yaml"));
        assert!(accepts_file("logo.svg", "image/svg+xml", "image/*"));
        assert!(accepts_file(
            "flow.json",
            "application/json",
            "application/json"
        ));
        assert!(accepts_file("notes.txt", "text/plain", " , "));
        assert!(!accepts_file("notes.txt", "text/plain", ".json, image/*"));
        assert!(!accepts_file("payload.exe", "", "application/json"));
    }
}
