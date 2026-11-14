use conctr_ui_dioxus::prelude::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut count = use_signal(|| 0usize);
    let file_word = if count() == 1 { "file" } else { "files" };
    rsx! {
        Stack { gap: Space::Lg,
            FileUploader {
                id: "flow-files",
                title: "Import connection files",
                description: "Drop JSON, YAML or CSV files here, or browse your device.",
                accept: ".json,.yaml,.yml,.csv",
                max_files: 3,
                max_file_size: 5 * 1024 * 1024,
                on_files_change: move |files: Vec<dioxus::html::FileData>| count.set(files.len()),
            }
            p { role: "status", "{count} {file_word} ready." }
            FileUploader {
                id: "disabled-files",
                title: "Attachments unavailable",
                disabled: true,
            }
        }
    }
}
