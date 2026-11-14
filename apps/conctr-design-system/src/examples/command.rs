use conctr_ui_dioxus::prelude::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut selected = use_signal(|| "None".to_string());
    let mut open = use_signal(|| false);
    rsx! {
        Stack {
            Command {
                id: "inline-commands",
                on_select: move |value| selected.set(value),
                CommandInput { placeholder: "Search connections, runs or settings…" }
                CommandList {
                    CommandEmpty { "No matching commands." }
                    CommandGroup { heading: "Navigate",
                        CommandItem {
                            index: 0,
                            value: "Connections",
                            keywords: vec!["source target integration".into()],
                            UiIcon { name: IconName::Connections }
                            "Connections"
                        }
                        CommandItem {
                            index: 1,
                            value: "Runs",
                            keywords: vec!["history executions".into()],
                            UiIcon { name: IconName::Runs }
                            "Runs"
                        }
                    }
                    CommandSeparator {}
                    CommandGroup { heading: "Workspace",
                        CommandItem { index: 2, value: "Settings",
                            UiIcon { name: IconName::Settings }
                            "Settings"
                            CommandShortcut {
                                Kbd { "↵" }
                            }
                        }
                        CommandItem {
                            index: 3,
                            value: "Delete workspace",
                            disabled: true,
                            "Delete workspace"
                        }
                    }
                }
            }
            p { role: "status", "Selected command: {selected}" }
            Button {
                variant: ButtonVariant::Secondary,
                onclick: move |_| open.set(true),
                "Open command palette"
            }
            CommandDialog {
                id: "dialog-commands",
                open,
                on_open_change: move |value| open.set(value),
                on_select: move |value| {
                    selected.set(value);
                    open.set(false);
                },
                CommandInput {}
                CommandList {
                    CommandEmpty { "No matching commands." }
                    CommandGroup { heading: "Actions",
                        CommandItem { index: 0, value: "Create connection",
                            UiIcon { name: IconName::Add }
                            "Create connection"
                        }
                        CommandItem { index: 1, value: "View runs",
                            UiIcon { name: IconName::Runs }
                            "View runs"
                        }
                    }
                }
            }
        }
    }
}
