use crate::{Route, sign_in};
use conctr_ui_dioxus::ui::{avatar::*, dropdown_menu::*};
use dioxus::prelude::*;

#[component]
pub(super) fn UserControl() -> Element {
    let mut session = use_context::<sign_in::SessionState>();
    let username = session
        .read()
        .as_ref()
        .map(|s| s.username.clone())
        .unwrap_or_default();
    let mut failure = use_signal(|| false);
    rsx! {
        div { class: "@container relative",
            DropdownMenu {
                DropdownMenuTrigger { class: "h-11 w-full justify-center gap-2 border-0 bg-transparent px-0 @min-[9rem]:justify-start @min-[9rem]:px-2",
                    span { class: "sr-only", "User options for {username}" }
                    UserAvatar {}
                    span { class: "hidden min-w-0 flex-1 text-left leading-tight @min-[9rem]:grid",
                        span { class: "truncate text-sm font-medium", "{username}" }
                        span { class: "truncate text-xs text-muted-foreground", "Workspace account" }
                    }
                    icons::ChevronsUpDown { class: "hidden size-4 text-muted-foreground @min-[9rem]:block" }
                }
                DropdownMenuContent { position: DropdownMenuPosition::Top,
                    class: "w-56 max-w-full rounded-lg shadow-lg data-[state=closed]:invisible md:absolute! md:top-auto! md:bottom-0 md:left-full! md:ml-4 md:max-w-none",
                    div { class: "flex items-center gap-2 px-2 py-2",
                        UserAvatar {}
                        div { class: "grid text-sm leading-tight",
                            strong { class: "font-medium", "{username}" }
                            span { class: "text-xs text-muted-foreground", "Workspace account" }
                        }
                    }
                    DropdownMenuSeparator {}
                    DropdownMenuGroup {
                        for (route, icon) in [(Route::Profile {}, rsx! { icons::UserRound {} }), (Route::Notifications {}, rsx! { icons::Bell {} })] {
                            DropdownMenuItem { key: "{route}", class: "p-0",
                                Link { to: route.clone(), class: "flex w-full items-center gap-2 px-2 py-2", "data-dropdown-close": "",
                                    {icon}
                                    "{route.label()}"
                                }
                            }
                        }
                    }
                    DropdownMenuSeparator {}
                    button { r#type: "button",
                        onclick: move |_| async move {
                            match conctr_api_dioxus::logout().await {
                                Ok(()) => session.set(None),
                                Err(_) => failure.set(true),
                            }
                        },
                        class: "flex w-full items-center gap-2 px-2 py-2 text-sm text-foreground hover:bg-accent",
                        icons::LogOut { class: "size-4" } "Log out"
                    }
                    if failure() { p { role: "alert", "Sign-out failed. Try again." } }
                }
            }
        }
    }
}

#[component]
fn UserAvatar() -> Element {
    rsx! {
        Avatar { class: "rounded-lg",
            AvatarFallback { class: "rounded-lg bg-secondary text-secondary-foreground", icons::UserRound {} }
        }
    }
}
